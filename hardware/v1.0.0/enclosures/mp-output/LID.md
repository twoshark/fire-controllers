# mp-output — LID

Export: `mp-output-lid.stl` · Print **groove up**.  
Coords: **outer (0,0)** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).  
Same geometry as [`../sign-output/LID.md`](../sign-output/LID.md) — you can duplicate that model.

## Envelope

| | mm |
| --- | ---: |
| L × W | **220 × 170** |
| Thickness | **5** |

---

## 1. Lid plate

1. Sketch rectangle **220 × 170** from **(0, 0)**.  
2. Extrude **+join** to Z=**5**.

## 2. Gasket groove

1. Sketch inset loop on Z=5, width **3.5**, BACK inboard of hinges.  
2. Extrude **−cut** **2.0** mm.

## 3. Hinge bosses (M2)

| Hinge | Pin mid (X, Y) | Boss (X, Y) |
| --- | ---: | ---: |
| A | **(43, 163)** | **(43, 155.5)** |
| B | **(177, 163)** | **(177, 155.5)** |

Underside: Ø**7** → Ø**3.2** cut.

## 4. Latch clearance (M3)

| Latch | X | Y |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

Ø**3.4** through-cut. Optional counterbore.

## 5. Export

`mp-output-lid.stl`. No arcade holes.
