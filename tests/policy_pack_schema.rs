use agent_preflight::adapters::policy_pack_schema;
use agent_preflight::domain::status::Status;

#[test]
fn valid_organization_policy_pack() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
rules:
  - id: "test-rule"
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
    let findings = policy_pack_schema::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "valid_organization_policy_pack");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn invalid_schema() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
rules:
  - id: "" # Empty rule id makes it invalid
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
    let findings = policy_pack_schema::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "invalid_schema");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn unsupported_threat_model_version() {
    let yaml = r#"
schema_version: 2
revision: "v1.0.0"
rules: []
"#;
    let findings = policy_pack_schema::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "unsupported_threat_model_version");
    assert_eq!(findings[0].status, Status::Verified);
}
