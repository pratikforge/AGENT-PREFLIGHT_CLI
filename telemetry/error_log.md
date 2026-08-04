# Error Log

## 2026-07-19 — Repair task evidence reader lacked EvidenceRef deserialization

```yaml
task: T10_repair_task_packet
error: "The local evidence artifact reader could not deserialize EvidenceRef because the type only derived Serialize."
cause: "Added a read path for an existing serialized domain type without checking its reciprocal serde capability."
rectification: "Before consuming a local serialized domain artifact, verify every nested domain type derives both required serde directions."
verification: "cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test repair_packet"
```

## 2026-07-19 — Repair packet test used heterogeneous fixed-size arrays

```yaml
task: T10_repair_task_packet
error: "Rust rejected a test loop because scan and approve argument arrays had different fixed lengths."
cause: "Tried to compact distinct CLI invocations into one homogeneous array literal."
rectification: "Use explicit command calls or homogeneous Vec<String> values for CLI tests with different argument counts."
verification: "cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test repair_packet"
```

## 2026-07-19 — MSVC linker returned an unspecified failure during T09 regression

```yaml
task: T09_review_and_approve
error: "cargo test reached link.exe, which returned exit code 1 without a concrete cause in the standard output."
cause: "Unknown Windows toolchain or artifact-state failure; no source assertion failed."
rectification: "Before proposing a Build Tools repair, run one verbose diagnostic build in the installed Visual Studio developer environment and inspect the full linker diagnostics."
verification: "cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked -vv"
```

## 2026-07-19 — T09 test assumed an undeclared predicates crate

```yaml
task: T09_review_and_approve
error: "The new integration test referenced predicates::str without predicates in Cargo.toml."
cause: "Used an assertion helper from memory instead of the current locked test dependencies."
rectification: "Inspect the manifest before introducing a test helper. Prefer existing stdlib/assert_cmd output APIs when they meet the assertion need."
verification: "cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test review_approve"
```

## 2026-07-19 — Clippy rejected needless Option::as_deref in parser controls

```yaml
task: T08_claude_adapter
error: "Strict Clippy flagged three needless as_deref calls on Option<&str> values."
cause: "Used an unnecessary dereference conversion after `utf8_text` already produced an Option<&str>."
rectification: "Use the existing Option<&str> directly; run strict Clippy after parser-model changes."
verification: "cargo +1.97.1 clippy --manifest-path agent-preflight/Cargo.toml --locked --all-targets -- -D warnings"
```

## 2026-07-19 — Parser keyword tuple collection needed an explicit type

```yaml
task: T07_google_adk_adapter
error: "Rust could not infer the collected type for the parser's keyword tuple list."
cause: "A collection used by both borrowed and consuming iterator paths lacked a concrete local type."
rectification: "Give multi-use collection intermediates an explicit concrete type when inference spans multiple iterator consumers."
verification: "cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test normalization --test google_adk_adapter"
```

## 2026-07-19 — T06 quality gates were incorrectly chained after a known lock failure

```yaml
task: T06_openai_adapter
error: "A transient Rust incremental-cache lock failed the full test suite, but a subsequent chained Clippy command completed successfully."
cause: "Combined mandatory quality gates with PowerShell separators despite the existing independent-gate rule."
rectification: "Run each required formatter, test, lint, dependency, and audit gate as a separate command and do not accept a later success as evidence for an earlier failed command."
verification: "Run cargo test and cargo clippy independently after the lock diagnostic."
```

## 2026-07-19 — OpenAI adapter test imported an unused symbol

```yaml
task: T06_openai_adapter
error: "The new test imported Finding but did not use it, producing a compiler warning that strict Clippy would reject."
cause: "Copied a broader module import while the assertion only exercised evaluate's returned value."
rectification: "Import only symbols used by a Rust test; run strict Clippy before accepting the task."
verification: "cargo +1.97.1 clippy --manifest-path agent-preflight/Cargo.toml --locked --all-targets -- -D warnings"
```

## 2026-07-19 — Ran Rust format check before formatting the latest test patch

```yaml
task: T05_scan_command
error: "cargo fmt --check reported formatting differences in tests/scan_command.rs."
cause: "Started the verification batch after adding a Rust test but skipped the required formatter step."
rectification: "Always run the pinned formatter immediately after every Rust source or test patch, before any --check, test, or lint command."
verification: "cargo +1.97.1 fmt --manifest-path agent-preflight/Cargo.toml && cargo +1.97.1 fmt --manifest-path agent-preflight/Cargo.toml --check"
```

## 2026-07-19 — Scan uncertainty patch used stale formatted context

```yaml
task: T05_scan_command
error: "apply_patch could not find the expected one-line ScanResult construction after formatter changes."
cause: "Prepared a context-sensitive patch without re-reading the target file after a preceding formatter run."
rectification: "Re-read each target source file immediately before a multi-file behavioral patch, then use its exact formatted context."
verification: "Get-Content -Raw agent-preflight/src/app/scan.rs and main.rs before retrying"
```

## 2026-07-19 — Profile detector reused a non-mutable iterator

```yaml
task: T05_scan_command
error: "Rust rejected `modules.any(...)` because the iterator binding was not mutable after clone-based checks."
cause: "Used one iterator across multiple profile predicates instead of querying the immutable source collection directly."
rectification: "Use independent `files.iter().any(...)` predicates for ordered profile detection; avoid mutable iterator state for static registry checks."
verification: "cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test scan_command"
```

## 2026-07-19 — Parser patch used stale context after formatting

```yaml
task: T04_parser_normalization
error: "apply_patch could not locate the expected collect_python_facts call context in src/infra/parser.rs."
cause: "Prepared the patch from an earlier unformatted view of the file."
rectification: "Inspect the current target file after any formatting or intervening patch before applying a context-sensitive update."
verification: "Get-Content -Raw agent-preflight/src/infra/parser.rs before retrying the targeted patch"
```

## 2026-07-19 — cargo-deny invocation and chained verification masked a failed gate

