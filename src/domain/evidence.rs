#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct EvidenceRef {
    pub path: String,
    pub line: u32,
    pub parser_error: bool,
}
