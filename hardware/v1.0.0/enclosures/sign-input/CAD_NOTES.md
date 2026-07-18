# sign-input — CAD

Printer bed ≤ 256 × 256 mm.

## Size

**220 × 150 × 85 mm** (L×W×H)

## Front panel (operator)

```text
One row (pitch ≥30 mm; needs face ≥220 mm):
  [POWER] [CH1] [CH2] [CH3] [CH4] [CH5] [ALL]

Or two rows:
  [POWER] [CH1] [CH2] [CH3]
  [CH4]   [CH5] [ALL]
```

| Control | Hole | Maps to |
| --- | --- | --- |
| Arcade POWER | per button (AC-rated) | C14 hot → RS-15 AC/L |
| CH1..CH5 | per arcade | MCU CH0..CH4 (`J2a.1`..`J2b.1`) |
| ALL | per arcade | buttons PCB D1..D5 → CH0..CH4 |
| RESET | Ø16 mm · Adafruit 559 | NRST |
| BOOT | Ø16 mm · Adafruit 481 | BOOT0 |

Pitch arcade buttons ≥30 mm center-to-center. RESET/BOOT on a side or under a flap — not in the fire-button row.

## Connector wall (rear or short end)

```text
[C14] [M12-5 RS-485] [HangTon USB]     LED window on adjacent wall
```

| Feature | Cutout |
| --- | --- |
| IEC C14 | per inlet drawing |
| M12-5 female | Ø16.2 mm · RS-485 |
| HangTon USB-C | D-cut per HangTon |
| LED window | **40 × 10 mm** (vertical) · align to PCB LED column |

## Internal layout

```text
FLOOR (top view, connector wall at top):

  [C14/POWER area]     [RS-15-12 62.5×51×28]
                              ↓ 18 AWG
  [J2 / buttons wires]   INPUT PCB ~80×75 keep-out
                         LEDs → LED window wall
                         CN2 → M12
                         J5  → HangTon
                         J1  → RS-15
```

### PCB orientation (from PnP centroids)

| Edge | Parts | Face toward |
| --- | --- | --- |
| Left X≈6 | `J1`, `J3`, `J2b`, `J2a` | Button wiring / open floor |
| Top Y≈−5 | `CN2` | M12-5 |
| Bottom Y≈−74 | `J5` USB, `J6` SWD | HangTon / service access |
| Right X≈79 | `LED10`..`LED1` (Y −18..−49) | LED window |

PnP bbox ≈ **80 × 75 mm** (centroids). Add ≥3 mm outline margin until Edge.Cuts exported. **No mounting holes** on v1.0.0 — 4× corner standoffs or clips.

### RS-15-12

| Spec | Value |
| --- | --- |
| Body | 62.5 × 51 × 28 mm |
| Mount | 2×M3 (Mean Well Case 971A) |
| Place | Adjacent to C14/POWER · AC leads short |

### Keep-outs

| Item | Clearance |
| --- | --- |
| M12 rear screw terminals | ≥35 mm |
| HangTon + USB-C jumper | ≥40 mm to `J5` |
| Arcade switch depth | per button (often ≥40 mm behind panel) |
| LED window | center on LED column mid Y ≈ −33 mm relative to PCB |

## Print

ASA or PETG · wall ≥3 mm · gasketed lid · LED window clear insert or open slot with recess for diffuser.
