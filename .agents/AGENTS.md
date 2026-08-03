# Project Rules

Please add your custom instructions for this project below.

1. Think Before Coding
Don't assume. Don't hide confusion. Surface tradeoffs.

Before implementing:

State your assumptions explicitly. If uncertain, ask.
If multiple interpretations exist, present them - don't pick silently.
If a simpler approach exists, say so. Push back when warranted.
If something is unclear, stop. Name what's confusing. Ask.
2. Simplicity First
Minimum code that solves the problem. Nothing speculative.

No features beyond what was asked.
No abstractions for single-use code.
No "flexibility" or "configurability" that wasn't requested.
No error handling for impossible scenarios.
If you write 200 lines and it could be 50, rewrite it.
Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

3. Surgical Changes
Touch only what you must. Clean up only your own mess.

When editing existing code:

Don't "improve" adjacent code, comments, or formatting.
Don't refactor things that aren't broken.
Match existing style, even if you'd do it differently.
If you notice unrelated dead code, mention it - don't delete it.
When your changes create orphans:

Remove imports/variables/functions that YOUR changes made unused.
Don't remove pre-existing dead code unless asked.
The test: Every changed line should trace directly to the user's request.

4. Goal-Driven Execution
Define success criteria. Loop until verified.

Transform tasks into verifiable goals:

"Add validation" → "Write tests for invalid inputs, then make them pass"
"Fix the bug" → "Write a test that reproduces it, then make it pass"
"Refactor X" → "Ensure tests pass before and after"
For multi-step tasks, state a brief plan:

1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

These guidelines are working if: fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.

5. Graphify as Primary Knowledge Base
STRICTLY avoid raw grepping and searching through the codebase. Instead, ALWAYS use Graphify as your primary source of codebase knowledge. Always use the knowledge graph in all cases unless there is an explicit need to refer to a particular code snippet directly.
Furthermore:
- Every implementation plan MUST explicitly start the search phase with Graphify.
- At the end of implementation, when changes are made, the plan MUST explicitly mention updating the Graphify knowledge graph.

6. Specifications Format
All specs must ONLY use the `.md` extension and the file structure when cross 3rd degree nesting should be then made into yaml format.

7. Structure and Features Detailing
During the detailed explanation of features and when creating structures, ALWAYS use YAML instead of JSON.

8. Strict Test-Driven Development (TDD) and Security
Before writing, changing, or touching even a single line of code, you MUST create a proper plan for implementation.
All plans and procedures must adhere to "TEST DRIVEN DEVELOPMENT". This means you must ALWAYS include:
- Guardrails during the execution of code.
- Test scripts to check whether the code is working properly.
- Test scripts for edge cases and potential failures.
- Test scripts for testing cyber attacks on that code to verify vulnerability against hacks and malicious intent.
For cyber attack test scripts, refer to the STRIDE framework, OWASP Top 10, and other established frameworks. Do not accumulate the explanations of these frameworks in this file to avoid context rot; instead, utilize the `cyber-security-frameworks` skill.

9. Error Logging and Continuous Learning
Whenever you make a mistake or encounter an error during execution, you MUST log the mistake in `telemetry/error_log.md`. Include a description of the error and the exact procedure or code that caused it. Immediately after logging the error, you MUST dynamically update this `AGENTS.md` file by explicitly writing a new rule or instruction detailing the mistake and exactly what to avoid doing in the future to prevent recurrence.

10. Pre-Commit Hooks and Automation
Whenever possible and structurally applicable, you MUST include a plan and scripts for pre-commit hooks (e.g., using Husky or native Git hooks). These hooks should automate our guardrails, testing, and formatting to ensure no code is permanently committed without passing the established validation and security checks.

11. No Force Commits
Under absolutely no circumstances should you ever use force commits (e.g., `git commit --no-verify`, `git push --force`) to bypass the pre-commit hooks or automated tests. If a commit is failing, the underlying code or test MUST be fixed before proceeding. If a pre-commit hook fails, you MUST stop, create a clear solving plan to address the failure, and then try again. Bypassing guardrails is strictly forbidden.

