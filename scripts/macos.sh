#!/usr/bin/env bash
# CasGarage macOS-Specific Installation Script
# Supports: macOS 10.15+ (Catalina and later)

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
DATA_DIR="/usr/local/var/${PROJECT_NAME}"
CONFIG_DIR="/usr/local/etc/${PROJECT_NAME}"
LOG_DIR="/usr/local/var/log/${PROJECT_NAME}"
USER_NAME="_${PROJECT_NAME}"

# Check macOS version
check_macos_version() {
    local version
    version=$(sw_vers -productVersion)
    local major
    major=$(echo "$version" | cut -d. -f1)

    if [ "$major" -lt 10 ]; then
        echo -e "${RED}❌ macOS version $version is not supported. Requires 10.15+${NC}"
        exit 1
    fi

    echo -e "${BLUE}🍎 macOS version: $version${NC}"
}

# Install Homebrew if not present
check_homebrew() {
    if ! command -v brew >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  Homebrew not found. Installing Homebrew...${NC}"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        echo -e "${GREEN}✓ Homebrew installed${NC}"
    else
        echo -e "${GREEN}✓ Homebrew found${NC}"
    fi
}

# Install dependencies via Homebrew
install_dependencies() {
    echo -e "${BLUE}📦 Installing dependencies...${NC}"
    brew install curl >/dev/null 2>&1 || true
    echo -e "${GREEN}✓ Dependencies installed${NC}"
}

# Create macOS-specific user
create_macos_user() {
    echo -e "${BLUE}👤 Creating macOS service user...${NC}"

    # Check if user exists
    if dscl . -read "/Users/${USER_NAME}" >/dev/null 2>&1; then
        echo -e "${GREEN}✓ User '${USER_NAME}' already exists${NC}"
        return
    fi

    # Find unused UID between 100-999
    local uid
    for uid in $(seq 100 999); do
        if ! dscl . -list /Users UniqueID | awk '{print $2}' | grep -q "^${uid}$"; then
            break
        fi
    done

    # Create group
    dscl . -create "/Groups/${USER_NAME}"
    dscl . -create "/Groups/${USER_NAME}" PrimaryGroupID "${uid}"
    dscl . -create "/Groups/${USER_NAME}" RealName "CasGarage Service Group"

    # Create user
    dscl . -create "/Users/${USER_NAME}"
    dscl . -create "/Users/${USER_NAME}" UniqueID "${uid}"
    dscl . -create "/Users/${USER_NAME}" PrimaryGroupID "${uid}"
    dscl . -create "/Users/${USER_NAME}" UserShell /usr/bin/false
    dscl . -create "/Users/${USER_NAME}" NFSHomeDirectory "${DATA_DIR}"
    dscl . -create "/Users/${USER_NAME}" RealName "CasGarage Service User"
    dscl . -create "/Users/${USER_NAME}" IsHidden 1

    echo -e "${GREEN}✓ User created with UID/GID: ${uid}${NC}"
}

# Create macOS-specific directories
create_macos_directories() {
    echo -e "${BLUE}📁 Creating directories...${NC}"

    mkdir -p "${DATA_DIR}/db" "${DATA_DIR}/blocks"
    mkdir -p "${CONFIG_DIR}/ssl/certs"
    mkdir -p "${LOG_DIR}"
    mkdir -p "/tmp/${PROJECT_NAME}"

    chown -R "${USER_NAME}:staff" "${DATA_DIR}" "${CONFIG_DIR}" "${LOG_DIR}" "/tmp/${PROJECT_NAME}"
    chmod 750 "${DATA_DIR}" "${CONFIG_DIR}" "${LOG_DIR}"

    echo -e "${GREEN}✓ Directories created${NC}"
}

