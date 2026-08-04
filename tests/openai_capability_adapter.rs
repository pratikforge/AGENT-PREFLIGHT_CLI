use agent_preflight::adapters::openai_agents;
use agent_preflight::domain::normalized::{CallFact, DecoratorFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn file_with_decorator(name: &str, args: &str) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![agent_preflight::domain::normalized::ImportFact {
            module: "agents".to_string(),
            symbol: Some("function_tool".to_string()),
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
        decorators: vec![DecoratorFact {
            name: name.to_string(),
            arguments: args.to_string(),
            span: Span { line: 2, column: 0 },
        }],
        calls: vec![],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

fn file_with_call_and_import(callee: &str, args: &[&str], import_symbol: &str) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![agent_preflight::domain::normalized::ImportFact {
            module: "agents".to_string(),
            symbol: Some(import_symbol.to_string()),
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
        decorators: vec![],
        calls: vec![CallFact {
            callee: callee.to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: args.iter().map(|s| s.to_string()).collect(),
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        }],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

fn file_with_mcp_import(callee: &str, args: &[&str]) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![agent_preflight::domain::normalized::ImportFact {
            module: "agents.mcp".to_string(),
            symbol: Some(callee.to_string()),
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
        decorators: vec![],
        calls: vec![CallFact {
            callee: callee.to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: args.iter().map(|s| s.to_string()).collect(),
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        }],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn function_tool_lacking_approval() {
    let file = file_with_decorator("function_tool", "needs_approval=False");
    let findings = openai_agents::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn approved_function_tool() {
    let file = file_with_decorator("function_tool", "needs_approval=True");
    let findings = openai_agents::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn agent_as_tool_lacking_approval() {
    let file = file_with_call_and_import("my_agent.as_tool", &[], "Agent");
    let _findings = openai_agents::evaluate(&[file]);
    // Our implementation checks file.calls for 'Agent' which we omitted. Let's fix the test object to include Agent call.
    let mut file_fixed = file_with_call_and_import("my_agent.as_tool", &[], "Agent");
    file_fixed.calls.push(CallFact {
        callee: "Agent".to_string(),
        enclosing_function: None,
        keyword_names: vec![],
        true_keywords: vec![],
        property_names: vec![],
        static_controls: vec![],
        keyword_arguments: vec![],
        span: Span { line: 1, column: 0 },
    });
    let findings = openai_agents::evaluate(&[file_fixed]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn approved_agent_as_tool() {
    let mut file = file_with_call_and_import("my_agent.as_tool", &["needs_approval=True"], "Agent");
    file.calls.push(CallFact {
        callee: "Agent".to_string(),
        enclosing_function: None,
        keyword_names: vec![],
        true_keywords: vec![],
        property_names: vec![],
        static_controls: vec![],
        keyword_arguments: vec![],
        span: Span { line: 1, column: 0 },
    });
    let findings = openai_agents::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn shell_apply_patch_tool_lacking_approval() {
    let file = file_with_call_and_import("ShellTool", &[], "ShellTool");
    let findings = openai_agents::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn local_mcp_lacking_always_approval() {
    let file = file_with_mcp_import("MCPServerStdio", &[]);
    let findings = openai_agents::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn hosted_mcp_missing_approval() {
    let file = file_with_call_and_import("HostedMCPTool", &[], "HostedMCPTool");
    let findings = openai_agents::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn dynamic_approval_expression() {
    // Dynamic logic missing "True"
    let file = file_with_decorator("function_tool", "needs_approval=is_admin()");
    let findings = openai_agents::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}
