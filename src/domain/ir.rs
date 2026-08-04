use crate::domain::evidence::EvidenceRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CapabilityIr {
    pub agents: Vec<Agent>,
    #[serde(default)]
    pub edges: Vec<EvidenceEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub provider: String,
    pub tools: Vec<Tool>,
    pub mcp_servers: Vec<McpServer>,
    pub sandbox: Option<Sandbox>,
    pub destinations: Vec<NetworkDestination>,
    pub sensitive_data: Vec<SensitiveData>,
    pub dependencies: Vec<Dependency>,
    pub evidence: EvidenceNode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub implementation: String,
    pub approval_control: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServer {
    pub endpoint: String,
    pub transport: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sandbox {
    pub network: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkDestination {
    pub hostname: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SensitiveData {
    pub classification: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    pub package: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceNode {
    pub origin: String,
    pub refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: crate::domain::status::Status,
    pub evidence: EvidenceNode,
    pub matrix_source: String,
}

impl EvidenceNode {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.origin.is_empty() {
            return Err("origin is required");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceEdge {
    pub source: EvidenceNode,
    pub derived: EvidenceNode,
}

impl CapabilityIr {
    pub fn add_evidence_edge(
        &mut self,
        source: EvidenceNode,
        derived: EvidenceNode,
    ) -> Result<(), &'static str> {
        // Simple cycle detection: if derived is already an ancestor of source
        if self.is_ancestor(&derived.origin, &source.origin) {
            return Err("derivation cycle detected");
        }

        // Depth limit detection
        let depth = self.get_depth(&source.origin);
        if depth >= 100 {
            return Err("derivation depth limit exceeded");
        }

        self.edges.push(EvidenceEdge { source, derived });
        Ok(())
    }

    fn get_depth(&self, node: &str) -> usize {
        let mut max_depth = 0;
        for edge in &self.edges {
            if edge.derived.origin == node {
                max_depth = max_depth.max(1 + self.get_depth(&edge.source.origin));
            }
        }
        max_depth
    }

    fn is_ancestor(&self, potential_ancestor: &str, node: &str) -> bool {
        if potential_ancestor == node {
            return true;
        }
        for edge in &self.edges {
            if edge.derived.origin == node
                && self.is_ancestor(potential_ancestor, &edge.source.origin)
            {
                return true;
            }
        }
        false
    }
}
