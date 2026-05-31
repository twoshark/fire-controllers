# Output Board PCB Appendix (Executable Constraints)

This appendix defines board-specific placement/routing/test constraints for the output board.

## 0) Pre-layout setup (output board)

Complete these setup steps before placing components:

1. Lock board stackup choice for this board (`2-layer` or `4-layer`) and record the choice in the layout release notes.
2. Configure net classes in CAD to match Section 2 of this appendix before routing.
3. Load DRC rules (min clearance/trace/via) to match the selected fab class and this appendix table.
4. Run one initial DRC on the empty board + footprints to catch rule or footprint errors before routing.

## 1) Placement zone map (output board)

| Zone | Required content | Notes |
| --- | --- | --- |
| Power-entry zone | `J1` (`KF128-7.5-2P`, 7.5mm pitch / 24A - larger footprint than 5.08mm terminals), `F9` (ATO blade fuse holder `178.6165.0002`, in series - large 8-pad THT footprint, pins 1-4=`VIN_12V_IN` / pins 5-8=`D1.A`), `D1`, `C17`, `C18` (both polarized), `C29`, buck chain (`U4`, `L1`, `C20`, `C19`, `C30`) | Reserve area for the larger `J1` 7.5mm terminal and the `F9` THT holder. Keep `VIN_12V_IN -> F9 -> D1 -> 12V_MAIN` compact and wide (16A trunk); keep buck `SW` loop short. `C17`, `C18`, `C19`, `C30` `+` to higher rail. `C19` and `C30` are both 22uF on `3V3` (review fix). |
| High-current output zone | `Q1..Q8`, `F1..F8`, flyback diodes, `J5a/J5b`, `J6` (`KF128-7.5-2P`, 7.5mm / 24A - larger footprint; both poles are `12V_MAIN`, the +12V load feed, up to 16A total) | Dedicated corridor; keep away from USB and MCU. Route `12V_MAIN` (including both `J6` poles) and the internal `LOAD_GND_RTN` (MOSFET-source ground back to `J1.2`) with 16A-rated copper. |
| Override-input zone | `J3a/J3b`, `J4`, RC cells, `U5/U6`, `C33` (`U5.VCC` 1uF), `C34` (`U6.VCC` 1uF) | Uniform per-channel topology. 1uF cap adjacent to each Schmitt VCC pin. |
| RS-485 zone | `J2`, `U2A/U2B`, `R52`, 2x SM712, `C31` (`U2A.VCC` 1uF), `C32` (`U2B.VCC` 1uF) | Keep near cable entry. 1uF cap adjacent to each transceiver VCC pin. |
| USB/debug zone | `J7`, `D20` (USB ESD), D+/D- resistors, `J8`, `SW1/SW2` | Keep accessible near board edge. `D20` within 5 mm of `J7`. |
| MCU core zone | `U1` + decoupling | Between noisy/high-current and sensitive interface zones. |
| LED edge zone | `LED1..LED10` + resistors | Single edge-aligned status window. |

## 2) Net-class and routing rules (output board)

| Net class | Nets | Min width | Min clearance | Via guidance |
| --- | --- | --- | --- | --- |
| Main high-current trunk | `12V_MAIN` (incl. `J6` load feed), `LOAD_GND_RTN` | 300 mil pour-equivalent for up to 16A peak | >=1.0 mm from USB/MCU traces | Dense stitching vias every 5-10 mm if both layers used. |
| Per-channel branches | `F1..F8` to channel load path (2A/channel) | 60 mil | >=1.0 mm from sensitive traces | >=3 vias in parallel at layer transitions. |
| Mid-current shared branches | shared 4A to 8A segments | 150 mil to 300 mil | >=1.0 mm | >=5 vias (4A), >=9 vias (8A). |
| Logic 3V3 | GPIO, Schmitt, LED controls | 8 mil | 0.5 mm | Standard 0.3 mm via allowed. |
| USB pair | `USB_DP`, `USB_DM` | per impedance target | keep clear of high-current zone | Max one via/line. |
| RS-485 pairs | `RS485_TX+/-`, `RS485_RX+/-` | 10 mil baseline | >=1.0 mm from high di/dt nodes | Length match within each pair. |

## 3) Output-stage routing constraints

Per channel loop (low-side switch, shared +12V feed). High side is common: `12V_MAIN -> J6.1/J6.2 (both poles)`. The per-channel fuse is in the LOW-side leg, between the `J5` terminal and the MOSFET drain:

```text
12V_MAIN -> J6.1/J6.2 (both poles) -> external load -> J5a/J5b.k (OUT_CHk) -> F(k) -> OUT_CHk_SW (Q.D) -> Q(k) -> Q.S -> LOAD_GND_RTN (internal source ground -> board GND -> J1.2)
flyback D(k): anode = OUT_CHk (J5 terminal) ; cathode = 12V_MAIN   (directly across the load)
```

Rules:

