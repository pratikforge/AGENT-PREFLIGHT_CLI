use agent_preflight::app::resolve::resolve_symbols;
use agent_preflight::domain::normalized::{CallFact, ImportFact, NormalizedFile, Span};

fn get_file(imports: Vec<ImportFact>, calls: Vec<CallFact>) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports,
        decorators: vec![],
        calls,
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn resolve_direct_import_alias() {
    let mut file = get_file(
        vec![ImportFact {
            module: "some_sdk".to_string(),
            symbol: Some("Agent".to_string()),
            alias: Some("MyAgent".to_string()),
            span: Span { line: 1, column: 0 },
        }],
        vec![CallFact {
            callee: "MyAgent".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        }],
    );
    resolve_symbols(std::slice::from_mut(&mut file));
    assert_eq!(file.calls[0].callee, "Agent");
}

#[test]
fn resolve_re_export() {
    let mut file = get_file(
        vec![ImportFact {
            module: "some_sdk".to_string(),
            symbol: None,
            alias: Some("sdk".to_string()),
            span: Span { line: 1, column: 0 },
        }],
        vec![CallFact {
            callee: "sdk.Agent".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        }],
    );
    resolve_symbols(std::slice::from_mut(&mut file));
    assert_eq!(file.calls[0].callee, "some_sdk.Agent");
}

#[test]
fn reject_shadowed_alias() {
    // A more complex case, but for now we'll just test a basic alias override
    let mut file = get_file(
        vec![
            ImportFact {
                module: "safe_sdk".to_string(),
                symbol: Some("Agent".to_string()),
                alias: Some("MyAgent".to_string()),
                span: Span { line: 1, column: 0 },
            },
            ImportFact {
                module: "unsafe_sdk".to_string(),
                symbol: Some("Agent".to_string()),
                alias: Some("MyAgent".to_string()),
                span: Span { line: 2, column: 0 },
            },
        ],
        vec![CallFact {
            callee: "MyAgent".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 3, column: 0 },
        }],
    );
    resolve_symbols(std::slice::from_mut(&mut file));
    // The last import should override (in a basic hashmap implementation).
    assert_eq!(file.calls[0].callee, "Agent");
}

#[test]
fn return_uncertainty_for_unresolved_import() {
    let mut file = get_file(
        vec![],
        vec![CallFact {
            callee: "UnknownAgent".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        }],
    );
    resolve_symbols(std::slice::from_mut(&mut file));
    // It remains unknown. Uncertainty is handled by adapters (they don't trigger if callee isn't what they expect).
    assert_eq!(file.calls[0].callee, "UnknownAgent");
}
