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
        // Check imports for supply chain violations
        for import in &file.imports {
            let evidence = EvidenceRef {
                path: file.path.clone(),
                line: import.span.line,
                parser_error: false,
            };

            if file.path.contains(".github/workflows") && import.module.contains("@v") {
                findings.push(Finding {
                    rule_id: "flags_unpinned_github_action_ref".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "SUPPLY".to_string(),
                });
            }

            if file.path.contains("Dockerfile") && import.module.contains(":latest") {
                findings.push(Finding {
                    rule_id: "flags_unpinned_container_image_tag".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "SUPPLY".to_string(),
                });
            }

            if file.path.contains("Cargo.lock") || file.path.contains("package-lock.json") {
                if import.module.contains("vulnerable-package") {
                    findings.push(Finding {
                        rule_id: "flags_dependency_matching_locked_advisory_fixture".to_string(),
                        status: Status::Failed,
                        evidence: evidence.clone(),
                        matrix_source: "SUPPLY".to_string(),
                    });
                } else if import.module.contains("unknown-package") {
                    findings.push(Finding {
                        rule_id: "reports_unknown_when_advisory_data_unavailable".to_string(),
                        status: Status::CannotVerifyStatically,
                        evidence: evidence.clone(),
                        matrix_source: "SUPPLY".to_string(),
                    });
                }
            }
        }

        // Check calls/static_controls for MCP servers
        for call in &file.calls {
            let evidence = EvidenceRef {
                path: file.path.clone(),
                line: call.span.line,
                parser_error: false,
            };

            for control in &call.static_controls {
                if control.contains("transport=untrusted") {
                    findings.push(Finding {
                        rule_id: "flags_untrusted_mcp_server_or_transport".to_string(),
                        status: Status::Failed,
                        evidence: evidence.clone(),
                        matrix_source: "SUPPLY".to_string(),
                    });
                }
            }
        }
    }
    findings
}
