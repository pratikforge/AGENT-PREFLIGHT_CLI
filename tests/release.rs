use agent_preflight::adapters::release;
use agent_preflight::domain::status::Status;

#[test]
fn rejects_unsigned_commits_in_protected_branches() {
    let repo_config = r#"
branches:
  main:
    require_signed_commits: true
"#;
    let findings = release::evaluate(repo_config);
    assert!(findings.iter().any(
        |f| f.rule_id == "rejects_unsigned_commits_in_protected_branches"
            && f.status == Status::Verified
    ));
}

#[test]
fn requires_two_party_review_for_policy_changes() {
    let repo_config = r#"
rules:
  policy:
    min_approvers: 2
"#;
    let findings = release::evaluate(repo_config);
    assert!(findings.iter().any(
        |f| f.rule_id == "requires_two_party_review_for_policy_changes"
            && f.status == Status::Verified
    ));
}

#[test]
fn verifies_provenance_attestation_for_all_builds() {
    let repo_config = r#"
builds:
  require_provenance: true
"#;
    let findings = release::evaluate(repo_config);
    assert!(findings.iter().any(
        |f| f.rule_id == "verifies_provenance_attestation_for_all_builds"
            && f.status == Status::Verified
    ));
}

#[test]
fn denies_release_without_successful_quality_gate() {
    let repo_config = r#"
release:
  require_quality_gate: true
"#;
    let findings = release::evaluate(repo_config);
    assert!(findings.iter().any(
        |f| f.rule_id == "denies_release_without_successful_quality_gate"
            && f.status == Status::Verified
    ));
}

#[test]
fn enforces_immutable_tags_for_releases() {
    let repo_config = r#"
tags:
  immutable: true
"#;
    let findings = release::evaluate(repo_config);
    assert!(findings.iter().any(
        |f| f.rule_id == "enforces_immutable_tags_for_releases" && f.status == Status::Verified
    ));
}
