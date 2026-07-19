# CAD parts — modeling guide

**All dimensions in these CAD docs are millimeters (mm).** Set your CAD document units to **mm**.

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

Optional separate prints: DT/DTP retainer clips. **Do not print hinges** (buy bociloy ~25 mm / “1 inch” SS butt).

---

## Coordinate system (all values mm)

**All XY on the outer base** — front-left corner of the footprint = **(0, 0) mm**.

```text
X = left → right   (0 … L) mm
Y = front → back   (0 … W) mm
Z = bottom → up    (0 … H) mm

FRONT wall = Y = 0 mm
BACK  wall = Y = W mm
LEFT  wall = X = 0 mm
RIGHT wall = X = L mm
Floor top  = Z = 3.0 mm
```

Wall / floor thickness **3.0 mm**. Inner cavity starts at **(3, 3, 3) mm** and is **(L−6) × (W−6) mm** in XY.

Panel cutout **Z** = hole center from outer bottom.  
Panel **Y** = hole center along the wall from the front.

---

## CAD vocabulary

| Phrase | Meaning |
| --- | --- |
| **Sketch on plane P** | Create a 2D sketch on that face/plane |
| **Rectangle / circle** | Draw the shape (sizes in mm) |
| **Extrude +join** | Extrude solid that adds material (distance in mm) |
| **Extrude −cut** | Extrude that removes material |
| **Boss** | Cylinder with heat-set hole |

---

## Insert hole sizes (mm)

| Insert | Sketch circle Ø | Boss outer Ø | Boss height |
| --- | ---: | ---: | ---: |
| M2 heat-set | **3.2 mm** | **≥7 mm** | **≥5 mm** (PCB: top at Z≥**12 mm**) |
| M3 heat-set | **4.2 mm** | **≥9 mm** | **≥6 mm** |

Boss recipe: sketch outer circle on floor → extrude **+join** to height → sketch inner circle on boss top → extrude **−cut**.

---

## Print

| Part | Orientation |
| --- | --- |
| BODY | Open cavity facing **up** (+Z) |
| LID | Gasket groove facing **up** |

PETG/ABS · **0.2 mm** layer · 3–4 walls · 25–40% infill. Bed ≤ **256 × 256 mm**.
