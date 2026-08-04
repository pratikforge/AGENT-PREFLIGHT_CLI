use agent_preflight::adapters::docker;
use agent_preflight::domain::status::Status;

#[test]
fn root_container() {
    let compose = r#"
services:
  agent:
    user: "root"
"#;
    let findings = docker::evaluate(compose, "test.yaml");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn missing_user_directive() {
    let compose = r#"
services:
  agent:
    image: "ubuntu"
"#;
    let findings = docker::evaluate(compose, "test.yaml");
    assert_eq!(findings[0].status, Status::Failed); // Fail-closed if no user specified
}

#[test]
fn safe_user_directive() {
    let compose = r#"
services:
  agent:
    user: "1000:1000"
"#;
    let findings = docker::evaluate(compose, "test.yaml");
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn capabilities_added() {
    let compose = r#"
services:
  agent:
    user: "1000:1000"
    cap_add:
      - SYS_ADMIN
"#;
    let findings = docker::evaluate(compose, "test.yaml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "docker-cap-add" && f.status == Status::Failed)
    );
}

#[test]
fn network_host_mode() {
    let compose = r#"
services:
  agent:
    user: "1000:1000"
    network_mode: "host"
"#;
    let findings = docker::evaluate(compose, "test.yaml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "docker-network-host" && f.status == Status::Failed)
    );
}
