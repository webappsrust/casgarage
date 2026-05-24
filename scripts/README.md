# CasGarage Installation Scripts

This directory contains all production installation and deployment scripts for CasGarage.

## 🚀 Quick Installation

### Linux / BSD / macOS
```bash
# Download and run installer
curl -fsSL https://raw.githubusercontent.com/casapps/casgarage/main/scripts/install.sh | sudo bash

# Or download first, then run
wget https://raw.githubusercontent.com/casapps/casgarage/main/scripts/install.sh
chmod +x install.sh
sudo ./install.sh
```

### Linux-Specific (with firewall & SELinux configuration)
```bash
curl -fsSL https://raw.githubusercontent.com/casapps/casgarage/main/scripts/linux.sh | sudo bash
```

### macOS-Specific (with Homebrew integration)
```bash
curl -fsSL https://raw.githubusercontent.com/casapps/casgarage/main/scripts/macos.sh | sudo bash
```

### Windows
```powershell
# Run PowerShell as Administrator
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/casapps/casgarage/main/scripts/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

---

## 📋 Installation Scripts

### `install.sh` - Universal POSIX Installer
**OS/Distribution Agnostic installer for all Unix-like systems**

**Supports:**
- ✅ Linux (all distributions)
- ✅ macOS (10.15+ / Catalina and later)
- ✅ FreeBSD, OpenBSD, NetBSD
- ✅ All init systems: systemd, OpenRC, runit, BSD rc.d, launchd

**Features:**
- Automatic OS and init system detection
- Downloads appropriate binary for your platform
- Creates system user with unused UID/GID (100-999)
- Sets up directories: `/var/lib/casgarage`, `/etc/casgarage`, `/var/log/casgarage`
- Installs and configures service for your init system
- Configures proper permissions and security

**Usage:**
```bash
sudo ./install.sh

# Or with environment variables
sudo VERSION=1.0.0 ./install.sh
```

**Environment Variables:**
- `VERSION` - Version to install (default: `latest`)

**Post-Installation:**

*Linux (systemd):*
```bash
sudo systemctl start casgarage
sudo systemctl enable casgarage
sudo systemctl status casgarage
journalctl -u casgarage -f
```

*macOS (launchd):*
```bash
sudo launchctl load /Library/LaunchDaemons/com.casapps.casgarage.plist
sudo launchctl start com.casapps.casgarage
casgarage --status
tail -f /usr/local/var/log/casgarage/stdout.log
```

*BSD (rc.d):*
```bash
echo 'casgarage_enable="YES"' | sudo tee -a /etc/rc.conf
sudo service casgarage start
sudo service casgarage status
```

---

### `linux.sh` - Linux-Specific Installer
**Enhanced installer with Linux distribution-specific features**

**Additional Features:**
- Installs required dependencies (curl, ca-certificates)
- Configures firewall (UFW, firewalld, iptables)
- Configures SELinux policies (RHEL/CentOS/Fedora)
- Distribution-specific optimizations

**Supports:**
- Ubuntu, Debian, and derivatives
- RHEL, CentOS, Fedora, Rocky, AlmaLinux
- Arch Linux, Manjaro
- Alpine Linux
- openSUSE, SLES

**Usage:**
```bash
sudo ./linux.sh
```

---

### `macos.sh` - macOS-Specific Installer
**Optimized installer for macOS with Homebrew integration**

**Features:**
- Checks and installs Homebrew if needed
- Creates macOS-specific service user (hidden, UID 100-999)
- Uses macOS-standard directories: `/usr/local/var/casgarage`, `/usr/local/etc/casgarage`
- Configures macOS Application Firewall
- Creates launchd service with proper permissions
- Supports both Intel and Apple Silicon

**Requirements:**
- macOS 10.15 (Catalina) or later
- Administrator privileges

**Usage:**
```bash
sudo ./macos.sh
```

---

### `install.ps1` - Windows PowerShell Installer
**Complete Windows installation with service management**

**Features:**
- Downloads appropriate binary (amd64/arm64)
- Creates directory structure in `C:\ProgramData\CasGarage`
- Installs as Windows Service (NSSM or sc.exe)
- Configures Windows Firewall rules
- Sets up log rotation
- Service auto-starts on boot

**Requirements:**
- Windows 10/11 or Windows Server 2019+
- PowerShell 5.1 or later
- Administrator privileges
- Optional: NSSM for better service management

**Usage:**
```powershell
# Run as Administrator
.\install.ps1

