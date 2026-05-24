# CasGarage Uninstallation Script for Windows
# PowerShell script for removing CasGarage Windows Service

#Requires -RunAsAdministrator

[CmdletBinding()]
param(
    [string]$InstallDir = "C:\Program Files\CasGarage",
    [string]$DataDir = "C:\ProgramData\CasGarage",
    [string]$ServiceName = "CasGarage",
    [switch]$RemoveData
)

$ErrorActionPreference = "Stop"

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

Write-Info "Starting CasGarage uninstallation for Windows"

# Stop and remove service
$service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue

if ($service) {
    Write-Info "Stopping service..."

    if ($service.Status -eq 'Running') {
        Stop-Service -Name $ServiceName -Force
    }

    Write-Info "Removing service..."
    $svc = Get-WmiObject -Class Win32_Service -Filter "Name='$ServiceName'"
    if ($svc) {
        $svc.Delete() | Out-Null
    }

    Write-Info "Service removed"
} else {
    Write-Warn "Service not found"
}

# Remove firewall rules
Write-Info "Removing firewall rules..."
Remove-NetFirewallRule -DisplayName "CasGarage Admin UI" -ErrorAction SilentlyContinue
Remove-NetFirewallRule -DisplayName "CasGarage S3 API" -ErrorAction SilentlyContinue
Remove-NetFirewallRule -DisplayName "CasGarage Metrics" -ErrorAction SilentlyContinue

# Remove installation directory
if (Test-Path $InstallDir) {
    Write-Info "Removing installation directory..."
    Remove-Item -Path $InstallDir -Recurse -Force
}

# Remove data directory (with confirmation)
if (Test-Path $DataDir) {
    if ($RemoveData) {
        Write-Info "Removing data directory..."
        Remove-Item -Path $DataDir -Recurse -Force
    } else {
        $response = Read-Host "Remove data directory at $DataDir? [y/N]"
        if ($response -match '^[Yy]') {
            Write-Info "Removing data directory..."
            Remove-Item -Path $DataDir -Recurse -Force
        } else {
            Write-Warn "Data directory preserved at $DataDir"
        }
    }
}

# Remove temp directory
$TmpDir = Join-Path $env:TEMP "casgarage"
if (Test-Path $TmpDir) {
    Remove-Item -Path $TmpDir -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Host ""
Write-Info "Uninstallation complete!"
Write-Host ""
