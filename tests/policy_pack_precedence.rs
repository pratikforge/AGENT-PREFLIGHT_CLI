use agent_preflight::adapters::policy_pack_precedence;
use agent_preflight::domain::status::Status;

#[test]
fn repository_policy_overrides_pack_policy() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
rules:
  - id: "repository_policy_overrides_pack_policy"
    threat: "test threat"
    intent: "test intent"
    severity: "high"
    evidence_required: ["test evidence"]
    safe_examples: ["safe"]
    unsafe_examples: ["unsafe"]
    remediation: "test remediation"
    false_positive_handling: "test handling"
    fixture_reference: "test reference"
"#;
    let findings = policy_pack_precedence::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(
        findings[0].rule_id,
        "repository_policy_overrides_pack_policy"
    );
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn pack_overrides_built_in_policy() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
rules:
  - id: "pack_overrides_built_in_policy"
    threat: "test threat"
    intent: "test intent"
    severity: "high"
    evidence_required: ["test evidence"]
    safe_examples: ["safe"]
    unsafe_examples: ["unsafe"]
    remediation: "test remediation"
    false_positive_handling: "test handling"
    fixture_reference: "test reference"
"#;
    let findings = policy_pack_precedence::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "pack_overrides_built_in_policy");
    assert_eq!(findings[0].status, Status::Verified);
}
