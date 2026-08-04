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
            if call.callee == "recognize_generated_file_with_checked_in_source" {
                findings.push(Finding {
                    rule_id: "recognize_generated_file_with_checked_in_source".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "generated_provenance".to_string(),
                });
            }
            if call.callee == "reject_untraceable_generated_output" {
                findings.push(Finding {
                    rule_id: "reject_untraceable_generated_output".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "generated_provenance".to_string(),
                });
            }
            if call.callee == "reject_source_generated_outside_scan_root" {
                findings.push(Finding {
                    rule_id: "reject_source_generated_outside_scan_root".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "generated_provenance".to_string(),
                });
            }
        }
    }
    findings
}
