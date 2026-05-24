#!/bin/sh
# Production build script for CasGarage
# Supports local builds and cross-compilation for all platforms
# POSIX-compliant, works on Linux, macOS, BSD

set -e

# Colors
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    RED='\033[0;31m'
    NC='\033[0m'
else
    GREEN=''
    YELLOW=''
    RED=''
    NC=''
fi

info() { printf "${GREEN}[INFO]${NC} %s\n" "$1"; }
warn() { printf "${YELLOW}[WARN]${NC} %s\n" "$1"; }
error() { printf "${RED}[ERROR]${NC} %s\n" "$1"; exit 1; }

# Supported cross-compilation targets
ALL_TARGETS="
x86_64-unknown-linux-musl
aarch64-unknown-linux-musl
armv7-unknown-linux-musleabihf
x86_64-pc-windows-msvc
x86_64-apple-darwin
aarch64-apple-darwin
x86_64-unknown-freebsd
"

# Show usage
show_usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Build CasGarage for local platform or cross-compile for multiple platforms.

OPTIONS:
    -h, --help              Show this help message
    -c, --cross, --all      Cross-compile for all supported platforms
    -t, --target TARGET     Build for specific target (e.g., x86_64-unknown-linux-musl)
    --list-targets          List all supported cross-compilation targets
    --frontend-only         Build only the frontend
    --backend-only          Build only the backend
    --no-strip              Skip stripping binaries

EXAMPLES:
    $0                      Build for current platform
    $0 --cross              Build for all platforms
    $0 --target x86_64-unknown-linux-musl
    $0 --frontend-only      Build only frontend

SUPPORTED TARGETS:
$(echo "$ALL_TARGETS" | sed 's/^/    /')
EOF
}

# Parse arguments
CROSS_COMPILE=0
SPECIFIC_TARGET=""
BUILD_FRONTEND=1
BUILD_BACKEND=1
STRIP_BINARY=1

while [ $# -gt 0 ]; do
    case "$1" in
        -h|--help)
            show_usage
            exit 0
            ;;
        -c|--cross|--all)
            CROSS_COMPILE=1
            shift
            ;;
        -t|--target)
            SPECIFIC_TARGET="$2"
            shift 2
            ;;
        --list-targets)
            echo "Supported cross-compilation targets:"
            echo "$ALL_TARGETS"
            exit 0
            ;;
        --frontend-only)
            BUILD_BACKEND=0
            shift
            ;;
        --backend-only)
            BUILD_FRONTEND=0
            shift
            ;;
        --no-strip)
            STRIP_BINARY=0
            shift
            ;;
        *)
            error "Unknown option: $1\nUse --help for usage information"
            ;;
    esac
done

# Check dependencies
check_dependencies() {
    info "Checking dependencies..."

    if ! command -v cargo >/dev/null 2>&1; then
        error "cargo not found. Please install Rust from https://rustup.rs/"
    fi

    if [ $BUILD_FRONTEND -eq 1 ]; then
        if ! command -v trunk >/dev/null 2>&1; then
            warn "trunk not found. Installing..."
            cargo install trunk
        fi

        # Check for wasm target
        if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
            info "Adding wasm32-unknown-unknown target..."
            rustup target add wasm32-unknown-unknown
        fi
    fi

    if [ $CROSS_COMPILE -eq 1 ] || [ -n "$SPECIFIC_TARGET" ]; then
        if ! command -v cross >/dev/null 2>&1; then
            warn "cross not found. Installing..."
            cargo install cross --git https://github.com/cross-rs/cross
        fi
    fi
}

# Build frontend
build_frontend() {
    info "Building Leptos frontend..."

    cd frontend
    trunk build --release
    cd ..

    info "Frontend build complete"
}

# Build backend for current platform
build_backend_local() {
    info "Building Rust backend for current platform..."

    cargo build --release --bin casgarage

    if [ $STRIP_BINARY -eq 1 ] && command -v strip >/dev/null 2>&1; then
        info "Stripping binary..."
        strip target/release/casgarage 2>/dev/null || true
    fi

    info "Backend build complete"
}

