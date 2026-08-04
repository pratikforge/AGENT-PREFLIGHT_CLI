use agent_preflight::adapters::unsafe_actions;
use agent_preflight::domain::source::{LanguageHint, SourceCandidate};
use agent_preflight::domain::status::Status;
use agent_preflight::infra::parser::normalize;

fn evaluate_source(code: &str) -> Vec<unsafe_actions::Finding> {
    let source = SourceCandidate {
        path: "test.py".to_string(),
        content: code.to_string(),
        language_hint: LanguageHint::Python,
        sha256: "dummy".to_string(),
    };
    let mut normalized = normalize(&source);
    agent_preflight::app::resolve::resolve_symbols(std::slice::from_mut(&mut normalized));
    unsafe_actions::evaluate(&[normalized])
}

#[test]
fn block_unapproved_rm_rf() {
    let code = r#"
import subprocess
def run():
    subprocess.run("rm -rf /", shell=True)
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "unsafe-rm-rf" && f.status == Status::Failed)
    );
}

#[test]
fn intercept_raw_eval() {
    let code = r#"
def run():
    eval("__import__('os').system('bash')")
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "unsafe-eval" && f.status == Status::Failed)
    );
}

#[test]
fn allow_scoped_git_commands() {
    let code = r#"
import subprocess
def run():
    subprocess.run("git status", shell=True)
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "safe-command" && f.status == Status::Verified)
    );
}
