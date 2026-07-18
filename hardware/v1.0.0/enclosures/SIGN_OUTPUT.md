# sign-output enclosure

Output PCB v1.0.0 + LRS-200-12. Five 12 V solenoids over Hotline from sign-input.

Shopping: [`SHOPPING_LIST.md`](SHOPPING_LIST.md).

## Channel map

| Function | MCU CH | Board | Panel |
| --- | ---: | --- | --- |
| Solenoid 1..5 | CH0..CH4 | `J6` + `J5a/b` | M12-8 SOL |

## BOM

| Assembly | Qty |
| --- | ---: |
| Output PCB v1.0.0 | 1 |
| LRS-200-12 | 1 |
| WRG32F2FBBNN POWER | 1 |
| Adafruit 559 + 481 | 1 / 1 |
| IEC C14 inlet | 1 |
| M12-5 RS-485 | 1 |
| M12-8 SOL | 1 |
| HangTon USB-C + jumper | 1 |
| Shell + gasket | 1 |

## Pin maps

**SOL M12-8:** 1–2 = +12V paralleled; 3–7 = OUT0..4; 8 = NC.  
**RS-485 M12-5:** [`WIRING.md`](WIRING.md).

## Power

| Domain | Budget |
| --- | --- |
| 12 V | ≈10.3 A of 17 A; `F9` 25 A ATO |
| 120 VAC | ~4 A · C14 → POWER → LRS |

## Layout

```text
FRONT:  [POWER] [RESET] [BOOT] [LED window]
EDGE:   [C14] [M12 RS-485] [M12 SOL] [USB-C]
INSIDE: LRS-200 → J1
```

## Bring-up

1. LRS trim 12.0 V; selector 115 V.  
2. Seat `F9`.  
3. Verify 12.0 V at J1.  
4. CH0..CH4 follow serial.
