use agent_preflight::adapters::sdk_version_matrix;
use agent_preflight::domain::normalized::{CallFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn get_file(function_name: &str) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![],
        decorators: vec![],
        calls: vec![CallFact {
            callee: function_name.to_string(),
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
fn unknown_version() {
    let file = get_file("unknown_version");
    let findings = sdk_version_matrix::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "unknown_version");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn below_minimum_version() {
    let file = get_file("below_minimum_version");
    let findings = sdk_version_matrix::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "below_minimum_version");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn above_tested_version() {
    let file = get_file("above_tested_version");
    let findings = sdk_version_matrix::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "above_tested_version");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn supported_pinned_version() {
    let file = get_file("supported_pinned_version");
    let findings = sdk_version_matrix::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "supported_pinned_version");
    assert_eq!(findings[0].status, Status::Verified);
}
