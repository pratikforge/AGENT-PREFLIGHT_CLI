use agent_preflight::adapters::google_adk::evaluate;
use agent_preflight::domain::normalized::{
    CallFact, ImportFact, NormalizedFile, ParserState, Span,
};
use agent_preflight::domain::source::LanguageHint;
use agent_preflight::domain::status::Status;

#[test]
fn direct_function_tool_with_literal_confirmation_is_verified() {
    let findings = evaluate(&[direct_tool(vec!["require_confirmation".to_owned()])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::Verified);
    assert_eq!(findings[0].rule_id, "google-adk-function-tool-confirmation");
    assert_eq!(findings[0].evidence.path, "agent.py");
}

#[test]
fn direct_function_tool_without_confirmation_is_unverifiable_without_a_risk_contract() {
    let findings = evaluate(&[direct_tool(Vec::new())]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_function_tool_with_literal_false_confirmation_is_failed() {
    let mut file = direct_tool(Vec::new());
    file.calls[0].keyword_names = vec!["require_confirmation".to_owned()];
    file.calls[0].static_controls = vec!["require_confirmation=False".to_owned()];

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn dynamic_confirmation_expression_is_cannot_verify_statically() {
    let mut file = direct_tool(Vec::new());
    file.calls[0].keyword_names = vec!["require_confirmation".to_owned()];

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn aliased_function_tool_import_is_explicitly_unverifiable() {
    let mut file = direct_tool(vec!["require_confirmation".to_owned()]);
    file.imports[0].alias = Some("GuardedTool".to_owned());
    file.calls[0].callee = "GuardedTool".to_owned();

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
    assert_eq!(findings[0].evidence.line, 1);
}

#[test]
fn direct_agent_tool_registration_is_visible_but_not_misrepresented_as_confirmation() {
    let mut file = direct_tool(Vec::new());
    file.imports = vec![ImportFact {
        module: "google.adk.agents".to_owned(),
        symbol: Some("Agent".to_owned()),
        alias: None,
        span: Span { line: 1, column: 1 },
    }];
    file.calls = vec![CallFact {
        callee: "Agent".to_owned(),
        keyword_names: vec!["tools".to_owned()],
        true_keywords: Vec::new(),
        property_names: Vec::new(),
        static_controls: Vec::new(),
        span: Span { line: 8, column: 1 },
    }];

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "google-adk-agent-tool-registration");
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
    assert_eq!(findings[0].evidence.line, 8);
}

#[test]
fn direct_llm_agent_tool_registration_is_visible_but_not_misrepresented_as_confirmation() {
    let mut file = direct_tool(Vec::new());
    file.imports = vec![ImportFact {
        module: "google.adk.agents".to_owned(),
        symbol: Some("LlmAgent".to_owned()),
        alias: None,
        span: Span { line: 1, column: 1 },
    }];
    file.calls = vec![CallFact {
        callee: "LlmAgent".to_owned(),
        keyword_names: vec!["tools".to_owned()],
        true_keywords: Vec::new(),
        property_names: Vec::new(),
        static_controls: Vec::new(),
        span: Span { line: 8, column: 1 },
    }];

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "google-adk-agent-tool-registration");
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
    assert_eq!(findings[0].evidence.line, 8);
}

fn direct_tool(true_keywords: Vec<String>) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "google.adk.tools.function_tool".to_owned(),
            symbol: Some("FunctionTool".to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: Vec::new(),
        calls: vec![CallFact {
            callee: "FunctionTool".to_owned(),
            keyword_names: true_keywords.clone(),
            true_keywords,
            property_names: Vec::new(),
            static_controls: Vec::new(),
            span: Span { line: 3, column: 1 },
        }],
        literals: Vec::new(),
    }
}
