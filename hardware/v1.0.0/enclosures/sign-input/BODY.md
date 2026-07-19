# sign-input — BODY

**All sizes in mm.** CAD document units = mm.  
Export: `sign-input-body.stl` · Print with open cavity **up**.  
Coords: **outer base (0, 0) mm** = front-left corner. See [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | Size |
| --- | ---: |
| L × W × H | **220 × 180 × 90 mm** |
| Wall / floor | **3.0 mm** |

---

## 1. Outer box

1. Sketch on **XY at Z = 0 mm**: rectangle **220 × 180 mm**, corner at **(0, 0) mm**.  
2. Extrude **+join** to **Z = 90 mm**.

## 2. Hollow the cavity (open top)

1. Sketch on **floor top** (plane Z = 3 mm): rectangle from **(3, 3) mm**, size **214 × 174 mm**.  
2. Extrude **−cut** from Z = 3 mm through the top (cut to Z = 90 mm).  
   Result: open-top shell, walls **3 mm**, floor **3 mm**.

## 3. Floor bosses — RS-15-12 (M3)

PSU sits on the floor, left side. Boss centers (outer XY):

| Boss | X (mm) | Y (mm) |
| --- | ---: | ---: |
| M3-A | **38.5** | **34.5** |
| M3-B | **38.5** | **73.6** |

For each:

1. Sketch on **floor (Z = 3 mm)**: circle Ø**9 mm** (boss OD) at the XY above.  
2. Extrude **+join** **+6 mm** (boss height).  
3. Sketch on boss top: circle Ø**4.2 mm** (M3 heat-set).  
4. Extrude **−cut** through the boss (insert depth).

Keep-out footprint for the PSU body: rectangle **(13, 23) mm** → **(64, 85.5) mm**.

## 4. Floor bosses — input PCB (M2)

Standoff: boss top at **Z ≥ 12 mm** (~9 mm up from floor).  
Verified from Gerber Ø2.54 drill → LED-edge→FRONT transform → outer XY (2026-07-18).

| Boss | Gerber (mil) | Outer X (mm) | Outer Y (mm) | Notes |
| --- | ---: | ---: | ---: | --- |
| H1 | **1020, −725** | **96.41** | **100.15** | nearest `J1` |
| H2 | **3115, −160** | **82.06** | **46.94** | LED edge |
| H3 | **925, −2955** | **153.06** | **102.56** | |
| H4 | **3115, −2960** | **153.18** | **46.94** | LED edge |
| — | 1700, −2145 | — | — | **no boss** (5th hole) |

For each H1–H4:

1. Sketch on floor Z = 3 mm: circle Ø**7 mm**.  
2. Extrude **+join** to Z = **12 mm**.  
3. Sketch on boss top: circle Ø**3.2 mm**.  
4. Extrude **−cut** through boss (M2 heat-set).

## 5. LEFT wall — panel cutouts (cut through X = 0 mm wall)

Sketch on the **LEFT outer face** (plane X = 0 mm). Y from front, Z from bottom. Extrude **−cut** through the **3 mm** wall (+X).

| Feature | Shape | Center Y, Z (mm) | Size (mm) |
| --- | --- | ---: | --- |
| C14 inlet | rectangle | **(35, 35)** | **27.5 × 20** (Y × Z) |
| POWER KCD4 | rectangle | **(60, 50)** | **30 × 22** |
| M12-5 | circle | **(85, 50)** | Ø**16.2** |

## 6. FRONT wall — LED window

Sketch on **FRONT outer face** (plane Y = 0 mm):

1. Rectangle **40 × 10 mm**, center at **(X = 110 mm, Z = 50 mm)**.  
2. Extrude **−cut** through the wall.  
3. Optional: on the **inside** of FRONT, sketch ~**42 × 12 mm** and extrude **−cut** a recess ~**1.5 mm** deep for acrylic + foam.

## 7. BACK — hinge bosses (body leaf, M2)

Two bociloy ~**25 mm** (“1 inch”) hinges. Pin midpoints (outer XY); pin axis parallel to +X.

| Hinge | Pin mid X (mm) | Pin mid Y (mm) |
| --- | ---: | ---: |
| A | **43** | **173** |
| B | **177** | **173** |

Boss hole center = **7.5 mm toward FRONT** from the pin (smaller Y):

| Hinge | Boss X (mm) | Boss Y (mm) |
| --- | ---: | ---: |
| A | **43** | **165.5** |
| B | **177** | **165.5** |

For each boss:

1. Sketch circle Ø**7 mm** at boss XY.  
2. Extrude **+join** ≥**5 mm** (M2×3 mm insert).  
3. Sketch Ø**3.2 mm** · extrude **−cut**.  

Leave ~**20 × 30 mm** clear around each hinge knuckle.

## 8. FRONT — latch inserts (M3)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

1. Add a short flange/rail on the inner FRONT if needed.  
2. Sketch Ø**9 mm** · extrude **+join** ≥**6 mm**.  
3. Sketch Ø**4.2 mm** · extrude **−cut** (M3 heat-set).

## 9. Export

Save as `sign-input-body.stl`. Arcade holes are on the [`LID.md`](LID.md).
