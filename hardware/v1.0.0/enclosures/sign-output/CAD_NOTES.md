# sign-output — CAD

Orientation: [`../README.md`](../README.md). Cutouts: [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 160 × 80** |
| Wall | 2.5–3.0 |
| Lid | openable (top only — no arcade) |

## Sides

Heights = hole center from **outer bottom**. Mate face aligns with sign-output-power.

```text
12V MATE FACE (short):  [PanelPole2 IN Ø28.6]  CL = 40  (centered)
SIDE:                   [POWER KCD4 30×22]     CL = 50
OTHER long:             [M12-5 Ø16.2] CL 50
                        [LED 40×10] CL 50
SOL face:               [SOL0][SOL1][SOL2][SOL3][SOL4]
```

| Feature | Cutout | CL height | Pitch / notes |
| --- | --- | ---: | --- |
| PanelPole2 IN | **Ø28.6** | **40** | Rear depth 22.2 behind wall |
| POWER KCD4 | 30×22 | **50** | Switches PanelPole + → `J1` |
| M12-5 | Ø16.2 | **50** | → `J2` |
| LED window | 40×10 | **50** | PCB LED edge X≈117; row 31.5 mm |
| SOL0..SOL4 | pocket ≈16×18 each | **40** | Centers **≥25** apart; row CL matches mate height |

SOL lean kit = free-hanging DT in printed pocket (or DT04-2P-L012 ≈22.5×17.2 + 2×M4).

No panel RESET/BOOT/USB — open lid → `J7` / `SW` / `F9`.

## Internal

| Item | Size / place |
| --- | --- |
| Output PCB | **126 × 114** keep-out |
| Wire to PanelPole | 12 AWG · short |
| SOL leads | 18 AWG · `J6` + `J5a`/`J5b` |

Orient: LEDs → window · `J5`/`J6` → SOL face · `J1` → mate face.

## Cables (external)

| Cable | Length |
| --- | ---: |
| Powerpole ← sign-output-power | **≤ 4 ft** (DIY 12 AWG) |
