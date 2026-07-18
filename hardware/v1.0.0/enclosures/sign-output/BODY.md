# sign-output — BODY (box without lid)

Export: `sign-output-body.stl` · Print: open face **up**.  
Shared: [`../CAD_PARTS.md`](../CAD_PARTS.md) · [`../MOUNTING.md`](../MOUNTING.md) · [`../POWER_OTS.md`](../POWER_OTS.md).

**No** C14 / KCD4 on this box — 12 V from outdoor **HLG-240H-12** via DTP.

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 170 × 85** |
| Wall / floor | **3.0** |
| Inner L × W × H | **214 × 164 × 79** |

## Build steps

1. **Outer** 220 × 170 × 85.  
2. **Cavity** open-top: 214 × 164 × 79.  
3. **Output PCB** @ `(25, 25)` · Z=12 · M2 bosses:

| Boss | Inner X | Inner Y |
| --- | ---: | ---: |
| H1 | 53.2 | 131.0 |
| H2 | 28.0 | 28.2 |
| H3 | 132.9 | 145.3 |
| H4 | 132.9 | 28.2 |

Outline keep-out ≈ X[25, 137.4] × Y[25, 148.9]. LED edge → FRONT.  
4. **BACK** — DTP04-2P pocket ≈**18 × 22** + lip · CL H **40** · mid-X. Separate retainer clip.  
5. **LEFT** cutouts (CL H · Y from FRONT):

| Feature | Cutout | CL H | Y |
| --- | --- | ---: | ---: |
| M12-5 | Ø16.2 | **50** | **30** |
| SOL0 | ≈16 × 18 pocket | **40** | **55** |
| SOL1 | same | **40** | **80** |
| SOL2 | same | **40** | **105** |
| SOL3 | same | **40** | **130** |
| SOL4 | same | **40** | **155** |

Pitch ≥25. Day-1 uses SOL0..4 only (5 channels).  
6. **FRONT** LED **40 × 10** @ CL H **50**, mid-X + acrylic recess.  
7. **Hinges** (body) pin mid:

| Hinge | Inner (X, Y) |
| --- | ---: |
| A | **(40, 160)** |
| B | **(174, 160)** |

M2 mid-pin · 7.5 mm into cavity.  
8. **Latch M3** @ ≈ **(8, 8)** / **(206, 8)**.  
9. Service clear under open lid: USB-C, SWD, `F9`.

## Not on body

No arcade holes. Lid = plain service cover → [`LID.md`](LID.md).
