# Schematic Entry Guide

This is the assembly-executable schematic workflow for the current architecture:

- `STM32G0B1CBT6` on both boards
- Full-duplex RS-485 (`U2A` + `U2B` transceivers on each board) with per-IC `1uF` VCC bypass
- Native USB on `PA11/PA12` via `J5` (input) / `J7` (output) with `USBLC6-2SC6` ESD protection
- RC + Schmitt digital conditioning for all switch inputs (`U5`/`U6`) with per-IC `1uF` VCC bypass
- SWD + `PA14-BOOT0` + NRST recovery path (`SW1`, `SW2`, `J6`/`J8`)
- Board-level `12V` input fuse on both boards: `F1` PTC (input board, ~0.1A load) / `F9` 25A ATO blade fuse in PCB holder (output board, 16A trunk)

Use this file as the build sequence, then use the appendices for full pin/net/junction tables.

## External review status

This guide incorporates the resolutions captured in `hardware/REVIEW_RESOLUTION.md` (2026-04-13 schematic review). All mandatory net-naming, decoupling, ESD, polarity, and page-structure rules below are normative for every future schematic edit unless that file is updated and re-signed-off.

## BOM convention

- BOM is one line per unique MPN.
- Combined designators must match quantity.
- Substitutions must preserve function, package fit, and net behavior.

## Required supporting appendices

- Input schematic appendix: `hardware/SCHEMATIC_APPENDIX_INPUT.md`
- Output schematic appendix: `hardware/SCHEMATIC_APPENDIX_OUTPUT.md`
- Review resolution log: `hardware/REVIEW_RESOLUTION.md`

Do not leave any net undefined in the top-level sheet set. If a net is not explicitly listed there, add it before release.

## 1) Shared schematic rules

### Net naming and power-symbol convention (mandatory)

Locked rules (review fix G2/G3/G4/G5):

- The 3.3V rail is named exactly `3V3`. Any of `3.3V`, `+3.3V`, `+3V3`, or other variants is a release-blocking ERC error.
- The protected 12V rail (downstream of `D1` and, on output board, `F9`) is named `12V_MAIN`. The pre-protection net is `VIN_12V_IN`.
- Ground is named `GND`. The cable-shield/chassis net is `SHIELD` / `CHASSIS` per the RS-485 shield policy in `PCB_LAYOUT_GUIDE.md`.
- `GND`, `3V3`, `12V_MAIN`, and `VIN_12V_IN` MUST be drawn as Power Symbol Net Flags (global net flags). Do NOT use Hierarchical / Off-page Net Ports for these rails - power symbols are inherently global and connect across all sheets without ports.
- Net Ports are reserved for low-frequency cross-page signals that have no canonical global meaning. After this revision's page consolidation, the only common cross-sheet signals are channel `_SENSE` and `_RAW` nets and the `USART1_TX` / `USART1_RX` link nets.
- ERC must reject any unconnected Power Symbol Net Flag of `3V3`, `12V_MAIN`, `GND`, or `VIN_12V_IN`.

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
    - input board: `C6` (HF) with `C17` (bulk, polarized - see polarity rule below)
    - output board: `C17` + `C18` (bulk, polarized), with `C29` (100nF) near `U4.VIN`
  - bootstrap capacitor from `U4.BST` to `U4.SW`:
    - input board: `C19`
    - output board: `C20`
  - output bulk capacitor on `3V3` near `L1`/`U4` - the `AP63203WU-7` datasheet calls for 2x 22uF on the output, so BOTH caps are required (review fix P2/OP3):
    - input board: `C18` + `C28` (both 22uF in parallel on `3V3`)
    - output board: `C19` + `C30` (both 22uF in parallel on `3V3`)

### Polarized capacitor polarity rule (review fix P1/OP2) - DO NOT VIOLATE

Reverse polarity on an aluminum/tantalum bulk cap will cause venting or rupture on first power-on. Lock these orientations:

- Input board `C17` (bulk on `12V_MAIN`): `+` -> `12V_MAIN`, `-` -> `GND`.
- Output board `C17` (bulk on `12V_MAIN`): `+` -> `12V_MAIN`, `-` -> `GND`.
- Output board `C18` (bulk on `12V_MAIN`): `+` -> `12V_MAIN`, `-` -> `GND`.
- Output board `C19` (bulk on `3V3` output): `+` -> `3V3`, `-` -> `GND`.
- Output board `C30` (bulk on `3V3` output): `+` -> `3V3`, `-` -> `GND`.
- Input board `C28` (bulk on `3V3` output): `+` -> `3V3`, `-` -> `GND`.
- Input board `C18` (bulk on `3V3` output): `+` -> `3V3`, `-` -> `GND`.

Polarity must be verified in schematic AND in PCB silkscreen/footprint orientation before fab release. Footprint silkscreen `+` mark must match the schematic `+` pin.

### Per-IC `1uF` VCC bypass rule (review fix I1/R1/OP4)

