$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repositoryRoot
try {
  $binary = Join-Path $repositoryRoot 'target/release/agent-preflight.exe'
  if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    throw "Release binary is missing: $binary"
  }
  $cases = @(
    @{ Fixture = 'fixtures/openai/hosted_mcp_literal_approval'; Expected = 0 },
    @{ Fixture = 'fixtures/google_adk/direct'; Expected = 0 },
    @{ Fixture = 'fixtures/claude/plan_mode'; Expected = 0 },
    @{ Fixture = 'fixtures/parser'; Expected = 4 }
  )
  foreach ($case in $cases) {
    & $binary scan (Join-Path $repositoryRoot $case.Fixture)
    if ($LASTEXITCODE -ne $case.Expected) {
      throw "Release fixture failed: $($case.Fixture) expected $($case.Expected), got $LASTEXITCODE"
    }
  }
} finally {
  Pop-Location
}

exit 0
