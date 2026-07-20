use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub intended_capability: String,
    pub risk_tier: String,
    pub approval_requirement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contract {
    pub schema_version: u32,
    pub profile: String,
    pub rules: Vec<Rule>,
    pub revision_sha256: String,
}

impl Contract {
    /// Parses a contract as data and validates its schema and required fields.
    pub fn from_yaml(input: &str) -> Result<Self, ContractError> {
        let contract: Self =
            serde_yaml_ng::from_str(input).map_err(|_| ContractError::InvalidYaml)?;
        contract
            .validate()
            .map_err(ContractError::InvalidContract)?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.schema_version != SCHEMA_VERSION {
            return Err("unsupported schema version");
        }
        if self.profile.is_empty() || self.rules.is_empty() {
            return Err("profile and rules are required");
        }
        if self.rules.iter().any(|rule| rule.id.is_empty()) {
            return Err("rule ids are required");
        }
        Ok(())
    }

    pub fn canonical_hash(&self) -> String {
        let mut copy = self.clone();
        copy.revision_sha256.clear();
        let canonical = serde_json::to_vec(&copy).expect("serializable contract");
        Sha256::digest(canonical)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }

    pub fn has_current_revision(&self) -> bool {
        self.revision_sha256 == self.canonical_hash()
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ContractError {
    #[error("contract YAML is invalid")]
    InvalidYaml,
    #[error("contract is invalid: {0}")]
    InvalidContract(&'static str),
}
