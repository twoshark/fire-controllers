# sign-input enclosure

Operator box for **5 channel buttons + ALL FIRE**, hosting the v1.0.0 input PCB.

## Role

Field “sign” station: five independent fires plus a diode-ORed **ALL FIRE**. Full system POWER, HangTon USB-C DFU (lid closed), GX16 RS-485 uplink, 120 VAC in.

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
| Adafruit **559** RESET + **481** BOOT | 1 / 1 |
| GX16-6 RS-485 panel + mate | 1 |
| HangTon USB-C bulkhead + M–M jumper | 1 |
| GX16-3 AC in + 1 A fuse | 1 |
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
sign-input channel set = {CH0,CH1,CH2,CH3,CH4}   (front labels CH1..CH5)

User
  ├─ front CH1 ──► GND net CH0 ──► Schmitt ──► Hotline bit0 only
  ├─ front CH2 ──► GND net CH1 ──► …          ──► bit1 only
  ├─ front CH3 ──► GND net CH2 ──► …          ──► bit2 only
  ├─ front CH4 ──► GND net CH3 ──► …          ──► bit3 only
  ├─ front CH5 ──► GND net CH4 ──► …          ──► bit4 only
  ├─ ALL FIRE  ──► GND ALL_BUTTON_A ──► D1..D5 ──► bits CH0..CH4 together
  │                 (same set as CH1..CH5 — not CH5..CH7)
  ├─ POWER     ──► latches AC to IRM
  ├─ RESET     ──► NRST
  └─ BOOT      ──► BOOT0

Host USB-C ──► HangTon panel ──► jumper ──► J5 DFU
sign-output ──► GX16-6 RS-485 ──► CN2
Wall 120VAC ──► GX16-3 ──► fuse ──► POWER ──► IRM
```

Priced budget BOM: [`PARTS_BOM.md`](PARTS_BOM.md) (~**$45**/box). Load box: [`SIGN_OUTPUT.md`](SIGN_OUTPUT.md).

## CAD

Shell **240 × 160 × 90 mm** starting size — [`CAD_NOTES.md`](CAD_NOTES.md).

## Bring-up extras

After [`../BRINGUP.md`](../BRINGUP.md) rail checks:

1. POWER ON → 12 V present; arcade LEDs as wired.
2. Each CH1..CH5 → **only** that channel’s input LED / Hotline bit.
3. ALL FIRE → **CH0..CH4 together**, and **not** CH5..CH7.
4. Unplug M12 → input link LED blinks (no heartbeat).
5. Hose-test with caps fitted.