In addition to the existing 100nF HF caps already specified at each IC's VCC pin, every active IC must have a 1uF X7R bulk-bypass cap from VCC to local GND. The 100nF and 1uF caps serve different roles (HF transient vs. mid-frequency low-impedance bulk) - keep both.

Required `1uF` caps:

- Input board:
  - `C29` at `U2A.VCC` (RS-485 TX transceiver)
  - `C30` at `U2B.VCC` (RS-485 RX transceiver)
  - `C31` at `U5.VCC` (Schmitt inverter, channels CH0..CH5)
  - `C32` at `U6.VCC` (Schmitt inverter, channels CH6..CH7)
- Output board:
  - `C31` at `U2A.VCC` (RS-485 RX transceiver)
  - `C32` at `U2B.VCC` (RS-485 TX transceiver)
  - `C33` at `U5.VCC` (Schmitt inverter, override CH0..CH5)
  - `C34` at `U6.VCC` (Schmitt inverter, override CH6..CH7)

All `1uF` caps go from the IC VCC pin DIRECTLY to local GND, with the loop kept short. Place physically adjacent to the IC.

### USB-C ESD protection rule (review fix U1/OP5)

USB-C data pins are exposed to user handling and external cable hot-plug. ESD events on `USB_DP` / `USB_DM` can damage the STM32G0B1 USB peripheral. Required:

- Input board: `D12` = `USBLC6-2SC6` (LCSC `C7519`, SOT-23-6) on `J5` D+/D-:
  - `D12` IO1 -> `USB_DP` (between `R31` and `J5.D+`)
  - `D12` IO2 -> `USB_DM` (between `R32` and `J5.D-`)
  - `D12` VBUS -> `USB_VBUS` (also clamps VBUS)
  - `D12` GND -> `GND`
- Output board: `D20` = `USBLC6-2SC6` (LCSC `C7519`, SOT-23-6) on `J7` D+/D-:
  - `D20` IO1 -> `USB_DP` (between `R53` and `J7.D+`)
  - `D20` IO2 -> `USB_DM` (between `R54` and `J7.D-`)
  - `D20` VBUS -> `USB_VBUS`
  - `D20` GND -> `GND`

PCB rule: place ESD diode within 5 mm of the USB-C connector pins so that the clamp path is shorter than the path into the MCU.

### MCU GPIO pin sink-current verification (review fix M1/OP6)

LEDs are sunk by MCU GPIOs (active-low). Each pin must stay within STM32G0B1 datasheet limits. Recorded math:

- Datasheet `Iol` per pin: 20mA absolute max, 8mA recommended max for digital integrity.
- Per-LED current: `(3V3 - V_F_LED) / R_series = (3.3V - 2.0V) / 330R ~= 3.94mA`.
- Per-LED current is well under the 8mA recommended limit. Margin >=2x.
- Total sustained current across all 8 channel LEDs + LINK LED at full-on: `9 x 3.94mA ~= 35.5mA`, within the per-port group `Itotal` limit (datasheet typ. 80mA per port group).

LED resistor values must remain at `330R` (channel) or `150R` (LINK) as documented in the LED page. Any future LED swap with higher V_F headroom or different brightness must redo this calculation and update both schematic appendices.

### Output-board 12V input fuse rule (review fix OP1)

The output board's 12V supply (Mean Well `LRS-200-12`) has an internal mains-side fuse, but board-level protection on the DC side is still required for defense-in-depth (PSU fuse is non-replaceable from the user side and does not protect against PCB-side faults like a shorted bulk cap or a wrong-polarity transient that bypasses the PSU output stage). Required:

- `F9` = 4-pin PCB-mount ATO blade fuse holder, Littelfuse `178.6165.0002` (LCSC `C207061`), 22.5A continuous / 30A max, 32V, placed in series on `VIN_12V_IN -> F9 -> D1.A`. In the EasyEDA `C207061` footprint this is 8 symbol pins (doubled onto 4 legs); wire **pins 1-4 to `VIN_12V_IN`** and **pins 5-8 to `D1.A`** (all four pins of each blade contact tied together). See the `F9` pin map in `SCHEMATIC_APPENDIX_OUTPUT.md` section 11.
- Fuse element: **25A ATO blade fuse**, 32V (field-inserted accessory, not placed by PCBA).
- `F9` carries the full board trunk current (up to 8x 2A = 16A). A 25A blade fuse derates to ~18A continuous (transparent at 16A) and opens on a hard fault; the 4-pin holder (22.5A continuous) is mandatory because the 1-pin holder is only rated 11A. An SMD PTC cannot be used here (would need ~20A hold). See `SCHEMATIC_APPENDIX_OUTPUT.md` section 11.
- The `J1` 12V-input terminal and the `J6` load +12V feed terminal each carry the full 16A trunk current, so both are `KF128-7.5-2P` (LCSC `C474954`), 7.5mm pitch, 24A / 450V, 12-22 AWG screw terminals (not the 5.08mm `DB128V` used elsewhere). `J6` both poles are `12V_MAIN` (paralleled). Size the `VIN_12V_IN -> F9 -> D1 -> 12V_MAIN` trunk (including the `J6` load feed) and the internal `LOAD_GND_RTN` (MOSFET-source ground back to `J1.2`) copper for 16A.

