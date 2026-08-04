use agent_preflight::adapters::openai_agents::evaluate;
use agent_preflight::domain::normalized::{
    CallFact, DecoratorFact, ImportFact, NormalizedFile, ParserState, Span,
};
use agent_preflight::domain::source::LanguageHint;
use agent_preflight::domain::status::Status;

#[test]
fn direct_function_tool_without_approval_is_unverifiable_without_a_risk_contract() {
    let file = NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "agents".to_owned(),
            symbol: Some("function_tool".to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: vec![DecoratorFact {
            name: "function_tool".to_owned(),
            arguments: String::new(),
            span: Span { line: 3, column: 1 },
        }],
        calls: Vec::new(),
        literals: Vec::new(),
        assignments: vec![],
        data_flows: vec![],
    };

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
    assert_eq!(findings[0].rule_id, "openai-function-tool-approval");
    assert_eq!(findings[0].evidence.path, "agent.py");
    assert_eq!(findings[0].evidence.line, 3);
    assert_eq!(
        findings[0].matrix_source,
        "ADAPTER_EVIDENCE_MATRIX.md#openai"
    );
}

#[test]
fn direct_documented_approval_is_structurally_verified() {
    let findings = evaluate(&[direct_tool("needs_approval=True")]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn dynamic_approval_expression_is_cannot_verify_statically() {
    let findings = evaluate(&[direct_tool("needs_approval=configuration.flag")]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn aliased_function_tool_import_is_explicitly_unverifiable() {
    let mut file = direct_tool("needs_approval=True");
    file.imports[0].alias = Some("guarded_tool".to_owned());
    file.decorators[0].name = "guarded_tool".to_owned();

    let findings = evaluate(&[file]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
    assert_eq!(findings[0].evidence.line, 1);
}

#[test]
fn direct_agent_as_tool_with_literal_approval_is_verified() {
    let findings = evaluate(&[agent_as_tool(vec!["needs_approval=True".to_owned()])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-agent-as-tool-approval");
    assert_eq!(findings[0].status, Status::Verified);
    assert_eq!(findings[0].evidence.line, 8);
}

#[test]
fn dynamic_agent_as_tool_approval_remains_unverifiable() {
    let findings = evaluate(&[agent_as_tool(Vec::new())]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-agent-as-tool-approval");
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_mcp_server_with_always_approval_is_verified() {
    let findings = evaluate(&[mcp_server(vec!["require_approval=always".to_owned()])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-mcp-server-approval");
    assert_eq!(findings[0].status, Status::Verified);
    assert_eq!(findings[0].evidence.line, 5);
}

#[test]
fn dynamic_or_partial_mcp_approval_remains_unverifiable() {
    let findings = evaluate(&[mcp_server(Vec::new())]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-mcp-server-approval");
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_shell_tool_with_literal_approval_is_verified() {
    let findings = evaluate(&[local_runtime_tool(
        "ShellTool",
        vec!["needs_approval=True".to_owned()],
    )]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-local-runtime-tool-approval");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn direct_apply_patch_tool_without_literal_approval_is_unverifiable() {
    let findings = evaluate(&[local_runtime_tool("ApplyPatchTool", Vec::new())]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-local-runtime-tool-approval");
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

#[test]
fn direct_hosted_mcp_with_literal_always_approval_is_verified() {
    let findings = evaluate(&[hosted_mcp(vec![
        "hosted_mcp_require_approval=always".to_owned(),
    ])]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-hosted-mcp-approval");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn hosted_mcp_with_indirect_or_missing_approval_is_unverifiable() {
    let findings = evaluate(&[hosted_mcp(Vec::new())]);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "openai-hosted-mcp-approval");
    assert_eq!(findings[0].status, Status::CannotVerifyStatically);
}

fn direct_tool(arguments: &str) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "agents".to_owned(),
            symbol: Some("function_tool".to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: vec![DecoratorFact {
            name: "function_tool".to_owned(),
            arguments: arguments.to_owned(),
            span: Span { line: 3, column: 1 },
        }],
        calls: Vec::new(),
        literals: Vec::new(),
        assignments: vec![],
        data_flows: vec![],
    }
}

fn agent_as_tool(static_controls: Vec<String>) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "agents".to_owned(),
            symbol: Some("Agent".to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: Vec::new(),
        calls: vec![
            CallFact {
                enclosing_function: None,
                callee: "Agent".to_owned(),
                keyword_names: Vec::new(),
                true_keywords: Vec::new(),
                property_names: Vec::new(),
                static_controls: Vec::new(),
                keyword_arguments: vec![],
                span: Span { line: 3, column: 1 },
            },
            CallFact {
                enclosing_function: None,
                callee: "specialist.as_tool".to_owned(),
                keyword_names: vec!["needs_approval".to_owned()],
                true_keywords: Vec::new(),
                property_names: Vec::new(),
                static_controls,
                keyword_arguments: vec![],
                span: Span { line: 8, column: 1 },
            },
        ],
        literals: Vec::new(),
        assignments: vec![],
        data_flows: vec![],
    }
}

fn mcp_server(static_controls: Vec<String>) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "agents.mcp".to_owned(),
            symbol: Some("MCPServerStdio".to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: Vec::new(),
        calls: vec![CallFact {
            enclosing_function: None,
            callee: "MCPServerStdio".to_owned(),
            keyword_names: vec!["require_approval".to_owned()],
            true_keywords: Vec::new(),
            property_names: Vec::new(),
            static_controls,
            keyword_arguments: vec![],
            span: Span { line: 5, column: 1 },
        }],
        literals: Vec::new(),
        assignments: vec![],
        data_flows: vec![],
    }
}

fn local_runtime_tool(callee: &str, static_controls: Vec<String>) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "agents".to_owned(),
            symbol: Some(callee.to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: Vec::new(),
        calls: vec![CallFact {
            enclosing_function: None,
            callee: callee.to_owned(),
            keyword_names: Vec::new(),
            true_keywords: Vec::new(),
            property_names: Vec::new(),
            static_controls,
            keyword_arguments: vec![],
            span: Span { line: 5, column: 1 },
        }],
        literals: Vec::new(),
        assignments: vec![],
        data_flows: vec![],
    }
}

fn hosted_mcp(static_controls: Vec<String>) -> NormalizedFile {
    NormalizedFile {
        path: "agent.py".to_owned(),
        language: LanguageHint::Python,
        parser_state: ParserState::Parsed,
        imports: vec![ImportFact {
            module: "agents".to_owned(),
            symbol: Some("HostedMCPTool".to_owned()),
            alias: None,
            span: Span { line: 1, column: 1 },
        }],
        decorators: Vec::new(),
        calls: vec![CallFact {
            enclosing_function: None,
            callee: "HostedMCPTool".to_owned(),
            keyword_names: vec!["tool_config".to_owned()],
            true_keywords: Vec::new(),
            property_names: Vec::new(),
            static_controls,
            keyword_arguments: vec![],
            span: Span { line: 5, column: 1 },
        }],
        literals: Vec::new(),
        assignments: vec![],
        data_flows: vec![],
    }
}
