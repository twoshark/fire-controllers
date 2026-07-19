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

**PCB placement — CW 90°, LEDs up, LED edge → FRONT of lid (det +1):**

```text
enc_x = ox + Y_gerber
enc_y = oy + (X_led − X_gerber)

X_led = board outline max X  (input 83.058 · output 123.952)
(ox, oy) = enclosure position of Gerber (X_led, 0)  // LED-edge corner at Gerber Y=0
```

Outer floor XY = inner + **3** (wall). Board sits toward the **FRONT**; lid LED strip is a front row along **+X** — [`LED_WINDOW.md`](LED_WINDOW.md). Arcade buttons sit **farther back** so they clear the window.

| Box | Inner L×W | `(ox, oy)` inner | Why |
| --- | ---: | ---: | --- |
| sign-input | 214×174 | **(148.994, 28)** | RS-15 LEFT; PCB front; LED strip Y≈35 |
| mp-input | 194×154 | **(143.994, 25)** | Same, tighter |
| sign-output | 214×164 | **(137.395, 25)** | LED strip at front of lid |
| mp-output | 214×164 | **(137.395, 25)** | Same |

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

### sign-input · outer = inner + 3

| Boss | Gerber mil | Outer X | Outer Y | Δ from H1 (mm) |
| --- | ---: | ---: | ---: | --- |
| H1 | 1020, −725 | **133.58** | **88.15** | — |
| H2 | 3115, −160 | **147.93** | **34.94** | (+14.35, −53.21) |
| H3 | 925, −2955 | **76.94** | **90.56** | (−56.64, +2.41) |
| H4 | 3115, −2960 | **76.81** | **34.94** | (−56.77, −53.21) |

PCB footprint (inner): X[70, 149] × Y[28, 111]. Lid LED pocket **(118.4, 34.8)**. Arcade hex center **(110, 100)** (back of front LED strip).

### mp-input · outer = inner + 3

| Boss | Gerber mil | Outer X | Outer Y |
| --- | ---: | ---: | ---: |
| H1 | 1020, −725 | **128.58** | **85.15** |
| H2 | 3115, −160 | **142.93** | **31.94** |
| H3 | 925, −2955 | **71.94** | **87.56** |
| H4 | 3115, −2960 | **71.81** | **31.94** |

Same Δ-from-H1 as sign-input. Footprint (inner): X[65, 144] × Y[25, 108]. Lid LED pocket **(113.4, 31.8)**. Arcade center **(100, 100)**.

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
| H1 | (17.907, −28.194) | **112.20** | **134.05** | — |
| H2 | (120.777, −3.048) | **137.35** | **31.17** | (+25.15, −102.88) |
| H3 | (3.640, −107.950) | **32.45** | **148.31** | (−79.75, +14.26) |
| H4 | (120.777, −107.950) | **32.45** | **31.17** | (−79.75, −102.88) |

PCB footprint (inner): X[25, 137] × Y[25, 149]. Mid Ø2.54 pair skipped. Lid LED pocket **(70.9, 34.7)**.

Face map (CW, LEDs up): LEDs → **lid FRONT** · `J1` → **BACK** · **DTP BACK** · **SOL + M12 LEFT**.

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

**sign-input** — **compact hex ring**. Center shifted **back** so CH1 clears the front LED strip. R = **38.3** (chord pitch **45**). Lid center ≈ outer **(110, 100)**.

| Button | ΔX | ΔY | Angle | Absolute (X, Y) |
| --- | ---: | ---: | ---: | ---: |
| ALL | 0 | 0 | — | **(110.0, 100.0)** |
| CH1 | 0 | −38.3 | 270° | **(110.0, 61.7)** |
| CH2 | +36.4 | −11.8 | 342° | **(146.4, 88.2)** |
| CH3 | +22.5 | +31.0 | 54° | **(132.5, 131.0)** |
| CH4 | −22.5 | +31.0 | 126° | **(87.5, 131.0)** |
| CH5 | −36.4 | −11.8 | 198° | **(73.6, 88.2)** |

Margins ≥12 from lid edge. LED pocket at Y≈**35** — keep ≥**14** mm from nearest button edge.

**mp-input** — pitch **≥50**, center outer **(100, 100)** (back of LED strip):

| Button | ΔX | ΔY | Absolute (X, Y) |
| --- | ---: | ---: | ---: |
| ALL | 0 | 0 | **(100.0, 100.0)** |
| CH1 | 0 | −50 | **(100.0, 50.0)** |
| CH2 | +43.3 | +25 | **(143.3, 125.0)** |
| CH3 | −43.3 | +25 | **(56.7, 125.0)** |

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
