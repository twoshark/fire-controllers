#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FIRMWARE_DIR="$SCRIPT_DIR/../firmware"

cd "$FIRMWARE_DIR"

echo "==> Running hotline-protocol unit tests (host target)..."
cargo test -p hotline-protocol --target "$(rustc -vV | grep '^host:' | cut -d' ' -f2)"

echo ""
echo "==> Cross-compile check (thumbv6m-none-eabi)..."
cargo build --release -p input-controller -p output-controller

echo ""
echo "==> All checks passed."