## 2) Input-board schematic capture sequence

1. Place connectors (`J1`, `J2a`, `J2b`, `J3`, `J4`, `J5`, `J6`) and assign pin numbers per appendix.
2. Draw power path: `J1(12V)` -> `D1(SS34)` -> `F1(PTC)` -> `12V_MAIN`, then buck stage:
   - `12V_MAIN` -> `U4.VIN`
   - `U4.SW` -> `L1` -> `3V3`
   - `U4.BST` -> `C19` -> `U4.SW`
   - `C6` + `C17` (polarized, `+` to `12V_MAIN`) on input side
   - `C18` + `C28` (both 22uF on `3V3`, polarized `+` to `3V3`) on output side
   - All caps return to low-impedance `GND`
3. Add USB-C debug nets on `J5` with `R31`/`R32` (D+/D- series) and `R29`/`R30` (CC pull-down). Insert `D12` (`USBLC6-2SC6`) ESD clamp on the connector side of `R31`/`R32` (clamp between the USB connector and the series resistors, upstream of `R31`/`R32`). Then wire SWD header `J6`.
4. Add dual SP3485 transceivers with per-IC bypass:
   - `U2A`: TX-only path from `PA9` + `C29` (1uF) at `U2A.VCC`
   - `U2B`: RX-only path to `PA10` + `R27` (120R) termination on RX pair + `C30` (1uF) at `U2B.VCC`
5. Add `D10` and `D11` (`SM712`, one per RS-485 pair) close to `J4` net entry.
6. Add 8 input RC + Schmitt cells (`R1-R16`, `C20-C27`, `U5/U6`) and map CH0..CH7 to MCU GPIOs. Add per-Schmitt VCC bypass: `C31` (1uF) at `U5.VCC`, `C32` (1uF) at `U6.VCC`.
7. Add status LEDs (`LED1-LED10`) with current-limit resistors (`R17-R26` 330R bank + `R28` 150R LINK; `R27` is the RS-485 termination, not an LED resistor). Verify per-pin sink current per the MCU GPIO sink-current rule.
8. Confirm net-flag rule: `GND`, `3V3`, `12V_MAIN`, `VIN_12V_IN` use Power Symbol Net Flags only.
9. Run ERC and verify against input appendix tables.

All endpoint mappings are in `hardware/SCHEMATIC_APPENDIX_INPUT.md`.

## 3) Output-board schematic capture sequence

1. Place connectors (`J1`, `J2`, `J3a`, `J3b`, `J4`, `J5a`, `J5b`, `J6`, `J7`, `J8`) and assign pin numbers per appendix.
2. Draw input protection and buck-based `3V3` generation path:
   - `J1.1` -> `VIN_12V_IN` -> `F9` pins 1-4 (input blade); `F9` pins 5-8 (output blade) -> `D1.A` (`SS34`) -> `12V_MAIN`
   - `12V_MAIN` -> `U4.VIN`
   - `U4.SW` -> `L1` -> `3V3`
   - `U4.BST` -> `C20` -> `U4.SW`
   - `C17` + `C18` (polarized, `+` to `12V_MAIN`) on input side, `C29` (100nF) close to `U4.VIN`
   - `C19` + `C30` (both 22uF on `3V3`, polarized `+` to `3V3`) on output side
   - All caps return to low-impedance `GND`
3. Add dual SP3485 transceivers with per-IC bypass:
   - `U2A`: RX-only from cable to `PA10` + `C31` (1uF) at `U2A.VCC`
   - `U2B`: TX-only heartbeat from `PA9` + `C32` (1uF) at `U2B.VCC`
4. Add RX-pair termination `R52` (120R) and two RS-485 TVS devices (`D18`, `D19`).
5. Add 8 override RC + Schmitt cells (`R17-R32`, `C21-C28`, `U5/U6`) into CH0..CH7 override GPIOs. Add per-Schmitt VCC bypass: `C33` (1uF) at `U5.VCC`, `C34` (1uF) at `U6.VCC`.
6. Add 8 MOSFET output channels (`Q1-Q8`, `R1-R8` gate resistors, `R9-R16` pulldowns, `F1-F8`, `D2-D9` flyback).
7. Add USB-C debug nets on `J7` with `R53`/`R54` (D+/D- series) and `R55`/`R56` (CC pull-down). Insert `D20` (`USBLC6-2SC6`) ESD clamp on the connector side of `R53`/`R54` (clamp between the USB connector and the series resistors, upstream of `R53`/`R54`).
8. Add status LEDs (`LED1-LED10`) with current-limit resistors (`R41-R51`). Verify per-pin sink current.
9. Confirm net-flag rule: `GND`, `3V3`, `12V_MAIN`, `VIN_12V_IN` use Power Symbol Net Flags only.
10. Run ERC and verify against output appendix tables.

All endpoint mappings are in `hardware/SCHEMATIC_APPENDIX_OUTPUT.md`.

## 4) Schematic release gate (must pass)