# Install launchd service
install_launchd_service() {
    echo -e "${BLUE}⚙️  Installing launchd service...${NC}"

    local plist_file="/Library/LaunchDaemons/com.casapps.${PROJECT_NAME}.plist"

    cat > "${plist_file}" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.casapps.${PROJECT_NAME}</string>

    <key>ProgramArguments</key>
    <array>
        <string>${INSTALL_DIR}/${PROJECT_NAME}</string>
    </array>

    <key>EnvironmentVariables</key>
    <dict>
        <key>DATA_DIR</key>
        <string>${DATA_DIR}</string>
        <key>CONFIG_DIR</key>
        <string>${CONFIG_DIR}</string>
        <key>LOG_DIR</key>
        <string>${LOG_DIR}</string>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>

    <key>UserName</key>
    <string>${USER_NAME}</string>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>

    <key>StandardOutPath</key>
    <string>${LOG_DIR}/stdout.log</string>

    <key>StandardErrorPath</key>
    <string>${LOG_DIR}/stderr.log</string>

    <key>ThrottleInterval</key>
    <integer>10</integer>
</dict>
</plist>
EOF

    chmod 644 "${plist_file}"
    chown root:wheel "${plist_file}"

    echo -e "${GREEN}✓ Launchd service installed${NC}"
}

# Configure macOS firewall
configure_firewall() {
    echo -e "${BLUE}🔥 Configuring macOS firewall...${NC}"

    # Add application to firewall
    if [ -f "/usr/libexec/ApplicationFirewall/socketfilterfw" ]; then
        /usr/libexec/ApplicationFirewall/socketfilterfw --add "${INSTALL_DIR}/${PROJECT_NAME}" >/dev/null 2>&1 || true
        /usr/libexec/ApplicationFirewall/socketfilterfw --unblockapp "${INSTALL_DIR}/${PROJECT_NAME}" >/dev/null 2>&1 || true
        echo -e "${GREEN}✓ Firewall configured${NC}"
    else
        echo -e "${YELLOW}⚠️  Firewall configuration skipped${NC}"
    fi
}

# Download and install binary
download_binary() {
    local arch
    case "$(uname -m)" in
        x86_64) arch="amd64";;
        arm64) arch="arm64";;
        *) echo -e "${RED}❌ Unsupported architecture: $(uname -m)${NC}"; exit 1;;
    esac

    local platform="macos_${arch}"
    echo -e "${BLUE}📦 Downloading ${PROJECT_NAME} for ${platform}...${NC}"

    local url="https://github.com/casapps/${PROJECT_NAME}/releases/latest/download/${PROJECT_NAME}-${platform}"

    curl -fsSL "${url}" -o "${INSTALL_DIR}/${PROJECT_NAME}"
    chmod +x "${INSTALL_DIR}/${PROJECT_NAME}"

    echo -e "${GREEN}✓ Binary installed${NC}"
}

# Main
main() {
    if [[ $EUID -ne 0 ]]; then
        echo -e "${RED}❌ This script must be run as root (use sudo)${NC}"
        exit 1
    fi

    echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║   🍎 CasGarage macOS Installation     ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
    echo ""

    check_macos_version
    check_homebrew
    install_dependencies
    create_macos_user
    download_binary
    create_macos_directories
    install_launchd_service
    configure_firewall

    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║     ✅ Installation Complete!          ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}📚 Next steps:${NC}"
    echo -e "   1. Load the service:"
    echo -e "      ${YELLOW}sudo launchctl load /Library/LaunchDaemons/com.casapps.${PROJECT_NAME}.plist${NC}"
    echo -e "   2. Start the service:"
    echo -e "      ${YELLOW}sudo launchctl start com.casapps.${PROJECT_NAME}${NC}"
    echo -e "   3. Check status:"
    echo -e "      ${YELLOW}${PROJECT_NAME} --status${NC}"
    echo -e "   4. View logs:"
    echo -e "      ${YELLOW}tail -f ${LOG_DIR}/stdout.log${NC}"
    echo -e "   5. Access the admin UI:"
    echo -e "      ${YELLOW}http://localhost:64900${NC}"
    echo ""
    echo -e "${BLUE}📖 Documentation: ${YELLOW}https://casgarage.readthedocs.io${NC}"
    echo ""
}

main "$@"
