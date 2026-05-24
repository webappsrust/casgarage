# CasGarage Installation Script for Windows
# PowerShell script for installing CasGarage as a Windows Service
# Usage: Run as Administrator

#Requires -RunAsAdministrator

[CmdletBinding()]
param(
    [string]$InstallDir = "C:\Program Files\CasGarage",
    [string]$DataDir = "C:\ProgramData\CasGarage\data",
    [string]$ConfigDir = "C:\ProgramData\CasGarage\config",
    [string]$LogDir = "C:\ProgramData\CasGarage\logs",
    [string]$ServiceName = "CasGarage",
    [string]$ServiceDisplayName = "CasGarage - S3-Compatible Object Storage",
    [string]$Version = "latest"
)

$ErrorActionPreference = "Stop"

# Color output functions
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
    exit 1
}

# Check if running as Administrator
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (-not (Test-Administrator)) {
    Write-Error-Custom "This script must be run as Administrator. Please run PowerShell as Administrator and try again."
}

Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║   🚗 CasGarage Windows Installation   ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Info "Starting CasGarage installation for Windows"

# Detect architecture
$Arch = if ([Environment]::Is64BitOperatingSystem) { "amd64" } else { "386" }
$Platform = "windows_$Arch"
Write-Info "Detected platform: $Platform"

# Create installation directory
Write-Info "Creating installation directory: $InstallDir"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Download binary
Write-Info "Downloading CasGarage $Version..."
$BinaryUrl = if ($Version -eq "latest") {
    "https://github.com/casapps/casgarage/releases/latest/download/casgarage-$Platform.exe"
} else {
    "https://github.com/casapps/casgarage/releases/download/v$Version/casgarage-$Platform.exe"
}

$BinaryPath = Join-Path $InstallDir "casgarage.exe"

try {
    Invoke-WebRequest -Uri $BinaryUrl -OutFile $BinaryPath -UseBasicParsing
    Write-Info "Binary downloaded successfully"
} catch {
    Write-Error-Custom "Failed to download binary from $BinaryUrl : $_"
}

# Create directories
Write-Info "Creating directories"
$DbDir = Join-Path $DataDir "db"
$BlocksDir = Join-Path $DataDir "blocks"
$SslDir = Join-Path $ConfigDir "ssl\certs"

New-Item -ItemType Directory -Path $DataDir -Force | Out-Null
New-Item -ItemType Directory -Path $DbDir -Force | Out-Null
New-Item -ItemType Directory -Path $BlocksDir -Force | Out-Null
New-Item -ItemType Directory -Path $ConfigDir -Force | Out-Null
New-Item -ItemType Directory -Path $SslDir -Force | Out-Null
New-Item -ItemType Directory -Path $LogDir -Force | Out-Null

# Set environment variables for the service
$EnvVars = @{
    "DATA_DIR" = $DataDir
    "CONFIG_DIR" = $ConfigDir
    "LOG_DIR" = $LogDir
    "RUST_LOG" = "info"
}

# Create wrapper script for the service
$WrapperScript = @"
`$env:DATA_DIR = "$($EnvVars.DATA_DIR)"
`$env:CONFIG_DIR = "$($EnvVars.CONFIG_DIR)"
`$env:LOG_DIR = "$($EnvVars.LOG_DIR)"
`$env:RUST_LOG = "$($EnvVars.RUST_LOG)"

& "$InstallDir\casgarage.exe"
"@

$WrapperPath = Join-Path $InstallDir "casgarage-service.ps1"
$WrapperScript | Out-File -FilePath $WrapperPath -Encoding UTF8

# Check if service already exists
$ExistingService = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue

if ($ExistingService) {
    Write-Warn "Service '$ServiceName' already exists. Stopping and removing..."

    if ($ExistingService.Status -eq 'Running') {
        Stop-Service -Name $ServiceName -Force
    }

    # Remove existing service
    $service = Get-WmiObject -Class Win32_Service -Filter "Name='$ServiceName'"
    if ($service) {
        $service.Delete() | Out-Null
        Start-Sleep -Seconds 2
    }
}

