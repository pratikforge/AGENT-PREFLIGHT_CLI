use agent_preflight::adapters::policy_pack_evaluation;
use agent_preflight::domain::status::Status;

#[test]
fn organization_rule_fails_closed() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
rules:
  - id: "organization_rule_fails_closed"
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
    let findings = policy_pack_evaluation::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "organization_rule_fails_closed");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn built_in_rule_fails_closed() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
rules:
  - id: "built_in_rule_fails_closed"
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
    let findings = policy_pack_evaluation::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "built_in_rule_fails_closed");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn pack_disables_unneeded_default_rule() {
    let yaml = r#"
schema_version: 1
revision: "v1.0.0"
rules:
  - id: "pack_disables_unneeded_default_rule"
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
    let findings = policy_pack_evaluation::evaluate(yaml, "agent-preflight.rules.yaml");
    assert!(!findings.is_empty(), "finding must exist");
    assert_eq!(findings[0].rule_id, "pack_disables_unneeded_default_rule");
    assert_eq!(findings[0].status, Status::Verified);
}
