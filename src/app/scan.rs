use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::adapters::{self, Profile};
use crate::domain::contract::{Contract, Rule, SCHEMA_VERSION};
use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::{NormalizedFile, ParserState};
use crate::infra::artifacts;
use crate::infra::parser::normalize;
use crate::infra::safe_reader::SafeReader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub profile: Profile,
    pub files_scanned: usize,
    pub has_parse_error: bool,
}

pub fn run(root: &Path) -> Result<ScanResult, ScanError> {
    let sources = SafeReader.read(root).map_err(ScanError::Read)?;
    let files: Vec<_> = sources.iter().map(normalize).collect();
    let has_parse_error = files
        .iter()
        .any(|file| file.parser_state == ParserState::ParseError);
    let profile = adapters::detect(&files);
    let findings = match profile {
        Profile::OpenAiAgents => crate::adapters::openai_agents::evaluate(&files)
            .into_iter()
            .map(ScanFinding::from_openai)
            .collect(),
        Profile::GoogleAdk => crate::adapters::google_adk::evaluate(&files)
            .into_iter()
            .map(ScanFinding::from_google)
            .collect(),
        Profile::ClaudeAgentSdk => crate::adapters::claude_agent::evaluate(&files)
            .into_iter()
            .map(ScanFinding::from_claude)
            .collect(),
        Profile::Unsupported => Vec::new(),
    };
    let evidence = EvidenceArtifact::from_files(profile, &files, findings.clone());
    let contract = proposed_contract(profile, &findings);
    let evidence_yaml = serde_yaml_ng::to_string(&evidence).map_err(|_| ScanError::Serialize)?;
    let contract_yaml = serde_yaml_ng::to_string(&contract).map_err(|_| ScanError::Serialize)?;
    let report = report(profile, files.len(), &findings);
    artifacts::write_all(
        root,
        [
            ("evidence.yaml", evidence_yaml),
            ("contract.proposed.yaml", contract_yaml),
            ("report.md", report),
        ],
    )
    .map_err(ScanError::Write)?;
    Ok(ScanResult {
        profile,
        files_scanned: files.len(),
        has_parse_error,
    })
}

fn proposed_contract(profile: Profile, findings: &[ScanFinding]) -> Contract {
    let mut rules = vec![Rule {
        id: "static-review-required".to_owned(),
        intended_capability: "Review detected agent capabilities before approval".to_owned(),
        risk_tier: "unknown".to_owned(),
        approval_requirement: "proposed; owner review required".to_owned(),
    }];
    for finding in findings {
        if rules.iter().any(|rule| rule.id == finding.rule_id) {
            continue;
        }
        rules.push(Rule {
            id: finding.rule_id.clone(),
            intended_capability: "Resolve the cited static finding without executing code"
                .to_owned(),
            risk_tier: "unknown".to_owned(),
            approval_requirement: "proposed; owner approval required before a repair packet"
                .to_owned(),
        });
    }
    let mut contract = Contract {
        schema_version: SCHEMA_VERSION,
        profile: profile.label().to_owned(),
        rules,
        revision_sha256: String::new(),
    };
    contract.revision_sha256 = contract.canonical_hash();
    contract
}

fn report(profile: Profile, files_scanned: usize, findings: &[ScanFinding]) -> String {
    let mut counts = BTreeMap::new();
    for finding in findings {
        *counts
            .entry((finding.rule_id.as_str(), status_label(finding.status)))
            .or_insert(0_usize) += 1;
    }
    let finding_lines = counts
        .into_iter()
        .map(|((rule_id, status), count)| format!("- `{rule_id}`: {status} ({count})\n"))
        .collect::<String>();
    format!(
        "# Agent Preflight scan\n\n- Profile: `{}`\n- Source files scanned: {files_scanned}\n- Result: proposed contract only; no approval or source modification occurred.\n{finding_lines}",
        profile.label(),
    )
}

fn status_label(status: crate::domain::status::Status) -> &'static str {
    match status {
        crate::domain::status::Status::Verified => "verified",
        crate::domain::status::Status::Failed => "failed",
        crate::domain::status::Status::Partial => "partial",
        crate::domain::status::Status::CannotVerifyStatically => "cannot_verify_statically",
        crate::domain::status::Status::Unsupported => "unsupported",
    }
}

#[derive(Serialize)]
struct EvidenceArtifact {
    schema_version: u32,
    profile: String,
    parse_errors: Vec<ParseErrorEvidence>,
    findings: Vec<ScanFinding>,
}

impl EvidenceArtifact {
    fn from_files(profile: Profile, files: &[NormalizedFile], findings: Vec<ScanFinding>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            profile: profile.label().to_owned(),
            parse_errors: files
                .iter()
                .filter(|file| file.parser_state == ParserState::ParseError)
                .map(ParseErrorEvidence::from)
                .collect(),
            findings,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct ScanFinding {
    rule_id: String,
    status: crate::domain::status::Status,
    evidence: EvidenceRef,
    matrix_source: String,
}

impl ScanFinding {
    fn from_openai(finding: crate::adapters::openai_agents::Finding) -> Self {
        Self {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence,
            matrix_source: finding.matrix_source,
        }
    }

    fn from_google(finding: crate::adapters::google_adk::Finding) -> Self {
        Self {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence,
            matrix_source: finding.matrix_source,
        }
    }

    fn from_claude(finding: crate::adapters::claude_agent::Finding) -> Self {
        Self {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence,
            matrix_source: finding.matrix_source,
        }
    }
}

#[derive(Serialize)]
struct ParseErrorEvidence {
    path: String,
    parser_state: ParserState,
}

impl From<&NormalizedFile> for ParseErrorEvidence {
    fn from(file: &NormalizedFile) -> Self {
        Self {
            path: file.path.clone(),
            parser_state: file.parser_state,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("repository input could not be scanned: {0}")]
    Read(crate::infra::safe_reader::ReaderError),
    #[error("scan artifacts could not be serialized")]
    Serialize,
    #[error("scan artifacts could not be written: {0}")]
    Write(artifacts::ArtifactError),
}
