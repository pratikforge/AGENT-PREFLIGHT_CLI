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
    let mut files: Vec<_> = sources.iter().map(normalize).collect();
    let has_parse_error = files
        .iter()
        .any(|file| file.parser_state == ParserState::ParseError);
    crate::app::resolve::resolve_symbols(&mut files);
    crate::app::resolve::resolve_constants(&mut files);
    crate::app::resolve::resolve_wrappers(&mut files, 3);
    let profile = adapters::detect(&files);
    let policy_path = root.join("agent-preflight.rules.yaml");
    let (policy_revision, policy_catalog, policy_content) = if policy_path.exists() {
        let content = std::fs::read_to_string(&policy_path).unwrap_or_default();
        if let Ok(catalog) = crate::domain::policy::PolicyCatalog::from_yaml(&content) {
            (catalog.revision.clone(), Some(catalog), Some(content))
        } else {
            ("v1.0.0".to_string(), None, Some(content))
        }
    } else {
        ("v1.0.0".to_string(), None, None)
    };

    let ir = match profile {
        Profile::OpenAiAgents => crate::adapters::openai_agents::to_ir(&files),
        Profile::GoogleAdk => crate::adapters::google_adk::to_ir(&files),
        Profile::Gemini => crate::adapters::gemini_api::to_ir(&files),
        Profile::ClaudeAgentSdk => crate::adapters::claude_agent::to_ir(&files),
        Profile::Unsupported => crate::domain::ir::CapabilityIr::default(),
    };

    let mut findings: Vec<ScanFinding> = match profile {
        Profile::OpenAiAgents => crate::adapters::openai_agents::evaluate(&files)
            .into_iter()
            .map(|f| ScanFinding::from_openai(f, &policy_revision))
            .collect(),
        Profile::GoogleAdk => crate::adapters::google_adk::evaluate(&files)
            .into_iter()
            .map(|f| ScanFinding::from_google(f, &policy_revision))
            .collect(),
        Profile::Gemini => crate::adapters::gemini_api::evaluate(&files)
            .into_iter()
            .map(|f| ScanFinding::from_gemini(f, &policy_revision))
            .collect(),
        Profile::ClaudeAgentSdk => crate::adapters::claude_agent::evaluate(&files)
            .into_iter()
            .map(|f| ScanFinding::from_claude(f, &policy_revision))
            .collect(),
        Profile::Unsupported => Vec::new(),
    };

    for source in &sources {
        let pi_findings = crate::adapters::prompt_injection::evaluate(&files);
        for f in pi_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }

        let egress_findings = crate::adapters::network_egress::evaluate(&files);
        for f in egress_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }

        let supply_findings = crate::adapters::supply_chain::evaluate(&files);
        for f in supply_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }
        let unsafe_findings = crate::adapters::unsafe_actions::evaluate(&files);
        for f in unsafe_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }

        let secrets_findings = crate::adapters::secrets_scanning::evaluate(&files);
        for f in secrets_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }

        let taint_findings = crate::adapters::taint_analysis::evaluate(&files);
        for f in taint_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }
        if source.language_hint == crate::domain::source::LanguageHint::Yaml {
            if source.path.contains(".github/workflows/") {
                let ci_findings = crate::adapters::ci::evaluate(&source.content, &source.path);
                for f in ci_findings {
                    findings.push(ScanFinding::from_generic(
                        f.rule_id,
                        f.status,
                        f.evidence,
                        f.matrix_source,
                        &policy_revision,
                    ));
                }
            }
            if source.path.contains("docker-compose") || source.path.contains("compose.y") {
                let docker_findings =
                    crate::adapters::docker::evaluate(&source.content, &source.path);
                for f in docker_findings {
                    findings.push(ScanFinding::from_generic(
                        f.rule_id,
                        f.status,
                        f.evidence,
                        f.matrix_source,
                        &policy_revision,
                    ));
                }
            }

            let k8s_findings = crate::adapters::kubernetes::evaluate(&source.content, &source.path);
            for f in k8s_findings {
                findings.push(ScanFinding::from_generic(
                    f.rule_id,
                    f.status,
                    f.evidence,
                    f.matrix_source,
                    &policy_revision,
                ));
            }
        }
    }

    if let Some(content) = &policy_content {
        let path_str = policy_path.to_string_lossy().to_string();

        let schema_findings = crate::adapters::policy_pack_schema::evaluate(content, &path_str);
        for f in schema_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }
        let integrity_findings =
            crate::adapters::policy_pack_integrity::evaluate(content, &path_str);
        for f in integrity_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }
        let precedence_findings =
            crate::adapters::policy_pack_precedence::evaluate(content, &path_str);
        for f in precedence_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }
        let eval_findings = crate::adapters::policy_pack_evaluation::evaluate(content, &path_str);
        for f in eval_findings {
            findings.push(ScanFinding::from_generic(
                f.rule_id,
                f.status,
                f.evidence,
                f.matrix_source,
                &policy_revision,
            ));
        }
    }

    if let Some(catalog) = policy_catalog {
        let evaluator = crate::domain::policy::PolicyEvaluator::new(catalog);
        let eval_findings = evaluator.evaluate(&ir);
        for f in eval_findings {
            let mut refs = Vec::new();
            if !f.evidence.origin.is_empty() {
                refs.push(crate::domain::evidence::EvidenceRef {
                    path: f.evidence.origin.clone(),
                    line: 1, // default line
                    parser_error: false,
                });
            }
            findings.push(ScanFinding {
                rule_id: f.rule_id,
                status: f.status,
                evidence: crate::domain::evidence::EvidenceRef {
                    path: f.evidence.origin.clone(),
                    line: 1,
                    parser_error: false,
                },
                matrix_source: f.matrix_source,
                policy_revision: policy_revision.clone(),
                provenance: vec![f.evidence],
            });
        }
    }

    let evidence = EvidenceArtifact::from_files(profile, &files, findings.clone());

    let contract = proposed_contract(profile, &findings, &policy_revision);
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

