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

**Gerber / PnP** (authoritative for hole tables):

```text
+X right · +Y down (Y values negative) · LED block at high X
Outline ≈ X∈[0,W], Y∈[−H,0]
```

**Enclosure floor** (CAD):

```text
Origin = inner floor, front-left corner (inside wall)
+X = toward right · +Y = toward back · +Z = up
FRONT = LED window wall
```

**Place PCB** with LED edge (max X) → FRONT. Map Gerber → enclosure:

```text
enc_x = ox + (−Y_gerber)
enc_y = oy + (W − X_gerber)
```

`(ox, oy)` = enclosure position of Gerber corner **(W, 0)** = board **front-left** (LED edge at front, drawing-top toward left).

Suggested `(ox, oy)`: **sign-input** `(20, 40)` · **sign-output** `(25, 25)` — nudge for wire dress; keep ≥10 mm wall clearance to outline.

---

## Heat-set inserts

| Spec | M2 (PCB) | M3 (lid / RS-15) |
| --- | --- | --- |
| Example | Ruthex / CNC Kitchen M2×4–5 | M3×5.7 |
| Print hole Ø | **~3.2** (per insert datasheet) | **4.2** |
| Boss outer Ø | **≥7** | **≥9** |
| Boss height | **≥5** | **≥6** |
| Screw | M2×6–8 pan | M3×8–12 pan |

Install with insert tip; cool before load. PCB standoff **Z = 12** (board bottom above floor).

---

## Input PCB — use 4 of 5 Ø2.54 holes

Outline **W = 83.06**. Skip center hole (not needed for stiffness).

| Boss | Gerber X | Gerber Y | Role |
| --- | ---: | ---: | --- |
| H1 | **25.908** | **−18.415** | TL |
| H2 | **79.121** | **−4.064** | TR (near LED edge) |
| H3 | **23.495** | **−75.057** | BL |
| H4 | **79.121** | **−75.184** | BR (near LED edge) |

Unused: `(43.180, −54.483)` mid-board.

Span ≈ **55.6 × 71.1** mm. Example bosses at `(ox,oy)=(20,40)`:

| Boss | enc X | enc Y |
| --- | ---: | ---: |
| H1 | 38.4 | 97.1 |
| H2 | 24.1 | 43.9 |
| H3 | 95.1 | 99.6 |
| H4 | 95.2 | 43.9 |

**RS-15-12** (62.5 × 51 × 28): LEFT near C14 · 2× **M3** · Z≈3 · lid clearance ≥10.

**Buttons daughter** (optional 4× M2): Gerber corners `(2.667,−2.667)`, `(68.199,−2.794)`, `(2.794,−41.910)`, `(68.072,−41.910)` — 2 diagonal OK if space tight.

---

## Output PCB — use 4 of 6 Ø2.54 holes

Outline **W = 123.95**. Skip mid-pair and fuse NPTH.

| Boss | Gerber X | Gerber Y | Role |
| --- | ---: | ---: | --- |
| H1 | **17.907** | **−28.194** | TL |
| H2 | **120.777** | **−3.048** | TR (near LED) |
| H3 | **3.640** | **−107.950** | BL |
| H4 | **120.777** | **−107.950** | BR (near LED) |

**Do not** boss:

| Gerber | Why |
| --- | --- |
| `(64.516, −76.073)`, `(76.850, −76.073)` | interior pair — unused |
| NPTH `(10.541, −5.461)` Ø2.50 | **F9** ATO fuse hole |

Span ≈ **117.1 × 104.9** mm. Example `(ox,oy)=(25,25)`:

| Boss | enc X | enc Y |
| --- | ---: | ---: |
| H1 | 53.2 | 131.0 |
| H2 | 28.0 | 28.2 |
| H3 | 133.0 | 145.3 |
| H4 | 133.0 | 28.2 |

Orientation: LED→FRONT · `J5`/`J6`→SOL · `J1`→DTP · no pillar over `F9`.

---

## Lid fasteners

| Spec | Value |
| --- | --- |
| Count | **4** corners |
| Insert | M3×5.7 in body flange |
| Screw | M3×10–12 |
| Gasket | 2–3 mm silicone · 20–30% compression |
| Groove | 3.5 W × 2.0 D · print face up |

## Qty

| Insert | Approx |
| --- | ---: |
| M2×4–5 | ≥20 (4 holes × 4 boards + spare; +4 if buttons) |
| M3×5.7 | ≥24 (lids + RS-15 + spare) |
| M2×6–8 pan screws | ≥20 |
| M3×10–12 pan screws | ≥20 |
