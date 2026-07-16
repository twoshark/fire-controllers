# mp-input enclosure

Operator box for **3 channel buttons + ALL FIRE**, hosting the v1.0.0 input PCB.

## Role

Compact “MP” station: three independent fires plus diode-ORed **ALL FIRE**. Same power, DFU, and RS-485 pattern as sign-input.

## Bill of materials (this box)

Use shared catalog in [`PARTS_BOM.md`](PARTS_BOM.md).

| Assembly | Qty |
| --- | ---: |
| Input PCB v1.0.0 | 1 |
| input-buttons daughter PCB | 1 (wire D1..D3 + ALL) |
| Mean Well `IRM-15-12` | 1 |
| Arcade momentary CH buttons (yours) | 3 |
| Arcade momentary ALL FIRE (yours) | 1 |
| Arcade / latch POWER, AC-rated (yours) | 1 |
| 16 mm IP67 RESET / BOOT | 1 / 1 |
| M12-8 panel + field cable + caps | 1 |
| IP67 USB-C bulkhead + cap | 1 |
| Bulgin IP67/68 inlet + mate plug + 1 A fuse | 1 |
| Printed shell + gasket | 1 |

## Channel assignment

| Button | Channel | `J2` pin |
| --- | --- | --- |
| MP / CH1 | CH0 | `J2a.1` |
| MP / CH2 | CH1 | `J2a.2` |
| MP / CH3 | CH2 | `J2a.3` |
| ALL FIRE | diode-OR → CH0..CH2 | — |

Leave `J2a.4` and `J2b.*` open.

## Power budget @12 V

| Load | Current |
| --- | ---: |
| Arcade button LEDs | **measure**; keep PCB+LEDs **&lt; 800 mA** |
| Input PCB | ≤120 mA |
| IRM-15-12 | 1250 mA |

## Interaction diagram

```text
User finger
  ├─ CH1..CH3 ──► GND that channel ──► input Schmitt ──► Hotline bit
  ├─ ALL FIRE ──► GND ALL_BUTTON_A ──► diodes ──► CH0..CH2 all active
  ├─ POWER    ──► latches AC to IRM
  ├─ RESET    ──► NRST
  └─ BOOT     ──► BOOT0

Same USB-C / M12 / 120VAC pattern as sign-input.
```

## CAD

Shell **200 × 140 × 90 mm** starting size — [`CAD_NOTES.md`](CAD_NOTES.md).

## Bring-up extras

Same sequence as [`SIGN_INPUT.md`](SIGN_INPUT.md), but ALL FIRE must assert only CH0..CH2.
