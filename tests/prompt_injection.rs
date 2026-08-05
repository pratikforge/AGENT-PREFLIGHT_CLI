use agent_preflight::adapters::prompt_injection;
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

#[test]
fn blocks_direct_user_override_reaching_system_prompt() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "system_prompt".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = prompt_injection::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "direct-prompt-injection" && f.status == Status::Failed)
    );
}

#[test]
fn blocks_retrieved_web_instruction_reaching_tool_selection() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "tool_selection".to_string(),
        taint: TaintLabel::Web,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = prompt_injection::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "indirect-prompt-injection" && f.status == Status::Failed)
    );
}

#[test]
fn detects_base64_encoded_payload() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "payload_base64".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = prompt_injection::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "base64-evasion" && f.status == Status::Failed)
    );
}

#[test]
fn detects_multilingual_and_role_play_fixture() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "role_play_prompt".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = prompt_injection::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "role-play-evasion" && f.status == Status::Failed)
    );
}

#[test]
fn flags_tool_output_injection_before_high_impact_action() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "high_impact_action".to_string(),
        taint: TaintLabel::Tool,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = prompt_injection::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "tool-output-injection" && f.status == Status::Failed)
    );
}

#[test]
fn does_not_verify_based_only_on_safe_identifier_name() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "safe_prompt_but_actually_user".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = prompt_injection::evaluate(&[file]);
    assert!(!findings.iter().any(|f| f.status == Status::Verified));
}

#[test]
fn allows_proven_isolation_with_typed_sanitizer() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "system_prompt".to_string(),
        taint: TaintLabel::System,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);
    let findings = prompt_injection::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "safe-system-prompt" && f.status == Status::Verified)
    );
}
