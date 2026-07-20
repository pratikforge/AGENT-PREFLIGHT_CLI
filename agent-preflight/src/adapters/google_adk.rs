use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::NormalizedFile;
use crate::domain::status::Status;

pub const RULE_ID: &str = "google-adk-function-tool-confirmation";
pub const AGENT_TOOLS_RULE_ID: &str = "google-adk-agent-tool-registration";
pub const MATRIX_SOURCE: &str = "ADAPTER_EVIDENCE_MATRIX.md#google-adk";

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
            if imports_direct_function_tool(file) {
                file.calls
                    .iter()
                    .filter(|call| call.callee == "FunctionTool")
                    .map(|call| Finding {
                        rule_id: RULE_ID.to_owned(),
                        status: if call
                            .true_keywords
                            .iter()
                            .any(|keyword| keyword == "require_confirmation")
                        {
                            Status::Verified
                        } else if has_static_control(call, "require_confirmation=False") {
                            Status::Failed
                        } else {
                            Status::CannotVerifyStatically
                        },
                        evidence: EvidenceRef {
                            path: file.path.clone(),
                            line: call.span.line,
                            parser_error: false,
                        },
                        matrix_source: MATRIX_SOURCE.to_owned(),
                    })
                    .collect()
            } else if imports_direct_agent(file) {
                file.calls
                    .iter()
                    .filter(|call| {
                        (call.callee == "Agent" || call.callee == "LlmAgent")
                            && call.keyword_names.iter().any(|keyword| keyword == "tools")
                    })
                    .map(|call| Finding {
                        rule_id: AGENT_TOOLS_RULE_ID.to_owned(),
                        status: Status::CannotVerifyStatically,
                        evidence: EvidenceRef {
                            path: file.path.clone(),
                            line: call.span.line,
                            parser_error: false,
                        },
                        matrix_source: MATRIX_SOURCE.to_owned(),
                    })
                    .collect()
            } else if let Some(import) = file.imports.iter().find(|fact| {
                fact.module == "google.adk.tools.function_tool"
                    && fact.symbol.as_deref() == Some("FunctionTool")
                    && fact.alias.is_some()
            }) {
                vec![Finding {
                    rule_id: RULE_ID.to_owned(),
                    status: Status::CannotVerifyStatically,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: import.span.line,
                        parser_error: false,
                    },
                    matrix_source: MATRIX_SOURCE.to_owned(),
                }]
            } else {
                Vec::new()
            }
        })
        .collect()
}

fn imports_direct_agent(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "google.adk.agents"
            && matches!(fact.symbol.as_deref(), Some("Agent") | Some("LlmAgent"))
            && fact.alias.is_none()
    })
}

fn imports_direct_function_tool(file: &NormalizedFile) -> bool {
    file.imports.iter().any(|fact| {
        fact.module == "google.adk.tools.function_tool"
            && fact.symbol.as_deref() == Some("FunctionTool")
            && fact.alias.is_none()
    })
}

fn has_static_control(call: &crate::domain::normalized::CallFact, control: &str) -> bool {
    call.static_controls.iter().any(|value| value == control)
}
