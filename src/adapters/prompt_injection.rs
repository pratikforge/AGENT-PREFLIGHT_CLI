use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::{NormalizedFile, TaintLabel};
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
        let mut has_direct_injection = false;
        let mut has_indirect_injection = false;
        let mut has_safe_prompt = false;

        for flow in &file.data_flows {
            if flow.variable_name.contains("prompt")
                || flow.variable_name.contains("system_message")
            {
                match flow.taint {
                    TaintLabel::User => {
                        has_direct_injection = true;
                        findings.push(Finding {
                            rule_id: "direct-prompt-injection".to_string(),
                            status: Status::Failed,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: flow.span.line,
                                parser_error: false,
                            },
                            matrix_source: "PROMPT_INJECTION".to_string(),
                        });
                    }
                    TaintLabel::Web => {
                        has_indirect_injection = true;
                        findings.push(Finding {
                            rule_id: "indirect-prompt-injection".to_string(),
                            status: Status::Failed,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: flow.span.line,
                                parser_error: false,
                            },
                            matrix_source: "PROMPT_INJECTION".to_string(),
                        });
                    }
                    TaintLabel::Tool | TaintLabel::Secret | TaintLabel::Pii => {
                        has_safe_prompt = true;
                    }
                }
            }
        }

        if !has_direct_injection && !has_indirect_injection && has_safe_prompt {
            findings.push(Finding {
                rule_id: "safe-system-prompt".to_string(),
                status: Status::Verified,
                evidence: EvidenceRef {
                    path: file.path.clone(),
                    line: 1, // Fallback line
                    parser_error: false,
                },
                matrix_source: "PROMPT_INJECTION".to_string(),
            });
        }
    }
    findings
}
