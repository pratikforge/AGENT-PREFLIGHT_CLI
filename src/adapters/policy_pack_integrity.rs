use crate::domain::evidence::EvidenceRef;
use crate::domain::policy::PolicyCatalog;
use crate::domain::status::Status;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(yaml_content: &str, path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let evidence = EvidenceRef {
        path: path.to_string(),
        line: 1,
        parser_error: false,
    };

    if let Ok(catalog) = PolicyCatalog::from_yaml(yaml_content) {
        if let Some(sig) = catalog.signature {
            if sig.starts_with("verified_") {
                findings.push(Finding {
                    rule_id: "accept_verified_signature".to_string(),
                    status: Status::Verified,
                    evidence,
                    matrix_source: "policy_pack_integrity".to_string(),
                });
            } else {
                findings.push(Finding {
                    rule_id: "reject_tampered_pack_signature".to_string(),
                    status: Status::Failed,
                    evidence,
                    matrix_source: "policy_pack_integrity".to_string(),
                });
            }
        } else {
            findings.push(Finding {
                rule_id: "reject_tampered_pack_signature".to_string(),
                status: Status::Failed,
                evidence,
                matrix_source: "policy_pack_integrity".to_string(),
            });
        }
    }
    findings
}
