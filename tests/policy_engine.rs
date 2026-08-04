use agent_preflight::domain::ir::{Agent, CapabilityIr, EvidenceNode, Tool};
use agent_preflight::domain::policy::{PolicyCatalog, PolicyEvaluator};
use agent_preflight::domain::status::Status;

#[test]
fn evaluator_returns_expected_status() {
    let catalog_yaml = r#"
    schema_version: 1
    revision: "v1.0.0"
    rules:
      - id: "require_approval"
        threat: "injection"
        intent: "require explicit approval for write tools"
        severity: "high"
        evidence_required: ["source"]
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "add approval"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
        adapter: "openai"
      - id: "deny_shell"
        threat: "injection"
        intent: "deny shell capabilities"
        severity: "critical"
        evidence_required: ["source"]
        safe_examples: ["safe()"]
        unsafe_examples: ["eval()"]
        remediation: "remove shell"
        false_positive_handling: "ignore"
        fixture_reference: "tests/fixtures/eval.rs"
        adapter: "openai"
    "#;
    let catalog = PolicyCatalog::from_yaml(catalog_yaml).unwrap();
    let evaluator = PolicyEvaluator::new(catalog);

    // Case 1: Require approval when a write capability lacks it
    let ir_no_approval = CapabilityIr {
        agents: vec![Agent {
            id: "agent1".to_string(),
            provider: "openai".to_string(),
            tools: vec![Tool {
                id: "tool1".to_string(),
                implementation: "write_file".to_string(),
                approval_control: "none".to_string(),
            }],
            mcp_servers: vec![],
            sandbox: None,
            destinations: vec![],
            sensitive_data: vec![],
            dependencies: vec![],
            evidence: EvidenceNode {
                origin: "test".to_string(),
                refs: vec![],
            },
        }],
        edges: vec![],
    };

    let findings = evaluator.evaluate(&ir_no_approval);
    let require_approval_finding = findings
        .iter()
        .find(|f| f.rule_id == "require_approval")
        .unwrap();
    assert_eq!(
        require_approval_finding.status,
        Status::CannotVerifyStatically
    );

    // Case 2: Deny a forbidden shell capability
    let ir_shell = CapabilityIr {
        agents: vec![Agent {
            id: "agent2".to_string(),
            provider: "openai".to_string(),
            tools: vec![Tool {
                id: "tool2".to_string(),
                implementation: "shell".to_string(),
                approval_control: "none".to_string(),
            }],
            mcp_servers: vec![],
            sandbox: None,
            destinations: vec![],
            sensitive_data: vec![],
            dependencies: vec![],
            evidence: EvidenceNode {
                origin: "test".to_string(),
                refs: vec![],
            },
        }],
        edges: vec![],
    };

    let findings_shell = evaluator.evaluate(&ir_shell);
    let shell_finding = findings_shell
        .iter()
        .find(|f| f.rule_id == "deny_shell")
        .unwrap();
    assert_eq!(shell_finding.status, Status::Failed); // or CannotVerifyStatically, based on fail-closed
}
