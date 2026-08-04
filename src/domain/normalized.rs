use crate::domain::source::LanguageHint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum ParserState {
    Parsed,
    ParseError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Span {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ImportFact {
    pub module: String,
    pub symbol: Option<String>,
    pub alias: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratorFact {
    pub name: String,
    pub arguments: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CallFact {
    pub callee: String,
    pub enclosing_function: Option<String>,
    pub keyword_names: Vec<String>,
    pub true_keywords: Vec<String>,
    pub property_names: Vec<String>,
    pub static_controls: Vec<String>,
    pub keyword_arguments: Vec<(String, String)>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum LiteralKind {
    String,
    Integer,
    Boolean,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct LiteralFact {
    pub kind: LiteralKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AssignmentFact {
    pub name: String,
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TaintLabel {
    User,
    Web,
    Tool,
    Secret,
    Pii,
    Uncertain,
    Retrieval,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DataFlowFact {
    pub variable_name: String,
    pub taint: TaintLabel,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedFile {
    pub path: String,
    pub language: LanguageHint,
    pub parser_state: ParserState,
    pub imports: Vec<ImportFact>,
    pub decorators: Vec<DecoratorFact>,
    pub calls: Vec<CallFact>,
    pub literals: Vec<LiteralFact>,
    pub assignments: Vec<AssignmentFact>,
    pub data_flows: Vec<DataFlowFact>,
}