- Keep each channel loop compact and independent.
- Place the flyback diode across the load terminal: anode at the `OUT_CHk` (`J5`) pad, cathode at the `12V_MAIN` tie, so freewheel current bypasses the fuse and MOSFET.
- The per-channel fuse sits between the `J5` terminal (`OUT_CHk`) and the MOSFET drain (`OUT_CHk_SW`), not in the shared `12V_MAIN` feed.
- Keep gate loop (`MCU -> Rg -> gate -> Rpd -> GND`) short and quiet.
- Avoid routing logic traces through MOSFET/fuse hot corridor.

## 3A) Output-board routing execution order (step-by-step)

Route in this exact order:

1. Route power-entry protection: `J1.1 -> VIN_12V_IN -> F9 -> D1 -> 12V_MAIN`. `F9` is the ATO blade fuse holder in series before `D1` (review fix OP1). Wire `F9` pins 1-4 (input blade) to `VIN_12V_IN` and pins 5-8 (output blade) to `D1.A`; solder and join all four pads of each blade contact with wide copper (they share the 16A current). This is the 16A board trunk - route wide/heavy copper.
2. Route buck cluster: `U4`, `L1`, `C20` (BST-SW), `C19` + `C30` (3V3 output, both 22uF in parallel - review fix OP3), `C29` (`U4.VIN` HF bypass), then `C17/C18` bulk path. Verify `C17`, `C18`, `C19`, `C30` `+` polarity to higher rail (review fix OP2).
3. Create `12V_MAIN` trunk copper (top+bottom if needed) and add stitching vias every 5-10 mm on shared high-current sections.
4. Route the shared load feed `12V_MAIN -> J6.1/J6.2 (both poles)` as wide copper (carries up to 16A).
5. Route each channel power path in order CH0..CH7:
   - `J5a/J5b.k (OUT_CHk) -> F(k) -> OUT_CHk_SW`
   - `OUT_CHk_SW -> Q(k).D`; `Q(k).S -> LOAD_GND_RTN` (internal source ground -> board `GND` -> `J1.2`)
   - flyback `D(k)`: anode at `OUT_CHk` (`J5` pad), cathode at `12V_MAIN`
6. Route gate-drive loops (`GATE_CHn -> R1..R8 -> Q gate`) and pulldowns (`R9..R16 -> GND`) only after power loops are complete.
7. Route RS-485 (`J2` + `U2A/U2B` + `R52` + `D18/D19`) with pair coupling preserved. Place `C31` (1uF) directly at `U2A.VCC` and `C32` (1uF) at `U2B.VCC` (review fix OP4).
8. Route USB (`J7` + `R53/R54` + `R55/R56`) with the pair over continuous ground. Insert `D20` (USB ESD, `USBLC6-2SC6`) within 5 mm of `J7` on the cable side of `R53`/`R54` (review fix OP5).
9. Route remaining low-speed logic (override conditioning with `C33`/`C34` 1uF caps at `U5.VCC`/`U6.VCC`, LEDs, SWD).
10. Refill pours, then run DRC and net-connectivity review.

## 3B) Ground and return strategy (output board)

- Keep one continuous logic/reference `GND` under MCU, USB, and RS-485.
- Keep `LOAD_GND_RTN` (the MOSFET-source ground tie; internal, not a terminal) as a low-impedance high-current return path into board `GND`, carrying the load return back to the PSU negative at `J1.2`. Use wide copper (avoid narrow necks).
- Do not route high-current return through thin digital-ground traces.
- Keep RS-485 and USB return paths local to their interfaces; do not force them through MOSFET/PTC current corridors.

## 4) Thermal execution checklist (output board)

- Provide >=100 mm^2 combined local copper (both layers preferred) around each MOSFET drain/source network.
- Keep PTCs at least 3 mm apart edge-to-edge.
- Keep `U4` GND return copper wide with dense stitching vias to improve buck thermal spread.
- Keep `L1`, bootstrap cap, and output cap tight to `U4` to control switching loop heating.
- If enclosure testing shows excessive regulator or inductor temperature rise, increase copper area/airflow margin before release.

## 5) RS-485 and USB constraints (output board)

- `J2.1/J2.2` = TX pair, `J2.3/J2.4` = RX pair, `J2.5` GND, `J2.6` shield.
- `R52` 120R only on RX pair.
- Place `D18` and `D19` (SM712) at connector entry and keep TVS return path short.
- Keep routing order at connector as: connector pins -> TVS (`D18/D19`) -> transceiver/termination path.
- USB pair targets:
  - `90R +/-15%` differential impedance
  - <=50 mm routed length
  - <=2 mm mismatch
  - no branch stubs
- USB ESD (`D20` = `USBLC6-2SC6`) is REQUIRED on this board (review fix OP5). Place within 5 mm of `J7` connector pins, on cable side of `R53`/`R54`. `D20.VBUS` must be wired to `USB_VBUS`, `D20.GND` to local digital `GND` with very short return.

Shield/chassis policy execution:

- Land `J2.6` shield at connector entry and route to `CHASSIS` entry strategy per top-level guide.
- Keep `CHASSIS`-to-`GND` RC coupling physically near cable-entry area.

