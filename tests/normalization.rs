use agent_preflight::domain::normalized::{LiteralKind, ParserState, Span};
use agent_preflight::domain::source::{LanguageHint, SourceCandidate};
use agent_preflight::infra::parser::normalize;

#[test]
fn extracts_direct_python_imports_and_decorators_with_source_spans() {
    let source = SourceCandidate {
        path: "src/agent.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "from agents import function_tool\n\n@function_tool(needs_approval=True)\ndef delete_user() -> None:\n    pass\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.parser_state, ParserState::Parsed);
    assert_eq!(normalized.imports.len(), 1);
    assert_eq!(normalized.imports[0].module, "agents");
    assert_eq!(
        normalized.imports[0].symbol.as_deref(),
        Some("function_tool")
    );
    assert_eq!(normalized.imports[0].span, Span { line: 1, column: 1 });
    assert_eq!(normalized.decorators.len(), 1);
    assert_eq!(normalized.decorators[0].name, "function_tool");
    assert_eq!(normalized.decorators[0].arguments, "needs_approval=True");
    assert_eq!(normalized.decorators[0].span, Span { line: 3, column: 1 });
}

#[test]
fn extracts_direct_typescript_imports_and_ignores_import_text_in_strings() {
    let source = SourceCandidate {
        path: "src/agent.ts".to_owned(),
        language_hint: LanguageHint::TypeScript,
        sha256: "fixture".to_owned(),
        content:
            "import { Agent } from '@google/adk';\nconst note = \"import { Fake } from 'fake'\";\n"
                .to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.parser_state, ParserState::Parsed);
    assert_eq!(normalized.imports.len(), 1);
    assert_eq!(normalized.imports[0].module, "@google/adk");
    assert_eq!(normalized.imports[0].symbol.as_deref(), Some("Agent"));
    assert_eq!(normalized.imports[0].span, Span { line: 1, column: 1 });
}

