use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn review_renders_pending_rule_from_the_repository_proposal() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    let path = repo.path().to_str().expect("utf-8 temp path");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();

    let output = Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["review", path])
        .output()
        .expect("review command should run");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("static-review-required"));
}

#[test]
fn review_interactive_approval_accepts_y() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    let path = repo.path().to_str().expect("utf-8 temp path");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["review", path])
        .env("AGENT_PREFLIGHT_FORCE_INTERACTIVE", "1")
        .write_stdin("y\n")
        .assert()
        .success();

    let approved = fs::read_to_string(repo.path().join(".agent-preflight/contract.yaml"))
        .expect("approved contract");
    assert!(approved.contains("static-review-required"));
}

#[test]
fn review_interactive_approval_rejects_n() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    let path = repo.path().to_str().expect("utf-8 temp path");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["review", path])
        .env("AGENT_PREFLIGHT_FORCE_INTERACTIVE", "1")
        .write_stdin("n\n")
        .assert()
        .success();

    assert!(!repo.path().join(".agent-preflight/contract.yaml").exists());
}

#[test]
fn approve_valid_rule_writes_revision_bound_local_contract() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    let path = repo.path().to_str().expect("utf-8 temp path");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["approve", path, "static-review-required"])
        .assert()
        .success();

    let approved = fs::read_to_string(repo.path().join(".agent-preflight/contract.yaml"))
        .expect("approved contract");
    assert!(approved.contains("static-review-required"));
    assert!(approved.contains("proposed_revision_sha256"));
}

#[test]
fn approve_rejects_missing_or_unknown_proposals_with_exit_two() {
    let repo = tempdir().expect("temporary repository");
    let path = repo.path().to_str().expect("utf-8 temp path");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["approve", path, "static-review-required"])
        .assert()
        .code(2);

    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["approve", path, "unknown-rule"])
        .assert()
        .code(2);
}

#[test]
fn approve_rejects_stale_proposal_revision() {
    let repo = tempdir().expect("temporary repository");
    let path = repo.path().to_str().expect("utf-8 temp path");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();

    let proposal = repo.path().join(".agent-preflight/contract.proposed.yaml");
    let changed = fs::read_to_string(&proposal)
        .expect("proposal")
        .replace("revision_sha256: ", "revision_sha256: stale-");
    fs::write(proposal, changed).expect("changed proposal");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["approve", path, "static-review-required"])
        .assert()
        .code(2);
}

#[test]
fn approve_rejects_repeated_approval_of_the_same_revision() {
    let repo = tempdir().expect("temporary repository");
    let path = repo.path().to_str().expect("utf-8 temp path");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();
    for expected_code in [0, 2] {
        Command::cargo_bin("agent-preflight")
            .expect("binary should exist")
            .args(["approve", path, "static-review-required"])
            .assert()
            .code(expected_code);
    }
}
