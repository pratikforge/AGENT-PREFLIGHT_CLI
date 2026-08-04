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
            if call.callee
                == "parse_safe_unsafe_json_yaml_toml_compose_kubernetes_ci_and_env_example"
            {
                findings.push(Finding {
                    rule_id:
                        "parse_safe_unsafe_json_yaml_toml_compose_kubernetes_ci_and_env_example"
                            .to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "config_analysis".to_string(),
                });
            }
            if call.callee == "reject_malformed_config" {
                findings.push(Finding {
                    rule_id: "reject_malformed_config".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "config_analysis".to_string(),
                });
            }
            if call.callee == "detect_conflicting_configuration" {
                findings.push(Finding {
                    rule_id: "detect_conflicting_configuration".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "config_analysis".to_string(),
                });
            }
        }
    }
    findings
}
