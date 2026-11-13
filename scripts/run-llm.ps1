#requires -Version 7.0
<#
.SYNOPSIS
  Starts llama-server on 127.0.0.1:8080 with the pinned Qwen3.5-4B vision model.

.DESCRIPTION
  Foreground process; Ctrl+C stops it. Everything it serves is local: the app talks to
  127.0.0.1 only. Run scripts/setup-models.ps1 first.
#>
[CmdletBinding()]
param(
    [int]$Port = 8080
)

$ErrorActionPreference = 'Stop'

$RepoRoot = Split-Path -Parent $PSScriptRoot
$ModelsRoot = Join-Path $RepoRoot 'models'
$Server = Join-Path $ModelsRoot 'llama.cpp/llama-server.exe'
$Model = Join-Path $ModelsRoot 'Qwen3.5-4B-GGUF/Qwen3.5-4B-Q4_K_M.gguf'
$Mmproj = Join-Path $ModelsRoot 'Qwen3.5-4B-GGUF/mmproj-F16.gguf'

foreach ($file in @($Server, $Model, $Mmproj)) {
    if (-not (Test-Path -LiteralPath $file)) {
        throw "missing $file - run scripts/setup-models.ps1 first"
    }
}

& $Server `
    -m $Model `
    --mmproj $Mmproj `
    -ngl 99 `
    -c 8192 `
    --parallel 2 `
    --image-max-tokens 256 `
    --host 127.0.0.1 `
    --port $Port `
    --jinja

exit $LASTEXITCODE