# Custom installation
.\install.ps1 -InstallDir "D:\CasGarage" -Version "1.0.0"
```

**Parameters:**
- `-InstallDir` - Installation directory (default: `C:\Program Files\CasGarage`)
- `-DataDir` - Data directory (default: `C:\ProgramData\CasGarage\data`)
- `-ConfigDir` - Config directory (default: `C:\ProgramData\CasGarage\config`)
- `-LogDir` - Log directory (default: `C:\ProgramData\CasGarage\logs`)
- `-Version` - Version to install (default: `latest`)

**Post-Installation:**
```powershell
# Start service
Start-Service CasGarage

# Check status
Get-Service CasGarage

# View logs
Get-Content "C:\ProgramData\CasGarage\logs\stdout.log" -Wait

# Access admin UI
# http://localhost:64900
```

---

## 🗑️ Uninstallation Scripts

### `uninstall.sh` - Unix/Linux/BSD/macOS Uninstaller
```bash
sudo ./uninstall.sh

# Remove data too
sudo ./uninstall.sh --remove-data
```

### `uninstall.ps1` - Windows Uninstaller
```powershell
.\uninstall.ps1

# Remove data too
.\uninstall.ps1 -RemoveData
```

---

## 📦 Build Scripts

### `build.sh` - Multi-Platform Build Script
**Cross-compilation build script (see Makefile for simplified usage)**

**Usage:**
```bash
# Build for current platform
./scripts/build.sh

