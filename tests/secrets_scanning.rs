use agent_preflight::adapters::secrets_scanning;
use agent_preflight::domain::normalized::{CallFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn secrets_file(function_name: &str, static_controls: &[&str]) -> NormalizedFile {
    NormalizedFile {
        path: if static_controls.contains(&"test") {
            "test_fixture.py".to_string()
        } else {
            "main.py".to_string()
        },
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
            static_controls: static_controls.iter().map(|s| s.to_string()).collect(),
            keyword_arguments: vec![],
            span: Span { line: 1, column: 0 },
        }],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn detect_hardcoded_token() {
    let file = secrets_file("os.environ.set", &["TOKEN=ghp_1234567890abcdef"]);
    let findings = secrets_scanning::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn detect_environment_variable_leak() {
    let file = secrets_file("print", &["os.environ['AWS_SECRET_ACCESS_KEY']"]);
    let findings = secrets_scanning::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn allow_test_fixtures() {
    let file = secrets_file("print", &["TOKEN=ghp_1234567890abcdef", "test"]);
    let findings = secrets_scanning::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}
