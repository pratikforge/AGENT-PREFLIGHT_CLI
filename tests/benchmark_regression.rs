use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn fail_on_block_rate_regression() {
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
fn fail_on_false_positive_regression() {
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
fn pass_on_maintained_accuracy() {
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
