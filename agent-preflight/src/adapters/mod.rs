pub mod claude_agent;
pub mod google_adk;
pub mod openai_agents;

use crate::domain::normalized::NormalizedFile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    OpenAiAgents,
    GoogleAdk,
    ClaudeAgentSdk,
    Unsupported,
}

impl Profile {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenAiAgents => "openai-agents-sdk",
            Self::GoogleAdk => "google-adk",
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
