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
        for call in &file.calls {
            if call.callee == "ensure_report_omits_policy_revision" {
                findings.push(Finding {
                    rule_id: "ensure_report_omits_policy_revision".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
            if call.callee == "ensure_direct_and_derived_evidence_are_indistinguishable" {
                findings.push(Finding {
                    rule_id: "ensure_direct_and_derived_evidence_are_indistinguishable".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "artifact_provenance".to_string(),
                });
            }
        }
    }
    findings
}
