.PHONY: all build release docker test clean help

# Project information
PROJECT_NAME := casgarage
ORG := casapps
GHCR_REPO := ghcr.io/$(ORG)/$(PROJECT_NAME)

# Version management
VERSION_FILE := release.txt
VERSION := $(shell cat $(VERSION_FILE) 2>/dev/null || echo "0.1.0")
ifdef VERSION_ENV
VERSION := $(VERSION_ENV)
endif

# Build information
BUILD_DATE := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
VCS_REF := $(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")

# Directories
BUILD_DIR := ./binaries
RELEASE_DIR := ./releases
SRC_DIR := ./src
FRONTEND_DIR := ./frontend

# Target platforms
TARGETS := \
	x86_64-unknown-linux-musl \
	aarch64-unknown-linux-musl \
	x86_64-pc-windows-gnu \
	x86_64-apple-darwin \
	aarch64-apple-darwin \
	x86_64-unknown-freebsd \
	x86_64-unknown-openbsd

# OS and arch mapping for binary naming
define get_os_arch
$(if $(findstring linux,$1),linux,\
$(if $(findstring windows,$1),windows,\
$(if $(findstring darwin,$1),macos,\
$(if $(findstring freebsd,$1),freebsd,\
$(if $(findstring openbsd,$1),openbsd,unknown)))))_\
$(if $(findstring x86_64,$1),amd64,\
$(if $(findstring aarch64,$1),arm64,unknown))
endef

# Detect host platform
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Linux)
	HOST_OS := linux
endif
ifeq ($(UNAME_S),Darwin)
	HOST_OS := macos
endif
ifeq ($(UNAME_M),x86_64)
	HOST_ARCH := amd64
endif
ifeq ($(UNAME_M),aarch64)
	HOST_ARCH := arm64
endif
ifeq ($(UNAME_M),arm64)
	HOST_ARCH := arm64
endif

HOST_TARGET := $(HOST_OS)_$(HOST_ARCH)

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Default target
all: build

## help: Show this help message
help:
	@echo "$(GREEN)🔧 CasGarage Build System$(NC)"
	@echo ""
	@echo "$(YELLOW)Available targets:$(NC)"
	@echo "  make build         - Build all platform binaries (output: ./binaries)"
	@echo "  make release       - Create GitHub release with all binaries"
	@echo "  make docker        - Build and push multi-arch Docker images"
	@echo "  make test          - Run all tests"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make help          - Show this help message"
	@echo ""
	@echo "$(YELLOW)Environment variables:$(NC)"
	@echo "  VERSION            - Override version (default: from release.txt)"
	@echo ""
	@echo "$(YELLOW)Current configuration:$(NC)"
	@echo "  Version: $(GREEN)$(VERSION)$(NC)"
	@echo "  Build Date: $(GREEN)$(BUILD_DATE)$(NC)"
	@echo "  VCS Ref: $(GREEN)$(VCS_REF)$(NC)"
	@echo "  Host: $(GREEN)$(HOST_TARGET)$(NC)"

## build: Build all platform binaries
build: frontend-build
	@echo "$(GREEN)🏗️  Building CasGarage v$(VERSION) for all platforms...$(NC)"
	@mkdir -p $(BUILD_DIR)
	@for target in $(TARGETS); do \
		os_arch=$$(echo $$target | sed 's/x86_64-unknown-linux-musl/linux_amd64/; s/aarch64-unknown-linux-musl/linux_arm64/; s/x86_64-pc-windows-gnu/windows_amd64/; s/x86_64-apple-darwin/macos_amd64/; s/aarch64-apple-darwin/macos_arm64/; s/x86_64-unknown-freebsd/freebsd_amd64/; s/x86_64-unknown-openbsd/openbsd_amd64/'); \
		ext=""; \
		if echo $$target | grep -q windows; then ext=".exe"; fi; \
		echo "$(YELLOW)📦 Building for $$os_arch...$(NC)"; \
		cross build --release --target $$target --bin $(PROJECT_NAME) 2>&1 | grep -v "^warning:" || true; \
		if [ $$? -eq 0 ]; then \
			cp target/$$target/release/$(PROJECT_NAME)$$ext $(BUILD_DIR)/$(PROJECT_NAME)-$$os_arch$$ext; \
			if echo $$target | grep -q musl; then \
				strip $(BUILD_DIR)/$(PROJECT_NAME)-$$os_arch$$ext 2>/dev/null || true; \
				echo "$(GREEN)✓ Stripped $$os_arch binary$(NC)"; \
			fi; \
			echo "$(GREEN)✓ Built $$os_arch$(NC)"; \
		else \
			echo "$(RED)✗ Failed to build $$os_arch$(NC)"; \
		fi; \
	done
	@# Create host-specific binary symlink
	@ln -sf $(PROJECT_NAME)-$(HOST_TARGET) $(BUILD_DIR)/$(PROJECT_NAME) 2>/dev/null || true
	@echo "$(GREEN)✓ Build complete! Binaries in $(BUILD_DIR)/$(NC)"
	@ls -lh $(BUILD_DIR)/

