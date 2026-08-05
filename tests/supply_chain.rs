use agent_preflight::adapters::supply_chain;
use agent_preflight::domain::normalized::{CallFact, ImportFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn supply_file(path: &str, _static_controls: &[&str], calls: Vec<CallFact>) -> NormalizedFile {
    NormalizedFile {
        path: path.to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python, // Just a placeholder
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![],
        decorators: vec![],
        calls,
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

fn file_with_imports(path: &str, imports: Vec<ImportFact>) -> NormalizedFile {
    NormalizedFile {
        path: path.to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports,
        decorators: vec![],
        calls: vec![],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn flags_unpinned_github_action_ref() {
    let file = file_with_imports(
        ".github/workflows/main.yml",
        vec![ImportFact {
            module: "actions/checkout@v2".to_string(),
            symbol: None,
            alias: None,
            span: Span {
                line: 10,
                column: 0,
            },
        }],
    );
    let findings = supply_chain::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "flags_unpinned_github_action_ref" && f.status == Status::Failed)
    );
}

#[test]
fn flags_unpinned_container_image_tag() {
    let file = file_with_imports(
        "Dockerfile",
        vec![ImportFact {
            module: "ubuntu:latest".to_string(),
            symbol: None,
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
    );
    let findings = supply_chain::evaluate(&[file]);
    assert!(
        findings.iter().any(
            |f| f.rule_id == "flags_unpinned_container_image_tag" && f.status == Status::Failed
        )
    );
}

#[test]
fn flags_dependency_matching_locked_advisory_fixture() {
    let file = file_with_imports(
        "Cargo.lock",
        vec![ImportFact {
            module: "vulnerable-package@1.0.0".to_string(), // Matches a known advisory
            symbol: None,
            alias: None,
            span: Span { line: 5, column: 0 },
        }],
    );
    let findings = supply_chain::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "flags_dependency_matching_locked_advisory_fixture"
        && f.status == Status::Failed));
}

#[test]
fn reports_unknown_when_advisory_data_unavailable() {
    let file = file_with_imports(
        "Cargo.lock",
        vec![ImportFact {
            module: "unknown-package@1.0.0".to_string(),
            symbol: None,
            alias: None,
            span: Span { line: 5, column: 0 },
        }],
    );
    let findings = supply_chain::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "reports_unknown_when_advisory_data_unavailable"
            && f.status == Status::CannotVerifyStatically
    ));
}

#[test]
fn flags_untrusted_mcp_server_or_transport() {
    let file = supply_file(
        "mcp_config.json",
        &[],
        vec![CallFact {
            callee: "mcp_server".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec!["transport=untrusted".to_string()],
            keyword_arguments: vec![],
            span: Span { line: 1, column: 0 },
        }],
    );
    let findings = supply_chain::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "flags_untrusted_mcp_server_or_transport"
                && f.status == Status::Failed)
    );
}

#[test]
fn does_not_treat_application_function_named_unpinned_as_supply_evidence() {
    let file = supply_file(
        "app.py",
        &[],
        vec![CallFact {
            callee: "unpinned".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![],
            keyword_arguments: vec![],
            span: Span { line: 1, column: 0 },
        }],
    );
    let findings = supply_chain::evaluate(&[file]);
    // The previous test logic would fail this. Now it should NOT return any supply-related findings for a callee named "unpinned".
    assert!(!findings.iter().any(|f| f.rule_id.contains("unpinned")));
}
