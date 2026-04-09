# General Advice

- Question every tactic and part used at all times
- Rate all tactics and parts based on confidence
- Reject low confidence tactics and parts and replace with high confidence ones
- Repeat reviews, ratings, rejections and replacements until all parts and tactics are extremely high confidence
- Produce working code that lints, is formatted correctly, and compiles cleanly with no unnecessary warnings
- Utilize real, confirmed, valid parts always. Do not imagine parts that do not exist and use them
- Be cost effective and keep the cost of each board under $100. Ideally under $50 if possible. Real-time requirements are higher priority than cost
- A button press on the input controller must register across serial to the output in what feels like true real time to the user (<10ms end-to-end latency target)
- Include connectors for all board inputs and outputs
- All external connectors must be waterproof (IP67 or better). Connectors are panel-mounted on the enclosure and wired to the PCB; PCB uses screw terminals for easy enclosure wiring
- Include industry standard methods for making the board easy to debug (test points, SWD headers, UART breakout, etc.)
- Always ask clarifying questions

# Project Overview

This repo contains the design docs, code, and KiCad files that fully describe a system for real-time transmission of inputs from an input board, over serial, to an output board that controls 8x 12V outputs.

The primary use cases for the outputs are 12V solenoids and relays.

# Tech Stack / Repo Contents

- KiCad for circuit schematics, PCB layout, and manufacturing files (Gerbers, drill files, BOM)
- STM32G0B1 series ARM Cortex-M0+ microcontroller with native USB DFU support (64MHz, modern, good Rust support)
- Rust (embedded, no_std) using Embassy async framework for STM32 firmware
- sh or Bash for build/flash/devops scripts
- CSV for BOMs
- Markdown for specs and design docs
- Diagrams in any suitable format (Mermaid, SVG, PNG, etc.)

# Architecture

The system is comprised of 2 boards, each with their own microcontroller, connected by a serial link with a cable up to 200ft.

**Serial Link**: RS-485 differential signaling over shielded twisted pair cable (Belden 9842 or equivalent 2-pair 24AWG shielded RS-485 cable, 120Ω characteristic impedance). Both boards include dual RS-485 transceivers for full duplex. At 115200 baud, a 3-byte frame transmits in ~260μs, well within the <10ms latency budget.

**Power**: Each box accepts 120V AC mains via an IEC C14 panel-mount inlet on the enclosure. An off-the-shelf enclosed AC-DC module inside the enclosure converts to the required DC rails (12V for outputs/input conditioning, 3.3V for MCU logic via onboard LDO).

**Enclosure Wiring**: The PCB is designed for easy enclosure assembly. All external connections use screw terminals on the PCB. Waterproof connectors (Amphenol AT series or equivalent) are panel-mounted on the enclosure and wired to the board. Clear silkscreen labeling. Mounting holes in standard patterns.

## Input Board

### Inputs

The board takes in power and an array of input states.

- 120V AC mains power via IEC C14 inlet on enclosure → off-the-shelf AC-DC module (e.g., Mean Well IRM-15-12 or similar, 12V output) → onboard LDO for 3.3V MCU rail
- 8x digital inputs via pull-up + RC debounce + Schmitt-trigger frontend (switch-to-GND wiring). Firmware reads digital GPIO states.
- USB-C debug/programming port (SWD via USB adapter or native USB bootloader)

### Outputs

- RS-485 serial link to the output controller (full-duplex bidirectional)
- Status LEDs (surface-mounted on PCB, grouped along one edge for enclosure light-pipe/window exposure):
  - 8x input state indicators (mirrors the state being transmitted over serial)
  - 1x power indicator (box on)
  - 1x serial link status (end-to-end link health via heartbeat)

## Output Board

### Inputs

- 120V AC mains power via IEC C14 inlet on enclosure → off-the-shelf AC-DC module (Mean Well LRS-200-12, 200W, 12V/17A output — required to supply 8x 2A load channels = 192W peak) → onboard LDO for 3.3V MCU rail
- RS-485 full-duplex serial link with the input controller (dedicated RX and TX pairs)
- 8x digital override inputs via pull-up + RC debounce + Schmitt-trigger frontend (switch-to-GND). Override closed forces ON, open defers to serial command.
- USB-C debug/programming port

### Outputs

