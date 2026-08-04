use agent_preflight::domain::policy::PolicyCatalog;

#[test]
fn reject_a_rule_without_threat() {
    let yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        intent: "block eval"
        severity: "high"
        evidence_required: ["source"]
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "dont"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
    "#;
    assert!(PolicyCatalog::from_yaml(yaml).is_err());
}

#[test]
fn reject_a_rule_without_evidence_requirement() {
    let yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        threat: "injection"
        intent: "block eval"
        severity: "high"
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "dont"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
    "#;
    assert!(PolicyCatalog::from_yaml(yaml).is_err());
}

#[test]
fn reject_a_rule_without_safe_unsafe_examples() {
    let yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        threat: "injection"
        intent: "block eval"
        severity: "high"
        evidence_required: ["source"]
        remediation: "dont"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
    "#;
    assert!(PolicyCatalog::from_yaml(yaml).is_err());
}

#[test]
fn reject_a_rule_without_fixture_reference() {
    let yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        threat: "injection"
        intent: "block eval"
        severity: "high"
        evidence_required: ["source"]
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "dont"
        false_positive_handling: "ignore"
    "#;
    assert!(PolicyCatalog::from_yaml(yaml).is_err());
}

#[test]
fn only_complete_versioned_rules_load() {
    let yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        threat: "injection"
        intent: "block eval"
        severity: "high"
        evidence_required: ["source"]
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "dont"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
    "#;
    let catalog = PolicyCatalog::from_yaml(yaml).expect("should load complete rule");
    assert_eq!(catalog.rules.len(), 1);
}
