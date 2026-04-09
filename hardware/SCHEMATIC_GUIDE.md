# Schematic Entry Guide

This is the assembly-executable schematic workflow for the current architecture:

- `STM32G0B1CBT6` on both boards
- Full-duplex RS-485 (2x transceivers per board)
- Native USB on `PA11/PA12`
- RC + Schmitt digital conditioning for all switch inputs
- SWD + `PA14-BOOT0` + NRST recovery path

Use this file as the build sequence, then use the appendices for full pin/net/junction tables.

## BOM convention

- BOM is one line per unique MPN.
- Combined designators must match quantity.
- Substitutions must preserve function, package fit, and net behavior.

## Required supporting appendices

- Input schematic appendix: `hardware/SCHEMATIC_APPENDIX_INPUT.md`
- Output schematic appendix: `hardware/SCHEMATIC_APPENDIX_OUTPUT.md`

Do not leave any net undefined in the top-level sheet set. If a net is not explicitly listed there, add it before release.

## 1) Shared schematic rules

### MCU and reset/debug

- MCU: `STM32G0B1CBT6`, LQFP-48.
- USB nets:
  - `PA11` -> `USB_DM`
  - `PA12` -> `USB_DP`
- SWD nets:
  - `PA13` -> `SWDIO`
  - `PA14-BOOT0` -> `SWCLK` / BOOT0 button net
  - `NRST` -> reset button + SWD header
- `PA14-BOOT0` is always named exactly `PA14-BOOT0`.

### BOOT0 and NRST behavior

- `PA14-BOOT0` has 10k pulldown to GND.
- `SW2` drives `PA14-BOOT0` HIGH only while pressed.
- `SW1` pulls `NRST` LOW.
- DFU entry sequence: hold `SW2`, tap `SW1`, release.
- Never latch BOOT0 HIGH during SWD attach.

### RS-485 cable mapping (mandatory)

- Signals: `TX+`, `TX-`, `RX+`, `RX-`, `GND`, `SHIELD`.
- Cable: Belden 9842 (or equivalent dual twisted pair + shield).
- Point-to-point crossover:
  - Input `TX+/-` -> Output `RX+/-`
  - Output `TX+/-` -> Input `RX+/-`
- Keep in-pair polarity (`+` to `+`, `-` to `-`).

### 3.3V regulator selection rule (both boards)

- Do not use `AMS1117-3.3` from `12V_MAIN` in this design revision.
- Preferred candidate from migration plan is `LMZM23601V33` only if assembly stock is acceptable at release time.
- Current selected regulator is `AP63203WU-7` (`C780769`) for both boards due to strong stock and sufficient electrical margin.
- Minimum validated load assumptions:
  - Input board peak `3V3` load budget: `120mA` (require >=`300mA` regulator capacity).
  - Output board peak `3V3` load budget: `140mA` (require >=`350mA` regulator capacity).
- Any future substitute must remain a buck regulator and preserve or exceed these margins.
- Required buck support network (minimum):
  - `U4` = `AP63203WU-7` (or validated equivalent)
  - `L1` power inductor from `U4.SW` to `3V3`
  - input bypass on `U4.VIN` (`12V_MAIN` to `GND`) using local HF ceramic + nearby bulk
  - bootstrap capacitor from `U4.BST` to `U4.SW`
  - output bulk capacitor on `3V3` near `L1`/`U4`

## 2) Input-board schematic capture sequence

1. Place connectors (`J1`, `J2a`, `J2b`, `J3`, `J4`, `J5`, `J6`) and assign pin numbers per appendix.
2. Draw power path: `J1(12V)` -> `D1(SS34)` -> `F1(PTC)` -> `12V_MAIN`, then buck stage:
   - `12V_MAIN` -> `U4.VIN`
   - `U4.SW` -> `L1` -> `3V3`
   - `U4.BST` capacitor to `SW`
   - local input/output capacitors tied to low-impedance `GND`
3. Add USB-C port and SWD header nets.
4. Add dual SP3485 transceivers:
   - `U2A`: TX-only path from `PA9`
   - `U2B`: RX-only path to `PA10` + 120R termination on RX pair
5. Add 2x SM712 (one per pair) close to connector net entry.
6. Add 8 input RC + Schmitt cells and map CH0..CH7 to MCU GPIOs.
7. Add 8 channel LEDs + power LED + link LED.
8. Run ERC and verify against input appendix tables.

All endpoint mappings are in `hardware/SCHEMATIC_APPENDIX_INPUT.md`.

## 3) Output-board schematic capture sequence

1. Place connectors (`J1`, `J2`, `J3a`, `J3b`, `J4`, `J5a`, `J5b`, `J6`, `J7`, `J8`) and assign pin numbers per appendix.
2. Draw `12V_MAIN` input and buck-based `3V3` generation path:
   - `12V_MAIN` -> `U4.VIN`
   - `U4.SW` -> `L1` -> `3V3`
   - `U4.BST` capacitor to `SW`
   - local input/output capacitors tied to low-impedance `GND`
3. Add dual SP3485 transceivers:
   - `U2A`: RX-only from cable to `PA10`
   - `U2B`: TX-only heartbeat from `PA9`
4. Add RX-pair 120R termination and two SM712 protection devices.
5. Add 8 override RC + Schmitt cells into CH0..CH7 override GPIOs.
6. Add 8 MOSFET output channels (gate resistor, pulldown, PTC, flyback).
7. Add 8 output LEDs + power LED + link LED.
8. Run ERC and verify against output appendix tables.

All endpoint mappings are in `hardware/SCHEMATIC_APPENDIX_OUTPUT.md`.

## 4) Schematic release gate (must pass)

- Every connector pin appears in a mapping table.
- Every CH0..CH7 signal has a full chain from connector -> frontend -> MCU -> protocol bit -> output gate/load.
- `PA9/PA10` used only for USART1 serial link.
- `PA11/PA12` reserved for USB only.
- `PA14-BOOT0` naming and behavior are consistent across both boards.
- RS-485 TX and RX pairs are separated and correctly crossed across the cable.
- SWD pinout, NRST, and BOOT0 controls are accessible and labeled.

## 5) Layout handoff

After schematic closure, implement layout using:

- `hardware/PCB_LAYOUT_GUIDE.md`
- `hardware/PCB_APPENDIX_INPUT.md`
- `hardware/PCB_APPENDIX_OUTPUT.md`

