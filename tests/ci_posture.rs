use agent_preflight::adapters::ci;
use agent_preflight::domain::status::Status;

#[test]
fn github_actions_write_all_permissions() {
    let yaml = r#"
permissions: write-all
jobs:
  build:
    runs-on: ubuntu-latest
"#;
    let findings = ci::evaluate(yaml, "test.yaml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "ci-permissions-write-all" && f.status == Status::Failed)
    );
}

#[test]
fn missing_oidc() {
    let yaml = r#"
jobs:
  build:
    runs-on: ubuntu-latest
"#;
    let findings = ci::evaluate(yaml, "test.yaml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "ci-missing-oidc" && f.status == Status::Failed)
    );
}

#[test]
fn unpinned_action_versions() {
    let yaml = r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
"#;
    let findings = ci::evaluate(yaml, "test.yaml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "ci-unpinned-action" && f.status == Status::Failed)
    );
}
