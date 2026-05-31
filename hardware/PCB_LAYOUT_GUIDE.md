# PCB Layout Guide

This guide is the layout execution workflow for both boards.
It pairs with:

- `hardware/SCHEMATIC_GUIDE.md`
- `hardware/PCB_APPENDIX_INPUT.md`
- `hardware/PCB_APPENDIX_OUTPUT.md`
- `hardware/REVIEW_RESOLUTION.md` (mandatory rules from the 2026-04-13 schematic review)

Architecture assumptions:

- STM32G0B1CBT6 MCU
- Full-duplex RS-485 (2x SP3485EN per board)
- Native USB on PA11/PA12
- RC + Schmitt digital input front-end
- SWD + NRST + BOOT0 debug/recovery controls

## Required appendices

- Input-board executable details: `hardware/PCB_APPENDIX_INPUT.md`
- Output-board executable details: `hardware/PCB_APPENDIX_OUTPUT.md`

This top-level guide defines common constraints. Appendix files carry board-specific placement maps, net classes, and test-point matrices.

## 1) Placement order (high-confidence workflow)

1. Place connectors (`Jx`) and mounting holes (mechanical constraints first).
2. Place power entry/protection parts and bulk capacitors:
   - Input board: `D1`, `F1`, `C17` (polarized `+` to `12V_MAIN`), `C6` HF, plus buck cluster `U4`/`L1`/`C18`/`C19`/`C28` (both 22uF on `3V3` are required per AP63203 datasheet).
   - Output board: `F9` (ATO blade fuse holder `178.6165.0002`, in series before `D1` - 8-pad THT footprint, pins 1-4=`VIN_12V_IN` / pins 5-8=`D1.A`, reserve power-entry-zone area), `D1`, `C17`/`C18` (both polarized `+` to `12V_MAIN`), `C29` HF, plus buck cluster `U4`/`L1`/`C19`/`C20`/`C30` (both 22uF on `3V3`). The `VIN_12V_IN -> F9 -> D1 -> 12V_MAIN` trunk carries up to 16A: use wide pour/heavy copper.
3. Place MCU (`U1`) and local decoupling (`C1`, `C2`, `C15` + per-IC 100nF caps).
4. Place RS-485 transceivers (`U2A`, `U2B`) + termination (`R27` input / `R52` output) + TVS (`D10/D11` input, `D18/D19` output) near RS-485 terminal (`J4` input / `J2` output). Include the 1uF VCC bypass cap at each transceiver (`C29/C30` input, `C31/C32` output) physically adjacent to the IC's VCC pin.
5. Place USB connector (`J5` input / `J7` output) + D+/D- series resistors (`R31/R32` input, `R53/R54` output) near MCU pins. Place USB ESD diode (`D12` input / `D20` output, `USBLC6-2SC6`) within 5 mm of the USB-C connector pins so the clamp is upstream of the series resistors.
6. Place Schmitt input conditioning channels (`U5/U6` with board-specific `R/C` channel networks) as repeated blocks. Include the 1uF VCC bypass cap at each Schmitt IC (`C31/C32` input, `C33/C34` output) physically adjacent to the IC's VCC pin.
7. Place output MOSFET/PTC/flyback blocks (output board).
8. Place status LED block (`LED1..LED10` + current-limit resistor banks) along one enclosure-visible edge.
9. Place SWD/NRST/BOOT0 hardware (`J6` input / `J8` output, `SW1`, `SW2`) where probe/tool access is easy.

## 2) Layer stack and trace guidance (design choice lock)

Recommended minimum:

- 2-layer is feasible if current paths are short/wide and grounding is disciplined.
- 4-layer is preferred for cleaner return paths and easier EMI control.

For 2-layer:

- Top: component + critical routing
- Bottom: near-continuous GND plane with minimal slotting

For release, lock one stackup per board in the appendix and keep all impedance calculations tied to that stackup.

For the output board, document the selected stackup and resulting USB/RS-485 pair geometry in `hardware/PCB_APPENDIX_OUTPUT.md` before routing.

## 3) Power routing

### Input board

- Keep the 12V entry path compact: `J1 -> D1 -> F1 -> bulk/HF caps`.
- Place `C17` and `C6` adjacent to the 12V entry node. `C17` `+` mark on PCB silkscreen MUST match schematic (`+` to `12V_MAIN`).
- Place buck stage (`U4`, `L1`, `C19` bootstrap, `C18` + `C28` output bulk) as one tight cluster.
- Both `C18` and `C28` are required on `3V3` per AP63203 datasheet (review fix). Place them both close to `L1`/`U4` and parallel each other, not split across the board.
- Keep the buck hot loop (`C6/C17 -> U4 VIN/GND -> return`) and switch loop (`U4 SW -> L1 -> C18/C28 -> GND -> U4`) as short as possible.
- Keep the `SW` copper island compact and do not route sensitive traces under/near the switch node.

