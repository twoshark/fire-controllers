# Mounting — inserts, pillars, bosses

Gerber source (2026-07-18): `hardware/v1.0.0/{input/easyeda,output/eda-exports,input-buttons-pcb/exports}/gerber/`. Hole XY from `Drill_PTH_Through.DRL` (Ø2.54). Same coordinate frame as PnP (mm).

## PCB mounting pads

| Dim | mil | mm |
| --- | ---: | ---: |
| Hole (drill) | **100** | **2.54** |
| Pad diameter | **160** | **4.06** |

**M2** pan screws through pad → **M2 heat-set** in floor bosses. Not every Ø2.54 hole is used — only the stable set below.

---

## Board outlines (Gerber GKO)

| Board | Outline W × H | Floor keep-out (margin) |
| --- | ---: | ---: |
| Input | **83.1 × 79.0** | **86 × 83** |
| Output | **124.0 × 112.4** | **126 × 114** |
| Buttons | **71.0 × 44.7** | under lid / standoffs |

---

## Coordinate frames

**Outer L × W × H** = enclosure **+X × +Y × +Z** (left→right × front→back × up). FRONT = LED window wall (Y≈0).

**Gerber / PnP:**

```text
+X right · +Y down (Y negative) · LED block at high X
Outline ≈ X∈[0,W], Y∈[−H,0]
```

**Enclosure floor:**

```text
Origin = inner floor, front-left (inside wall)
+X right · +Y back · +Z up
```

**PCB placement** — LED edge (max X) → FRONT:

```text
enc_x = ox + (−Y_gerber)
enc_y = oy + (W − X_gerber)
```

`(ox, oy)` = enclosure position of Gerber **(W, 0)** = board front-left.

| Box | Inner L×W | `(ox, oy)` | Why |
| --- | ---: | ---: | --- |
| sign-input | 214×174 | **(75, 40)** | RS-15 LEFT (X≈10–61); PCB right of PSU |
| mp-input | 194×154 | **(70, 30)** | Same pack, tighter |
| sign-output | 214×164 | **(25, 25)** | Max clearance; aft clearance ~15 |
| mp-output | 214×164 | **(25, 25)** | Same |

Keep ≥10 mm wall clearance to PCB outline.

---

## Heat-set inserts

| Spec | M2 (PCB / buttons) | M3 (lid / RS-15) |
| --- | --- | --- |
| Example | Ruthex / CNC Kitchen M2×4–5 | M3×5.7 |
| Print hole Ø | **~3.2** | **4.2** |
| Boss outer Ø | **≥7** | **≥9** |
| Boss height | **≥5** | **≥6** |
| Screw | M2×6–8 pan | M3×8–12 pan (RS-15 screw L≤3–4 into PSU) |

PCB standoff **Z = 12**. RS-15 chassis Z≈3.

---

## Input PCB bosses (4 of 5 Ø2.54)

Outline **W = 83.06**. Skip `(43.180, −54.483)`.

| Boss | Gerber X | Gerber Y |
| --- | ---: | ---: |
| H1 | **25.908** | **−18.415** |
| H2 | **79.121** | **−4.064** |
| H3 | **23.495** | **−75.057** |
| H4 | **79.121** | **−75.184** |

### sign-input @ (75, 40)

| Boss | enc X | enc Y |
| --- | ---: | ---: |
| H1 | 93.4 | 97.2 |
| H2 | 79.1 | 43.9 |
| H3 | 150.1 | 99.6 |
| H4 | 150.2 | 43.9 |

PCB outline on floor: X[75, 154] × Y[40, 123].

### mp-input @ (70, 30)

| Boss | enc X | enc Y |
| --- | ---: | ---: |
| H1 | 88.4 | 87.2 |
| H2 | 74.1 | 33.9 |
| H3 | 145.1 | 89.6 |
| H4 | 145.2 | 33.9 |

PCB outline: X[70, 149] × Y[30, 113].

---

## RS-15-12 bosses (Mean Well case 971A)

Body **62.5 × 51 × 28**. **2× M3**, spacing **39.1** along length, inset **11.5** from each short end (datasheet ±1 mm). Place length along +Y; width along +X; left of PCB.

| Box | Body X×Y (front-left) | M3 A (enc) | M3 B (enc) |
| --- | --- | ---: | ---: |
| sign-input | (10, 20) → (61, 82.5) | **(35.5, 31.5)** | **(35.5, 70.6)** |
| mp-input | (10, 15) → (61, 77.5) | **(35.5, 26.5)** | **(35.5, 65.6)** |

Hole X = body_x + 25.5 (width centerline). Verify on your unit before drilling inserts.

