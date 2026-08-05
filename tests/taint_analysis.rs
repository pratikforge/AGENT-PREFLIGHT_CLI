use agent_preflight::adapters::taint_analysis;
use agent_preflight::domain::normalized::{DataFlowFact, NormalizedFile, Span, TaintLabel};
use agent_preflight::domain::status::Status;

fn get_file(data_flows: Vec<DataFlowFact>) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![],
        decorators: vec![],
        calls: vec![],
        literals: vec![],
        assignments: vec![],
        data_flows,
    }
}

// 4.1 Typed Flow Model

#[test]
fn propagates_user_input_through_template_to_prompt_sink() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "template_prompt_sink".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "propagates_user_input_through_template_to_prompt_sink"
        && f.status == Status::Failed));
}

#[test]
fn propagates_web_content_through_wrapper_to_tool_arguments() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "wrapper_tool_args".to_string(),
        taint: TaintLabel::Web,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "propagates_web_content_through_wrapper_to_tool_arguments"
        && f.status == Status::Failed));
}

#[test]
fn tracks_secret_and_pii_to_log_file_shell_and_network_sinks() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "log_sink".to_string(),
        taint: TaintLabel::Secret,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "tracks_secret_and_pii_to_log_file_shell_and_network_sinks"
        && f.status == Status::Failed));
}

#[test]
fn marks_dynamic_reflection_uncertain_not_verified() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "dynamic_reflection".to_string(),
        taint: TaintLabel::Uncertain,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "marks_dynamic_reflection_uncertain_not_verified"
        && f.status == Status::CannotVerifyStatically));
}

// 4.3 Taint, secret, and PII policy

#[test]
fn fails_pii_flow_to_unapproved_provider() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "unapproved_provider".to_string(),
        taint: TaintLabel::Pii,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "fails_pii_flow_to_unapproved_provider" && f.status == Status::Failed
    ));
}

#[test]
fn fails_secret_flow_to_audit_log_and_shell() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "audit_shell".to_string(),
        taint: TaintLabel::Secret,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "fails_secret_flow_to_audit_log_and_shell"
                && f.status == Status::Failed)
    );
}

#[test]
fn allows_html_encoding_only_for_html_not_shell_sink() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "html_encoding_html".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "allows_html_encoding_only_for_html_not_shell_sink"
        && f.status == Status::Verified));
}

#[test]
fn reports_each_cross_function_flow_hop() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "cross_function_hop".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "reports_each_cross_function_flow_hop"
                && f.status == Status::Failed)
    );
}

#[test]
fn unknown_sanitizer_is_uncertain_or_failed_by_policy() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "unknown_sanitizer".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "unknown_sanitizer_is_uncertain_or_failed_by_policy"
        && f.status == Status::CannotVerifyStatically));
}

#[test]
fn canary_values_never_appear_in_rendered_reports() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "canary_report".to_string(),
        taint: TaintLabel::Secret,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = taint_analysis::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "canary_values_never_appear_in_rendered_reports"
            && f.status == Status::Verified
    ));
}
