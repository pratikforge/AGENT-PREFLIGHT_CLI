$ErrorActionPreference = 'Stop'

$Repo = "pratikforge/agent-preflight"
$InstallDir = Join-Path $env:USERPROFILE ".agent-preflight\bin"
$ZipPath = Join-Path $env:TEMP "agent-preflight-latest.zip"

Write-Host "Fetching latest release of Agent Preflight..." -ForegroundColor Cyan

# Get latest release from GitHub API
$ReleaseApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
try {
    $ReleaseData = Invoke-RestMethod -Uri $ReleaseApiUrl -UseBasicParsing
} catch {
    Write-Error "Failed to fetch release data. Are there any releases published yet?"
    exit 1
}

$Asset = $ReleaseData.assets | Where-Object { $_.name -like "*windows-msvc.zip" }
if (-not $Asset) {
    Write-Error "Could not find a Windows artifact for the latest release."
    exit 1
}

Write-Host "Downloading $($Asset.name)..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $ZipPath -UseBasicParsing

Write-Host "Installing to $InstallDir..." -ForegroundColor Cyan
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force

# Add to PATH if missing
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notmatch [regex]::Escape($InstallDir)) {
    Write-Host "Adding $InstallDir to your PATH..." -ForegroundColor Yellow
    $NewPath = $UserPath
    if (-not $NewPath.EndsWith(";")) {
        $NewPath += ";"
    }
    $NewPath += $InstallDir
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    $env:PATH = "$env:PATH;$InstallDir"
}

# Clean up
Remove-Item $ZipPath -Force

Write-Host ""
Write-Host "Agent Preflight installed successfully!" -ForegroundColor Green
Write-Host "Run 'agent-preflight scan .' to get started." -ForegroundColor White
