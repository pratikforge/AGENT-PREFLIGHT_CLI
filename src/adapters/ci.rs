use crate::domain::evidence::EvidenceRef;
use crate::domain::status::Status;
use serde_yaml_ng::Value;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(yaml: &str, path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let doc = match serde_yaml_ng::from_str::<Value>(yaml) {
        Ok(v) => v,
        Err(_) => return findings,
    };

    // check permissions: write-all
    if let Some(perms) = doc.get("permissions").and_then(|v| v.as_str())
        && perms == "write-all"
    {
        findings.push(Finding {
            rule_id: "ci-permissions-write-all".to_string(),
            status: Status::Failed,
            evidence: EvidenceRef {
                path: path.to_string(),
                line: 0,
                parser_error: false,
            },
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    // OIDC check
    if let Some(perms) = doc.get("permissions").and_then(|v| v.as_mapping()) {
        if !perms.contains_key(Value::String("id-token".to_string())) {
            findings.push(Finding {
                rule_id: "ci-missing-oidc".to_string(),
                status: Status::Failed,
                evidence: EvidenceRef {
                    path: path.to_string(),
                    line: 0,
                    parser_error: false,
                },
                matrix_source: "CI_POSTURE".to_string(),
            });
        }
    } else {
        findings.push(Finding {
            rule_id: "ci-missing-oidc".to_string(),
            status: Status::Failed,
            evidence: EvidenceRef {
                path: path.to_string(),
                line: 0,
                parser_error: false,
            },
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    // jobs unpinned actions
    if let Some(jobs) = doc.get("jobs").and_then(|v| v.as_mapping()) {
        for (_job_name, job) in jobs {
            if let Some(steps) = job.get("steps").and_then(|v| v.as_sequence()) {
                for step in steps {
                    if let Some(uses) = step.get("uses").and_then(|v| v.as_str()) {
                        // a simplistic check for unpinned @version vs @sha
                        if uses.contains('@')
                            && !uses.contains("@sha256:")
                            && uses.len() < uses.find('@').unwrap() + 40
                        {
                            findings.push(Finding {
                                rule_id: "ci-unpinned-action".to_string(),
                                status: Status::Failed,
                                evidence: EvidenceRef {
                                    path: path.to_string(),
                                    line: 0,
                                    parser_error: false,
                                },
                                matrix_source: "CI_POSTURE".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    findings
}
