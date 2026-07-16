# mp-input enclosure

Operator box for **3 channel buttons + ALL FIRE**, hosting the v1.0.0 input PCB.

## Role

Compact “MP” station: three independent fires plus diode-ORed **ALL FIRE**. Same power, HangTon USB DFU, and RS-485 pattern as sign-input.

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
| Adafruit **559** RESET + **481** BOOT | 1 / 1 |
| GX16-6 RS-485 panel + mate | 1 |
| HangTon USB-C bulkhead + M–M jumper | 1 |
| GX16-3 AC in + 1 A fuse | 1 |
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
mp-input channel set = {CH0,CH1,CH2}   (front labels CH1..CH3)

User
  ├─ front CH1 ──► GND net CH0 ──► Schmitt ──► Hotline bit0 only
  ├─ front CH2 ──► GND net CH1 ──► …          ──► bit1 only
  ├─ front CH3 ──► GND net CH2 ──► …          ──► bit2 only
  ├─ ALL FIRE  ──► GND ALL_BUTTON_A ──► D1..D3 ──► bits CH0..CH2 together
  │                 (same set as CH1..CH3 — not CH3..CH7)
  ├─ POWER / RESET / BOOT — same as sign-input

HangTon USB / GX16 RS-485 / GX16 AC — same budget platform as sign-input.

Priced budget BOM: [`PARTS_BOM.md`](PARTS_BOM.md) (~**$45**/box). Load box: [`MP_OUTPUT.md`](MP_OUTPUT.md).

## CAD

Shell **200 × 140 × 90 mm** starting size — [`CAD_NOTES.md`](CAD_NOTES.md).

## Bring-up extras

Same as [`SIGN_INPUT.md`](SIGN_INPUT.md), but:

1. Each CH1..CH3 → **only** that bit.
2. ALL FIRE → **CH0..CH2 together**, and **not** CH3..CH7.
