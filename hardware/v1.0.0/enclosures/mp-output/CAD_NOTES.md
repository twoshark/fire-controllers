# mp-output — CAD

Orientation: [`../README.md`](../README.md). Cutouts: [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **200 × 150 × 80** |
| Wall | 2.5–3.0 |
| Lid | openable (top) |

## Sides

Align mate face with mp-output-power (**H = 40**).

```text
12V MATE FACE:  [PanelPole2 IN Ø28.6]  CL = 40
SIDE:           [POWER KCD4 30×22]     CL = 50
OTHER:          [M12-5 Ø16.2] CL 50  [LED 40×10] CL 50
SOL face:       [SOL0][SOL1][SOL2]     CL = 40  pitch ≥25
```

| Feature | Cutout | CL height |
| --- | --- | ---: |
| PanelPole2 IN | Ø28.6 | **40** |
| POWER KCD4 | 30×22 | **50** |
| M12-5 | Ø16.2 | **50** |
| LED window | 40×10 | **50** |
| SOL0..SOL2 | pocket ≈16×18 | **40** |

Same output PCB keep-out **126 × 114**. Open lid for DFU/`F9`.

## Cables (external)

| Cable | Length |
| --- | ---: |
| Powerpole ← mp-output-power | **≤ 4 ft** |
