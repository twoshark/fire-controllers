# mp-input — LID

**All sizes in mm.** CAD document units = mm.  
Export: `mp-input-lid.stl` · Print **groove up**.  
Coords: **outer (0, 0) mm** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | Size |
| --- | ---: |
| L × W | **200 × 160 mm** |
| Thickness | **5 mm** |

---

## 1. Lid plate

1. Sketch rectangle **200 × 160 mm** from **(0, 0) mm**.  
2. Extrude **+join** to Z = **5 mm**.

## 2. Gasket groove

1. Sketch on Z = 5 mm: closed loop inset ~**4 mm**; groove width **3.5 mm**; BACK inboard of hinges.  
2. Extrude **−cut** **2.0 mm** deep.

## 3. Hinge bosses (M2)

| Hinge | Pin mid (X, Y) mm | Boss (X, Y) mm |
| --- | ---: | ---: |
| A | **(38, 153)** | **(38, 145.5)** |
| B | **(162, 153)** | **(162, 145.5)** |

Underside: Ø**7 mm** extrude ≥**5 mm** → Ø**3.2 mm** cut.

## 4. Latch clearance (M3)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **189** | **11** |

Sketch Ø**3.4 mm** → extrude **−cut** through. Optional Ø**6 mm** × **1.5 mm** counterbore.

## 5. Arcade button holes

Ø**28 mm** through (verify ring; may be Ø**24 mm**). Ring center **(100, 80) mm**:

| Button | Center X (mm) | Center Y (mm) |
| --- | ---: | ---: |
| ALL | **100.0** | **80.0** |
| CH1 | **100.0** | **30.0** |
| CH2 | **143.3** | **105.0** |
| CH3 | **56.7** | **105.0** |

Each: sketch circle → extrude **−cut** through lid.

```text
            [CH1]
      [CH3]  [ALL]  [CH2]
```

## 6. LED window (top) — dividers + clear insert

LEDs point **up**. Full recipe: [`../LED_WINDOW.md`](../LED_WINDOW.md).

| | mm |
| --- | ---: |
| Pocket center **(Xc, Yc)** | **(177.3, 71.5)** |
| Recess | **9.4 × 38.4 × 1.5** deep |
| Cells | **10×** **2.8 × 2.8** · pitch **3.50** along Y |
| Clear insert (separate print) | **9.0 × 38.0 × 1.2** clear PETG/PLA |

## 7. Export

`mp-input-lid.stl` · clear insert `mp-input-led-window.stl`.