- Every connector pin appears in a mapping table.
- Every CH0..CH7 signal has a full chain from connector -> frontend -> MCU -> protocol bit -> output gate/load.
- `PA9/PA10` used only for USART1 serial link.
- `PA11/PA12` reserved for USB only.
- `PA14-BOOT0` naming and behavior are consistent across both boards.
- RS-485 TX and RX pairs are separated and correctly crossed across the cable.
- SWD pinout, NRST, and BOOT0 controls are accessible and labeled.
- Net-flag rule: `GND`, `3V3`, `12V_MAIN`, `VIN_12V_IN` use Power Symbol Net Flags only. No `3.3V`, `+3.3V`, or other 3V3 alias appears anywhere.
- Polarized-cap orientation rule: every `C17`, `C18` (output board), `C19` (output board), `C28` (input board), `C30` (output board) has `+` on the higher-potential rail per the polarity table.
- Per-IC `1uF` VCC bypass exists on `U2A`, `U2B`, `U5`, and `U6` on both boards (in addition to existing 100nF HF caps).
- USB ESD device (`D12` input / `D20` output) is wired between the connector and series resistors, with VBUS clamp leg also wired.
- Output board has `F9` ATO blade fuse holder (4-pin, `178.6165.0002`) with 25A fuse in series on `VIN_12V_IN -> D1`.

## 5) EasyEDA page/sheet structure (consolidated, review fix G1)

Use this page map in EasyEDA (or KiCad hierarchical sheets) so entry is deterministic and reviewable. Keep sheet names and intent aligned across both boards.

Page count was reduced per reviewer feedback (G1) to minimize cross-sheet hops during review:

- Input board: 4 pages (was 6).
- Output board: 5 pages (was 7).

The MCU, USB-C, and LED blocks now live on a single "MCU + Interfaces" page on each board, since they all terminate at MCU pins and benefit from being readable side-by-side.

### Input board page map

#### Page 1 - Power entry and 3V3 buck

- Place: `J1`, `D1`, `F1`, `C17`, `C6`, `U4`, `L1`, `C18`, `C19`, `C28`.
- Wiring matrix:
  - `J1.1` -> `VIN_12V_IN`; `J1.2` -> `GND`
  - `D1`: anode -> `VIN_12V_IN`; cathode -> `F1` input node
  - `F1`: input -> `D1` cathode node; output -> `12V_MAIN`
  - `C17` (polarized): `+` -> `12V_MAIN`; `-` -> `GND`
  - `C6` (HF): one side -> `12V_MAIN`; other side -> `GND`
  - `U4` (`AP63203WU-7`): `VIN` -> `12V_MAIN`; `GND` -> `GND`; `SW` -> `L1` + `C19`; `BST` -> `C19`
  - `C19` (bootstrap): one side -> `U4.BST`; other side -> `U4.SW`
  - `L1`: one side -> `U4.SW`; other side -> `3V3`
  - `C18` (polarized, 22uF): `+` -> `3V3`; `-` -> `GND`
  - `C28` (polarized, 22uF, parallel to `C18`, review fix P2): `+` -> `3V3`; `-` -> `GND`
- Verify:
  - `12V_MAIN` exists only after `D1` + `F1`.
  - `C19` is only on `BST`/`SW` loop (not tied to `3V3` directly).
  - `3V3` does not bypass `L1`.
  - `C17`, `C18`, `C28` all have `+` on the higher-potential rail (review fix P1).
  - `GND`, `3V3`, `12V_MAIN`, `VIN_12V_IN` use Power Symbol Net Flags (no Net Ports).

#### Page 2 - MCU + USB-C + Status LEDs (consolidated)

- Place: `U1`, `C1`, `C15`, `SW1`, `SW2`, `R33`, `J6`, `J5`, `R31`, `R32`, `R29`, `R30`, `D12`, `LED1..LED10`, `R17-R26` (330R LED bank), `R28` (150R LINK). Note: `R27` (120R RS-485 termination) lives on page 4, not here.
- Sub-block A: MCU core + reset/debug
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
- Sub-block B: USB-C debug + ESD (review fix U1)
  - `J5.D+` -> `USB_DP` clamp node (`D12.IO1`) -> `R31` (22R) -> `U1.PA12`
  - `J5.D-` -> `USB_DM` clamp node (`D12.IO2`) -> `R32` (22R) -> `U1.PA11`
  - `J5.CC1` -> `R29` (5.1k) -> `GND`
  - `J5.CC2` -> `R30` (5.1k) -> `GND`
  - `J5.VBUS` -> `USB_VBUS` -> `D12.VBUS`
  - `D12.GND` -> `GND`
  - `J5.GND/shell` -> `GND` (and chassis policy per PCB appendix)
- Sub-block C: Status LEDs (review fix M1 verified)
  - POWER LED: `3V3` -> `R25` (330R) -> `LED1` -> `GND` (always on)
  - LINK LED: `3V3` -> `R28` (150R) -> `LED2` -> `LED_LINK_N` (`PB9`, active-low sink)
  - CH LEDs (`LED3-LED10`): each channel LED uses one 330R resistor from `R17-R26` group and sinks to its channel GPIO (`PB2`, `PA15`, `PB3`, `PB4`, `PB5`, `PB6`, `PB7`, `PB8`)
