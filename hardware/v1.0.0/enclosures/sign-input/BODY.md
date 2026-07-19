# sign-input — BODY

Export: `sign-input-body.stl` · Print with open cavity **up**.  
Coords: **outer base** (0,0) = front-left corner of the box footprint. See [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | mm |
| --- | ---: |
| L × W × H | **220 × 180 × 90** |
| Wall / floor | **3.0** |

---

## 1. Outer box

1. Sketch on **XY at Z=0**: rectangle **220 × 180**, corner at **(0, 0)**.  
2. Extrude **+join** to **Z = 90**.

## 2. Hollow the cavity (open top)

1. Sketch on **floor top** (plane Z=3): rectangle from **(3, 3)** size **214 × 174**.  
2. Extrude **−cut** from Z=3 through the top (cut to Z=90).  
   Result: open-top shell, walls 3 mm, floor 3 mm.

## 3. Floor bosses — RS-15-12 (M3)

PSU sits on the floor, left side. Boss centers (outer XY):

| Boss | X | Y |
| --- | ---: | ---: |
| M3-A | **38.5** | **34.5** |
| M3-B | **38.5** | **73.6** |

For each:

1. Sketch on **floor (Z=3)**: circle Ø**9** (boss OD) at the XY above.  
2. Extrude **+join** **+6** mm (boss height).  
3. Sketch on boss top: circle Ø**4.2** (M3 heat-set).  
4. Extrude **−cut** through the boss (insert depth).

Keep-out footprint for the PSU body (do not put other bosses inside): rectangle **(13, 23)** → **(64, 85.5)**.

## 4. Floor bosses — input PCB (M2)

Standoff: boss top at **Z ≥ 12** (≈9 mm up from floor). Centers (outer XY):

| Boss | X | Y |
| --- | ---: | ---: |
| H1 | **96.4** | **100.2** |
| H2 | **82.1** | **46.9** |
| H3 | **153.1** | **102.6** |
| H4 | **153.2** | **46.9** |

For each:

1. Sketch on floor Z=3: circle Ø**7**.  
2. Extrude **+join** to Z=**12**.  
3. Sketch on boss top: circle Ø**3.2**.  
4. Extrude **−cut** through boss (M2 heat-set).

## 5. LEFT wall — panel cutouts (cut through X=0 wall)

Sketch on the **LEFT outer face** (plane X=0). Use Y from front, Z from bottom. Extrude **−cut** through the 3 mm wall (+X).

| Feature | Shape | Center (Y, Z) | Size |
| --- | --- | ---: | --- |
| C14 inlet | rectangle | **(35, 35)** | **27.5 × 20** (Y×Z) |
| POWER KCD4 | rectangle | **(60, 50)** | **30 × 22** |
| M12-5 | circle | **(85, 50)** | Ø**16.2** |

## 6. FRONT wall — LED window

Sketch on **FRONT outer face** (plane Y=0):

1. Rectangle **40 × 10**, center at **(X=110, Z=50)**.  
2. Extrude **−cut** through the wall.  
3. Optional: on the **inside** of FRONT, sketch a larger rectangle (~42×12) and extrude **−cut** a shallow recess (~1.5 mm) for acrylic + foam (not a through-slot wider than the outer aperture).

## 7. BACK — hinge bosses (body leaf, M2)

Two bociloy 1" hinges. Pin midpoints (outer XY); pin axis parallel to +X; leaves inside the box.

| Hinge | Pin mid X | Pin mid Y |
| --- | ---: | ---: |
| A | **43** | **173** |
| B | **177** | **173** |

Boss hole center = **7.5 mm toward FRONT** from the pin (smaller Y):

| Hinge | Boss X | Boss Y |
| --- | ---: | ---: |
| A | **43** | **165.5** |
| B | **177** | **165.5** |

For each boss (on floor or on a short pad up the BACK wall — leaf must sit flat):

1. Sketch on a plane facing the cavity: circle Ø**7** at boss XY.  
2. Extrude **+join** enough for M2×3 insert (≥5 mm).  
3. Sketch Ø**3.2** · extrude **−cut** for the heat-set.  

Leave **~20 × 30** clear around each hinge knuckle (no PCB/wire).

## 8. FRONT — latch inserts (M3)

Body receives the lid screws. Centers (outer XY), near FRONT corners:

| Latch | X | Y |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

1. Add a short flange/rail on the inner FRONT if needed so the insert is accessible from above.  
2. Sketch Ø**9** · extrude **+join** ≥6 mm.  
3. Sketch Ø**4.2** · extrude **−cut** (M3 heat-set).

## 9. Export

Save as `sign-input-body.stl`. No arcade holes on the body — those are on the [`LID.md`](LID.md).
