# Output Board Schematic Appendix (Executable Mapping)

This appendix is the pin/net/junction reference for `hardware/SCHEMATIC_GUIDE.md`.

## 1) MCU used-pin map (output board)

| MCU GPIO/net | Direction | Function | Endpoint |
| --- | --- | --- | --- |
| `PA0` | In | `OVR_CH0_SENSE` | CH0 override Schmitt output |
| `PA1` | In | `OVR_CH1_SENSE` | CH1 override Schmitt output |
| `PA4` | In | `OVR_CH2_SENSE` | CH2 override Schmitt output |
| `PA5` | In | `OVR_CH3_SENSE` | CH3 override Schmitt output |
| `PA6` | In | `OVR_CH4_SENSE` | CH4 override Schmitt output |
| `PA7` | In | `OVR_CH5_SENSE` | CH5 override Schmitt output |
| `PB0` | In | `OVR_CH6_SENSE` | CH6 override Schmitt output |
| `PB1` | In | `OVR_CH7_SENSE` | CH7 override Schmitt output |
| `PB2` | Out | `GATE_CH0` | MOSFET CH0 gate |
| `PB10` | Out | `GATE_CH1` | MOSFET CH1 gate |
| `PB11` | Out | `GATE_CH2` | MOSFET CH2 gate |
| `PB12` | Out | `GATE_CH3` | MOSFET CH3 gate |
| `PB13` | Out | `GATE_CH4` | MOSFET CH4 gate |
| `PA8` | Out | `GATE_CH5` | MOSFET CH5 gate (USB pin conflict avoided) |
| `PA15` | Out | `GATE_CH6` | MOSFET CH6 gate |
| `PB3` | Out | `GATE_CH7` | MOSFET CH7 gate |
| `PA9` | Out | `USART1_TX` | RS-485 TX transceiver `U2B.DI` (heartbeat) |
| `PA10` | In | `USART1_RX` | RS-485 RX transceiver `U2A.RO` |
| `PA11` | BiDir | `USB_DM` | USB-C D- through 22R |
| `PA12` | BiDir | `USB_DP` | USB-C D+ through 22R |
| `PA13` | SWD | `SWDIO` | SWD header |
| `PA14-BOOT0` | SWD/In | `SWCLK` + BOOT0 | SWD header + BOOT0 button + 10k pulldown |
| `NRST` | In | Reset | NRST button + SWD header |
| `PB4` | Out | `LED_CH0_N` | CH0 output LED cathode |
| `PB5` | Out | `LED_CH1_N` | CH1 output LED cathode |
| `PB6` | Out | `LED_CH2_N` | CH2 output LED cathode |
| `PB7` | Out | `LED_CH3_N` | CH3 output LED cathode |
| `PB8` | Out | `LED_CH4_N` | CH4 output LED cathode |
| `PB9` | Out | `LED_CH5_N` | CH5 output LED cathode |
| `PC6` | Out | `LED_CH6_N` | CH6 output LED cathode |
| `PC7` | Out | `LED_CH7_N` | CH7 output LED cathode |
| `PC14` | Out | `LED_LINK_N` | Link LED cathode |

Note: power LED is not MCU-driven; wire as always-on from `3V3` through resistor to LED.

## 2) Connector pin map (output board)

### `J1` (2-pos, 5.08 mm, board 12V input)

| Pin | Net |
| --- | --- |
| 1 | `VIN_12V_IN` |
| 2 | `GND` |

### `J2` (6-pos, RS-485 terminal)

| Pin | Net |
| --- | --- |
| 1 | `RS485_TX+` |
| 2 | `RS485_TX-` |
| 3 | `RS485_RX+` |
| 4 | `RS485_RX-` |
| 5 | `GND` |
| 6 | `SHIELD` |

### `J3a` (4-pos, OVR0..OVR3)

| Pin | Net |
| --- | --- |
| 1 | `OVR_CH0_RAW` |
| 2 | `OVR_CH1_RAW` |
| 3 | `OVR_CH2_RAW` |
| 4 | `OVR_CH3_RAW` |

### `J3b` (4-pos, OVR4..OVR7)

| Pin | Net |
| --- | --- |
| 1 | `OVR_CH4_RAW` |
| 2 | `OVR_CH5_RAW` |
| 3 | `OVR_CH6_RAW` |
| 4 | `OVR_CH7_RAW` |

### `J4` (2-pos, override COM/GND)

| Pin | Net |
| --- | --- |
| 1 | `OVERRIDE_COM_GND` |
| 2 | `GND` |

### `J5a` (4-pos, OUT0..OUT3 switched outputs)

| Pin | Net |
| --- | --- |
| 1 | `OUT_CH0_SW` |
| 2 | `OUT_CH1_SW` |
| 3 | `OUT_CH2_SW` |
| 4 | `OUT_CH3_SW` |