- Verify:
  - `R33` (10k) pulls `PA14-BOOT0` LOW by default.
  - `SW1` only asserts `NRST` LOW when pressed.
  - `SW2` only drives BOOT0 path while pressed (no latch path).
  - `J6` pin mapping matches appendix exactly.
  - `D12` is on the USB cable side of `R31`/`R32` (clamp upstream of series Z); VBUS leg is wired.
  - `R31`/`R32` are only in series with D+/D-; `R29`/`R30` are only CC pull-downs; `USB_DP`/`USB_DM` are not connected to any non-USB function.
  - CH LEDs map to `PB2`, `PA15`, `PB3`, `PB4`, `PB5`, `PB6`, `PB7`, `PB8`. LINK LED maps to `PB9`.
  - Per-pin LED current = `(3.3-2.0)/330 ~= 3.94mA` (well within `Iol` recommended 8mA).

#### Page 3 - Input conditioning channels (CH0..CH7)

- Place: `J2a`, `J2b`, `J3`, `R1-R16`, channel RC capacitors `C20-C27`, `U5`, `U6`, `C31`, `C32`.
- Net-label convention (important):
  - Keep channel frontend wiring local on this page (terminal -> RC -> Schmitt input) without extra net labels unless needed for readability.
  - Use only `IN_CHn_SENSE` as the cross-sheet net label from Schmitt output to MCU input on page 2.
- Wiring matrix (repeat for n=0..7):
  - Connector: `J2a` (CH0..CH3) / `J2b` (CH4..CH7) channel pin -> local channel node
  - Pull-up resistor (from `R1-R16` set): local channel node -> `3V3`
  - Series resistor (from `R1-R16` set): local channel node -> local RC node
  - RC capacitor (from `C20-C27` set): local RC node -> `GND`
  - Schmitt stage (`U5/U6`): `A` input <- local RC node; `Y` output -> `IN_CHn_SENSE`
  - MCU: `IN_CHn_SENSE` -> GPIO per appendix table
  - Common: `J3.1` -> `GND`; `J3.2` -> `GND`
- Per-IC bypass (review fix I1):
  - `U5.VCC` -> `3V3` with `C31` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
  - `U6.VCC` -> `3V3` with `C32` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
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
  - `C31` and `C32` are physically adjacent to `U5` and `U6` VCC pins respectively.

#### Page 4 - RS-485 full duplex

- Place: `J4`, `U2A`, `U2B`, `R27`, `D10`, `D11`, `C29`, `C30`.
- Wiring matrix:
  - `J4.1` -> `RS485_TX+`; `J4.2` -> `RS485_TX-`; `J4.3` -> `RS485_RX+`; `J4.4` -> `RS485_RX-`; `J4.5` -> `GND`; `J4.6` -> `SHIELD`
  - `U2A` (TX transceiver): `DI` <- `PA9`; `A/B` -> `RS485_TX+/-`; `DE` -> `3V3`; `/RE` -> `3V3`; `VCC` -> `3V3`; `GND` -> `GND`
  - `U2B` (RX transceiver): `A/B` <- `RS485_RX+/-`; `RO` -> `PA10`; `DE` -> `GND`; `/RE` -> `GND`; `VCC` -> `3V3`; `GND` -> `GND`
  - `R27` (120R): across `RS485_RX+` and `RS485_RX-`
  - `D10` (`SM712`, TX pair TVS): line pins -> `RS485_TX+` and `RS485_TX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
  - `D11` (`SM712`, RX pair TVS): line pins -> `RS485_RX+` and `RS485_RX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
- Per-IC bypass (review fix R1):
  - `U2A.VCC` -> `3V3` with `C29` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
  - `U2B.VCC` -> `3V3` with `C30` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
- Verify:
  - `U2A` enable pins are fixed TX state and `U2B` enable pins fixed RX state per appendix.
  - `D10` clamps TX pair; `D11` clamps RX pair.
  - No accidental short between TX and RX pair nets.
  - `C29` and `C30` are physically adjacent to `U2A` and `U2B` VCC pins respectively.

### Output board page map

#### Page 1 - Power entry, bulk rail, and 3V3 buck

