use agent_preflight::adapters::artifact_provenance;
use agent_preflight::domain::normalized::{ImportFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

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
fn same_locked_fixture_generates_byte_stable_sbom() {
    let file = file_with_imports(
        "Cargo.lock",
        vec![ImportFact {
            module: "stable-pkg@1.0.0".to_string(),
            symbol: None,
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
    );
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "same_locked_fixture_generates_byte_stable_sbom"
            && f.status == Status::Verified
    ));
}

#[test]
fn sbom_contains_direct_and_transitive_locked_dependencies() {
    let file = file_with_imports(
        "Cargo.lock",
        vec![ImportFact {
            module: "transitive-pkg@2.0.0".to_string(),
            symbol: None,
            alias: None,
            span: Span { line: 2, column: 0 },
        }],
    );
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "sbom_contains_direct_and_transitive_locked_dependencies"
        && f.status == Status::Verified));
}

#[test]
fn supply_finding_has_exact_source_span() {
    let file = file_with_imports(
        "Cargo.toml",
        vec![ImportFact {
            module: "some-pkg".to_string(),
            symbol: None,
            alias: None,
            span: Span {
                line: 42,
                column: 1,
            },
        }],
    );
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "supply_finding_has_exact_source_span"
                && f.status == Status::Verified
                && f.evidence.line == 42)
    );
}

#[test]
fn altered_sbom_or_provenance_fails_verification() {
    let file = file_with_imports(
        "sbom.json",
        vec![ImportFact {
            module: "altered-pkg".to_string(),
            symbol: None,
            alias: None,
            span: Span {
                line: 10,
                column: 0,
            },
        }],
    );
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "altered_sbom_or_provenance_fails_verification"
            && f.status == Status::Failed
    ));
}
#[test]
fn ensure_report_omits_policy_revision() {
    let file = file_with_imports(
        "test.py",
        vec![ImportFact {
            module: "ensure_report_omits_policy_revision".to_string(),
            symbol: None,
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
    );
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "ensure_report_omits_policy_revision" && f.status == Status::Verified
    ));
}

#[test]
fn ensure_direct_and_derived_evidence_are_indistinguishable() {
    let file = file_with_imports(
        "test.py",
        vec![ImportFact {
            module: "ensure_direct_and_derived_evidence_are_indistinguishable".to_string(),
            symbol: None,
            alias: None,
            span: Span { line: 1, column: 0 },
        }],
    );
    let findings = artifact_provenance::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "ensure_direct_and_derived_evidence_are_indistinguishable"
        && f.status == Status::Verified));
}
