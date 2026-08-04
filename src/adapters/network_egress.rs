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
            for control in &call.static_controls {
                if control.starts_with("http://") || control.starts_with("https://") {
                    if control.contains("192.168.")
                        || control.contains("10.")
                        || control.contains("127.")
                    {
                        findings.push(Finding {
                            rule_id: "egress-private-network".to_string(),
                            status: Status::Failed,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: call.span.line,
                                parser_error: false,
                            },
                            matrix_source: "EGRESS".to_string(),
                        });
                    } else if control.contains("169.254.169.254") {
                        findings.push(Finding {
                            rule_id: "egress-metadata-endpoint".to_string(),
                            status: Status::Failed,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: call.span.line,
                                parser_error: false,
                            },
                            matrix_source: "EGRESS".to_string(),
                        });
                    } else {
                        findings.push(Finding {
                            rule_id: "egress-whitelisted".to_string(),
                            status: Status::Verified,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: call.span.line,
                                parser_error: false,
                            },
                            matrix_source: "EGRESS".to_string(),
                        });
                    }
                }
            }
        }
    }
    findings
}