# Cross-compile for all platforms
./scripts/build.sh --cross
```

### `release.sh` - Release Packaging
**Creates GitHub release packages with checksums**

```bash
./scripts/release.sh
```

---

## 🏗️ Directory Structure After Installation

### Linux/BSD
```
/usr/local/bin/casgarage              # Binary
/var/lib/casgarage/                   # Data directory
├── db/                               # SQLite database
└── blocks/                           # Object storage blocks
/etc/casgarage/                       # Config directory
└── ssl/certs/                        # SSL certificates
/var/log/casgarage/                   # Logs
/etc/systemd/system/casgarage.service # systemd service (Linux)
```

### macOS
```
/usr/local/bin/casgarage              # Binary
/usr/local/var/casgarage/             # Data directory
├── db/
└── blocks/
/usr/local/etc/casgarage/             # Config directory
└── ssl/certs/
/usr/local/var/log/casgarage/         # Logs
/Library/LaunchDaemons/com.casapps.casgarage.plist
```

### Windows
```
C:\Program Files\CasGarage\casgarage.exe
C:\ProgramData\CasGarage\data\        # Data directory
├── db\
└── blocks\
C:\ProgramData\CasGarage\config\      # Config directory
└── ssl\certs\
C:\ProgramData\CasGarage\logs\        # Logs
```

---

## 🔧 Configuration

CasGarage uses **database-driven configuration** - no config files needed! All settings are managed through the Admin UI or stored in the database.

**Environment Variables (Docker/Container):**
- `DATA_DIR` - Data directory path
- `CONFIG_DIR` - Config directory path (for SSL certs, runtime files)
- `LOG_DIR` - Log directory path
- `PORT` - Port(s) to listen on (e.g., `80` or `8080,8443`)
- `SERVER_ADDRESS` - Listen address

**Command-Line Options:**
```bash
casgarage --help
casgarage --port 8080
casgarage --port 80,443              # HTTP,HTTPS
casgarage --address 192.168.1.100
casgarage --datadir /custom/data
casgarage --configdir /custom/config
casgarage --logdir /custom/logs
casgarage --status                   # Health check
```

---

## 🔐 Security Features

### Automatic Let's Encrypt
If running on ports 80,443, CasGarage automatically:
1. Checks `/etc/letsencrypt/live` for existing certificates
2. If not found, requests certificates via Let's Encrypt
3. Saves certificates to `/etc/casgarage/ssl/certs` or `CONFIG_DIR/ssl/certs`
4. Automatically renews certificates before expiry

**Supported ACME Challenges:**
- HTTP-01 (standard HTTP challenge)
- DNS-01 (all DNS providers + RFC2136)
- TLS-ALPN-01 (TLS-based challenge)

### System User
All installations create a dedicated system user with:
- Non-interactive shell (`/sbin/nologin` or `/usr/bin/false`)
- UID/GID between 100-999
- Home directory set to data directory
- Minimal permissions (only data/config/log access)

### Systemd Security Hardening (Linux)
- `NoNewPrivileges=true`
- `PrivateTmp=true`
- `ProtectSystem=strict`
- `ProtectHome=true`
- `ReadWritePaths` limited to data directories
- `CapabilityBoundingSet=CAP_NET_BIND_SERVICE`

---

## 🌐 Network Configuration

### Default Ports
- **Development/User Install**: Random unused port in 64xxx range (e.g., 64900)
- **Production (80,443)**: Full HTTP/HTTPS with automatic SSL

### Reverse Proxy Support
CasGarage is designed to run behind a reverse proxy:
- Nginx, Apache, Caddy, Traefik supported
- Respects `X-Forwarded-*` headers
- WebSocket support for real-time features
- Proper CORS handling

**Example Nginx Configuration:**
```nginx
server {
    listen 80;
    server_name casgarage.example.com;

    location / {
        proxy_pass http://127.0.0.1:64900;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

---

## 🐛 Troubleshooting

### Installation Issues

**"Permission denied"**
```bash
# Make scripts executable
chmod +x scripts/*.sh

# Run with sudo
sudo ./install.sh
```

**"Binary not found" / Download fails**
```bash
# Check internet connectivity
curl -I https://github.com

# Manually specify version
sudo VERSION=0.1.0 ./install.sh

# Or download manually
wget https://github.com/casapps/casgarage/releases/latest/download/casgarage-linux_amd64
chmod +x casgarage-linux_amd64
sudo mv casgarage-linux_amd64 /usr/local/bin/casgarage
```

### Service Issues

**Service won't start**
```bash
# Check logs
journalctl -u casgarage -n 50        # Linux (systemd)
tail -f /var/log/casgarage/*.log     # BSD
cat /usr/local/var/log/casgarage/*.log  # macOS

# Check status
casgarage --status

# Test manually
sudo -u casgarage /usr/local/bin/casgarage
```

**Port already in use**
```bash
# Linux/BSD/macOS
sudo lsof -i :64900
sudo netstat -tlnp | grep 64900

# Windows
netstat -ano | findstr :64900
```

**Database errors on first start**
- Normal! Database is created on first run
- Server enters read-only mode if database unreachable
- Follow Admin UI instructions for recovery

### Firewall Issues

**Can't access from other machines**

*Linux (UFW):*
```bash
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
```

*Linux (firewalld):*
```bash
sudo firewall-cmd --permanent --add-service=http
sudo firewall-cmd --permanent --add-service=https
sudo firewall-cmd --reload
```

*macOS:*
```bash
# Add to Application Firewall
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add /usr/local/bin/casgarage
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --unblock /usr/local/bin/casgarage
```

*Windows:*
```powershell
New-NetFirewallRule -DisplayName "CasGarage HTTP" -Direction Inbound -Protocol TCP -LocalPort 80 -Action Allow
New-NetFirewallRule -DisplayName "CasGarage HTTPS" -Direction Inbound -Protocol TCP -LocalPort 443 -Action Allow
```

---

## 📚 Additional Resources

- **Documentation**: https://casgarage.readthedocs.io
- **GitHub Repository**: https://github.com/casapps/casgarage
- **Issue Tracker**: https://github.com/casapps/casgarage/issues
- **Community Discord**: https://discord.gg/casapps

---

## 🤝 Contributing

See `CONTRIBUTING.md` in the project root for development setup and contribution guidelines.

For development scripts, see `tests/` directory.
