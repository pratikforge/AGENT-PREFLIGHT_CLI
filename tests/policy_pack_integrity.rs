use agent_preflight::adapters::policy_pack_integrity;
use agent_preflight::domain::status::Status;

#[test]
fn reject_tampered_pack_signature() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
signature: "invalid_signature"
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
    let findings = policy_pack_integrity::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "reject_tampered_pack_signature");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn accept_verified_signature() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
signature: "verified_xyz"
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
    let findings = policy_pack_integrity::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "accept_verified_signature");
    assert_eq!(findings[0].status, Status::Verified);
}
