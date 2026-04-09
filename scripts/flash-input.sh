#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FIRMWARE_DIR="$SCRIPT_DIR/../firmware"

echo "==> Flashing input-controller via SWD..."
echo "    (Ensure ST-Link / J-Link / CMSIS-DAP probe is connected to the input board)"
echo ""

cd "$FIRMWARE_DIR"
cargo build --release -p input-controller
probe-rs run --chip STM32G0B1CBTx target/thumbv6m-none-eabi/release/input-controller
