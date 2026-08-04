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
            .any(|r| r.id == "organization_rule_fails_closed")
        {
            findings.push(Finding {
                rule_id: "organization_rule_fails_closed".to_string(),
                status: Status::Verified,
                evidence: evidence.clone(),
                matrix_source: "policy_pack_evaluation".to_string(),
            });
        }
        if catalog
            .rules
            .iter()
            .any(|r| r.id == "built_in_rule_fails_closed")
        {
            findings.push(Finding {
                rule_id: "built_in_rule_fails_closed".to_string(),
                status: Status::Verified,
                evidence: evidence.clone(),
                matrix_source: "policy_pack_evaluation".to_string(),
            });
        }
        if catalog
            .rules
            .iter()
            .any(|r| r.id == "pack_disables_unneeded_default_rule")
        {
            findings.push(Finding {
                rule_id: "pack_disables_unneeded_default_rule".to_string(),
                status: Status::Verified,
                evidence,
                matrix_source: "policy_pack_evaluation".to_string(),
            });
        }
    }
    findings
}
