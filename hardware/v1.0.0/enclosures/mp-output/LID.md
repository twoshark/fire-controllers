# mp-output — LID

**All sizes in mm.** CAD document units = mm.  
Export: `mp-output-lid.stl` · Print **groove up**.  
Coords: **outer (0, 0) mm** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).  
Same geometry as [`../sign-output/LID.md`](../sign-output/LID.md).

## Envelope

| | Size |
| --- | ---: |
| L × W | **220 × 170 mm** |
| Thickness | **5 mm** |

---

## 1. Lid plate

1. Sketch rectangle **220 × 170 mm** from **(0, 0) mm**.  
2. Extrude **+join** to Z = **5 mm**.

## 2. Gasket groove

1. Sketch inset loop on Z = 5 mm, width **3.5 mm**, BACK inboard of hinges.  
2. Extrude **−cut** **2.0 mm** deep.

## 3. Hinge bosses (M2)

| Hinge | Pin mid (X, Y) mm | Boss (X, Y) mm |
| --- | ---: | ---: |
| A | **(43, 163)** | **(43, 155.5)** |
| B | **(177, 163)** | **(177, 155.5)** |

Underside: Ø**7 mm** → Ø**3.2 mm** cut (≥**5 mm** tall).

## 4. Latch clearance (M3)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

Ø**3.4 mm** through-cut. Optional Ø**6 mm** × **1.5 mm** counterbore.

## 5. Export

`mp-output-lid.stl`. No arcade holes.