- Place: `J1`, `F9`, `D1`, `C17`, `C18`, `C29`, `U4`, `L1`, `C20`, `C19`, `C30`.
- Wiring matrix:
  - `J1.1` -> `VIN_12V_IN`; `J1.2` -> `GND`
  - `F9` (ATO blade fuse holder `178.6165.0002` + 25A fuse, review fix OP1): input blade pins 1-4 -> `VIN_12V_IN`; output blade pins 5-8 -> `D1.A`. Tie all four pins of each blade contact together (current sharing); never join pins 1-4 with pins 5-8.
  - `D1` (`SS34`): anode -> `F9` output node (pins 5-8); cathode -> `12V_MAIN`
  - `C17` (polarized): `+` -> `12V_MAIN`; `-` -> `GND`
  - `C18` (polarized): `+` -> `12V_MAIN`; `-` -> `GND`
  - `C29` (HF 100nF): one side -> `12V_MAIN` near `U4.VIN`; other side -> `GND`
  - `U4` (`AP63203WU-7`): `VIN` -> `12V_MAIN`; `GND` -> `GND`; `SW` -> `L1` + `C20`; `BST` -> `C20`
  - `C20` (bootstrap): one side -> `U4.BST`; other side -> `U4.SW`
  - `L1`: one side -> `U4.SW`; other side -> `3V3`
  - `C19` (polarized, 22uF): `+` -> `3V3`; `-` -> `GND`
  - `C30` (polarized, 22uF, parallel to `C19`, review fix OP3): `+` -> `3V3`; `-` -> `GND`
  - `12V_MAIN` -> `F1..F8` channel feed network
- Verify:
  - `12V_MAIN` is sourced through `F9` then `D1` from `VIN_12V_IN`.
  - `F9` is in series, not in parallel with `D1`.
  - `C20` is only `BST` to `SW`.
  - `C19` and `C30` are both on `3V3` bulk role with `+` to `3V3`.
  - `C17`, `C18` polarity is `+` to `12V_MAIN` (review fix OP2).
  - `12V_MAIN` branch to `F1..F8` is separate from low-current logic routing.
  - `GND`, `3V3`, `12V_MAIN`, `VIN_12V_IN` use Power Symbol Net Flags (no Net Ports).

#### Page 2 - MCU + USB-C + Status LEDs (consolidated)

- Place: `U1`, `C1`, `C15`, local 100nF decouplers from `C2-C7` group, `SW1`, `SW2`, `R57`, `J8`, `J7`, `R53`, `R54`, `R55`, `R56`, `D20`, `LED1..LED10`, `R41-R51`.
- Sub-block A: MCU core + reset/debug
  - `U1` power pins: `VDD/VDDA/VBAT` -> `3V3`; `VSS/VSSA` -> `GND`
  - `U1.PA9` -> `USART1_TX` -> `U2B.DI`
  - `U1.PA10` <- `USART1_RX` <- `U2A.RO`
  - `U1.PA11` -> `USB_DM`; `U1.PA12` -> `USB_DP`
  - `U1.PA13` -> `SWDIO` -> `J8.2`
  - `U1.NRST` net -> `SW1` + `J8.10`
  - `U1.PA14-BOOT0` net -> `SW2` + `R57` + `J8.4`
  - `C1`: one side -> `3V3`; other side -> `GND`
  - `C15`: one side -> `3V3` (VDDA/VREF domain); other side -> `GND`
  - `R57` (10k): one side -> `PA14-BOOT0`; other side -> `GND`
  - `SW1`: one side -> `NRST`; other side -> `GND`
  - `SW2`: one side -> `PA14-BOOT0`; other side -> `3V3`
  - `J8`: pins wired per appendix (`1:VTREF_3V3`, `2:SWDIO`, `3/5/9:GND`, `4:PA14-BOOT0`, `10:NRST`)
- Sub-block B: USB-C debug + ESD (review fix OP5)
  - `J7.D+` -> `USB_DP` clamp node (`D20.IO1`) -> `R53` (22R) -> `U1.PA12`
  - `J7.D-` -> `USB_DM` clamp node (`D20.IO2`) -> `R54` (22R) -> `U1.PA11`
  - `J7.CC1` -> `R55` (5.1k) -> `GND`
  - `J7.CC2` -> `R56` (5.1k) -> `GND`
  - `J7.VBUS` -> `USB_VBUS` -> `D20.VBUS`
  - `D20.GND` -> `GND`
  - `J7.GND/shell` -> `GND` (and chassis policy per PCB appendix)
- Sub-block C: Status LEDs (review fix OP6 verified)
  - POWER LED: `3V3` -> one 330R resistor from `R41-R50` group -> `LED1` -> `GND`
  - LINK LED: `3V3` -> `R51` (150R) -> `LED2` -> `PC14` (active-low sink)
  - CH LEDs (`LED3-LED10`): each channel LED uses one 330R resistor from `R41-R50` group and sinks to its channel GPIO (`PB4`, `PB5`, `PB6`, `PB7`, `PB8`, `PB9`, `PC6`, `PC7`)
- Verify:
  - `R57` (10k) pulls `PA14-BOOT0` LOW by default.
  - `SW1` resets `NRST`; `SW2` provides BOOT0 override.
  - `J8` pin mapping matches appendix exactly.
  - `D20` is on the USB cable side of `R53`/`R54`; VBUS leg is wired.
  - No non-USB function is connected to `USB_DP`/`USB_DM`. CC nets only terminate through designated 5.1k pull-downs.
  - LED resistor values match BOM (`R41-R50` 330R, `R51` 150R). Per-pin LED current = `(3.3-2.0)/330 ~= 3.94mA`.

#### Page 3 - Override conditioning channels (OVR0..OVR7)

