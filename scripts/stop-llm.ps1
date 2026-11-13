#requires -Version 7.0
<#
.SYNOPSIS
  Stops the local inference servers started by scripts/run-llm.ps1.

.DESCRIPTION
  Only processes listening on the two local ports are stopped. Nothing else is touched.
#>
[CmdletBinding()]
param(
    [int]$BonsaiPort = 8091,
    [int]$LayaPort = 8000
)

$ErrorActionPreference = 'Stop'

foreach ($port in @($BonsaiPort, $LayaPort)) {
    $listeners = @(Get-NetTCPConnection -LocalPort $port -State Listen -ErrorAction SilentlyContinue)
    if ($listeners.Count -eq 0) {
        Write-Host "port ${port}: already free"
        continue
    }
    $pids = $listeners.OwningProcess | Sort-Object -Unique
    foreach ($processId in $pids) {
        Stop-Process -Id $processId -Force -ErrorAction SilentlyContinue
        Write-Host "port ${port}: stopped pid ${processId}"
    }
}
