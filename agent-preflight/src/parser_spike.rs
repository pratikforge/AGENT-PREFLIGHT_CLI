//! Minimal syntax-parser compatibility layer for supported source fixtures.
//!
//! This module only parses supplied text. It never imports, executes, or otherwise
//! evaluates repository code.

use std::path::Path;

use tree_sitter::{Language, Parser};

/// Whether a parsed fixture is syntactically valid or malformed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseExpectation {
    /// The parser produced a syntax tree without error nodes.
    Valid,
    /// The parser recovered a syntax tree that contains one or more error nodes.
    Malformed,
}

/// The static parsing result for a source fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseOutcome {
    /// The derived fixture classification.
    pub expectation: ParseExpectation,
    /// Whether Tree-sitter reported a syntax-error node anywhere in the tree.
    pub has_error: bool,
}

/// Parses Python, TypeScript, or TSX source text without executing it.
///
/// The source extension in `fixture_path` selects the grammar. Unsupported
/// extensions return an error instead of attempting a best-effort parse.
pub fn parse_fixture(fixture_path: &str, source: &str) -> Result<ParseOutcome, String> {
    let language = language_for(fixture_path)?;
    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .map_err(|error| format!("could not configure parser for {fixture_path}: {error}"))?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| format!("parser produced no syntax tree for {fixture_path}"))?;
    let has_error = tree.root_node().has_error();

    Ok(ParseOutcome {
        expectation: if has_error {
            ParseExpectation::Malformed
        } else {
            ParseExpectation::Valid
        },
        has_error,
    })
}

fn language_for(fixture_path: &str) -> Result<Language, String> {
    match Path::new(fixture_path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("py") => Ok(tree_sitter_python::LANGUAGE.into()),
        Some("ts") => Ok(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        Some("tsx") => Ok(tree_sitter_typescript::LANGUAGE_TSX.into()),
        _ => Err(format!(
            "unsupported parser fixture extension: {fixture_path}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseExpectation, parse_fixture};

    #[test]
    fn rejects_unknown_extensions() {
        assert!(parse_fixture("fixture.rs", "fn main() {}").is_err());
    }

    #[test]
    fn classifies_valid_python() {
        let outcome = parse_fixture("fixture.py", "def run() -> None:\n    pass\n")
            .expect("Python fixture should parse");

        assert_eq!(outcome.expectation, ParseExpectation::Valid);
        assert!(!outcome.has_error);
    }
}
