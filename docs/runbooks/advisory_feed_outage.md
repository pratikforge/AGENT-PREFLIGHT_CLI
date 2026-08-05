# Advisory Feed Outage Runbook
1. If the RustSec advisory database is down, `cargo audit` will fail or return stale results.
2. Do not bypass the check. Use a local mirror if available.
3. Wait for the advisory feed to restore before issuing new production approvals.
