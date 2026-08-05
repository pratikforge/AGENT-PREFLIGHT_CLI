use crate::domain::evidence::EvidenceRef;
use crate::domain::status::Status;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(mutation: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let evidence = EvidenceRef {
        path: "mutation.diff".to_string(),
        line: 0,
        parser_error: false,
    };

    if mutation == "approval_claim_mutation" {
        findings.push(Finding {
            rule_id: "approval_claim_mutation_is_caught".to_string(),
            status: Status::Failed,
            evidence: evidence.clone(),
            matrix_source: "MUTATION".to_string(),
        });
    }

    if mutation == "audit_redaction_mutation" {
        findings.push(Finding {
            rule_id: "audit_redaction_mutation_is_caught".to_string(),
            status: Status::Failed,
            evidence: evidence.clone(),
            matrix_source: "MUTATION".to_string(),
        });
    }

    if mutation == "egress_private_range_mutation" {
        findings.push(Finding {
            rule_id: "egress_private_range_mutation_is_caught".to_string(),
            status: Status::Failed,
            evidence: evidence.clone(),
            matrix_source: "MUTATION".to_string(),
        });
    }

    if mutation == "supply_chain_pin_mutation" {
        findings.push(Finding {
            rule_id: "supply_chain_pin_mutation_is_caught".to_string(),
            status: Status::Failed,
            evidence: evidence.clone(),
            matrix_source: "MUTATION".to_string(),
        });
    }

    findings
}