```yaml
task: T03_safe_repository_reader
error: "cargo-deny rejected --manifest-path, while a later cargo audit command returned success and caused the combined PowerShell command to exit successfully."
cause: "Assumed cargo-deny accepts Cargo's manifest flag and chained independent quality gates without preserving each command's exit status."
rectification: "Run cargo-deny from the crate working directory using its supported command form, and execute each mandatory quality gate as an independent command."
verification: "cargo-deny check (workdir: agent-preflight)"
```

## 2026-07-19 — Clippy rejected default construction of a unit reader

```yaml
task: T03_safe_repository_reader
error: "cargo clippy -- -D warnings rejected SafeReader::default() because SafeReader is a unit struct."
cause: "Tests used a generic default-construction habit instead of the direct unit-struct value."
rectification: "Use SafeReader directly in tests and preserve the strict lint gate."
verification: "cargo +1.97.1 clippy --manifest-path agent-preflight/Cargo.toml --locked --all-targets -- -D warnings"
```

## 2026-07-19 — Safe-reader depth guard was hidden by walker cutoff

```yaml
task: T03_safe_repository_reader
error: "The initial max_depth(MAX_DEPTH + 1) walker configuration silently omitted a 33-level source, returning Ok([]) instead of ReaderError::DepthExceeded."
cause: "Assumed ignore::WalkBuilder would yield the entry that crossed the configured depth boundary."
rectification: "Remove the walker depth cutoff for the bounded source set and enforce the depth check on every normalized repository-relative path before parsing."
verification: "cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test safe_reader"
```

## 2026-07-19 — Governance lookup and PowerShell inspection errors

### What happened

- An initial `rg --files -g AGENTS.md` lookup did not include hidden directories, so it missed the repository governance file at `.agents/AGENTS.md`.
- Two read-only PowerShell inspection commands failed before any project code or configuration was changed: one used an unbraced variable immediately before `:`, and one passed a complex inline Python command with unsafe quoting.

### Cause

- `rg --files` ignores hidden directories unless `--hidden` is supplied.
- PowerShell requires `${variable}` when a colon immediately follows a variable name, and complex multi-line Python is not safe as a single quoted `-c` argument.

### Resolution and prevention

- Read `.agents/AGENTS.md` before planning and use `rg --files --hidden -g AGENTS.md` for future governance discovery.
- Use `${variable}` before a colon and a PowerShell here-string piped to Python for multi-line inspection scripts.
- No repository source, package, CI, or product configuration was modified by the failed commands.

## 2026-07-19 — Invalid multi-file documentation patch

### What happened

- A large `apply_patch` request was rejected before changes were applied because lines inside a Markdown command block were missing the required `+` addition prefix.

### Cause

- The patch combined several new documents and was not mechanically checked for patch-line prefixes before submission.

### Resolution and prevention

- Apply planning documents in smaller patches and ensure every line in each new-file hunk begins with `+`.
- No PRD, plan, task, CI, dependency, or product source file was changed by the rejected patch.
# 2026-07-19 — Combined quality-check command treated an expected no-match as failure

- **What happened:** A parallel verification call combined `git diff --check` with `rg -n "\\s+$"`. `rg` exits with code 1 when it finds no trailing whitespace, which caused the orchestration call to report failure even though that is the desired outcome.
- **Exact procedure:** `git diff --check; rg -n "\\s+$" 'spec\\agent-preflight' 'tasks\\agent-preflight'`
- **Prevention:** Run expected-no-match searches separately and explicitly accept exit code 1 as a clean result; do not combine them with required-success checks.

# 2026-07-19 — Whitespace check included generated graph JSON

- **What happened:** The trailing-whitespace search covered generated `graphify-out/graph.json` files. Their large pretty-printed output overwhelmed the shared verification call and timed out.
- **Exact procedure:** `rg -n "\\s+$" 'spec\\agent-preflight' 'tasks\\agent-preflight'` after Graphify generated its outputs.
- **Prevention:** Exclude generated output directories from text-quality searches and run output-heavy checks separately with an appropriate timeout.

# 2026-07-19 — Empty-directory removal was blocked by command policy

- **What happened:** A PowerShell `Remove-Item` request to remove two verified-empty obsolete directories was rejected by the command policy before execution.
- **Exact procedure:** `Remove-Item -LiteralPath $target -Force` after checking the directories were empty.
- **Prevention:** Use `apply_patch` for file deletion and leave empty directories in place when the command policy blocks directory removal; do not retry with alternate destructive shell commands.

# 2026-07-19 — Large multi-file task-plan patch had an unprefixed Markdown line

- **What happened:** A batch replacement of task-plan Markdown files was rejected because a line inside a fenced command block did not begin with `+` in an `*** Add File` hunk.
- **Exact procedure:** One `apply_patch` request deleted and re-added T00–T04 detailed task plans.
- **Prevention:** Replace detailed task-plan files one at a time (or in very small groups) and validate every added fenced-code line has a `+` prefix before submitting the patch.

# 2026-07-19 — Single-file detailed plan repeated the fenced-command prefix error

- **What happened:** T01 replacement was rejected because the command inside the `text` fenced block lacked the `+` prefix required by the add-file hunk.
- **Exact procedure:** Replacing `tasks/agent-preflight/T01_workspace_quality_baseline.md` through `apply_patch`.
- **Prevention:** Avoid `text` fences in add-file patches; represent commands in YAML lists or inline code so each source line can be visually validated as an added line.

# 2026-07-19 — Graphify refused a smaller replacement graph after task-plan rewrites

- **What happened:** `graphify update tasks\\agent-preflight --no-cluster` refused to overwrite the old 136-node graph with a new 127-node graph after task documents were replaced.
- **Exact procedure:** Graphify update executed in parallel with documentation quality checks.
- **Prevention:** When intentional deletions or rewrites reduce graph nodes, run Graphify separately with `--force` after confirming the target scope. Do not treat a smaller graph as an automatic data-loss error.

# 2026-07-19 — Combined T00 environment discovery obscured the failing prerequisite

