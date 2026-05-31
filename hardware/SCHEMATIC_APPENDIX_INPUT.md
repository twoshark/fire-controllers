# Input Board Schematic Appendix (Executable Mapping)

This appendix is the pin/net/junction reference for `hardware/SCHEMATIC_GUIDE.md`.

Review-related additions (per `hardware/REVIEW_RESOLUTION.md`):

- New designators: `C28` (2nd 22uF on `3V3`), `C29` (`U2A.VCC`), `C30` (`U2B.VCC`), `C31` (`U5.VCC`), `C32` (`U6.VCC`), `D12` (USB ESD).
- Net naming: `3V3` only - never `3.3V`, `+3.3V`, etc.
- All `GND`/`3V3`/`12V_MAIN`/`VIN_12V_IN` use Power Symbol Net Flags, not Net Ports.
- Polarized cap polarity is locked: see Section 8 below.

## 1) MCU used-pin map (input board)

| MCU GPIO/net | Direction | Function | Endpoint |
| --- | --- | --- | --- |
| `PA0` | In | `IN_CH0_SENSE` | CH0 Schmitt output |
| `PA1` | In | `IN_CH1_SENSE` | CH1 Schmitt output |
| `PA4` | In | `IN_CH2_SENSE` | CH2 Schmitt output |
| `PA5` | In | `IN_CH3_SENSE` | CH3 Schmitt output |
| `PA6` | In | `IN_CH4_SENSE` | CH4 Schmitt output |
| `PA7` | In | `IN_CH5_SENSE` | CH5 Schmitt output |
| `PB0` | In | `IN_CH6_SENSE` | CH6 Schmitt output |
| `PB1` | In | `IN_CH7_SENSE` | CH7 Schmitt output |
| `PA9` | Out | `USART1_TX` | RS-485 TX transceiver `U2A.DI` |
| `PA10` | In | `USART1_RX` | RS-485 RX transceiver `U2B.RO` |
| `PA11` | BiDir | `USB_DM` | USB-C D- through 22R |
| `PA12` | BiDir | `USB_DP` | USB-C D+ through 22R |
| `PA13` | SWD | `SWDIO` | SWD header |
| `PA14-BOOT0` | SWD/In | `SWCLK` + BOOT0 | SWD header + BOOT0 button + 10k pulldown |
| `NRST` | In | Reset | NRST button + SWD header |
| `PB2` | Out | `LED_CH0_N` | CH0 LED cathode (active low) |
| `PA15` | Out | `LED_CH1_N` | CH1 LED cathode (active low) |
| `PB3` | Out | `LED_CH2_N` | CH2 LED cathode (active low) |
| `PB4` | Out | `LED_CH3_N` | CH3 LED cathode (active low) |
| `PB5` | Out | `LED_CH4_N` | CH4 LED cathode (active low) |
| `PB6` | Out | `LED_CH5_N` | CH5 LED cathode (active low) |
| `PB7` | Out | `LED_CH6_N` | CH6 LED cathode (active low) |
| `PB8` | Out | `LED_CH7_N` | CH7 LED cathode (active low) |
| `PB9` | Out | `LED_LINK_N` | Link LED cathode (active low) |

Note: power LED is not MCU-driven; wire as always-on with `3V3 -> R25 -> LED1 -> GND`.
`R25` uses the 330R BOM line (`C23138`, `0603WAF3300T5E`).

## 2) Connector pin map (input board)

### `J1` (2-pos, 5.08 mm, 12V input)

| Pin | Net |
| --- | --- |
| 1 | `VIN_12V_IN` |
| 2 | `GND` |

### `J2a` (4-pos, CH0..CH3 inputs)

| Pin | Net |
| --- | --- |
| 1 | `IN_CH0_RAW` |
| 2 | `IN_CH1_RAW` |
| 3 | `IN_CH2_RAW` |
| 4 | `IN_CH3_RAW` |

### `J2b` (4-pos, CH4..CH7 inputs)

| Pin | Net |
| --- | --- |
| 1 | `IN_CH4_RAW` |
| 2 | `IN_CH5_RAW` |
| 3 | `IN_CH6_RAW` |
| 4 | `IN_CH7_RAW` |

### `J3` (2-pos, input GND/GND)

| Pin | Net |
| --- | --- |
| 1 | `GND` |
| 2 | `GND` |

### `J4` (6-pos, RS-485 terminal)

| Pin | Net |
| --- | --- |
| 1 | `RS485_TX+` |
| 2 | `RS485_TX-` |
| 3 | `RS485_RX+` |
| 4 | `RS485_RX-` |
| 5 | `GND` |
| 6 | `SHIELD` |

### `J5` (USB-C receptacle)

| Connector function | Net |
| --- | --- |
| D+ | `USB_DP` (clamped by `D12.IO1` before MCU) |
| D- | `USB_DM` (clamped by `D12.IO2` before MCU) |
| CC1 | `USB_CC1` (5.1k pulldown to GND via `R29`) |
| CC2 | `USB_CC2` (5.1k pulldown to GND via `R30`) |
| VBUS | `USB_VBUS` (also clamped by `D12.VBUS`) |
| GND/shell | `GND` / chassis policy per PCB appendix |

