use agent_preflight::adapters::supply_chain;
use agent_preflight::domain::normalized::{CallFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn supply_file(function_name: &str) -> NormalizedFile {
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
fn reject_unpinned_dependencies() {
    let file = supply_file("unpinned");
    let findings = supply_chain::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn flag_vulnerable_packages() {
    let file = supply_file("vulnerable");
    let findings = supply_chain::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn require_sbom_generation() {
    let file = supply_file("sbom");
    let findings = supply_chain::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}