- **What happened:** A parallel command combined worktree inspection, a Graphify query, and Cargo/Rust discovery. The overall command exited 1 and only returned partial output, so it did not identify which prerequisite failed.
- **Exact procedure:** Parallel `git status`, `graphify query`, and `Get-Command cargo; cargo --version; rustc --version` checks.
- **Prevention:** During environment discovery, run prerequisite checks separately from graph/worktree inspection. A missing tool must be diagnosed with an isolated command before attempting implementation.

# 2026-07-19 — T00 test could not reach the intended red state because the MSVC linker is absent

- **What happened:** The Rust 1.97.1 toolchain installed successfully, but `cargo test` failed before compiling the planned missing parser module because the Windows MSVC target cannot find `link.exe`. One parallel dependency also reported a transient locked output file during the failed compilation.
- **Exact procedure:** `cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test parser_compatibility`
- **Prevention:** On Windows, verify the selected Rust target has an available linker (`link.exe` for MSVC or a configured GNU linker) before declaring a Rust test as a valid red/green TDD result. Do not install system build tools without explicit user approval.

# 2026-07-19 — Linker-alternative discovery still treated an absent optional command as fatal

- **What happened:** The command successfully found the bundled `rust-lld.exe`, but querying optional `clang` and `lld-link` commands in the same PowerShell invocation returned exit code 1 when they were absent.
- **Exact procedure:** `Get-Command clang` and `Get-Command lld-link` after locating Rust's bundled linker.
- **Prevention:** For optional executable discovery, use a helper that always exits 0 when a command is absent. Keep successful linker-path discovery separate from optional-tool checks.

# 2026-07-19 — Adding Rust's bundled linker directory to PATH did not override the MSVC linker name

- **What happened:** Rust still invoked `link.exe` even after the directory containing `rust-lld.exe` was prepended to PATH, because the MSVC target specifically requests the `link.exe` program name.
- **Exact procedure:** Prepending the Rust toolchain `bin` directory to PATH, then running the T00 test.
- **Prevention:** Do not assume a bundled linker is selected by PATH alone. Test an explicit per-command Rust linker override before relying on it; keep the override local and do not commit an absolute developer-machine path.

# 2026-07-19 — Bundled `rust-lld` cannot replace the MSVC SDK import libraries

- **What happened:** An explicit `rust-lld.exe` override was selected, but linking still failed because the Windows SDK import libraries (`kernel32.lib`, `ntdll.lib`, and others) are not installed.
- **Exact procedure:** Setting `RUSTFLAGS=-C linker=<toolchain rust-lld.exe>` for the T00 test.
- **Prevention:** Treat the Windows SDK libraries as part of the MSVC build prerequisite. A bundled linker alone is insufficient; do not spend further implementation attempts on local Rust compilation until the Microsoft C++ Build Tools and Windows SDK are installed or the user authorizes a different target/toolchain.

# 2026-07-19 — Build Tools installer exceeded the bounded command window

- **What happened:** The approved `winget` installation did not complete within the 60-second command limit and the command timed out without final installer output.
- **Exact procedure:** `winget install --id Microsoft.VisualStudio.2022.BuildTools ... --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended`
- **Prevention:** For long-running approved installers, launch once with a bounded wait, then check installer process state and installed prerequisites separately. Do not rerun the installer merely because the first bounded wait timed out.

# 2026-07-19 — Rust formatter was omitted from the minimal toolchain profile

- **What happened:** `cargo +1.97.1 fmt --check` could not run because `cargo-fmt.exe` is not included in the Rust toolchain's `minimal` profile.
- **Exact procedure:** Formatting check immediately after the first T00 parser implementation.
- **Prevention:** Before the first formatting or lint gate, explicitly install and verify the `rustfmt` and `clippy` components for the pinned Rust toolchain. The `minimal` profile alone is insufficient for the project's quality commands.

# 2026-07-19 — T00 parser patch reached the quality gate before formatting

- **What happened:** The strict `cargo fmt --check` gate found formatting differences in the new parser module and the pre-existing parser compatibility test import order, so the combined test command stopped before compilation.
- **Exact procedure:** Formatting verification after adding `src/parser_spike.rs`.
- **Prevention:** After every Rust patch, run `cargo fmt` (not only `cargo fmt --check`) before the combined test and lint command. Keep the check gate as verification, not as the first formatter invocation.

# 2026-07-19 — Task-status read used an outdated T00 filename

- **What happened:** The T00 implementation checks passed, but the follow-up documentation read referenced `T00_parser_compatibility_spike.md`, which does not exist; the current task document has a different name.
- **Exact procedure:** Reading the T00 task plan and the checklist in one PowerShell command.
- **Prevention:** Before reading or patching a task document by name, discover its current path with `rg --files tasks/agent-preflight`. Do not infer filenames from task titles or earlier drafts.

# 2026-07-19 — T00 Graphify query ran before its project graph existed

- **What happened:** The required parser-compatibility Graphify query failed because `agent-preflight/graphify-out/graph.json` had not yet been generated.
- **Exact procedure:** `graphify query "parser compatibility" --graph agent-preflight/graphify-out/graph.json --budget 1200`.
- **Prevention:** For a new project scope, run `graphify update <scope> --no-cluster` before any query that names that scope's graph file. Verify the graph path exists before submitting the query.

# 2026-07-19 — Pinned cargo-deny installation exceeded the command window

- **What happened:** Building the required pinned `cargo-deny` tool did not finish within the bounded command window; the command timed out before reporting a final installation result.
- **Exact procedure:** `cargo +1.97.1 install cargo-deny --version 0.20.2 --locked`.
- **Prevention:** Treat long cargo tool installations like other long installers: start once, inspect the active cargo process and installed binary after timeout, and never restart it merely because the bounded command returned before completion.

# 2026-07-19 — Foreground retry for cargo-deny was also killed by the command limit

