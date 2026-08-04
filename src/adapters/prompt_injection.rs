use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::{NormalizedFile, TaintLabel};
use crate::domain::status::Status;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

/// Variable name fragments that identify privileged prompt sinks (system-level influence).
const PRIVILEGED_PROMPT_SINKS: &[&str] = &["system_prompt", "system_message", "prompt"];

/// Variable name fragments that identify tool-selection / high-impact action sinks.
const TOOL_SELECTION_SINKS: &[&str] = &["tool_selection", "tool_call", "action_selector"];

/// Variable name fragments that identify high-impact action sinks (tool outputs flowing here
/// constitutes tool-output injection).
const HIGH_IMPACT_ACTION_SINKS: &[&str] = &[
    "high_impact_action",
    "execute_action",
    "run_command",
    "shell_command",
    "file_write",
];

/// Variable name fragments that suggest base64-encoded content (evasion attempt).
const BASE64_INDICATORS: &[&str] = &["base64", "b64", "encoded_payload", "payload_base64"];

/// Variable name fragments that suggest role-play or persona evasion patterns.
const ROLE_PLAY_INDICATORS: &[&str] = &[
    "role_play",
    "roleplay",
    "persona",
    "jailbreak",
    "dan_prompt",
];

fn name_matches_any(name: &str, patterns: &[&str]) -> bool {
    let lower = name.to_lowercase();
    patterns.iter().any(|p| lower.contains(p))
}

pub fn evaluate(files: &[NormalizedFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        let mut has_direct_injection = false;
        let mut has_indirect_injection = false;
        let mut has_safe_prompt = false;

        for flow in &file.data_flows {
            let name = &flow.variable_name;
            let is_privileged_sink = name_matches_any(name, PRIVILEGED_PROMPT_SINKS);
            let is_tool_selection_sink = name_matches_any(name, TOOL_SELECTION_SINKS);
            let is_high_impact_sink = name_matches_any(name, HIGH_IMPACT_ACTION_SINKS);
            let is_base64 = name_matches_any(name, BASE64_INDICATORS);
            let is_role_play = name_matches_any(name, ROLE_PLAY_INDICATORS);

            // --- direct-prompt-injection: User/Uncertain taint → privileged prompt sink ---
            if is_privileged_sink {
                match flow.taint {
                    TaintLabel::User | TaintLabel::Uncertain => {
                        has_direct_injection = true;
                        findings.push(Finding {
                            rule_id: "direct-prompt-injection".to_string(),
                            status: Status::Failed,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: flow.span.line,
                                parser_error: false,
                            },
                            matrix_source: "PROMPT_INJECTION".to_string(),
                        });
                    }
                    TaintLabel::Web => {
                        has_indirect_injection = true;
                        findings.push(Finding {
                            rule_id: "indirect-prompt-injection".to_string(),
                            status: Status::Failed,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: flow.span.line,
                                parser_error: false,
                            },
                            matrix_source: "PROMPT_INJECTION".to_string(),
                        });
                    }
                    TaintLabel::Tool
                    | TaintLabel::Secret
                    | TaintLabel::Pii
                    | TaintLabel::Retrieval
                    | TaintLabel::System => {
                        has_safe_prompt = true;
                    }
                }
            }

            // --- indirect-prompt-injection: Web taint → tool-selection sink ---
            if is_tool_selection_sink
                && (flow.taint == TaintLabel::Web || flow.taint == TaintLabel::Retrieval)
            {
                has_indirect_injection = true;
                findings.push(Finding {
                    rule_id: "indirect-prompt-injection".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: flow.span.line,
                        parser_error: false,
                    },
                    matrix_source: "PROMPT_INJECTION".to_string(),
                });
            }

            // --- tool-output-injection: Tool taint → high-impact action sink ---
            if is_high_impact_sink && flow.taint == TaintLabel::Tool {
                findings.push(Finding {
                    rule_id: "tool-output-injection".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: flow.span.line,
                        parser_error: false,
                    },
                    matrix_source: "PROMPT_INJECTION".to_string(),
                });
            }

            // --- base64-evasion: User taint + base64 variable pattern ---
            if is_base64 && (flow.taint == TaintLabel::User || flow.taint == TaintLabel::Web) {
                findings.push(Finding {
                    rule_id: "base64-evasion".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: flow.span.line,
                        parser_error: false,
                    },
                    matrix_source: "PROMPT_INJECTION".to_string(),
                });
            }

            // --- role-play-evasion: User/Uncertain taint + role-play variable pattern ---
            if is_role_play
                && (flow.taint == TaintLabel::User || flow.taint == TaintLabel::Uncertain)
            {
                findings.push(Finding {
                    rule_id: "role-play-evasion".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: flow.span.line,
                        parser_error: false,
                    },
                    matrix_source: "PROMPT_INJECTION".to_string(),
                });
            }
        }

        // --- safe-system-prompt: no injections found and at least one System-tainted prompt sink ---
        if !has_direct_injection && !has_indirect_injection && has_safe_prompt {
            findings.push(Finding {
                rule_id: "safe-system-prompt".to_string(),
                status: Status::Verified,
                evidence: EvidenceRef {
                    path: file.path.clone(),
                    line: 1,
                    parser_error: false,
                },
                matrix_source: "PROMPT_INJECTION".to_string(),
            });
        }
    }

    findings
}
