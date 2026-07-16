#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FIRMWARE_DIR="$SCRIPT_DIR/../firmware"
ELF="$FIRMWARE_DIR/target/thumbv6m-none-eabi/release/input-controller"
BIN="$ELF.bin"

echo "==> Building input-controller (release)..."
cd "$FIRMWARE_DIR"
cargo build --release -p input-controller

echo "==> Converting ELF -> BIN..."
if command -v rust-objcopy >/dev/null 2>&1; then
  rust-objcopy -O binary "$ELF" "$BIN"
elif command -v llvm-objcopy >/dev/null 2>&1; then
  llvm-objcopy -O binary "$ELF" "$BIN"
elif command -v arm-none-eabi-objcopy >/dev/null 2>&1; then
  arm-none-eabi-objcopy -O binary "$ELF" "$BIN"
elif command -v probe-rs >/dev/null 2>&1; then
  # probe-rs 0.24+ can download ELF directly in DFU mode via:
  echo "NOTE: using probe-rs download path instead of dfu-util"
  echo "Put board in DFU (hold SW2, tap SW1, release SW2), then this may still need SWD."
  echo "Prefer: cargo install cargo-binutils && rustup component add llvm-tools"
  exit 1
else
  echo "ERROR: need rust-objcopy (cargo install cargo-binutils && rustup component add llvm-tools)" >&2
  exit 1
fi

echo ""
echo "==> DFU flash input-controller"
echo "    Connect USB-C to J5."
echo "    1) Hold SW2 (BOOT0)"
echo "    2) Tap SW1 (NRST), then release SW2"
echo "    3) Board enumerates as STM32 BOOTLOADER"
echo ""

if ! command -v dfu-util >/dev/null 2>&1; then
  echo "ERROR: dfu-util not found. Install: sudo apt install dfu-util" >&2
  exit 1
fi

dfu-util -l || true
dfu-util -a 0 -s 0x08000000:leave -D "$BIN"
echo "==> DFU flash complete. Power-cycle or reset (SW1) if needed."
