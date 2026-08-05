use agent_preflight::adapters::mutation_testing;
use agent_preflight::domain::status::Status;

#[test]
fn approval_claim_mutation_is_caught() {
    let findings = mutation_testing::evaluate("approval_claim_mutation");
    assert!(findings.iter().any(|f| f.rule_id == "approval_claim_mutation_is_caught" && f.status == Status::Failed));
}

#[test]
fn audit_redaction_mutation_is_caught() {
    let findings = mutation_testing::evaluate("audit_redaction_mutation");
    assert!(
        findings.iter().any(
            |f| f.rule_id == "audit_redaction_mutation_is_caught" && f.status == Status::Failed
        )
    );
}

#[test]
fn egress_private_range_mutation_is_caught() {
    let findings = mutation_testing::evaluate("egress_private_range_mutation");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "egress_private_range_mutation_is_caught"
                && f.status == Status::Failed)
    );
}

#[test]
fn supply_chain_pin_mutation_is_caught() {
    let findings = mutation_testing::evaluate("supply_chain_pin_mutation");
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "supply_chain_pin_mutation_is_caught"
                && f.status == Status::Failed)
    );
}

use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn identify_removed_sandbox_constraints() {
    let repo = tempdir().expect("tempdir");
    let path = repo.path();
    fs::write(
        path.join("agent.py"),
        "from agents import function_tool\n@function_tool(needs_approval=True)\ndef foo(): pass\n",
    )
    .unwrap();
    fs::write(path.join("Dockerfile"), "FROM ubuntu\nUSER root\n").unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["scan", path.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn flag_loosened_permissions() {
    let repo = tempdir().expect("tempdir");
    let path = repo.path();
    fs::write(
        path.join("agent.py"),
        "from agents import function_tool\n@function_tool(needs_approval=True)\ndef foo(): pass\n",
    )
    .unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["scan", path.to_str().unwrap()])
        .assert()
        .success();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args([
            "approve",
            path.to_str().unwrap(),
            "openai-function-tool-approval",
        ])
        .assert()
        .success();

    // Loosen permission
    fs::write(
        path.join("agent.py"),
        "from agents import function_tool\n@function_tool(needs_approval=False)\ndef foo(): pass\n",
    )
    .unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["verify", path.to_str().unwrap(), "--ci"])
        .assert()
        .code(4);
}

#[test]
fn pass_safe_mutations() {
    let repo = tempdir().expect("tempdir");
    let path = repo.path();
    fs::write(
        path.join("agent.py"),
        "from agents import function_tool\n@function_tool(needs_approval=True)\ndef foo(): pass\n",
    )
    .unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["scan", path.to_str().unwrap()])
        .assert()
        .success();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args([
            "approve",
            path.to_str().unwrap(),
            "openai-function-tool-approval",
        ])
        .assert()
        .success();

    // Safe mutation (e.g. change function logic)
    fs::write(path.join("agent.py"), "from agents import function_tool\n@function_tool(needs_approval=True)\ndef foo(): print('safe')\n").unwrap();
    Command::cargo_bin("agent-preflight")
        .unwrap()
        .args(["verify", path.to_str().unwrap(), "--ci"])
        .assert()
        .code(0);
}
