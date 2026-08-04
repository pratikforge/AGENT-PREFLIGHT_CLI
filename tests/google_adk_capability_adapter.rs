use agent_preflight::adapters::google_adk;
use agent_preflight::domain::normalized::{CallFact, ImportFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn adk_file(
    callee: &str,
    static_controls: &[&str],
    true_keywords: &[&str],
    keywords: &[&str],
) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![
            ImportFact {
                module: "google.adk.tools.function_tool".to_string(),
                symbol: Some("FunctionTool".to_string()),
                alias: None,
                span: Span { line: 1, column: 0 },
            },
            ImportFact {
                module: "google.adk.agents".to_string(),
                symbol: Some("Agent".to_string()),
                alias: None,
                span: Span { line: 1, column: 0 },
            },
        ],
        decorators: vec![],
        calls: vec![CallFact {
            callee: callee.to_string(),
            enclosing_function: None,
            keyword_names: keywords.iter().map(|s| s.to_string()).collect(),
            true_keywords: true_keywords.iter().map(|s| s.to_string()).collect(),
            property_names: vec![],
            static_controls: static_controls.iter().map(|s| s.to_string()).collect(),
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        }],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn function_tool_confirmation_true() {
    let file = adk_file("FunctionTool", &[], &["require_confirmation"], &[]);
    let findings = google_adk::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn explicit_false() {
    let file = adk_file("FunctionTool", &["require_confirmation=False"], &[], &[]);
    let findings = google_adk::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn dynamic_expression() {
    let file = adk_file("FunctionTool", &[], &[], &[]);
    let findings = google_adk::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn aliased_import() {
    let mut file = adk_file("FunctionTool", &[], &[], &[]);
    file.imports[0].alias = Some("MyFunc".to_string());
    let findings = google_adk::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn agent_tool_registration_without_confirmable_control() {
    let file = adk_file("Agent", &[], &[], &["tools"]);
    let findings = google_adk::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}
