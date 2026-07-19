# CAD parts — modeling guide

Every enclosure is **two solids**. Export separately.

| Part | File | What |
| --- | --- | --- |
| **BODY** | `{unit}-body.stl` | Open-top box |
| **LID** | `{unit}-lid.stl` | Lid that closes the body |

| Unit | BODY | LID |
| --- | --- | --- |
| sign-input | [`sign-input/BODY.md`](sign-input/BODY.md) | [`sign-input/LID.md`](sign-input/LID.md) |
| mp-input | [`mp-input/BODY.md`](mp-input/BODY.md) | [`mp-input/LID.md`](mp-input/LID.md) |
| sign-output | [`sign-output/BODY.md`](sign-output/BODY.md) | [`sign-output/LID.md`](sign-output/LID.md) |
| mp-output | [`mp-output/BODY.md`](mp-output/BODY.md) | [`mp-output/LID.md`](mp-output/LID.md) |

Optional separate prints: DT/DTP retainer clips. **Do not print hinges** (buy bociloy 1").

---

## Coordinate system (all units)

**All XY in BODY/LID docs are on the outer base** — front-left corner of the finished part footprint = **(0, 0)**.

```text
X = left → right   (0 … L)
Y = front → back   (0 … W)
Z = bottom → up    (0 … H)

FRONT wall = Y = 0
BACK  wall = Y = W
LEFT  wall = X = 0
RIGHT wall = X = L
Floor top  = Z = wall (3.0)
```

Wall / floor thickness **3.0 mm**. Inner cavity starts at **(3, 3, 3)** and is **(L−6) × (W−6)** in XY.

Panel cutout **heights** = hole center **Z** from the outer bottom (Z=0).  
Panel **Y** = hole center along the wall from the front (Y=0).

---

## CAD vocabulary used in the docs

| Phrase | Meaning |
| --- | --- |
| **Sketch on plane P** | Create a 2D sketch on that face/plane |
| **Rectangle / circle** | Draw the shape in the sketch |
| **Extrude +join** | Extrude solid that adds material |
| **Extrude −cut** | Extrude that removes material (through-hole or pocket) |
| **Shell** | Hollow a solid, keep wall thickness |
| **Boss** | Cylinder (or tapered cylinder) standing up from the floor/underside with a hole for a heat-set insert |

Tool-agnostic: Fusion, FreeCAD, Onshape, OpenSCAD — same operations.

---

## Insert hole sizes

| Insert | Sketch circle Ø | Boss outer Ø | Boss height |
| --- | ---: | ---: | ---: |
| M2 heat-set | **3.2** | **≥7** | **≥5** (PCB bosses: top at Z≥**12**) |
| M3 heat-set | **4.2** | **≥9** | **≥6** |

Boss recipe: sketch **outer circle** on floor → extrude **+join** to height → sketch **inner circle** on boss top → extrude **−cut** through boss (and optionally into floor a little for insert depth).

---

## Print

| Part | Orientation |
| --- | --- |
| BODY | Open cavity facing **up** (+Z) |
| LID | Gasket groove facing **up** |

PETG/ABS · 0.2 mm · 3–4 walls · 25–40% infill. Bed ≤ 256×256.
