#!/usr/bin/env bash
# CasGarage Linux-Specific Installation Script
# Supports: Ubuntu, Debian, RHEL, CentOS, Fedora, Arch, Alpine, and derivatives

set -e

# Colors and emojis
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
PROJECT_NAME="casgarage"
INSTALL_DIR="/usr/local/bin"
DATA_DIR="/var/lib/${PROJECT_NAME}"
CONFIG_DIR="/etc/${PROJECT_NAME}"
LOG_DIR="/var/log/${PROJECT_NAME}"
USER_NAME="${PROJECT_NAME}"

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO="$ID"
        DISTRO_VERSION="$VERSION_ID"
    elif [ -f /etc/debian_version ]; then
        DISTRO="debian"
    elif [ -f /etc/redhat-release ]; then
        DISTRO="rhel"
    elif [ -f /etc/arch-release ]; then
        DISTRO="arch"
    else
        DISTRO="unknown"
    fi

    echo -e "${BLUE}🐧 Detected Linux distribution: ${DISTRO} ${DISTRO_VERSION}${NC}"
}

# Install dependencies
install_dependencies() {
    echo -e "${BLUE}📦 Installing dependencies...${NC}"

    case "$DISTRO" in
        ubuntu|debian)
            apt-get update -qq
            apt-get install -y curl ca-certificates >/dev/null 2>&1
            ;;
        rhel|centos|fedora|rocky|almalinux)
            yum install -y curl ca-certificates >/dev/null 2>&1 || \
            dnf install -y curl ca-certificates >/dev/null 2>&1
            ;;
        arch|manjaro)
            pacman -Sy --noconfirm curl ca-certificates >/dev/null 2>&1
            ;;
        alpine)
            apk add --no-cache curl ca-certificates bash >/dev/null 2>&1
            ;;
        opensuse*|sles)
            zypper install -y curl ca-certificates >/dev/null 2>&1
            ;;
    esac

    echo -e "${GREEN}✓ Dependencies installed${NC}"
}

# Configure firewall
configure_firewall() {
    echo -e "${BLUE}🔥 Configuring firewall...${NC}"

    # Detect and configure firewall
    if command -v ufw >/dev/null 2>&1 && ufw status | grep -q "Status: active"; then
        # UFW (Ubuntu/Debian)
        ufw allow 80/tcp comment "CasGarage HTTP" >/dev/null 2>&1
        ufw allow 443/tcp comment "CasGarage HTTPS" >/dev/null 2>&1
        echo -e "${GREEN}✓ UFW rules added${NC}"
    elif command -v firewall-cmd >/dev/null 2>&1 && systemctl is-active firewalld >/dev/null 2>&1; then
        # firewalld (RHEL/CentOS/Fedora)
        firewall-cmd --permanent --add-service=http >/dev/null 2>&1
        firewall-cmd --permanent --add-service=https >/dev/null 2>&1
        firewall-cmd --reload >/dev/null 2>&1
        echo -e "${GREEN}✓ Firewalld rules added${NC}"
    else
        echo -e "${YELLOW}⚠️  No active firewall detected. You may need to manually configure firewall rules.${NC}"
    fi
}

# Enable and configure SELinux policies (if applicable)
configure_selinux() {
    if command -v getenforce >/dev/null 2>&1 && [ "$(getenforce)" != "Disabled" ]; then
        echo -e "${BLUE}🔒 Configuring SELinux policies...${NC}"

        # Allow network binding
        setsebool -P httpd_can_network_connect 1 2>/dev/null || true

        # Set contexts
        semanage fcontext -a -t bin_t "${INSTALL_DIR}/${PROJECT_NAME}" 2>/dev/null || true
        semanage fcontext -a -t var_lib_t "${DATA_DIR}(/.*)?" 2>/dev/null || true
        semanage fcontext -a -t etc_t "${CONFIG_DIR}(/.*)?" 2>/dev/null || true
        semanage fcontext -a -t var_log_t "${LOG_DIR}(/.*)?" 2>/dev/null || true

        restorecon -Rv "${INSTALL_DIR}/${PROJECT_NAME}" 2>/dev/null || true
        restorecon -Rv "${DATA_DIR}" 2>/dev/null || true
        restorecon -Rv "${CONFIG_DIR}" 2>/dev/null || true
        restorecon -Rv "${LOG_DIR}" 2>/dev/null || true

        echo -e "${GREEN}✓ SELinux policies configured${NC}"
    fi
}

# Main
main() {
    if [[ $EUID -ne 0 ]]; then
        echo -e "${RED}❌ This script must be run as root${NC}"
        exit 1
    fi

    detect_distro
    install_dependencies

    # Call generic installer
    if [ -f "./install.sh" ]; then
        bash ./install.sh
    else
        echo -e "${RED}❌ Generic install.sh not found${NC}"
        exit 1
    fi

    configure_firewall
    configure_selinux

    echo -e "${GREEN}✓ Linux-specific configuration complete!${NC}"
}

main "$@"
