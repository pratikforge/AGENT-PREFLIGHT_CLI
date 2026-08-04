use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::{NormalizedFile, TaintLabel};
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
        for flow in &file.data_flows {
            if flow.taint == TaintLabel::Web && flow.variable_name.contains("sink") {
                findings.push(Finding {
                    rule_id: "identify_data_flow_to_untrusted_sink".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: flow.span.line,
                        parser_error: false,
                    },
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::Pii && flow.variable_name.contains("exfiltrate") {
                findings.push(Finding {
                    rule_id: "block_pii_exfiltration".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: flow.span.line,
                        parser_error: false,
                    },
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::User && flow.variable_name.contains("sanitized") {
                findings.push(Finding {
                    rule_id: "allow_sanitized_data_flow".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: flow.span.line,
                        parser_error: false,
                    },
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
        }
    }
    findings
}
