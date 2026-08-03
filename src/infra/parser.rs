use std::path::Path;

use tree_sitter::{Language, Node, Parser};

use crate::domain::normalized::{
    CallFact, DecoratorFact, ImportFact, LiteralFact, LiteralKind, NormalizedFile, ParserState,
    Span,
};
use crate::domain::source::{LanguageHint, SourceCandidate};

pub fn normalize(source: &SourceCandidate) -> NormalizedFile {
    let mut parser = Parser::new();
    let language = grammar(source);
    let configured = parser.set_language(&language).is_ok();
    let tree = configured
        .then(|| parser.parse(&source.content, None))
        .flatten();

    let Some(tree) = tree else {
        return empty_normalized(source, ParserState::ParseError);
    };
    if tree.root_node().has_error() {
        return empty_normalized(source, ParserState::ParseError);
    }

    let mut normalized = empty_normalized(source, ParserState::Parsed);
    collect_facts(tree.root_node(), source.content.as_bytes(), &mut normalized);
    normalized
}

fn empty_normalized(source: &SourceCandidate, parser_state: ParserState) -> NormalizedFile {
    NormalizedFile {
        path: source.path.clone(),
        language: source.language_hint,
        parser_state,
        imports: Vec::new(),
        decorators: Vec::new(),
        calls: Vec::new(),
        literals: Vec::new(),
    }
}

fn grammar(source: &SourceCandidate) -> Language {
    match source.language_hint {
        LanguageHint::Python => tree_sitter_python::LANGUAGE.into(),
        LanguageHint::TypeScript
            if Path::new(&source.path)
                .extension()
                .and_then(|extension| extension.to_str())
                == Some("tsx") =>
        {
            tree_sitter_typescript::LANGUAGE_TSX.into()
        }
        LanguageHint::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
    }
}

fn collect_facts(node: Node<'_>, source: &[u8], normalized: &mut NormalizedFile) {
    if matches!(node.kind(), "call" | "call_expression") {
        collect_direct_call(node, source, normalized);
    }
    collect_literal(node, normalized);
    match normalized.language {
        LanguageHint::Python => match node.kind() {
            "import_from_statement" => collect_python_from_import(node, source, normalized),
            "import_statement" => collect_python_module_import(node, source, normalized),
            "decorator" => collect_python_decorator(node, source, normalized),
            _ => {}
        },
        LanguageHint::TypeScript if node.kind() == "import_statement" => {
            collect_typescript_import(node, source, normalized);
        }
        LanguageHint::TypeScript => {}
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_facts(child, source, normalized);
    }
}

fn collect_literal(node: Node<'_>, normalized: &mut NormalizedFile) {
    let kind = match node.kind() {
        "string" => LiteralKind::String,
        "integer" | "number" => LiteralKind::Integer,
        "true" | "false" => LiteralKind::Boolean,
        _ => return,
    };
    normalized.literals.push(LiteralFact {
        kind,
        span: span(node),
    });
}

fn collect_direct_call(node: Node<'_>, source: &[u8], normalized: &mut NormalizedFile) {
    let Some(function) = node.child_by_field_name("function") else {
        return;
    };
    if !matches!(
        function.kind(),
        "identifier" | "attribute" | "member_expression"
    ) {
        return;
    }
    let Ok(callee) = function.utf8_text(source) else {
        return;
    };
    let call_text = node.utf8_text(source).ok();
    let keywords: Vec<(String, bool)> = call_text
        .and_then(|text| text.split_once('(').map(|(_, arguments)| arguments))
        .and_then(|arguments| arguments.strip_suffix(')'))
        .map(|arguments| {
            arguments
                .split(',')
                .filter_map(|argument| argument.split_once('='))
                .map(|(name, value)| (name.trim().to_owned(), value.trim() == "True"))
                .collect()
        })
        .unwrap_or_default();
    normalized.calls.push(CallFact {
        callee: callee.to_owned(),
        keyword_names: keywords.iter().map(|(name, _)| name.clone()).collect(),
        true_keywords: keywords
            .into_iter()
            .filter_map(|(name, is_true)| is_true.then_some(name))
            .collect(),
        property_names: call_text
            .filter(|text| text.contains("permissionMode:"))
            .map(|_| vec!["permissionMode".to_owned()])
            .unwrap_or_default(),
        static_controls: call_text.map(extract_static_controls).unwrap_or_default(),
        span: span(node),
    });
}

