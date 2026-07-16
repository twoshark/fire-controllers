# Fire Controllers

Real-time two-board control system:

- **Input board** reads 8 switch channels.
- **Output board** drives 8× 12V low-side MOSFET channels (solenoids/relays).
- Boards communicate over **full-duplex RS-485** with <10ms end-to-end latency target.

## As-built (2026-07-15 EasyEDA)

Printed-board truth lives in [`hardware/as-built/`](hardware/as-built/):

| Doc | Role |
| --- | --- |
| [`hardware/as-built/README.md`](hardware/as-built/README.md) | Index + export list |
| [`hardware/as-built/PIN_MAP.md`](hardware/as-built/PIN_MAP.md) | MCU pin ↔ channel map |
| [`hardware/as-built/INPUT_BOARD.md`](hardware/as-built/INPUT_BOARD.md) | Input board |
| [`hardware/as-built/OUTPUT_BOARD.md`](hardware/as-built/OUTPUT_BOARD.md) | Output board |
| [`hardware/as-built/BRINGUP.md`](hardware/as-built/BRINGUP.md) | Power-on / flash / link test |
| [`hardware/as-built/exports/`](hardware/as-built/exports/) | BOM, PnP, netlist |

## Architecture snapshot

- MCU: `STM32G0B1CBT6`
- 3.3V: `AP63203WU-7` buck
- RS-485: dual `SP3485EN` (`U2A`=TX, `U2B`=RX on both boards)
- Output reverse-polarity: `Q9` `IPB110P06LM` + ATO `F9`
- Channel PTC: `1812L200/16GR`

## Repo layout

```text
fire-controllers/
├── firmware/              # Embassy Rust firmware
├── hardware/as-built/     # Printed board truth + EasyEDA exports
├── docs/                  # Protocol + system architecture
└── scripts/               # build / flash / test
```

## Quick start

```bash
./scripts/test.sh
./scripts/flash-input.sh
./scripts/flash-output.sh
```

Full checklist: [`hardware/as-built/BRINGUP.md`](hardware/as-built/BRINGUP.md)

## Build

```bash
./scripts/build.sh
./scripts/test.sh
```

## Flashing

```bash
./scripts/flash-input.sh      # SWD
./scripts/flash-output.sh
./scripts/monitor.sh          # RTT only
./scripts/dfu-flash-input.sh  # USB DFU fallback
./scripts/dfu-flash-output.sh
```

Chip: `STM32G0B1CBTx`. DFU: hold `SW2` (BOOT0), tap `SW1` (NRST), release `SW2`.

## Protocol

- Input→output: `[0xAA][state][CRC8]`
- Output→input: `[0x55][status][CRC8]` @ 10Hz
- CRC-8/MAXIM, UART 115200 8N1
- Spec: [`docs/serial-protocol.md`](docs/serial-protocol.md)

## Safety

- LRS-200-12 selector **115V**; trim **12.0V**.
- Insert **25A ATO** in output `F9` before load tests.
- Loads: `J6` (+12V) → load → `J5x` (never return to board GND).
