# Fetches the open-source GBA BIOS replacement used by oemu.
#
# This downloads the VBA-M / Normmatt replacement BIOS (GPL-2.0) that the
# rustboyadvance-ng project itself recommends. It is NOT a Nintendo BIOS dump,
# and it is gitignored (`*.bin`, `bios/`).
#
# Usage: powershell -ExecutionPolicy Bypass -File scripts/setup-bios.ps1

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
$biosDir = Join-Path $repoRoot 'bios'
$biosPath = Join-Path $biosDir 'gba_bios.bin'
$expectedSha = '661a9afb93624f2c5e77d07dab137bd8d23cff79e8639c62fe621b49bb064749'
$url = 'https://raw.githubusercontent.com/Nebuleon/ReGBA/master/bios/gba_bios.bin'

if (Test-Path $biosPath) {
    Write-Host "BIOS already present at $biosPath"
    exit 0
}

New-Item -ItemType Directory -Force -Path $biosDir | Out-Null

Write-Host "Downloading replacement GBA BIOS..."
Invoke-WebRequest -Uri $url -OutFile $biosPath

$size = (Get-Item $biosPath).Length
if ($size -ne 16384) {
    Remove-Item $biosPath
    throw "Downloaded BIOS has unexpected size $size (expected 16384)."
}

$actualSha = (Get-FileHash -Algorithm SHA256 -Path $biosPath).Hash.ToLower()
if ($actualSha -ne $expectedSha) {
    Remove-Item $biosPath
    throw "BIOS checksum mismatch. Expected $expectedSha, got $actualSha."
}

Write-Host "BIOS installed at $biosPath"
