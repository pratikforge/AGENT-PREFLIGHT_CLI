use agent_preflight::adapters::config_analysis;
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
fn parse_safe_unsafe_json_yaml_toml_compose_kubernetes_ci_and_env_example() {
    let file = get_file("parse_safe_unsafe_json_yaml_toml_compose_kubernetes_ci_and_env_example");
    let findings = config_analysis::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(
        findings[0].rule_id,
        "parse_safe_unsafe_json_yaml_toml_compose_kubernetes_ci_and_env_example"
    );
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn reject_malformed_config() {
    let file = get_file("reject_malformed_config");
    let findings = config_analysis::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "reject_malformed_config");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn detect_conflicting_configuration() {
    let file = get_file("detect_conflicting_configuration");
    let findings = config_analysis::evaluate(&[file]);
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "detect_conflicting_configuration");
    assert_eq!(findings[0].status, Status::Verified);
}
