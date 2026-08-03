use std::fs;
use std::path::Path;

use ignore::WalkBuilder;
use sha2::{Digest, Sha256};

use crate::domain::source::{LanguageHint, SourceCandidate};

const MAX_FILE_BYTES: u64 = 1_048_576;
const MAX_FILES: usize = 20_000;
const MAX_DEPTH: usize = 32;

#[derive(Debug, Default)]
pub struct SafeReader;

impl SafeReader {
    pub fn read(&self, root: &Path) -> Result<Vec<SourceCandidate>, ReaderError> {
        let root_metadata = fs::symlink_metadata(root).map_err(|_| ReaderError::InvalidRoot)?;
        if root_metadata.file_type().is_symlink() {
            return Err(ReaderError::SymlinkRejected);
        }
        if !root.is_dir() {
            return Err(ReaderError::InvalidRoot);
        }
        let root = root.canonicalize().map_err(|_| ReaderError::InvalidRoot)?;
        let mut sources = Vec::new();
        let mut walker = WalkBuilder::new(&root);
        walker
            .follow_links(false)
            .hidden(false)
            .standard_filters(true);

        for entry in walker.build() {
            let entry = entry.map_err(|_| ReaderError::WalkFailed)?;
            let path = entry.path();
            let relative = path
                .strip_prefix(&root)
                .map_err(|_| ReaderError::EscapePath)?;
            let relative_text = relative.to_string_lossy().replace('\\', "/");
            if is_ignored_path(&relative_text) {
                continue;
            }
            if relative.components().count() > MAX_DEPTH {
                return Err(ReaderError::DepthExceeded);
            }
            let file_type = entry.file_type().ok_or(ReaderError::WalkFailed)?;
            if file_type.is_symlink() {
                return Err(ReaderError::SymlinkRejected);
            }
            if !file_type.is_file() {
                continue;
            }
            let Some(language_hint) = language_hint(path) else {
                continue;
            };
            let metadata = fs::metadata(path).map_err(|_| ReaderError::ReadFailed)?;
            if metadata.len() > MAX_FILE_BYTES {
                return Err(ReaderError::FileTooLarge);
            }
            let content = fs::read_to_string(path).map_err(|_| ReaderError::InvalidUtf8)?;
            sources.push(SourceCandidate {
                path: relative_text,
                language_hint,
                sha256: hex_sha256(content.as_bytes()),
                content,
            });
            if sources.len() > MAX_FILES {
                return Err(ReaderError::TooManyFiles);
            }
        }
        sources.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(sources)
    }
}

fn language_hint(path: &Path) -> Option<LanguageHint> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("py") => Some(LanguageHint::Python),
        Some("ts" | "tsx" | "mts" | "cts") => Some(LanguageHint::TypeScript),
        _ => None,
    }
}

fn is_ignored_path(path: &str) -> bool {
    if path.split('/').any(|segment| {
        matches!(
            segment,
            ".git" | "node_modules" | "target" | "dist" | ".agent-preflight"
        )
    }) {
        return true;
    }
    let name = path.rsplit('/').next().unwrap_or(path);
    name == ".env" || name.starts_with(".env.") || name.ends_with(".pem") || name.ends_with(".key")
}

fn hex_sha256(input: &[u8]) -> String {
    Sha256::digest(input)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ReaderError {
    #[error("scan root is not a readable directory")]
    InvalidRoot,
    #[error("walk failed")]
    WalkFailed,
    #[error("path escapes scan root")]
    EscapePath,
    #[error("source file exceeds the maximum size")]
    FileTooLarge,
    #[error("source file count exceeds the maximum")]
    TooManyFiles,
    #[error("source path exceeds the maximum depth")]
    DepthExceeded,
    #[error("symlinked source paths are not allowed")]
    SymlinkRejected,
    #[error("source file is not valid UTF-8")]
    InvalidUtf8,
    #[error("source file could not be read")]
    ReadFailed,
}
