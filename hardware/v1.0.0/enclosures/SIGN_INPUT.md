# sign-input enclosure

Operator box for **5 channel buttons + ALL FIRE**, hosting the v1.0.0 input PCB.

## Role

Field “sign” station: five independent fires plus a diode-ORed **ALL FIRE**. Full system POWER, sealed USB-C DFU, M12 RS-485 uplink, 120 VAC in.

## Bill of materials (this box)

Use shared catalog in [`PARTS_BOM.md`](PARTS_BOM.md).

| Assembly | Qty |
| --- | ---: |
| Input PCB v1.0.0 | 1 |
| input-buttons daughter PCB | 1 (wire D1..D5 + ALL) |
| Mean Well `IRM-15-12` | 1 |
| Arcade momentary CH buttons (yours) | 5 |
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
| Sign / CH1 | CH0 | `J2a.1` |
| Sign / CH2 | CH1 | `J2a.2` |
| Sign / CH3 | CH2 | `J2a.3` |
| Sign / CH4 | CH3 | `J2a.4` |
| Sign / CH5 | CH4 | `J2b.1` |
| ALL FIRE | diode-OR → CH0..CH4 | — |

Leave `J2b.2..4` (CH5..CH7) open.

## Power budget @12 V

| Load | Current |
| --- | ---: |
| Arcade button LEDs | **measure**; keep PCB+LEDs **&lt; 800 mA** |
| Input PCB | ≤120 mA |
| IRM-15-12 | 1250 mA |

## Interaction diagram

```text
User finger
  ├─ CH1..CH5 ──► GND that channel ──► input Schmitt ──► Hotline bit
  ├─ ALL FIRE ──► GND ALL_BUTTON_A ──► diodes ──► CH0..CH4 all active
  ├─ POWER    ──► latches AC to IRM ──► whole box 12V on/off
  ├─ RESET    ──► NRST pulse
  └─ BOOT     ──► BOOT0 high for DFU with RESET

Host PC USB-C ──► panel ──► J5 DFU
Output box     ──► M12 ──► CN2 Hotline
Wall 120VAC    ──► inlet ──► fuse ──► POWER ──► IRM
```

## CAD

Shell **240 × 160 × 90 mm** starting size — [`CAD_NOTES.md`](CAD_NOTES.md).

## Bring-up extras

After [`../BRINGUP.md`](../BRINGUP.md) rail checks:

1. POWER ON → all LEDs lit (if wired always-on) or only when intended.
2. Each CH button → matching input PCB LED + RS-485 bit.
3. ALL FIRE → CH0..CH4 LEDs together.
4. Unplug M12 → input link LED blinks (no heartbeat).
5. Hose-test panel seals with POWER off and USB/M12 capped.
