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
fn reject_direct_injection_attempts() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "prompt".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);

    let findings = prompt_injection::evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "direct-prompt-injection");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn flag_indirect_injection_from_web() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "system_message".to_string(),
        taint: TaintLabel::Web,
        span: Span {
            line: 12,
            column: 5,
        },
    }]);

    let findings = prompt_injection::evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "indirect-prompt-injection");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn allow_safe_system_prompt() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "prompt".to_string(),
        taint: TaintLabel::Tool,
        span: Span {
            line: 15,
            column: 5,
        },
    }]);

    let findings = prompt_injection::evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "safe-system-prompt");
    assert_eq!(findings[0].status, Status::Verified);
}
