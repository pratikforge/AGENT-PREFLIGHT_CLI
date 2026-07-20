use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::{DecoratorFact, NormalizedFile};
use crate::domain::status::Status;

pub const RULE_ID: &str = "openai-function-tool-approval";
pub const AGENT_TOOL_RULE_ID: &str = "openai-agent-as-tool-approval";
pub const MCP_RULE_ID: &str = "openai-mcp-server-approval";
pub const LOCAL_RUNTIME_TOOL_RULE_ID: &str = "openai-local-runtime-tool-approval";
pub const HOSTED_MCP_RULE_ID: &str = "openai-hosted-mcp-approval";
pub const MATRIX_SOURCE: &str = "ADAPTER_EVIDENCE_MATRIX.md#openai";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(files: &[NormalizedFile]) -> Vec<Finding> {
    files
        .iter()
        .flat_map(|file| {
            let mut findings = Vec::new();
            if imports_direct_function_tool(file) {
                findings.extend(
                    file.decorators
                        .iter()
                        .map(|decorator| finding(file, decorator)),
                );
            } else if let Some(import) = file.imports.iter().find(|fact| {
                fact.module == "agents"
                    && fact.symbol.as_deref() == Some("function_tool")
                    && fact.alias.is_some()
            }) {
                findings.push(unverifiable_import(file, import.span.line));
            }

            if imports_direct_agent(file) && file.calls.iter().any(|call| call.callee == "Agent") {
                findings.extend(
                    file.calls
                        .iter()
                        .filter(|call| call.callee.ends_with(".as_tool"))
                        .map(|call| agent_tool_finding(file, call)),
                );
            }

            if imports_direct_mcp_server(file) {
                findings.extend(
                    file.calls
                        .iter()
                        .filter(|call| is_direct_mcp_server_constructor(&call.callee))
                        .map(|call| mcp_finding(file, call)),
                );
            }

            if imports_direct_local_runtime_tool(file) {
                findings.extend(
                    file.calls
                        .iter()
                        .filter(|call| is_direct_local_runtime_tool_constructor(&call.callee))
                        .map(|call| local_runtime_tool_finding(file, call)),
                );
            }

            if imports_direct_hosted_mcp_tool(file) {
                findings.extend(
                    file.calls
                        .iter()
                        .filter(|call| call.callee == "HostedMCPTool")
                        .map(|call| hosted_mcp_finding(file, call)),
                );
            }

            findings
        })
        .collect()
}

fn unverifiable_import(file: &NormalizedFile, line: u32) -> Finding {
    Finding {
        rule_id: RULE_ID.to_owned(),
        status: Status::CannotVerifyStatically,
        evidence: EvidenceRef {
            path: file.path.clone(),
            line,
            parser_error: false,
        },
        matrix_source: MATRIX_SOURCE.to_owned(),
    }
}

fn imports_direct_function_tool(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "agents"
            && fact.symbol.as_deref() == Some("function_tool")
            && fact.alias.is_none()
    })
}

fn imports_direct_agent(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "agents" && fact.symbol.as_deref() == Some("Agent") && fact.alias.is_none()
    })
}

fn imports_direct_mcp_server(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "agents.mcp"
            && fact.alias.is_none()
            && fact
                .symbol
                .as_deref()
                .is_some_and(is_direct_mcp_server_constructor)
    })
}

fn is_direct_mcp_server_constructor(callee: &str) -> bool {
    matches!(
        callee,
        "MCPServerStdio" | "MCPServerSse" | "MCPServerStreamableHttp"
    )
}

fn imports_direct_local_runtime_tool(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "agents"
            && fact.alias.is_none()
            && fact
                .symbol
                .as_deref()
                .is_some_and(is_direct_local_runtime_tool_constructor)
    })
}

fn is_direct_local_runtime_tool_constructor(callee: &str) -> bool {
    matches!(callee, "ShellTool" | "ApplyPatchTool")
}

fn imports_direct_hosted_mcp_tool(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "agents"
            && fact.symbol.as_deref() == Some("HostedMCPTool")
            && fact.alias.is_none()
    })
}

fn finding(file: &NormalizedFile, decorator: &DecoratorFact) -> Finding {
    let status =
        if decorator.name == "function_tool" && decorator.arguments == "needs_approval=True" {
            Status::Verified
        } else {
            Status::CannotVerifyStatically
        };
    Finding {
        rule_id: RULE_ID.to_owned(),
        status,
        evidence: EvidenceRef {
            path: file.path.clone(),
            line: decorator.span.line,
            parser_error: false,
        },
        matrix_source: MATRIX_SOURCE.to_owned(),
    }
}

fn agent_tool_finding(
    file: &NormalizedFile,
    call: &crate::domain::normalized::CallFact,
) -> Finding {
    Finding {
        rule_id: AGENT_TOOL_RULE_ID.to_owned(),
        status: if has_static_control(call, "needs_approval=True") {
            Status::Verified
        } else {
            Status::CannotVerifyStatically
        },
        evidence: EvidenceRef {
            path: file.path.clone(),
            line: call.span.line,
            parser_error: false,
        },
        matrix_source: MATRIX_SOURCE.to_owned(),
    }
}

fn mcp_finding(file: &NormalizedFile, call: &crate::domain::normalized::CallFact) -> Finding {
    Finding {
        rule_id: MCP_RULE_ID.to_owned(),
        status: if has_static_control(call, "require_approval=always") {
            Status::Verified
        } else {
            Status::CannotVerifyStatically
        },
        evidence: EvidenceRef {
            path: file.path.clone(),
            line: call.span.line,
            parser_error: false,
        },
        matrix_source: MATRIX_SOURCE.to_owned(),
    }
}

fn local_runtime_tool_finding(
    file: &NormalizedFile,
    call: &crate::domain::normalized::CallFact,
) -> Finding {
    Finding {
        rule_id: LOCAL_RUNTIME_TOOL_RULE_ID.to_owned(),
        status: if has_static_control(call, "needs_approval=True") {
            Status::Verified
        } else {
            Status::CannotVerifyStatically
        },
        evidence: EvidenceRef {
            path: file.path.clone(),
            line: call.span.line,
            parser_error: false,
        },
        matrix_source: MATRIX_SOURCE.to_owned(),
    }
}

fn hosted_mcp_finding(
    file: &NormalizedFile,
    call: &crate::domain::normalized::CallFact,
) -> Finding {
    Finding {
        rule_id: HOSTED_MCP_RULE_ID.to_owned(),
        status: if has_static_control(call, "hosted_mcp_require_approval=always") {
            Status::Verified
        } else {
            Status::CannotVerifyStatically
        },
        evidence: EvidenceRef {
            path: file.path.clone(),
            line: call.span.line,
            parser_error: false,
        },
        matrix_source: MATRIX_SOURCE.to_owned(),
    }
}

fn has_static_control(call: &crate::domain::normalized::CallFact, control: &str) -> bool {
    call.static_controls.iter().any(|value| value == control)
}
