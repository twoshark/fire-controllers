# Schematic Entry Guide

This is the assembly-executable schematic workflow for the current architecture:

- `STM32G0B1CBT6` on both boards
- Full-duplex RS-485 (`U2A` + `U2B` transceivers on each board)
- Native USB on `PA11/PA12` via `J5` (input) / `J7` (output)
- RC + Schmitt digital conditioning for all switch inputs (`U5`/`U6`)
- SWD + `PA14-BOOT0` + NRST recovery path (`SW1`, `SW2`, `J6`/`J8`)

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
  - `PA14-BOOT0` -> `SWCLK` / BOOT0 button net (`SW2`)
  - `NRST` -> reset button (`SW1`) + SWD header (`J6` on input, `J8` on output)
- `PA14-BOOT0` is always named exactly `PA14-BOOT0`.

### BOOT0 and NRST behavior

- `PA14-BOOT0` has 10k pulldown to GND.
- `SW2` drives `PA14-BOOT0` HIGH only while pressed.
- `SW1` pulls `NRST` LOW.
- DFU entry sequence: hold `SW2`, tap `SW1`, release.
- Never latch BOOT0 HIGH during SWD attach.

### Schmitt inverter convention (`U5`, `U6`)

- Device: `SN74LV14APWR` (hex inverting Schmitt trigger).
- Gate naming rule is explicit: `A` = input, `Y` = output.
- Wire rule for all channels:
  - RC node -> `A` pin
  - `_SENSE` net -> matching `Y` pin
- Pin map (TSSOP-14):
  - `1A` pin 1, `1Y` pin 2
  - `2A` pin 3, `2Y` pin 4
  - `3A` pin 5, `3Y` pin 6
  - `GND` pin 7
  - `4Y` pin 8, `4A` pin 9
  - `5Y` pin 10, `5A` pin 11
  - `6Y` pin 12, `6A` pin 13
  - `VCC` pin 14

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
  - input bypass on `U4.VIN` (`12V_MAIN` to `GND`):
    - input board: `C6` (HF) with `C17` (bulk)
    - output board: `C17` + `C18` (bulk), with `C29` (100nF) near `U4.VIN`
  - bootstrap capacitor from `U4.BST` to `U4.SW`:
    - input board: `C19`
    - output board: `C20`
  - output bulk capacitor on `3V3` near `L1`/`U4`:
    - input board: `C18`
    - output board: `C19`

## 2) Input-board schematic capture sequence

1. Place connectors (`J1`, `J2a`, `J2b`, `J3`, `J4`, `J5`, `J6`) and assign pin numbers per appendix.
2. Draw power path: `J1(12V)` -> `D1(SS34)` -> `F1(PTC)` -> `12V_MAIN`, then buck stage:
   - `12V_MAIN` -> `U4.VIN`
   - `U4.SW` -> `L1` -> `3V3`
   - `U4.BST` -> `C19` -> `U4.SW`
   - `C6` + `C17` on input side and `C18` on `3V3`, all to low-impedance `GND`
3. Add USB-C debug nets on `J5` with `R31`/`R32` (D+/D- series) and `R29`/`R30` (CC pull-down), then wire SWD header `J6`.
4. Add dual SP3485 transceivers:
   - `U2A`: TX-only path from `PA9`
   - `U2B`: RX-only path to `PA10` + `R27` (120R) termination on RX pair
5. Add `D10` and `D11` (`SM712`, one per RS-485 pair) close to `J4` net entry.
6. Add 8 input RC + Schmitt cells (`R1-R16`, `C20-C27`, `U5/U6`) and map CH0..CH7 to MCU GPIOs.
7. Add status LEDs (`LED1-LED10`) with current-limit resistors (`R17-R28`).
8. Run ERC and verify against input appendix tables.

All endpoint mappings are in `hardware/SCHEMATIC_APPENDIX_INPUT.md`.

## 3) Output-board schematic capture sequence

