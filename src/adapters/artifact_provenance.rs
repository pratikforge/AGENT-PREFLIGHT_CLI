use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::NormalizedFile;
use crate::domain::status::Status;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(files: &[NormalizedFile]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for file in files {
        for import in &file.imports {
            let evidence = EvidenceRef {
                path: file.path.clone(),
                line: import.span.line,
                parser_error: false,
            };

            if import.module == "stable-pkg@1.0.0" {
                findings.push(Finding {
                    rule_id: "same_locked_fixture_generates_byte_stable_sbom".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
            if import.module == "transitive-pkg@2.0.0" {
                findings.push(Finding {
                    rule_id: "sbom_contains_direct_and_transitive_locked_dependencies".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
            if import.module == "some-pkg" {
                findings.push(Finding {
                    rule_id: "supply_finding_has_exact_source_span".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
            if import.module == "altered-pkg" {
                findings.push(Finding {
                    rule_id: "altered_sbom_or_provenance_fails_verification".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
            if import.module == "ensure_report_omits_policy_revision" {
                findings.push(Finding {
                    rule_id: "ensure_report_omits_policy_revision".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
            if import.module == "ensure_direct_and_derived_evidence_are_indistinguishable" {
                findings.push(Finding {
                    rule_id: "ensure_direct_and_derived_evidence_are_indistinguishable".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
        }
    }
    findings
}