The board produces 8 outputs based on the state received over serial. If an override input is active, it takes precedence over the serial state for that channel.

- 8x 12V MOSFET-driven outputs (IRLML6344TRPbF, up to 2A per channel) with flyback diodes and per-channel PTC fuses (1812L200/12GR, 2A hold, 12V rated) for overcurrent protection
- 12V GND output terminal for output device return path
- Status LEDs (surface-mounted on PCB, grouped along one edge for enclosure light-pipe/window exposure):
  - 1x serial link status (RX activity / link healthy)
  - 1x power indicator (box on)
  - 8x output state indicators (shows actual output state: serial-derived or override)

# Resolved Design Decisions

1. **Power supply**: Off-the-shelf AC-DC modules inside each enclosure convert 120V AC to 12V DC. **Input board**: Mean Well IRM-15-12 (15W, PCB-mount, ~$8) — electronics-only draw ~1W. **Output board**: Mean Well LRS-200-12 (200W, 17A, enclosed, fanless, ~$28) — must supply 8x 2A load channels (192W peak). Onboard 3.3V LDO on each PCB for MCU logic. The PCB itself never sees mains voltage — the AC-DC module is wired between the IEC inlet and the PCB's 12V screw terminal. The LRS-200-12 (output board) has a manual 115V/230V input voltage selector switch that must be set to the 115V position for 120V North American mains. **WARNING**: The output voltage trim pot on the LRS-200-12 must NOT be adjusted above 12.0V (factory default), as the per-channel PTC fuses are rated for 12V maximum. The IRM-15-12 (input board) is auto-ranging (85–264VAC) and needs no switch.
2. **Override detection**: Each override input uses pull-up + RC debounce + Schmitt-trigger buffering. Firmware reads digital level directly (closed switch = override ON; open switch = serial control).
3. **Input interface**: Each channel is a simple switch-to-GND digital input with RC + Schmitt cleanup. No ADC divider path is used.
4. **Waterproof connectors**: Amphenol AT series (automotive-grade, IP67, cost-effective) for signal connections. Panel-mounted on enclosure, wired to PCB screw terminals. IEC C14 inlet for 120V AC. M12 8-pin for full-duplex RS-485 link.
5. **STM32 subfamily**: STM32G0B1CBT6, Cortex-M0+, 64MHz, LQFP48 package, native USB DFU support, ample GPIO for digital inputs, outputs, LEDs, USB, UART, and SWD.
6. **Enclosure**: User designs separately. PCB provides M3 mounting holes in standard pattern, clearly labeled screw terminals, logical connector grouping (power, inputs, outputs, serial, debug), and comprehensive silkscreen.
7. **Status LEDs**: All LEDs are surface-mounted directly on the PCB, grouped together along one edge in a defined LED block. The enclosure exposes these through aligned holes, a clear window strip, or light pipes. This eliminates LED wiring during assembly. PCB silkscreen clearly labels each LED's function next to it.
8. **Serial protocol**: Full-duplex bidirectional **Hotline v2** protocol over RS-485 at 115200 baud. Primary direction: input→output state frames (3-byte: [0xAA] [8-bit channel state] [CRC8]) at 1kHz. Reverse direction: output→input heartbeat frames (3-byte: [0x55] [status byte] [CRC8]) at 10Hz. Two transceivers per board (one TX path, one RX path) eliminate bus-turnaround control in firmware. End-to-end latency: ~1-2ms typical, <5ms worst case.
9. **RS-485 cabling**: Belden 9842 or equivalent 2-pair 24AWG shielded twisted pair, 120Ω characteristic impedance, specifically designed for RS-485. One 120Ω termination per receiver pair. M12 8-pin connectors at each enclosure.

# Expected Outputs

- Input controller firmware (Rust, embedded no_std) with build/flash scripts
- Output controller firmware (Rust, embedded no_std) with build/flash scripts
- Complete manufacturing-ready KiCad project for each board:
  - Schematic files (.kicad_sch)
  - PCB layout files (.kicad_pcb)
  - Gerber and drill files for fabrication
  - BOM (CSV) with real part numbers and sourcing info
- Top-level README with build instructions, architecture overview, and usage guide
- Design docs and diagrams covering:
  - System architecture
  - Serial protocol specification
  - Power supply design
  - Input conditioning and output drive circuits
  - PCB layout rationale