12. No Vague Plans (Strict Adherence to Structure)
Whenever making an implementation plan or a detailed architectural spec, you MUST NOT make a vague or generic plan. You must strictly follow the required structure, particularly Rules #5 and #8. Every single plan document must independently and explicitly include its own Graphify Search/Update phases, Guardrails, TDD scripts, and Cyber Attack testing sections. Creating a separate, generic "testing" file instead of embedding these details into the specific component plans is a violation of this rule.

13. Strict Pre-Commit Hook Standards
Whenever setting up or modifying pre-commit hooks, you MUST configure them with maximum strictness. NEVER write generic or weak hooks. You must ensure that the hooks proactively block commits by strictly checking types (e.g., `tsc --noEmit`), enforcing zero-tolerance linting (e.g., `--max-warnings=0`), and comprehensively running all associated test suites (including unit, integration, and security tests). Do not assume basic validation is enough; enforce the highest code quality standards directly in the automation pipeline.

14. Governance and PowerShell Inspection Safety
When locating repository governance files, always include hidden directories (for example, `rg --files --hidden -g AGENTS.md`) before concluding that no project-level instructions exist. In PowerShell, use `${variable}` when a colon directly follows a variable name, and use a here-string for complex multi-line Python inspection scripts rather than a fragile quoted `-c` command.

15. Multi-file Patch Validation
Before submitting an `apply_patch` that adds multiple Markdown files, keep the patch in small reviewable groups and verify that every content line in each `*** Add File` hunk begins with `+`. A rejected patch is not a harmless shortcut: log it, fix the patch structure, and retry without changing unrelated files.

16. Expected No-Match Verification
When using a search command as a cleanliness assertion (for example, checking for trailing whitespace), handle its no-match exit code explicitly. Do not combine an expected `rg` exit code of 1 with mandatory-success commands in one verification chain; report no match as a successful clean check.

17. Generated Output Exclusion
Before running text-quality searches over documentation, exclude generated directories such as `graphify-out`. Run generated-artifact validation separately so large machine-generated files cannot drown out or time out the source-document check.

18. Empty Directory Cleanup
Use `apply_patch` for file deletion. If the environment blocks removal of a verified-empty directory, leave the empty directory rather than retrying with another destructive shell command; record the outcome and continue with non-destructive verification.

19. Detailed Task-Plan Patch Granularity
When replacing long Markdown task plans, use one file per patch whenever practical. Before applying, verify that every line within fenced examples and command blocks carries the required `+` prefix; code fences do not exempt content from patch syntax.

20. Task-Plan Command Formatting
In `apply_patch` add-file hunks for detailed task plans, prefer YAML command lists over `text` fenced blocks. This keeps every command visibly prefixed as added content and prevents malformed-patch retries.

21. Intentional Graph Reduction
If an intentional documentation rewrite or deletion reduces Graphify nodes, confirm the target directory and rerun `graphify update` with `--force` in a separate command. Record the reason for the reduction rather than retaining a stale graph.

22. Isolated Prerequisite Discovery
When checking build tools or runtimes, do not combine their commands with unrelated Graphify or worktree inspection. Run each prerequisite check independently so a missing executable or version mismatch is directly identifiable and can be handled before implementation.

23. Windows Rust Linker Gate
Before treating a Windows Rust test failure as a valid TDD red state, verify that the selected target has a working linker. For `*-pc-windows-msvc`, check `link.exe`; for GNU, check the configured GNU linker. Never install system-wide build tooling without explicit user approval.

24. Optional Tool Discovery Exit Codes
When probing optional executables in PowerShell, explicitly normalize an absent command to a successful diagnostic result. Do not let a non-essential `Get-Command` lookup hide successful prerequisite discovery in the same command.

25. Explicit Rust Linker Selection
For an MSVC Rust target without `link.exe`, PATH changes alone do not select `rust-lld.exe`. Validate an explicit, per-command linker override first and never commit a machine-specific linker path into project configuration.

