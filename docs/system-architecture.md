# System Architecture

## System summary

This project has two boards:

- **Input board**: reads 8 switch inputs and sends channel state on every edge (25 ms idle keepalive).
- **Output board**: receives state and drives 8x 12V outputs, with local override inputs.

Target latency remains `<10ms`, with typical low-millisecond behavior.

## MCU platform

- Primary MCU: `STM32G0B1CBT6`
- Reasons:
  - Native USB device support for DFU programming
  - Sufficient flash/SRAM headroom

## 3.3V rail and regulator

Both v1.0.0 boards use an onboard `AP63203WU-7` (`C780769`) fixed 3.3V synchronous buck from `12V_MAIN`.

| Board | Estimated typical 3.3V load | Estimated peak 3.3V load | Regulator |
| --- | ---: | ---: | --- |
| Input board | ~55mA | ~120mA | `AP63203WU-7` (2A) |
| Output board | ~65mA | ~140mA | `AP63203WU-7` (2A) |

Assumptions used in peak estimate:

- MCU active at 64MHz plus dual RS-485 transceivers in active link state.
- Power/link LEDs on and all 8 channel LEDs on simultaneously.
- Worst-case pull-up/pulldown static current for all channels asserted.

Why `AP63203WU-7`:

- Input range (`3.8V` to `32V`) supports `12V_MAIN` with margin.
- 2A capability is far above both boards' peak demand.
- Avoids LDO thermal loss from 12V → 3.3V (rejected: AMS1117 linear path).

## BOM sourcing and substitution policy

- Prefer JLCPCB basic parts first for cost and assembly availability.
- Use extended parts only when no basic part meets the electrical/thermal requirement.
- Any substitution must preserve function, ratings, footprint compatibility, and protocol timing behavior.
- Re-verify live LCSC stock for every BOM line before manufacturing release.
- Current substitutions in use:
  - RS-485 TVS: `C521963` (`SM712`, SOT-23)
  - Power LED green 0603: `C7496818`
  - Link LED blue 0603: `C7496819`

## Communications architecture

### RS-485

- Full-duplex over two differential pairs
- 2x SP3485EN per board:
  - `U2A` dedicated TX path (DE/RE# → 3V3)
  - `U2B` dedicated RX path (DE/RE# → GND)
- One 120R termination on each receiver pair
- One SM712 per differential pair (2 per board)
- Cable: Belden 9842 or equivalent 2-pair shielded twisted pair

### Protocol

- Hotline v2 frames (3-byte fixed format)
- Input -> output: `[0xAA][channels][crc8]` on edge + 25 ms keepalive
- Output -> input: `[0x55][status][crc8]` at 10Hz
- CRC-8/MAXIM validation

## Input/override front-end architecture

Both boards use the same digital front-end style:

- 10k pull-up
- 10k series resistor
- 100nF shunt capacitor
- Schmitt-trigger inverter stage (`SN74LV14A`)

The design uses digital RC + Schmitt conditioning on all input and override channels.

## USB and debug strategy

- Native USB on MCU `PA11/PA12` (USB DM/DP)
- USB-C connector with 22R data series resistors and CC pull-downs
- SWD 10-pin header for bring-up, option-byte programming, and recovery

BOOT0 access:

- BOOT0 defaults LOW via 10k pulldown
- Dedicated BOOT0 button asserts HIGH
- NRST button used with BOOT0 for DFU entry

## Output drive architecture

- 8x IRLML6344 low-side MOSFET channels
- Per-channel PTC `1812L200/16GR` (2A hold, 16V)
- SS34 flyback diode per channel
- Output failsafe forces serial-derived channels OFF on comm watchdog timeout; local overrides still work

## Firmware architecture

- Rust `no_std` + Embassy async runtime
- Both controllers run USART1 in simultaneous TX/RX mode
- Input controller:
  - reads 8 digital channels
  - transmits state frame on edge or 25 ms keepalive
  - receives heartbeat frames continuously
- Output controller:
  - receives state frames continuously
  - applies local override logic
  - drives outputs and transmits periodic heartbeat

## Safety and integration notes

- PCB uses screw terminals for enclosure wiring
- External waterproof panel connectors (IP67+) remain enclosure-side
- SWD remains required for robust manufacturing/debug/recovery flow
- Printed board truth: `hardware/v1.0.0/` (`PIN_MAP.md`, `INPUT_BOARD.md`, `OUTPUT_BOARD.md`, `exports/`)

## Hardware v1.0.0 (2026-07-15)

Printed PCB truth (BOM / PnP / netlist): [`hardware/v1.0.0/`](../hardware/v1.0.0/).