- **What happened:** After confirming the first foreground installation had ended, a legitimate retry again exceeded the terminal limit; its child compiler processes later ended without installing `cargo-deny`.
- **Exact procedure:** A second bounded `cargo +1.97.1 install cargo-deny --version 0.20.2 --locked` invocation.
- **Prevention:** Do not use a foreground terminal command for a Cargo tool build known to exceed the execution limit. Launch exactly one hidden background process with redirected logs, then inspect its process, exit state, and binary separately.

# 2026-07-19 — T00 audit policy was missing and the first remediation patch was malformed

- **What happened:** `cargo-deny` rejected the project because no `deny.toml` policy existed to allow the locked graph's MIT, Apache-2.0, Unicode-3.0, and Unlicense terms. The first patch to add that policy was rejected because an add-file TOML line lacked `+`; the immediate retry assumed partial changes and failed its context match.
- **Exact procedure:** `cargo-deny --manifest-path agent-preflight/Cargo.toml check advisories bans licenses sources`, followed by two `apply_patch` attempts for `agent-preflight/deny.toml`.
- **Prevention:** Create the minimal planned policy configuration before a required dependency-policy command. For every add-file hunk, validate every line starts with `+`; after any rejected patch, inspect the files before retrying rather than assuming a partial application.

# 2026-07-19 — Initial cargo-deny allow-list included an unused license

- **What happened:** The new policy passed but emitted `license-not-encountered` for `Unicode-3.0`, which is not present in the locked dependency graph.
- **Exact procedure:** First `cargo-deny ... check advisories bans licenses sources` run after adding `deny.toml`.
- **Prevention:** Build license allow-lists from the actual lockfile audit output, not from a broader anticipated ecosystem set. A successful exit with a policy warning is not a clean quality result.

# 2026-07-19 — cargo-audit was invoked with the wrong executable form

- **What happened:** The planned `cargo-audit --file ...` command failed because the installed plugin exposes the lockfile option through `cargo audit --file ...`, not the direct executable invocation used here.
- **Exact procedure:** `cargo-audit --file agent-preflight/Cargo.lock --deny warnings`.
- **Prevention:** For Cargo plugins, verify their documented invocation form after installation. Prefer `cargo <plugin> ...` in plans and automation unless the tool's own help explicitly confirms direct-binary flags are supported.

# 2026-07-19 — Windows build artifact was locked during the T01 green test

- **What happened:** Cargo could not remove a prior `target/debug/deps` object file because another Windows process held it open, preventing compilation of the new CLI binary.
- **Exact procedure:** T01 `cargo test --manifest-path agent-preflight/Cargo.toml --locked --test cli_smoke` immediately after adding `src/main.rs`.
- **Prevention:** After a Windows file-lock build failure, inspect active Rust, Cargo, editor, and indexer processes before retrying. Do not delete target artifacts or bypass the failing test while the lock owner is unknown.

# 2026-07-19 — Optional process discovery returned a misleading failure code

- **What happened:** The lock-diagnostic command printed active Node processes but returned exit code 1 because one or more optional process names were absent.
- **Exact procedure:** Combined `Get-Process -Name cargo,rustc,graphify,node,Code -ErrorAction SilentlyContinue` lookup.
- **Prevention:** Probe optional process names separately or normalize absent-name outcomes before treating a diagnostics command as failed.

# 2026-07-19 — Persistent-goal continuation was answered with repeated status-only messages

- **What happened:** After brief final responses, the active implementation goal automatically resumed. I replied with the same status phrase rather than taking a concrete next action or remaining silent, which created a visible message loop.
- **Exact procedure:** Multiple goal-triggered continuations during the Agent Preflight implementation.
- **Prevention:** A goal-triggered continuation must begin with a concrete tool action, a meaningful progress update, or a genuine blocker assessment. Never emit a placeholder final response such as "Continuing the implementation plan." When pausing for the user, end once and wait for explicit input.

# 2026-07-19 — Rust incremental cache could not finalize on Windows

- **What happened:** The T02 contract test passed, but Rust could not finalize its incremental-compilation cache because of an access-denied error.
- **Exact procedure:** `cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test contract_model`.
- **Prevention:** Treat a successful test with an incremental-cache permission warning as green functional evidence but not a clean environment result. Before later performance-sensitive builds, inspect for file-locking/indexer activity; never delete build output blindly.

# 2026-07-19 — New T02 dependencies changed the locked license and duplicate graph

- **What happened:** `cargo deny` rejected `Unicode-3.0`, newly introduced transitively by the pinned derive stack, and found the required `syn` v2/v3 split between pinned Clap and Serde/Thiserror dependencies.
- **Exact procedure:** T02 full quality gate after adding serde, YAML, SHA-256, and typed-error dependencies.
- **Prevention:** After adding a planned dependency, run the complete dependency-policy gate before declaring the feature complete. Add only exact, documented license allowances and version skips that the lock graph proves necessary; never broadly downgrade duplicate checks.

# 2026-07-19 — T03 reader-test patch assumed an undeclared dependency line

- **What happened:** The first T03 patch was rejected because it tried to update an `ignore` dependency line that had not yet been added to the manifest.
- **Exact procedure:** Combined manifest/test patch for the safe-reader red scaffold.
- **Prevention:** Before updating a dependency section, read the current manifest and add new entries relative to verified existing lines. Do not include speculative update hunks in a multi-file patch.

# 2026-07-19 — T10 path sanitizer violated strict Clippy

- **What happened:** The repair-packet inline sanitizer used two consecutive single-character `str::replace` calls for carriage returns and newlines. Strict Clippy rejected it with `collapsible_str_replace` under `-D warnings`.
- **Exact procedure:** `cargo +1.97.1 clippy --manifest-path agent-preflight/Cargo.toml --locked --all-targets -- -D warnings` during the T10 quality gate.
- **Prevention:** When replacing multiple individual characters with the same output, use the combined character-pattern form such as `replace(['\r', '\n'], " ")`. Treat all strict Clippy warnings as implementation defects and rerun formatting before retesting.

# 2026-07-19 — T10 lint-fix patch used pre-format source context