26. MSVC SDK Library Prerequisite
On Windows, `rust-lld.exe` alone cannot build an MSVC Rust target because it still needs Windows SDK import libraries. After this failure is confirmed, stop local compilation attempts and request approval for Microsoft C++ Build Tools plus the Windows SDK instead of trying more linker flags.

27. Long Installer Verification
For an approved installer that exceeds the command time limit, do not immediately start a second installation. First inspect the installer process and the expected executable/library paths; only retry after proving that the original process has ended unsuccessfully.

28. Pinned Rust Quality Components
The Rust `minimal` toolchain profile does not include all project quality tools. Before invoking `cargo fmt` or `cargo clippy` for a pinned toolchain, install and verify its `rustfmt` and `clippy` components; do not treat their absence as a source-code failure.

29. Rust Formatting Before Verification
After every Rust source or Rust test patch, run the pinned `cargo fmt` command before the strict `cargo fmt --check`, test, and clippy gates. The check gate must verify formatting rather than being the first action to discover it.

30. Discover Task-Document Paths
Before reading or modifying a numbered implementation task, use `rg --files` in the task directory to identify its current filename. Task titles and old draft names are not reliable paths.

31. New-Scope Graphify Initialization
For a newly created project directory, run `graphify update <scope> --no-cluster` and verify its `graphify-out/graph.json` exists before issuing a `graphify query` against that graph. Do not query an assumed graph path.

32. Long Cargo Tool Installations
When a pinned `cargo install` exceeds the command time limit, do not run it again. First inspect whether its cargo process is still active and whether the expected binary has appeared; continue only after that check establishes the installation state.

33. Background Cargo Tool Builds
If a checked Cargo-tool installation has exceeded the foreground command limit twice, launch one hidden background build with separate stdout and stderr logs. Monitor its process and expected binary; never create concurrent Cargo installs for the same tool.

34. Dependency Policy Prerequisite
Before treating a required `cargo deny` result as an implementation failure, verify that the planned `deny.toml` policy exists. Create only the minimal policy needed for the locked dependency graph and keep the workspace package private unless the user has explicitly chosen a public license.

35. Rejected-Patch Recovery
For every `*** Add File` hunk, validate every content line begins with `+`. If an `apply_patch` is rejected, inspect the affected files before retrying; do not assume partial application or reuse stale patch context.

36. Exact Dependency License Allow-Lists
Derive `cargo-deny` license allowances from the current locked graph. Treat an unmatched-license warning as a policy defect and remove it before accepting the audit gate.

37. Cargo Plugin Invocation
After installing a Cargo plugin, verify the command form with its help output. Default to `cargo <plugin> ...` rather than assuming `cargo-<plugin>` accepts the same flags.

38. Windows Cargo File Locks
When Cargo reports a locked `target` artifact on Windows, identify and safely stop or wait for the lock-owning build/indexer process before retrying. Never delete `target` or skip the test to mask the lock.

39. Optional Process Diagnostics
When querying multiple optional Windows process names, normalize each absent name to a successful diagnostic result. Do not let a non-essential missing process hide the state needed for root-cause analysis.

40. Persistent Goal Response Discipline
When an active goal automatically resumes, never respond with a generic status-only final message. Take a concrete planned action, report a real blocker, or wait for explicit user input; do not create repeated continuation messages.

41. Rust Incremental Cache Warnings
If Rust reports an incremental-cache access warning after a successful build, record it and investigate lock activity before relying on incremental performance. Do not treat it as a functional test failure or delete target artifacts blindly.

42. Dependency-Graph Policy Updates
After adding a dependency, rerun `cargo deny` and update policy only from the locked graph's concrete findings. Use exact license allowances and justified version-specific skips; never weaken the entire duplicate or license policy to make a new feature pass.

43. Manifest Patch Preconditions
Before patching a dependency declaration, inspect the current manifest. Add a new dependency relative to an existing verified section instead of assuming an undeclared line can be updated.

