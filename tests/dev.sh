#!/usr/bin/env bash
# Development server script

set -euo pipefail

echo "Starting CasGarage in development mode..."

# Start backend and frontend concurrently
trap 'kill 0' EXIT

# Start backend
cargo watch -x 'run -- server' &

# Start frontend
cd frontend
trunk serve &

wait
