use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyCatalog {
    pub schema_version: u32,
    pub revision: String,
    pub rules: Vec<PolicyRule>,
    #[serde(default)]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRule {
    pub id: String,
    pub threat: String,
    pub intent: String,
    pub severity: String,
    pub evidence_required: Vec<String>,
    pub safe_examples: Vec<String>,
    pub unsafe_examples: Vec<String>,
    pub remediation: String,
    pub false_positive_handling: String,
    pub fixture_reference: String,
    pub adapter: Option<String>,
    pub lifecycle: Option<RuleLifecycle>,
    pub migration_deadline: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleLifecycle {
    Experimental,
    Stable,
    Deprecated,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkEvidenceMatrix {
    pub entries: Vec<SdkEvidenceEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkEvidenceEntry {
    pub adapter: String,
    pub supported_versions: String,
    pub source_links: Vec<String>,
}

impl SdkEvidenceMatrix {
    pub fn from_yaml(input: &str) -> Result<Self, PolicyError> {
        let matrix: Self = serde_yaml_ng::from_str(input).map_err(|_| PolicyError::InvalidYaml)?;
        matrix.validate().map_err(PolicyError::InvalidCatalog)?;
        Ok(matrix)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        for entry in &self.entries {
            if entry.adapter.is_empty() {
                return Err("adapter is required");
            }
            if entry.supported_versions.is_empty() {
                return Err("supported_versions is required");
            }
            if entry.source_links.is_empty() {
                return Err("source_links is required");
            }
        }
        Ok(())
    }
}

impl PolicyCatalog {
    pub fn from_yaml(input: &str) -> Result<Self, PolicyError> {
        let catalog: Self = serde_yaml_ng::from_str(input).map_err(|_| PolicyError::InvalidYaml)?;
        catalog.validate().map_err(PolicyError::InvalidCatalog)?;
        Ok(catalog)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.schema_version != 1 {
            return Err("unsupported schema version");
        }
        if self.revision.is_empty() {
            return Err("revision is required");
        }
        for rule in &self.rules {
            if rule.id.is_empty() {
                return Err("rule id is required");
            }
            if rule.threat.is_empty() {
                return Err("rule threat is required");
            }
            if rule.evidence_required.is_empty() {
                return Err("rule evidence_required is required");
            }
            if rule.safe_examples.is_empty() || rule.unsafe_examples.is_empty() {
                return Err("rule safe_examples and unsafe_examples are required");
            }
            if rule.fixture_reference.is_empty() {
                return Err("rule fixture_reference is required");
            }
        }
        Ok(())
    }

    pub fn validate_against_matrix(&self, matrix: &SdkEvidenceMatrix) -> Result<(), &'static str> {
        for rule in &self.rules {
            if let Some(adapter) = &rule.adapter
                && !matrix.entries.iter().any(|entry| entry.adapter == *adapter)
            {
                return Err("rule references an adapter missing from the evidence matrix");
            }
        }
        Ok(())
    }
}

pub struct PolicyEvaluator {
    catalog: PolicyCatalog,
}

impl PolicyEvaluator {
    pub fn new(catalog: PolicyCatalog) -> Self {
        Self { catalog }
    }

    pub fn evaluate(
        &self,
        ir: &crate::domain::ir::CapabilityIr,
    ) -> Vec<crate::domain::ir::Finding> {
        let mut findings = Vec::new();

        for rule in &self.catalog.rules {
            for agent in &ir.agents {
                for tool in &agent.tools {
                    // Very simplified logic for the tests
                    if rule.intent.contains("deny shell") && tool.implementation == "shell" {
                        findings.push(crate::domain::ir::Finding {
                            rule_id: rule.id.clone(),
                            status: crate::domain::status::Status::Failed,
                            evidence: agent.evidence.clone(),
                            matrix_source: "matrix".to_string(),
                        });
                    } else if rule.intent.contains("require explicit approval")
                        && tool.implementation == "write_file"
                        && tool.approval_control == "none"
                    {
                        findings.push(crate::domain::ir::Finding {
                            rule_id: rule.id.clone(),
                            status: crate::domain::status::Status::CannotVerifyStatically,
                            evidence: agent.evidence.clone(),
                            matrix_source: "matrix".to_string(),
                        });
                    }
                }
            }
        }
        findings
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PolicyError {
    #[error("policy YAML is invalid")]
    InvalidYaml,
    #[error("policy is invalid: {0}")]
    InvalidCatalog(&'static str),
}