## 6) Test-point matrix (output board)

| Test point | Net | Expected state |
| --- | --- | --- |
| TP1 | `VIN_12V_IN` | 12V nominal |
| TP2 | `12V_MAIN` | 12V bulk rail |
| TP3 | `3V3` | 3.3V logic rail |
| TP4 | `GND` | 0V |
| TP5 | `USART1_RX` | valid state frame stream |
| TP6 | `USART1_TX` | heartbeat frame stream |
| TP7 | `RS485_RX+/-` | differential serial activity |
| TP8 | `RS485_TX+/-` | differential serial activity |
| TP9 | `PA14-BOOT0` | LOW default |
| TP10 | `NRST` | HIGH default |
| TP11 | `GATE_CH0..CH7` | follows serial/override/failsafe logic |
| TP12 | `OUT_CH0..CH7` | switched low-side outputs |

## 7) Bring-up probe order (output board)

1. Before applying power: visually verify polarity of `C17`, `C18`, `C19`, `C30` against silkscreen `+` mark (review fix OP2). Stop and rework if any is reversed.
2. Confirm `F9` holder is populated (all 8 pads soldered: pins 1-4 on `VIN_12V_IN`, pins 5-8 on `D1.A`), the 25A ATO blade fuse is inserted, and the path is not bypassed (review fix OP1).
3. Validate `VIN_12V_IN`, `12V_MAIN`, and `3V3`.
4. Verify BOOT0/NRST behavior and SWD programming access.
5. Confirm receive path (`USART1_RX`) with input-board traffic.
6. Confirm heartbeat transmit (`USART1_TX`) at 10Hz frame cadence.
7. Force failsafe and verify all `GATE_CHn` and `OUT_CHn` go OFF.
8. Validate override priority channel-by-channel.
9. Validate output LEDs and link LED fault/healthy indications.

## 8) Output-board layout signoff checklist

- [ ] Stackup choice is locked and documented for this board.
- [ ] Buck cluster (`U4`, `L1`, `C20`, `C19`, `C30`, `C29`) is compact and isolated from sensitive routing. Both 22uF on `3V3` (`C19`+`C30`) are placed in parallel adjacent to `L1`/`U4`.
- [ ] `J1 -> F9 -> D1 -> 12V_MAIN` protection path is routed exactly as intended.
- [ ] `J1` and `J6` use the `KF128-7.5-2P` (7.5mm, 24A) footprint, not the 5.08mm `DB128V` footprint, and both poles of each are on 16A-rated copper.
- [ ] `F9` ATO blade fuse holder (`178.6165.0002`, 22.5A cont.) is in series with `VIN_12V_IN` before `D1` (pins 1-4 = `VIN_12V_IN`, pins 5-8 = `D1.A`; all 8 pads soldered, each blade's 4 pads joined; pins 1-4 not shorted to 5-8), fitted with a 25A ATO blade fuse, and its 8-pad THT footprint/clearance is correct.
- [ ] Polarized cap silkscreen `+` marks match schematic (`C17`, `C18`, `C19`, `C30`).
- [ ] Per-IC 1uF VCC bypass caps placed within ~3 mm of each IC: `C31` at `U2A`, `C32` at `U2B`, `C33` at `U5`, `C34` at `U6`.
- [ ] `12V_MAIN` (incl. `J6` load feed) and `LOAD_GND_RTN` copper meets width/via rules from Section 2.
- [ ] CH0..CH7 mapping is consistent (`F1..F8`, `Q1..Q8`, `D2..D9`, `J5a/J5b` channel pins).
- [ ] `R52` is only on RX pair; `D18/D19` are at connector entry.
- [ ] USB pair meets impedance/length/mismatch limits and avoids high-current corridor. `D20` is within 5 mm of `J7` and clamps `USB_DP`/`USB_DM`/`USB_VBUS`.
- [ ] `J2.6` shield/chassis handling is implemented per policy.
- [ ] All `GND`, `3V3`, `12V_MAIN`, `VIN_12V_IN` symbols are Power Symbol Net Flags (no Net Ports). No `3.3V` / `+3.3V` aliases anywhere on schematic.
- [ ] DRC/ERC are clean (or waivers documented), and Gerber review is complete before release.

## 9) Review-fix component placement summary (output board)

| New designator | Required placement |
| --- | --- |
| `F9` | ATO blade fuse holder (THT, 8-pad footprint) in the power-entry zone: pins 1-4 -> `VIN_12V_IN`, pins 5-8 -> `D1.A`. Solder all 8 pads; join each blade contact's 4 pads with 16A trunk copper. |
| `C30` | Adjacent to `C19`, both 22uF on `3V3` near `L1`/`U4` buck output |
| `C31` | Within ~3 mm of `U2A.VCC` pin |
| `C32` | Within ~3 mm of `U2B.VCC` pin |
| `C33` | Within ~3 mm of `U5.VCC` pin |
| `C34` | Within ~3 mm of `U6.VCC` pin |
| `D20` | Within 5 mm of `J7` USB-C pins, on cable side of `R53`/`R54` |
