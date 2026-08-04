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

    let spec = doc.get("spec").and_then(|s| s.as_mapping());
    if let Some(spec) = spec {
        // hostNetwork
        if let Some(host_net) = spec.get("hostNetwork").and_then(|v| v.as_bool())
            && host_net
        {
            findings.push(Finding {
                rule_id: "k8s-host-network".to_string(),
                status: Status::Failed,
                evidence: EvidenceRef {
                    path: path.to_string(),
                    line: 0,
                    parser_error: false,
                },
                matrix_source: "K8S_POSTURE".to_string(),
            });
        }

        // containers
        if let Some(containers) = spec.get("containers").and_then(|v| v.as_sequence()) {
            for container in containers {
                let sec_ctx = container
                    .get("securityContext")
                    .and_then(|s| s.as_mapping());

                let mut privileged = false;
                let mut read_only_root = false;

                if let Some(ctx) = sec_ctx {
                    privileged = ctx
                        .get("privileged")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    read_only_root = ctx
                        .get("readOnlyRootFilesystem")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                }

                if privileged {
                    findings.push(Finding {
                        rule_id: "k8s-privileged".to_string(),
                        status: Status::Failed,
                        evidence: EvidenceRef {
                            path: path.to_string(),
                            line: 0,
                            parser_error: false,
                        },
                        matrix_source: "K8S_POSTURE".to_string(),
                    });
                }

                if !read_only_root {
                    findings.push(Finding {
                        rule_id: "k8s-writable-root".to_string(),
                        status: Status::Failed,
                        evidence: EvidenceRef {
                            path: path.to_string(),
                            line: 0,
                            parser_error: false,
                        },
                        matrix_source: "K8S_POSTURE".to_string(),
                    });
                }

                // seccomp
                let seccomp = container
                    .get("securityContext")
                    .and_then(|s| s.get("seccompProfile"));
                if seccomp.is_none() {
                    findings.push(Finding {
                        rule_id: "k8s-missing-seccomp".to_string(),
                        status: Status::Failed,
                        evidence: EvidenceRef {
                            path: path.to_string(),
                            line: 0,
                            parser_error: false,
                        },
                        matrix_source: "K8S_POSTURE".to_string(),
                    });
                }
            }
        }
    }

    findings
}
