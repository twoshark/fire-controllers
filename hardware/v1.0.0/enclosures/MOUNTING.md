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

**Outer L × W × H** = enclosure **+X × +Y × +Z** (left→right × front→back × up).  
**LED window is on the lid (top)** — LEDs point **+Z**. FRONT wall has **no** LED cutout.

**Gerber / PnP:**

```text
+X right · +Y down (Y negative) · LED block at high X
Outline ≈ X∈[0,W], Y∈[−H,0]
Component / LED side faces +Z (up) toward the lid
```

**Enclosure floor:**

```text
Origin = inner floor, front-left (inside wall)
+X right · +Y back · +Z up
```

**PCB placement — flat, LEDs up (det +1, no side-wall aim):**

```text
enc_x = ox + X_gerber
enc_y = oy − Y_gerber

(ox, oy) = enclosure position of Gerber (0, 0)
```

Outer floor XY = inner + **3** (wall). LED column → **RIGHT** side of the cavity; lid window above it — [`LED_WINDOW.md`](LED_WINDOW.md).

| Box | Inner L×W | `(ox, oy)` inner | Why |
| --- | ---: | ---: | --- |
| sign-input | 214×174 | **(110, 45)** | RS-15 LEFT; PCB RIGHT clears arcade hex |
| mp-input | 194×154 | **(95, 35)** | Same; LED X clears arcade CH2 |
| sign-output | 214×164 | **(45, 30)** | Clearance |
| mp-output | 214×164 | **(45, 30)** | Same |

**Supersedes 2026-07-18 CW (LED→FRONT wall).** That map aimed LEDs at the FRONT face; optical axis is now **+Z**. Keep ≥10 mm wall clearance to PCB outline.

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

### sign-input · outer = inner + 3

| Boss | Gerber mil | Outer X | Outer Y | Δ from H1 (mm) |
| --- | ---: | ---: | ---: | --- |
| H1 | 1020, −725 | **138.91** | **66.41** | — |
| H2 | 3115, −160 | **192.12** | **52.06** | (+53.21, −14.35) |
| H3 | 925, −2955 | **136.50** | **123.06** | (−2.41, +56.64) |
| H4 | 3115, −2960 | **192.12** | **123.18** | (+53.21, +56.77) |

PCB footprint (inner): X[110, 193] × Y[45, 124]. Lid LED pocket **(192.3, 81.5)**.

### mp-input · outer = inner + 3

| Boss | Gerber mil | Outer X | Outer Y |
| --- | ---: | ---: | ---: |
| H1 | 1020, −725 | **123.91** | **56.41** |
| H2 | 3115, −160 | **177.12** | **42.06** |
| H3 | 925, −2955 | **121.50** | **113.06** |
| H4 | 3115, −2960 | **177.12** | **113.18** |

Same Δ-from-H1 as sign-input. Footprint (inner): X[95, 178] × Y[35, 114]. Lid LED pocket **(177.3, 71.5)**.

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

### sign/mp-output · outer = inner + 3

| Boss | Gerber mm | Outer X | Outer Y | Δ from H1 (mm) |
| --- | ---: | ---: | ---: | --- |
| H1 | (17.907, −28.194) | **65.91** | **61.19** | — |
| H2 | (120.777, −3.048) | **168.78** | **36.05** | (+102.87, −25.15) |
| H3 | (3.640, −107.950) | **51.64** | **140.95** | (−14.27, +79.76) |
| H4 | (120.777, −107.950) | **168.78** | **140.95** | (+102.87, +79.76) |

PCB footprint (inner): X[45, 169] × Y[30, 142]. Mid Ø2.54 pair skipped. Lid LED pocket **(165.2, 102.5)**.

Face map (flat, LEDs up): LEDs → **lid / RIGHT** · `J1` → **LEFT** · panel still **DTP BACK** · **SOL + M12 LEFT** (harness length OK).

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

## Lid — internal hinges + front latches

**Do not print hinges.** Prefab Amazon **bociloy 1"** SS butt (cart default).

