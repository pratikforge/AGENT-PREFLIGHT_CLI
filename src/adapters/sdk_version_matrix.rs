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
            if call.callee == "unknown_version" {
                findings.push(Finding {
                    rule_id: "unknown_version".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "sdk_version_matrix".to_string(),
                });
            }
            if call.callee == "below_minimum_version" {
                findings.push(Finding {
                    rule_id: "below_minimum_version".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "sdk_version_matrix".to_string(),
                });
            }
            if call.callee == "above_tested_version" {
                findings.push(Finding {
                    rule_id: "above_tested_version".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "sdk_version_matrix".to_string(),
                });
            }
            if call.callee == "supported_pinned_version" {
                findings.push(Finding {
                    rule_id: "supported_pinned_version".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "sdk_version_matrix".to_string(),
                });
            }
        }
    }
    findings
}
