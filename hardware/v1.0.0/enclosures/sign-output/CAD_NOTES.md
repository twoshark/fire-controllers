# sign-output — CAD

Orientation: [`../README.md`](../README.md). Cutouts: [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md). Seal: [`../SEALING.md`](../SEALING.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 160 × 80** |
| Wall | 2.5–3.0 |
| Lid | openable + gasket |

**No POWER rocker** — kill 12 V via sign-output-power AC switch.

## Sides

```text
12V MATE FACE:  [DTP04-2P pocket ≈18×22]  CL = 40  (centered)
OTHER:          [M12-5 Ø16.2] CL 50  [LED 40×10] CL 50
SOL face:       [SOL0..SOL4] DT pockets  CL = 40  pitch ≥25
```

| Feature | Cutout | CL |
| --- | --- | ---: |
| DTP IN | pocket ≈18×22 | **40** |
| M12-5 | Ø16.2 | **50** |
| LED | 40×10 + lens | **50** |
| SOL0..4 | pocket ≈16×18 | **40** |

## Internal

| Item | Place |
| --- | --- |
| Output PCB | keep-out **126 × 114**; pillars + M3 inserts |
| DTP → `J1` | 12 AWG short |
| `J6` → SOL | star · 12 AWG stub → 18 AWG/ch |

## Cables

DTP jumper from sign-output-power **≤ 4 ft**.
