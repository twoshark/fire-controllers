# CAD verification

Printer bed ≤ **256 × 256 mm**. Wall **3.0 mm** nominal (2.5 min at thin features).

## Scorecard

| # | Question | Result | Notes |
| --- | --- | --- | --- |
| 1 | Envelope vs contents | **PASS*** | *verify EG STARTS Ø + C14 style |
| 2 | Heat-set inserts | **PASS** | M2 under PCB (100 mil holes); M3 lids/RS-15 — [`MOUNTING.md`](MOUNTING.md) |
| 3 | Pillar locations | **PASS** | Gerber Ø2.54 XY locked; 4 bosses/board (stable set) |
| 4 | Layout / wiring | **PASS** | Face map + proximity rules |
| 5 | Build plate | **PASS** | Largest **220×180**; brim OK |
| 6 | Supports | **PASS** | Body open-up; lid groove up; chamfered pockets |

## Envelope sizes

Inner ≈ outer − **6** (2×3 mm walls).

| Box | Outer L×W×H | Inner | Why |
| --- | ---: | ---: | --- |
| sign-input | **220 × 180 × 90** | 214×174×84 | PCB outline 83×79 (KO 86×83) + RS-15 + hex ~110×100 |
| mp-input | **200 × 160 × 90** | 194×154×84 | Same guts, 4 buttons |
| sign-output | **220 × 170 × 85** | 214×164×79 | PCB outline 124×112 (KO 126×114) + DTP/SOL pockets |
| mp-output | **220 × 170 × 85** | 214×164×79 | Same |

12 V supply is OTS **HLG-240H-12** — no printed power shell ([`POWER_OTS.md`](POWER_OTS.md)).

## Inserts

| Insert | Use | Print hole | Boss OD |
| --- | --- | ---: | ---: |
| M2×4–5 heat-set | PCB (hole Ø2.54 / pad Ø4.06) | ~**3.2** | **≥7** |
| M3×5.7 heat-set | Lid, RS-15 | **Ø4.2** | **≥9** |

## Layout / wiring

| Rule | Value |
| --- | --- |
| PCB LED edge | faces LED-window wall |
| Terminal → panel | ≤80 mm preferred; ≤120 mm max |
| DTP on output | short-end · CL **40** · mates HLG pigtail |
| SOL row | same face as `J5`/`J6` |
| Output kill | unplug HLG AC |
| Service | lid open → USB-C, SWD, `F9`, SW; standoff **≥12** |

## Build / DFM

Print body **open face up**. Lid **gasket groove up**.

| Feature | Strategy |
| --- | --- |
| Lid gasket groove | Groove up — no supports |
| Connector pocket lip | 45° chamfer or separate clip |
| Screw bosses | Taper 2° |
| Arcade underside | Holes vertical in lid |

PETG or ABS · 0.2 mm · 3–4 perimeters · 25–40% infill.

## Verify before freeze

| Item | Action |
| --- | --- |
| EG STARTS ring | Measure Ø (28 vs 24) |
| C14 pack | Snap vs flange |
| DTP / DT housing | Measure for pocket CAD |
| PCB mount XY | **DONE** — Gerber 2026-07-18; see [`MOUNTING.md`](MOUNTING.md) |
