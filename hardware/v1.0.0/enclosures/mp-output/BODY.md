# mp-output — BODY (box without lid)

Export: `mp-output-body.stl` · Print: open face **up**.  
Shared: [`../CAD_PARTS.md`](../CAD_PARTS.md) · [`../MOUNTING.md`](../MOUNTING.md).

Same envelope and PCB as sign-output; **3 SOL** pockets only (SOL0..2).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 170 × 85** |
| Wall / floor | **3.0** |
| Inner L × W × H | **214 × 164 × 79** |

## Build steps

1. **Outer** 220 × 170 × 85 · open-top cavity 214 × 164 × 79.  
2. **Output PCB** @ `(25, 25)` · Z=12 · same H1–H4 as sign-output:

| Boss | Inner X | Inner Y |
| --- | ---: | ---: |
| H1 | 53.2 | 131.0 |
| H2 | 28.0 | 28.2 |
| H3 | 132.9 | 145.3 |
| H4 | 132.9 | 28.2 |

3. **BACK** — DTP pocket ≈18×22 · CL H **40** · mid-X · retainer clip.  
4. **LEFT**:

| Feature | Cutout | CL H | Y |
| --- | --- | ---: | ---: |
| M12-5 | Ø16.2 | **50** | **30** |
| SOL0 | ≈16 × 18 | **40** | **55** |
| SOL1 | same | **40** | **80** |
| SOL2 | same | **40** | **105** |

Do **not** cut SOL3/SOL4 (day-1 mp = 3 channels).  
5. **FRONT** LED 40×10 @ CL H 50.  
6. **Hinges** pins **(40, 160)** / **(174, 160)** · M2 @ mid-pin Y+7.5.  
7. **Latch M3** **(8, 8)** / **(206, 8)**.

## Not on body

Lid → [`LID.md`](LID.md). HLG is OTS outdoor — not inside this print.