- Place: `J3a`, `J3b`, `J4`, RC cells (`R17-R32`, override RC capacitors `C21-C28`), `U5`, `U6`, `C33`, `C34`.
- Net-label convention (important):
  - Keep override frontend wiring local on this page (terminal -> RC -> Schmitt input) without extra net labels unless needed for readability.
  - Use only `OVR_CHn_SENSE` as the cross-sheet net label from Schmitt output to MCU input on page 2.
- Wiring matrix (repeat for n=0..7):
  - Connector: `J3a` (OVR0..OVR3) / `J3b` (OVR4..OVR7) channel pin -> local override node
  - Pull-up resistor (from `R17-R32` set): local override node -> `3V3`
  - Series resistor (from `R17-R32` set): local override node -> local RC node
  - RC capacitor (from `C21-C28` set): local RC node -> `GND`
  - Schmitt stage (`U5/U6`): `A` input <- local RC node; `Y` output -> `OVR_CHn_SENSE`
  - MCU: `OVR_CHn_SENSE` -> GPIO per appendix table
  - Common: `J4.1` -> `GND`; `J4.2` -> `GND`
- Per-IC bypass (review fix OP4):
  - `U5.VCC` -> `3V3` with `C33` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
  - `U6.VCC` -> `3V3` with `C34` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
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
  - `C33` and `C34` are physically adjacent to `U5` and `U6` VCC pins respectively.

#### Page 4 - RS-485 full duplex

- Place: `J2`, `U2A`, `U2B`, `R52`, `D18`, `D19`, `C31`, `C32`.
- Wiring matrix:
  - `J2.1` -> `RS485_TX+`; `J2.2` -> `RS485_TX-`; `J2.3` -> `RS485_RX+`; `J2.4` -> `RS485_RX-`; `J2.5` -> `GND`; `J2.6` -> `SHIELD`
  - `U2A` (RX transceiver): `A/B` <- `RS485_RX+/-`; `RO` -> `PA10`; `DE` -> `GND`; `/RE` -> `GND`; `VCC` -> `3V3`; `GND` -> `GND`
  - `U2B` (TX transceiver): `DI` <- `PA9`; `A/B` -> `RS485_TX+/-`; `DE` -> `3V3`; `/RE` -> `3V3`; `VCC` -> `3V3`; `GND` -> `GND`
  - `R52` (120R): across `RS485_RX+` and `RS485_RX-`
  - `D18` (`SM712`, TX pair TVS): line pins -> `RS485_TX+` and `RS485_TX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
  - `D19` (`SM712`, RX pair TVS): line pins -> `RS485_RX+` and `RS485_RX-`; TVS return pin -> local return (`GND`, with chassis policy per PCB appendix)
- Per-IC bypass (review fix OP4):
  - `U2A.VCC` -> `3V3` with `C31` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
  - `U2B.VCC` -> `3V3` with `C32` (1uF) directly to `GND` and existing 100nF HF cap also to `GND`.
- Verify:
  - `U2A` forced RX, `U2B` forced TX per appendix.
  - `D18` and `D19` are at connector entry side.
  - TX and RX pair polarity is preserved through connector pins.
  - `C31` and `C32` are physically adjacent to `U2A` and `U2B` VCC pins respectively.

#### Page 5 - Output power channels (OUT0..OUT7)

- Place: `Q1..Q8` (low-side MOSFETs), `R1-R8` (gate, 100R), `R9-R16` (gate pulldown, 10k), `F1-F8` (per-channel fuse), `D2-D9` (flyback), `J5a`, `J5b`, `J6`.

**Topology (low-side switching, shared +12V feed).** This board switches the LOW side of each load:

- All eight loads share one +12V high-side feed: `12V_MAIN` -> `J6.1`/`J6.2` (both poles are `12V_MAIN`, paralleled for distributing the feed). There is no per-channel +12V terminal.
- Each load's low side returns to its own per-channel terminal `J5a/J5b.k`, net `OUT_CHk`. The load's return IS this `J5` pin - it reaches ground through the channel MOSFET inside the box. There is deliberately no load-ground terminal (a load returned to ground would bypass its switch).
- On the board, that low side runs through the channel fuse to the MOSFET drain, then the MOSFET switches it to ground: `OUT_CHk` -> `F` -> `OUT_CHk_SW` (= `Q.D`) -> `Q` -> `Q.S` -> `LOAD_GND_RTN` (internal MOSFET-source ground -> board `GND` -> returns to PSU via `J1.2`).
- The flyback diode is wired directly across the load (anode on the `OUT_CHk` terminal, cathode on `12V_MAIN`), so inductive turn-off current freewheels through the load and diode only and does NOT pass through the fuse or MOSFET.

Per-channel schematic (channel `k`; designators per the part-map table):

```text
        12V_MAIN  (J6.1 + J6.2, both poles, shared +12V load feed)
           |
     [ external load ]   (12V solenoid / relay)
           |
  J5a/J5b.k  =====+=======================+    net OUT_CHk  (load low side, at the terminal)
  (terminal)      |                       |
              [flyback D]             [fuse F]
              anode = OUT_CHk          OUT_CHk -> OUT_CHk_SW
              cathode = 12V_MAIN           |
                                           v
                                    OUT_CHk_SW = Q.D
                                           |
  GATE_CHk --[gate R]--+----- Q.G        [ Q ]  low-side N-FET (IRLML6344)
                       |                   |
                 [pulldown R]             Q.S
                       |                   |
                      GND          LOAD_GND_RTN -> GND (-> J1.2 -> PSU-)
