# sign-output — LID

**All sizes in mm.** CAD document units = mm.  
Export: `sign-output-lid.stl` · Print **groove up**.  
Coords: **outer (0, 0) mm** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).  
**No arcade holes.**

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

1. Sketch on Z = 5 mm: inset loop, width **3.5 mm**, BACK path inboard of hinges.  
2. Extrude **−cut** **2.0 mm** deep.

## 3. Hinge bosses (M2)

| Hinge | Pin mid (X, Y) mm | Boss (X, Y) mm |
| --- | ---: | ---: |
| A | **(43, 163)** | **(43, 155.5)** |
| B | **(177, 163)** | **(177, 155.5)** |

Underside: Ø**7 mm** extrude ≥**5 mm** → Ø**3.2 mm** cut.

## 4. Latch clearance (M3)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

Sketch Ø**3.4 mm** → extrude **−cut** through. Optional Ø**6 mm** × **1.5 mm** counterbore.

## 5. LED window (top) — dividers + clear insert

LEDs point **up**. Full recipe: [`../LED_WINDOW.md`](../LED_WINDOW.md).

| | mm |
| --- | ---: |
| Pocket center **(Xc, Yc)** | **(165.2, 102.5)** |
| Recess | **9.4 × 38.4 × 1.5** deep |
| Cells | **10×** **2.8 × 2.8** · pitch **3.50** along Y (`CH7` front → `PWR` back) |
| Clear insert (separate print) | **9.0 × 38.0 × 1.2** clear PETG/PLA |

## 6. Export

`sign-output-lid.stl` · clear insert `sign-output-led-window.stl`.
