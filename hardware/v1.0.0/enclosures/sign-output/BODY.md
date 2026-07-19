# sign-output — BODY

Export: `sign-output-body.stl` · Print cavity **up**.  
Coords: **outer base (0,0)** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).  
No AC inlet / rocker — power is DTP from outdoor HLG.

## Envelope

| | mm |
| --- | ---: |
| L × W × H | **220 × 170 × 85** |
| Wall / floor | **3.0** |

---

## 1. Outer box

1. Sketch rectangle **220 × 170** from **(0, 0)** on Z=0.  
2. Extrude **+join** to Z=**85**.

## 2. Hollow cavity

1. Sketch on Z=3: rectangle from **(3, 3)** size **214 × 164**.  
2. Extrude **−cut** through the top.

## 3. Floor bosses — output PCB (M2)

Boss top Z=**12**. Centers (outer XY):

| Boss | X | Y |
| --- | ---: | ---: |
| H1 | **56.2** | **134.0** |
| H2 | **31.0** | **31.2** |
| H3 | **135.9** | **148.3** |
| H4 | **135.9** | **31.2** |

Each: sketch Ø**7** on floor → extrude to Z=12 → sketch Ø**3.2** → extrude **−cut**.

## 4. BACK wall — DTP 12 V pocket

On plane **Y = 170** (BACK outer):

1. Sketch rectangle ≈**18 × 22** (X×Z), center at **(X=110, Z=40)**.  
2. Extrude **−cut** through the wall.  
3. Optional: add a shallow inner lip; retain the connector with a **separate printed clip** (do not rely on an unsupported overhang).

## 5. LEFT wall — M12 + SOL0..4

Sketch on plane **X=0**. Extrude **−cut** through wall.

| Feature | Center (Y, Z) | Shape |
| --- | ---: | --- |
| M12-5 | **(30, 50)** | circle Ø**16.2** |
| SOL0 | **(55, 40)** | rect ≈**16 × 18** |
| SOL1 | **(80, 40)** | rect ≈**16 × 18** |
| SOL2 | **(105, 40)** | rect ≈**16 × 18** |
| SOL3 | **(130, 40)** | rect ≈**16 × 18** |
| SOL4 | **(155, 40)** | rect ≈**16 × 18** |

Pitch between SOL centers = **25** mm in Y. Use retainer clips for DT housings.

## 6. FRONT — LED window

1. Sketch on Y=0: rectangle **40 × 10**, center **(X=110, Z=50)**.  
2. Extrude **−cut** through.  
3. Optional acrylic recess on the inside.

## 7. BACK — hinge bosses (M2)

| Hinge | Pin mid (X, Y) | Boss (X, Y) |
| --- | ---: | ---: |
| A | **(43, 163)** | **(43, 155.5)** |
| B | **(177, 163)** | **(177, 155.5)** |

Each: Ø**7** → extrude → Ø**3.2** cut.

## 8. FRONT — latch inserts (M3)

| Latch | X | Y |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

Ø**9** → Ø**4.2** cut.

## 9. Export

`sign-output-body.stl`. Plain lid → [`LID.md`](LID.md).
