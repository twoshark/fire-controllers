# mp-input — LID

Export: `mp-input-lid.stl` · Print **groove up**.  
Coords: **outer (0,0)** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | mm |
| --- | ---: |
| L × W | **200 × 160** |
| Thickness | **5** |

---

## 1. Lid plate

1. Sketch rectangle **200 × 160** from **(0, 0)**.  
2. Extrude **+join** to Z=**5**.

## 2. Gasket groove

1. Sketch on Z=5: closed loop inset ~4 mm; groove width **3.5**; depth cut **2.0**.  
2. On BACK, keep groove **inboard of hinges** (clear Y≈153 region).  
3. Extrude **−cut** 2.0 mm.

## 3. Hinge bosses (M2)

| Hinge | Pin mid (X, Y) | Boss (X, Y) |
| --- | ---: | ---: |
| A | **(38, 153)** | **(38, 145.5)** |
| B | **(162, 153)** | **(162, 145.5)** |

Underside: Ø**7** extrude → Ø**3.2** cut.

## 4. Latch clearance (M3)

| Latch | X | Y |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **189** | **11** |

Sketch Ø**3.4** → extrude **−cut** through. Optional head counterbore.

## 5. Arcade button holes

Ø**28** through (verify ring). Ring center **(100, 80)**:

| Button | Center X | Center Y |
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

## 6. Export

`mp-input-lid.stl`.
