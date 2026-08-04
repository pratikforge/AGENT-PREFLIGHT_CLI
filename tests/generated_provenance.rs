use agent_preflight::adapters::generated_provenance;
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
fn recognize_generated_file_with_checked_in_source() {
    let file = get_file("recognize_generated_file_with_checked_in_source");
    let findings = generated_provenance::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(
        findings[0].rule_id,
        "recognize_generated_file_with_checked_in_source"
    );
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn reject_untraceable_generated_output() {
    let file = get_file("reject_untraceable_generated_output");
    let findings = generated_provenance::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "reject_untraceable_generated_output");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn reject_source_generated_outside_scan_root() {
    let file = get_file("reject_source_generated_outside_scan_root");
    let findings = generated_provenance::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(
        findings[0].rule_id,
        "reject_source_generated_outside_scan_root"
    );
    assert_eq!(findings[0].status, Status::Verified);
}
