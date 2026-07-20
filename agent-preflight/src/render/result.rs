use serde::Serialize;

use crate::domain::evidence::EvidenceRef;
use crate::domain::status::Status;

#[derive(Serialize)]
pub struct ResultArtifact<'a> {
    pub schema_version: u32,
    pub profile: &'a str,
    pub status: Status,
    pub contract_revision_sha256: &'a str,
    pub approved_rule_id: &'a str,
    pub findings: &'a [ResultFinding],
}

#[derive(Serialize)]
pub struct ResultFinding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
}

pub fn to_yaml(result: &ResultArtifact<'_>) -> Result<String, serde_yaml_ng::Error> {
    serde_yaml_ng::to_string(result)
}