- **What happened:** The first patch for the Clippy correction failed because it assumed the formatter's previous line layout rather than the current file content.
- **Exact procedure:** First `apply_patch` attempt for `agent-preflight/src/render/repair_packet.rs` after the strict Clippy failure.
- **Prevention:** Re-read every patch target after formatting or a failed patch, then construct the next hunk from its exact current context. This is covered by AGENTS rules 35 and 47.

# 2026-07-19 — T10 full test gate encountered a Windows Cargo object lock

- **What happened:** Cargo could not remove two normalization-test object files because another process held them open (`os error 32`). The source had already formatted successfully; no artifacts were deleted.
- **Exact procedure:** `cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked` after the T10 sanitizer correction.
- **Prevention:** Inspect active Cargo, Rust compiler, editor, and indexing processes before a single clean retry. Preserve `target` rather than deleting it to hide an environmental lock.

# 2026-07-19 — T10 lock diagnostic used invalid PowerShell interpolation

- **What happened:** The process-diagnostic script placed `$name` immediately before a colon inside a double-quoted string, which PowerShell parsed as an invalid scoped-variable reference.
- **Exact procedure:** First optional-process inspection after the T10 Cargo lock.
- **Prevention:** Use `${name}:` when a PowerShell interpolated variable is immediately followed by a colon. Confirm diagnostics execute successfully before using their output.

# 2026-07-19 — T11 initially prioritized unsupported profile over parse uncertainty

- **What happened:** A malformed agent source was classified as unsupported because the parser could not recover its imports, producing CI exit 3 instead of the required uncertainty exit 4.
- **Exact procedure:** Initial green attempt: `cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test verifier`.
- **Prevention:** In verification result precedence, handle parser uncertainty before profile detection. A parse error is an explicit inability to verify, not evidence that the repository lacks a supported SDK.

# 2026-07-19 — T12 malformed-fixture profile was inferred incorrectly

- **What happened:** The evaluation matrix expected the parser fixture to be unsupported, but its valid source file imports the OpenAI SDK before a separate malformed file introduces parse uncertainty.
- **Exact procedure:** First `cargo +1.97.1 test --manifest-path agent-preflight/Cargo.toml --locked --test fixture_matrix` run.
- **Prevention:** Read fixture contents before recording expected profile and status values. Folder names are organizational, not behavioral contracts.

# 2026-07-19 — T12 Unix fixture runner could not execute on the Windows host

- **What happened:** The local `bash` command delegated to WSL, but no Linux distribution was installed, so the Unix evaluation runner could not start.
- **Exact procedure:** `bash agent-preflight/scripts/evaluate-fixtures.sh` on the Windows development host.
- **Prevention:** Execute platform-specific runners only where their runtime is available. Validate the script syntax locally when possible and rely on the matching GitHub Actions matrix job for execution evidence.

# 2026-07-19 — Generated Cargo and Graphify output was left visible to Git

- **What happened:** The Agent Preflight directory had no `.gitignore` before builds, package checks, Graphify updates, and tool-install logs were created. Source Control consequently displayed 5,561 entries, of which 5,296 were Cargo target files.
- **Exact procedure:** User review of Source Control after the complete implementation run.
- **Prevention:** Add targeted ignore rules before generating project output, then verify the untracked-file view contains only intended source, tests, fixtures, docs, and configuration.

# 2026-07-19 — Google ADK red-test verification chained two gates

- **What happened:** Formatting and the red test were invoked in one PowerShell command during the real-world ADK adapter improvement.
- **Prevention:** Run each Rust verification gate independently so its status remains unambiguous.

# 2026-07-19 — External audit search was too broad and assumed a test directory

- **What happened:** A first-pass `rg` searched every OpenAI example and test for approval markers, producing far more output than the targeted audit needed; it also assumed the Claude SDK clone had a top-level `tests/` directory.
- **Exact procedure:** Broad multi-path `rg` immediately after cloning the two official SDK repositories.
- **Prevention:** For cloned external repositories, list candidate paths first, then inspect at most a small fixed number of named examples or tests per adapter. Treat a missing optional directory as a diagnostic result, not a search target.

# 2026-07-19 — Adapter file name was assumed during inspection

- **What happened:** An adapter-inspection command addressed `src/adapters/claude.rs`, but the actual module has a different filename.
- **Exact procedure:** First direct read of all three adapter modules during the cross-adapter audit.
- **Prevention:** Enumerate adapter module paths with `rg --files` before reading a named adapter; do not infer filenames from their public module names.

# 2026-07-19 — Claude contract correction left an integration fixture stale

- **What happened:** The full suite found a scan-command test whose inline Claude source still used `dontAsk` without the newly required literal allow-list.
- **Exact procedure:** Full locked Cargo test run after tightening the Claude permission contract.
- **Prevention:** When a static-control contract changes, locate and update unit, fixture-matrix, and command-level inline fixtures before accepting the new boundary.

# 2026-07-19 — GitHub contents API was unavailable through the browser tool

- **What happened:** The browser safety layer rejected direct `api.github.com/repos/.../contents/...` requests during metadata-only corpus inventory.
- **Exact procedure:** Attempted GitHub Contents API reads for three selected Awesome LLM Apps paths.
- **Prevention:** Do not retry blocked API endpoints. Use linked GitHub file views or a deliberately sparse local clone after path selection instead.

# 2026-07-19 — OpenAI adapter green verification chained formatter and test

- **What happened:** The formatter and targeted OpenAI adapter test were invoked in one PowerShell command after the risk-contract correction.
- **Exact procedure:** `cargo +1.97.1 fmt ...; cargo +1.97.1 test ... --test openai_adapter`.
- **Prevention:** Run formatter and tests as separate commands, including targeted TDD gates; the independent-gate rule applies before full quality verification as well.

# 2026-07-19 — OpenAI risk-boundary correction invalidated the old repair demo

- **What happened:** The full test suite found that the demo contract expected an unguarded OpenAI function tool to produce a failed finding, but the corrected adapter now reports uncertainty without a risk contract.
- **Exact procedure:** Full locked Cargo test run after the false-positive correction.
- **Prevention:** When replacing a status contract, identify all downstream demo and task workflows that require the removed status, then migrate them to a deterministic failure case before accepting the change.