### Output board

Use this step-by-step order for first-pass success:

1. Route input protection first: `J1.1 -> VIN_12V_IN -> F9 -> D1 -> 12V_MAIN`. `F9` is the 8-pad ATO holder: pins 1-4 -> `VIN_12V_IN`, pins 5-8 -> `D1.A` (solder all 8 pads, join each blade's 4 pads). This trunk carries up to 16A (8x 2A channels) - size copper accordingly (wide pour / heavy copper / multiple vias to inner-layer pours).
2. Route buck cluster second: `U4`, `L1`, `C20` (bootstrap), `C19` + `C30` (3V3 output, both 22uF), `C29` (`U4.VIN` HF bypass), with `C17/C18` bulk return loop kept short.
3. Both `C19` and `C30` are required on `3V3` per AP63203 datasheet (review fix). Place them both close to `L1`/`U4` and parallel each other.
4. `C17`, `C18`, `C19`, `C30` polarity: `+` mark on PCB silkscreen MUST match schematic.
5. Build `12V_MAIN` high-current trunk copper next.
6. Route the shared load feed `12V_MAIN -> J6.1/J6.2 (both poles)` as wide copper (up to 16A), then CH0..CH7 output-power loops one channel at a time (low-side switch, fuse in the low-side leg):
   - `J5a/J5b.k (OUT_CHk) -> F1..F8 -> OUT_CHk_SW -> Q1..Q8.D`
   - `Q1..Q8.S -> LOAD_GND_RTN` (internal source-ground tie -> board `GND` -> returns to PSU via `J1.2`; not a terminal)
   - place flyback `D2..D9` directly across the load: anode at `OUT_CHk` (`J5` pad), cathode at `12V_MAIN` tie.
7. Route gate-control loops (`R1..R8`, `R9..R16`) after power loops are complete.
8. Route interfaces (`RS-485`, `USB`) only after high-current copper is stable.

Routing quality checks (output board):

- Keep MOSFET + flyback + terminal loops tight per channel.
- Keep `LOAD_GND_RTN` low-impedance; avoid narrow return bottlenecks that carry multi-channel current.
- Keep buck `SW` copper compact and isolated from USB/RS-485/MCU traces.

### Quantified current sizing (1 oz outer copper baseline)

Use these as minimum starting widths for <10 C estimated temperature rise, then validate with an IPC-2152 calculator for your exact stackup and ambient.

- Branch trace (one channel, up to 2 A): >=60 mil (1.5 mm) on outer layer.
- Shared rail up to 4 A: >=150 mil (3.8 mm) or copper pour with equivalent cross-section.
- Shared rail up to 8 A: >=300 mil (7.6 mm) copper pour.
- Main 12V trunk up to 16 A peak: use top+bottom pours in parallel with dense stitching vias every 5-10 mm along the trunk.
- Keep each 2 A branch length as short as practical (target <=50 mm from fuse to terminal).

Via current criteria (through-layer transitions in high-current paths):

- Assume ~1 A per 0.3 mm finished via as a conservative planning value.
- 2 A branch transition: >=3 vias in parallel.
- 4 A shared segment transition: >=5 vias in parallel.
- 8 A shared segment transition: >=9 vias in parallel.
- If current must neck down, do it only for very short segments (<5 mm).

Treat these as minima. If enclosure temperature or duty-cycle analysis indicates risk, increase copper width/area.

## 4) MCU + decoupling

- Place each 100nF decoupler at its IC supply pin with very short loop to GND.
- Keep VREF/analog decoupling traces short and quiet.
- Keep BOOT0 pulldown physically close to MCU pin.
- Keep NRST network (`SW1` + `NRST` net to `J6/J8`) close to MCU NRST pin with short SWD routing.

### Per-IC `1uF` VCC bypass placement (review fix)

In addition to the 100nF HF cap, every transceiver and Schmitt IC has a dedicated 1uF VCC bypass cap placed within ~3 mm of the IC's VCC pin. Both 100nF and 1uF caps are required - do not delete one to save the other:

- Input board:
  - `C29` adjacent to `U2A.VCC`
  - `C30` adjacent to `U2B.VCC`
  - `C31` adjacent to `U5.VCC`
  - `C32` adjacent to `U6.VCC`
- Output board:
  - `C31` adjacent to `U2A.VCC`
  - `C32` adjacent to `U2B.VCC`
  - `C33` adjacent to `U5.VCC`
  - `C34` adjacent to `U6.VCC`

Routing rule: the `1uF` bypass cap return path must reach local GND with a single short trace or via, ideally on the same layer as the IC pad.

## 5) RS-485 full-duplex layout

- Place both SP3485EN parts (`U2A`, `U2B`) close to the RS-485 terminal connector (`J4` input / `J2` output).
- Keep each differential pair (`TX+/-`, `RX+/-`) tightly coupled and length-matched within each pair.
- Place 120R termination (`R27` input / `R52` output) directly across receiver A/B pins.
- Place SM712 TVS (`D10/D11` input, `D18/D19` output) as close as practical to connector entry on each pair.
- Keep pair reference over continuous ground; avoid crossing plane gaps.
- Connector-side routing order: connector pins -> TVS -> transceiver/termination path.

Implementation details (pair ordering and connector pin numbers) are in the board appendices.

## 6) USB layout

- Route PA11/PA12 to USB connector as a coupled differential pair.
- Place 22R D+/D- resistors (`R31/R32` input, `R53/R54` output) near MCU pins.
- Keep noisy switching nodes (MOSFET drains, PTC branches) away from USB pair.
- Place USB ESD diode (`D12` input / `D20` output, `USBLC6-2SC6`) physically between the connector and the series resistors so the clamp is on the connector side of the impedance. Target <=5 mm from the connector pins (review fix U1/OP5).
- ESD diode VBUS leg must be wired to `USB_VBUS` and ground to local digital `GND` with very short return.

### USB physical constraints (FS USB)

- Target differential impedance: 90R +/-15% (board-house controlled impedance if available).
- Keep D+/D- total routed length <=50 mm on this form factor where possible.
- Keep intra-pair mismatch <=2 mm.
- Avoid pair stubs (no branch test pads on D+/D-). If test access is required, use in-line pads only.
- Use at most one via per line; avoid layer swaps unless mechanically required.
- USB ESD protection (`D12` / `D20`) is required on both boards (no longer optional). Route the IO1/IO2 stubs to the data lines as short stubs perpendicular to the pair without breaking pair coupling.
- Connector shield policy:
  - If enclosure/chassis ground exists, bond connector shell to chassis at entry.
  - If no dedicated chassis net exists, connect shell to PCB GND through RC bleed (`1 nF` in parallel with `1 Mohm`) and keep an optional `0R` DNI footprint for bring-up tuning.

## 7) RC + Schmitt input/override channels

Per-channel block:

- Place `R_pullup`, `R_series`, and `C` as a compact repeated cell.
- Place Schmitt ICs (`U5/U6`, SN74LV14A) near MCU-side routing transition.
- Keep channel routing topologically identical across channels where possible.
- Route external switch return currents from `IN_CHn_RAW`/`OVR_CHn_RAW` channel networks to ground without long shared narrow bottlenecks.

## 8) Output stage (output board)

- Keep each gate loop short (`MCU -> R_gate -> MOSFET gate -> R_pd -> GND`).
- Place each flyback diode (`D2..D9`) physically close to its load switching node and 12V rail tie point.
- Separate high di/dt output-current loops from MCU/USB/RS-485 regions.

### Thermal guidance (output stage + buck)

- Give each MOSFET drain/source node copper spreading area (target >=100 mm^2 combined local copper, both layers preferred with stitching vias).
- Keep PTCs thermally separated from each other and from MOSFET bodies (target >=3 mm edge-to-edge) to reduce thermal coupling/nuisance trips.
- Avoid placing temperature-sensitive logic components in the hot corridor formed by PTC + MOSFET rows.
- Keep buck power-stage thermal spreading on `U4` GND pad/return copper with dense vias to the opposite layer.
- Keep inductor (`L1`) and output capacitor (`C18` input board / `C19` output board) close to `U4` to minimize ripple current loop heating and EMI.
- If measured buck case/inductor temperature rise is high in enclosure testing, increase copper area and airflow margin before release.

## 9) Grounding and return paths

- Maintain a continuous ground reference under MCU, USB, and RS-485.
- Connect noisy power-return regions with low-impedance stitching to global ground.
- Avoid narrow necks in return path that force unrelated currents through one segment.

### RS-485 shield and chassis grounding policy

- Terminate cable shield/drain at connector entry to `CHASSIS` (or enclosure bond point), not deep in the board interior.
- Keep shield-to-chassis path short and wide; avoid routing shield current through digital ground traces.
- Provide a single-point `CHASSIS` to `GND` coupling near cable entry using RC (`4.7 nF` in parallel with `1 Mohm`) with optional `0R` DNI link for EMI bring-up experiments.
- Keep TVS return paths referenced to the same local ground/chassis strategy used at the connector.
- On output board, explicitly map `J2.6` (shield) to this entry strategy in the board appendix and keep this connection physically local to cable entry.

## 10) Debuggability and DFM

- Keep SWD header unobstructed by tall components.
- Expose NRST and BOOT0 controls without removing enclosure internals.
- Add test points for:
  - 12V, 3V3, GND
  - USART1 TX/RX logic nodes
  - RS-485 differential pairs (if space allows)
- Ensure silkscreen includes connector pin labels and channel numbering.
- Add at least 3 global fiducials (non-collinear, board corners) plus 1 local fiducial near fine-pitch MCU.
- Keep at least 3 mm component courtyard clearance around mounting holes and connector mechanical keepouts.
- Respect assembly height limits at enclosure boundaries (define and check max component height zones).
- If panelized, include tooling rails and breakaway/tab strategy that does not stress connector solder joints.

### Creepage/clearance and manufacturability minima (12V system)

- Electrical clearance minimum for unrelated low-voltage nets: >=0.5 mm; target >=1.0 mm where board area allows.
- Keep >=1.0 mm between high-current 12V copper and sensitive USB/MCU traces to reduce coupling and simplify rework.
- Keep >=2.0 mm between board edge and exposed copper unless edge-plated by design.
- Avoid acute copper slivers and neck-downs below fab capability; keep trace/space at or above chosen fab class with margin.

## 11) Board-specific execution references

Use the appendices to remove ambiguity during actual layout:

- `hardware/PCB_APPENDIX_INPUT.md`
  - Connector-edge placement map
  - Input channel block replication rules
  - Input-board net-class table and test matrix
- `hardware/PCB_APPENDIX_OUTPUT.md`
  - High-current corridor zoning and branch geometry
  - Output-stage thermal path and via fences
  - Output-board net-class table and test matrix

## 12) Pre-release layout checklist

- [ ] Power-entry polarity/protection path matches schematic, including output board `F9` ATO blade fuse holder in series before `D1` (pins 1-4 = `VIN_12V_IN`, pins 5-8 = `D1.A`; all 8 pads soldered, pins 1-4 not shorted to 5-8), and the 16A `VIN_12V_IN -> F9 -> D1 -> 12V_MAIN` trunk uses adequate copper width.
- [ ] Polarized cap silkscreen `+` marks match schematic on every electrolytic/tantalum cap (`C17` both boards, `C18` both boards, `C19` output, `C28` input, `C30` output).
- [ ] Both 22uF caps on `3V3` (`C18`+`C28` input / `C19`+`C30` output) are placed adjacent to `L1`/`U4` and routed as a parallel pair, not split.
- [ ] Every IC has local decoupling placed and routed (`C1`, `C2`, `C15` + all per-IC 100nF decouplers + the new per-IC 1uF caps `C29-C32` input / `C31-C34` output).
- [ ] RS-485 TVS (`D10/D11` input, `D18/D19` output) + termination (`R27` input, `R52` output) are on the correct pair and close enough.
- [ ] USB pair is short, coupled, and clear of noisy nodes.
- [ ] USB ESD device (`D12` input / `D20` output, `USBLC6-2SC6`) is within 5 mm of connector and clamps `USB_DP`/`USB_DM`/`USB_VBUS`. Shield grounding policy is implemented.
- [ ] BOOT0 defaults LOW and DFU button wiring (`SW2` + `SW1`) is correct.
- [ ] SWD header pinout/orientation (`J6` input / `J8` output) matches documentation.
- [ ] Output current paths meet quantified width/via targets for peak current.
- [ ] Thermal review completed for MOSFET/PTC row and buck power stage (`U4` + `L1`).
- [ ] RS-485 shield/chassis entry bond and CHASSIS-GND coupling are implemented per policy.
- [ ] Low-voltage clearance targets and board-edge copper setbacks are met.
- [ ] Global/local fiducials, height keepouts, and panelization constraints are checked.
- [ ] Mounting holes and connector clearances fit enclosure plan.
- [ ] Board-specific appendix checklist is fully completed and signed off.
- [ ] All `GND`, `3V3`, `12V_MAIN`, `VIN_12V_IN` symbols are Power Symbol Net Flags (no Net Ports). No `3.3V` / `+3.3V` aliases anywhere on schematic.
- [ ] DRC/ERC are clean (or waivers documented), and Gerber output review is completed before release.