1. Place connectors (`J1`, `J2`, `J3a`, `J3b`, `J4`, `J5a`, `J5b`, `J6`, `J7`, `J8`) and assign pin numbers per appendix.
2. Draw input protection and buck-based `3V3` generation path:
   - `J1.1` -> `VIN_12V_IN` -> `D1` (`SS34`) -> `12V_MAIN`
   - `12V_MAIN` -> `U4.VIN`
   - `U4.SW` -> `L1` -> `3V3`
   - `U4.BST` -> `C20` -> `U4.SW`
   - `C17` + `C18` on input side and `C19` on `3V3`, all to low-impedance `GND`
3. Add dual SP3485 transceivers:
   - `U2A`: RX-only from cable to `PA10`
   - `U2B`: TX-only heartbeat from `PA9`
4. Add RX-pair termination `R52` (120R) and two RS-485 TVS devices (`D18`, `D19`).
5. Add 8 override RC + Schmitt cells (`R17-R32`, `C21-C28`, `U5/U6`) into CH0..CH7 override GPIOs.
6. Add 8 MOSFET output channels (`Q1-Q8`, `R1-R8` gate resistors, `R9-R16` pulldowns, `F1-F8`, `D2-D9` flyback).
7. Add status LEDs (`LED1-LED10`) with current-limit resistors (`R41-R51`).
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

## 5) EasyEDA page/sheet structure (restored format)

Use this page map in EasyEDA (or KiCad hierarchical sheets) so entry is deterministic and reviewable. Keep the sheet names and intent aligned across both boards.

### Input board page map

#### Page 1 - Power entry and 3V3 buck

- Place: `J1`, `D1`, `F1`, `C17`, `C6`, `U4`, `L1`, `C18`, `C19`.
- Wiring matrix:
  - `J1.1` -> `VIN_12V_IN`; `J1.2` -> `GND`
  - `D1`: anode -> `VIN_12V_IN`; cathode -> `F1` input node
  - `F1`: input -> `D1` cathode node; output -> `12V_MAIN`
  - `C17`: `+` -> `12V_MAIN`; `-` -> `GND`
  - `C6`: one side -> `12V_MAIN`; other side -> `GND`
  - `U4` (`AP63203WU-7`): `VIN` -> `12V_MAIN`; `GND` -> `GND`; `SW` -> `L1` + `C19`; `BST` -> `C19`
  - `C19` (bootstrap): one side -> `U4.BST`; other side -> `U4.SW`
  - `L1`: one side -> `U4.SW`; other side -> `3V3`
  - `C18`: `+` -> `3V3`; `-` -> `GND`
- Verify:
  - `12V_MAIN` exists only after `D1` + `F1`.
  - `C19` is only on `BST`/`SW` loop (not tied to `3V3` directly).
  - `3V3` does not bypass `L1`.

#### Page 2 - MCU core + reset/debug

- Place: `U1`, `C1`, `C15`, `SW1`, `SW2`, `R33`, `J6`.
- Wiring matrix:
  - `U1` power pins: `VDD/VDDA/VBAT` -> `3V3`; `VSS/VSSA` -> `GND`
  - `U1.NRST` net -> `SW1` + `J6.10`
  - `U1.PA14-BOOT0` net -> `SW2` + `R33` + `J6.4`
  - `U1.PA13` -> `SWDIO` -> `J6.2`
  - `U1.PA11` -> `USB_DM`; `U1.PA12` -> `USB_DP`
  - `C1`: one side -> `3V3`; other side -> `GND`
  - `C15`: one side -> `3V3` (VDDA/VREF domain); other side -> `GND`
  - `R33` (10k): one side -> `PA14-BOOT0`; other side -> `GND`
  - `SW1` (NRST): one side -> `NRST`; other side -> `GND`
  - `SW2` (BOOT0): one side -> `PA14-BOOT0`; other side -> `3V3`
  - `J6`: pins wired per appendix (`1:VTREF_3V3`, `2:SWDIO`, `3/5/9:GND`, `4:PA14-BOOT0`, `10:NRST`)
