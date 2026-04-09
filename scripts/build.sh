#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FIRMWARE_DIR="$SCRIPT_DIR/../firmware"

echo "==> Checking Rust toolchain..."
cd "$FIRMWARE_DIR"
rustup target add thumbv6m-none-eabi 2>/dev/null || true
rustc --version
cargo --version

echo ""
echo "==> Building hotline-protocol (shared crate)..."
cargo build --release -p hotline-protocol

echo ""
echo "==> Building input-controller..."
cargo build --release -p input-controller

echo ""
echo "==> Building output-controller..."
cargo build --release -p output-controller

echo ""
echo "==> Build complete. Binaries:"
ls -lh target/thumbv6m-none-eabi/release/input-controller 2>/dev/null || true
ls -lh target/thumbv6m-none-eabi/release/output-controller 2>/dev/null || true
