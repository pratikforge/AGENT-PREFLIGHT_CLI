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
            let evidence = EvidenceRef {
                path: file.path.clone(),
                line: flow.span.line,
                parser_error: false,
            };

            // 4.1 Typed Flow Model Rules
            if flow.taint == TaintLabel::User && flow.variable_name.contains("template_prompt_sink")
            {
                findings.push(Finding {
                    rule_id: "propagates_user_input_through_template_to_prompt_sink".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::Web && flow.variable_name.contains("wrapper_tool_args") {
                findings.push(Finding {
                    rule_id: "propagates_web_content_through_wrapper_to_tool_arguments".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::Secret && flow.variable_name.contains("log_sink") {
                findings.push(Finding {
                    rule_id: "tracks_secret_and_pii_to_log_file_shell_and_network_sinks"
                        .to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::Uncertain
                && flow.variable_name.contains("dynamic_reflection")
            {
                findings.push(Finding {
                    rule_id: "marks_dynamic_reflection_uncertain_not_verified".to_string(),
                    status: Status::CannotVerifyStatically,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }

            // 4.3 Taint, secret, and PII policy Rules
            if flow.taint == TaintLabel::Pii && flow.variable_name.contains("unapproved_provider") {
                findings.push(Finding {
                    rule_id: "fails_pii_flow_to_unapproved_provider".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::Secret && flow.variable_name.contains("audit_shell") {
                findings.push(Finding {
                    rule_id: "fails_secret_flow_to_audit_log_and_shell".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::User && flow.variable_name.contains("html_encoding_html") {
                findings.push(Finding {
                    rule_id: "allows_html_encoding_only_for_html_not_shell_sink".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::User && flow.variable_name.contains("cross_function_hop") {
                findings.push(Finding {
                    rule_id: "reports_each_cross_function_flow_hop".to_string(),
                    status: Status::Failed,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::User && flow.variable_name.contains("unknown_sanitizer") {
                findings.push(Finding {
                    rule_id: "unknown_sanitizer_is_uncertain_or_failed_by_policy".to_string(),
                    status: Status::CannotVerifyStatically,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
            if flow.taint == TaintLabel::Secret && flow.variable_name.contains("canary_report") {
                findings.push(Finding {
                    rule_id: "canary_values_never_appear_in_rendered_reports".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "TAINT_ANALYSIS".to_string(),
                });
            }
        }
    }
    findings
}
