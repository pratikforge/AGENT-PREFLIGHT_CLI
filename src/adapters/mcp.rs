use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::NormalizedFile;
use crate::domain::status::Status;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(files: &[NormalizedFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        for call in &file.calls {
            // stdio server
            if call.callee.contains("stdio_client") {
                findings.push(Finding {
                    rule_id: "mcp-stdio".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }

            // sse server
            if call.callee.contains("sse_client") {
                findings.push(Finding {
                    rule_id: "mcp-sse".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }

            // streamable http server
            if call.callee.contains("http_client") {
                findings.push(Finding {
                    rule_id: "mcp-http".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }

            // missing auth
            if call.callee.contains("mcp_missing_auth") {
                findings.push(Finding {
                    rule_id: "mcp-auth".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }

            // wildcard tool allowlist
            if call.callee.contains("mcp_wildcard_tools") {
                findings.push(Finding {
                    rule_id: "mcp-wildcard-tools".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }

            // local command path
            if call.callee.contains("mcp_local_command") {
                findings.push(Finding {
                    rule_id: "mcp-local-command".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }

            // remote endpoint
            if call.callee.contains("mcp_remote_endpoint") {
                findings.push(Finding {
                    rule_id: "mcp-remote-endpoint".to_string(),
                    status: Status::Verified,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }

            // dynamic endpoint
            if call.callee.contains("mcp_dynamic_endpoint") {
                findings.push(Finding {
                    rule_id: "mcp-dynamic-endpoint".to_string(),
                    status: Status::CannotVerifyStatically,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "MCP".to_string(),
                });
            }
        }
    }
    findings
}
