$ErrorActionPreference = 'Stop'

$workflowPath = Join-Path $PSScriptRoot '..\..\.github\workflows\agent-preflight-ci.yml'
$workflow = Get-Content -Raw -LiteralPath $workflowPath

if ($workflow -notmatch '(?m)^\s*components:\s*["'']rustfmt,\s*clippy["'']\s*$') {
    throw "The CI workflow must pass rustfmt and clippy as one quoted components input. An inline YAML map can silently omit clippy."
}

Write-Output 'CI toolchain component declaration is valid.'
