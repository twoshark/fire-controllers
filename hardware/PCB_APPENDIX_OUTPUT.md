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
| Power-entry zone | `J1`, `D1`, `C17`, `C18`, `C29`, buck chain (`U4`, `L1`, `C20`, `C19`) | Keep `VIN_12V_IN -> D1 -> 12V_MAIN` compact; keep buck `SW` loop short. |
| High-current output zone | `Q1..Q8`, `F1..F8`, flyback diodes, `J5a/J5b`, `J6` | Dedicated corridor; keep away from USB and MCU. |
| Override-input zone | `J3a/J3b`, `J4`, RC cells, `U5/U6` | Uniform per-channel topology. |
| RS-485 zone | `J2`, `U2A/U2B`, `R52`, 2x SM712 | Keep near cable entry. |
| USB/debug zone | `J7`, D+/D- resistors, `J8`, `SW1/SW2` | Keep accessible near board edge. |
| MCU core zone | `U1` + decoupling | Between noisy/high-current and sensitive interface zones. |
| LED edge zone | `LED1..LED10` + resistors | Single edge-aligned status window. |

## 2) Net-class and routing rules (output board)

| Net class | Nets | Min width | Min clearance | Via guidance |
| --- | --- | --- | --- | --- |
| Main high-current trunk | `12V_MAIN`, `LOAD_12V`, `LOAD_GND_RTN` | 300 mil pour-equivalent for up to 16A peak | >=1.0 mm from USB/MCU traces | Dense stitching vias every 5-10 mm if both layers used. |
| Per-channel branches | `F1..F8` to channel load path (2A/channel) | 60 mil | >=1.0 mm from sensitive traces | >=3 vias in parallel at layer transitions. |
| Mid-current shared branches | shared 4A to 8A segments | 150 mil to 300 mil | >=1.0 mm | >=5 vias (4A), >=9 vias (8A). |
| Logic 3V3 | GPIO, Schmitt, LED controls | 8 mil | 0.5 mm | Standard 0.3 mm via allowed. |
| USB pair | `USB_DP`, `USB_DM` | per impedance target | keep clear of high-current zone | Max one via/line. |
| RS-485 pairs | `RS485_TX+/-`, `RS485_RX+/-` | 10 mil baseline | >=1.0 mm from high di/dt nodes | Length match within each pair. |

## 3) Output-stage routing constraints

Per channel loop:

```text
12V_MAIN -> PTC(F1..F8) -> J5a/J5b channel pin -> external load -> OUT_CHn_SW -> Q1..Q8 -> LOAD_GND_RTN
```

Rules:

- Keep each channel loop compact and independent.
- Place flyback diode close to switched node and local 12V tie.
- Keep gate loop (`MCU -> Rg -> gate -> Rpd -> GND`) short and quiet.
- Avoid routing logic traces through MOSFET/PTC hot corridor.

## 3A) Output-board routing execution order (step-by-step)

Route in this exact order:

1. Route power-entry protection: `J1.1 -> VIN_12V_IN -> D1 -> 12V_MAIN`.
2. Route buck cluster first: `U4`, `L1`, `C20` (BST-SW), `C19` (3V3 output), `C29` (`U4.VIN` HF bypass), then `C17/C18` bulk path.
3. Create `12V_MAIN` trunk copper (top+bottom if needed) and add stitching vias every 5-10 mm on shared high-current sections.
4. Route each channel power path in order CH0..CH7:
   - `12V_MAIN -> F1..F8 -> J5a/J5b pin`
   - `OUT_CHn_SW -> Q1..Q8`
   - `Q1..Q8 source -> LOAD_GND_RTN`
5. Route gate-drive loops (`GATE_CHn -> R1..R8 -> Q gate`) and pulldowns (`R9..R16 -> GND`) only after power loops are complete.
6. Route RS-485 (`J2` + `U2A/U2B` + `R52` + `D18/D19`) with pair coupling preserved.
7. Route USB (`J7` + `R53/R54` + `R55/R56`) with the pair over continuous ground.
8. Route remaining low-speed logic (override conditioning, LEDs, SWD).
9. Refill pours, then run DRC and net-connectivity review.

## 3B) Ground and return strategy (output board)

- Keep one continuous logic/reference `GND` under MCU, USB, and RS-485.
- Keep `LOAD_GND_RTN` as a low-impedance high-current return path tied into board `GND` with wide copper (avoid narrow necks).
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

1. Validate `VIN_12V_IN`, `12V_MAIN`, and `3V3`.
2. Verify BOOT0/NRST behavior and SWD programming access.
3. Confirm receive path (`USART1_RX`) with input-board traffic.
4. Confirm heartbeat transmit (`USART1_TX`) at 10Hz frame cadence.
5. Force failsafe and verify all `GATE_CHn` and `OUT_CHn` go OFF.
6. Validate override priority channel-by-channel.
7. Validate output LEDs and link LED fault/healthy indications.

## 8) Output-board layout signoff checklist

- [ ] Stackup choice is locked and documented for this board.
- [ ] Buck cluster (`U4`, `L1`, `C20`, `C19`, `C29`) is compact and isolated from sensitive routing.
- [ ] `J1 -> D1 -> 12V_MAIN` protection path is routed exactly as intended.
- [ ] `12V_MAIN`, `LOAD_12V`, and `LOAD_GND_RTN` copper meets width/via rules from Section 2.
- [ ] CH0..CH7 mapping is consistent (`F1..F8`, `Q1..Q8`, `D2..D9`, `J5a/J5b` channel pins).
- [ ] `R52` is only on RX pair; `D18/D19` are at connector entry.
- [ ] USB pair meets impedance/length/mismatch limits and avoids high-current corridor.
- [ ] `J2.6` shield/chassis handling is implemented per policy.
- [ ] DRC/ERC are clean (or waivers documented), and Gerber review is complete before release.