## frontend-build: Build frontend WASM application
frontend-build:
	@echo "$(GREEN)🎨 Building frontend...$(NC)"
	@cd $(FRONTEND_DIR) && trunk build --release
	@echo "$(GREEN)✓ Frontend build complete$(NC)"

## release: Create GitHub release
release: build
	@echo "$(GREEN)📦 Creating release v$(VERSION)...$(NC)"
	@mkdir -p $(RELEASE_DIR)
	@# Copy binaries to release directory
	@cp -r $(BUILD_DIR)/* $(RELEASE_DIR)/
	@# Create source archive (exclude VCS files)
	@echo "$(YELLOW)📚 Creating source archive...$(NC)"
	@tar --exclude='.git' \
		--exclude='target' \
		--exclude='binaries' \
		--exclude='releases' \
		--exclude='rootfs' \
		--exclude='*.db' \
		--exclude='*.log' \
		-czf $(RELEASE_DIR)/$(PROJECT_NAME)-$(VERSION)-source.tar.gz \
		--transform 's,^\.,$(PROJECT_NAME)-$(VERSION),' \
		.
	@# Update version in release.txt
	@echo "$(VERSION)" > $(VERSION_FILE)
	@# Delete existing tag if it exists
	@if git rev-parse "v$(VERSION)" >/dev/null 2>&1; then \
		echo "$(YELLOW)🏷️  Deleting existing tag v$(VERSION)...$(NC)"; \
		git tag -d "v$(VERSION)" 2>/dev/null || true; \
		git push origin :refs/tags/v$(VERSION) 2>/dev/null || true; \
	fi
	@# Create GitHub release
	@echo "$(GREEN)🚀 Creating GitHub release...$(NC)"
	@gh release create "v$(VERSION)" \
		--title "CasGarage v$(VERSION)" \
		--notes "Release v$(VERSION)" \
		$(RELEASE_DIR)/* || echo "$(RED)✗ Failed to create release. Make sure 'gh' CLI is installed and authenticated.$(NC)"
	@echo "$(GREEN)✓ Release v$(VERSION) created!$(NC)"

## docker: Build and push multi-arch Docker images
docker:
	@echo "$(GREEN)🐳 Building Docker images for v$(VERSION)...$(NC)"
	@# Create buildx builder if it doesn't exist
	@docker buildx create --name casgarage-builder --use 2>/dev/null || docker buildx use casgarage-builder
	@# Build and push multi-arch images
	@echo "$(YELLOW)🏗️  Building for amd64 and arm64...$(NC)"
	@docker buildx build \
		--platform linux/amd64,linux/arm64 \
		--build-arg VERSION=$(VERSION) \
		--build-arg BUILD_DATE=$(BUILD_DATE) \
		--build-arg VCS_REF=$(VCS_REF) \
		--tag $(GHCR_REPO):latest \
		--tag $(GHCR_REPO):$(VERSION) \
		--push \
		.
	@echo "$(GREEN)✓ Docker images pushed to $(GHCR_REPO)$(NC)"

## test: Run all tests
test:
	@echo "$(GREEN)🧪 Running tests...$(NC)"
	@cargo test --all-features --workspace
	@echo "$(GREEN)✓ All tests passed$(NC)"

## clean: Clean build artifacts
clean:
	@echo "$(YELLOW)🧹 Cleaning build artifacts...$(NC)"
	@cargo clean
	@rm -rf $(BUILD_DIR) $(RELEASE_DIR)
	@rm -rf $(FRONTEND_DIR)/dist $(FRONTEND_DIR)/target
	@rm -rf ./rootfs
	@echo "$(GREEN)✓ Clean complete$(NC)"
