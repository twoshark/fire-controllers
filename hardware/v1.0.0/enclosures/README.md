# Enclosures (v1.0.0)

3D-printed gasketed boxes. **Budget target: ≤ $50 / box** excluding filament, arcade buttons, and PCBs — inputs ~$45; outputs exceed because of LRS PSU + HangTon USB ([`PARTS_BOM.md`](PARTS_BOM.md)).

| Enclosure | Hosts | Loads | Doc |
| --- | --- | --- | --- |
| **sign-input** | Input PCB + IRM-15-12 | Buttons CH0..CH4 + ALL | [`SIGN_INPUT.md`](SIGN_INPUT.md) |
| **mp-input** | Input PCB + IRM-15-12 | Buttons CH0..CH2 + ALL | [`MP_INPUT.md`](MP_INPUT.md) |
| **sign-output** | Output PCB + **LRS-200-12** (17 A) | Solenoids CH0..CH4 + glow CH7 | [`SIGN_OUTPUT.md`](SIGN_OUTPUT.md) |
| **mp-output** | Output PCB + **LRS-150-12** (12.5 A) | Solenoids CH0..CH2 + glow CH7 | [`MP_OUTPUT.md`](MP_OUTPUT.md) |

| Doc | Contents |
| --- | --- |
| [`SHOPPING_LIST.md`](SHOPPING_LIST.md) | **Order carts:** Bravo / DigiKey / Adafruit / LCSC |
| [`PARTS_BOM.md`](PARTS_BOM.md) | Budget buy list, cost autopsy |
| [`WIRING.md`](WIRING.md) | Interconnect, GX16 pin maps, gauges |
| [`CAD_NOTES.md`](CAD_NOTES.md) | Shell sizes / cutouts |

## Design goals

- ≤ **$50** hardware per box where possible (ex filament / arcade / PCB); HangTon USB + LRS push outputs above
- **IP65–IP67** mated (GX16 + gasket + caps); hose-test
- POWER breaks **AC hot** after fuse
- Glowflies: CH7 + OVR→GND always on
- Dropped DigiKey-premium Buccaneer / Phoenix M12 / AT PanelMate / Bulgin POWER (too expensive)

## System map

```text
sign-input ──RS-485──► sign-output (5 solenoids + glow)
mp-input   ──RS-485──► mp-output   (3 solenoids + glow)
```

Cross-pairing is fine electrically (unused bits stay off).

## Power

| Box | PSU | 12 V budget |
| --- | --- | --- |
| Inputs | IRM-15-12 (1.25 A) | PCB + arcade LEDs &lt; 800 mA |
| sign-output | **LRS-200-12 (17 A)** | ≤10.3 A continuous + derating margin |
| mp-output | **LRS-150-12 (12.5 A)** | ≤6.3 A continuous + derating margin |
