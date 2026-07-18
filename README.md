# Fire Controllers

Real-time two-board control system:

- **Input board** reads 8 switch channels.
- **Output board** drives 8× 12V low-side MOSFET channels (solenoids/relays).
- Boards communicate over **full-duplex RS-485** with <10ms end-to-end latency target.

## Hardware v1.0.0 (2026-07-15 EasyEDA)

Printed-board truth lives in [`hardware/v1.0.0/`](hardware/v1.0.0/):

| Doc | Role |
| --- | --- |
| [`hardware/v1.0.0/README.md`](hardware/v1.0.0/README.md) | Index + export list |
| [`hardware/v1.0.0/PIN_MAP.md`](hardware/v1.0.0/PIN_MAP.md) | MCU pin ↔ channel map |
| [`hardware/v1.0.0/INPUT_BOARD.md`](hardware/v1.0.0/INPUT_BOARD.md) | Input board |
| [`hardware/v1.0.0/OUTPUT_BOARD.md`](hardware/v1.0.0/OUTPUT_BOARD.md) | Output board |
| [`hardware/v1.0.0/BRINGUP.md`](hardware/v1.0.0/BRINGUP.md) | Power-on / flash / link test |
| [`hardware/v1.0.0/exports/`](hardware/v1.0.0/exports/) | BOM, PnP, netlist |

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
├── hardware/v1.0.0/     # Printed board truth + EasyEDA exports
├── docs/                  # Protocol + system architecture
└── scripts/               # build / flash / test
```

## Quick start

```bash
./scripts/test.sh
./scripts/flash-input.sh
./scripts/flash-output.sh
```

Full checklist: [`hardware/v1.0.0/BRINGUP.md`](hardware/v1.0.0/BRINGUP.md)

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

- Input→output: `[0xAA][state][CRC8]` on every input edge, plus 25 ms idle keepalive (~40 Hz when quiet)
- Output→input: `[0x55][status][CRC8]` @ 10Hz
- CRC-8/MAXIM, UART 115200 8N1
- Spec: [`docs/serial-protocol.md`](docs/serial-protocol.md)

## Safety

- Output 12 V from **HLG-240H-12** (IP67); earth FG; see `hardware/v1.0.0/enclosures/POWER_OTS.md`.
- Insert **20 A ATO** in output `F9` before load tests.
- Loads: `J6` (+12V) → load → `J5x` (never return to board GND).
