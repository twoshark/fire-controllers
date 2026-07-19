# mp-input — BODY

Export: `mp-input-body.stl` · Print cavity **up**.  
Coords: **outer base (0,0)** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | mm |
| --- | ---: |
| L × W × H | **200 × 160 × 90** |
| Wall / floor | **3.0** |

---

## 1. Outer box

1. Sketch XY Z=0: rectangle **200 × 160** from **(0, 0)**.  
2. Extrude **+join** to Z=**90**.

## 2. Hollow cavity

1. Sketch on Z=3: rectangle from **(3, 3)** size **194 × 154**.  
2. Extrude **−cut** through the top.

## 3. Floor bosses — RS-15 (M3)

| Boss | X | Y |
| --- | ---: | ---: |
| M3-A | **38.5** | **29.5** |
| M3-B | **38.5** | **68.6** |

Each: sketch Ø**9** on floor → extrude **+join** +6 → sketch Ø**4.2** → extrude **−cut**.  
PSU keep-out: **(13, 18)** → **(64, 80.5)**.

## 4. Floor bosses — input PCB (M2)

Boss top Z=**12**. Centers (outer XY):

| Boss | X | Y |
| --- | ---: | ---: |
| H1 | **91.4** | **90.2** |
| H2 | **77.1** | **36.9** |
| H3 | **148.1** | **92.6** |
| H4 | **148.2** | **36.9** |

Each: Ø**7** extrude to Z=12 → Ø**3.2** cut (M2).

## 5. LEFT wall cutouts

Sketch on plane **X=0**. Extrude **−cut** through wall. Centers (Y, Z):

| Feature | Center (Y, Z) | Shape |
| --- | ---: | --- |
| C14 | **(30, 35)** | rect **27.5 × 20** |
| KCD4 | **(55, 50)** | rect **30 × 22** |
| M12-5 | **(80, 50)** | circle Ø**16.2** |

## 6. FRONT — LED window

1. Sketch on Y=0: rectangle **40 × 10**, center **(X=100, Z=50)**.  
2. Extrude **−cut** through wall.  
3. Optional inner recess for acrylic + foam.

## 7. BACK — hinge bosses (M2)

| Hinge | Pin mid (X, Y) | Boss (X, Y) = pin − 7.5 in Y |
| --- | ---: | ---: |
| A | **(38, 153)** | **(38, 145.5)** |
| B | **(162, 153)** | **(162, 145.5)** |

Each: Ø**7** boss → Ø**3.2** cut.

## 8. FRONT — latch inserts (M3)

| Latch | X | Y |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **189** | **11** |

Ø**9** boss → Ø**4.2** cut.

## 9. Export

`mp-input-body.stl`. Arcade holes → [`LID.md`](LID.md).
