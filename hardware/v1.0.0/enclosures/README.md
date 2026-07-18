# Enclosures (v1.0.0)

3D-printed gasketed boxes for the four Hotline controllers.

| Enclosure | Hosts | Loads | Doc |
| --- | --- | --- | --- |
| **sign-input** | Input PCB + IRM-15-12 | Buttons CH0..CH4 + ALL | [`SIGN_INPUT.md`](SIGN_INPUT.md) |
| **mp-input** | Input PCB + IRM-15-12 | Buttons CH0..CH2 + ALL | [`MP_INPUT.md`](MP_INPUT.md) |
| **sign-output** | Output PCB + LRS-200-12 | Solenoids CH0..CH4 | [`SIGN_OUTPUT.md`](SIGN_OUTPUT.md) |
| **mp-output** | Output PCB + LRS-150-12 | Solenoids CH0..CH2 | [`MP_OUTPUT.md`](MP_OUTPUT.md) |

| Doc | Contents |
| --- | --- |
| [`SHOPPING_LIST.md`](SHOPPING_LIST.md) | Order carts |
| [`PARTS_BOM.md`](PARTS_BOM.md) | Parts catalog |
| [`WIRING.md`](WIRING.md) | Interconnect and pin maps |
| [`CAD_NOTES.md`](CAD_NOTES.md) | Shell sizes and cutouts |

## System

```text
sign-input ──RS-485──► sign-output (5 solenoids)
mp-input   ──RS-485──► mp-output   (3 solenoids)
```

## Power

| Box | PSU | 12 V | AC |
| --- | --- | --- | --- |
| Inputs | IRM-15-12 | &lt;800 mA | C14 |
| sign-output | LRS-200-12 | ≤10.3 A | C14 |
| mp-output | LRS-150-12 | ≤6.2 A | C14 |

Panel: C14 AC in, M12 screw-terminal field I/O (RS-485 5-pin; SOL 8-pin sign / 5-pin mp), HangTon USB-C. POWER switches AC into the PSU.
