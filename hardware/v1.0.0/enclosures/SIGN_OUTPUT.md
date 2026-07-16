# sign-output enclosure

Field box: v1.0.0 **output PCB** + **LRS-200-12** (17 A) + Omron **G5LE** for 120 VAC glowflies.

Shopping: [`SHOPPING_LIST.md`](SHOPPING_LIST.md).  
**Budget:** ~$65 with LRS-200 + HangTon USB (PSU alone is $26 — see [`PARTS_BOM.md`](PARTS_BOM.md)).

## Role

Hotline v2 over RS-485. Drives **5× 12 V solenoids** (CH0..CH4) and **always-on glowflies** via **CH7** + hardwired override.

## Channel map

| Function | MCU CH | Board | Panel |
| --- | ---: | --- | --- |
| Solenoid 1..5 | CH0..CH4 | `J6` + `J5a.1..4` / `J5b.1` | GX16-8 multipin |
| unused | CH5, CH6 | open | — |
| **GLOW / CH8** | **CH7** | `J6` + `J5b.4` → relay coil | GX16-3 glow out |

Confirm: CH8 = MCU CH7. OVR7 strapped to GND → always on when POWER is on.

## BOM (this box)

| Assembly | Qty |
| --- | ---: |
| Output PCB v1.0.0 | 1 |
| Mean Well `LRS-200-12` (115 V, 12.0 V trim, **17 A**) | 1 |
| Omron `G5LE-14-DC12` | 1 |
| DigiKey `WRG32F2FBBNN` POWER rocker (seal cutout) | 1 |
| Adafruit **559** RESET + **481** BOOT | 1 / 1 |
| GX16-3 AC in (panel male) | 1 pair |
| GX16-3 glow out (panel female) | 1 pair |
| GX16-6 RS-485 | 1 pair |
| GX16-8 solenoid multipin | 1 pair |
| HangTon USB-C bulkhead + M–M jumper | 1 |
| Fuse **BK/GMC-5-R** (5 A TD) + holder | 1 |
| Printed shell + gasket | 1 |

## Solenoid GX16-8 pinout (paralleled +12 V)

| Pin | Net |
| ---: | --- |
| **1–2** | `+12V` (`J6`) paralleled |
| 3 | OUT0 |
| 4 | OUT1 |
| 5 | OUT2 |
| 6 | OUT3 |
| 7 | OUT4 |
| 8 | NC |

## Feature ↔ enclosure

```text
FRONT:  [POWER]  [RESET] [BOOT]  [LED window]
EDGE:   [GX16 AC IN] [GX16 GLOW] [GX16 RS-485] [GX16 SOL×5] [USB-C]
INSIDE: LRS-200 → J1/J6; G5LE coil on CH7; OVR7→GND; AC hot→relay COM
```

## Power budget @12 V

| Load | Current |
| --- | ---: |
| 5× solenoids ≤2 A | ≤10 A |
| Relay coil | ~30 mA |
| PCB | ≤200 mA |
| **Total** | **≈10.3 A** of **17 A** (LRS-200) |

Glowflies on **120 VAC** via relay — not on 12 V. LRS-200 AC ≈ **4 A @ 115 V** full → fuse **5 A** TD.

## Interaction

```text
Wall 120VAC → GX16-3 IN → fuse 5A → POWER → LRS-200
                              └──────────→ relay COM → NO → GX16 GLOW hot
AC N → LRS + GLOW N

Input box → GX16-6 RS-485 (crossover) → J2
CH0..CH4 → GX16-8 (pins 1–2 = +12V)
CH7 + OVR7→GND → G5LE → glowflies always on with POWER
```

## CAD

Start **300 × 220 × 110 mm** (fits LRS-200 215×115×30) — [`CAD_NOTES.md`](CAD_NOTES.md).

## Bring-up

1. 115 V selector; 12.0 V at J1.  
2. OVR7 strapped → glow live with POWER.  
3. CH0..CH4 follow serial only.  
4. Hose-test mated GX16 + caps.