`D12` (`USBLC6-2SC6`, LCSC `C7519`, SOT-23-6) pinout (per ST datasheet, top view: `1=I/O1, 2=GND, 3=I/O2, 4=I/O2, 5=VBUS, 6=I/O1`; pins 1&6 are internally common I/O1, pins 3&4 are internally common I/O2):

| Pin | Signal | Net |
| --- | --- | --- |
| 1 | `I/O1` | `USB_DP` (on the `J5.D+` side of `R31`) |
| 2 | `GND` | `GND` |
| 3 | `I/O2` | `USB_DM` (on the `J5.D-` side of `R32`) |
| 4 | `I/O2` | `USB_DM` (internally common with pin 3; route to the same net or leave open) |
| 5 | `VBUS` | `USB_VBUS` |
| 6 | `I/O1` | `USB_DP` (internally common with pin 1; route to the same net or leave open) |

### `J6` (2x5 SWD, 1.27 mm ARM convention)

| Pin | Net |
| --- | --- |
| 1 | `VTREF_3V3` |
| 2 | `SWDIO` |
| 3 | `GND` |
| 4 | `PA14-BOOT0` (`SWCLK`) |
| 5 | `GND` |
| 6 | `SWO_NC` |
| 7 | `KEY_NC` |
| 8 | `NC` |
| 9 | `GND` |
| 10 | `NRST` |

## 3) Power/junction endpoint map (input board)

| Source | Through | Destination |
| --- | --- | --- |
| `VIN_12V_IN` (`J1.1`) | `D1` then `F1` | `12V_MAIN` |
| `12V_MAIN` | `C17` (bulk, polarized `+` to `12V_MAIN`) + `C6` (HF) | `GND` return |
| `12V_MAIN` | `U4.VIN` (`AP63203WU-7`) | buck input stage |
| `U4.SW` | `L1` | `3V3` |
| `U4.BST` | `C19` to `U4.SW` | bootstrap drive loop |
| `3V3` | `C18` + `C28` (both 22uF, polarized `+` to `3V3`) + logic decoupling | `GND` return |
| `3V3` | MCU `VDD` pins + logic loads | digital domain |
| `3V3` | `C29` (`U2A.VCC`), `C30` (`U2B.VCC`), `C31` (`U5.VCC`), `C32` (`U6.VCC`) - 1uF each | `GND` return at each IC |

## 4) RS-485 transceiver endpoint map (input board)

### `U2A` (TX path, always transmit)

| Transceiver pin role | Net |
| --- | --- |
| `DI` | `USART1_TX` (`PA9`) |
| `DE` | `3V3` |
| `/RE` | `3V3` |
| `A/B` | `RS485_TX+` / `RS485_TX-` via `J4.1/J4.2` |

### `U2B` (RX path, always receive)

| Transceiver pin role | Net |
| --- | --- |
| `RO` | `USART1_RX` (`PA10`) |
| `DE` | `GND` |
| `/RE` | `GND` |
| `A/B` | `RS485_RX+` / `RS485_RX-` via `J4.3/J4.4` |
| Termination | `R27 = 120R` across `RS485_RX+/-` |

Protection devices:

- `SM712` on TX pair: clamp between `RS485_TX+/-` and local return.
- `SM712` on RX pair: clamp between `RS485_RX+/-` and local return.

## 5) CH0..CH7 net/junction map (input to protocol)

Schmitt stage pin convention for `U5/U6` (`SN74LV14APWR`):

- `A` pins are inputs (from RC node), `Y` pins are outputs (to `_SENSE` net).
- Pin map: `1A/1Y=1/2`, `2A/2Y=3/4`, `3A/3Y=5/6`, `4A/4Y=9/8`, `5A/5Y=11/10`, `6A/6Y=13/12`.

Per channel signal chain:

```text
J2x.CHn -> IN_CHn_RAW -> 10k pull-up to 3V3, switch to GND
          -> 10k series -> RC node with channel capacitor C20..C27 to GND
          -> SN74LV14 A-input -> SN74LV14 Y-output -> IN_CHn_SENSE (MCU GPIO)
          -> protocol bit n in [0xAA][channels][CRC8]
```

| Channel | Connector pin | MCU sense net | Protocol bit |
| --- | --- | --- | --- |
| CH0 | `J2a.1` | `PA0` | bit0 |
| CH1 | `J2a.2` | `PA1` | bit1 |
| CH2 | `J2a.3` | `PA4` | bit2 |
| CH3 | `J2a.4` | `PA5` | bit3 |
| CH4 | `J2b.1` | `PA6` | bit4 |
| CH5 | `J2b.2` | `PA7` | bit5 |
| CH6 | `J2b.3` | `PB0` | bit6 |
| CH7 | `J2b.4` | `PB1` | bit7 |

