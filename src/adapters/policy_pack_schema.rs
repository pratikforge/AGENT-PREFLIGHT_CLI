use crate::domain::evidence::EvidenceRef;
use crate::domain::policy::{PolicyCatalog, PolicyError};
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

    match PolicyCatalog::from_yaml(yaml_content) {
        Ok(_) => {
            findings.push(Finding {
                rule_id: "valid_organization_policy_pack".to_string(),
                status: Status::Verified,
                evidence,
                matrix_source: "policy_pack_schema".to_string(),
            });
        }
        Err(PolicyError::InvalidCatalog("unsupported schema version")) => {
            findings.push(Finding {
                rule_id: "unsupported_threat_model_version".to_string(),
                status: Status::Verified,
                evidence,
                matrix_source: "policy_pack_schema".to_string(),
            });
        }
        Err(_) => {
            findings.push(Finding {
                rule_id: "invalid_schema".to_string(),
                status: Status::Verified,
                evidence,
                matrix_source: "policy_pack_schema".to_string(),
            });
        }
    }
    findings
}



