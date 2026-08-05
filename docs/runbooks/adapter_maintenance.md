# Adapter Maintenance Runbook
1. When a new LLM SDK version is released, update the fixture matrix.
2. Run `cargo run -- scan fixtures/evaluation`.
3. Update `src/adapters/sdk_version_matrix.rs` with the new version bounds.
4. Re-run all tests to confirm the adapter correctly hooks the new SDK structures.
