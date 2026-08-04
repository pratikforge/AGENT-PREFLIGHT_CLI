use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::NormalizedFile;
use crate::domain::status::Status;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(files: &[NormalizedFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        for call in &file.calls {
            // function call enabled
            if call.callee == "Tool" || call.callee.contains("FunctionDeclaration") {
                findings.push(Finding {
                    rule_id: "gemini-function-call".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "GEMINI".to_string(),
                });
            }

            // code execution
            if call.callee.contains("CodeExecution") {
                findings.push(Finding {
                    rule_id: "gemini-code-execution".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "GEMINI".to_string(),
                });
            }

            // url context grounding
            if call.callee.contains("GoogleSearchRetrieval") {
                findings.push(Finding {
                    rule_id: "gemini-url-context".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "GEMINI".to_string(),
                });
            }

            // File search
            if call.callee.contains("FileSearch") {
                findings.push(Finding {
                    rule_id: "gemini-file-search".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "GEMINI".to_string(),
                });
            }

            // MCP integration
            if call.callee.contains("McpServer") {
                findings.push(Finding {
                    rule_id: "gemini-mcp".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "GEMINI".to_string(),
                });
            }
        }
    }
    findings
}

pub fn to_ir(files: &[NormalizedFile]) -> crate::domain::ir::CapabilityIr {
    let mut ir = crate::domain::ir::CapabilityIr::default();

    for file in files {
        let mut tools = Vec::new();
        let mut mcp_servers = Vec::new();

        for call in &file.calls {
            if call.callee == "Tool" || call.callee.contains("FunctionDeclaration") {
                tools.push(crate::domain::ir::Tool {
                    id: "function_tool".to_string(),
                    implementation: call.callee.clone(),
                    approval_control: "unknown".to_string(),
                });
            }
            if call.callee.contains("CodeExecution") {
                tools.push(crate::domain::ir::Tool {
                    id: "code_execution".to_string(),
                    implementation: call.callee.clone(),
                    approval_control: "unknown".to_string(),
                });
            }
            if call.callee.contains("GoogleSearchRetrieval") {
                tools.push(crate::domain::ir::Tool {
                    id: "url_context".to_string(),
                    implementation: call.callee.clone(),
                    approval_control: "unknown".to_string(),
                });
            }
            if call.callee.contains("FileSearch") {
                tools.push(crate::domain::ir::Tool {
                    id: "file_search".to_string(),
                    implementation: call.callee.clone(),
                    approval_control: "unknown".to_string(),
                });
            }
            if call.callee.contains("McpServer") {
                mcp_servers.push(crate::domain::ir::McpServer {
                    endpoint: call.callee.clone(),
                    transport: "stdio".to_string(),
                });
            }
        }

        if !tools.is_empty() || !mcp_servers.is_empty() {
            ir.agents.push(crate::domain::ir::Agent {
                id: "agent".to_string(),
                provider: "gemini".to_string(),
                tools,
                mcp_servers,
                sandbox: None,
                destinations: vec![],
                sensitive_data: vec![],
                dependencies: vec![],
                evidence: crate::domain::ir::EvidenceNode {
                    origin: file.path.clone(),
                    refs: vec![],
                },
            });
        }
    }

    ir
}
