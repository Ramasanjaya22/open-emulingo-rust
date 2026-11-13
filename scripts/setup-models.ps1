#requires -Version 7.0
<#
.SYNOPSIS
  Downloads the llama.cpp Vulkan server build and the Qwen3.5-4B vision model into models/.

.DESCRIPTION
  Offline-friendly and idempotent: a file already present with the pinned SHA-256 is skipped.
  A file whose hash does not match is deleted and fetched again; a fresh download that still
  fails its hash is removed and the script exits non-zero. Nothing is ever accepted unverified.

  Pinned 2026-11-13 (llama.cpp b12344, unsloth/Qwen3.5-4B-GGUF @ e87f1764).
  Bump $LlamaBuild and the matching hash together; see models/README.md.
#>
[CmdletBinding()]
param(
    [switch]$Force
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

$RepoRoot = Split-Path -Parent $PSScriptRoot
$ModelsRoot = Join-Path $RepoRoot 'models'
$LlamaDir = Join-Path $ModelsRoot 'llama.cpp'
$GgufDir = Join-Path $ModelsRoot 'Qwen3.5-4B-GGUF'

$LlamaBuild = 'b12344'
$LlamaZipName = "llama-$LlamaBuild-bin-win-vulkan-x64.zip"
$HfBase = 'https://huggingface.co/unsloth/Qwen3.5-4B-GGUF/resolve/main'

$Artifacts = @(
    @{
        Path   = Join-Path $LlamaDir $LlamaZipName
        Url    = "https://github.com/ggml-org/llama.cpp/releases/download/$LlamaBuild/$LlamaZipName"
        Sha256 = 'c64f8e79bdc877065bfefb45fe292c0fee5cbf1a0342cb9c728031dc51d548ed'
        Zip    = $true
    },
    @{
        Path   = Join-Path $GgufDir 'Qwen3.5-4B-Q4_K_M.gguf'
        Url    = "$HfBase/Qwen3.5-4B-Q4_K_M.gguf"
        Sha256 = '00fe7986ff5f6b463e62455821146049db6f9313603938a70800d1fb69ef11a4'
    },
    # unsloth publishes no Q8_0 mmproj for this repo, so F16 is the documented choice (models/README.md).
    @{
        Path   = Join-Path $GgufDir 'mmproj-F16.gguf'
        Url    = "$HfBase/mmproj-F16.gguf"
        Sha256 = 'cd88edcf8d031894960bb0c9c5b9b7e1fea6ebee02b9f7ce925a00d12891f864'
    }
)

function Get-FileSha256([string]$Path) {
    (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash.ToLowerInvariant()
}

New-Item -ItemType Directory -Force -Path $LlamaDir, $GgufDir | Out-Null

foreach ($artifact in $Artifacts) {
    $name = Split-Path -Leaf $artifact.Path
    $dest = $artifact.Path
    $expected = $artifact.Sha256

    if ((Test-Path -LiteralPath $dest) -and -not $Force) {
        if ((Get-FileSha256 $dest) -eq $expected) {
            Write-Host "skip  $name (sha256 ok)"
            continue
        }
        Write-Warning "hash mismatch: $name is not the pinned file, re-downloading"
        Remove-Item -LiteralPath $dest -Force
    }
    if ($Force -and (Test-Path -LiteralPath $dest)) {
        Remove-Item -LiteralPath $dest -Force
    }

    Write-Host "get   $name"
    $tmp = "$dest.part"
    Remove-Item -LiteralPath $tmp -Force -ErrorAction SilentlyContinue
    # curl.exe, not Invoke-WebRequest: IWR stalls on Hugging Face's chunked LFS redirects.
    & curl.exe -L --fail --silent --show-error --output $tmp $artifact.Url
    if ($LASTEXITCODE -ne 0) {
        Remove-Item -LiteralPath $tmp -Force -ErrorAction SilentlyContinue
        throw "download failed for $name (curl exit $LASTEXITCODE)"
    }

    $actual = Get-FileSha256 $tmp
    if ($actual -ne $expected) {
        Remove-Item -LiteralPath $tmp -Force -ErrorAction SilentlyContinue
        throw "SHA-256 mismatch for $name`: expected $expected, got $actual"
    }

    Move-Item -LiteralPath $tmp -Destination $dest -Force
    Write-Host "ok    $name"

    if ($artifact.Zip) {
        Expand-Archive -LiteralPath $dest -DestinationPath $LlamaDir -Force
        Write-Host "ok    extracted $name"
    }
}

$server = Get-ChildItem -Path $LlamaDir -Filter 'llama-server.exe' -Recurse -File | Select-Object -First 1
if (-not $server) {
    throw "llama-server.exe not found under $LlamaDir"
}

Write-Host ""
Write-Host "ready: $($server.FullName)"
Write-Host "next : pwsh scripts/run-llm.ps1"
