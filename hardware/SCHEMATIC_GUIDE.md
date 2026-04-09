# Schematic Entry Guide

This guide defines the current hardware design:

- STM32G0B1 MCU (native USB DFU capable)
- Full-duplex RS-485 (2x SP3485EN per board)
- Digital input front-end using RC + Schmitt-trigger buffer
- SWD retained, plus BOOT0 button for DFU mode

## BOM convention

- BOMs are normalized to one line per unique part number/MPN.
- Repeated use of the same part is represented by combined designators and summed quantity.

## MCU and core nets

### MCU

- Part: `STM32G0B1CBT6`
- Package: LQFP-48
- Keep SWD header on both boards (`PA13`, `PA14-BOOT0`, `NRST`, `VTref`, `GND`)
- USB data pins:
  - `PA11` -> `USB_DM`
  - `PA12` -> `USB_DP`

### BOOT0 / reset network

- Name this shared net `PA14-BOOT0` in schematics and silkscreen.
- `PA14-BOOT0` is shared with SWD clock (`PA14/SWCLK`) during debug/programming.
- `PA14-BOOT0` has a **10k pulldown to GND** (normal boot default).
- `SW2` connects `PA14-BOOT0` to `3V3` when pressed.
- `SW1` is NRST-to-GND reset button.
- DFU entry: hold `SW2`, tap/reset with `SW1`, then release.
- Keep `SW2` as a momentary button only; do not leave BOOT0 asserted while attaching SWD.

## Input board

### Power

- J1(12V) -> D1 (SS34) -> F1 (PTC) -> 12V rail.
- 12V rail: `C17` bulk + `C6` HF cap.
- `AMS1117-3.3` creates 3V3 with `C18` + `C19` output caps.

### Input front-end (x8)

For each channel:

```text
3V3 -> R_pullup(10k) -> switch_node -> R_series(10k) -> rc_node -> Schmitt input
                          |                                 |
                        switch                           C(100nF)
                          |                                 |
                         GND                               GND
```

- `R_pullup`: 10k (default HIGH)
- `R_series`: 10k
- `C`: 100nF (tau ~= 1ms)
- Buffer ICs: `U5/U6 = SN74LVC14APWR` (Schmitt inverter)
- Firmware interprets buffered HIGH as switch-closed/active.

Connector mapping:

- `J2a` pins 1..4 -> CH0..CH3
- `J2b` pins 1..4 -> CH4..CH7
- `J3` -> input COM/GND

Channel-to-GPIO mapping (CH index is protocol bit index):

| Channel | Input board sense GPIO | Output board override GPIO | Output board MOSFET gate GPIO |
| --- | --- | --- | --- |
| CH0 | `PA0` | `PA0` | `PB2` |
| CH1 | `PA1` | `PA1` | `PB10` |
| CH2 | `PA4` | `PA4` | `PB11` |
| CH3 | `PA5` | `PA5` | `PB12` |
| CH4 | `PA6` | `PA6` | `PB13` |
| CH5 | `PA7` | `PA7` | `PA8` |
| CH6 | `PB0` | `PB0` | `PA15` |
| CH7 | `PB1` | `PB1` | `PB3` |

### Full-duplex RS-485

Two transceivers on the input board:

- `U2A` TX path:
  - `DI <- PA9 (USART1_TX)`
  - `DE = 3V3`, `RE = 3V3` (always transmit)
  - Bus pair -> `RS485_TX+ / RS485_TX-`
- `U2B` RX path:
  - `RO -> PA10 (USART1_RX)`
  - `DE = GND`, `RE = GND` (always receive)
  - Bus pair -> `RS485_RX+ / RS485_RX-`
  - `R27 = 120R` across RX pair (populated)

Protection:

- One `SM712` per differential pair (2 total).

### USB-C (native USB)

- `PA11/PA12` route to USB D-/D+ through 22R series resistors.
- Keep CC pull-downs (`5.1k` on CC1/CC2).
- Keep SWD header.

## Output board

### Power/output stage

- Power chain keeps the same 12V->3V3 logic-rail approach as input board.
- Output board does **not** include a single high-current board-level input fuse in this BOM.
- Use an enclosure-level inline fuse on the PSU-to-board feed sized for system current.
- Keep output MOSFET section unchanged except control source.
- CH5 gate is moved to `PA8` (frees `PA11` for USB).

### Override front-end (x8)

Same RC + Schmitt approach as input board:

- switch-to-GND override wiring
- 10k pull-up + 10k series + 100nF
- `U5/U6 = SN74LVC14APWR`
- Buffered HIGH means local override active (force ON).
- Buffered LOW means serial controls channel.

### Full-duplex RS-485

Two transceivers on the output board:

- `U2A` RX path:
  - `RO -> PA10 (USART1_RX)`
  - `DE = GND`, `RE = GND`
  - `R52 = 120R` across RX pair (populated)
- `U2B` TX path:
  - `DI <- PA9 (USART1_TX)`
  - `DE = 3V3`, `RE = 3V3`

Add one `SM712` per pair (2 total).

### USB-C / debug

- Native USB on `PA11/PA12` with 22R series resistors.
- Keep SWD header and NRST button.
- Add BOOT0 button + pulldown network identical to input board.

## RS-485 cable and terminal mapping

Use two differential pairs plus GND and shield:

- `TX+`, `TX-`, `RX+`, `RX-`, `GND`, `SHIELD`
- Recommended cable: Belden 9842 (2-pair shielded twisted pair)
- PCB terminal: 6-pos screw terminal (`J4` input board, `J2` output board)
- Required crossover for full-duplex point-to-point wiring:
  - Input `TX+` -> Output `RX+`
  - Input `TX-` -> Output `RX-`
  - Input `RX+` <- Output `TX+`
  - Input `RX-` <- Output `TX-`
- Keep pair polarity consistent end-to-end (`+` to `+`, `-` to `-`); never swap polarity within a pair.

## Layout checklist

- Route USB D+/D- as a short differential pair.
- Keep each RS-485 pair tightly coupled and short to transceiver.
- Place 100nF decoupling directly at each IC VCC pin.
- Keep SWD and BOOT0/NRST controls accessible near board edge.
- Wide copper for output current paths on output board.
- Keep LEDs in a single edge-aligned block for enclosure light-pipe/window use.

