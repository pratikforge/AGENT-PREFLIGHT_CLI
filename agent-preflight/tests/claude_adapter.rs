use agent_preflight::adapters::claude_agent::evaluate;
use agent_preflight::domain::normalized::{
    CallFact, ImportFact, NormalizedFile, ParserState, Span,
};
use agent_preflight::domain::source::LanguageHint;
use agent_preflight::domain::status::Status;

#[test]
fn direct_query_with_dont_ask_mode_is_verified() {
    let findings = evaluate(&[direct_query(
        vec![
            "permissionMode=dontAsk".to_owned(),
            "allowedTools=literal-nonempty".to_owned(),
        ],
        vec!["permissionMode".to_owned(), "allowedTools".to_owned()],
    )]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::Verified);
    assert_eq!(findings[0].rule_id, "claude-query-permission-mode");
}

#[test]
fn dont_ask_without_an_explicit_allowlist_is_not_verified() {
    let findings = evaluate(&[direct_query(
        vec!["permissionMode=dontAsk".to_owned()],
        vec!["permissionMode".to_owned()],
    )]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_query_with_dynamic_permission_mode_is_unverifiable() {
    let findings = evaluate(&[direct_query(Vec::new(), vec!["permissionMode".to_owned()])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_query_without_permission_mode_is_unverifiable() {
    let findings = evaluate(&[direct_query(Vec::new(), Vec::new())]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_query_with_bypass_permissions_is_failed() {
    let findings = evaluate(&[direct_query(
        vec!["permissionMode=bypassPermissions".to_owned()],
        vec!["permissionMode".to_owned()],
    )]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn direct_query_with_plan_mode_is_verified_as_read_only() {
    let findings = evaluate(&[direct_query(
        vec!["permissionMode=plan".to_owned()],
        vec!["permissionMode".to_owned()],
    )]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "claude-query-permission-mode");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn aliased_query_import_is_explicitly_unverifiable() {
    let mut file = direct_query(
        vec!["permissionMode=dontAsk".to_owned()],
        vec!["permissionMode".to_owned()],
    );
    file.imports[0].alias = Some("run_query".to_owned());
    file.calls[0].callee = "run_query".to_owned();

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_python_query_with_locked_down_literal_options_is_verified() {
    let findings = evaluate(&[python_query(vec![
        "permission_mode=dontAsk".to_owned(),
        "allowed_tools=literal-nonempty".to_owned(),
    ])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::Verified);
    assert_eq!(findings[0].evidence.line, 5);
}

#[test]
fn direct_python_query_with_bypass_permissions_is_failed() {
    let findings = evaluate(&[python_query(vec![
        "permission_mode=bypassPermissions".to_owned(),
    ])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn direct_python_query_with_plan_mode_is_verified_as_read_only() {
    let findings = evaluate(&[python_query(vec!["permission_mode=plan".to_owned()])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "claude-query-permission-mode");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn direct_python_query_with_callback_or_missing_policy_is_unverifiable() {
    let findings = evaluate(&[python_query(Vec::new())]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

fn direct_query(static_controls: Vec<String>, property_names: Vec<String>) -> NormalizedFile {
    NormalizedFile {
        path: "agent.ts".to_owned(),
        language: LanguageHint::TypeScript,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "@anthropic-ai/claude-agent-sdk".to_owned(),
            symbol: Some("query".to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: Vec::new(),
        calls: vec![CallFact {
            callee: "query".to_owned(),
            keyword_names: Vec::new(),
            true_keywords: Vec::new(),
            property_names,
            static_controls,
            span: Span { line: 3, column: 1 },
        }],
        literals: Vec::new(),
    }
}

fn python_query(static_controls: Vec<String>) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![
            ImportFact {
                module: "claude_agent_sdk".to_owned(),
                symbol: Some("query".to_owned()),
                alias: None,
                span: Span { line: 1, column: 1 },
            },
            ImportFact {
                module: "claude_agent_sdk".to_owned(),
                symbol: Some("ClaudeAgentOptions".to_owned()),
                alias: None,
                span: Span { line: 1, column: 1 },
            },
        ],
        decorators: Vec::new(),
        calls: vec![
            CallFact {
                callee: "query".to_owned(),
                keyword_names: vec!["options".to_owned()],
                true_keywords: Vec::new(),
                property_names: Vec::new(),
                static_controls: Vec::new(),
                span: Span { line: 3, column: 1 },
            },
            CallFact {
                callee: "ClaudeAgentOptions".to_owned(),
                keyword_names: Vec::new(),
                true_keywords: Vec::new(),
                property_names: Vec::new(),
                static_controls,
                span: Span { line: 5, column: 1 },
            },
        ],
        literals: Vec::new(),
    }
}
