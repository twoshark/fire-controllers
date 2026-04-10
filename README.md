# Fire Controllers

Real-time two-board control system:

- **Input board** reads 8 switch channels.
- **Output board** drives 8x 12V low-side MOSFET channels (solenoids/relays).
- Boards communicate over **full-duplex RS-485** with <10ms end-to-end latency target.

## Current architecture

- MCU: `STM32G0B1CBT6`
- USB: native USB DFU on `PA11/PA12`
- RS-485: two SP3485EN per board (dedicated TX and RX paths)
- Inputs/overrides: 10k pull-up + 10k series + 100nF + Schmitt (`SN74LV14APWR`)
- Debug/recovery: SWD header retained + NRST + BOOT0 button flow

## Repository layout

```text
fire_controllers/
├── firmware/
│   ├── hotline-protocol/
│   ├── input-controller/
│   └── output-controller/
├── hardware/
│   ├── input-board/
│   ├── output-board/
│   ├── SCHEMATIC_GUIDE.md
│   └── PCB_LAYOUT_GUIDE.md
├── docs/
│   ├── system-architecture.md
│   ├── serial-protocol.md
│   ├── input-board-design.md
│   └── output-board-design.md
└── scripts/
```

## Build and validation

```bash
cd firmware
cargo fmt --all --check
cargo check
cargo clippy -p hotline-protocol --target aarch64-apple-darwin -- -D warnings
cargo clippy -p input-controller --target thumbv6m-none-eabi -- -D warnings
cargo clippy -p output-controller --target thumbv6m-none-eabi -- -D warnings
```

## Flashing via SWD (recommended for bring-up/recovery)

```bash
./scripts/flash-input.sh
./scripts/flash-output.sh
```

These scripts build release artifacts first, then run `probe-rs`.
Expected chip target is `STM32G0B1CBTx`.

## USB DFU programming flow

1. Hold BOOT0 button (`SW2`).
2. Press and release reset (`SW1`).
3. Release BOOT0.
4. Program over USB using `dfu-util` or STM32CubeProgrammer.

SWD remains strongly recommended for manufacturing, option-byte recovery, and debug.

## Protocol summary

- Input -> output: `[0xAA][state][CRC8]` at 1kHz
- Output -> input: `[0x55][status][CRC8]` at 10Hz
- CRC: CRC-8/MAXIM
- UART: 115200, 8N1

## Hardware/documentation references

- Schematic entry: `hardware/SCHEMATIC_GUIDE.md`
- PCB layout: `hardware/PCB_LAYOUT_GUIDE.md`
- System architecture: `docs/system-architecture.md`
- Input board details: `docs/input-board-design.md`
- Output board details: `docs/output-board-design.md`
- Serial protocol: `docs/serial-protocol.md`

## Safety

- Set LRS-200-12 input selector to **115V** for 120V mains.
- Do not trim output above **12.0V**.
- Mains wiring must be done by a qualified person.
