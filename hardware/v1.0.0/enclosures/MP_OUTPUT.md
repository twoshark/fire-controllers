# mp-output enclosure

Same architecture as [`SIGN_OUTPUT.md`](SIGN_OUTPUT.md), but **3 solenoid channels** and a smaller PSU.

**Budget:** ≤ ~$58 ex filament/arcade/PCB — see [`PARTS_BOM.md`](PARTS_BOM.md).  
Shopping: [`SHOPPING_LIST.md`](SHOPPING_LIST.md).  
PSU: **LRS-150-12** (12.5 A) — enough for 3×2 A with thermal margin (LRS-100 was too tight).

## Role

Receives Hotline from **mp-input** (or sign-input bits 0..2). Drives **3× 12 V solenoids** (CH0..CH2) and **always-on glowflies** via **CH7** + OVR7→GND.

## Channel map

| Function | MCU CH | Board | Panel |
| --- | ---: | --- | --- |
| Solenoid 1..3 | CH0..CH2 | `J6` + `J5a.1..3` | GX16-6 multipin |
| unused | CH3..CH6 | open | — |
| **GLOW / CH8** | **CH7** | `J6` + `J5b.4` → relay | GX16-3 glow out |

## BOM (this box)

| Assembly | Qty |
| --- | ---: |
| Output PCB v1.0.0 | 1 |
| Mean Well `LRS-150-12` (115 V, 12.0 V trim, **12.5 A**) | 1 |
| Omron `G5LE-14-DC12` | 1 |
| DigiKey `WRG32F2FBBNN` POWER | 1 |
| Adafruit **559** RESET + **481** BOOT | 1 / 1 |
| GX16-3 AC in (panel male) | 1 pair |
| GX16-3 glow out (panel female) | 1 pair |
| GX16-6 RS-485 | 1 pair |
| GX16-6 solenoid multipin | 1 pair |
| HangTon USB-C bulkhead + M–M jumper | 1 |
| Fuse **BK/GMC-3.15-R** (3.15 A TD) + holder | 1 |
| Printed shell + gasket | 1 |

Identical wiring rules to sign-output for AC, relay, override, RESET/BOOT — only solenoid count and PSU change.

## Solenoid GX16-6 pinout (paralleled +12 V)

| Pin | Net |
| ---: | --- |
| **1–2** | `+12V` (`J6`) paralleled |
| 3 | OUT0 |
| 4 | OUT1 |
| 5 | OUT2 |
| 6 | NC |

## Power budget @12 V

| Load | Current |
| --- | ---: |
| 3× solenoids ≤2 A | ≤6 A |
| Relay + PCB | ≤0.25 A |
| **Total** | **≈6.3 A** of **12.5 A** (LRS-150) |

LRS-150 AC ≈ **2.8 A @ 115 V** full → fuse **3.15 A** TD.

## Feature ↔ enclosure

```text
FRONT:  [POWER]  [RESET] [BOOT]  [LED window]
EDGE:   [GX16 AC IN] [GX16 GLOW] [GX16 RS-485] [GX16 SOL×3] [USB-C]
INSIDE: LRS-150 → J1/J6; G5LE on CH7; OVR7→GND
```

## CAD

Start **280 × 200 × 100 mm** (LRS-150) — [`CAD_NOTES.md`](CAD_NOTES.md).

## Bring-up

Same as sign-output, but only CH0..CH2 solenoids; ALL FIRE / mp-input bits must not assert CH3+.
