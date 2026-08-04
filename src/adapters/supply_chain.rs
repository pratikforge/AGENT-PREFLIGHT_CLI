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
            if call.callee.contains("unpinned") {
                findings.push(Finding {
                    rule_id: "supply-unpinned".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "SUPPLY".to_string(),
                });
            }

            if call.callee.contains("vulnerable") {
                findings.push(Finding {
                    rule_id: "supply-vulnerable".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "SUPPLY".to_string(),
                });
            }

            if call.callee.contains("sbom") {
                findings.push(Finding {
                    rule_id: "supply-sbom".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "SUPPLY".to_string(),
                });
            }
        }
    }
    findings
}
