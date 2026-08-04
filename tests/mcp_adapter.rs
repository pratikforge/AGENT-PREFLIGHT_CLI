use agent_preflight::adapters::mcp;
use agent_preflight::domain::source::{LanguageHint, SourceCandidate};
use agent_preflight::domain::status::Status;
use agent_preflight::infra::parser::normalize;

fn evaluate_source(code: &str) -> Vec<mcp::Finding> {
    let source = SourceCandidate {
        path: "test.py".to_string(),
        content: code.to_string(),
        language_hint: LanguageHint::Python,
        sha256: "dummy".to_string(),
    };
    let mut normalized = normalize(&source);
    agent_preflight::app::resolve::resolve_symbols(std::slice::from_mut(&mut normalized));
    mcp::evaluate(&[normalized])
}

#[test]
fn stdio_server() {
    let code = r#"
from mcp.client.stdio import stdio_client
async def run():
    async with stdio_client(server_params) as client:
        pass
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-stdio" && f.status == Status::Verified)
    );
}

#[test]
fn sse_server() {
    let code = r#"
from mcp.client.sse import sse_client
async def run():
    async with sse_client(url) as client:
        pass
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-sse" && f.status == Status::Verified)
    );
}

#[test]
fn streamable_http_server() {
    let code = r#"
from mcp.client.http import http_client
async def run():
    async with http_client(url) as client:
        pass
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-http" && f.status == Status::Verified)
    );
}

#[test]
fn local_command_path() {
    let code = r#"
def run():
    mcp_local_command()
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-local-command" && f.status == Status::Verified)
    );
}

#[test]
fn missing_auth() {
    let code = r#"
def run():
    mcp_missing_auth()
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-auth" && f.status == Status::Failed)
    );
}

#[test]
fn wildcard_tool_allowlist() {
    let code = r#"
def run():
    mcp_wildcard_tools()
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-wildcard-tools" && f.status == Status::Verified)
    );
}

#[test]
fn remote_endpoint() {
    let code = r#"
def run():
    mcp_remote_endpoint()
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-remote-endpoint" && f.status == Status::Verified)
    );
}

#[test]
fn dynamic_endpoint() {
    let code = r#"
def run():
    mcp_dynamic_endpoint()
"#;
    let findings = evaluate_source(code);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "mcp-dynamic-endpoint"
                && f.status == Status::CannotVerifyStatically)
    );
}
