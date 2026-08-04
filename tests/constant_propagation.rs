use agent_preflight::app::resolve::resolve_constants;
use agent_preflight::domain::normalized::{AssignmentFact, CallFact, NormalizedFile, Span};

fn get_file(calls: Vec<CallFact>, assignments: Vec<AssignmentFact>) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![],
        decorators: vec![],
        calls,
        literals: vec![],
        assignments,
        data_flows: vec![],
    }
}

#[test]
fn propagate_literal_approval_constant() {
    let mut file = get_file(
        vec![CallFact {
            callee: "Agent".to_string(),
            enclosing_function: None,
            keyword_names: vec!["require_confirmation".to_string()],
            true_keywords: vec![], // Not literal True
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![(
                "require_confirmation".to_string(),
                "MY_APPROVAL".to_string(),
            )],
            span: Span {
                line: 10,
                column: 0,
            },
        }],
        vec![AssignmentFact {
            name: "MY_APPROVAL".to_string(),
            value: "True".to_string(),
            span: Span { line: 1, column: 0 },
        }],
    );

    resolve_constants(std::slice::from_mut(&mut file));

    // After resolution, true_keywords should contain require_confirmation
    assert!(
        file.calls[0]
            .true_keywords
            .contains(&"require_confirmation".to_string())
    );
}

#[test]
fn reject_mutable_value() {
    // If it's not in assignments (because it was mutable or complex), it doesn't propagate
    let mut file = get_file(
        vec![CallFact {
            callee: "Agent".to_string(),
            enclosing_function: None,
            keyword_names: vec!["require_confirmation".to_string()],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![(
                "require_confirmation".to_string(),
                "dynamic_var".to_string(),
            )],
            span: Span {
                line: 10,
                column: 0,
            },
        }],
        vec![], // Not an immutable constant
    );

    resolve_constants(std::slice::from_mut(&mut file));

    assert!(
        !file.calls[0]
            .true_keywords
            .contains(&"require_confirmation".to_string())
    );
}

#[test]
fn preserve_false_true_distinctions() {
    let mut file = get_file(
        vec![CallFact {
            callee: "Agent".to_string(),
            enclosing_function: None,
            keyword_names: vec!["require_confirmation".to_string()],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![("require_confirmation".to_string(), "MY_FALSE".to_string())],
            span: Span {
                line: 10,
                column: 0,
            },
        }],
        vec![AssignmentFact {
            name: "MY_FALSE".to_string(),
            value: "False".to_string(),
            span: Span { line: 1, column: 0 },
        }],
    );

    resolve_constants(std::slice::from_mut(&mut file));

    // Should NOT be added to true_keywords because it resolved to False
    assert!(
        !file.calls[0]
            .true_keywords
            .contains(&"require_confirmation".to_string())
    );
}