# 2026-07-19 — OpenAI risk-boundary correction left repair-packet fixtures stale

- **What happened:** The full suite found two repair-packet tests still requesting an OpenAI failure after that adapter correctly changed unguarded tools to uncertainty.
- **Exact procedure:** Full locked Cargo test run after migrating the demo contract.
- **Prevention:** Treat every status-contract migration as cross-layer: migrate demo, task-packet, approval, and verification fixtures together before the next full suite.

# 2026-07-19 — Diagnostic test failure was masked by a trailing inspection command

- **What happened:** A focused failing test and a file inspection were separated with a PowerShell semicolon, so the final inspection command returned success even though the test had failed.
- **Exact procedure:** Focused `scan_command` test followed by `Get-Content` to locate its stale assertion.
- **Prevention:** Run a failing test and any follow-up source inspection as independent commands. Never use a trailing successful diagnostic command to mask a test exit code.

# 2026-07-19 — OpenAI status migration left the CI verifier scenario stale

- **What happened:** The full suite found the verifier test still expected CI failure for an unguarded OpenAI function tool, after that shape was correctly changed to uncertainty.
- **Exact procedure:** Full locked Cargo test run after scan-command fixture migration.
- **Prevention:** Include CI verifier scenarios in the complete status-contract migration checklist before rerunning the full suite.

# 2026-07-19 — Error-log patch used stale exact context

- **What happened:** An `apply_patch` intended to record the verifier failure used an outdated prevention sentence and was rejected before writing anything.
- **Exact procedure:** First error-log patch after the verifier-suite failure.
- **Prevention:** Before appending to a frequently edited governance file, refresh its tail and patch against the exact current text.

# 2026-07-19 — OpenAI status correction left identical adapter branches

- **What happened:** Strict Clippy detected that the non-`function_tool` and empty-argument branches both returned `CannotVerifyStatically` after the risk-contract correction.
- **Exact procedure:** Strict Clippy gate after full tests passed.
- **Prevention:** After collapsing status outcomes, simplify adjacent conditional branches before running the lint gate; do not suppress structural lint findings.

# 2026-07-19 — Generated OpenAI scan evidence was read without a size bound

- **What happened:** A full raw read of the generated `.agent-preflight/evidence.yaml` from the sparse OpenAI course scan exceeded the useful output budget and was truncated.
- **Exact procedure:** `Get-Content -Raw` of the complete scan artifact immediately after `agent-preflight scan`.
- **Prevention:** Inspect generated evidence through file size, targeted status/rule counts, structured parsing, or bounded selections. Never use an unrestricted raw read for a generated scan artifact.

# 2026-07-19 — Graphify query was attempted before the root index existed

- **What happened:** A Graphify query for repository planning failed because `graphify-out/graph.json` did not exist at the workspace root.
- **Exact procedure:** Initial Graphify discovery command while locating existing corpus-research documents.
- **Prevention:** Confirm the index exists before querying. If it is absent, run a scoped update first or use a bounded direct listing only for the immediate documentation-discovery fallback.

# 2026-07-19 — Evidence-matrix path was resolved from the wrong project level

- **What happened:** A source-inspection command ran from `agent-preflight/` but addressed `spec/agent-preflight/ADAPTER_EVIDENCE_MATRIX.md` as though it were at the workspace root.
- **Exact procedure:** Initial contract inspection before the OpenAI adapter expansion.
- **Prevention:** Resolve workspace documentation paths relative to the current project directory (`../spec/...` here), or confirm their locations with a bounded file listing before reading.

# 2026-07-19 — OpenAI MCP-only fixture bypassed the OpenAI profile

- **What happened:** The new fixture imported only `agents.mcp.MCPServerStdio`, while profile detection recognized only the exact module `agents`; the scanner returned unsupported and never invoked the new MCP rule.
- **Exact procedure:** End-to-end static scan of `fixtures/openai/mcp_with_always_approval` after the OpenAI MCP rule was added.
- **Prevention:** Every new adapter construct from a submodule must include a profile-detection regression test and an end-to-end fixture scan. Match documented SDK submodules deliberately rather than assuming root-package imports.

# 2026-07-19 — Claude status correction left repair-packet failure seeds stale

- **What happened:** Two repair-packet tests still used a query with no explicit permission mode after that shape was correctly reclassified as `CannotVerifyStatically`; the task command rejected it because it creates packets only for failed findings.
- **Exact procedure:** Full locked Cargo suite after correcting Claude Agent SDK missing-mode semantics.
- **Prevention:** When a rule status becomes uncertainty, migrate every repair-packet input to a documented deterministic failure—in this case literal `permissionMode: 'bypassPermissions'`—before accepting the new contract.

# 2026-07-19 — Claude status correction left a CI verifier failure seed stale

- **What happened:** The CI verification test still expected exit code 1 from a query with no explicit permission mode. After the correction, that source correctly returned uncertainty and CI exit code 4.
- **Exact procedure:** Full locked Cargo suite after migrating the repair-packet fixtures.
- **Prevention:** Migrate CI verifier inputs alongside demo and repair-packet inputs whenever an adapter status contract changes. Use an explicit documented failure such as literal `permissionMode: 'bypassPermissions'`.

# 2026-07-19 — Nested-source inspection repeated the known path mistake

- **What happened:** A post-lint inspection addressed `src/adapters/google_adk.rs` from the workspace root instead of the nested `agent-preflight/` project, so the command did not read the intended source.
- **Exact procedure:** Inspecting the strict-Clippy finding after the full test suite.
- **Prevention:** For every nested-project command, state the target as either an explicit nested path from the workspace root or set the nested working directory—never mix the two conventions in one command.

# 2026-07-19 — Broad documentation patch used brittle multi-hunk context

