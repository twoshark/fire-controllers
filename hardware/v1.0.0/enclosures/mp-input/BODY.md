# mp-input — BODY

**All sizes in mm.** CAD document units = mm.  
Export: `mp-input-body.stl` · Print cavity **up**.  
Coords: **outer (0, 0) mm** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).

## Envelope

| | Size |
| --- | ---: |
| L × W × H | **200 × 160 × 90 mm** |
| Wall / floor | **3.0 mm** |

---

## 1. Outer box

1. Sketch XY Z = 0 mm: rectangle **200 × 160 mm** from **(0, 0) mm**.  
2. Extrude **+join** to Z = **90 mm**.

## 2. Hollow cavity

1. Sketch on Z = 3 mm: rectangle from **(3, 3) mm**, size **194 × 154 mm**.  
2. Extrude **−cut** through the top.

## 3. Floor bosses — RS-15 (M3)

| Boss | X (mm) | Y (mm) |
| --- | ---: | ---: |
| M3-A | **38.5** | **29.5** |
| M3-B | **38.5** | **68.6** |

Each: sketch Ø**9 mm** on floor → extrude **+join** **+6 mm** → sketch Ø**4.2 mm** → extrude **−cut**.  
PSU keep-out: **(13, 18) mm** → **(64, 80.5) mm**.

## 4. Floor bosses — input PCB (M2)

Boss top Z = **12 mm**. CW LED-edge→FRONT ([`../MOUNTING.md`](../MOUNTING.md)); origin shifted vs sign-input.

| Boss | Gerber (mil) | Outer X (mm) | Outer Y (mm) |
| --- | ---: | ---: | ---: |
| H1 | **1020, −725** | **128.58** | **85.15** |
| H2 | **3115, −160** | **142.93** | **31.94** |
| H3 | **925, −2955** | **71.94** | **87.56** |
| H4 | **3115, −2960** | **71.81** | **31.94** |

Δ from H1 same as sign-input. Each: Ø**7 mm** → Z=12 → Ø**3.2 mm** cut.

## 5. LEFT wall cutouts

Sketch on plane X = 0 mm. Extrude **−cut** through **3 mm** wall. Centers (Y, Z) in mm:

| Feature | Center Y, Z (mm) | Shape (mm) |
| --- | ---: | --- |
| C14 | **(30, 35)** | rect **27.5 × 20** |
| KCD4 | **(55, 50)** | rect **30 × 22** |
| M12-5 | **(80, 50)** | circle Ø**16.2** |

## 6. FRONT — no LED window

LEDs face **up**; window is in the **lid** ([`LID.md`](LID.md) · [`../LED_WINDOW.md`](../LED_WINDOW.md)).

## 7. BACK — hinge bosses (M2)

| Hinge | Pin mid (X, Y) mm | Boss (X, Y) mm |
| --- | ---: | ---: |
| A | **(38, 153)** | **(38, 145.5)** |
| B | **(162, 153)** | **(162, 145.5)** |

Boss = pin − **7.5 mm** in Y. Each: Ø**7 mm** → Ø**3.2 mm** cut.

## 8. FRONT — latch inserts (M3)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **189** | **11** |

Ø**9 mm** boss → Ø**4.2 mm** cut (≥**6 mm** tall).

## 9. Export

`mp-input-body.stl`. Arcade → [`LID.md`](LID.md).