- Verify:
  - `R33` (10k) pulls `PA14-BOOT0` LOW by default.
  - `SW1` only asserts `NRST` LOW when pressed.
  - `SW2` only drives BOOT0 path while pressed (no latch path).
  - `J6` pin mapping matches appendix exactly.

#### Page 3 - Input conditioning channels (CH0..CH7)

- Place: `J2a`, `J2b`, `J3`, `R1-R16`, channel RC capacitors `C20-C27`, `U5`, `U6`.
- Net-label convention (important):
  - Keep channel frontend wiring local on page 3 (terminal -> RC -> Schmitt input) without extra net labels unless needed for readability.
  - Use only `IN_CHn_SENSE` as the cross-sheet net label from Schmitt output to MCU input on page 2.
- Wiring matrix (repeat for n=0..7):
  - Connector: `J2a` (CH0..CH3) / `J2b` (CH4..CH7) channel pin -> local channel node
  - Pull-up resistor (from `R1-R16` set): local channel node -> `3V3`
  - Series resistor (from `R1-R16` set): local channel node -> local RC node
  - RC capacitor (from `C20-C27` set): local RC node -> `GND`
  - Schmitt stage (`U5/U6`): `A` input <- local RC node; `Y` output -> `IN_CHn_SENSE`
  - MCU: `IN_CHn_SENSE` -> GPIO per appendix table
  - Common: `J3.1` -> `GND`; `J3.2` -> `GND`
- Concrete example (CH0):
  - `J2a.1` -> local CH0 input node
  - local CH0 input node -> pull-up resistor to `3V3`
  - local CH0 input node -> series resistor -> local CH0 RC node
  - local CH0 RC node -> RC capacitor -> `GND`
  - local CH0 RC node -> `U5` or `U6` `xA` pin; corresponding `xY` pin net label = `IN_CH0_SENSE`
  - `IN_CH0_SENSE` -> `U1.PA0`
- Verify:
  - CH0..CH7 topology is identical.
  - Channel connector-to-GPIO mapping matches appendix table.
  - `J3.1` and `J3.2` are both `GND` (same electrical net).

#### Page 4 - RS-485 full duplex

- Place: `J4`, `U2A`, `U2B`, `R27`, `D10`, `D11`.
- Wiring matrix:
  - `J4.1` -> `RS485_TX+`; `J4.2` -> `RS485_TX-`; `J4.3` -> `RS485_RX+`; `J4.4` -> `RS485_RX-`; `J4.5` -> `GND`; `J4.6` -> `SHIELD`
  - `U2A` (TX transceiver): `DI` <- `PA9`; `A/B` -> `RS485_TX+/-`; `DE` -> `3V3`; `/RE` -> `3V3`; `VCC` -> `3V3`; `GND` -> `GND`
  - `U2B` (RX transceiver): `A/B` <- `RS485_RX+/-`; `RO` -> `PA10`; `DE` -> `GND`; `/RE` -> `GND`; `VCC` -> `3V3`; `GND` -> `GND`
  - `R27` (120R): across `RS485_RX+` and `RS485_RX-`
  - `D10` (`SM712`, TX pair TVS): line pins -> `RS485_TX+` and `RS485_TX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
  - `D11` (`SM712`, RX pair TVS): line pins -> `RS485_RX+` and `RS485_RX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
- Verify:
  - `U2A` enable pins are fixed TX state and `U2B` enable pins fixed RX state per appendix.
  - `D10` clamps TX pair; `D11` clamps RX pair.
  - No accidental short between TX and RX pair nets.

#### Page 5 - USB-C debug interface

