use agent_preflight::adapters::{Profile, detect};
use agent_preflight::domain::normalized::{ImportFact, NormalizedFile, ParserState, Span};
use agent_preflight::domain::source::LanguageHint;

#[test]
fn openai_agents_sdk_submodule_import_is_detected_as_openai() {
    let profile = detect(&[file_with_import("agents.mcp", "MCPServerStdio")]);

    assert_eq!(profile, Profile::OpenAiAgents);
}

#[test]
fn python_claude_agent_sdk_import_is_detected_as_claude_agent_sdk() {
    let profile = detect(&[file_with_import("claude_agent_sdk", "query")]);

    assert_eq!(profile, Profile::ClaudeAgentSdk);
}

#[test]
fn generic_anthropic_api_sdk_is_not_misidentified_as_claude_agent_sdk() {
    let profile = detect(&[file_with_import("@anthropic-ai/sdk", "Anthropic")]);

    assert_eq!(profile, Profile::Unsupported);
}

fn file_with_import(module: &str, symbol: &str) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: module.to_owned(),
            symbol: Some(symbol.to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: Vec::new(),
        calls: Vec::new(),
        literals: Vec::new(),
    }
}
