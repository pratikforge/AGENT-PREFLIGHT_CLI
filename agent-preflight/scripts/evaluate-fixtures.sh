#!/usr/bin/env bash
set -euo pipefail
repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"
binary="$repository_root/target/release/agent-preflight"
if [[ ! -x "$binary" ]]; then
  echo "Release binary is missing or not executable: $binary" >&2
  exit 2
fi

run_case() {
  local fixture="$1"
  local expected="$2"
  set +e
  "$binary" scan "$repository_root/$fixture"
  local actual=$?
  set -e
  if [[ "$actual" -ne "$expected" ]]; then
    echo "Release fixture failed: $fixture expected $expected, got $actual" >&2
    exit 1
  fi
}

run_case "fixtures/openai/hosted_mcp_literal_approval" 0
run_case "fixtures/google_adk/direct" 0
run_case "fixtures/claude/plan_mode" 0
run_case "fixtures/parser" 4