fn proposed_contract(
    profile: Profile,
    findings: &[ScanFinding],
    policy_revision: &str,
) -> Contract {
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
        policy_revision: policy_revision.to_owned(),
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
    policy_revision: String,
    provenance: Vec<crate::domain::ir::EvidenceNode>,
}

impl ScanFinding {
    fn from_openai(
        finding: crate::adapters::openai_agents::Finding,
        policy_revision: &str,
    ) -> Self {
        Self {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence.clone(),
            matrix_source: finding.matrix_source,
            policy_revision: policy_revision.to_string(),
            provenance: vec![crate::domain::ir::EvidenceNode {
                origin: finding.evidence.path.clone(),
                refs: vec![finding.evidence.clone()],
            }],
        }
    }

    fn from_google(finding: crate::adapters::google_adk::Finding, policy_revision: &str) -> Self {
        Self {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence.clone(),
            matrix_source: finding.matrix_source,
            policy_revision: policy_revision.to_string(),
            provenance: vec![crate::domain::ir::EvidenceNode {
                origin: finding.evidence.path.clone(),
                refs: vec![finding.evidence.clone()],
            }],
        }
    }

    fn from_gemini(finding: crate::adapters::gemini_api::Finding, policy_revision: &str) -> Self {
        Self {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence.clone(),
            matrix_source: finding.matrix_source,
            policy_revision: policy_revision.to_string(),
            provenance: vec![crate::domain::ir::EvidenceNode {
                origin: finding.evidence.path.clone(),
                refs: vec![finding.evidence.clone()],
            }],
        }
    }

    fn from_claude(finding: crate::adapters::claude_agent::Finding, policy_revision: &str) -> Self {
        Self {
            rule_id: finding.rule_id,
            status: finding.status,
            evidence: finding.evidence.clone(),
            matrix_source: finding.matrix_source,
            policy_revision: policy_revision.to_string(),
            provenance: vec![crate::domain::ir::EvidenceNode {
                origin: finding.evidence.path.clone(),
                refs: vec![finding.evidence.clone()],
            }],
        }
    }
    fn from_generic(
        rule_id: String,
        status: crate::domain::status::Status,
        evidence: EvidenceRef,
        matrix_source: String,
        policy_revision: &str,
    ) -> Self {
        Self {
            rule_id,
            status,
            evidence: evidence.clone(),
            matrix_source,
            policy_revision: policy_revision.to_string(),
            provenance: vec![crate::domain::ir::EvidenceNode {
                origin: evidence.path.clone(),
                refs: vec![evidence],
            }],
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
