use std::fs;
use std::path::Path;

use crate::domain::contract::Contract;

pub fn run(root: &Path) -> Result<String, ReviewError> {
    let path = root.join(".agent-preflight").join("contract.proposed.yaml");
    let input = fs::read_to_string(path).map_err(|_| ReviewError::MissingProposal)?;
    let contract = Contract::from_yaml(&input).map_err(|_| ReviewError::InvalidProposal)?;
    let rules = contract
        .rules
        .iter()
        .map(|rule| format!("- `{}`: {}\n", rule.id, rule.intended_capability))
        .collect::<String>();
    Ok(format!(
        "# Pending capability rules\n\nRevision: `{}`\n\n{rules}Approve one rule explicitly with `agent-preflight approve <repository-path> <rule-id>`.\n",
        contract.revision_sha256
    ))
}

#[derive(Debug, thiserror::Error)]
pub enum ReviewError {
    #[error("no proposed contract exists; run scan first")]
    MissingProposal,
    #[error("proposed contract is invalid")]
    InvalidProposal,
}