- **What happened:** A documentation update with many unrelated hunks was rejected because one long Markdown line did not match the patch context exactly.
- **Exact procedure:** Recording the newly admitted OpenAI local-runtime and hosted-MCP evidence.
- **Prevention:** Split documentation changes into narrow patches grouped by one file and one nearby heading; refresh the exact affected lines immediately before applying each patch.

# 2026-07-19 — Claude plan-mode branch duplicated the verified outcome

- **What happened:** The plan-mode addition created two adjacent `Verified` branches, which strict Clippy rejected.
- **Exact procedure:** Full quality gate after adding direct Claude plan-mode coverage.
- **Prevention:** Apply the existing status-branch simplification rule when adding a second positive contract: combine predicates that intentionally produce the same status before running Clippy.

# 2026-07-19 — Nested test command and trailing Graphify masked a failed gate

- **What happened:** A documentation test used `--manifest-path Cargo.toml` from the workspace root, where that manifest does not exist. A subsequent Graphify command in the same shell sequence returned success and masked the failed test's exit status.
- **Exact procedure:** Verifying the version-compatibility documentation baseline.
- **Prevention:** Run each verification gate from its project working directory as an independent command. Never append a non-verification command after a test or gate in the same shell invocation.

# 2026-07-19 — First Windows release build failed without a decisive diagnostic

- **What happened:** The first `cargo build --release --locked` stopped during a dependency build script with only exit code 1 visible. The immediate verbose reproduction completed successfully, confirming no source change was justified.
- **Exact procedure:** Local release-binary smoke check after adapter compatibility auditing.
- **Prevention:** When a native release build fails before project compilation, reproduce once with bounded verbose output and capture the toolchain environment before changing source or dependency configuration. Do not label a transient build failure a product defect.

# 2026-07-19 — Workflow diff was inspected from the nested Rust project

- **What happened:** A `git diff` path for `.github/workflows` was evaluated from `agent-preflight/`, where that relative path does not exist, producing no meaningful diff output.
- **Exact procedure:** Verifying the cross-platform release-build CI addition.
- **Prevention:** Use the workspace root for Git paths outside the nested Rust project; an empty path-filtered diff must not be treated as verification without confirming the path resolves.

# 2026-07-20 — Optional pre-commit probe returned a misleading failure

- **What happened:** A combined diagnostic read `.pre-commit-config.yaml` and then ran `Get-Command pre-commit -ErrorAction SilentlyContinue`. Because `pre-commit` was not on PATH, PowerShell returned a nonzero exit code even though the configuration read succeeded.
- **Exact procedure:** Checking whether the repository pre-commit executable was available before committing the completed Agent Preflight source.
- **Prevention:** Normalize optional `Get-Command` probes with an explicit success fallback, and run prerequisite discovery separately from configuration inspection. Treat an absent optional executable as an environment limitation, not as a repository failure.

# 2026-07-20 — Pre-commit hooks ignored the pinned Rust toolchain

- **What happened:** `uv tool run pre-commit run --all-files` invoked plain `cargo` from the local hook definitions. Rust selected `1.94.0`, while `agent-preflight` requires `1.97.1`, so the clippy and test hooks failed before compiling the project.
- **Exact procedure:** Running the repository pre-commit suite after the explicit `cargo +1.97.1` quality gates had passed.
- **Prevention:** Rust hooks must invoke the pinned toolchain explicitly (`cargo +1.97.1 ...`) or use a verified repository toolchain-selection wrapper. A passing explicit command does not validate a hook that selects a different compiler.

# 2026-07-20 — Full-workspace pre-commit normalized existing documentation

- **What happened:** Running the complete pre-commit suite over the workspace changed trailing whitespace and missing final-newline formatting in several newly staged Markdown/YAML/source fixtures. The hooks correctly stopped with a nonzero status so the fixes could be reviewed and staged.
- **Exact procedure:** `uv tool run pre-commit run --all-files` before the full-workspace commit.
- **Prevention:** Run the formatting hooks before the final staged-diff review, then stage their deterministic formatting changes. Do not treat a hook that modifies files as a passing gate until it is rerun and passes.

# 2026-07-20 — Push destination was not confirmed before publishing

- **What happened:** The local `origin` remote pointed to `pratikforge/Universal_Project_Boilerplate`, but the intended Agent Preflight repository had not been created. The complete workspace was pushed to that existing remote before confirming the destination.
- **Exact procedure:** `git push origin main` after interpreting the request to push the codebase without first asking for the new repository URL.
- **Prevention:** Before any first push for a project, verify the intended repository URL and whether it is empty/approved. Never infer the destination from an inherited `origin` remote.

# 2026-07-20 — README add-file patch missed required prefixes

- **What happened:** The first large root README `apply_patch` was rejected because lines inside fenced command examples did not begin with `+` in the add-file hunk.
- **Exact procedure:** Adding the public README before rebuilding the clean publication history.
- **Prevention:** For every multi-line Markdown add-file patch, validate that every content line—including fenced examples—has the required `+` prefix before submitting it. A rejected patch must be inspected and retried with a smaller hunk.

# 2026-07-20 — Orphan publication branch was attempted with governance files unstashed

- **What happened:** `git switch --orphan agent-preflight-main` was blocked because `.agents/AGENTS.md` and `telemetry/error_log.md` had uncommitted local updates that were intentionally excluded from the public repository.
- **Exact procedure:** Starting the clean 14-commit publication history before isolating excluded governance files.
- **Prevention:** Before creating an orphan publication branch, identify excluded modified files and preserve them in a named local stash; never commit sensitive governance or telemetry files merely to unblock branch setup.

# 2026-07-20 — Empty orphan index was redundantly cleared

- **What happened:** After switching to the empty `agent-preflight-main` orphan branch, `git rm -r --cached .` failed with `pathspec '.' did not match any files` because the index already contained no paths.
- **Exact procedure:** Attempted to clear inherited tracked files while preparing a filtered public publication branch.
- **Prevention:** Inspect `git ls-files` immediately after an orphan switch. If the index is empty, restore only the explicitly approved publication paths and do not run a blanket cached removal.