Input RC capacitor assignment (input board):

| Channel | RC capacitor |
| --- | --- |
| CH0 | `C20` |
| CH1 | `C21` |
| CH2 | `C22` |
| CH3 | `C23` |
| CH4 | `C24` |
| CH5 | `C25` |
| CH6 | `C26` |
| CH7 | `C27` |

## 6) LED endpoint map (input board)

All status/channel LEDs are active-low outputs from MCU.

| LED function | MCU pin/net |
| --- | --- |
| CH0 | `PB2` |
| CH1 | `PA15` |
| CH2 | `PB3` |
| CH3 | `PB4` |
| CH4 | `PB5` |
| CH5 | `PB6` |
| CH6 | `PB7` |
| CH7 | `PB8` |
| LINK | `PB9` |
| POWER | Always-on from `3V3` |

LED resistor references (input board):

- `R25`: POWER LED series resistor (330R, LCSC `C23138`)
- `R28`: LINK LED series resistor (150R, LCSC `C22808`)
- Remaining channel LED series resistors come from `R17-R26` 330R bank.

## 7) Cable crossover map (input side view)

| Input board net | Output board destination |
| --- | --- |
| `RS485_TX+` | `RS485_RX+` |
| `RS485_TX-` | `RS485_RX-` |
| `RS485_RX+` | `RS485_TX+` |
| `RS485_RX-` | `RS485_TX-` |
| `GND` | `GND` |
| `SHIELD` | `SHIELD`/chassis entry |

## 8) Polarized capacitor polarity reference (review fix P1)

Reverse polarity will cause venting/rupture at first power-on. Confirmed orientations on the input board:

| Designator | Value | `+` net | `-` net |
| --- | --- | --- | --- |
| `C17` | bulk on `12V_MAIN` | `12V_MAIN` | `GND` |
| `C18` | 22uF on `3V3` | `3V3` | `GND` |
| `C28` | 22uF on `3V3` (added) | `3V3` | `GND` |

PCB silkscreen `+` mark must agree with schematic `+` pin before fab release.

## 9) Per-IC `1uF` VCC bypass map (review fix I1/R1)

Each cap goes from the IC VCC pin DIRECTLY to local GND, supplementing the existing 100nF HF caps (do not replace, place in parallel).

| Designator | IC | Function |
| --- | --- | --- |
| `C29` | `U2A` | RS-485 TX transceiver bypass |
| `C30` | `U2B` | RS-485 RX transceiver bypass |
| `C31` | `U5` | Schmitt CH0..CH5 bypass |
| `C32` | `U6` | Schmitt CH6..CH7 bypass |

## 10) USB ESD protection map (review fix U1)

| Designator | Part | Pin map |
| --- | --- | --- |
| `D12` | `USBLC6-2SC6` (LCSC `C7519`, SOT-23-6) | IO1->`USB_DP`, IO2->`USB_DM`, VBUS->`USB_VBUS`, GND->`GND` |

Insertion topology (clamp on the connector side of the series resistor): `J5.D+` -> `D12.IO1` (clamp) -> `R31` (22R) -> `U1.PA12` (and same for the D-/IO2 path through `R32`). The ESD diode must be upstream of the series resistor so the strike is shunted before reaching `R31`/the MCU. Place within 5 mm of `J5` per `PCB_APPENDIX_INPUT.md`.

## 11) MCU GPIO pin sink-current verification (review fix M1)

Per-channel LED current calculation:

- `I_pin = (3.3V - V_F_LED) / R_series`
- For 330R + V_F = 2.0V: `I_pin = 1.3 / 330 ~= 3.94mA` per CH LED
- For 150R + V_F = 2.0V: `I_pin = 1.3 / 150 ~= 8.67mA` for LINK LED (within 8mA recommended; consider 220R if margin desired)
- STM32G0B1 datasheet limits: `Iol_max = 20mA` per pin; recommended <=8mA for digital integrity
- Sustained worst-case: 8 channel LEDs + 1 LINK LED = `8 * 3.94 + 8.67 ~= 40.2mA` group total (within typ. 80mA per port group)

If the LINK LED resistor `R28` is changed away from 150R, update this table and re-verify per-pin and per-port-group margin.

## 12) BOM additions summary (review fixes)

| Designator | Value | LCSC / MPN | BOM line |
| --- | --- | --- | --- |
| `C28` | 22uF 10V tantalum | `C11366` / TAJA226K010RNJ | merged onto the `C18` line (qty 2) |
| `C29` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `C30` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `C31` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `C32` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `D12` | `USBLC6-2SC6` SOT-23-6 | `C7519` / USBLC6-2SC6 | own line (qty 1) |

All of the above reuse part numbers already on the BOM except `D12`. `C7519` (USBLC6-2SC6, ST) confirmed in LCSC/JLCPCB stock. The 22uF and 1uF additions reuse the existing tantalum/ceramic lines, so they carry the same stock confidence as the parts they parallel.
