#!/usr/bin/env bash
# Test script

set -euo pipefail

echo "Running tests..."

# Run unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'

# Run clippy
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check

echo "All tests passed!"
