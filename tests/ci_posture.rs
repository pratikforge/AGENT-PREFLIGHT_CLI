use agent_preflight::adapters::ci;
use agent_preflight::domain::status::Status;
use std::fs;

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

#[test]
fn ci_uses_pinned_cargo_commands() {
    let content = fs::read_to_string(".github/workflows/agent-preflight-ci.yml").expect("missing ci yaml");
    for line in content.lines() {
        if line.contains("cargo ") && !line.contains("cargo +1.97.1") {
            panic!("Found unpinned cargo command in CI config: {}", line);
        }
    }
}

#[test]
fn pre_commit_uses_pinned_cargo_commands() {
    let content = fs::read_to_string(".pre-commit-config.yaml").expect("missing pre-commit yaml");
    for line in content.lines() {
        if line.contains("cargo ") && !line.contains("cargo +1.97.1") {
            panic!("Found unpinned cargo command in pre-commit config: {}", line);
        }
    }
}
