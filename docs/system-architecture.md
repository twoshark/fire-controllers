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

## BOM sourcing and substitution policy

- Prefer JLCPCB basic parts first for cost and assembly availability.
- Use extended parts only when no basic part meets the electrical/thermal requirement.
- Any substitution must preserve function, ratings, footprint compatibility, and protocol timing behavior.
- Re-verify live LCSC stock for every BOM line before manufacturing release.

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
