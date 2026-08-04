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
        if catalog
            .rules
            .iter()
            .any(|r| r.id == "repository_policy_overrides_pack_policy")
        {
            findings.push(Finding {
                rule_id: "repository_policy_overrides_pack_policy".to_string(),
                status: Status::Verified,
                evidence: evidence.clone(),
                matrix_source: "policy_pack_precedence".to_string(),
            });
        }
        if catalog
            .rules
            .iter()
            .any(|r| r.id == "pack_overrides_built_in_policy")
        {
            findings.push(Finding {
                rule_id: "pack_overrides_built_in_policy".to_string(),
                status: Status::Verified,
                evidence,
                matrix_source: "policy_pack_precedence".to_string(),
            });
        }
    }
    findings
}
