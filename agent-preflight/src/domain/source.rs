#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCandidate {
    pub path: String,
    pub language_hint: LanguageHint,
    pub sha256: String,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum LanguageHint {
    Python,
    TypeScript,
}