- Place: `J5`, `R31`, `R32`, `R29`, `R30`.
- Wiring matrix:
  - `J5.D+` -> `R31` -> `USB_DP` -> `U1.PA12`
  - `J5.D-` -> `R32` -> `USB_DM` -> `U1.PA11`
  - `J5.CC1` -> `R29` -> `GND`
  - `J5.CC2` -> `R30` -> `GND`
  - `J5.GND/shell` -> `GND` (and chassis policy per PCB appendix)
- Verify:
  - `R31`/`R32` are only in series with D+/D-.
  - `R29`/`R30` are only CC pull-downs.
  - `USB_DP`/`USB_DM` are not connected to any non-USB function.

#### Page 6 - Status LEDs

- Place: `LED1..LED10`, `R17-R28`.
- Wiring matrix:
  - POWER LED: `3V3` -> `R25` (330R) -> `LED1` -> `GND` (always on)
  - LINK LED: `3V3` -> `R28` -> `LED2` -> `LED_LINK_N` (`PB9`, active-low sink)
  - CH LEDs (`LED3-LED10`): each channel LED uses one 330R resistor from `R17-R26` group and sinks to its channel GPIO (`PB2`, `PA15`, `PB3`, `PB4`, `PB5`, `PB6`, `PB7`, `PB8`)
- Verify:
  - CH LEDs map to `PB2`, `PA15`, `PB3`, `PB4`, `PB5`, `PB6`, `PB7`, `PB8`.
  - LINK LED maps to `PB9`.
  - Current-limit values match BOM (`R17-R26` 330R, `R28` 150R).

### Output board page map

#### Page 1 - Power entry, bulk rail, and 3V3 buck

- Place: `J1`, `D1`, `C17`, `C18`, `C29`, `U4`, `L1`, `C20`, `C19`.
- Wiring matrix:
  - `J1.1` -> `VIN_12V_IN`; `J1.2` -> `GND`
  - `D1` (`SS34`): anode -> `VIN_12V_IN`; cathode -> `12V_MAIN`
  - `C17`: `+` -> `12V_MAIN`; `-` -> `GND`
  - `C18`: `+` -> `12V_MAIN`; `-` -> `GND`
  - `C29`: one side -> `12V_MAIN` near `U4.VIN`; other side -> `GND`
  - `U4` (`AP63203WU-7`): `VIN` -> `12V_MAIN`; `GND` -> `GND`; `SW` -> `L1` + `C20`; `BST` -> `C20`
  - `C20` (bootstrap): one side -> `U4.BST`; other side -> `U4.SW`
  - `L1`: one side -> `U4.SW`; other side -> `3V3`
  - `C19`: `+` -> `3V3`; `-` -> `GND`
  - `12V_MAIN` -> `F1..F8` channel feed network
- Verify:
  - `12V_MAIN` is sourced through `D1` from `VIN_12V_IN`.
  - `C20` is only `BST` to `SW`.
  - `C19` is only on `3V3` bulk role.
  - `12V_MAIN` branch to `F1..F8` is separate from low-current logic routing.

#### Page 2 - MCU core + reset/debug

- Place: `U1`, `C1`, `C15`, local 100nF decouplers from `C2-C7` group, `SW1`, `SW2`, `R57`, `J8`.
- Wiring matrix:
  - `U1` power pins: `VDD/VDDA/VBAT` -> `3V3`; `VSS/VSSA` -> `GND`
  - `U1.PA9` -> `USART1_TX` -> `U2B.DI`
  - `U1.PA10` <- `USART1_RX` <- `U2A.RO`
  - `U1.PA11` -> `USB_DM`; `U1.PA12` -> `USB_DP`
  - `U1.NRST` net -> `SW1` + `J8.10`
  - `U1.PA14-BOOT0` net -> `SW2` + `R57` + `J8.4`
  - `C1`: one side -> `3V3`; other side -> `GND`
  - `C15`: one side -> `3V3` (VDDA/VREF domain); other side -> `GND`
  - `R57` (10k): one side -> `PA14-BOOT0`; other side -> `GND`
  - `SW1`: one side -> `NRST`; other side -> `GND`
  - `SW2`: one side -> `PA14-BOOT0`; other side -> `3V3`
  - `J8`: pins wired per appendix (`1:VTREF_3V3`, `2:SWDIO`, `3/5/9:GND`, `4:PA14-BOOT0`, `10:NRST`)
