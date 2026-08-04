use agent_preflight::domain::contract::Contract;
use agent_preflight::domain::policy::PolicyCatalog;

#[test]
fn reject_use_of_removed_rule() {
    let catalog_yaml = r#"
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
        lifecycle: "removed"
    "#;
    let contract_yaml = r#"
    schema_version: 1
    profile: "test"
    policy_revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        intended_capability: "none"
        risk_tier: "high"
        approval_requirement: "always"
    revision_sha256: "dummy"
    "#;
    let catalog = PolicyCatalog::from_yaml(catalog_yaml).unwrap();
    let contract = Contract::from_yaml(contract_yaml).unwrap();
    assert!(contract.validate_against_catalog(&catalog).is_err());
}

#[test]
fn fail_ci_on_expired_migration_window() {
    let catalog_yaml = r#"
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
        lifecycle: "deprecated"
        migration_deadline: "2020-01-01"
    "#;
    let contract_yaml = r#"
    schema_version: 1
    profile: "test"
    policy_revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        intended_capability: "none"
        risk_tier: "high"
        approval_requirement: "always"
    revision_sha256: "dummy"
    "#;
    let catalog = PolicyCatalog::from_yaml(catalog_yaml).unwrap();
    let contract = Contract::from_yaml(contract_yaml).unwrap();
    assert!(contract.validate_against_catalog(&catalog).is_err());
}

#[test]
fn prevent_stable_contracts_from_silently_selecting_experimental_rules() {
    let catalog_yaml = r#"
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
        lifecycle: "experimental"
    "#;
    let contract_yaml = r#"
    schema_version: 1
    profile: "stable"
    policy_revision: "v1.0.0"
    rules:
      - id: "unsafe_eval"
        intended_capability: "none"
        risk_tier: "high"
        approval_requirement: "always"
    revision_sha256: "dummy"
    "#;
    let catalog = PolicyCatalog::from_yaml(catalog_yaml).unwrap();
    let contract = Contract::from_yaml(contract_yaml).unwrap();
    assert!(contract.validate_against_catalog(&catalog).is_err());
}
