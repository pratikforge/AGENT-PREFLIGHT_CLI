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
        let is_test_fixture = file.path.contains("test");
        for call in &file.calls {
            let mut is_leak = false;
            let mut is_hardcoded = false;
            for ctrl in &call.static_controls {
                if ctrl.contains("AWS_SECRET_ACCESS_KEY") {
                    is_leak = true;
                }
                if ctrl.contains("ghp_") {
                    is_hardcoded = true;
                }
            }

            if is_test_fixture {
                if is_leak || is_hardcoded {
                    findings.push(Finding {
                        rule_id: "secrets-test-fixture".to_string(),
                        status: Status::Verified,
                        evidence: EvidenceRef {
                            path: file.path.clone(),
                            line: call.span.line,
                            parser_error: false,
                        },
                        matrix_source: "SECRETS".to_string(),
                    });
                }
            } else {
                if is_leak {
                    findings.push(Finding {
                        rule_id: "secrets-env-leak".to_string(),
                        status: Status::Failed,
                        evidence: EvidenceRef {
                            path: file.path.clone(),
                            line: call.span.line,
                            parser_error: false,
                        },
                        matrix_source: "SECRETS".to_string(),
                    });
                }
                if is_hardcoded {
                    findings.push(Finding {
                        rule_id: "secrets-hardcoded-token".to_string(),
                        status: Status::Failed,
                        evidence: EvidenceRef {
                            path: file.path.clone(),
                            line: call.span.line,
                            parser_error: false,
                        },
                        matrix_source: "SECRETS".to_string(),
                    });
                }
            }
        }
    }
    findings
}
