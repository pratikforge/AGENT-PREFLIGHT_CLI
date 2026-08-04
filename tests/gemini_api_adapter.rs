use agent_preflight::adapters::gemini_api;
use agent_preflight::domain::source::{LanguageHint, SourceCandidate};
use agent_preflight::domain::status::Status;
use agent_preflight::infra::parser::normalize;

fn evaluate_source(code: &str) -> Vec<gemini_api::Finding> {
    let source = SourceCandidate {
        path: "test.py".to_string(),
        content: code.to_string(),
        language_hint: LanguageHint::Python,
        sha256: "dummy".to_string(),
    };
    let mut normalized = normalize(&source);
    // run resolve_symbols so that imports are applied
    agent_preflight::app::resolve::resolve_symbols(std::slice::from_mut(&mut normalized));
    gemini_api::evaluate(&[normalized])
}

#[test]
fn gemini_function_call() {
    let code = r#"
from google.genai import types
def run():
    tool = types.Tool(
        function_declarations=[
            types.FunctionDeclaration(name="delete_user")
        ]
    )
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "gemini-function-call" && f.status == Status::Verified)
    );
}

#[test]
fn gemini_code_execution() {
    let code = r#"
from google.genai import types
def run():
    tool = types.Tool(
        code_execution=types.CodeExecution()
    )
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "gemini-code-execution" && f.status == Status::Verified)
    );
}

#[test]
fn gemini_url_context() {
    let code = r#"
from google.genai import types
def run():
    tool = types.Tool(
        google_search_retrieval=types.GoogleSearchRetrieval()
    )
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "gemini-url-context" && f.status == Status::Verified)
    );
}

#[test]
fn gemini_file_search() {
    let code = r#"
from google.genai import types
def run():
    tool = types.Tool(
        file_search=types.FileSearch()
    )
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "gemini-file-search" && f.status == Status::Verified)
    );
}

#[test]
fn gemini_mcp() {
    let code = r#"
from google.genai import types
def run():
    tool = types.Tool(
        mcp_server=types.McpServer()
    )
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "gemini-mcp" && f.status == Status::Verified)
    );
}