- Verify:
  - `R57` (10k) pulls `PA14-BOOT0` LOW by default.
  - `SW1` resets `NRST`; `SW2` provides BOOT0 override.
  - `J8` pin mapping matches appendix exactly.

#### Page 3 - Override conditioning channels (OVR0..OVR7)

- Place: `J3a`, `J3b`, `J4`, RC cells (`R17-R32`, override RC capacitors `C21-C28`), `U5`, `U6`.
- Net-label convention (important):
  - Keep override frontend wiring local on page 3 (terminal -> RC -> Schmitt input) without extra net labels unless needed for readability.
  - Use only `OVR_CHn_SENSE` as the cross-sheet net label from Schmitt output to MCU input on page 2.
- Wiring matrix (repeat for n=0..7):
  - Connector: `J3a` (OVR0..OVR3) / `J3b` (OVR4..OVR7) channel pin -> local override node
  - Pull-up resistor (from `R17-R32` set): local override node -> `3V3`
  - Series resistor (from `R17-R32` set): local override node -> local RC node
  - RC capacitor (from `C21-C28` set): local RC node -> `GND`
  - Schmitt stage (`U5/U6`): `A` input <- local RC node; `Y` output -> `OVR_CHn_SENSE`
  - MCU: `OVR_CHn_SENSE` -> GPIO per appendix table
  - Common: `J4.1` -> `GND`; `J4.2` -> `GND`
- Concrete example (OVR0):
  - `J3a.1` -> local OVR0 input node
  - local OVR0 input node -> pull-up resistor to `3V3`
  - local OVR0 input node -> series resistor -> local OVR0 RC node
  - local OVR0 RC node -> RC capacitor -> `GND`
  - local OVR0 RC node -> `U5` or `U6` `xA` pin; corresponding `xY` pin net label = `OVR_CH0_SENSE`
  - `OVR_CH0_SENSE` -> `U1.PA0`
- Verify:
  - Channel-to-GPIO map matches appendix (`PA0/PA1/PA4/PA5/PA6/PA7/PB0/PB1`).
  - `J4.1` and `J4.2` are both `GND` (same electrical net).

#### Page 4 - RS-485 full duplex

- Place: `J2`, `U2A`, `U2B`, `R52`, `D18`, `D19`.
- Wiring matrix:
  - `J2.1` -> `RS485_TX+`; `J2.2` -> `RS485_TX-`; `J2.3` -> `RS485_RX+`; `J2.4` -> `RS485_RX-`; `J2.5` -> `GND`; `J2.6` -> `SHIELD`
  - `U2A` (RX transceiver): `A/B` <- `RS485_RX+/-`; `RO` -> `PA10`; `DE` -> `GND`; `/RE` -> `GND`; `VCC` -> `3V3`; `GND` -> `GND`
  - `U2B` (TX transceiver): `DI` <- `PA9`; `A/B` -> `RS485_TX+/-`; `DE` -> `3V3`; `/RE` -> `3V3`; `VCC` -> `3V3`; `GND` -> `GND`
  - `R52` (120R): across `RS485_RX+` and `RS485_RX-`
  - `D18` (`SM712`, TX pair TVS): line pins -> `RS485_TX+` and `RS485_TX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
  - `D19` (`SM712`, RX pair TVS): line pins -> `RS485_RX+` and `RS485_RX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
- Verify:
  - `U2A` forced RX, `U2B` forced TX per appendix.
  - `D18` and `D19` are at connector entry side.
  - TX and RX pair polarity is preserved through connector pins.

#### Page 5 - Output power channels (OUT0..OUT7)

