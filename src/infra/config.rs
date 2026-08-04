use crate::domain::ir::EvidenceNode;

pub struct ConfigParser;

impl ConfigParser {
    pub fn parse(_content: &str, _extension: &str) -> Result<Vec<EvidenceNode>, String> {
        // stub implementation
        Ok(vec![])
    }
}
