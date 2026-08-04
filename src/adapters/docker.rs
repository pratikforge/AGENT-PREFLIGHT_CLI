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

pub fn evaluate(compose_yaml: &str, path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let docs = match serde_yaml_ng::from_str::<Value>(compose_yaml) {
        Ok(v) => v,
        Err(_) => return findings,
    };

    let services = docs.get("services").and_then(|s| s.as_mapping());
    if let Some(services) = services {
        for (_name, service) in services {
            // Check user
            let user = service.get("user").and_then(|u| u.as_str());
            let status = match user {
                Some("root") | None => Status::Failed,
                Some(_) => Status::Verified,
            };
            findings.push(Finding {
                rule_id: "docker-root-user".to_string(),
                status,
                evidence: EvidenceRef {
                    path: path.to_string(),
                    line: 0,
                    parser_error: false,
                },
                matrix_source: "DOCKER_POSTURE".to_string(),
            });

            // Check cap_add
            if service.get("cap_add").is_some() {
                findings.push(Finding {
                    rule_id: "docker-cap-add".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: path.to_string(),
                        line: 0,
                        parser_error: false,
                    },
                    matrix_source: "DOCKER_POSTURE".to_string(),
                });
            }

            // Check network_mode
            if let Some(net) = service.get("network_mode").and_then(|n| n.as_str())
                && net == "host"
            {
                findings.push(Finding {
                    rule_id: "docker-network-host".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: path.to_string(),
                        line: 0,
                        parser_error: false,
                    },
                    matrix_source: "DOCKER_POSTURE".to_string(),
                });
            }
        }
    }

    findings
}