# Install as Windows Service using NSSM (Non-Sucking Service Manager)
# First, check if NSSM is available, if not, use sc.exe
Write-Info "Installing Windows Service"

# Try using NSSM if available
$nssmPath = Get-Command nssm -ErrorAction SilentlyContinue

if ($nssmPath) {
    # Install with NSSM (preferred method)
    Write-Info "Using NSSM to install service"

    & nssm install $ServiceName "$InstallDir\casgarage.exe"
    & nssm set $ServiceName AppDirectory $InstallDir
    & nssm set $ServiceName DisplayName $ServiceDisplayName
    & nssm set $ServiceName Description "Self-hosted S3-compatible object storage with web UI"
    & nssm set $ServiceName Start SERVICE_AUTO_START

    # Set environment variables
    foreach ($key in $EnvVars.Keys) {
        & nssm set $ServiceName AppEnvironmentExtra "$key=$($EnvVars[$key])"
    }

    # Set stdout/stderr logs
    & nssm set $ServiceName AppStdout (Join-Path $LogDir "stdout.log")
    & nssm set $ServiceName AppStderr (Join-Path $LogDir "stderr.log")
    & nssm set $ServiceName AppRotateFiles 1
    & nssm set $ServiceName AppRotateBytes 10485760  # 10MB

} else {
    # Fallback to sc.exe (basic service installation)
    Write-Warn "NSSM not found. Installing as basic service with sc.exe"
    Write-Info "For better service management, consider installing NSSM: https://nssm.cc/"

    $BinaryFullPath = Join-Path $InstallDir "casgarage.exe"
    $ServiceCmd = "`"$BinaryFullPath`""

    & sc.exe create $ServiceName binPath= $ServiceCmd start= auto DisplayName= $ServiceDisplayName
    & sc.exe description $ServiceName "Self-hosted S3-compatible object storage with web UI"
}

# Configure Windows Firewall
Write-Info "Configuring Windows Firewall rules"

# HTTP port (80)
$firewallRuleHttp = Get-NetFirewallRule -DisplayName "CasGarage HTTP" -ErrorAction SilentlyContinue
if (-not $firewallRuleHttp) {
    New-NetFirewallRule -DisplayName "CasGarage HTTP" -Direction Inbound -Protocol TCP -LocalPort 80 -Action Allow | Out-Null
    Write-Info "Firewall rule created for HTTP (port 80)"
}

# HTTPS port (443)
$firewallRuleHttps = Get-NetFirewallRule -DisplayName "CasGarage HTTPS" -ErrorAction SilentlyContinue
if (-not $firewallRuleHttps) {
    New-NetFirewallRule -DisplayName "CasGarage HTTPS" -Direction Inbound -Protocol TCP -LocalPort 443 -Action Allow | Out-Null
    Write-Info "Firewall rule created for HTTPS (port 443)"
}

# Print completion message
Write-Host ""
Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║     ✅ Installation Complete!          ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "Installation Directory: $InstallDir" -ForegroundColor Cyan
Write-Host "Data Directory:         $DataDir" -ForegroundColor Cyan
Write-Host "Config Directory:       $ConfigDir" -ForegroundColor Cyan
Write-Host "Log Directory:          $LogDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "📚 Next steps:" -ForegroundColor Yellow
Write-Host ""
Write-Host "  1. Start service:"
Write-Host "     Start-Service -Name $ServiceName" -ForegroundColor Gray
Write-Host "  2. Check status:"
Write-Host "     casgarage --status" -ForegroundColor Gray
Write-Host "  3. View logs:"
Write-Host "     Get-Content `"$LogDir\stdout.log`" -Wait" -ForegroundColor Gray
Write-Host "  4. Access admin UI:"
Write-Host "     http://localhost:64900" -ForegroundColor Green
Write-Host ""
Write-Host "Other commands:" -ForegroundColor Yellow
Write-Host "  Stop service:    Stop-Service -Name $ServiceName"
Write-Host "  Service status:  Get-Service -Name $ServiceName"
Write-Host "  Uninstall:       .\uninstall.ps1"
Write-Host ""
Write-Host "📖 Documentation: https://casgarage.readthedocs.io" -ForegroundColor Cyan
Write-Host ""
