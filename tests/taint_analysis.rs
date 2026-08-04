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

#[test]
fn identify_data_flow_to_untrusted_sink() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "sink_exec".to_string(),
        taint: TaintLabel::Web,
        span: Span {
            line: 10,
            column: 5,
        },
    }]);

    let findings = taint_analysis::evaluate(&[file]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "identify_data_flow_to_untrusted_sink");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn block_pii_exfiltration() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "exfiltrate_req".to_string(),
        taint: TaintLabel::Pii,
        span: Span {
            line: 12,
            column: 5,
        },
    }]);

    let findings = taint_analysis::evaluate(&[file]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "block_pii_exfiltration");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn allow_sanitized_data_flow() {
    let file = get_file(vec![DataFlowFact {
        variable_name: "sanitized_input".to_string(),
        taint: TaintLabel::User,
        span: Span {
            line: 15,
            column: 5,
        },
    }]);

    let findings = taint_analysis::evaluate(&[file]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "allow_sanitized_data_flow");
    assert_eq!(findings[0].status, Status::Verified);
}
