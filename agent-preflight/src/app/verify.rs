use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::adapters::Profile;
use crate::app::scan;
use crate::domain::contract::Contract;
use crate::domain::evidence::EvidenceRef;
use crate::domain::status::Status;
use crate::render::result::{self, ResultArtifact, ResultFinding};

pub fn run(root: &Path) -> Result<VerificationResult, VerifyError> {
    let scan_result = scan::run(root).map_err(VerifyError::Scan)?;
    let output = root.join(".agent-preflight");
    if scan_result.has_parse_error {
        return write_result(
            root,
            scan_result.profile.label(),
            Status::CannotVerifyStatically,
            "",
            "",
            Vec::new(),
        );
    }
    if scan_result.profile == Profile::Unsupported {
        return write_result(root, "unsupported", Status::Unsupported, "", "", Vec::new());
    }

    let proposal: Contract = serde_yaml_ng::from_str(
        &fs::read_to_string(output.join("contract.proposed.yaml"))
            .map_err(|_| VerifyError::MissingProposal)?,
    )
    .map_err(|_| VerifyError::InvalidProposal)?;
    if !proposal.has_current_revision() {
        return Err(VerifyError::StaleProposal);
    }
    let approval: ApprovalRecord = serde_yaml_ng::from_str(
        &fs::read_to_string(output.join("contract.yaml"))
            .map_err(|_| VerifyError::MissingApproval)?,
    )
    .map_err(|_| VerifyError::InvalidApproval)?;
    if approval.proposed_revision_sha256 != proposal.revision_sha256 {
        return Err(VerifyError::StaleApproval);
    }
    if !proposal
        .rules
        .iter()
        .any(|rule| rule.id == approval.approved_rule_id)
    {
        return Err(VerifyError::InvalidApproval);
    }

    let evidence: EvidenceArtifact = serde_yaml_ng::from_str(
        &fs::read_to_string(output.join("evidence.yaml"))
            .map_err(|_| VerifyError::MissingEvidence)?,
    )
    .map_err(|_| VerifyError::InvalidEvidence)?;
    let findings = evidence
        .findings
        .into_iter()
        .map(|finding| ResultFinding {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence,
        })
        .collect::<Vec<_>>();
    let status = aggregate_status(&findings);
    write_result(
        root,
        &evidence.profile,
        status,
        &proposal.revision_sha256,
        &approval.approved_rule_id,
        findings,
    )
}

pub fn ci_exit_code(status: Status) -> i32 {
    match status {
        Status::Verified => 0,
        Status::Failed => 1,
        Status::Unsupported => 3,
        Status::Partial | Status::CannotVerifyStatically => 4,
    }
}

fn aggregate_status(findings: &[ResultFinding]) -> Status {
    if findings
        .iter()
        .any(|finding| finding.status == Status::Failed)
    {
        Status::Failed
    } else if findings
        .iter()
        .any(|finding| finding.status == Status::CannotVerifyStatically)
    {
        Status::CannotVerifyStatically
    } else if findings
        .iter()
        .any(|finding| finding.status == Status::Partial)
    {
        Status::Partial
    } else {
        Status::Verified
    }
}

fn write_result(
    root: &Path,
    profile: &str,
    status: Status,
    revision: &str,
    approved_rule_id: &str,
    findings: Vec<ResultFinding>,
) -> Result<VerificationResult, VerifyError> {
    let result = VerificationResult { status };
    let output = root.join(".agent-preflight");
    let artifact = ResultArtifact {
        schema_version: crate::domain::contract::SCHEMA_VERSION,
        profile,
        status,
        contract_revision_sha256: revision,
        approved_rule_id,
        findings: &findings,
    };
    let yaml = result::to_yaml(&artifact).map_err(|_| VerifyError::Serialize)?;
    let temporary = output.join(".result.yaml.tmp");
    fs::write(&temporary, yaml).map_err(|_| VerifyError::Write)?;
    fs::rename(temporary, output.join("result.yaml")).map_err(|_| VerifyError::Write)?;
    Ok(result)
}

#[derive(Debug, Clone, Copy)]
pub struct VerificationResult {
    pub status: Status,
}

#[derive(Deserialize)]
struct ApprovalRecord {
    approved_rule_id: String,
    proposed_revision_sha256: String,
}

#[derive(Deserialize)]
struct EvidenceArtifact {
    profile: String,
    findings: Vec<StoredFinding>,
}

#[derive(Deserialize)]
struct StoredFinding {
    rule_id: String,
    status: Status,
    evidence: EvidenceRef,
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("repository input could not be verified: {0}")]
    Scan(scan::ScanError),
    #[error("no proposed contract exists")]
    MissingProposal,
    #[error("proposed contract is invalid")]
    InvalidProposal,
    #[error("proposed contract revision is stale")]
    StaleProposal,
    #[error("no approved contract exists")]
    MissingApproval,
    #[error("approved contract is invalid")]
    InvalidApproval,
    #[error("approved contract does not match the current proposal")]
    StaleApproval,
    #[error("no evidence artifact exists")]
    MissingEvidence,
    #[error("evidence artifact is invalid")]
    InvalidEvidence,
    #[error("result could not be serialized")]
    Serialize,
    #[error("result could not be written")]
    Write,
}
