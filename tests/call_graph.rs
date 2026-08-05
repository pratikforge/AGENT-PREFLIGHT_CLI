use agent_preflight::app::resolve::resolve_wrappers;
use agent_preflight::domain::normalized::{CallFact, NormalizedFile, Span};

fn get_file(calls: Vec<CallFact>) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![],
        decorators: vec![],
        calls,
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn resolve_one_hop_safe_wrapper() {
    let mut file = get_file(vec![
        CallFact {
            enclosing_function: Some("safe_wrapper".to_string()),
            callee: "Agent".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec!["require_confirmation".to_string()],
            keyword_arguments: vec![],
            span: Span {
                line: 10,
                column: 0,
            },
        },
        CallFact {
            enclosing_function: Some("main".to_string()),
            callee: "safe_wrapper".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span {
                line: 20,
                column: 0,
            },
        },
    ]);

    resolve_wrappers(std::slice::from_mut(&mut file), 3);

    // main's call to Agent should inherit require_confirmation
    let main_agent_call = file
        .calls
        .iter()
        .find(|c| c.enclosing_function.as_deref() == Some("main") && c.callee == "Agent")
        .expect("should have expanded agent call in main");

    assert!(
        main_agent_call
            .static_controls
            .contains(&"require_confirmation".to_string())
    );
}

#[test]
fn detect_one_hop_unsafe_wrapper() {
    let mut file = get_file(vec![
        CallFact {
            enclosing_function: Some("unsafe_wrapper".to_string()),
            callee: "Agent".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span {
                line: 10,
                column: 0,
            },
        },
        CallFact {
            enclosing_function: Some("main".to_string()),
            callee: "unsafe_wrapper".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span {
                line: 20,
                column: 0,
            },
        },
    ]);

    resolve_wrappers(std::slice::from_mut(&mut file), 3);

    let main_agent_call = file
        .calls
        .iter()
        .find(|c| c.enclosing_function.as_deref() == Some("main") && c.callee == "Agent")
        .expect("should have expanded agent call in main");

    assert!(main_agent_call.static_controls.is_empty());
}

#[test]
fn stop_at_configured_depth() {
    let file = get_file(vec![
        CallFact {
            enclosing_function: Some("depth1".to_string()),
            callee: "depth2".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 1, column: 0 },
        },
        CallFact {
            enclosing_function: Some("depth2".to_string()),
            callee: "Agent".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        },
        CallFact {
            enclosing_function: Some("main".to_string()),
            callee: "depth1".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 3, column: 0 },
        },
    ]);

    // Test with depth 1
    let mut shallow_file = file.clone();
    resolve_wrappers(std::slice::from_mut(&mut shallow_file), 1);

    // With depth 1, main -> depth1 (depth 0), then depth1 -> depth2 (depth 1), but NO further!
    // So main shouldn't have Agent call.
    let agent_in_main = shallow_file
        .calls
        .iter()
        .find(|c| c.enclosing_function.as_deref() == Some("main") && c.callee == "Agent");
    assert!(agent_in_main.is_none());

    // Test with depth 3
    let mut deep_file = file.clone();
    resolve_wrappers(std::slice::from_mut(&mut deep_file), 3);

    let agent_in_main = deep_file
        .calls
        .iter()
        .find(|c| c.enclosing_function.as_deref() == Some("main") && c.callee == "Agent");
    assert!(agent_in_main.is_some());
}

#[test]
fn stop_recursive_cycle() {
    let mut file = get_file(vec![
        CallFact {
            enclosing_function: Some("cycle_a".to_string()),
            callee: "cycle_b".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 1, column: 0 },
        },
        CallFact {
            enclosing_function: Some("cycle_b".to_string()),
            callee: "cycle_a".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        },
        CallFact {
            enclosing_function: Some("main".to_string()),
            callee: "cycle_a".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 3, column: 0 },
        },
    ]);

    // Should not panic or hang
    resolve_wrappers(std::slice::from_mut(&mut file), 10);
    // Success means it returned
}

#[test]
fn enforces_interprocedural_depth_bound() {
    let file = get_file(vec![
        CallFact {
            enclosing_function: Some("depth1".to_string()),
            callee: "depth2".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 1, column: 0 },
        },
        CallFact {
            enclosing_function: Some("depth2".to_string()),
            callee: "Agent".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 2, column: 0 },
        },
        CallFact {
            enclosing_function: Some("main".to_string()),
            callee: "depth1".to_string(),
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 3, column: 0 },
        },
    ]);

    let mut shallow_file = file.clone();
    // Use depth limit of 1
    resolve_wrappers(std::slice::from_mut(&mut shallow_file), 1);

    // Check that Agent call is not in main
    let agent_in_main = shallow_file
        .calls
        .iter()
        .find(|c| c.enclosing_function.as_deref() == Some("main") && c.callee == "Agent");
    assert!(agent_in_main.is_none());
}
