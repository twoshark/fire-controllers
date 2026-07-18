# CAD verification (v1.0.0 enclosures)

Printer bed ≤ **256 × 256 mm**. Wall **3.0 mm** nominal (2.5 min at thin features).

Audit date 2026-07-18. Fixes below are **locked into CAD notes** after FAIL items.

---

## Scorecard (after size/mount fixes)

| # | Question | Result | Notes |
| --- | --- | --- | --- |
| 1 | Envelope vs contents | **PASS*** | Power boxes grown; mp-output widened; *verify EG STARTS Ø + C14 style |
| 2 | Heat-set inserts | **PASS** | M3×5.7 Ruthex-class specified + shopping line |
| 3 | Pillar locations | **PASS** | Relative patterns in [`MOUNTING.md`](MOUNTING.md) |
| 4 | Layout / wiring | **PASS** | Face map + terminal proximity rules |
| 5 | Build plate | **PASS** | Largest body **256×200**; brim ≤4 mm or none |
| 6 | Supports | **PASS** (DFM) | Print rules: body open-up; lid groove up; pocket lips chamfered |

---

## 1. Envelope sizes (locked)

Inner ≈ outer − **6** (2×3 mm walls).

| Box | Outer L×W×H | Inner | Why |
| --- | ---: | ---: | --- |
| sign-input | **220 × 180 × 90** | 214×174×84 | PCB 86×83 + RS-15 62.5×51 + hex ~110×100 |
| mp-input | **200 × 160 × 90** | 194×154×84 | Same guts, 4 buttons |
| sign-output | **220 × 170 × 85** | 214×164×79 | PCB 126×114 + DTP/SOL pocket intrusion |
| mp-output | **220 × 170 × 85** | 214×164×79 | Was 200×150 — too tight after pockets |
| sign-output-power | **256 × 200 × 100** | 250×194×94 | LRS 215 + terminal wire + DTP; bed-max L |
| mp-output-power | **256 × 200 × 100** | 250×194×94 | Same |

### Power-box stack-up (critical)

```text
Outer L = 256
Inner L = 250
LRS length = 215
Remaining = 35  → all at TERMINAL end of LRS (≥30 wire bend)
Far end clearance ≥ 5 (LRS body to wall)

C14 + POWER on LONG wall near terminal end (not on short ends)
DTP mate on SHORT end opposite terminals (pocket into wall ≤15)
Fans recessed in LONG walls (body mostly in wall pocket; ≤10 into cavity)
```

Fan flush rule: usable W ≥ 115 + 20 = 135; have **194** → OK.

---

## 2–3. Inserts & pillars

See [`MOUNTING.md`](MOUNTING.md).

| Insert | Use | Print hole | Boss OD | Qty (all boxes) |
| --- | --- | ---: | ---: | ---: |
| M3×5.7 heat-set | PCB clamps, lid, RS-15 adapters | **Ø4.2** | **≥9** | ~40 |
| M4×6 heat-set *or* M4 through-bolt | LRS / Al plate | **Ø5.6** / clearance Ø4.5 | **≥11** | 8 (2 boxes×4) |

---

## 4. Layout / wiring rules

| Rule | Value |
| --- | --- |
| PCB LED edge | faces LED-window wall; window CL aligns to LED column mid |
| Terminal → panel | ≤80 mm wire run preferred; ≤120 mm max |
| DTP mate faces | opposing short ends · CL **40** · boxes face each other |
| SOL row | same face as `J5`/`J6` edge of PCB |
| Output boxes | no local rocker; short DTP→`J1` 12 AWG |
| Service | lid open → USB-C, SWD, `F9`, on-board SW; standoff **≥12** above tallest part under lid plane |

---

## 5. Build plate

| Part | Footprint | Fit |
| --- | ---: | --- |
| Power body | 256×200 | flush to bed L — **no brim** on L, or rotate 90° if brim needed |
| Power lid | ≤256×200 | separate print |
| Others | ≤220×180 | OK with 5 mm brim |

Print body **open face up**. Lid **gasket groove up**.

---

## 6. Supports / DFM

| Feature | Strategy |
| --- | --- |
| Lid gasket groove | Print groove facing up — **no supports** |
| Connector pocket lip | 45° chamfer retention or separate clip ring — avoid horizontal lip overhang |
| Screw bosses | Taper 2°; bridge OK under 5 mm |
| Arcade underside | Print top as lid with holes; buttons installed after — no supports if holes vertical |
| Fan duct | Print with axis vertical or 45° chamfer |

Material assumption: PETG or ABS · 0.2 mm layers · 3–4 perimeters · 25–40% infill · 5 top/bottom.

---

## Still verify before freeze (parts in hand)

| Item | Action |
| --- | --- |
| EG STARTS adapter ring | Measure hole Ø (28 vs 24) |
| C14 Amazon pack | Snap vs flange cutout |
| DTP / DT housing | Measure for pocket CAD |
| LRS unit | Confirm mount hole span 150 / end 32.5 |
