use agent_preflight::adapters::kubernetes;
use agent_preflight::domain::status::Status;

#[test]
fn privileged_pod() {
    let yaml = r#"
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: agent
    securityContext:
      privileged: true
"#;
    let findings = kubernetes::evaluate(yaml, "test.yaml");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn host_network() {
    let yaml = r#"
apiVersion: v1
kind: Pod
spec:
  hostNetwork: true
"#;
    let findings = kubernetes::evaluate(yaml, "test.yaml");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn missing_seccomp_profile() {
    let yaml = r#"
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: agent
"#;
    let findings = kubernetes::evaluate(yaml, "test.yaml");
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn writable_root_filesystem() {
    let yaml = r#"
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: agent
    securityContext:
      readOnlyRootFilesystem: false
"#;
    let findings = kubernetes::evaluate(yaml, "test.yaml");
    assert_eq!(findings[0].status, Status::Failed);
}
