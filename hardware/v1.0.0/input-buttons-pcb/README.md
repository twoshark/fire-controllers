# Input Buttons Daughter PCB

EasyEDA export 2026-07-15. Diode-OR board for the enclosure **ALL FIRE** button.

## What it does

`1N4148W` diodes form a wired-OR so **ALL FIRE** pulls the **same channel nets as that enclosure’s individual buttons** to GND (same polarity as per-channel NO switches).

| Net | Role |
| --- | --- |
| `IN_CHx_BUTTON_A` | Tied to input-board channel terminal **and** that channel’s arcade NO |
| `ALL_BUTTON_A` | ALL FIRE NO → this net → GND |

Diode orientation (netlist): `D*.1` on `ALL_BUTTON_A`, `D*.2` on `IN_CHx_BUTTON_A`. Cathode on pin 1 → grounding `ALL_BUTTON_A` pulls wired channels **low**.

## Scope per enclosure (critical)

| Enclosure | Individual buttons | ALL FIRE must assert | Wire diodes | Leave open |
| --- | --- | --- | --- | --- |
| **sign-input** | front CH1..CH5 → MCU CH0..CH4 | **exactly CH0..CH4** | D1..D5 | D6..D8 pads; `J2b.2..4` |
| **mp-input** | front CH1..CH3 → MCU CH0..CH2 | **exactly CH0..CH2** | D1..D3 | D4..D8 pads; unused `J2` |

Do **not** strap unused diode cathodes onto the input PCB or ALL FIRE will light unused Hotline bits.

## Exports

| File | Contents |
| --- | --- |
| [`exports/BOM.csv`](exports/BOM.csv) | 8× `1N4148W` (LCSC `C22374707`) |
| [`exports/PnP.csv`](exports/PnP.csv) | Pick-and-place |
| [`exports/Netlist.asc`](exports/Netlist.asc) | PADS netlist |

## Gaps / wiring note

Export is **diodes only** (no connectors). For builds: solder flying leads to used pads, or rev the PCB with screw terminals (`ALL`, used `CHx`, `GND`).

See [`../enclosures/sign-input/WIRING.md`](../enclosures/sign-input/WIRING.md) or [`../enclosures/mp-input/WIRING.md`](../enclosures/mp-input/WIRING.md).
