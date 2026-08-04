use agent_preflight::domain::contract::Contract;
use agent_preflight::domain::policy::PolicyCatalog;

#[test]
fn reject_unsupported_policy_schema() {
    let yaml = r#"
    schema_version: 2
    revision: "v1.0.0"
    rules: []
    "#;
    assert!(PolicyCatalog::from_yaml(yaml).is_err());
}

#[test]
fn reject_stale_rule_revision() {
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
    let catalog_yaml = r#"
    schema_version: 1
    revision: "v1.1.0"
    rules: []
    "#;
    let contract = Contract::from_yaml(contract_yaml).unwrap();
    let catalog = PolicyCatalog::from_yaml(catalog_yaml).unwrap();
    assert!(!contract.is_compatible_with(&catalog));
}

#[test]
fn reject_contract_whose_policy_revision_changed_incompatibly() {
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
    let catalog_yaml = r#"
    schema_version: 1
    revision: "v2.0.0"
    rules: []
    "#;
    let contract = Contract::from_yaml(contract_yaml).unwrap();
    let catalog = PolicyCatalog::from_yaml(catalog_yaml).unwrap();
    assert!(!contract.is_compatible_with(&catalog));
}

#[test]
fn compatible_policies_and_contracts_load_deterministically() {
    let contract_yaml = r#"
    schema_version: 1
    profile: "test"
    policy_revision: "v1.1.0"
    rules:
      - id: "unsafe_eval"
        intended_capability: "none"
        risk_tier: "high"
        approval_requirement: "always"
    revision_sha256: "dummy"
    "#;
    let catalog_yaml = r#"
    schema_version: 1
    revision: "v1.1.0"
    rules: []
    "#;
    let contract = Contract::from_yaml(contract_yaml).unwrap();
    let catalog = PolicyCatalog::from_yaml(catalog_yaml).unwrap();
    assert!(contract.is_compatible_with(&catalog));
}
