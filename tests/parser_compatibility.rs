use std::fs;
use std::path::Path;

use agent_preflight::parser_spike::{ParseExpectation, parse_fixture};

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("parser")
        .join(name);

    fs::read_to_string(path).expect("fixture must be UTF-8")
}

#[test]
fn parses_direct_python_without_error() {
    let outcome = parse_fixture("direct_python.py", &fixture("direct_python.py"))
        .expect("Python fixture should use a supported grammar");

    assert_eq!(outcome.expectation, ParseExpectation::Valid);
    assert!(!outcome.has_error);
}

#[test]
fn parses_direct_typescript_without_error() {
    let outcome = parse_fixture("direct_typescript.ts", &fixture("direct_typescript.ts"))
        .expect("TypeScript fixture should use a supported grammar");

    assert_eq!(outcome.expectation, ParseExpectation::Valid);
    assert!(!outcome.has_error);
}

#[test]
fn parses_direct_tsx_without_error() {
    let outcome = parse_fixture("direct_tsx.tsx", &fixture("direct_tsx.tsx"))
        .expect("TSX fixture should use a supported grammar");

    assert_eq!(outcome.expectation, ParseExpectation::Valid);
    assert!(!outcome.has_error);
}

#[test]
fn reports_malformed_python_as_parse_error() {
    let outcome = parse_fixture("malformed_python.py", &fixture("malformed_python.py"))
        .expect("malformed Python remains parseable input");

    assert_eq!(outcome.expectation, ParseExpectation::Malformed);
    assert!(outcome.has_error);
}
