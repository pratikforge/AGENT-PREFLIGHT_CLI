use agent_preflight::adapters::network_egress;
use agent_preflight::domain::normalized::{CallFact, NormalizedFile, Span};
use agent_preflight::domain::status::Status;

fn get_file(url: &str) -> NormalizedFile {
    NormalizedFile {
        path: "test.py".to_string(),
        language: agent_preflight::domain::source::LanguageHint::Python,
        parser_state: agent_preflight::domain::normalized::ParserState::Parsed,
        imports: vec![],
        decorators: vec![],
        calls: vec![CallFact {
            callee: "requests.get".to_string(),
            enclosing_function: None,
            keyword_names: vec![],
            true_keywords: vec![],
            property_names: vec![],
            static_controls: vec![url.to_string()],
            keyword_arguments: vec![],
            span: Span {
                line: 10,
                column: 5,
            },
        }],
        literals: vec![],
        assignments: vec![],
        data_flows: vec![],
    }
}

#[test]
fn deny_private_network_access() {
    let file = get_file("http://192.168.1.1/api");
    let findings = network_egress::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn block_metadata_endpoints() {
    let file = get_file("http://169.254.169.254/latest/meta-data");
    let findings = network_egress::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Failed);
}

#[test]
fn allow_whitelisted_domain() {
    let file = get_file("https://api.github.com/users");
    let findings = network_egress::evaluate(&[file]);
    assert_eq!(findings[0].status, Status::Verified);
}