fn extract_static_controls(text: &str) -> Vec<String> {
    let mut controls = Vec::new();
    if text.contains("permissionMode:")
        && (text.contains("'dontAsk'") || text.contains("\"dontAsk\""))
    {
        controls.push("permissionMode=dontAsk".to_owned());
    }
    if text.contains("permissionMode:")
        && (text.contains("'bypassPermissions'") || text.contains("\"bypassPermissions\""))
    {
        controls.push("permissionMode=bypassPermissions".to_owned());
    }
    if text.contains("permissionMode:") && (text.contains("'plan'") || text.contains("\"plan\"")) {
        controls.push("permissionMode=plan".to_owned());
    }
    if text.contains("permission_mode=")
        && (text.contains("'dontAsk'") || text.contains("\"dontAsk\""))
    {
        controls.push("permission_mode=dontAsk".to_owned());
    }
    if text.contains("permission_mode=")
        && (text.contains("'bypassPermissions'") || text.contains("\"bypassPermissions\""))
    {
        controls.push("permission_mode=bypassPermissions".to_owned());
    }
    if text.contains("permission_mode=") && (text.contains("'plan'") || text.contains("\"plan\"")) {
        controls.push("permission_mode=plan".to_owned());
    }
    if text.contains("allowedTools:") && text.contains('[') && !text.contains("allowedTools: []") {
        controls.push("allowedTools=literal-nonempty".to_owned());
    }
    if text.contains("allowed_tools=") && text.contains('[') && !text.contains("allowed_tools=[]") {
        controls.push("allowed_tools=literal-nonempty".to_owned());
    }
    if text.contains("needs_approval=True") {
        controls.push("needs_approval=True".to_owned());
    }
    if text.contains("require_confirmation=False") {
        controls.push("require_confirmation=False".to_owned());
    }
    if text.contains("require_approval=\"always\"") || text.contains("require_approval='always'") {
        controls.push("require_approval=always".to_owned());
    }
    let compact: String = text
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    if compact.contains("tool_config={") && compact.contains("\"require_approval\":\"always\"") {
        controls.push("hosted_mcp_require_approval=always".to_owned());
    }
    controls
}

fn collect_python_from_import(node: Node<'_>, source: &[u8], normalized: &mut NormalizedFile) {
    let Ok(text) = node.utf8_text(source) else {
        return;
    };
    let Some(remainder) = text.strip_prefix("from ") else {
        return;
    };
    let Some((module, symbols)) = remainder.split_once(" import ") else {
        return;
    };
    let span = span(node);
    for symbol in symbols
        .split(',')
        .map(str::trim)
        .filter(|symbol| !symbol.is_empty())
    {
        let (symbol, alias) = symbol_and_alias(symbol);
        normalized.imports.push(ImportFact {
            module: module.trim().to_owned(),
            symbol: Some(symbol),
            alias,
            span,
        });
    }
}

fn collect_python_module_import(node: Node<'_>, source: &[u8], normalized: &mut NormalizedFile) {
    let Ok(text) = node.utf8_text(source) else {
        return;
    };
    let Some(modules) = text.strip_prefix("import ") else {
        return;
    };
    let span = span(node);
    for module in modules
        .split(',')
        .map(str::trim)
        .filter(|module| !module.is_empty())
    {
        let (module, alias) = symbol_and_alias(module);
        normalized.imports.push(ImportFact {
            module,
            symbol: None,
            alias,
            span,
        });
    }
}

fn collect_python_decorator(node: Node<'_>, source: &[u8], normalized: &mut NormalizedFile) {
    let Ok(text) = node.utf8_text(source) else {
        return;
    };
    let Some(text) = text.strip_prefix('@') else {
        return;
    };
    let (name, arguments) = text
        .split_once('(')
        .map(|(name, arguments)| (name, arguments.strip_suffix(')').unwrap_or(arguments)))
        .unwrap_or((text, ""));
    normalized.decorators.push(DecoratorFact {
        name: name.trim().to_owned(),
        arguments: arguments.trim().to_owned(),
        span: span(node),
    });
}

fn collect_typescript_import(node: Node<'_>, source: &[u8], normalized: &mut NormalizedFile) {
    let Ok(text) = node.utf8_text(source) else {
        return;
    };
    let Some(remainder) = text.strip_prefix("import ") else {
        return;
    };
    let Some((symbols, module)) = remainder.split_once(" from ") else {
        return;
    };
    let module = module
        .trim()
        .trim_end_matches(';')
        .trim_matches(['\'', '"']);
    let symbols = symbols.trim().trim_start_matches('{').trim_end_matches('}');
    for symbol in symbols
        .split(',')
        .map(str::trim)
        .filter(|symbol| !symbol.is_empty())
    {
        let (symbol, alias) = symbol_and_alias(symbol);
        normalized.imports.push(ImportFact {
            module: module.to_owned(),
            symbol: Some(symbol),
            alias,
            span: span(node),
        });
    }
}

fn symbol_and_alias(text: &str) -> (String, Option<String>) {
    match text.split_once(" as ") {
        Some((name, alias)) => (name.trim().to_owned(), Some(alias.trim().to_owned())),
        None => (text.trim().to_owned(), None),
    }
}

fn span(node: Node<'_>) -> Span {
    let position = node.start_position();
    Span {
        line: u32::try_from(position.row + 1).unwrap_or(u32::MAX),
        column: u32::try_from(position.column + 1).unwrap_or(u32::MAX),
    }
}
