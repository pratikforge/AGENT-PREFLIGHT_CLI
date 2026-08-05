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
fn denies_unlisted_public_host() {
    let file = get_file("http://unlisted.example.com");
    let findings = network_egress::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "denies_unlisted_public_host" && f.status == Status::Failed)
    );
}

#[test]
fn allows_only_configured_host_scheme_and_port() {
    let file = get_file("https://api.github.com:443/users");
    let findings = network_egress::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "allows_only_configured_host_scheme_and_port"
            && f.status == Status::Verified
    ));
}

#[test]
fn denies_localhost_ipv4_private_and_172_16_range() {
    let file = get_file("http://192.168.1.1");
    let findings = network_egress::evaluate(&[file]);
    assert!(findings.iter().any(
        |f| f.rule_id == "denies_localhost_ipv4_private_and_172_16_range"
            && f.status == Status::Failed
    ));
}

#[test]
fn denies_ipv6_loopback_ula_and_link_local() {
    let file = get_file("http://[::1]");
    let findings = network_egress::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "denies_ipv6_loopback_ula_and_link_local"
                && f.status == Status::Failed)
    );
}

#[test]
fn denies_metadata_endpoint_variants() {
    let file = get_file("http://169.254.169.254");
    let findings = network_egress::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id == "denies_metadata_endpoint_variants" && f.status == Status::Failed));
}

#[test]
fn blocks_case_trailing_dot_and_alternative_ip_bypasses() {
    let file = get_file("http://2852039166");
    let findings = network_egress::evaluate(&[file]);
    assert!(findings.iter().any(|f| f.rule_id
        == "blocks_case_trailing_dot_and_alternative_ip_bypasses"
        && f.status == Status::Failed));
}

#[test]
fn marks_dynamic_destination_uncertain() {
    let file = get_file("dynamic_url");
    let findings = network_egress::evaluate(&[file]);
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "marks_dynamic_destination_uncertain"
                && f.status == Status::CannotVerifyStatically)
    );
}