44. Reader Depth Enforcement
Do not depend on a filesystem walker's maximum-depth option to produce a typed depth-limit error: it may silently omit entries beyond the boundary. For repository readers, inspect each normalized relative path and return the typed limit error from explicit application logic.

45. Rust Unit-Struct Construction
When strict Clippy is enabled, construct unit structs directly (for example, `SafeReader`) rather than via `::default()`. Keep the lint enabled; correct the call site instead of suppressing the warning.

46. Independent Rust Quality Gates
Verify each external Rust quality tool's supported flags before use. Run mandatory quality gates as independent commands (or preserve every exit code explicitly); never let a later successful command mask an earlier failed gate.

47. Context-Sensitive Patch Refresh
Before applying a patch against an existing source file, re-read that file after any formatter or intervening change. Do not reuse stale surrounding context; inspect first, then patch the exact current text.

48. Static Detection Iterator Discipline
For ordered static-profile predicates in Rust, query the immutable collection independently with `iter().any(...)` rather than reusing a mutable iterator across branches.

49. Cross-File Patch Context
For a multi-file behavioral patch, refresh every target file—not merely the first—after formatting or another intervening edit. Use the exact current formatted construction in each hunk.

50. Formatter-First Verification Batch
After every Rust source or test patch, run the pinned formatter before beginning any test, lint, or `fmt --check` verification batch. A prior successful formatter run does not cover later patches.

51. Minimal Rust Test Imports
When adding Rust tests, import only symbols used by assertions or setup. Treat compiler warnings as defects before the strict Clippy gate, rather than relying on later cleanup.

52. No Chained Mandatory Gates
Never chain mandatory quality gates with PowerShell separators. Execute formatter, tests, lint, dependency policy, and audit independently so a later success cannot conceal an earlier failure.

53. Rust Multi-Use Collection Types
When a collected Rust iterator value is borrowed and later consumed, explicitly declare its concrete collection type if inference is ambiguous; do not rely on downstream iterator branches to infer it.

54. Rust Option Reference Simplicity
When an API already returns `Option<&str>`, use it directly in combinators. Do not add `as_deref()` unless the option contains an owned dereferenceable type.

55. Locked Test Dependency Discipline
Before using a new Rust test helper crate, inspect the locked manifest. Prefer existing standard-library and declared dependency APIs over adding or assuming a helper dependency.

56. Opaque MSVC Linker Failures
When link.exe returns an unspecified error, do one verbose diagnostic build under the installed Visual Studio developer environment before suggesting toolchain repair or changing source code.

57. Rust CLI Argument Collection Shapes
Do not place fixed-size argument arrays of different lengths into one Rust array. Use explicit command invocations or a homogeneous owned collection for differing CLI argument counts.

58. Serialized Domain Read Compatibility
Before adding a reader for a serialized local artifact, verify every nested domain type derives the required deserialization trait as well as serialization.

59. Strict Clippy String Sanitizers
When sanitizing several single characters in Rust, use the combined `str::replace(['a', 'b'], replacement)` form rather than consecutive single-character `replace` calls. Run strict Clippy before accepting the implementation.

60. Recurrent Windows Cargo Locks
When a repeat Cargo gate fails with `os error 32`, pause the implementation gate and inspect active Cargo, Rust compiler, editor, and indexing processes before one clean retry. Preserve `target`; never delete artifacts merely to force a green result.

61. PowerShell Variable Delimiters
When interpolating a PowerShell variable directly before a colon or another name-valid character, use `${variable}`. Validate diagnostic scripts syntactically before interpreting their output.

62. Verification Uncertainty Precedence
In static verification, return parse uncertainty before profile unsupportedness. A malformed source must not be downgraded to an unsupported repository merely because normalization could not recover imports.

63. Fixture Expectations Must Be Source-Grounded
Before adding a fixture-matrix expectation, read the exact fixture source files. Do not infer its profile or status from the directory name; malformed fixtures may still contain a supported SDK import before the syntax error.

