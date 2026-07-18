# mp-input — BODY (box without lid)

Export: `mp-input-body.stl` · Print: open face **up**.  
Shared: [`../CAD_PARTS.md`](../CAD_PARTS.md) · [`../MOUNTING.md`](../MOUNTING.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **200 × 160 × 90** |
| Wall / floor | **3.0** |
| Inner L × W × H | **194 × 154 × 84** |

## Build steps

1. **Outer** 200 × 160 × 90.  
2. **Cavity** open-top: inner 194 × 154 × 84 (floor Z=3).  
3. **RS-15-12** — footprint (10, 15) → (61, 77.5); M3 @ **(35.5, 26.5)** / **(35.5, 65.6)**.  
4. **Input PCB** @ `(70, 30)` · Z=12 · M2:

| Boss | Inner X | Inner Y |
| --- | ---: | ---: |
| H1 | 88.4 | 87.2 |
| H2 | 74.1 | 33.9 |
| H3 | 145.1 | 89.6 |
| H4 | 145.2 | 33.9 |

Channels D4..D8 unused in wiring — still mount full board.  
5. **LEFT** cutouts (CL H from outer bottom · Y from FRONT):

| Feature | Cutout | CL H | Y |
| --- | --- | ---: | ---: |
| C14 | 27.5 × 20 | **35** | **30** |
| KCD4+boot | 30 × 22 | **50** | **55** |
| M12-5 | Ø16.2 | **50** | **80** |

6. **FRONT** LED **40 × 10** @ CL H **50**, mid-X + acrylic recess.  
7. **Hinges** (body leaf) pin mid:

| Hinge | Inner (X, Y) |
| --- | ---: |
| A | **(35, 150)** |
| B | **(159, 150)** |

M2 @ mid-pin · Y+7.5 into cavity.  
8. **Latch M3** body @ ≈ **(8, 8)** and **(186, 8)**.  
9. Retainer clip for M12 (separate print).

## Not on body

Arcade layout → [`LID.md`](LID.md).
