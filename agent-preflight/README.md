# Agent Preflight

Agent Preflight is a local static checker for Python and TypeScript agent repositories. It records evidence-backed findings, requires explicit owner approval, produces a bounded repair handoff, and verifies source structure.

It supports direct patterns from OpenAI Agents SDK, Google ADK, and Claude Agent SDK. It does not execute the scanned repository, an agent, or a tool.

## Commands

```text
cargo run --manifest-path Cargo.toml -- scan <repository-path>
cargo run --manifest-path Cargo.toml -- review <repository-path>
cargo run --manifest-path Cargo.toml -- approve <repository-path> <rule-id>
cargo run --manifest-path Cargo.toml -- task <repository-path> <rule-id>
cargo run --manifest-path Cargo.toml -- verify <repository-path> --ci
```

See [the demo](docs/demo.md) and [limitations](docs/limitations.md).

## Installation

You can download pre-built release binaries for your platform, or build from source.

### Windows (PowerShell)

```powershell
# Verify checksum
$expected = (Get-Content SHA256SUMS.txt).Split(' ')[0]
$actual = (Get-FileHash agent-preflight-x86_64-pc-windows-msvc.zip -Algorithm SHA256).Hash.ToLower()
if ($expected -ne $actual) { throw "Checksum mismatch" }

# Extract the executable
Expand-Archive -Path agent-preflight-x86_64-pc-windows-msvc.zip -DestinationPath .
```

### macOS and Linux (Bash)

```bash
# Replace TARGET with x86_64-apple-darwin (macOS) or x86_64-unknown-linux-gnu (Linux)
export TARGET="x86_64-unknown-linux-gnu"

# Verify checksum
shasum -a 256 -c SHA256SUMS.txt

# Extract the executable
tar -xzvf agent-preflight-${TARGET}.tar.gz
```

### Build from source

```bash
cargo install --path . --locked
```
