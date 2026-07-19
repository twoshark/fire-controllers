# sign-input — LID

Export: `sign-input-lid.stl` · Print with **gasket groove up**.  
Coords: **outer base** (0,0) = front-left corner of the lid (same footprint as body). See [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | mm |
| --- | ---: |
| L × W | **220 × 180** |
| Thickness | **5** (use 4–5) |

---

## 1. Lid plate

1. Sketch on XY at Z=0: rectangle **220 × 180** from **(0, 0)**.  
2. Extrude **+join** to **Z = 5**.

## 2. Gasket groove (print this face up)

Groove on the face that will crush the foam (print-up = groove face).

1. Sketch on **Z = 5** (top of plate):  
   - Outer path: inset ~**4 mm** from each edge (rectangle ~212 × 172).  
   - Inner path: offset **3.5 mm** inward from that path (groove width).  
   - On the **BACK** edge: pull the groove **inboard** (~15–20 mm from Y=180) so it clears hinge knuckles.  
2. Extrude **−cut** **2.0 mm** deep (groove depth).

## 3. Hinge bosses (lid leaf, M2) — underside

Pin mids match the body (outer XY):

| Hinge | Pin mid X | Pin mid Y |
| --- | ---: | ---: |
| A | **43** | **173** |
| B | **177** | **173** |

Boss centers **7.5 mm toward FRONT**:

| Hinge | Boss X | Boss Y |
| --- | ---: | ---: |
| A | **43** | **165.5** |
| B | **177** | **165.5** |

Work on the **underside** (Z=0 face after flip, or sketch on Z=0 and extrude −Z before export):

1. Sketch circle Ø**7** at boss XY.  
2. Extrude **+join** ≥5 mm into the lid thickness / down as a pad.  
3. Sketch Ø**3.2** · extrude **−cut** for M2 heat-set.

## 4. Latch holes (M3 clearance)

Through-holes so M3 screws reach the body inserts:

| Latch | X | Y |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

1. Sketch on lid top: circle Ø**3.4** (clearance) at each XY.  
2. Extrude **−cut** through the lid.  
3. Optional: Ø**6** counterbore ~1.5 mm for screw head.

## 5. Arcade button holes (through)

Hole Ø**28** (measure your EG STARTS ring — may be Ø24).  
Ring center (outer XY): **(110, 90)**.

| Button | Center X | Center Y |
| --- | ---: | ---: |
| ALL | **110.0** | **90.0** |
| CH1 | **110.0** | **51.7** |
| CH2 | **146.4** | **78.2** |
| CH3 | **132.5** | **121.0** |
| CH4 | **87.5** | **121.0** |
| CH5 | **73.6** | **78.2** |

For each:

1. Sketch on lid top: circle Ø**28** at the center.  
2. Extrude **−cut** through the lid.

## 6. Optional — buttons PCB bosses

Under the lid: 2–4× M2 bosses (Ø7 / hole Ø3.2) for the buttons daughter. Orient so the harness can reach the input board.

## 7. Export

`sign-input-lid.stl`. No C14 / M12 / LED on the lid — those are on the body.
