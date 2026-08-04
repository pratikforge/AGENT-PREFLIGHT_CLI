use agent_preflight::adapters::docker;
use agent_preflight::domain::status::Status;

#[test]
fn correlate_required_mcp_sandbox_with_actual_docker_posture() {
    let compose = r#"
services:
  agent:
    user: "1000:1000"
"#;
    let findings = docker::evaluate(compose, "test.yaml");
    // Verified because user is specified, simulating safe posture
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn detect_sandbox_mismatch() {
    let compose = r#"
services:
  agent:
    user: "root"
"#;
    let findings = docker::evaluate(compose, "test.yaml");
    // Sandbox mismatch because of root user
    assert_eq!(findings[0].status, Status::Failed);
}
