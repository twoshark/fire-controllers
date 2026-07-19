# sign-input — LID

**All sizes in mm.** CAD document units = mm.  
Export: `sign-input-lid.stl` · Print with **gasket groove up**.  
Coords: **outer base (0, 0) mm** = front-left. See [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | Size |
| --- | ---: |
| L × W | **220 × 180 mm** |
| Thickness | **5 mm** (4–5 mm OK) |

---

## 1. Lid plate

1. Sketch on XY at Z = 0 mm: rectangle **220 × 180 mm** from **(0, 0) mm**.  
2. Extrude **+join** to **Z = 5 mm**.

## 2. Gasket groove (print this face up)

1. Sketch on **Z = 5 mm**:  
   - Outer path: inset ~**4 mm** from each edge (~**212 × 172 mm**).  
   - Inner path: offset **3.5 mm** inward (groove width).  
   - On **BACK**: pull groove inboard ~**15–20 mm** from Y = 180 mm (clear hinges).  
2. Extrude **−cut** **2.0 mm** deep.

## 3. Hinge bosses (lid leaf, M2) — underside

| Hinge | Pin mid X (mm) | Pin mid Y (mm) | Boss X (mm) | Boss Y (mm) |
| --- | ---: | ---: | ---: | ---: |
| A | **43** | **173** | **43** | **165.5** |
| B | **177** | **173** | **177** | **165.5** |

Boss = **7.5 mm toward FRONT** from pin.

1. Sketch circle Ø**7 mm** at boss XY.  
2. Extrude **+join** ≥**5 mm**.  
3. Sketch Ø**3.2 mm** · extrude **−cut**.

## 4. Latch holes (M3 clearance)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

1. Sketch circle Ø**3.4 mm** at each XY.  
2. Extrude **−cut** through the lid.  
3. Optional: Ø**6 mm** counterbore ~**1.5 mm** deep for screw head.

## 5. Arcade button holes (through)

Hole Ø**28 mm** (verify EG STARTS ring — may be Ø**24 mm**).  
Ring center: **(110, 90) mm**.

| Button | Center X (mm) | Center Y (mm) |
| --- | ---: | ---: |
| ALL | **110.0** | **90.0** |
| CH1 | **110.0** | **51.7** |
| CH2 | **146.4** | **78.2** |
| CH3 | **132.5** | **121.0** |
| CH4 | **87.5** | **121.0** |
| CH5 | **73.6** | **78.2** |

For each: sketch circle Ø**28 mm** → extrude **−cut** through lid.

## 6. Optional — buttons PCB bosses

Under lid: 2–4× M2 bosses (OD Ø**7 mm** / hole Ø**3.2 mm**).

## 7. Export

`sign-input-lid.stl`.
