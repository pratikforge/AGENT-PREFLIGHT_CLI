# Rollback Runbook
1. Revert the commit containing the faulty policy or code.
2. Re-run `cargo run -- scan` and `cargo run -- approve`.
3. Push the reverted, newly signed contract and code.
