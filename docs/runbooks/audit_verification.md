# Audit Verification Runbook
1. Locate the audit log file for the agent execution.
2. Run `cargo run -- verify-audit <path_to_audit_log>`.
3. Check for any reported integrity or hash-chain failures.
