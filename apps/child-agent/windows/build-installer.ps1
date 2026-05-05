$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$appRoot = Split-Path -Parent $scriptDir
$releaseDir = Join-Path $appRoot "src-tauri\target\release"
$serviceExe = Join-Path $releaseDir "child-agent-service.exe"
$trayExe = Join-Path $releaseDir "child-agent-tray.exe"
$wixFile = Join-Path $scriptDir "child-agent-service.wxs"

if (-not (Test-Path $serviceExe)) {
  throw "Expected service binary at $serviceExe"
}

if (-not (Test-Path $trayExe)) {
  throw "Expected tray binary at $trayExe"
}

if (-not (Test-Path $wixFile)) {
  throw "Expected WiX definition at $wixFile"
}

Write-Host "Release binaries and WiX definition are ready."
Write-Host "Run WiX candle/light or wix build against $wixFile to produce the MSI."
