# Input Board (As-Built)

Exports: `exports/BOM_input.csv`, `PnP_input.csv`, `Netlist_input.asc`.
Pins: [`PIN_MAP.md`](PIN_MAP.md). Bring-up: [`BRINGUP.md`](BRINGUP.md).

## Connectors

| Ref | MPN | Function |
| --- | --- | --- |
| `J1` | `DB128V-5.08-2P-GN-S` | `VIN_12V_IN` / `GND` |
| `J2a` | `DB128V-5.08-4P-GN-S` | Inputs CH0..CH3 |
| `J2b` | `DB128V-5.08-4P-GN-S` | Inputs CH4..CH7 |
| `J3` | `DB128V-5.08-2P-GN-S` | Switch GND / GND |
| `CN2` | `DB128V-5.08-6P-GN-S` | RS-485 full duplex |
| `J5` | USB-C 16-pin | DFU / USB |
| `J6` | 2x5 1.27 mm | SWD |

### Connector nets

| Connector | Pin | Net |
| --- | --- | --- |
| `J1` | 1/2 | `VIN_12V_IN` / `GND` |
| `J2a` | 1..4 | `IN_CH0_RAW`..`IN_CH3_RAW` |
| `J2b` | 1..4 | `IN_CH4_RAW`..`IN_CH7_RAW` |
| `J3` | 1/2 | `GND` / `GND` |
| `CN2` | 1..6 | TX+, TX-, RX+, RX-, GND, SHIELD |
| `J6` | ARM 10-pin | VTREF, SWDIO, GND, SWCLK, … NRST |

## Power

```text
J1.1 -> D1 (SS34) -> F1 (BSMD1812-300-16V) -> 12V_MAIN
  bulk C17=100uF
  -> U4 AP63203WU-7 + L1 4.7uH -> 3V3
  bulk C18+C28 (22uF)
```

## RS-485

- `U2A` TX: `PA9` → DI; A/B on `CN2.1/2`; TVS `D10`
- `U2B` RX: RO → `PA10`; A/B on `CN2.3/4`; term `R27` 120R; TVS `D11`

## Channels / LEDs / USB

See [`PIN_MAP.md`](PIN_MAP.md). USB ESD `D12` (`USBLC6-2SC6`); CC pull-downs `R35`/`R36` 5.1k; D+/D- series `R31`/`R32` 22R.

## Major ICs

| Ref | Part |
| --- | --- |
| `U1` | `STM32G0B1CBT6` |
| `U2A`/`U2B` | `SP3485EN-L/TR` |
| `U4` | `AP63203WU-7` |
| `U5`/`U6` | `SN74LV14APWR` |

Enclosure PSU: Mean Well `IRM-15-12` (auto-ranging) → board `J1`.
