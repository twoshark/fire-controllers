#!/usr/bin/env bash
set -euo pipefail

CHIP="${CHIP:-STM32G0B1CBTx}"

echo "==> Attaching probe-rs RTT (no flash) to $CHIP"
echo "    Ctrl-C to detach"
echo ""

if ! command -v probe-rs >/dev/null 2>&1; then
  echo "ERROR: probe-rs not found. Install: cargo install probe-rs-tools --locked" >&2
  exit 1
fi

probe-rs attach --chip "$CHIP"
