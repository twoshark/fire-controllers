# mp-output — BODY

Export: `mp-output-body.stl` · Print cavity **up**.  
Coords: **outer base (0,0)** = front-left. [`../CAD_PARTS.md`](../CAD_PARTS.md).  
Same size as sign-output; only **SOL0..2** (3 channels).

## Envelope

| | mm |
| --- | ---: |
| L × W × H | **220 × 170 × 85** |
| Wall / floor | **3.0** |

---

## 1. Outer box

1. Sketch rectangle **220 × 170** from **(0, 0)**.  
2. Extrude **+join** to Z=**85**.

## 2. Hollow cavity

1. Sketch on Z=3: rectangle from **(3, 3)** size **214 × 164**.  
2. Extrude **−cut** through the top.

## 3. Floor bosses — output PCB (M2)

Boss top Z=**12**:

| Boss | X | Y |
| --- | ---: | ---: |
| H1 | **56.2** | **134.0** |
| H2 | **31.0** | **31.2** |
| H3 | **135.9** | **148.3** |
| H4 | **135.9** | **31.2** |

Each: Ø**7** → extrude to Z=12 → Ø**3.2** cut.

## 4. BACK — DTP pocket

1. Sketch on Y=170: rect ≈**18 × 22**, center **(X=110, Z=40)**.  
2. Extrude **−cut** through. Retainer clip separate.

## 5. LEFT — M12 + SOL0..2 only

Sketch on X=0 · extrude **−cut**:

| Feature | Center (Y, Z) | Shape |
| --- | ---: | --- |
| M12-5 | **(30, 50)** | Ø**16.2** |
| SOL0 | **(55, 40)** | ≈**16 × 18** |
| SOL1 | **(80, 40)** | ≈**16 × 18** |
| SOL2 | **(105, 40)** | ≈**16 × 18** |

Do **not** cut SOL3/SOL4.

## 6. FRONT — LED window

1. Sketch on Y=0: **40 × 10** centered at **(110, Z=50)**.  
2. Extrude **−cut** through.

## 7. Hinge bosses (M2)

| Hinge | Pin mid (X, Y) | Boss (X, Y) |
| --- | ---: | ---: |
| A | **(43, 163)** | **(43, 155.5)** |
| B | **(177, 163)** | **(177, 155.5)** |

Ø**7** → Ø**3.2** cut.

## 8. Latch inserts (M3)

| Latch | X | Y |
| --- | ---: | ---: |
| L | **11** | **11** |
| R | **209** | **11** |

Ø**9** → Ø**4.2** cut.

## 9. Export

`mp-output-body.stl`. Lid → [`LID.md`](LID.md).
