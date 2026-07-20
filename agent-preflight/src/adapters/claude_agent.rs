use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::NormalizedFile;
use crate::domain::status::Status;

pub const RULE_ID: &str = "claude-query-permission-mode";
pub const MATRIX_SOURCE: &str = "ADAPTER_EVIDENCE_MATRIX.md#claude-agent-sdk";

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
            if imports_direct_typescript_query(file) {
                file.calls
                    .iter()
                    .filter(|call| call.callee == "query")
                    .map(|call| finding(file, call, "permissionMode", "allowedTools"))
                    .collect()
            } else if imports_direct_python_query(file) {
                python_findings(file)
            } else if let Some(import) = aliased_query_import(file) {
                vec![unverifiable_import(file, import.span.line)]
            } else {
                Vec::new()
            }
        })
        .collect()
}

fn imports_direct_typescript_query(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "@anthropic-ai/claude-agent-sdk"
            && fact.symbol.as_deref() == Some("query")
            && fact.alias.is_none()
    })
}

fn imports_direct_python_query(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "claude_agent_sdk"
            && fact.symbol.as_deref() == Some("query")
            && fact.alias.is_none()
    })
}

fn imports_direct_python_options(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "claude_agent_sdk"
            && fact.symbol.as_deref() == Some("ClaudeAgentOptions")
            && fact.alias.is_none()
    })
}

fn aliased_query_import(file: &NormalizedFile) -> Option<&crate::domain::normalized::ImportFact> {
    file.imports.iter().find(|fact| {
        matches!(
            fact.module.as_str(),
            "@anthropic-ai/claude-agent-sdk" | "claude_agent_sdk"
        ) && fact.symbol.as_deref() == Some("query")
            && fact.alias.is_some()
    })
}

fn python_findings(file: &NormalizedFile) -> Vec<Finding> {
    if imports_direct_python_options(file) {
        let option_findings: Vec<_> = file
            .calls
            .iter()
            .filter(|call| call.callee == "ClaudeAgentOptions")
            .map(|call| finding(file, call, "permission_mode", "allowed_tools"))
            .collect();
        if !option_findings.is_empty() {
            return option_findings;
        }
    }
    file.calls
        .iter()
        .filter(|call| call.callee == "query")
        .map(|call| finding(file, call, "permission_mode", "allowed_tools"))
        .collect()
}

fn finding(
    file: &NormalizedFile,
    call: &crate::domain::normalized::CallFact,
    permission_key: &str,
    allowed_tools_key: &str,
) -> Finding {
    let bypass = format!("{permission_key}=bypassPermissions");
    let locked_down = format!("{permission_key}=dontAsk");
    let plan = format!("{permission_key}=plan");
    let allowed_tools = format!("{allowed_tools_key}=literal-nonempty");
    Finding {
        rule_id: RULE_ID.to_owned(),
        status: if has_static_control(call, &bypass) {
            Status::Failed
        } else if has_static_control(call, &plan)
            || (has_static_control(call, &locked_down) && has_static_control(call, &allowed_tools))
        {
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

fn has_static_control(call: &crate::domain::normalized::CallFact, control: &str) -> bool {
    call.static_controls.iter().any(|value| value == control)
}
