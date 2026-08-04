use agent_preflight::adapters::audit_layer;
use agent_preflight::domain::source::{LanguageHint, SourceCandidate};
use agent_preflight::domain::status::Status;
use agent_preflight::infra::parser::normalize;

fn evaluate_source(code: &str) -> Vec<audit_layer::Finding> {
    let source = SourceCandidate {
        path: "test.py".to_string(),
        content: code.to_string(),
        language_hint: LanguageHint::Python,
        sha256: "dummy".to_string(),
    };
    let mut normalized = normalize(&source);
    agent_preflight::app::resolve::resolve_symbols(std::slice::from_mut(&mut normalized));
    audit_layer::evaluate(&[normalized])
}

#[test]
fn write_immutable_audit_log() {
    let code = r#"
def run():
    write_immutable_audit_log()
"#;
    let findings = evaluate_source(code);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn emit_structured_json() {
    let code = r#"
def run():
    emit_structured_json()
"#;
    let findings = evaluate_source(code);
    assert_eq!(findings[0].status, Status::Verified);
}

#[test]
fn forward_logs_to_siem() {
    let code = r#"
def run():
    forward_logs_to_siem()
"#;
    let findings = evaluate_source(code);
    assert_eq!(findings[0].status, Status::Verified);
}