| Spec | Value |
| --- | --- |
| Buy | [bociloy 1" 10-pack](https://www.amazon.com/bociloy-Cabinet-Rectangle-Stainless-Folding/dp/B0D43MBGXC) · **$5.29** (2026-07-18) |
| Open | **25 × 28** mm (pin length × leaf-to-leaf) |
| Folded | **25 × 16.5** mm · leaf **1.2** mm thick |
| Leaf depth | ~**14** mm from pin axis to outer edge |
| Holes | Listing **Hole Count: 2** → treat as **1 hole per leaf** until calipered |
| Qty | **2 per box** · **8** total (pack of 10) |

**Premium alt (no CAD change from old layout):** Sugatsune **KHA-25C** — 2 holes/leaf @ P1=6 / P2=13, M3 screws ([Alema](https://www.alema.com/kha-25c-25mm-stainless-steel-butt-hinge-with-screw-holes.html)).

### Placement (all four boxes)

```text
Hinge pin on BACK edge (internal) · lid opens toward FRONT
2× bociloy 1" · leaves inside cavity (not through outer wall)
Latches: 2× M3 at FRONT corners (screw into body flange inserts)
```

| Box | Inner L×W | Hinge A pin center (X, Y) | Hinge B | Latch M3 (FRONT) |
| --- | ---: | ---: | ---: | --- |
| sign-input | 214×174 | **(40, 170)** | **(174, 170)** | (8,8) (206,8) |
| mp-input | 194×154 | **(35, 150)** | **(159, 150)** | (8,8) (186,8) |
| sign/mp-output | 214×164 | **(40, 160)** | **(174, 160)** | (8,8) (206,8) |

Y = near back wall (inner W − ~4). Pin axis parallel to **+X**. Pin length **25** mm; centers above are **mid-pin**.

### Boss / hole pattern (bociloy — CAD target)

Coordinate origin for one hinge: pin axis at placement (X₀, Y₀), +X along pin, +Y into cavity (away from outer BACK wall).

| Feature | Body leaf | Lid leaf |
| --- | --- | --- |
| Heat-set | **M2×3** (small leaf holes; discard included wood screws) | same |
| Hole CL from pin | **Y = 7.5** mm | **Y = 7.5** mm |
| Hole along pin | **X = 0** (mid-pin) | **X = 0** |
| Print hole | ~**Ø3.2** | ~**Ø3.2** |
| Boss OD | **≥7** | **≥7** |
| Screw | **M2×6** pan | **M2×6** pan |

**If the pack arrives with 2 holes/leaf:** keep Y=7.5; place holes at mid-pin **±6.5** mm along X (typical for 25 mm leaves). Re-measure before final print.

**Body leaf:** bosses on inner BACK wall / flange, leaf face toward cavity.  
**Lid leaf:** bosses on lid underside near back edge.  
**Latches:** still **M3** heat-set + **M3×10** pan (2 per box).  
**RS-15:** still **M3** (unchanged).

Keep-out: no PCB / wire in the **20 × 30** mm zone around each hinge when closed. Lid swing: leave **≥90°** clear above BACK face.

Gasket groove in **lid**, print groove-up. Groove 3.5 W × 2.0 D · 20–30% crush. Run gasket continuous on FRONT/LEFT/RIGHT; on BACK run **inboard of hinge knuckles** so hinges stay internal and dry.

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

| Item | Approx |
| --- | ---: |
| bociloy 1" hinge | **8** (2×4 boxes) · pack of 10 |
| M2×3 heat-set | ≥**40** (PCB H1–H4×4 + hinge 16 + spare; +buttons) |
| M3×5.7 heat-set | ≥**20** (latch 8 + RS-15 4 + spare) — **not** for Amazon hinge leaves |
| M2×6–8 pan | ≥40 (PCB + hinges) |
| M3×10–12 pan | ≥12 (front latches + spare) |
| M3×6–8 into RS-15 | ≥4 (L≤4 into PSU) |
