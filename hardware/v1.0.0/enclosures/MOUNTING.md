# Mounting — inserts, pillars, bosses

Boards have **no mounting holes** (v1.0.0). Use **corner clamp pillars** outside the keep-out that bear on the PCB top with M3 screws into heat-set inserts in the floor bosses.

Shopping: M3×5.7 + M4×6 heat-set packs — [`SHOPPING_LIST.md`](SHOPPING_LIST.md).

---

## Heat-set inserts

| Spec | M3 | M4 |
| --- | --- | --- |
| Example | Ruthex / CNC Kitchen M3×5.7 | M4×6–8 |
| Print hole Ø | **4.2** | **5.6** |
| Insert length | 5.7 | 6–8 |
| Boss outer Ø | **≥9** | **≥11** |
| Boss height above floor | **≥6** | **≥8** |
| Local wall around boss | ≥1.5 | ≥2.0 |
| Screw | M3×8–10 pan | M4×10–12 |

Install with soldering iron / insert tip perpendicular; allow cool before load.

---

## Coordinate system (all boxes)

```text
Origin = inner floor, front-left corner (inside wall)
+X = toward right (along width)
+Y = toward back (along length)
+Z = up
FRONT = LED window wall (controller boxes) or intake wall (power)
```

PCB keep-out rectangles are placed with **LED edge parallel to FRONT** (+X along LED column).

---

## Input PCB (sign-input / mp-input)

Keep-out **86 × 83** (X × Y). Place with margin ≥10 from walls.

| Boss | Role | Pattern |
| --- | --- | --- |
| C1..C4 | Corner clamps | At keep-out corners **outward** 4 mm (centers outside copper) |
| Clamp finger | M3 screw + printed washer/clip | Bears on PCB top copper-free margin |

Suggested keep-out origin (inner, mm) — **sign-input**:

| Corner | X | Y |
| --- | ---: | ---: |
| KO front-left | 20 | 40 |
| KO size | 86 | 83 |

Boss centers = KO corners ± (outward 6 mm along both axes as needed).

**RS-15-12** (62.5 × 51 × 28): place near LEFT (AC) wall. Datasheet 2× M3 on chassis — bosses match RS-15 bottom holes (measure unit; typ. along 62.5 length). Standoff height **Z = 3** (or direct to Al/pad). Clearance above RS-15 to lid ≥10.

**Buttons daughter**: 2× M3 bosses under top lid or on standoffs from floor; flying leads to `J2`/`J3`.

**Standoff height (PCB top from floor)**: **12 mm** min under PCB for wire dress; USB-C/SWD face open toward lid volume.

---

## Output PCB (sign-output / mp-output)

Keep-out **126 × 114**.

| Item | Rule |
| --- | --- |
| LED edge | FRONT |
| `J5`/`J6` edge | toward SOL face |
| `J1` edge | toward DTP mate face |
| Corner clamps | 4× M3 same as input |
| `F9` access | lid open · no pillar over fuse |

Suggested keep-out origin — **sign-output** (inner mm):

| | X | Y |
| --- | ---: | ---: |
| KO front-left | 25 | 25 |
| KO size | 126 | 114 |

PCB standoff **Z = 12**.

---

## LRS-200-12 + Al plate (power boxes)

Al plate **215 × 115 × 3** on floor bosses.

| Feature | Spec |
| --- | --- |
| Plate holes | Match LRS bottom: span along L typ. **150**, end offset typ. **32.5** — **verify on unit** |
| Fasteners | M4 through plate into heat-set in floor **or** M4 into LRS (L≤5 into PSU) |
| Plate position | Terminals toward LONG-wall AC (C14/POWER); far short end = DTP mate |
| Gap terminal end | **≥30** inner to wall for AC/DC wire |
| Gap DTP end | ≥5 body + pocket in wall |

Floor bosses: 4× under plate holes.

---

## Fans (power boxes)

| Spec | Value |
| --- | --- |
| Pattern | 50 × 50 · Ø3.5 through · M3 screws into heat-set **or** nuts |
| Airflow hole | Ø57 |
| CL height | **48** from outer bottom |
| Recess | Fan body in wall pocket; ≤10 into cavity |
| Seal | Foam between filter/grill and outer face |

---

## Lid fasteners (all boxes)

| Spec | Value |
| --- | --- |
| Count | **4** (corners) or **6** (power boxes) |
| Insert | M3×5.7 in body flange |
| Screw | M3×10–12 |
| Gasket | 2–3 mm silicone cord/strip in lid groove · 20–30% compression |
| Groove | 3.5 W × 2.0 D (tune to gasket) · print face up |

---

## Qty checklist (shopping)

| Insert | Approx qty |
| --- | ---: |
| M3×5.7 | 50 (PCBs + lids + RS-15 + fans + spare) |
| M4×6–8 | 12 (LRS plates + spare) |
