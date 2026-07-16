#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FIRMWARE_DIR="$SCRIPT_DIR/../firmware"
CHIP="${CHIP:-STM32G0B1CBTx}"

echo "==> Flashing output-controller via SWD ($CHIP)"
echo "    Connect probe to J8 (2x5 SWD). Power board (3V3 up)."
echo "    Pin map: hardware/v1.0.0/PIN_MAP.md   Bring-up: hardware/v1.0.0/BRINGUP.md"
echo ""

if ! command -v probe-rs >/dev/null 2>&1; then
  echo "ERROR: probe-rs not found. Install: cargo install probe-rs-tools --locked" >&2
  exit 1
fi

cd "$FIRMWARE_DIR"
cargo build --release -p output-controller
probe-rs run --chip "$CHIP" target/thumbv6m-none-eabi/release/output-controller
