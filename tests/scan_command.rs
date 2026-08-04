use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn scan_writes_deterministic_proposed_artifacts_for_a_supported_repository() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n\n@function_tool\ndef search() -> None:\n    pass\n",
    )
    .expect("agent source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let artifacts = repo.path().join(".agent-preflight");
    for file in ["evidence.yaml", "contract.proposed.yaml", "report.md"] {
        assert!(artifacts.join(file).is_file(), "missing {file}");
    }
    let evidence = fs::read_to_string(artifacts.join("evidence.yaml")).expect("evidence file");
    assert!(evidence.contains("agent.py"));
    assert!(!evidence.contains("def search"));
}

#[test]
fn repeated_scans_replace_artifacts_with_byte_stable_output() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n",
    )
    .expect("agent source");
    let path = repo.path().to_str().expect("utf-8 temp path");

    for _ in 0..2 {
        Command::cargo_bin("agent-preflight")
            .expect("binary should exist")
            .args(["scan", path])
            .assert()
            .success();
    }

    let artifacts = repo.path().join(".agent-preflight");
    let first = fs::read(artifacts.join("evidence.yaml")).expect("evidence file");
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", path])
        .assert()
        .success();
    let second = fs::read(artifacts.join("evidence.yaml")).expect("evidence file");
    assert_eq!(first, second);
}

#[test]
fn unsupported_repository_is_visible_with_a_distinct_exit_code() {
    let repo = tempdir().expect("temporary repository");
    fs::write(repo.path().join("plain.py"), "print('not an agent')\n").expect("plain source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .code(3);
}

#[test]
fn malformed_source_returns_uncertainty_without_crashing() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("broken.py"),
        "from agents import function_tool\ndef broken(:\n",
    )
    .expect("malformed source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .code(4);

    assert!(repo.path().join(".agent-preflight/evidence.yaml").is_file());
}

#[test]
fn empty_repository_is_reported_as_unsupported_with_proposal_artifacts() {
    let repo = tempdir().expect("temporary repository");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .code(3);

    let report =
        fs::read_to_string(repo.path().join(".agent-preflight/report.md")).expect("scan report");
    assert!(report.contains("unsupported"));
    assert!(!report.contains("verified"));
}

#[test]
fn scan_surfaces_openai_adapter_findings_in_redacted_evidence_and_report() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n\n@function_tool\ndef delete_user() -> None:\n    pass\n",
    )
    .expect("agent source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    let report = fs::read_to_string(repo.path().join(".agent-preflight/report.md"))
        .expect("report artifact");
    assert!(evidence.contains("openai-function-tool-approval"));
    assert!(report.contains("cannot_verify_statically"));
    assert!(!evidence.contains("def delete_user"));
}

#[test]
fn evidence_keeps_only_adapter_findings_and_parse_errors_not_normalized_repository_facts() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n\n@function_tool(needs_approval=True)\ndef search() -> None:\n    pass\n",
    )
    .expect("agent source");
    fs::write(
        repo.path().join("unrelated.py"),
        "from pathlib import Path\nsecret = 'do-not-serialize'\nPath('notes.txt').read_text()\n",
    )
    .expect("unrelated source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    assert!(evidence.contains("openai-function-tool-approval"));
    assert!(evidence.contains("agent.py"));
    assert!(!evidence.contains("unrelated.py"));
    assert!(!evidence.contains("pathlib"));
    assert!(!evidence.contains("read_text"));
    assert!(!evidence.contains("do-not-serialize"));
}

#[test]
fn evidence_retains_only_parse_error_location_for_malformed_source() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("broken.py"),
        "from agents import function_tool\ndef broken(:\n",
    )
    .expect("malformed source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .code(4);

    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    assert!(evidence.contains("parse_errors:"));
    assert!(evidence.contains("broken.py"));
    assert!(!evidence.contains("imports:"));
    assert!(!evidence.contains("calls:"));
}

