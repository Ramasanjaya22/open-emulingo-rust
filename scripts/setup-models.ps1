#requires -Version 7.0
<#
.SYNOPSIS
  Verifies the local model assets this repo depends on (Bonsai 2 9B + Laya) and the
  llama.cpp runtime, against the SHA-256 pins in models/assets.json.

.DESCRIPTION
  The models are already installed on this machine, so this script downloads nothing and
  touches the network not at all. It is a verifier: every pinned file must exist with the
  expected size and hash, otherwise the script exits non-zero and names the bad asset.

  Idempotent by construction: it only reads. Running it twice produces identical output.

  -Manifest <path>  check a different manifest (used by the tests and by CI).
#>
[CmdletBinding()]
param(
    [string]$Manifest
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

$RepoRoot = Split-Path -Parent $PSScriptRoot
if (-not $Manifest) {
    $Manifest = Join-Path $RepoRoot 'models/assets.json'
}

if (-not (Test-Path -LiteralPath $Manifest)) {
    throw "manifest not found: $Manifest"
}

$manifestData = Get-Content -LiteralPath $Manifest -Raw | ConvertFrom-Json
$assets = @($manifestData.assets)
if ($assets.Count -eq 0) {
    throw "manifest has no assets: $Manifest"
}

$failures = @()

foreach ($asset in $assets) {
    $path = [Environment]::ExpandEnvironmentVariables($asset.path)
    $name = $asset.name

    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        Write-Host ("MISSING  {0}" -f $name)
        Write-Host ("         {0}" -f $path)
        $failures += $name
        continue
    }

    $file = Get-Item -LiteralPath $path
    if ($file.Length -ne [int64]$asset.bytes) {
        Write-Host ("SIZE     {0}: expected {1} bytes, found {2}" -f $name, $asset.bytes, $file.Length)
        $failures += $name
        continue
    }

    $actual = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -ne $asset.sha256) {
        Write-Host ("HASH     {0}: expected {1}, found {2}" -f $name, $asset.sha256, $actual)
        $failures += $name
        continue
    }

    Write-Host ("ok       {0}  ({1} MB, sha256 {2})" -f $name, [math]::Round($file.Length / 1MB, 1), $actual.Substring(0, 12))
}

Write-Host ""
if ($failures.Count -gt 0) {
    Write-Host ("FAILED: {0} asset(s) missing or corrupt: {1}" -f $failures.Count, ($failures -join ', '))
    Write-Host "Sources and install notes: models/README.md"
    exit 1
}

Write-Host ("verified {0} asset(s). No downloads performed." -f $assets.Count)
exit 0
