# mp-output — BODY

**All sizes in mm.** CAD document units = mm.  
Export: `mp-output-body.stl` · Print cavity **up**.  
Coords: **outer (0, 0) mm** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).  
Same size as sign-output; only **SOL0..2** (3 channels).

## Envelope

| | Size |
| --- | ---: |
| L × W × H | **220 × 170 × 85 mm** |
| Wall / floor | **3.0 mm** |

---

## 1. Outer box

1. Sketch rectangle **220 × 170 mm** from **(0, 0) mm**.  
2. Extrude **+join** to Z = **85 mm**.

## 2. Hollow cavity

1. Sketch on Z = 3 mm: rectangle from **(3, 3) mm**, size **214 × 164 mm**.  
2. Extrude **−cut** through the top.

## 3. Floor bosses — output PCB (M2)

Boss top Z = **12 mm**. CW LED-edge→FRONT — same as sign-output ([`../MOUNTING.md`](../MOUNTING.md)).

| Boss | Gerber (mm) | Outer X (mm) | Outer Y (mm) |
| --- | ---: | ---: | ---: |
| H1 | (17.907, −28.194) | **112.20** | **134.05** |
| H2 | (120.777, −3.048) | **137.35** | **31.17** |
| H3 | (3.640, −107.950) | **32.45** | **148.31** |
| H4 | (120.777, −107.950) | **32.45** | **31.17** |

Each: Ø**7 mm** → Z=12 → Ø**3.2 mm** cut.

## 4. BACK — DTP pocket

1. Sketch on Y = 170 mm: rect ≈**18 × 22 mm**, center **(X = 110 mm, Z = 40 mm)**.  
2. Extrude **−cut** through **3 mm** wall. Retainer clip separate.

## 5. LEFT — M12 + SOL0..2 only

Sketch on X = 0 mm · extrude **−cut**:

| Feature | Center Y, Z (mm) | Shape (mm) |
| --- | ---: | --- |
| M12-5 | **(30, 50)** | Ø**16.2** |
| SOL0 | **(55, 40)** | ≈**16 × 18** |
| SOL1 | **(80, 40)** | ≈**16 × 18** |
| SOL2 | **(105, 40)** | ≈**16 × 18** |

Do **not** cut SOL3/SOL4.

## 6. FRONT — no LED window

LEDs face **up**; window is in the **lid** ([`LID.md`](LID.md) · [`../LED_WINDOW.md`](../LED_WINDOW.md)).

## 7. Hinge bosses (M2)

| Hinge | Pin mid (X, Y) mm | Boss (X, Y) mm |
| --- | ---: | ---: |
| A | **(43, 163)** | **(43, 155.5)** |
| B | **(177, 163)** | **(177, 155.5)** |

Ø**7 mm** → Ø**3.2 mm** cut (≥**5 mm** tall).

## 8. Latch inserts (M3)

| Latch | X (mm) | Y (mm) |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

Ø**9 mm** → Ø**4.2 mm** cut (≥**6 mm** tall).

## 9. Export

`mp-output-body.stl`. Lid → [`LID.md`](LID.md).
