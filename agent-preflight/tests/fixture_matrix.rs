use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde::Deserialize;
use tempfile::tempdir;

#[derive(Deserialize)]
struct Expectations {
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    name: String,
    fixture: PathBuf,
    scan_exit: i32,
    profile: String,
    rule_id: Option<String>,
    status: Option<String>,
    parser_state: Option<String>,
}

#[test]
fn reviewed_fixture_matrix_matches_the_static_scan_contract() {
    let expectations: Expectations =
        serde_yaml_ng::from_str(include_str!("../fixtures/evaluation/expected.yaml"))
            .expect("reviewed fixture expectations");
    for case in expectations.cases {
        let repository = copy_fixture(&case.fixture, &case.name);
        let path = repository.path().to_str().expect("utf-8 temporary path");
        Command::cargo_bin("agent-preflight")
            .expect("binary should exist")
            .args(["scan", path])
            .assert()
            .code(case.scan_exit);

        let evidence = fs::read_to_string(repository.path().join(".agent-preflight/evidence.yaml"))
            .expect("scan evidence");
        assert!(
            evidence.contains(&format!("profile: {}", case.profile)),
            "{} profile",
            case.name
        );
        if let Some(rule_id) = case.rule_id {
            assert!(evidence.contains(&rule_id), "{} rule", case.name);
        }
        if let Some(status) = case.status {
            assert!(
                evidence.contains(&format!("status: {status}")),
                "{} status",
                case.name
            );
        }
        if let Some(parser_state) = case.parser_state {
            assert!(
                evidence.contains(&format!("parser_state: {parser_state}")),
                "{} parser state",
                case.name
            );
        }
    }
}

fn copy_fixture(relative: &Path, case_name: &str) -> tempfile::TempDir {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(relative);
    let repository = tempdir().expect("temporary repository");
    copy_directory(&source, repository.path());
    fs::write(repository.path().join(".case-name"), case_name).expect("case marker");
    repository
}

fn copy_directory(source: &Path, destination: &Path) {
    for entry in fs::read_dir(source).expect("fixture directory") {
        let entry = entry.expect("fixture entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("fixture file type").is_dir() {
            fs::create_dir_all(&target).expect("fixture directory target");
            copy_directory(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("fixture file copy");
        }
    }
}
