use agent_preflight::adapters::claude_agent;
use agent_preflight::domain::normalized::{CallFact, ImportFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn claude_file(function_name: &str, static_controls: &[&str]) -> NormalizedFile {
    NormalizedFile {
        path: "test.ts".to_string(),
        language: agent_preflight::domain::source::LanguageHint::TypeScript,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![ImportFact {
            module: "@anthropic-ai/claude-agent-sdk".to_string(),
            symbol: Some("query".to_string()),
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
        decorators: vec![],
        calls: vec![CallFact {
            callee: function_name.to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
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
fn plan() {
    let file = claude_file("query", &["permissionMode=plan"]);
    let findings = claude_agent::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn dont_ask_with_empty_allowlist() {
    let file = claude_file("query", &["permissionMode=dontAsk"]);
    let findings = claude_agent::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn dont_ask_with_literal_allowlist() {
    let file = claude_file(
        "query",
        &["permissionMode=dontAsk", "allowedTools=literal-nonempty"],
    );
    let findings = claude_agent::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn bypass_permissions() {
    let file = claude_file("query", &["permissionMode=bypassPermissions"]);
    let findings = claude_agent::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn dynamic_permission_mode() {
    let file = claude_file("query", &[]);
    let findings = claude_agent::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn python_and_typescript_forms() {
    let file = claude_file("query", &["permissionMode=plan"]);
    let findings = claude_agent::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}
