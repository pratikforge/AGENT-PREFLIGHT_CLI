use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::domain::contract::Contract;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterceptDecision {
    Allow,
    Deny(String),
    RequireApproval(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocation {
    pub tool_name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeApproval {
    pub rule_id: String,
    pub caller_id: String,
    pub context_hash: String,
    pub expires_at: u64,
    pub nonce: String,
}

pub struct RuntimeInterceptor {
    contract: Option<Contract>,
    audit_log_path: std::path::PathBuf,
}

impl RuntimeInterceptor {
    pub fn new(contract: Option<Contract>, root: &Path) -> Self {
        Self {
            contract,
            audit_log_path: root.join("audit.log"),
        }
    }

    pub fn intercept(&self, invocation: &ToolInvocation) -> InterceptDecision {
        self.intercept_with_approval(invocation, None)
    }

    pub fn intercept_with_approval(
        &self,
        invocation: &ToolInvocation,
        approval: Option<&RuntimeApproval>,
    ) -> InterceptDecision {
        let decision = self.evaluate(invocation);
        match decision {
            InterceptDecision::RequireApproval(ref rule_id) => {
                #[allow(clippy::collapsible_if)] if let Some(appr) = approval { if &appr.rule_id == rule_id {
                        let now = SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        if appr.expires_at > now {
                            // Log and allow
                            self.record_audit(invocation, &InterceptDecision::Allow);
                            return InterceptDecision::Allow;
                        }
                    }
                }
                self.record_audit(invocation, &decision);
                decision
            }
            _ => {
                self.record_audit(invocation, &decision);
                decision
            }
        }
    }

    fn evaluate(&self, invocation: &ToolInvocation) -> InterceptDecision {
        let Some(contract) = &self.contract else {
            return InterceptDecision::Deny("No approved contract found".to_owned());
        };

        let rule_id_match = match invocation.tool_name.as_str() {
            "bash" | "sh" | "powershell" => {
                let args_str = invocation.args.to_string();
                if args_str.contains("rm -rf") {
                    Some("unsafe-rm-rf")
                } else {
                    None
                }
            }
            "git" => {
                let args_str = invocation.args.to_string();
                if args_str.contains("push --force") {
                    Some("unsafe-git-force-push")
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(rule_id) = rule_id_match {
            if let Some(rule) = contract.rules.iter().find(|r| r.id == rule_id) {
                if rule.approval_requirement.contains("runtime") {
                    return InterceptDecision::RequireApproval(rule_id.to_string());
                } else if rule.approval_requirement.contains("deny") {
                    return InterceptDecision::Deny(rule_id.to_string());
                } else {
                    return InterceptDecision::Allow;
                }
            } else {
                return InterceptDecision::Deny(rule_id.to_string());
            }
        }

        InterceptDecision::Allow
    }

    fn record_audit(&self, invocation: &ToolInvocation, decision: &InterceptDecision) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.audit_log_path)
        {
            let timestamp = match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Ok(n) => n.as_secs().to_string(),
                Err(_) => "unknown".to_string(),
            };

            // Redact args for sensitive operations in audit log
            let redacted_args = match decision {
                InterceptDecision::Deny(_) | InterceptDecision::RequireApproval(_) => {
                    serde_json::json!("[REDACTED]")
                }
                _ => invocation.args.clone(),
            };

            let log_entry = serde_json::json!({
                "timestamp": timestamp,
                "tool": invocation.tool_name,
                "args": redacted_args,
                "decision": decision,
            });
            let _ = writeln!(file, "{}", log_entry);
        }
    }
}


