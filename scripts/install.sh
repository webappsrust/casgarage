#!/bin/sh
# CasGarage Installation Script
# POSIX-compliant installer for Linux, macOS, and BSD systems
# Detects and supports: systemd, OpenRC, runit, BSD rc.d, launchd (macOS)

set -e

# Configuration
BINARY_NAME="casgarage"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
DATA_DIR="${DATA_DIR:-/var/lib/casgarage}"
USER="${CASGARAGE_USER:-casgarage}"
GROUP="${CASGARAGE_GROUP:-casgarage}"

# Colors for output
if [ -t 1 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    NC='\033[0m'
else
    RED=''
    GREEN=''
    YELLOW=''
    NC=''
fi

# Logging functions
info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
    exit 1
}

# Detect operating system
detect_os() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS="$ID"
        OS_VERSION="$VERSION_ID"
    elif [ "$(uname)" = "Darwin" ]; then
        OS="macos"
        OS_VERSION="$(sw_vers -productVersion)"
    elif [ "$(uname)" = "FreeBSD" ]; then
        OS="freebsd"
        OS_VERSION="$(uname -r)"
    elif [ "$(uname)" = "OpenBSD" ]; then
        OS="openbsd"
        OS_VERSION="$(uname -r)"
    elif [ "$(uname)" = "NetBSD" ]; then
        OS="netbsd"
        OS_VERSION="$(uname -r)"
    else
        error "Unsupported operating system"
    fi
    info "Detected OS: $OS $OS_VERSION"
}

# Detect init system
detect_init() {
    if command -v systemctl >/dev/null 2>&1 && systemctl --version >/dev/null 2>&1; then
        INIT_SYSTEM="systemd"
    elif [ -d /etc/runit ]; then
        INIT_SYSTEM="runit"
    elif [ -f /sbin/openrc ] || [ -f /usr/sbin/openrc ]; then
        INIT_SYSTEM="openrc"
    elif [ "$(uname)" = "Darwin" ]; then
        INIT_SYSTEM="launchd"
    elif [ -d /etc/rc.d ] && [ "$(uname)" != "Linux" ]; then
        INIT_SYSTEM="bsd-rc"
    else
        INIT_SYSTEM="unknown"
        warn "Could not detect init system, service installation will be skipped"
    fi
    info "Detected init system: $INIT_SYSTEM"
}

# Check if running as root
check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        error "This script must be run as root. Please use sudo or run as root."
    fi
}

# Create user and group
create_user() {
    info "Creating user and group: $USER"

    case "$OS" in
        macos)
            # macOS uses dscl
            if ! dscl . -read /Users/"$USER" >/dev/null 2>&1; then
                # Find next available UID
                NEXT_UID=$(dscl . -list /Users UniqueID | awk '{print $2}' | sort -n | tail -1)
                NEXT_UID=$((NEXT_UID + 1))

                dscl . -create /Users/"$USER"
                dscl . -create /Users/"$USER" UserShell /usr/bin/false
                dscl . -create /Users/"$USER" UniqueID "$NEXT_UID"
                dscl . -create /Users/"$USER" PrimaryGroupID 20
                dscl . -create /Users/"$USER" RealName "CasGarage Service"
            fi
            ;;
        freebsd|openbsd|netbsd)
            # BSD systems
            if ! id "$USER" >/dev/null 2>&1; then
                pw useradd "$USER" -d /nonexistent -s /usr/sbin/nologin -c "CasGarage Service" || true
            fi
            ;;
        *)
            # Linux systems
            if ! id "$USER" >/dev/null 2>&1; then
                useradd -r -s /bin/false -d /nonexistent -c "CasGarage Service" "$USER" || true
            fi
            ;;
    esac
}

# Install binary
install_binary() {
    info "Installing binary to $INSTALL_DIR"

    BINARY_PATH="target/release/$BINARY_NAME"
    if [ ! -f "$BINARY_PATH" ]; then
        error "Binary not found at $BINARY_PATH. Please build first with: cargo build --release"
    fi

    cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
    chmod 755 "$INSTALL_DIR/$BINARY_NAME"

    info "Binary installed successfully"
}

# Create data directories
create_directories() {
    info "Creating data directories"

    mkdir -p "$DATA_DIR/blocks"
    mkdir -p "$DATA_DIR/db"
    mkdir -p /tmp/casgarage

    chown -R "$USER:$GROUP" "$DATA_DIR" 2>/dev/null || chown -R "$USER" "$DATA_DIR"
    chown -R "$USER:$GROUP" /tmp/casgarage 2>/dev/null || chown -R "$USER" /tmp/casgarage

    chmod 755 "$DATA_DIR"
    chmod 700 "$DATA_DIR/db"
    chmod 755 /tmp/casgarage

    info "Directories created and permissions set"
}

# Install systemd service
install_systemd() {
    info "Installing systemd service"

    cat > /etc/systemd/system/casgarage.service <<'EOF'
[Unit]
Description=CasGarage S3 Object Storage
After=network.target
Documentation=https://github.com/casapps/casgarage

[Service]
Type=simple
User=casgarage
Group=casgarage
ExecStart=/usr/local/bin/casgarage server
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=casgarage

# Environment
Environment="RUST_LOG=info"
Environment="TMPDIR=/tmp/casgarage"

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/casgarage /tmp/casgarage
PrivateDevices=true
ProtectKernelTunables=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    info "Systemd service installed"
}

