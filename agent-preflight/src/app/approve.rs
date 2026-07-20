use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::domain::contract::{Contract, SCHEMA_VERSION};

pub fn run(root: &Path, rule_id: &str) -> Result<(), ApproveError> {
    if rule_id.is_empty()
        || !rule_id.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
    {
        return Err(ApproveError::InvalidRuleId);
    }
    let output = root.join(".agent-preflight");
    let proposal = fs::read_to_string(output.join("contract.proposed.yaml"))
        .map_err(|_| ApproveError::MissingProposal)?;
    let contract = Contract::from_yaml(&proposal).map_err(|_| ApproveError::InvalidProposal)?;
    if !contract.has_current_revision() {
        return Err(ApproveError::StaleProposal);
    }
    if !contract.rules.iter().any(|rule| rule.id == rule_id) {
        return Err(ApproveError::UnknownRule);
    }
    let approved_path = output.join("contract.yaml");
    if let Ok(existing) = fs::read_to_string(&approved_path) {
        let existing: ApprovedContract =
            serde_yaml_ng::from_str(&existing).map_err(|_| ApproveError::InvalidApproval)?;
        if existing.approved_rule_id == rule_id
            && existing.proposed_revision_sha256 == contract.revision_sha256
        {
            return Err(ApproveError::RepeatedApproval);
        }
    }
    let approved = ApprovedContract {
        schema_version: SCHEMA_VERSION,
        approved_rule_id: rule_id.to_owned(),
        proposed_revision_sha256: contract.revision_sha256.clone(),
        contract,
    };
    let content = serde_yaml_ng::to_string(&approved).map_err(|_| ApproveError::Serialize)?;
    let temporary = output.join(".contract.yaml.tmp");
    fs::write(&temporary, content).map_err(|_| ApproveError::Write)?;
    fs::rename(temporary, approved_path).map_err(|_| ApproveError::Write)
}

#[derive(Serialize, Deserialize)]
struct ApprovedContract {
    schema_version: u32,
    approved_rule_id: String,
    proposed_revision_sha256: String,
    contract: Contract,
}

#[derive(Debug, thiserror::Error)]
pub enum ApproveError {
    #[error("rule id is invalid")]
    InvalidRuleId,
    #[error("no proposed contract exists; run scan first")]
    MissingProposal,
    #[error("proposed contract is invalid")]
    InvalidProposal,
    #[error("proposed contract revision is stale")]
    StaleProposal,
    #[error("rule id is not present in the proposal")]
    UnknownRule,
    #[error("existing approval artifact is invalid")]
    InvalidApproval,
    #[error("rule is already approved for this proposal revision")]
    RepeatedApproval,
    #[error("approved contract could not be serialized")]
    Serialize,
    #[error("approved contract could not be written")]
    Write,
}
