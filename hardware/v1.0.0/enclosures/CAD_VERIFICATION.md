# CAD verification

Printer bed ≤ **256 × 256 mm**. Wall **3.0 mm** nominal (2.5 min at thin features).

Axis: outer **L × W × H** = **+X × +Y × +Z** (right × back × up). FRONT = LED wall.

## Scorecard

| # | Question | Result | Notes |
| --- | --- | --- | --- |
| 1 | Envelope vs contents | **PASS*** | Floor pack OK; arcade = compact hex ring; *measure EG STARTS Ø + DT/DTP housings |
| 2 | Heat-set inserts | **PASS** | M2 PCB; M3 lids + RS-15 — [`MOUNTING.md`](MOUNTING.md) + cart |
| 3 | Pillar locations | **PASS** | PCB H1–H4, RS-15 M3, lid M3, arcade centers — [`MOUNTING.md`](MOUNTING.md) |
| 4 | Layout | **PASS** | LED→FRONT; M12 LEFT; SOL LEFT; DTP BACK; service under lid |
| 5 | Wiring | **PASS** | Faces match terminal mids; star `J6`; ≤80 mm preferred |
| 6 | Build plate | **PASS** | Largest **220×180** |
| 7 | Supports / DFAM | **PASS*** | *DT/DTP = **separate retainer clips** (default); lid groove-up; body open-up |

## Envelope sizes

Inner ≈ outer − **6** (2×3 mm walls).

| Box | Outer L×W×H | Inner | Floor pack |
| --- | ---: | ---: | --- |
| sign-input | **220 × 180 × 90** | 214×174×84 | RS-15 LEFT + PCB `(75,40)` + hex lid |
| mp-input | **200 × 160 × 90** | 194×154×84 | RS-15 LEFT + PCB `(70,30)` |
| sign-output | **220 × 170 × 85** | 214×164×79 | PCB `(25,25)` + DTP BACK + SOL LEFT |
| mp-output | **220 × 170 × 85** | 214×164×79 | Same · SOL0..2 |

12 V supply is OTS **HLG-240H-12** — [`POWER_OTS.md`](POWER_OTS.md).

## Inserts

| Insert | Use | Print hole | Boss OD |
| --- | --- | ---: | ---: |
| M2×4–5 heat-set | PCB (Ø2.54) · buttons | ~**3.2** | **≥7** |
| M3×5.7 heat-set | Lid · RS-15 floor | **Ø4.2** | **≥9** |

## Layout / wiring

| Rule | Value |
| --- | --- |
| PCB LED edge | FRONT |
| Input M12 | **LEFT** (near `CN2`) |
| Input C14 + KCD4 | LEFT (near RS-15) |
| Output DTP | **BACK** (near `J1`) |
| Output SOL row | **LEFT** (near `J5`/`J6`) |
| Output M12 | **LEFT** near front (near `J2`) |
| Terminal → panel | ≤80 mm preferred; ≤120 mm max |
| Service | lid open → USB-C, SWD, `F9`, SW; standoff **≥12** |
| Output kill | unplug HLG AC |

## Build / DFAM

| Part | Print |
| --- | --- |
| Body | Open face **up** · no supports |
| Lid | Gasket groove **up** · arcade holes vertical |
| DT/DTP retainers | **Separate clips** (default) — do not rely on unsupported pocket lips |
| Screw bosses | Taper 2° |
| Optional | Chamfered integral pocket only after print test |

PETG or ABS · 0.2 mm · 3–4 perimeters · 25–40% infill. Brim OK ≤5 mm.

## Verify before cutout freeze

| Item | Action |
| --- | --- |
| EG STARTS ring | Measure Ø (28 vs 24) |
| C14 pack | Snap vs flange |
| DT / DTP housing | Measure for clip CAD |
| RS-15 hole inset | Confirm 11.5 / 39.1 on your unit |
