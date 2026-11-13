#requires -Version 7.0
<#
.SYNOPSIS
  Brings up the local inference stack this repo uses: Bonsai 2 9B (chat/translation) and
  Laya (typed decisions), both on 127.0.0.1.

.DESCRIPTION
  Nothing is downloaded and no cloud endpoint is contacted. Each server is reused if it is
  already healthy, and started otherwise:

    Bonsai  127.0.0.1:8091  OpenAI-compatible  /v1/chat/completions   model bonsai-2-9b
    Laya    127.0.0.1:8000  typed decisions    /v1/systemone, /v1/route

  Bonsai runs on the Prism build of llama-server, because the ternary PQ2_0 GGUF uses a
  tensor type that stock llama.cpp rejects. Laya runs on CPU.

  The script waits until both report healthy, then prints the endpoints. Servers are left
  running; stop them with scripts/stop-llm.ps1.
#>
[CmdletBinding()]
param(
    [int]$BonsaiPort = 8091,
    [int]$LayaPort = 8000,
    [int]$TimeoutSeconds = 240
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

$RepoRoot = Split-Path -Parent $PSScriptRoot
$ManifestPath = Join-Path $RepoRoot 'models/assets.json'
if (-not (Test-Path -LiteralPath $ManifestPath)) {
    throw "manifest not found: $ManifestPath"
}

$assets = @((Get-Content -LiteralPath $ManifestPath -Raw | ConvertFrom-Json).assets)

function Get-AssetPath([string]$Role) {
    $asset = $assets | Where-Object { $_.role -eq $Role } | Select-Object -First 1
    if (-not $asset) {
        throw "no asset with role '$Role' in models/assets.json"
    }
    [Environment]::ExpandEnvironmentVariables($asset.path)
}

function Test-Healthy([string]$Url) {
    try {
        $response = Invoke-WebRequest -Uri $Url -TimeoutSec 3 -UseBasicParsing
        return $response.StatusCode -eq 200
    } catch {
        return $false
    }
}

function Wait-Healthy([string]$Url, [string]$Label) {
    $deadline = (Get-Date).AddSeconds($TimeoutSeconds)
    while ((Get-Date) -lt $deadline) {
        if (Test-Healthy $Url) {
            return $true
        }
        Start-Sleep -Seconds 2
    }
    Write-Warning "$Label did not become healthy at $Url within $TimeoutSeconds s"
    return $false
}

$logDir = Join-Path $RepoRoot 'models/logs'
New-Item -ItemType Directory -Force -Path $logDir | Out-Null

# --- Bonsai -----------------------------------------------------------------
$bonsaiRuntime = Get-AssetPath 'runtime'
$bonsaiModel = Get-AssetPath 'llm'
$bonsaiHealth = "http://127.0.0.1:$BonsaiPort/health"

if (Test-Healthy $bonsaiHealth) {
    Write-Host "bonsai already healthy on 127.0.0.1:$BonsaiPort"
} else {
    if (-not (Test-Path -LiteralPath $bonsaiRuntime)) { throw "missing runtime: $bonsaiRuntime" }
    if (-not (Test-Path -LiteralPath $bonsaiModel)) { throw "missing model: $bonsaiModel" }
    Write-Host "starting bonsai (Prism llama-server, ROCm) on 127.0.0.1:$BonsaiPort"
    # The Prism llama-server.exe is a small launcher; it only finds its DLLs when the
    # working directory is the runtime folder. Started elsewhere it dies with 0xC0000409.
    Start-Process -FilePath $bonsaiRuntime -WorkingDirectory (Split-Path -Parent $bonsaiRuntime) -WindowStyle Hidden -ArgumentList @(
        '-m', $bonsaiModel,
        '--alias', 'bonsai-2-9b',
        '--host', '127.0.0.1',
        '--port', "$BonsaiPort",
        '--cors-origins', 'localhost',
        '--no-cors-credentials',
        '--device', 'ROCm0',
        '-ngl', '99',
        '-fa', 'on',
        '-c', '65536',
        '-np', '1',
        '-t', '4',
        '-b', '2048',
        '-ub', '256',
        '-ctk', 'f16',
        '-ctv', 'f16',
        '--jinja',
        '--reasoning', 'off'
    ) -RedirectStandardOutput (Join-Path $logDir 'bonsai.out.log') -RedirectStandardError (Join-Path $logDir 'bonsai.err.log')
    Wait-Healthy $bonsaiHealth 'bonsai' | Out-Null
}

# --- Laya -------------------------------------------------------------------
$layaServer = Get-AssetPath 'laya'
$layaHealth = "http://127.0.0.1:$LayaPort/health"

if (Test-Healthy $layaHealth) {
    Write-Host "laya already healthy on 127.0.0.1:$LayaPort"
} else {
    if (-not (Test-Path -LiteralPath $layaServer)) { throw "missing laya server: $layaServer" }
    Write-Host "starting laya on 127.0.0.1:$LayaPort (CPU, first load takes a couple of minutes)"
    $env:LAYA_HOST = '127.0.0.1'
    $env:LAYA_PORT = "$LayaPort"
    $env:LAYA_DEVICE = 'cpu'
    $env:LAYA_PRELOAD = '1'
    $env:LAYA_MODELS = 'english,multilingual,typed-decisions'
    $env:LAYA_THREADS = '8'
    Start-Process -FilePath $layaServer -WindowStyle Hidden `
        -RedirectStandardOutput (Join-Path $logDir 'laya.out.log') -RedirectStandardError (Join-Path $logDir 'laya.err.log')
    Wait-Healthy $layaHealth 'laya' | Out-Null
}

Write-Host ""
foreach ($probe in @(@{ Name = 'bonsai'; Url = $bonsaiHealth }, @{ Name = 'laya'; Url = $layaHealth })) {
    $status = if (Test-Healthy $probe.Url) { 'ok' } else { 'UNHEALTHY' }
    Write-Host ("{0,-8} {1,-6} {2}" -f $probe.Name, $status, $probe.Url)
}

Write-Host ""
Write-Host "bonsai  http://127.0.0.1:$BonsaiPort/v1   model bonsai-2-9b   (OpenAI chat completions)"
Write-Host "laya    http://127.0.0.1:$LayaPort/v1   /v1/systemone, /v1/route"

if (-not (Test-Healthy $bonsaiHealth) -or -not (Test-Healthy $layaHealth)) {
    exit 1
}
exit 0
