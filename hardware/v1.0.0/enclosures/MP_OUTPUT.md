# mp-output enclosure

Output PCB v1.0.0 + LRS-150-12. Three 12 V solenoids over Hotline from mp-input.

Shopping: [`SHOPPING_LIST.md`](SHOPPING_LIST.md).

## Channel map

| Function | MCU CH | Panel |
| --- | ---: | --- |
| Solenoid 1..3 | CH0..CH2 | M12-5 SOL |

## BOM

| Assembly | Qty |
| --- | ---: |
| Output PCB v1.0.0 | 1 |
| LRS-150-12 | 1 |
| WRG32F2FBBNN POWER | 1 |
| Adafruit 559 + 481 | 1 / 1 |
| IEC C14 inlet | 1 |
| M12-5 RS-485 | 1 |
| M12-5 SOL | 1 |
| HangTon USB-C + jumper | 1 |
| Shell + gasket | 1 |

## SOL M12 pinout

| Pin | Net |
| ---: | --- |
| 1–2 | +12V paralleled |
| 3–5 | OUT0..2 |

## Power

| Domain | Budget |
| --- | --- |
| 12 V | ≈6.2 A of 12.5 A; `F9` 25 A ATO |
| 120 VAC | ~2.8 A · C14 → POWER → LRS |

## Layout

```text
FRONT:  [POWER] [RESET] [BOOT] [LED window]
EDGE:   [C14] [M12 RS-485] [M12 SOL] [USB-C]
INSIDE: LRS-150 → J1
```

## Bring-up

1. LRS trim 12.0 V; selector 115 V.  
2. Seat `F9`.  
3. Verify 12.0 V at J1.  
4. CH0..CH2 follow serial.