64. Platform-Specific Runner Verification
Run a platform-specific evaluation runner only on a matching available runtime. If the local machine lacks that runtime, record the limitation and perform only syntax-level validation; let the matching CI matrix runner provide execution evidence.

65. Generated-Output Ignore Rules
Before the first build, test, graph update, or tool installation in a new project directory, add and verify `.gitignore` rules for build output, generated indexes, and transient logs. Confirm with `git status --untracked-files=all` before handoff.

66. Separate Rust Verification Commands
Run Rust formatting, tests, linting, dependency policy, and audit commands as independent shell invocations. Do not chain them with PowerShell separators because a later outcome can obscure an earlier gate.

67. Bounded External Repository Audits
For cloned external repositories, enumerate candidate paths before content searches. Inspect a small, named set of adapter-relevant examples, tests, and documentation; never run an unrestricted recursive pattern search over a whole SDK clone or assume an optional `tests/` directory exists.

68. Adapter Module Path Discovery
Before reading a specific adapter implementation, enumerate `src/adapters` with `rg --files`. Public module names are not a reliable filesystem-path convention.

69. Cross-Layer Contract Fixture Updates
When changing a static adapter contract, inspect all unit tests, fixture matrices, and command-level inline sources for the old shape. Update each source-grounded fixture before the full suite.

70. Blocked Repository API Fallback
If the browser safety layer blocks a repository API endpoint, do not retry variants of that endpoint. Fall back to linked repository views or a selected sparse clone and retain the fixed-file audit budget.

71. Independent Targeted TDD Gates
Run formatter and targeted red/green tests independently. Do not chain them merely because they are smaller than the full quality gates.

72. Status-Contract Demo Migration
When an adapter status changes, review repair demos and task-generation tests for dependencies on the prior status. Migrate the demonstration to another deterministic contract failure; never reintroduce a false-positive rule to preserve a demo.

73. Complete Status-Contract Fixture Migration
For a changed adapter status, migrate all fixture categories—demo, repair packet, approval, verification, and matrix—not only the first failing category. Use the full-suite failure list as an audit checklist.

74. Independent Failure Diagnostics
When reproducing a test failure, run the test and the subsequent file inspection in separate commands. A trailing inspection command must never determine the shell exit code for a test gate.

75. CI Scenario Status Migration
When a status contract changes, migrate CI verifier scenarios along with demos, packet tests, matrices, and command tests. CI failure examples must use a deterministic framework contract violation.

76. Governance Patch Refresh
Before appending an error record or rule to a frequently edited governance file, re-read its current tail and use exact current patch context.

77. Status-Branch Simplification
After changing a status outcome, inspect adjacent conditional branches for identical results and merge them before strict Clippy. Never suppress the lint to retain redundant control flow.

78. Graphify Index Preconditions
Before issuing a Graphify query for a project scope, verify that the scope has a current `graphify-out/graph.json`. If it is missing, run the scoped Graphify update first or use a bounded direct file listing for documentation discovery; do not treat a failed query as repository evidence.

79. Bounded Generated-Evidence Inspection
Never open a complete generated scan artifact such as `evidence.yaml` with an unrestricted raw read. First inspect file size and use targeted counts, structured fields, or a bounded line selection; generated evidence can be much larger than the source slice that produced it.

80. Nested Project Path Resolution
When working inside a nested project directory, resolve documentation and governance paths relative to the active working directory before reading them. Do not reuse workspace-root-relative paths unchanged; confirm the path with a bounded file listing first.

81. SDK Submodule Profile Detection
When adding support for a framework construct imported from an SDK submodule, add a profile-detection regression test and verify a fixture scan. A profile matcher that recognizes only the package root can silently bypass the new adapter rule.

82. Repair-Packet Deterministic Failure Seeds
When an adapter reclassifies an old failure as static uncertainty, update every repair-packet fixture to a documented deterministic failure before the full suite. Packet generation is defined only for failed rules and must not rely on ambiguous source.

