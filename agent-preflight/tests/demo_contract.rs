use std::fs;
use std::path::Path;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn documented_demo_proves_failed_then_manually_repaired_verification_without_source_auto_edits() {
    let before = copy_demo("claude_before", "agent.ts");
    let before_path = before.path().to_str().expect("utf-8 path");
    run(["scan", before_path]).success();
    run(["approve", before_path, "claude-query-permission-mode"]).success();
    run(["task", before_path, "claude-query-permission-mode"]).success();
    run(["verify", before_path, "--ci"]).code(1);

    let after = copy_demo("claude_after", "agent.ts");
    let after_path = after.path().to_str().expect("utf-8 path");
    run(["scan", after_path]).success();
    run(["approve", after_path, "claude-query-permission-mode"]).success();
    run(["verify", after_path, "--ci"]).code(0);
}

#[test]
fn release_docs_are_present_and_do_not_make_prohibited_assurance_claims() {
    for path in ["README.md", "docs/demo.md", "docs/limitations.md"] {
        let document = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
            .expect("release document");
        let lowered = document.to_ascii_lowercase();
        for prohibited in ["production-ready", "safe", "secure"] {
            assert!(!lowered.contains(prohibited), "{path} claims {prohibited}");
        }
    }
}

fn run<'a>(arguments: impl IntoIterator<Item = &'a str>) -> assert_cmd::assert::Assert {
    Command::cargo_bin("agent-preflight")
        .expect("binary should exist")
        .args(arguments)
        .assert()
}

fn copy_demo(name: &str, file_name: &str) -> tempfile::TempDir {
    let repository = tempdir().expect("temporary repository");
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/demo")
        .join(name)
        .join(file_name);
    fs::copy(source, repository.path().join(file_name)).expect("demo source copy");
    repository
}
