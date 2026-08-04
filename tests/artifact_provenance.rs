use agent_preflight::adapters::artifact_provenance;
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
            callee: _function_name.to_string(),
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
fn ensure_report_omits_policy_revision() {
    let file = get_file("ensure_report_omits_policy_revision");
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "ensure_report_omits_policy_revision");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn ensure_direct_and_derived_evidence_are_indistinguishable() {
    let file = get_file("ensure_direct_and_derived_evidence_are_indistinguishable");
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(
        findings[0].rule_id,
        "ensure_direct_and_derived_evidence_are_indistinguishable"
    );
    assert_eq!(findings[0].status, Status::Verified);
}
