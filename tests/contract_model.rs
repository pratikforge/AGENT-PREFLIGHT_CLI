use agent_preflight::domain::contract::{Contract, Rule, SCHEMA_VERSION};
use agent_preflight::domain::status::Status;

#[test]
fn every_public_status_has_a_distinct_exit_code() {
    let statuses = [
        Status::Verified,
        Status::Failed,
        Status::Partial,
        Status::CannotVerifyStatically,
        Status::Unsupported,
    ];

    let mut codes = statuses.map(Status::exit_code).to_vec();
    codes.sort_unstable();
    codes.dedup();

    assert_eq!(codes.len(), statuses.len());
}

#[test]
fn yaml_contracts_round_trip_and_invalid_inputs_fail_closed() {
    let input = r#"
schema_version: 1
profile: openai_agents_sdk_python
policy_revision: v1.0.0
rules:
  - id: tool.send_email
    intended_capability: send_email
    risk_tier: consequential
    approval_requirement: explicit_owner_approval
revision_sha256: stale
"#;

    let contract = Contract::from_yaml(input).expect("valid YAML contract should parse");
    assert_eq!(contract.profile, "openai_agents_sdk_python");
    assert!(Contract::from_yaml("schema_version: nope").is_err());
    assert!(
        Contract::from_yaml(
            "schema_version: 99\nprofile: x\npolicy_revision: x\nrules: []\nrevision_sha256: x"
        )
        .is_err()
    );
}

#[test]
fn contract_rejects_unknown_schema_and_detects_stale_hash() {
    let mut contract = Contract {
        schema_version: SCHEMA_VERSION,
        profile: "openai_agents_sdk_python".into(),
        policy_revision: "latest".into(),
        rules: vec![Rule {
            id: "tool.send_email".into(),
            intended_capability: "send_email".into(),
            risk_tier: "consequential".into(),
            approval_requirement: "explicit_owner_approval".into(),
        }],
        revision_sha256: String::new(),
    };
    contract.revision_sha256 = contract.canonical_hash();
    assert!(contract.validate().is_ok());
    assert!(contract.has_current_revision());
    contract.schema_version = 99;
    assert!(contract.validate().is_err());
    assert!(!contract.has_current_revision());
}