### `J5b` (4-pos, OUT4..OUT7 switched outputs)

| Pin | Net |
| --- | --- |
| 1 | `OUT_CH4_SW` |
| 2 | `OUT_CH5_SW` |
| 3 | `OUT_CH6_SW` |
| 4 | `OUT_CH7_SW` |

### `J6` (2-pos, load supply out)

| Pin | Net |
| --- | --- |
| 1 | `LOAD_12V` |
| 2 | `LOAD_GND_RTN` |

### `J7` (USB-C receptacle)

| Connector function | Net |
| --- | --- |
| D+ | `USB_DP` |
| D- | `USB_DM` |
| CC1 | `USB_CC1` (5.1k pulldown to GND) |
| CC2 | `USB_CC2` (5.1k pulldown to GND) |
| VBUS | `USB_VBUS` |
| GND/shell | `GND` / chassis policy per PCB appendix |

### `J8` (2x5 SWD, 1.27 mm ARM convention)

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

## 3) Power/junction endpoint map (output board)

| Source | Through | Destination |
| --- | --- | --- |
| `VIN_12V_IN` (`J1.1`) | board input path | `12V_MAIN` |
| `12V_MAIN` | `C17` + `C18` bulk | `GND` return |
| `12V_MAIN` | `U4.VIN` (`AP63203WU-7`) | buck input stage |
| `U4.SW` | `L1` | `3V3` |
| `U4.BST` | `C20` to `U4.SW` | bootstrap drive loop |
| `3V3` | `C19` + decoupling network | digital logic domain |
| `12V_MAIN` | per-channel `F1..F8` | `LOAD_12V` distributed to each channel |

## 4) RS-485 transceiver endpoint map (output board)

### `U2A` (RX path, always receive)

| Transceiver pin role | Net |
| --- | --- |
| `RO` | `USART1_RX` (`PA10`) |
| `DE` | `GND` |
| `/RE` | `GND` |
| `A/B` | `RS485_RX+` / `RS485_RX-` via `J2.3/J2.4` |
| Termination | `R52 = 120R` across `RS485_RX+/-` |

### `U2B` (TX path, always transmit)

| Transceiver pin role | Net |
| --- | --- |
| `DI` | `USART1_TX` (`PA9`) |
| `DE` | `3V3` |
| `/RE` | `3V3` |
| `A/B` | `RS485_TX+` / `RS485_TX-` via `J2.1/J2.2` |

Protection devices:

- `SM712` on TX pair and one `SM712` on RX pair.

## 5) Override CH0..CH7 junction map

Per channel override chain:

```text
J3x.OVR_CHn -> OVR_CHn_RAW -> 10k pull-up to 3V3, switch to GND
              -> 10k series -> RC node with 100nF to GND
              -> SN74LVC14 output -> OVR_CHn_SENSE (MCU GPIO)
```

| Channel | Override connector pin | MCU GPIO |
| --- | --- | --- |
| CH0 | `J3a.1` | `PA0` |
| CH1 | `J3a.2` | `PA1` |
| CH2 | `J3a.3` | `PA4` |
| CH3 | `J3a.4` | `PA5` |
| CH4 | `J3b.1` | `PA6` |
| CH5 | `J3b.2` | `PA7` |
| CH6 | `J3b.3` | `PB0` |
| CH7 | `J3b.4` | `PB1` |

## 6) Output-channel endpoint map (serial/override to load terminal)

| CH | Gate GPIO | MOSFET | Output terminal | LED GPIO |
| --- | --- | --- | --- | --- |
| CH0 | `PB2` | `Q1` | `J5a.1` | `PB4` |
| CH1 | `PB10` | `Q2` | `J5a.2` | `PB5` |
| CH2 | `PB11` | `Q3` | `J5a.3` | `PB6` |
| CH3 | `PB12` | `Q4` | `J5a.4` | `PB7` |
| CH4 | `PB13` | `Q5` | `J5b.1` | `PB8` |
| CH5 | `PA8` | `Q6` | `J5b.2` | `PB9` |
| CH6 | `PA15` | `Q7` | `J5b.3` | `PC6` |
| CH7 | `PB3` | `Q8` | `J5b.4` | `PC7` |

Per-channel power side:

`12V_MAIN` -> `F(n)` -> load + side (via `J6.1` distribution) -> load -> `OUT_CHn_SW` -> MOSFET low-side -> `LOAD_GND_RTN` (`J6.2`).

## 7) Cable crossover map (output side view)

| Output board net | Input board destination |
| --- | --- |
| `RS485_RX+` | `RS485_TX+` |
| `RS485_RX-` | `RS485_TX-` |
| `RS485_TX+` | `RS485_RX+` |
| `RS485_TX-` | `RS485_RX-` |
| `GND` | `GND` |
| `SHIELD` | `SHIELD`/chassis entry |
