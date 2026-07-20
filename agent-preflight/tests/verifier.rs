use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

fn write_openai_agent(repo: &std::path::Path, approval: &str) {
    fs::write(
        repo.join("agent.py"),
        format!(
            "from agents import function_tool\n\n@function_tool({approval})\ndef delete_user() -> None:\n    pass\n"
        ),
    )
    .expect("agent source");
}

fn scan_and_approve(repo: &std::path::Path) {
    let path = repo.to_str().expect("utf-8 temporary path");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["approve", path, "openai-function-tool-approval"])
        .assert()
        .success();
}

fn write_claude_agent(repo: &std::path::Path, options: &str) {
    fs::write(
        repo.join("agent.ts"),
        format!(
            "import {{ query }} from '@anthropic-ai/claude-agent-sdk';\nquery({{ prompt: 'inspect', options: {options} }});\n"
        ),
    )
    .expect("Claude source");
}

fn scan_and_approve_claude(repo: &std::path::Path) {
    let path = repo.to_str().expect("utf-8 temporary path");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["approve", path, "claude-query-permission-mode"])
        .assert()
        .success();
}

#[test]
fn repaired_repository_verifies_and_writes_a_deterministic_result() {
    let repo = tempdir().expect("temporary repository");
    write_openai_agent(repo.path(), "");
    scan_and_approve(repo.path());
    write_openai_agent(repo.path(), "needs_approval=True");
    let path = repo.path().to_str().expect("utf-8 temporary path");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["verify", path, "--ci"])
        .assert()
        .code(0);
    let first = fs::read(repo.path().join(".agent-preflight/result.yaml")).expect("result");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["verify", path, "--ci"])
        .assert()
        .code(0);
    let second = fs::read(repo.path().join(".agent-preflight/result.yaml")).expect("result");
    assert_eq!(first, second);
    assert!(String::from_utf8_lossy(&first).contains("Verified"));
}

#[test]
fn source_that_changes_the_proposed_contract_fails_closed() {
    let repo = tempdir().expect("temporary repository");
    write_openai_agent(repo.path(), "");
    scan_and_approve(repo.path());
    fs::write(
        repo.path().join("agent.py"),
        "from agents import Agent\nagent = Agent(name='changed')\n",
    )
    .expect("changed source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args([
            "verify",
            repo.path().to_str().expect("utf-8 temporary path"),
            "--ci",
        ])
        .assert()
        .code(2);
}

#[test]
fn missing_control_fails_ci_with_a_redacted_result() {
    let repo = tempdir().expect("temporary repository");
    write_claude_agent(repo.path(), "{ permissionMode: 'bypassPermissions' }");
    scan_and_approve_claude(repo.path());
    let path = repo.path().to_str().expect("utf-8 temporary path");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["verify", path, "--ci"])
        .assert()
        .code(1);
    let result =
        fs::read_to_string(repo.path().join(".agent-preflight/result.yaml")).expect("result");
    assert!(result.contains("Failed"));
    assert!(!result.contains("inspect"));
}

#[test]
fn stale_approval_unsupported_profile_and_parse_uncertainty_return_distinct_ci_codes() {
    let stale = tempdir().expect("stale repository");
    write_openai_agent(stale.path(), "needs_approval=True");
    scan_and_approve(stale.path());
    let approval = stale.path().join(".agent-preflight/contract.yaml");
    let stale_contract = fs::read_to_string(&approval)
        .expect("approval")
        .replace("revision_sha256: ", "revision_sha256: stale-");
    fs::write(approval, stale_contract).expect("stale approval");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["verify", stale.path().to_str().expect("utf-8 path"), "--ci"])
        .assert()
        .code(2);

    let unsupported = tempdir().expect("unsupported repository");
    fs::write(unsupported.path().join("plain.py"), "print('plain')\n").expect("plain source");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args([
            "verify",
            unsupported.path().to_str().expect("utf-8 path"),
            "--ci",
        ])
        .assert()
        .code(3);

    let uncertain = tempdir().expect("uncertain repository");
    fs::write(
        uncertain.path().join("agent.py"),
        "from agents import function_tool\ndef broken(:\n",
    )
    .expect("broken source");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args([
            "verify",
            uncertain.path().to_str().expect("utf-8 path"),
            "--ci",
        ])
        .assert()
        .code(4);
}
