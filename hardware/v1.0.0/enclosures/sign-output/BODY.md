# sign-output — BODY

**All sizes in mm.** CAD document units = mm.  
Export: `sign-output-body.stl` · Print cavity **up**.  
Coords: **outer (0, 0) mm** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).  
No AC inlet / rocker — power is DTP from outdoor HLG.

## Envelope

| | Size |
| --- | ---: |
| L × W × H | **220 × 170 × 85 mm** |
| Wall / floor | **3.0 mm** |

---

## 1. Outer box

1. Sketch rectangle **220 × 170 mm** from **(0, 0) mm** on Z = 0 mm.  
2. Extrude **+join** to Z = **85 mm**.

## 2. Hollow cavity

1. Sketch on Z = 3 mm: rectangle from **(3, 3) mm**, size **214 × 164 mm**.  
2. Extrude **−cut** through the top.

## 3. Floor bosses — output PCB (M2)

Boss top Z = **12 mm**. Centers (outer XY):

| Boss | X (mm) | Y (mm) |
| --- | ---: | ---: |
| H1 | **56.2** | **134.0** |
| H2 | **31.0** | **31.2** |
| H3 | **135.9** | **148.3** |
| H4 | **135.9** | **31.2** |

Each: sketch Ø**7 mm** on floor → extrude to Z = 12 mm → sketch Ø**3.2 mm** → extrude **−cut**.

## 4. BACK wall — DTP 12 V pocket

On plane **Y = 170 mm** (BACK outer):

1. Sketch rectangle ≈**18 × 22 mm** (X × Z), center at **(X = 110 mm, Z = 40 mm)**.  
2. Extrude **−cut** through the **3 mm** wall.  
3. Retain connector with a **separate printed clip**.

## 5. LEFT wall — M12 + SOL0..4

Sketch on plane **X = 0 mm**. Extrude **−cut** through wall.

| Feature | Center Y, Z (mm) | Shape (mm) |
| --- | ---: | --- |
| M12-5 | **(30, 50)** | circle Ø**16.2** |
| SOL0 | **(55, 40)** | rect ≈**16 × 18** |
| SOL1 | **(80, 40)** | rect ≈**16 × 18** |
| SOL2 | **(105, 40)** | rect ≈**16 × 18** |
| SOL3 | **(130, 40)** | rect ≈**16 × 18** |
| SOL4 | **(155, 40)** | rect ≈**16 × 18** |

Pitch between SOL centers = **25 mm** in Y.

## 6. FRONT — LED window

1. Sketch on Y = 0 mm: rectangle **40 × 10 mm**, center **(X = 110 mm, Z = 50 mm)**.  
2. Extrude **−cut** through.  
3. Optional acrylic recess ~**1.5 mm** deep inside.

## 7. BACK — hinge bosses (M2)

| Hinge | Pin mid (X, Y) mm | Boss (X, Y) mm |
| --- | ---: | ---: |
| A | **(43, 163)** | **(43, 155.5)** |
| B | **(177, 163)** | **(177, 155.5)** |

Boss = pin − **7.5 mm** in Y. Each: Ø**7 mm** → Ø**3.2 mm** cut (≥**5 mm** tall).

## 8. FRONT — latch inserts (M3)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

Ø**9 mm** → Ø**4.2 mm** cut (≥**6 mm** tall).

## 9. Export

`sign-output-body.stl`. Lid → [`LID.md`](LID.md).
