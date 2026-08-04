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
            if call.callee == "propagate_literal_approval_constant" {
                findings.push(Finding {
                    rule_id: "propagate_literal_approval_constant".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "constant_propagation".to_string(),
                });
            }
            if call.callee == "reject_mutable_value" {
                findings.push(Finding {
                    rule_id: "reject_mutable_value".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "constant_propagation".to_string(),
                });
            }
            if call.callee == "reject_environment_derived_unknown" {
                findings.push(Finding {
                    rule_id: "reject_environment_derived_unknown".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "constant_propagation".to_string(),
                });
            }
            if call.callee == "preserve_false_true_distinctions" {
                findings.push(Finding {
                    rule_id: "preserve_false_true_distinctions".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "constant_propagation".to_string(),
                });
            }
        }
    }
    findings
}
