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
- Outdoor power/signal links (DT, DTP, M12-5) must be IP67 when mated. Input HMI (C14, KCD4, arcade) is splash-rated with covers/boots — park under canopy; do not claim whole printed box IP67. Panel connectors wire to PCB screw terminals
- Include industry standard methods for making the board easy to debug (SWD header + USB DFU on v1.0.0; UART breakout / named test points optional on future revisions)
- Always ask clarifying questions

# Project Overview

This repo contains the design docs, firmware, and EasyEDA manufacturing exports that fully describe a system for real-time transmission of inputs from an input board, over serial, to an output board that controls 8x 12V outputs.

The primary use cases for the outputs are 12V solenoids and relays.

# Tech Stack / Repo Contents

- EasyEDA for schematics/PCB; manufacturing truth in `hardware/v1.0.0/exports/` (BOM, Pick-and-Place, PADS netlist)
- STM32G0B1 series ARM Cortex-M0+ microcontroller with native USB DFU support (64MHz, modern, good Rust support)
- Rust (embedded, no_std) using Embassy async framework for STM32 firmware
- sh or Bash for build/flash/devops scripts
- CSV for BOMs
- Markdown for specs and design docs
- Diagrams in any suitable format (Mermaid, SVG, PNG, etc.)

# Architecture

The system is comprised of 2 boards, each with their own microcontroller, connected by a serial link with a cable up to 200ft.

**Serial Link**: RS-485 differential signaling over shielded twisted pair cable (Belden 9842 or equivalent 2-pair 24AWG shielded RS-485 cable, 120Ω characteristic impedance). Both boards include dual RS-485 transceivers for full duplex. At 115200 baud, a 3-byte frame transmits in ~260μs, well within the <10ms latency budget.

**Power**: Input boxes: C14 → RS-15-12 → PCB `J1`. Output boxes: outdoor HLG-240H-12 (IP67) → DTP → PCB `J1`. Each PCB generates 3.3V via onboard `AP63203WU-7` buck.

**Enclosure Wiring**: Screw terminals on the PCB; outdoor links use DT / DTP / M12-5 (IP67 when mated). Clear silkscreen. Seal truth: `hardware/v1.0.0/enclosures/SEALING.md`.

## Input Board

### Inputs

The board takes in power and an array of input states.

- 120V AC mains power via IEC C14 inlet on enclosure → Mean Well RS-15-12 (chassis, 12V) → onboard `AP63203WU-7` buck for 3.3V MCU rail
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

- 12V DC from outdoor Mean Well HLG-240H-12 (IP67, 12V/16A) via DTP → board `J1` → onboard `AP63203WU-7` buck for 3.3V MCU rail
- RS-485 full-duplex serial link with the input controller (dedicated RX and TX pairs)
- 8x digital override inputs via pull-up + RC debounce + Schmitt-trigger frontend (switch-to-GND). Override closed forces ON, open defers to serial command.
- USB-C debug/programming port

### Outputs

The board produces 8 outputs based on the state received over serial. If an override input is active, it takes precedence over the serial state for that channel.

- 8x 12V MOSFET-driven outputs (IRLML6344TRPbF, up to 2A/ch absolute) with flyback diodes and per-channel PTC fuses (`1812L200/16GR`, 2A hold @20°C / ~1.5A continuous hot)
- Shared `+12V` load feed on `J6` (both poles); loads return through per-channel low-side switches (no external load-GND terminal)
- Board 12V input protected by ATO fuse `F9` + `Q9` (`IPB110P06LM`) P-MOS reverse-polarity circuit
- Status LEDs (surface-mounted on PCB, grouped along one edge for enclosure light-pipe/window exposure):
  - 1x serial link status (RX activity / link healthy)
  - 1x power indicator (box on)
  - 8x output state indicators (shows actual output state: serial-derived or override)

# Resolved Design Decisions

1. **Power supply**: **Input boxes**: C14 → RS-15-12 (12V chassis) → PCB `J1`. **Output boxes**: outdoor Mean Well **HLG-240H-12** (IP67, 12V/16A, 192W) → DTP → PCB `J1`. Day-1 panels sign **5 SOL** / mp **3 SOL**; PSU sized for full 8-ch board. Each PCB uses onboard `AP63203WU-7` 3.3V buck. Channel PTCs `1812L200/16GR` (16V). Truth: `hardware/v1.0.0/enclosures/POWER_OTS.md`.
2. **Override detection**: Each override input uses pull-up + RC debounce + Schmitt-trigger buffering. Firmware reads digital level directly (closed switch = override ON; open switch = serial control).
3. **Input interface**: Each channel is a simple switch-to-GND digital input with RC + Schmitt cleanup. No ADC divider path is used.
4. **Outdoor connectors**: Deutsch DT (SOL) / DTP (12V) / M12-5 (RS-485) — IP67 when mated; wired to PCB screw terminals. IEC C14 + KCD4 + arcade on inputs (splash/canopy). See `enclosures/SEALING.md`.
5. **STM32 subfamily**: STM32G0B1CBT6, Cortex-M0+, 64MHz, LQFP48 package, native USB DFU support, ample GPIO for digital inputs, outputs, LEDs, USB, UART, and SWD.
6. **Enclosure**: User designs separately. PCB mounting pads: hole **100 mil** / pad **160 mil** (**M2**). Screw terminals, logical connector grouping, comprehensive silkscreen. See `hardware/v1.0.0/enclosures/MOUNTING.md`.
7. **Status LEDs**: All LEDs are surface-mounted directly on the PCB, grouped together along one edge in a defined LED block. The enclosure exposes these through aligned holes, a clear window strip, or light pipes. This eliminates LED wiring during assembly. PCB silkscreen clearly labels each LED's function next to it.
8. **Serial protocol**: Full-duplex bidirectional **Hotline v2** protocol over RS-485 at 115200 baud. Primary direction: input→output state frames (3-byte: [0xAA] [8-bit channel state] [CRC8]) on every input edge, with 25 ms idle keepalive (~40 Hz). Reverse direction: output→input heartbeat frames (3-byte: [0x55] [status byte] [CRC8]) at 10Hz. Two transceivers per board (one TX path, one RX path) eliminate bus-turnaround control in firmware. End-to-end latency: ~1-2ms typical, <5ms worst case.
9. **RS-485 cabling**: Belden 9842 or equivalent 2-pair 24AWG shielded twisted pair, 120Ω characteristic impedance. One 120Ω termination per receiver pair. M12-5 connectors at each controller enclosure.

# Hardware v1.0.0 (authoritative)

Printed PCB documentation and EasyEDA exports live in `hardware/v1.0.0/` (`PIN_MAP.md`, `INPUT_BOARD.md`, `OUTPUT_BOARD.md`, `BRINGUP.md`, `exports/`). Prefer those over historical capture guides (removed).

# Expected Outputs

- Input controller firmware (Rust, embedded no_std) with build/flash scripts
- Output controller firmware (Rust, embedded no_std) with build/flash scripts
- EasyEDA manufacturing exports per board in `hardware/v1.0.0/exports/`:
  - BOM (CSV)
  - Pick-and-Place / CPL (CSV)
  - PADS netlist (ASC)
- Top-level README with build instructions, architecture overview, and usage guide
- Design docs covering:
  - System architecture (`docs/system-architecture.md`)
  - Serial protocol specification (`docs/serial-protocol.md`)
  - Board truth docs under `hardware/v1.0.0/`
