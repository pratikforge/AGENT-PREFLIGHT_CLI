$ErrorActionPreference = 'Stop'

$workflowPath = Join-Path $PSScriptRoot '..\.github\workflows\agent-preflight-ci.yml'
$workflow = Get-Content -Raw -LiteralPath $workflowPath

if ($workflow -notmatch '(?m)^\s*components:\s*["'']rustfmt,\s*clippy["'']\s*$') {
    throw "The CI workflow must pass rustfmt and clippy as one quoted components input. An inline YAML map can silently omit clippy."
}

if ($workflow -notmatch '(?m)^\s*run:\s*bash\s+\./scripts/evaluate-fixtures\.sh\s*$') {
    throw "Unix fixture evaluation must invoke the script through bash so CI does not depend on a Git executable-bit checkout."
}

$fixtureScriptPath = Join-Path $PSScriptRoot 'evaluate-fixtures.ps1'
$fixtureScript = Get-Content -Raw -LiteralPath $fixtureScriptPath
if ($fixtureScript -notmatch '(?s)finally\s*\{\s*Pop-Location\s*\}\s*exit\s+0\s*$') {
    throw "The Windows fixture evaluator must explicitly exit 0 after accepting an expected non-zero fixture result."
}

Write-Output 'CI toolchain component declaration is valid.'
