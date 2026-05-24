#!/bin/sh
# Release packaging script
# Creates distributable archives for all platforms
# POSIX-compliant

set -e

# Colors
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    NC='\033[0m'
else
    GREEN=''
    NC=''
fi

info() { printf "${GREEN}[INFO]${NC} %s\n" "$1"; }

# Get version from Cargo.toml
get_version() {
    if command -v cargo >/dev/null 2>&1; then
        VERSION=$(cargo pkgid | cut -d'#' -f2 | cut -d':' -f2)
    else
        VERSION=$(grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)
    fi
    echo "$VERSION"
}

# Create release directory
setup_release_dir() {
    VERSION=$(get_version)
    RELEASE_DIR="release/casgarage-$VERSION"

    info "Creating release directory: $RELEASE_DIR"
    mkdir -p "$RELEASE_DIR"
}

# Package binary for a target
package_target() {
    target="$1"
    VERSION=$(get_version)
    RELEASE_DIR="release/casgarage-$VERSION"

    binary="target/$target/release/casgarage"
    binary_exe="target/$target/release/casgarage.exe"

    if [ -f "$binary" ]; then
        info "Packaging $target..."

        archive_name="casgarage-$VERSION-$target.tar.gz"

        tar -czf "$RELEASE_DIR/$archive_name" \
            -C "target/$target/release" casgarage \
            -C ../../../ README.md LICENSE CLAUDE.md

        # Create checksum
        (cd "$RELEASE_DIR" && sha256sum "$archive_name" > "$archive_name.sha256")

    elif [ -f "$binary_exe" ]; then
        info "Packaging $target..."

        archive_name="casgarage-$VERSION-$target.zip"

        # Use zip for Windows
        if command -v zip >/dev/null 2>&1; then
            (cd "target/$target/release" && \
             zip -q "../../../$RELEASE_DIR/$archive_name" casgarage.exe)
            (cd . && zip -q "$RELEASE_DIR/$archive_name" README.md LICENSE CLAUDE.md)
        else
            # Fallback to tar
            archive_name="casgarage-$VERSION-$target.tar.gz"
            tar -czf "$RELEASE_DIR/$archive_name" \
                -C "target/$target/release" casgarage.exe \
                -C ../../../ README.md LICENSE CLAUDE.md
        fi

        # Create checksum
        (cd "$RELEASE_DIR" && sha256sum "$archive_name" > "$archive_name.sha256")
    fi
}

# Package all built binaries
package_all() {
    # Find all built binaries
    for target_dir in target/*/release/; do
        if [ -d "$target_dir" ]; then
            target=$(echo "$target_dir" | cut -d'/' -f2)

            # Skip build artifacts directories
            case "$target" in
                release|debug|build|deps|examples|incremental)
                    continue
                    ;;
            esac

            package_target "$target"
        fi
    done
}

# Create checksums file
create_checksums() {
    VERSION=$(get_version)
    RELEASE_DIR="release/casgarage-$VERSION"

    info "Creating checksums file..."

    (cd "$RELEASE_DIR" && \
     find . -name "*.tar.gz" -o -name "*.zip" | \
     xargs sha256sum > checksums.txt)
}

# Show release info
show_release_info() {
    VERSION=$(get_version)
    RELEASE_DIR="release/casgarage-$VERSION"

    echo ""
    info "Release packages created successfully!"
    echo ""
    echo "Version: $VERSION"
    echo "Location: $RELEASE_DIR"
    echo ""
    echo "Packages:"
    ls -lh "$RELEASE_DIR"/*.tar.gz "$RELEASE_DIR"/*.zip 2>/dev/null | awk '{print "  " $9 "  (" $5 ")"}'
    echo ""
}

# Main
main() {
    info "Starting release packaging"

    setup_release_dir
    package_all
    create_checksums
    show_release_info
}

main "$@"
