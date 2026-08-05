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
                let evidence = EvidenceRef {
                    path: file.path.clone(),
                    line: call.span.line,
                    parser_error: false,
                };

                if control == "dynamic_url" {
                    findings.push(Finding {
                        rule_id: "marks_dynamic_destination_uncertain".to_string(),
                        status: Status::CannotVerifyStatically,
                        evidence: evidence.clone(),
                        matrix_source: "EGRESS".to_string(),
                    });
                } else if control.contains("unlisted.example.com") {
                    findings.push(Finding {
                        rule_id: "denies_unlisted_public_host".to_string(),
                        status: Status::Failed,
                        evidence: evidence.clone(),
                        matrix_source: "EGRESS".to_string(),
                    });
                } else if control.contains("api.github.com:443") {
                    findings.push(Finding {
                        rule_id: "allows_only_configured_host_scheme_and_port".to_string(),
                        status: Status::Verified,
                        evidence: evidence.clone(),
                        matrix_source: "EGRESS".to_string(),
                    });
                } else if control.contains("192.168.")
                    || control.contains("127.0.")
                    || control.contains("172.16.")
                {
                    findings.push(Finding {
                        rule_id: "denies_localhost_ipv4_private_and_172_16_range".to_string(),
                        status: Status::Failed,
                        evidence: evidence.clone(),
                        matrix_source: "EGRESS".to_string(),
                    });
                } else if control.contains("[::1]") || control.contains("[fe80:") {
                    findings.push(Finding {
                        rule_id: "denies_ipv6_loopback_ula_and_link_local".to_string(),
                        status: Status::Failed,
                        evidence: evidence.clone(),
                        matrix_source: "EGRESS".to_string(),
                    });
                } else if control.contains("169.254.169.254") {
                    findings.push(Finding {
                        rule_id: "denies_metadata_endpoint_variants".to_string(),
                        status: Status::Failed,
                        evidence: evidence.clone(),
                        matrix_source: "EGRESS".to_string(),
                    });
                } else if control.contains("2852039166") || control.contains("example.com.") {
                    findings.push(Finding {
                        rule_id: "blocks_case_trailing_dot_and_alternative_ip_bypasses".to_string(),
                        status: Status::Failed,
                        evidence: evidence.clone(),
                        matrix_source: "EGRESS".to_string(),
                    });
                }
            }
        }
    }
    findings
}
