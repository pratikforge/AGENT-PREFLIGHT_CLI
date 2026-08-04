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
            if call.callee.contains("write_immutable_audit_log") {
                findings.push(Finding {
                    rule_id: "audit-immutable".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "AUDIT".to_string(),
                });
            }

            if call.callee.contains("emit_structured_json") {
                findings.push(Finding {
                    rule_id: "audit-structured".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "AUDIT".to_string(),
                });
            }

            if call.callee.contains("forward_logs_to_siem") {
                findings.push(Finding {
                    rule_id: "audit-siem".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "AUDIT".to_string(),
                });
            }
        }
    }
    findings
}