# 2026-07-20 — GitHub Actions omitted clippy from an inline YAML map

- **What happened:** All three matrix jobs failed at the clippy step with `cargo-clippy is not installed for the toolchain 1.97.1`. The workflow used `with: { components: rustfmt, clippy }`; YAML parsed `clippy` as another map key instead of part of the components input.
- **Exact procedure:** First public push of the cross-platform Agent Preflight quality workflow.
- **Prevention:** Quote comma-separated action inputs (`components: "rustfmt, clippy"`) and guard the declaration with `agent-preflight/scripts/verify-ci-workflow.ps1`, run both in CI and pre-commit.

# 2026-07-20 — Fixture checks depended on implicit process exit behavior

- **What happened:** The macOS and Ubuntu fixture steps failed with `Permission denied` because `evaluate-fixtures.sh` was committed with mode `100644` but CI executed it directly. The Windows fixture step completed all four scans but then failed with `Process completed with exit code 1`: its parser fixture intentionally returned a non-zero result, and that residual native exit code escaped the dot-sourced PowerShell script.
- **Exact procedure:** Second public GitHub Actions run after correcting the Rust toolchain component declaration.
- **Prevention:** Invoke the Unix fixture script with `bash ./agent-preflight/scripts/evaluate-fixtures.sh`; explicitly `exit 0` after the Windows evaluator accepts its expected non-zero case. `verify-ci-workflow.ps1` asserts both invariants and runs in pre-commit and before every CI quality job.

# 2026-08-03 — Executing a commit without running the planned tool edit and local TDD verification

- **What happened:** The CI failed with a hardcoded path error. I analyzed it, correctly identified the fix for scripts/verify-ci-workflow.ps1, wrote an implementation plan, but then directly ran git commit without ever executing multi_replace_file_content to actually apply the fix to the script. The commit naturally pushed the still-broken script, causing the exact same CI failure again.
- **Exact procedure:** Planned a fix for a .ps1 script but omitted the tool execution step, then pushed the unmodified file.
- **Prevention:** Before running git commit, explicitly verify that the planned tool call to edit the file has actually been executed, and run the script locally (TDD) as a final guardrail before committing.
# 2026-08-03 — Incorrect repository path and missing cache-busting in installation script instructions

- **What happened:** Provided a raw irm command without ?v=1 cache-busting parameter for a rapidly updated script, and possibly an incorrect repository name in the path.
- **Cause:** Assumed GitHub's aw.githubusercontent.com would serve the latest commit instantly, but it caches for 5 minutes.
- **Prevention:** Always append ?v=1 (or another dynamic cache-busting string) to raw GitHub URLs when the user needs to download a script that was just updated. Always double check exact repository names (e.g. AGENT-PREFLIGHT_CLI vs AGENT-PREFLIGHT-CLI).

# 2026-08-03 — cargo install failed due to incorrect Rust toolchain version

- **What happened:** Instructed the user to run cargo install --path ... which failed because it used their default Rust 1.94.0 toolchain instead of the project's pinned 1.97.1 toolchain.
- **Cause:** Forgot that user's global cargo might resolve to an older version, and the project explicitly requires ustc 1.97.1 in Cargo.toml.
- **Prevention:** Whenever instructing the user or running a cargo command in a project with a pinned rust version, explicitly specify the toolchain version (e.g., cargo +1.97.1 install --path ...).

# 2026-08-03 — Invalid JavaScript syntax in a documentation verification snippet

- **What happened:** A Node REPL verification command intended to count required TDD-plan headings failed with `SyntaxError: Invalid or unexpected token` before reading or modifying the plan.
- **Exact procedure:** Used a compact JavaScript expression with escaped regular-expression literals in the Node REPL tool call while validating `spec/agent-safety-expansion-implementation-plan.md`.
- **Prevention:** For Node REPL verification, prefer simple string splitting and `includes()` counts over escaped regular-expression literals embedded in tool-call source. Run the smallest syntax-safe check independently before combining multiple document assertions.

# 2026-08-03 — Embedded Markdown delimiter broke a Node REPL verification string

- **What happened:** The follow-up plan check failed with `SyntaxError: Unexpected identifier 'tests'` because a Markdown backtick was embedded inside the JavaScript template literal used by the Node REPL tool call.
- **Exact procedure:** Counted Markdown-formatted test-module paths with `includes('`tests/')` inside a template-literal tool-call source.
- **Prevention:** In Node REPL verification snippets, match plain path substrings such as `tests/` and never embed Markdown delimiters inside a template literal. Keep document checks to plain-string inputs.

# 2026-08-04 — Graphify query assumed a missing saved interpreter path

- **What happened:** A review-time Graphify query failed with `ENOENT` because `graphify-out/.graphify_python` was not present even though `graphify-out/graph.json` existed.
- **Exact procedure:** Tried to read the optional saved Graphify interpreter path before checking whether the Graphify executable was available through the environment.
- **Prevention:** When a Graphify graph exists, check for `.graphify_python` first; if it is absent, independently discover the installed `graphify` command or initialize Graphify before querying. Do not treat a missing interpreter-path helper file as absence of the graph.

# 2026-08-04 — Combined Node REPL quality-gate snippet had unbalanced syntax

- **What happened:** A review command intended to run formatting and strict test linting failed with `Unexpected token: ')'` before either quality gate executed.
- **Exact procedure:** Combined two promisified command expressions in one Node REPL snippet and omitted the closing structure for the first error-handling expression.
- **Prevention:** Run mandatory quality gates in separate Node REPL calls. This preserves independent results and prevents JavaScript composition errors from masking gate execution.

# 2026-08-04 — Callback-based Node REPL error handler omitted its function closure

- **What happened:** The isolated formatter wrapper repeated the `Unexpected token: ')'` error before invoking Cargo.
- **Exact procedure:** Used `.catch(function(e){ return {...});` and closed the call before closing the callback function body.
- **Prevention:** For Node REPL command execution, use a plain `try/catch` statement rather than chained callback error handlers. Keep one command and one result per snippet.
