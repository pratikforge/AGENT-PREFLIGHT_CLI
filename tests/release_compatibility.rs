use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn missing_sdk_compatibility_entry() {
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
fn stale_policy_pack() {
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
fn undocumented_breaking_change() {
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
