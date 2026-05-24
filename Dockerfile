# Multi-stage Alpine-based Dockerfile for CasGarage
# Builds both Rust backend and Leptos frontend, creates single static binary

# Read version from release.txt
ARG VERSION=0.1.0

# Stage 1: Build frontend (WASM)
FROM rust:1.83-alpine AS frontend-builder

WORKDIR /build

# Install frontend build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    && cargo install trunk \
    && rustup target add wasm32-unknown-unknown

# Copy workspace Cargo.toml
COPY Cargo.toml ./

# Copy frontend files
COPY frontend/Cargo.toml ./frontend/
COPY frontend/src ./frontend/src
COPY frontend/style ./frontend/style
COPY frontend/index.html ./frontend/
COPY frontend/Trunk.toml ./frontend/

WORKDIR /build/frontend

# Build frontend (WASM + assets)
RUN trunk build --release

# Stage 2: Build backend
FROM rust:1.83-alpine AS backend-builder

ARG VERSION

WORKDIR /build

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Copy built frontend assets to embed
COPY --from=frontend-builder /build/frontend/dist ./frontend/dist

# Build backend in release mode (static binary)
RUN cargo build --release --bin casgarage --target x86_64-unknown-linux-musl

# Strip binary to reduce size
RUN strip /build/target/x86_64-unknown-linux-musl/release/casgarage

# Stage 3: Runtime (Alpine-based)
FROM alpine:3.19

ARG VERSION
ARG BUILD_DATE
ARG VCS_REF

# OCI Labels
LABEL org.opencontainers.image.title="CasGarage" \
      org.opencontainers.image.description="Self-hosted S3-compatible object storage with administrative web UI" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.authors="Jason Hempstead (casjay) <casjay@yahoo.com>" \
      org.opencontainers.image.url="https://casgarage.readthedocs.io" \
      org.opencontainers.image.documentation="https://casgarage.readthedocs.io" \
      org.opencontainers.image.source="https://github.com/casapps/casgarage" \
      org.opencontainers.image.revision="${VCS_REF}" \
      org.opencontainers.image.vendor="CasApps" \
      org.opencontainers.image.licenses="MIT" \
      maintainer="Jason Hempstead (casjay) <casjay@yahoo.com>"

# Install runtime dependencies (curl, bash, ca-certificates)
RUN apk add --no-cache \
    ca-certificates \
    bash \
    curl \
    tzdata \
    && update-ca-certificates

# Create directories
RUN mkdir -p \
    /data/db \
    /data/blocks \
    /config/ssl/certs \
    /var/log/casgarage

# Find unused UID/GID between 100-999 and create system user
RUN for uid in $(seq 100 999); do \
        if ! getent passwd $uid >/dev/null 2>&1 && ! getent group $uid >/dev/null 2>&1; then \
            addgroup -g $uid -S casgarage && \
            adduser -u $uid -S -G casgarage -h /data -s /sbin/nologin casgarage && \
            break; \
        fi; \
    done \
    && chown -R casgarage:casgarage /data /config /var/log/casgarage

# Copy binary to /usr/local/bin
COPY --from=backend-builder /build/target/x86_64-unknown-linux-musl/release/casgarage /usr/local/bin/casgarage

# Ensure binary is executable
RUN chmod +x /usr/local/bin/casgarage

USER casgarage

# Expose ports (internal port 80 for HTTP)
EXPOSE 80

# Set environment variables
ENV DATA_DIR=/data \
    CONFIG_DIR=/config \
    LOG_DIR=/var/log/casgarage \
    RUST_LOG=info \
    TMPDIR=/tmp/casgarage

# Create tmp directory
RUN mkdir -p /tmp/casgarage

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/casgarage", "--status"]

# Set working directory
WORKDIR /data

ENTRYPOINT ["/usr/local/bin/casgarage"]
CMD ["--port", "80"]
