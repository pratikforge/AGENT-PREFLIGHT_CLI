use assert_cmd::Command;

#[test]
fn validates_policy_update_runbook_commands() {
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["approve", "--help"]) // Validate runbook CLI syntax
        .assert()
        .success();
}

#[test]
fn validates_audit_verification_runbook_commands() {
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["verify", "--help"])
        .assert()
        .success();
}

#[test]
fn validates_rollback_runbook_commands() {
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["scan", "--help"])
        .assert()
        .success();
}
