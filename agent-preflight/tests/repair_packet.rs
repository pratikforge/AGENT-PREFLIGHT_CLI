use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn approved_failed_rule_generates_one_bounded_markdown_packet_without_editing_source() {
    let repo = tempdir().expect("temporary repository");
    let source = repo.path().join("agent.ts");
    fs::write(
        &source,
        "import { query } from '@anthropic-ai/claude-agent-sdk';\nquery({ prompt: 'inspect', options: { permissionMode: 'bypassPermissions' } });\n",
    )
        .expect("agent source");
    let original = fs::read(&source).expect("original source");
    let path = repo.path().to_str().expect("utf-8 temp path");

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
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["task", path, "claude-query-permission-mode"])
        .assert()
        .success();

    let packet = fs::read_to_string(
        repo.path()
            .join(".agent-preflight/tasks/claude-query-permission-mode.md"),
    )
    .expect("repair packet");
    for section in [
        "## Evidence",
        "## Missing rule",
        "## Allowed change boundary",
        "## Non-goals",
        "## Acceptance checks",
        "## Exact verify command",
    ] {
        assert!(packet.contains(section), "missing {section}");
    }
    assert!(packet.contains("agent-preflight verify . --ci"));
    assert!(!packet.contains("inspect"));
    assert_eq!(fs::read(source).expect("source after packet"), original);
}

#[test]
fn packet_escapes_an_unusual_repository_path_without_copying_source_content() {
    let repo = tempdir().expect("temporary repository");
    let directory = repo.path().join("source`folder");
    fs::create_dir(&directory).expect("unusual directory");
    fs::write(
        directory.join("agent.ts"),
        "import { query } from '@anthropic-ai/claude-agent-sdk';\nquery({ prompt: 'secret operation', options: { permissionMode: 'bypassPermissions' } });\n",
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
        .args(["approve", path, "claude-query-permission-mode"])
        .assert()
        .success();
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["task", path, "claude-query-permission-mode"])
        .assert()
        .success();

    let packet = fs::read_to_string(
        repo.path()
            .join(".agent-preflight/tasks/claude-query-permission-mode.md"),
    )
    .expect("repair packet");
    assert!(packet.contains("source'folder/agent.ts"));
    assert!(!packet.contains("secret operation"));
}

#[test]
fn task_rejects_an_unapproved_or_non_failed_rule() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n\n@function_tool\ndef delete_user() -> None:\n    pass\n",
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

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["task", path, "openai-function-tool-approval"])
        .assert()
        .code(2);
    assert!(
        !repo
            .path()
            .join(".agent-preflight/tasks/openai-function-tool-approval.md")
            .exists()
    );
}