# Build backend for specific target
build_backend_target() {
    target="$1"
    info "Building for $target..."

    cross build --release --target "$target" --bin casgarage

    # Strip binary if not Windows and strip is enabled
    if [ $STRIP_BINARY -eq 1 ]; then
        case "$target" in
            *windows*)
                ;;
            *)
                if [ -f "target/$target/release/casgarage" ]; then
                    cross-util strip "target/$target/release/casgarage" 2>/dev/null || true
                fi
                ;;
        esac
    fi
}

# Build backend for all targets
build_backend_cross() {
    info "Cross-compiling for all supported platforms..."

    for target in $ALL_TARGETS; do
        # Skip macOS targets if not on macOS (they require Xcode)
        case "$target" in
            *apple-darwin*)
                if [ "$(uname)" != "Darwin" ]; then
                    warn "Skipping $target (requires macOS)"
                    continue
                fi
                ;;
        esac

        build_backend_target "$target" || warn "Failed to build $target"
    done
}

# List built binaries
list_binaries() {
    echo ""
    info "Built binaries:"
    echo ""

    if [ $CROSS_COMPILE -eq 1 ]; then
        for target in $ALL_TARGETS; do
            binary="target/$target/release/casgarage"
            binary_exe="target/$target/release/casgarage.exe"

            if [ -f "$binary" ]; then
                size=$(du -h "$binary" | cut -f1)
                printf "  %-35s %s\n" "$target" "$size"
            elif [ -f "$binary_exe" ]; then
                size=$(du -h "$binary_exe" | cut -f1)
                printf "  %-35s %s\n" "$target" "$size"
            fi
        done
    elif [ -n "$SPECIFIC_TARGET" ]; then
        binary="target/$SPECIFIC_TARGET/release/casgarage"
        binary_exe="target/$SPECIFIC_TARGET/release/casgarage.exe"

        if [ -f "$binary" ]; then
            size=$(du -h "$binary" | cut -f1)
            printf "  %-35s %s\n" "$SPECIFIC_TARGET" "$size"
        elif [ -f "$binary_exe" ]; then
            size=$(du -h "$binary_exe" | cut -f1)
            printf "  %-35s %s\n" "$SPECIFIC_TARGET" "$size"
        fi
    else
        if [ -f "target/release/casgarage" ]; then
            size=$(du -h "target/release/casgarage" | cut -f1)
            printf "  %-35s %s\n" "current platform" "$size"
        fi
    fi

    echo ""
}

# Show build info
show_build_info() {
    echo ""
    info "Build complete!"

    if [ $CROSS_COMPILE -eq 0 ] && [ -z "$SPECIFIC_TARGET" ]; then
        echo ""
        echo "Binary location: target/release/casgarage"
        echo ""
        echo "To install system-wide, run:"
        echo "  sudo ./scripts/install.sh"
        echo ""
        echo "To run directly:"
        echo "  ./target/release/casgarage server"
        echo ""
    else
        echo ""
        echo "To create release packages, run:"
        echo "  ./scripts/release.sh"
        echo ""
    fi
}

# Main build flow
main() {
    if [ $CROSS_COMPILE -eq 1 ]; then
        info "Starting cross-compilation build for all platforms"
    elif [ -n "$SPECIFIC_TARGET" ]; then
        info "Starting build for target: $SPECIFIC_TARGET"
    else
        info "Starting CasGarage production build for current platform"
    fi

    check_dependencies

    # Build frontend (only for local builds unless specified)
    if [ $BUILD_FRONTEND -eq 1 ] && [ $CROSS_COMPILE -eq 0 ] && [ -z "$SPECIFIC_TARGET" ]; then
        build_frontend
    elif [ $BUILD_FRONTEND -eq 1 ] && [ $BUILD_BACKEND -eq 0 ]; then
        build_frontend
    fi

    # Build backend
    if [ $BUILD_BACKEND -eq 1 ]; then
        if [ $CROSS_COMPILE -eq 1 ]; then
            build_backend_cross
        elif [ -n "$SPECIFIC_TARGET" ]; then
            build_backend_target "$SPECIFIC_TARGET"
        else
            build_backend_local
        fi
    fi

    list_binaries
    show_build_info
}

main "$@"
