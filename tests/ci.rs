use agent_preflight::adapters::ci;
use agent_preflight::domain::status::Status;

#[test]
fn runs_clippy_on_all_targets() {
    let yaml = r#"
jobs:
  test:
    steps:
      - run: cargo clippy --all-targets -- -D warnings
"#;
    let findings = ci::evaluate(yaml, ".github/workflows/ci.yml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "runs_clippy_on_all_targets" && f.status == Status::Verified)
    );
}

#[test]
fn runs_format_check_before_build() {
    let yaml = r#"
jobs:
  test:
    steps:
      - run: cargo fmt --check
"#;
    let findings = ci::evaluate(yaml, ".github/workflows/ci.yml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "runs_format_check_before_build" && f.status == Status::Verified)
    );
}

#[test]
fn denies_new_untested_adapter_rules() {
    let yaml = r#"
jobs:
  test:
    steps:
      - run: ./scripts/check_adapter_tests.sh
"#;
    let findings = ci::evaluate(yaml, ".github/workflows/ci.yml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "denies_new_untested_adapter_rules"
                && f.status == Status::Verified)
    );
}

#[test]
fn fails_ci_if_dependency_licenses_unapproved() {
    let yaml = r#"
jobs:
  test:
    steps:
      - run: cargo deny check licenses
"#;
    let findings = ci::evaluate(yaml, ".github/workflows/ci.yml");
    assert!(findings.iter().any(
        |f| f.rule_id == "fails_ci_if_dependency_licenses_unapproved"
            && f.status == Status::Verified
    ));
}

#[test]
fn requires_audit_layer_coverage() {
    let yaml = r#"
jobs:
  test:
    steps:
      - run: cargo test --test audit_layer
"#;
    let findings = ci::evaluate(yaml, ".github/workflows/ci.yml");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "requires_audit_layer_coverage" && f.status == Status::Verified)
    );
}

#[test]
fn prevents_bypassing_approval_in_ci_environment() {
    let yaml = r#"
env:
  REQUIRE_APPROVAL: true
"#;
    let findings = ci::evaluate(yaml, ".github/workflows/ci.yml");
    assert!(findings.iter().any(
        |f| f.rule_id == "prevents_bypassing_approval_in_ci_environment"
            && f.status == Status::Verified
    ));
}

#[test]
fn fails_ci_if_test_coverage_drops() {
    let yaml = r#"
jobs:
  test:
    steps:
      - run: cargo tarpaulin --fail-under 80
"#;
    let findings = ci::evaluate(yaml, ".github/workflows/ci.yml");
    assert!(findings.iter().any(|f| f.rule_id == "fails_ci_if_test_coverage_drops" && f.status == Status::Verified));
}
