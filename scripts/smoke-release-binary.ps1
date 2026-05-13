Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$BinPath = $args[0]
if ([string]::IsNullOrWhiteSpace($BinPath)) {
  $BinPath = ".\\target\\release\\creditlint.exe"
}

$ResolvedBinPath = (Resolve-Path $BinPath).Path
if (-not (Test-Path $ResolvedBinPath)) {
  throw "release binary does not exist: $BinPath"
}

$Repo = Join-Path $env:RUNNER_TEMP "creditlint-smoke-repo"
Remove-Item -Recurse -Force $Repo -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Repo | Out-Null
Set-Location $Repo

git init | Out-Null
git config user.name "Creditlint Smoke Test"
git config user.email "creditlint-smoke@example.com"

& $ResolvedBinPath init | Out-Null
"Reviewed-by: Jane Doe <jane@example.com>" | & $ResolvedBinPath check --stdin | Out-Null
"Co-authored-by: Codex <codex@example.com>" | & $ResolvedBinPath check --stdin *> $null
if ($LASTEXITCODE -ne 1) {
  throw "expected violating input to exit 1"
}

& $ResolvedBinPath github ruleset-pattern | Out-Null
