# System Architecture

## System summary

This project has two boards:

- **Input board**: reads 8 switch inputs and sends channel state at 1kHz.
- **Output board**: receives state and drives 8x 12V outputs, with local override inputs.

Target latency remains `<10ms`, with typical low-millisecond behavior.

## MCU platform

- Primary MCU: `STM32G0B1CBT6`
- Reasons:
  - Native USB device support for DFU programming
  - Sufficient flash/SRAM headroom

## 3.3V rail budget and regulator policy

The previous `AMS1117-3.3` linear path from `12V_MAIN` causes avoidable thermal loss. For regulator sizing, the design now budgets board-level 3.3V current with explicit headroom:

| Board | Estimated typical 3.3V load | Estimated peak 3.3V load | Required regulator minimum (with margin) |
| --- | ---: | ---: | ---: |
| Input board | ~55mA | ~120mA | >=300mA continuous |
| Output board | ~65mA | ~140mA | >=350mA continuous |

Assumptions used in peak estimate:

- MCU active at 64MHz plus dual RS-485 transceivers in active link state.
- Power/link LEDs on and all 8 channel LEDs on simultaneously.
- Worst-case pull-up/pulldown static current for all channels asserted.
- Schmitt front-end dynamic current remains small versus LED and transceiver load.

Regulator selection rule:

- Preferred candidate from plan: `LMZM23601V33`.
- If preferred part is not production-available, use an in-stock fixed 3.3V buck with enough current and thermal headroom.
- Final selected part for both boards: `AP63203WU-7` (`C780769`), fixed 3.3V synchronous buck, 2A capability.

Why `AP63203WU-7` is selected:

- Live availability is high in both LCSC and JLC part channels (tens of thousands of units at time of check).
- Input range (`3.8V` to `32V`) comfortably supports `12V_MAIN` with margin.
- 2A capability is far above both boards' peak demand, giving strong thermal and transient margin.
- Cost is low enough to keep board BOM targets practical while removing LDO heat risk.

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
  - one dedicated TX path
  - one dedicated RX path
- One 120R termination on each receiver pair
- One SM712 per differential pair (2 per board)
- Cable: Belden 9842 or equivalent 2-pair shielded twisted pair

### Protocol

- Hotline v2 frames (3-byte fixed format)
- Input -> output: `[0xAA][channels][crc8]` at 1kHz
- Output -> input: `[0x55][status][crc8]` at 10Hz
- CRC-8/MAXIM validation

## Input/override front-end architecture

Both boards now use the same digital front-end style:

- 10k pull-up
- 10k series resistor
- 100nF shunt capacitor
- Schmitt-trigger inverter stage (`SN74LVC14A`)

The design uses digital RC + Schmitt conditioning on all input and override channels.

## USB and debug strategy

- Native USB on MCU `PA11/PA12` (USB DM/DP)
- USB-C connector retained with 22R data series resistors and CC pull-downs
- Native USB data interface on PA11/PA12
- SWD 10-pin header retained for bring-up, option-byte programming, and recovery

BOOT0 access:

- BOOT0 defaults LOW via 10k pulldown
- Dedicated BOOT0 button asserts HIGH
- NRST button used with BOOT0 for DFU entry

## Output drive architecture

- 8x IRLML6344 low-side MOSFET channels
- 2A PTC fuse per channel
- SS34 flyback diode per channel
- Output failsafe forces all channels OFF on comm watchdog timeout

## Firmware architecture

- Rust `no_std` + Embassy async runtime
- Both controllers run USART1 in simultaneous TX/RX mode
- Input controller:
  - reads 8 digital channels
  - transmits state frame each 1ms
  - receives heartbeat frames continuously
- Output controller:
  - receives state frames continuously
  - applies local override logic
  - drives outputs and transmits periodic heartbeat

## Safety and integration notes

- PCB uses screw terminals for enclosure wiring
- External waterproof panel connectors (IP67+) remain enclosure-side
- SWD remains required for robust manufacturing/debug/recovery flow
- Detailed schematic capture sequence: `hardware/SCHEMATIC_GUIDE.md`
- Detailed PCB placement/routing sequence: `hardware/PCB_LAYOUT_GUIDE.md`
- Input schematic pin/net appendix: `hardware/SCHEMATIC_APPENDIX_INPUT.md`
- Output schematic pin/net appendix: `hardware/SCHEMATIC_APPENDIX_OUTPUT.md`
- Input PCB execution appendix: `hardware/PCB_APPENDIX_INPUT.md`
- Output PCB execution appendix: `hardware/PCB_APPENDIX_OUTPUT.md`
