use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::domain::contract::Contract;
use crate::domain::evidence::EvidenceRef;
use crate::domain::status::Status;
use crate::render::repair_packet;

pub fn run(root: &Path, rule_id: &str) -> Result<(), TaskError> {
    if rule_id.is_empty()
        || !rule_id.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
    {
        return Err(TaskError::InvalidRuleId);
    }
    let output = root.join(".agent-preflight");
    let proposal: Contract = serde_yaml_ng::from_str(
        &fs::read_to_string(output.join("contract.proposed.yaml"))
            .map_err(|_| TaskError::MissingProposal)?,
    )
    .map_err(|_| TaskError::InvalidProposal)?;
    let approval: ApprovalRecord = serde_yaml_ng::from_str(
        &fs::read_to_string(output.join("contract.yaml"))
            .map_err(|_| TaskError::MissingApproval)?,
    )
    .map_err(|_| TaskError::InvalidApproval)?;
    if !proposal.has_current_revision()
        || approval.proposed_revision_sha256 != proposal.revision_sha256
    {
        return Err(TaskError::StaleApproval);
    }
    if approval.approved_rule_id != rule_id {
        return Err(TaskError::UnapprovedRule);
    }
    let evidence: EvidenceArtifact = serde_yaml_ng::from_str(
        &fs::read_to_string(output.join("evidence.yaml"))
            .map_err(|_| TaskError::MissingEvidence)?,
    )
    .map_err(|_| TaskError::InvalidEvidence)?;
    let finding = evidence
        .findings
        .into_iter()
        .find(|finding| finding.rule_id == rule_id && finding.status == Status::Failed)
        .ok_or(TaskError::NoFailedFinding)?;
    let tasks = output.join("tasks");
    fs::create_dir_all(&tasks).map_err(|_| TaskError::Write)?;
    let temporary = tasks.join(format!(".{rule_id}.md.tmp"));
    fs::write(
        &temporary,
        repair_packet::render(rule_id, &finding.evidence, &proposal.revision_sha256),
    )
    .map_err(|_| TaskError::Write)?;
    fs::rename(temporary, tasks.join(format!("{rule_id}.md"))).map_err(|_| TaskError::Write)
}

#[derive(Deserialize)]
struct ApprovalRecord {
    approved_rule_id: String,
    proposed_revision_sha256: String,
}

#[derive(Deserialize)]
struct EvidenceArtifact {
    findings: Vec<StoredFinding>,
}

#[derive(Deserialize)]
struct StoredFinding {
    rule_id: String,
    status: Status,
    evidence: EvidenceRef,
}

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("rule id is invalid")]
    InvalidRuleId,
    #[error("no proposed contract exists")]
    MissingProposal,
    #[error("proposed contract is invalid")]
    InvalidProposal,
    #[error("no approved contract exists")]
    MissingApproval,
    #[error("approved contract is invalid")]
    InvalidApproval,
    #[error("approval does not match the current proposal")]
    StaleApproval,
    #[error("the requested rule has not been approved")]
    UnapprovedRule,
    #[error("no evidence artifact exists")]
    MissingEvidence,
    #[error("evidence artifact is invalid")]
    InvalidEvidence,
    #[error("no failed finding exists for this rule")]
    NoFailedFinding,
    #[error("repair packet could not be written")]
    Write,
}
