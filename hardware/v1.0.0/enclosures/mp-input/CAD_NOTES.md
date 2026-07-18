# mp-input — CAD

Printer bed ≤ 256 × 256 mm.

## Size

**200 × 140 × 85 mm** (L×W×H)

## Front panel (operator)

```text
ROW:  [POWER] [CH1] [CH2] [CH3] [ALL]
```

| Control | Maps to |
| --- | --- |
| Arcade POWER | C14 hot → RS-15 AC/L |
| CH1..CH3 | MCU CH0..CH2 |
| ALL | buttons PCB D1..D3 only |
| RESET / BOOT | Ø16 mm · Adafruit 559 / 481 → NRST / BOOT0 |

Pitch ≥30 mm. Leave D4..D8 on buttons PCB unwired.

## Connector wall

```text
[C14] [M12-5 RS-485] [HangTon USB]
```

| Feature | Cutout |
| --- | --- |
| IEC C14 | per inlet drawing |
| M12-5 | Ø16.2 mm |
| HangTon | D-cut per part |
| LED window | **40 × 10 mm** vertical · LED column |

## Internal layout

Same PCB orientation as [`../sign-input/CAD_NOTES.md`](../sign-input/CAD_NOTES.md):

| Edge | Toward |
| --- | --- |
| `CN2` | M12-5 |
| `J5` | HangTon |
| `J1` | RS-15-12 |
| LED column | window |
| `J2a` (CH0..2 used) | front buttons |

RS-15-12: 62.5 × 51 × 28 mm · 2×M3 · next to C14/POWER.

PCB keep-out ≈ **80 × 75 mm** + 3 mm. No PCB mounting holes in v1.0.0.

## Print

ASA or PETG · wall ≥3 mm · gasketed lid.
