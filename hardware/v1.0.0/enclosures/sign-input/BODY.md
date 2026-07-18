# sign-input — BODY (box without lid)

Export: `sign-input-body.stl` · Print: open face **up**.  
Shared recipes: [`../CAD_PARTS.md`](../CAD_PARTS.md) · dims: [`../MOUNTING.md`](../MOUNTING.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 180 × 90** |
| Wall / floor | **3.0** |
| Inner L × W × H | **214 × 174 × 84** |

## Build steps

1. **Outer solid** 220 × 180 × 90.  
2. **Cavity** — subtract from Z=3 up: 214 × 174 × 87 (open top). Inner origin = outer (+3, +3, +3).  
3. **Floor — RS-15-12** (LEFT of PCB)  
   - Body footprint: (10, 20) → (61, 82.5) inner XY  
   - M3 bosses @ **(35.5, 31.5)** and **(35.5, 70.6)** · Ø4.2 · OD≥9 · H≥6  
4. **Floor — input PCB** @ `(ox,oy)=(75,40)` · standoff Z=12 · M2 bosses:

| Boss | Inner X | Inner Y |
| --- | ---: | ---: |
| H1 | 93.4 | 97.2 |
| H2 | 79.1 | 43.9 |
| H3 | 150.1 | 99.6 |
| H4 | 150.2 | 43.9 |

5. **LEFT wall cutouts** (CL height from **outer** bottom · Y from FRONT along face):

| Feature | Cutout | CL H | Y |
| --- | --- | ---: | ---: |
| C14 | 27.5 × 20 | **35** | **35** |
| POWER KCD4+boot | 30 × 22 | **50** | **60** |
| M12-5 | Ø16.2 | **50** | **85** |

6. **FRONT** — LED window **40 × 10** @ CL H **50**, mid-X; recess for 1 mm acrylic + foam (not an open slot).  
7. **Hinge — body leaf** (BACK, internal) · bociloy 1" · 2×:

| Hinge | Pin mid (inner X, Y) |
| --- | ---: |
| A | **(40, 170)** |
| B | **(174, 170)** |

M2 boss each: mid-pin · **7.5 mm** into cavity (−Y from BACK) · Ø3.2 · OD≥7. Keep-out **20 × 30** around each knuckle.  
8. **Front latch** — M3 heat-set in body flange/rail @ approx **(8, 8)** and **(206, 8)** inner (mate with lid screws).  
9. **DFAM** — no supports; M12 uses **separate retainer clip** (do not rely on unsupported pocket lip).

## Not on body

- Arcade button holes → [`LID.md`](LID.md)  
- Gasket groove → lid  
- Prefab hinges → buy, do not print  
