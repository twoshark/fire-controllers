# Output Board (v1.0.0)

Exports: `exports/BOM_output.csv`, `PnP_output.csv`, `Netlist_output.asc`.
Pins: [`PIN_MAP.md`](PIN_MAP.md). Bring-up: [`BRINGUP.md`](BRINGUP.md).

## Connectors

| Ref | MPN | Function |
| --- | --- | --- |
| `J1` | `KF128-7.5-2P` | Board 12V in (24A) |
| `J2` | `DB128V-5.08-6P-GN-S` | RS-485 |
| `J3a`/`J3b` | `DB128V-5.08-4P-GN-S` | Overrides CH0..3 / CH4..7 |
| `J4` | `DB128V-5.08-2P-GN-S` | Override GND / GND |
| `J5a`/`J5b` | `DB128V-5.08-4P-GN-S` | Load low-side OUT0..3 / OUT4..7 |
| `J6` | `KF128-7.5-2P` | Load +12V (both poles `12V_MAIN`) |
| `J7` | USB-C | DFU / USB |
| `J8` | 2x5 1.27 mm | SWD |

No external load-GND terminal.

## Power

```text
J1.1 VIN
  -> F9 (178.6165.0002 ATO holder + 20A blade recommended, field-inserted)
       netlist blades: F9.3/4/5/6 = VIN; F9.1/2/7/8 = to Q9.2
  -> Q9 IPB110P06LM P-MOS reverse-polarity
       gate: R59 10k to GND + D1 MMSZ5240B to 12V_MAIN
  -> 12V_MAIN (C17+C35 470uF, C18 100uF)
  -> U4 AP63203 + L1 -> 3V3 (C19+C30 22uF)
```

## Per-channel outputs

| CH | Fuse | MOSFET | Flyback | Gate PD | Terminal |
| --- | --- | --- | --- | --- | --- |
| 0..7 | `F1`..`F8` `1812L200/16GR` | `Q1`..`Q8` IRLML6344 | `D3`,`D20`..`D26` SS34 | `R9`..`R16` | `J5a`/`J5b` |

```text
12V_MAIN -> J6 -> load -> J5 -> F -> Q.D -> Q.S -> GND (J1.2)
flyback across load; gate = MCU direct + 10k PD
```

## RS-485

Same as input: `U2A`=TX (`PA9`), `U2B`=RX (`PA10`), term `R58` on RX, TVS `D10`/`D11`.

## Overrides / LEDs / USB

See [`PIN_MAP.md`](PIN_MAP.md). USB ESD `D2`; CC `R55`/`R56`; series `R53`/`R54`.

## Major ICs

| Ref | Part |
| --- | --- |
| `U1` | `STM32G0B1CBT6` |
| `U2A`/`U2B` | `SP3485EN-L/TR` |
| `U4` | `AP63203WU-7` |
| `U5`/`U6` | `SN74LV14APWR` |
| `Q9` | `IPB110P06LM` |

Enclosure 12 V: Mean Well **HLG-185H-12** (IP67 OTS) → DTP → board `J1`. See `enclosures/POWER_OTS.md`.