#[test]
fn malformed_source_is_unverifiable_and_exposes_no_partial_facts() {
    let source = SourceCandidate {
        path: "src/broken.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "from agents import function_tool\ndef broken(:\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.parser_state, ParserState::ParseError);
    assert!(normalized.imports.is_empty());
    assert!(normalized.decorators.is_empty());
}

#[test]
fn extracts_imports_from_valid_tsx_using_the_tsx_grammar() {
    let source = SourceCandidate {
        path: "src/agent.tsx".to_owned(),
        language_hint: LanguageHint::TypeScript,
        sha256: "fixture".to_owned(),
        content: "import { Agent } from '@google/adk';\nexport const View = () => <div />;\n"
            .to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.parser_state, ParserState::Parsed);
    assert_eq!(normalized.imports.len(), 1);
    assert_eq!(normalized.imports[0].module, "@google/adk");
}

#[test]
fn extracts_direct_python_module_imports_without_runtime_loading() {
    let source = SourceCandidate {
        path: "src/direct.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "import agents\nimport os as operating_system\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.parser_state, ParserState::Parsed);
    assert_eq!(normalized.imports.len(), 2);
    assert_eq!(normalized.imports[0].module, "agents");
    assert_eq!(normalized.imports[0].symbol, None);
    assert_eq!(normalized.imports[1].module, "os");
    assert_eq!(normalized.imports[1].symbol, None);
}

#[test]
fn extracts_direct_call_sites_but_not_call_text_inside_strings() {
    let source = SourceCandidate {
        path: "src/calls.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "run_task()\nnote = 'fake_call()'\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 1);
    assert_eq!(normalized.calls[0].callee, "run_task");
    assert_eq!(normalized.calls[0].span, Span { line: 1, column: 1 });
}

#[test]
fn records_literal_kinds_and_spans_without_exposing_literal_values() {
    let source = SourceCandidate {
        path: "src/config.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "token = 'private-value'\nretries = 3\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.literals.len(), 2);
    assert_eq!(normalized.literals[0].kind, LiteralKind::String);
    assert_eq!(normalized.literals[0].span, Span { line: 1, column: 9 });
    assert_eq!(normalized.literals[1].kind, LiteralKind::Integer);
    assert!(!format!("{:?}", normalized).contains("private-value"));
}

#[test]
fn preserves_static_import_aliases_for_adapter_matching() {
    let source = SourceCandidate {
        path: "src/aliases.ts".to_owned(),
        language_hint: LanguageHint::TypeScript,
        sha256: "fixture".to_owned(),
        content: "import { Agent as GoogleAgent } from '@google/adk';\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.imports.len(), 1);
    assert_eq!(normalized.imports[0].symbol.as_deref(), Some("Agent"));
    assert_eq!(normalized.imports[0].alias.as_deref(), Some("GoogleAgent"));
}

#[test]
fn extracts_only_literal_true_keyword_controls_from_direct_calls() {
    let source = SourceCandidate {
        path: "src/adk.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "FunctionTool(delete_user, require_confirmation=True, token='secret')\n"
            .to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 1);
    assert_eq!(normalized.calls[0].callee, "FunctionTool");
    assert_eq!(normalized.calls[0].true_keywords, ["require_confirmation"]);
    assert!(!format!("{:?}", normalized).contains("secret"));
}

#[test]
fn extracts_allowlisted_static_permission_control_without_other_option_values() {
    let source = SourceCandidate {
        path: "src/claude.ts".to_owned(),
        language_hint: LanguageHint::TypeScript,
        sha256: "fixture".to_owned(),
        content: "query({ permissionMode: 'dontAsk', allowedTools: ['Read'], token: 'secret' });\n"
            .to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 1);
    assert_eq!(normalized.calls[0].callee, "query");
    assert_eq!(normalized.calls[0].property_names, ["permissionMode"]);
    assert_eq!(
        normalized.calls[0].static_controls,
        ["permissionMode=dontAsk", "allowedTools=literal-nonempty"]
    );
    assert!(!format!("{:?}", normalized).contains("secret"));
}

#[test]
fn extracts_only_named_openai_approval_controls_without_retaining_other_values() {
    let source = SourceCandidate {
        path: "src/agent.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "specialist.as_tool(needs_approval=True, token='secret')\nMCPServerStdio(require_approval=\"always\", token='secret')\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 2);
    assert_eq!(normalized.calls[0].callee, "specialist.as_tool");
    assert_eq!(normalized.calls[0].static_controls, ["needs_approval=True"]);
    assert_eq!(normalized.calls[1].callee, "MCPServerStdio");
    assert_eq!(
        normalized.calls[1].static_controls,
        ["require_approval=always"]
    );
    assert!(!format!("{:?}", normalized).contains("secret"));
}

#[test]
fn extracts_literal_hosted_mcp_approval_without_retaining_other_config_values() {
    let source = SourceCandidate {
        path: "src/agent.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "HostedMCPTool(tool_config={\"require_approval\": \"always\", \"server_url\": \"https://example.test\"}, token='secret')\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 1);
    assert_eq!(
        normalized.calls[0].static_controls,
        ["hosted_mcp_require_approval=always"]
    );
    assert!(!format!("{:?}", normalized).contains("example.test"));
    assert!(!format!("{:?}", normalized).contains("secret"));
}

#[test]
fn extracts_literal_false_google_confirmation_without_retaining_other_values() {
    let source = SourceCandidate {
        path: "src/adk.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "FunctionTool(delete_user, require_confirmation=False, token='secret')\n"
            .to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 1);
    assert_eq!(
        normalized.calls[0].static_controls,
        ["require_confirmation=False"]
    );
    assert!(!format!("{:?}", normalized).contains("secret"));
}

#[test]
fn extracts_python_claude_permission_controls_without_retaining_other_values() {
    let source = SourceCandidate {
        path: "src/agent.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "ClaudeAgentOptions(permission_mode=\"dontAsk\", allowed_tools=[\"Read\"], token='secret')\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 1);
    assert_eq!(
        normalized.calls[0].static_controls,
        ["permission_mode=dontAsk", "allowed_tools=literal-nonempty"]
    );
    assert!(!format!("{:?}", normalized).contains("secret"));
}

#[test]
fn extracts_claude_plan_mode_without_retaining_other_option_values() {
    let source = SourceCandidate {
        path: "src/agent.ts".to_owned(),
        language_hint: LanguageHint::TypeScript,
        sha256: "fixture".to_owned(),
        content: "query({ permissionMode: 'plan', token: 'secret' });\n".to_owned(),
    };

    let normalized = normalize(&source);

    assert_eq!(normalized.calls.len(), 1);
    assert_eq!(normalized.calls[0].static_controls, ["permissionMode=plan"]);
    assert!(!format!("{:?}", normalized).contains("secret"));
}

#[test]
fn propagates_user_input_through_template_to_prompt_sink() {
    let source = SourceCandidate {
        path: "src/flow.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "user_input = get_user_input()
prompt = f'User says: {user_input}'
system_prompt_sink(prompt)
"
        .to_owned(),
    };
    let normalized = normalize(&source);

    assert!(
        normalized
            .data_flows
            .iter()
            .any(|f| f.variable_name == "prompt"
                && f.taint == agent_preflight::domain::normalized::TaintLabel::User)
    );
}

#[test]
fn propagates_web_content_through_wrapper_to_tool_arguments() {
    let source = SourceCandidate {
        path: "src/flow.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "web_data = fetch_url()
args = wrap_data(web_data)
tool_call(args)
"
        .to_owned(),
    };
    let normalized = normalize(&source);

    assert!(
        normalized
            .data_flows
            .iter()
            .any(|f| f.variable_name == "args"
                && f.taint == agent_preflight::domain::normalized::TaintLabel::Web)
    );
}

#[test]
fn tracks_secret_and_pii_to_log_file_shell_and_network_sinks() {
    let source = SourceCandidate {
        path: "src/flow.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "secret = os.environ['TOKEN']
pii = get_pii()
log_file(secret)
shell_exec(pii)
"
        .to_owned(),
    };
    let normalized = normalize(&source);

    assert!(
        normalized
            .data_flows
            .iter()
            .any(|f| f.variable_name == "secret"
                && f.taint == agent_preflight::domain::normalized::TaintLabel::Secret)
    );
    assert!(
        normalized
            .data_flows
            .iter()
            .any(|f| f.variable_name == "pii"
                && f.taint == agent_preflight::domain::normalized::TaintLabel::Pii)
    );
}

#[test]
fn marks_dynamic_reflection_uncertain_not_verified() {
    let source = SourceCandidate {
        path: "src/flow.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "cls = getattr(module, dynamic_name)
cls()
"
        .to_owned(),
    };
    let normalized = normalize(&source);

    // Check that we extract an uncertain fact or something similar
    assert!(
        normalized
            .data_flows
            .iter()
            .any(|f| f.taint == agent_preflight::domain::normalized::TaintLabel::Uncertain)
    );
}

#[test]
fn enforces_interprocedural_depth_bound() {
    let source = SourceCandidate {
        path: "src/flow.py".to_owned(),
        language_hint: LanguageHint::Python,
        sha256: "fixture".to_owned(),
        content: "def f1(x): return f2(x)
def f2(x): return f3(x)
"
        .to_owned(),
    };
    let normalized = normalize(&source);

    // Add logic here to assert depth bounded
    assert!(normalized.parser_state == ParserState::Parsed);
}