---

## Output PCB bosses (4 of 6 Ø2.54)

Outline **W = 123.95**. Skip mid-pair and fuse NPTH `(10.541, −5.461)`.

| Boss | Gerber X | Gerber Y |
| --- | ---: | ---: |
| H1 | **17.907** | **−28.194** |
| H2 | **120.777** | **−3.048** |
| H3 | **3.640** | **−107.950** |
| H4 | **120.777** | **−107.950** |

### sign/mp-output @ (25, 25)

| Boss | enc X | enc Y |
| --- | ---: | ---: |
| H1 | 53.2 | 131.0 |
| H2 | 28.0 | 28.2 |
| H3 | 132.9 | 145.3 |
| H4 | 132.9 | 28.2 |

PCB outline: X[25, 137.4] × Y[25, 148.9].

Face map (after transform): `J5`/`J6` → **LEFT** · `J1` → **BACK** · LEDs → **FRONT** · `J2` (RS-485) → **LEFT** near front.

---

## Buttons daughter (optional)

Gerber Ø2.54 corners — use **2 diagonal** or all 4. Mount under lid / to lid bosses; orient so harness reaches `J2`.

| Boss | Gerber X | Gerber Y |
| --- | ---: | ---: |
| B1 | 2.667 | −2.667 |
| B2 | 68.199 | −2.794 |
| B3 | 2.794 | −41.910 |
| B4 | 68.072 | −41.910 |

---

## Lid M3 flange (4 corners)

Body flange inserts, inset **8 mm** from inner corners (outer = inner + wall).

| Box | Inner L×W | Lid M3 (enc X, Y) |
| --- | ---: | --- |
| sign-input | 214×174 | (8,8) (206,8) (8,166) (206,166) |
| mp-input | 194×154 | (8,8) (186,8) (8,146) (186,146) |
| sign/mp-output | 214×164 | (8,8) (206,8) (8,156) (206,156) |

Gasket groove in **lid**, print groove-up. Groove 3.5 W × 2.0 D · 20–30% crush.

---

## Arcade lid holes (centers — verify EG STARTS Ø)

**sign-input** — **compact hex ring** (fits 180 mm depth). ALL at lid center; others on R = **38.3** (chord pitch **45**). Lid center ≈ outer **(110, 90)**. Hole bbox ≈ **110 × 110**.

| Button | ΔX | ΔY | Angle |
| --- | ---: | ---: | ---: |
| ALL | 0 | 0 | — |
| CH1 | 0 | −38.3 | 270° |
| CH2 | +36.4 | −11.8 | 342° |
| CH3 | +22.5 | +31.0 | 54° |
| CH4 | −22.5 | +31.0 | 126° |
| CH5 | −36.4 | −11.8 | 198° |

Margins ≥12 from lid edge. Absolute = center + Δ.

**mp-input** — pitch **≥50**, center outer **(100, 80)**:

| Button | ΔX | ΔY |
| --- | ---: | ---: |
| ALL | 0 | 0 |
| CH1 | 0 | −50 |
| CH2 | +43.3 | +25 |
| CH3 | −43.3 | +25 |

---

## Panel feature Y (along face, from FRONT, outer)

Heights (CL from outer bottom) stay in CAD_NOTES. These are **Y along the wall**.

### sign-input LEFT (inner depth 174)

| Feature | Y (front→back) |
| --- | ---: |
| C14 | **35** |
| KCD4 | **60** |
| M12-5 | **85** (near `CN2` @ Y≈71) |

### mp-input LEFT (inner depth 154)

| Feature | Y |
| --- | ---: |
| C14 | **30** |
| KCD4 | **55** |
| M12-5 | **80** |

### sign-output LEFT (inner depth 164)

| Feature | Y |
| --- | ---: |
| M12-5 | **30** (near `J2`) |
| SOL0 | **55** |
| SOL1 | **80** |
| SOL2 | **105** |
| SOL3 | **130** |
| SOL4 | **155** |

DTP on **BACK** face, centered in X, CL height **40**.

### mp-output LEFT

| Feature | Y |
| --- | ---: |
| M12-5 | **30** |
| SOL0 | **55** |
| SOL1 | **80** |
| SOL2 | **105** |

---

## Qty

| Insert | Approx |
| --- | ---: |
| M2×4–5 | ≥20 (4×4 boards + spare; +4 if buttons) |
| M3×5.7 | ≥24 (lids 16 + RS-15 4 + spare) |
| M2×6–8 pan | ≥20 |
| M3×8–12 pan | ≥20 (lid); M3×6–8 for RS-15 (L≤4 into PSU) |
