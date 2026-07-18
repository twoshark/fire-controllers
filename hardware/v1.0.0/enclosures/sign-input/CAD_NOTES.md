# sign-input — CAD

Refs: [`../CAD_VERIFICATION.md`](../CAD_VERIFICATION.md) · [`../MOUNTING.md`](../MOUNTING.md) · [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md) · [`../SEALING.md`](../SEALING.md).

Bed ≤ **256 × 256**. Body open-up; lid groove-up; DT/M12 retainers = **separate clips**.

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 180 × 90** |
| Inner (3 mm wall) | **214 × 174 × 84** |
| Wall | 3.0 |

## Top — EG STARTS hex ring (ALL + CH1..5)

Pitch **45** on R≈38.3. Hole bbox ≈ **110 × 110** — centers in [`../MOUNTING.md`](../MOUNTING.md).

| Dim | Value |
| --- | ---: |
| Button hole | **Ø28** (verify ring; may be Ø24) |

## Sides (CL from outer bottom · Y from FRONT)

| Feature | Cutout | CL | Wall | Y |
| --- | --- | ---: | --- | ---: |
| C14 | 27.5×20 | **35** | **LEFT** | **35** |
| POWER KCD4+boot | 30×22 | **50** | **LEFT** | **60** |
| M12-5 | Ø16.2 | **50** | **LEFT** | **85** |
| LED 40×10 + PC lens | — | **50** | **FRONT** | mid |

## Internal

| Item | Spec |
| --- | --- |
| RS-15-12 | Body (10,20)–(61,82.5) · M3 @ (35.5, 31.5)/(35.5, 70.6) |
| Input PCB | `(ox,oy)=(75,40)` · H1–H4 · LED→FRONT · Z=12 |
| Buttons PCB | optional under lid · 2–4× M2 |
| Lid | **2× bociloy 1"** internal @ BACK pin (40/174, Y170) · M2 hinge bosses @ mid-pin Y+7.5 into cavity · 2× M3 latch FRONT |

## Cables

C13→C14 **6 ft**. M12 dust cap when unused.

## DFAM

No supports on body/lid. Panel connectors use **separate retainer clips**.
