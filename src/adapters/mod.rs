pub mod ci;
pub mod claude_agent;
pub mod docker;
pub mod google_adk;
pub mod kubernetes;
pub mod openai_agents;

use crate::domain::normalized::NormalizedFile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    OpenAiAgents,
    GoogleAdk,
    Gemini,
    ClaudeAgentSdk,
    Unsupported,
}

impl Profile {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenAiAgents => "openai-agents-sdk",
            Self::GoogleAdk => "google-adk",
            Self::Gemini => "gemini-api",
            Self::ClaudeAgentSdk => "claude-agent-sdk",
            Self::Unsupported => "unsupported",
        }
    }
}

pub fn detect(files: &[NormalizedFile]) -> Profile {
    if contains_module(files, |module| {
        module == "agents" || module.starts_with("agents.")
    }) {
        Profile::OpenAiAgents
    } else if contains_module(files, |module| {
        module.contains("google.adk") || module == "@google/adk"
    }) {
        Profile::GoogleAdk
    } else if contains_module(files, |module| {
        module.contains("google.genai") || module == "@google/genai"
    }) {
        Profile::Gemini
    } else if contains_module(files, |module| {
        module == "@anthropic-ai/claude-agent-sdk"
            || module == "claude_agent_sdk"
            || module.starts_with("claude_agent_sdk.")
    }) {
        Profile::ClaudeAgentSdk
    } else {
        Profile::Unsupported
    }
}

fn contains_module(files: &[NormalizedFile], predicate: impl Fn(&str) -> bool) -> bool {
    files
        .iter()
        .any(|file| file.imports.iter().any(|fact| predicate(&fact.module)))
}
pub mod adapter_ir_regression;
pub mod artifact_provenance;
pub mod audit_layer;
pub mod config_analysis;
pub mod constant_propagation;
pub mod gemini_api;
pub mod generated_provenance;
pub mod mcp;
pub mod network_egress;
pub mod policy_pack_evaluation;
pub mod policy_pack_integrity;
pub mod policy_pack_precedence;
pub mod policy_pack_schema;
pub mod prompt_injection;
pub mod sdk_version_matrix;
pub mod secrets_scanning;
pub mod supply_chain;
pub mod taint_analysis;
pub mod unsafe_actions;
