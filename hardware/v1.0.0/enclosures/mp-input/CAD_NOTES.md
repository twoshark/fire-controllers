# mp-input — CAD

Orientation: [`../README.md`](../README.md). Cutouts: [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md). Bed ≤ **256 × 256**.

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **200 × 160 × 90** |
| Wall | 2.5–3.0 |
| Lid | openable (top) |

## Top — triangle + ALL center

```text
            [CH1]
      [CH3]  [ALL]  [CH2]
```

| Dim | Value |
| --- | ---: |
| Button hole | **Ø28** (verify adapter ring; may be Ø24) |
| Pitch CH↔CH / CH↔ALL | **≥50** |
| Triangle bounding box | ~**100 × 90** |

Same EG STARTS part as sign-input. LED **12 V** from RS-15. D4..D8 unused.

## Sides

Heights = hole center from **outer bottom**.

```text
FRONT (LED):  [LED 40×10] CL 50
LEFT (AC):    [C14] CL 35  [POWER KCD4] CL 50
RIGHT:        [M12-5 Ø16.2] CL 50
```

| Feature | Cutout | CL height |
| --- | --- | ---: |
| C14 | 27.5×20 or flange 27×19 | **35** |
| POWER KCD4 | 30×22 | **50** |
| M12-5 | Ø16.2 | **50** |
| LED window | 40×10 | **50** |

No panel RESET/BOOT/USB.

## Internal

Same input PCB keep-out **86 × 83** and RS-15 **62.5 × 51 × 28** as sign-input.

## Cables (external)

| Cable | Length |
| --- | ---: |
| C13→C14 mains | **6 ft** |
