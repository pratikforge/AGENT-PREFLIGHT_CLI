use agent_preflight::domain::policy::{PolicyCatalog, SdkEvidenceMatrix};

#[test]
fn reject_an_adapter_rule_with_no_official_evidence_matrix_entry() {
    let yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "openai_missing_evidence"
        threat: "injection"
        intent: "block eval"
        severity: "high"
        evidence_required: ["source"]
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "dont"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
        adapter: "openai"
    "#;
    let matrix_yaml = r#"
    entries: []
    "#;
    let catalog = PolicyCatalog::from_yaml(yaml).unwrap();
    let matrix = SdkEvidenceMatrix::from_yaml(matrix_yaml).unwrap();
    assert!(catalog.validate_against_matrix(&matrix).is_err());
}

#[test]
fn reject_a_matrix_entry_missing_supported_version_bounds() {
    let matrix_yaml = r#"
    entries:
      - adapter: "openai"
        source_links: ["https://example.com"]
    "#;
    assert!(SdkEvidenceMatrix::from_yaml(matrix_yaml).is_err());
}

#[test]
fn every_adapter_rule_maps_to_a_versioned_evidence_entry() {
    let yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "openai_has_evidence"
        threat: "injection"
        intent: "block eval"
        severity: "high"
        evidence_required: ["source"]
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "dont"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
        adapter: "openai"
    "#;
    let matrix_yaml = r#"
    entries:
      - adapter: "openai"
        supported_versions: ">=1.0.0"
        source_links: ["https://example.com"]
    "#;
    let catalog = PolicyCatalog::from_yaml(yaml).unwrap();
    let matrix = SdkEvidenceMatrix::from_yaml(matrix_yaml).unwrap();
    assert!(catalog.validate_against_matrix(&matrix).is_ok());
}
