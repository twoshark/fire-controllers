# sign-input — CAD

Orientation: [`../README.md`](../README.md). Cutouts: [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md). Seal: [`../SEALING.md`](../SEALING.md). Bed ≤ **256 × 256**.

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 180 × 90** |
| Wall | 2.5–3.0 |
| Lid | openable (top) + silicone gasket |

## Top — EG STARTS hex (ALL at top)

```text
              [ALL]
         [CH1]     [CH2]
         [CH5]     [CH3]
              [CH4]
```

| Dim | Value |
| --- | ---: |
| Button hole | **Ø28** (verify adapter ring) |
| Pitch | **≥45** |
| Hex bbox | ~**110 × 100** |

LED **12 V** from RS-15. [B07R565HM6](https://www.amazon.com/dp/B07R565HM6).

## Sides

Heights = CL from outer bottom.

```text
FRONT (LED):  [LED 40×10 + PC lens] CL 50
LEFT (AC):    [C14] CL 35  [POWER KCD4+boot] CL 50
RIGHT:        [M12-5 Ø16.2 + dust cap] CL 50
```

| Feature | Cutout | CL |
| --- | --- | ---: |
| C14 | 27.5×20 or 27×19 | **35** |
| POWER | 30×22 | **50** |
| M12-5 | Ø16.2 | **50** |
| LED window | 40×10 | **50** |

No panel RESET/BOOT/USB.

## Internal

| Item | Place |
| --- | --- |
| Input PCB | keep-out **86 × 83**; pillars + M3 inserts at corners |
| RS-15-12 | **62.5 × 51 × 28** near C14/POWER |
| Buttons PCB | under top |

## Cables

| Cable | Length |
| --- | ---: |
| C13→C14 | **6 ft** |