# Install OpenRC service
install_openrc() {
    info "Installing OpenRC service"

    cat > /etc/init.d/casgarage <<'EOF'
#!/sbin/openrc-run

name="CasGarage"
description="CasGarage S3 Object Storage"
command="/usr/local/bin/casgarage"
command_args="server"
command_user="casgarage:casgarage"
pidfile="/run/casgarage.pid"
command_background="yes"

depend() {
    need net
    after firewall
}

start_pre() {
    checkpath --directory --owner casgarage:casgarage --mode 0755 /tmp/casgarage
}
EOF

    chmod 755 /etc/init.d/casgarage
    info "OpenRC service installed"
}

# Install runit service
install_runit() {
    info "Installing runit service"

    mkdir -p /etc/sv/casgarage

    cat > /etc/sv/casgarage/run <<'EOF'
#!/bin/sh
exec 2>&1
export RUST_LOG=info
export TMPDIR=/tmp/casgarage
exec chpst -u casgarage:casgarage /usr/local/bin/casgarage server
EOF

    chmod 755 /etc/sv/casgarage/run

    # Create log directory
    mkdir -p /etc/sv/casgarage/log
    cat > /etc/sv/casgarage/log/run <<'EOF'
#!/bin/sh
exec svlogd -tt /var/log/casgarage
EOF
    chmod 755 /etc/sv/casgarage/log/run
    mkdir -p /var/log/casgarage
    chown casgarage:casgarage /var/log/casgarage

    info "Runit service installed"
}

# Install BSD rc.d service
install_bsd_rc() {
    info "Installing BSD rc.d service"

    cat > /etc/rc.d/casgarage <<'EOF'
#!/bin/sh

# PROVIDE: casgarage
# REQUIRE: NETWORKING
# KEYWORD: shutdown

. /etc/rc.subr

name="casgarage"
rcvar="casgarage_enable"
command="/usr/local/bin/casgarage"
command_args="server"
casgarage_user="casgarage"

load_rc_config $name
: ${casgarage_enable:=NO}

pidfile="/var/run/casgarage.pid"
command_args="server &"

run_rc_command "$1"
EOF

    chmod 755 /etc/rc.d/casgarage
    info "BSD rc.d service installed"
}

# Install macOS launchd service
install_launchd() {
    info "Installing launchd service"

    cat > /Library/LaunchDaemons/com.casapps.casgarage.plist <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.casapps.casgarage</string>

    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/casgarage</string>
        <string>server</string>
    </array>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <true/>

    <key>StandardOutPath</key>
    <string>/var/log/casgarage.log</string>

    <key>StandardErrorPath</key>
    <string>/var/log/casgarage.error.log</string>

    <key>UserName</key>
    <string>casgarage</string>

    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>info</string>
        <key>TMPDIR</key>
        <string>/tmp/casgarage</string>
    </dict>
</dict>
</plist>
EOF

    chmod 644 /Library/LaunchDaemons/com.casapps.casgarage.plist
    info "Launchd service installed"
}

# Install service based on init system
install_service() {
    case "$INIT_SYSTEM" in
        systemd)
            install_systemd
            ;;
        openrc)
            install_openrc
            ;;
        runit)
            install_runit
            ;;
        bsd-rc)
            install_bsd_rc
            ;;
        launchd)
            install_launchd
            ;;
        *)
            warn "Service installation skipped for unknown init system"
            return
            ;;
    esac
}

# Print post-installation instructions
print_instructions() {
    echo ""
    info "Installation complete!"
    echo ""
    echo "Next steps:"
    echo ""

    case "$INIT_SYSTEM" in
        systemd)
            echo "  Start service:   sudo systemctl start casgarage"
            echo "  Enable on boot:  sudo systemctl enable casgarage"
            echo "  Check status:    sudo systemctl status casgarage"
            echo "  View logs:       sudo journalctl -u casgarage -f"
            ;;
        openrc)
            echo "  Start service:   sudo rc-service casgarage start"
            echo "  Enable on boot:  sudo rc-update add casgarage"
            echo "  Check status:    sudo rc-service casgarage status"
            ;;
        runit)
            echo "  Enable service:  sudo ln -s /etc/sv/casgarage /var/service/"
            echo "  Check status:    sudo sv status casgarage"
            echo "  View logs:       sudo svlogd /var/log/casgarage"
            ;;
        bsd-rc)
            echo "  Enable service:  Add 'casgarage_enable=\"YES\"' to /etc/rc.conf"
            echo "  Start service:   sudo service casgarage start"
            echo "  Check status:    sudo service casgarage status"
            ;;
        launchd)
            echo "  Load service:    sudo launchctl load /Library/LaunchDaemons/com.casapps.casgarage.plist"
            echo "  Start service:   sudo launchctl start com.casapps.casgarage"
            echo "  Check status:    sudo launchctl list | grep casgarage"
            echo "  View logs:       tail -f /var/log/casgarage.log"
            ;;
    esac

    echo ""
    echo "  Access admin UI: http://localhost:3900"
    echo "  S3 API endpoint: http://localhost:3901"
    echo ""
}

# Main installation flow
main() {
    info "Starting CasGarage installation"

    detect_os
    detect_init
    check_root
    create_user
    install_binary
    create_directories
    install_service
    print_instructions
}

# Run main function
main "$@"
