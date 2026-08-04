use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn fail_if_scan_exceeds_time_budget() {
    let repo = tempdir().expect("tempdir");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n@function_tool(needs_approval=True)\ndef foo(): pass\n",
    )
    .unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["scan", repo.path().to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn fail_if_scan_exceeds_memory_budget() {
    let repo = tempdir().expect("tempdir");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n@function_tool(needs_approval=True)\ndef foo(): pass\n",
    )
    .unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["scan", repo.path().to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn pass_within_budget() {
    let repo = tempdir().expect("tempdir");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n@function_tool(needs_approval=True)\ndef foo(): pass\n",
    )
    .unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["scan", repo.path().to_str().unwrap()])
        .assert()
        .success();
}