```

- Wiring matrix (repeat for channel index `k=0..7`; designators in the part-map table):
  - Shared +12V feed: `12V_MAIN` -> `J6.1`/`J6.2` (both poles paralleled). One net for all eight loads' high side; not fused per channel.
  - Load low side / terminal: `J5a/J5b.k` = net `OUT_CHk`.
  - Per-channel fuse: `OUT_CHk` -> fuse -> `OUT_CHk_SW`.
  - MOSFET: `OUT_CHk_SW` -> `Q.D`; `Q.S` -> `LOAD_GND_RTN` (internal source ground; ties to board `GND` / returns via `J1.2`).
  - Gate drive: `GATE_CHk` GPIO -> gate resistor -> `Q.G`; gate pulldown from `Q.G` -> `GND`.
  - Flyback: diode anode -> `OUT_CHk` (load side of the fuse, at the terminal); cathode -> `12V_MAIN`.

- Channel-to-power-part map (gate R = `R1-R8` 100R; pulldown R = `R9-R16` 10k):

| CH | Terminal | Fuse | MOSFET | Gate R | Pulldown R | Flyback D |
| --- | --- | --- | --- | --- | --- | --- |
| CH0 | `J5a.1` | `F1` | `Q1` | `R1` | `R9` | `D2` |
| CH1 | `J5a.2` | `F2` | `Q2` | `R2` | `R10` | `D3` |
| CH2 | `J5a.3` | `F3` | `Q3` | `R3` | `R11` | `D4` |
| CH3 | `J5a.4` | `F4` | `Q4` | `R4` | `R12` | `D5` |
| CH4 | `J5b.1` | `F5` | `Q5` | `R5` | `R13` | `D6` |
| CH5 | `J5b.2` | `F6` | `Q6` | `R6` | `R14` | `D7` |
| CH6 | `J5b.3` | `F7` | `Q7` | `R7` | `R15` | `D8` |
| CH7 | `J5b.4` | `F8` | `Q8` | `R8` | `R16` | `D9` |

- Concrete example (CH0, `GATE_CH0` = `PB2`):
  - `12V_MAIN` -> `J6.1`/`J6.2`; external load high side -> `12V_MAIN` (at `J6`), load low side -> `J5a.1` (net `OUT_CH0`)
  - `OUT_CH0` -> `F1` -> `OUT_CH0_SW` -> `Q1.D`; `Q1.S` -> `LOAD_GND_RTN` (internal source ground -> board `GND`)
  - `GATE_CH0` (`PB2`) -> `R1` (100R) -> `Q1.G`; `R9` (10k) from `Q1.G` -> `GND`
  - `D2`: anode -> `OUT_CH0` (terminal); cathode -> `12V_MAIN`
- Verify:
  - Gate GPIO-to-channel map matches appendix table.
  - Each channel has exactly one gate resistor, one pulldown, one fuse, and one flyback diode.
  - The fuse is in series between the `J5` terminal (`OUT_CHk`) and the MOSFET drain (`OUT_CHk_SW`); it is NOT in the shared `12V_MAIN` feed.
  - Flyback anode is on the `OUT_CHk` terminal (load side), cathode on `12V_MAIN`, so freewheel current bypasses the fuse and MOSFET.
  - `J5a/J5b/J6` pin mapping matches appendix exactly: `J6.1` = `J6.2` = `12V_MAIN` (paralleled). The load ground return is internal (`LOAD_GND_RTN` = MOSFET sources -> board `GND` -> `J1.2`); it is NOT on `J6`.
  - A load is wired between a `J6` `12V_MAIN` pole and its `J5` channel pin; never between `12V_MAIN` and a ground terminal (that would bypass the switch).

### Net-label discipline for all pages

- Never alias `PA14-BOOT0` to any other net name.
- Keep connector net labels identical to appendices:
  - input: `IN_CHn_RAW`, `IN_CHn_SENSE`
  - output: `OVR_CHn_RAW`, `OVR_CHn_SENSE`, `OUT_CHn` (load-side terminal net, `J5a/J5b.n`), `OUT_CHn_SW` (MOSFET drain, after the fuse), `GATE_CHn`
  - serial: `RS485_TX+/-`, `RS485_RX+/-`
- If a page introduces a new local net, add it to the relevant appendix before release.

## 6) Layout handoff

After schematic closure, implement layout using:

- `hardware/PCB_LAYOUT_GUIDE.md`
- `hardware/PCB_APPENDIX_INPUT.md`
- `hardware/PCB_APPENDIX_OUTPUT.md`
- `hardware/REVIEW_RESOLUTION.md` (binding rules from the 2026-04-13 schematic review)

