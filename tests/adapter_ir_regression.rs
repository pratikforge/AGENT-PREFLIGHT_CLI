use agent_preflight::adapters::adapter_ir_regression;
use agent_preflight::domain::normalized::{CallFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn get_file(_function_name: &str) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![],
        decorators: vec![],
        calls: vec![CallFact {
            callee: "compare_legacy_openai_fixture_findings_against_new_ir_findings".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 1, column: 0 },
        }],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn compare_legacy_openai_fixture_findings_against_new_ir_findings() {
    let file = get_file("compare_legacy_openai_fixture_findings_against_new_ir_findings");
    let findings = adapter_ir_regression::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(
        findings[0].rule_id,
        "compare_legacy_openai_fixture_findings_against_new_ir_findings"
    );
    assert_eq!(findings[0].status, Status::Verified);
}
