#!/bin/sh
# CasGarage Uninstallation Script
# POSIX-compliant uninstaller for Linux, macOS, and BSD systems

set -e

# Configuration
BINARY_NAME="casgarage"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
DATA_DIR="${DATA_DIR:-/var/lib/casgarage}"
USER="${CASGARAGE_USER:-casgarage}"

# Colors
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

info() { printf "${GREEN}[INFO]${NC} %s\n" "$1"; }
warn() { printf "${YELLOW}[WARN]${NC} %s\n" "$1"; }
error() { printf "${RED}[ERROR]${NC} %s\n" "$1"; exit 1; }

# Detect init system
detect_init() {
    if command -v systemctl >/dev/null 2>&1; then
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
    fi
}

# Check root
check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        error "This script must be run as root"
    fi
}

# Stop and remove service
remove_service() {
    info "Removing service..."

    case "$INIT_SYSTEM" in
        systemd)
            systemctl stop casgarage 2>/dev/null || true
            systemctl disable casgarage 2>/dev/null || true
            rm -f /etc/systemd/system/casgarage.service
            systemctl daemon-reload
            ;;
        openrc)
            rc-service casgarage stop 2>/dev/null || true
            rc-update del casgarage 2>/dev/null || true
            rm -f /etc/init.d/casgarage
            ;;
        runit)
            sv stop casgarage 2>/dev/null || true
            rm -rf /etc/sv/casgarage
            rm -f /var/service/casgarage
            ;;
        bsd-rc)
            service casgarage stop 2>/dev/null || true
            rm -f /etc/rc.d/casgarage
            ;;
        launchd)
            launchctl unload /Library/LaunchDaemons/com.casapps.casgarage.plist 2>/dev/null || true
            rm -f /Library/LaunchDaemons/com.casapps.casgarage.plist
            ;;
    esac
}

# Remove binary
remove_binary() {
    info "Removing binary..."
    rm -f "$INSTALL_DIR/$BINARY_NAME"
}

# Remove data (with confirmation)
remove_data() {
    printf "${YELLOW}Remove data directory $DATA_DIR? [y/N]: ${NC}"
    read -r response
    case "$response" in
        [yY][eE][sS]|[yY])
            info "Removing data directory..."
            rm -rf "$DATA_DIR"
            rm -rf /tmp/casgarage
            ;;
        *)
            warn "Data directory preserved at $DATA_DIR"
            ;;
    esac
}

# Remove user
remove_user() {
    info "Removing user: $USER"

    if [ "$(uname)" = "Darwin" ]; then
        dscl . -delete /Users/"$USER" 2>/dev/null || true
    elif [ "$(uname)" = "FreeBSD" ] || [ "$(uname)" = "OpenBSD" ] || [ "$(uname)" = "NetBSD" ]; then
        pw userdel "$USER" 2>/dev/null || true
    else
        userdel "$USER" 2>/dev/null || true
    fi
}

main() {
    info "Starting CasGarage uninstallation"

    detect_init
    check_root
    remove_service
    remove_binary
    remove_data
    remove_user

    info "Uninstallation complete!"
}

main "$@"