83. CI Deterministic Failure Migration
When an adapter reclassifies a control shape, update CI verifier failure seeds in the same change as demos and repair packets. Verify expected CI exit codes continue to represent a documented deterministic failure, never uncertainty.

84. One Nested-Path Convention per Command
For a nested project, use either workspace-root-relative paths with the workspace root as working directory or project-relative paths with the nested project as working directory. Do not combine them; a failed inspection is not evidence.

85. Narrow Documentation Patches
For long Markdown governance or evidence files, patch one nearby heading or table block at a time after refreshing the exact lines. Do not bundle unrelated documentation hunks behind brittle long-line context.

86. Unmaskable Verification Gates
Every test, formatter, lint, dependency, or documentation gate must be the final and only substantive command in its shell invocation. Run Graphify, inspection, or cleanup only after the gate's exit status has been observed.

87. Native Release-Build Triage
For a release build failure inside a dependency or native build script, reproduce once with bounded verbose diagnostics before changing product code, dependencies, or toolchain configuration. Treat a successful verbatim reproduction as environment evidence, not a reason to invent a code fix.

88. Optional Pre-Commit Probe Normalization
When checking whether `pre-commit` or another optional executable exists, normalize an absent `Get-Command` result to a successful diagnostic status and run it separately from configuration inspection. Never let an optional missing-command probe hide successful repository inspection.

89. Pinned Toolchain in Hooks
When a Rust project declares a pinned toolchain, every pre-commit Rust hook must invoke that toolchain explicitly (for example, `cargo +1.97.1 ...`) or use a verified selector. Never validate hooks with plain `cargo` when the machine default may differ from the project toolchain.

90. Pre-Commit Formatting Stabilization
When pre-commit modifies staged documentation or fixtures, review and stage those deterministic formatting changes, then rerun the complete hook suite. Do not call a modifying hook run a pass until a subsequent run completes without modifications.

91. Confirm New Repository Before First Push
Before the first push of a project, confirm the exact destination repository URL and visibility with the user, and inspect whether it is empty or already contains work. Never reuse an inherited `origin` remote as the destination by assumption.

92. Add-File Patch Prefix Validation
For a Markdown add-file patch, every content line—including fenced code, YAML, and command examples—must begin with `+`. If a patch is rejected, inspect the target state and retry with a smaller validated patch.

93. Isolate Excluded Files Before Orphan Publication
Before creating an orphan branch for a filtered public publication, identify modified files that must remain private and preserve them in a named local stash. Do not commit excluded governance, telemetry, or sensitive files just to make branch switching succeed.

94. Check Orphan Index Before Clearing
After switching to an orphan branch, inspect `git ls-files` before attempting to clear the index. An orphan branch starts with an empty index; do not run a blanket cached removal when there is nothing tracked.

95. Quote Multi-Component GitHub Action Inputs
For GitHub Actions inputs that contain comma-separated components, quote the complete value or use the documented multiline form. Never place a comma-separated component list inside an inline YAML map; YAML can parse later components as separate keys and silently omit required tooling.

96. Make Cross-Platform Fixture Scripts Exit-Deterministic
In GitHub Actions, invoke Unix `.sh` fixture scripts through `bash ./path/script.sh` instead of relying on a Git executable bit. When a PowerShell fixture intentionally accepts a non-zero native exit code, explicitly `exit 0` after validation; GitHub's dot-sourced PowerShell runner can otherwise surface the last expected non-zero code as the step failure. Add a guard that rejects either regression before commit.

97. Verify Archive Contents by Archive Format
Release ZIP archives and tar archives may use different member-path prefixes. Inspect the actual member list first, then assert the required basename set rather than assuming a shared `./` prefix.

98. Local TDD Verification Before Commit
Before running git commit, explicitly verify that the planned tool call to edit the file has actually been executed, and run the script locally (TDD) as a final guardrail before committing. Never assume a file was edited just because the plan was written.
