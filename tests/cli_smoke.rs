use assert_cmd::Command;

#[test]
fn help_lists_the_planned_workflow_commands_without_running_them() {
    let mut command = Command::cargo_bin("agent-preflight").expect("binary should exist");

    command.arg("--help").assert().success();

    let output = Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .arg("--help")
        .output()
        .expect("help invocation should run");
    let help = String::from_utf8_lossy(&output.stdout);

    for expected_command in ["scan", "review", "approve", "task", "verify"] {
        assert!(
            help.contains(expected_command),
            "missing {expected_command}"
        );
    }
}

#[test]
fn unknown_command_exits_with_cli_usage_error() {
    let mut command = Command::cargo_bin("agent-preflight").expect("binary should exist");

    command.arg("unknown-command").assert().code(2);
}