#[test]
fn report_aggregates_repeated_rule_statuses_instead_of_listing_each_finding() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n\n@function_tool\ndef first() -> None:\n    pass\n\n@function_tool\ndef second() -> None:\n    pass\n",
    )
    .expect("agent source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let report = fs::read_to_string(repo.path().join(".agent-preflight/report.md"))
        .expect("report artifact");
    assert!(report.contains("`openai-function-tool-approval`: cannot_verify_statically (2)"));
    assert_eq!(
        report
            .matches("`openai-function-tool-approval`: cannot_verify_statically")
            .count(),
        1
    );
}

#[test]
fn scan_surfaces_google_adk_confirmation_findings() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.py"),
        "from google.adk.tools.function_tool import FunctionTool\ntool = FunctionTool(delete_user, require_confirmation=True)\n",
    )
    .expect("ADK source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    assert!(evidence.contains("google-adk-function-tool-confirmation"));
    assert!(evidence.contains("Verified"));
}

#[test]
fn scan_surfaces_claude_permission_mode_findings() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("agent.ts"),
        "import { query } from '@anthropic-ai/claude-agent-sdk';\nquery({ permissionMode: 'dontAsk', allowedTools: ['Read'] });\n",
    )
    .expect("Claude source");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    assert!(evidence.contains("claude-query-permission-mode"));
    assert!(evidence.contains("Verified"));
}

#[test]
fn multifile_scan_emits_one_finding_for_one_violation() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("a.py"),
        "from agents import function_tool\n@function_tool\ndef f(): pass\neval('print(1)')\n",
    )
    .expect("source a");
    fs::write(repo.path().join("b.py"), "def b(): pass\n").expect("source b");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    assert_eq!(evidence.matches("rule_id: unsafe-eval").count(), 1);
}

#[test]
fn multifile_scan_preserves_distinct_findings_from_distinct_files() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("a.py"),
        "from agents import function_tool\n@function_tool\ndef f(): pass\neval('print(1)')\n",
    )
    .expect("source a");
    fs::write(
        repo.path().join("b.py"),
        "from agents import function_tool\n@function_tool\ndef g(): pass\neval('print(2)')\n",
    )
    .expect("source b");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    assert_eq!(evidence.matches("rule_id: unsafe-eval").count(), 2);
}

#[test]
fn repeated_scan_has_stable_order_and_evidence() {
    let repo = tempdir().expect("temporary repository");
    fs::write(
        repo.path().join("a.py"),
        "from agents import function_tool\n@function_tool\ndef f(): pass\neval('print(1)')\n",
    )
    .expect("source a");

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence1 = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml")).unwrap();

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let evidence2 = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml")).unwrap();
    assert_eq!(evidence1, evidence2);
}

#[test]
fn deduplication_does_not_suppress_per_file_yaml_posture_finding() {
    let repo = tempdir().expect("temporary repository");
    fs::create_dir_all(repo.path().join(".github/workflows")).unwrap();
    fs::write(
        repo.path().join("agent.py"),
        "from agents import function_tool\n@function_tool\ndef f(): pass\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(".github/workflows/ci.yml"),
        "jobs:\n  build:\n    steps:\n      - run: make\n",
    )
    .unwrap();
    fs::write(
        repo.path().join(".github/workflows/deploy.yml"),
        "jobs:\n  deploy:\n    steps:\n      - run: make deploy\n",
    )
    .unwrap();

    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(["scan", repo.path().to_str().expect("utf-8 temp path")])
        .assert()
        .success();

    let sources = agent_preflight::infra::safe_reader::SafeReader
        .read(repo.path())
        .unwrap();
    println!("SOURCES:\n{:?}", sources);
    let evidence = fs::read_to_string(repo.path().join(".agent-preflight/evidence.yaml"))
        .expect("evidence artifact");
    println!("EVIDENCE:\n{}", evidence);
    assert!(evidence.contains("ci.yml"));
    assert!(evidence.contains("deploy.yml"));
}