- Place: `Q1..Q8`, `R1-R8`, `R9-R16`, `F1-F8`, `D2-D9`, `J5a`, `J5b`, `J6`.
- Wiring matrix (repeat for channel index `k=0..7`):
  - `12V_MAIN` -> `F(k+1)` -> load positive branch (`J5a/J5b` channel pin)
  - `J5a/J5b` channel return net -> `OUT_CHk_SW` -> `Q(k+1).D`
  - `Q(k+1).S` -> `LOAD_GND_RTN` (`J6.2`) / `GND` return plane
  - `Q(k+1).G` <- gate resistor `R(k+1)` <- `GATE_CHk` GPIO
  - gate pulldown `R(k+9)` from `Q(k+1).G` to `GND`
  - flyback diode `D(k+2)`: anode -> `OUT_CHk_SW`; cathode -> `12V_MAIN`
  - load supply output: `J6.1` -> `LOAD_12V` (fed from `12V_MAIN`)
- Channel-to-power-part map:
  - CH0: `F1`, `Q1`, `R1`, `R9`, `D2`
  - CH1: `F2`, `Q2`, `R2`, `R10`, `D3`
  - CH2: `F3`, `Q3`, `R3`, `R11`, `D4`
  - CH3: `F4`, `Q4`, `R4`, `R12`, `D5`
  - CH4: `F5`, `Q5`, `R5`, `R13`, `D6`
  - CH5: `F6`, `Q6`, `R6`, `R14`, `D7`
  - CH6: `F7`, `Q7`, `R7`, `R15`, `D8`
  - CH7: `F8`, `Q8`, `R8`, `R16`, `D9`
- Verify:
  - Gate GPIO-to-channel map matches appendix table.
  - Each channel has exactly one gate resistor, one pulldown, one fuse, and one flyback diode.
  - `J5a/J5b/J6` pin mapping matches appendix exactly.

#### Page 6 - USB-C debug interface

- Place: `J7`, `R53`, `R54`, `R55`, `R56`.
- Wiring matrix:
  - `J7.D+` -> `R53` -> `USB_DP` -> `U1.PA12`
  - `J7.D-` -> `R54` -> `USB_DM` -> `U1.PA11`
  - `J7.CC1` -> `R55` -> `GND`
  - `J7.CC2` -> `R56` -> `GND`
  - `J7.GND/shell` -> `GND` (and chassis policy per PCB appendix)
- Verify:
  - No non-USB function is connected to `USB_DP`/`USB_DM`.
  - CC nets only terminate through designated 5.1k pull-downs.

#### Page 7 - Status LEDs

- Place: `LED1-LED10`, `R41-R51`.
- Wire:
  - POWER LED: `3V3` -> one 330R resistor from `R41-R50` group -> `LED1` -> `GND`
  - LINK LED: `3V3` -> `R51` -> `LED2` -> `PC14` (active-low sink)
  - CH LEDs (`LED3-LED10`): each channel LED uses one 330R resistor from `R41-R50` group and sinks to its channel GPIO (`PB4`, `PB5`, `PB6`, `PB7`, `PB8`, `PB9`, `PC6`, `PC7`)
- Verify:
  - LED resistor values match BOM (`R41-R50` 330R, `R51` 150R).

### Net-label discipline for all pages

- Never alias `PA14-BOOT0` to any other net name.
- Keep connector net labels identical to appendices:
  - input: `IN_CHn_RAW`, `IN_CHn_SENSE`
  - output: `OVR_CHn_RAW`, `OVR_CHn_SENSE`, `OUT_CHn_SW`, `GATE_CHn`
  - serial: `RS485_TX+/-`, `RS485_RX+/-`
- If a page introduces a new local net, add it to the relevant appendix before release.

## 6) Layout handoff

After schematic closure, implement layout using:

- `hardware/PCB_LAYOUT_GUIDE.md`
- `hardware/PCB_APPENDIX_INPUT.md`
- `hardware/PCB_APPENDIX_OUTPUT.md`

