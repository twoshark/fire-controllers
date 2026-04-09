# Input Board Schematic Appendix (Executable Mapping)

This appendix is the pin/net/junction reference for `hardware/SCHEMATIC_GUIDE.md`.

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

Note: power LED is not MCU-driven; wire as always-on from `3V3` through resistor to LED.

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

### `J3` (2-pos, input COM/GND)

| Pin | Net |
| --- | --- |
| 1 | `INPUT_COM_GND` |
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
| D+ | `USB_DP` |
| D- | `USB_DM` |
| CC1 | `USB_CC1` (5.1k pulldown to GND) |
| CC2 | `USB_CC2` (5.1k pulldown to GND) |
| VBUS | `USB_VBUS` (debug power sense/power only) |
| GND/shell | `GND` / chassis policy per PCB appendix |

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
| `12V_MAIN` | `C17` (bulk) + `C6` (HF) | `GND` return |
| `12V_MAIN` | `U4.VIN` (`AP63203WU-7`) | buck input stage |
| `U4.SW` | `L1` | `3V3` |
| `U4.BST` | `C19` to `U4.SW` | bootstrap drive loop |
| `3V3` | `C18` + logic decoupling | `GND` return |
| `3V3` | MCU `VDD` pins + logic loads | digital domain |

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

Per channel signal chain:

```text
J2x.CHn -> IN_CHn_RAW -> 10k pull-up to 3V3, switch to GND
          -> 10k series -> RC node with 100nF to GND
          -> SN74LVC14 output -> IN_CHn_SENSE (MCU GPIO)
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

## 7) Cable crossover map (input side view)

| Input board net | Output board destination |
| --- | --- |
| `RS485_TX+` | `RS485_RX+` |
| `RS485_TX-` | `RS485_RX-` |
| `RS485_RX+` | `RS485_TX+` |
| `RS485_RX-` | `RS485_TX-` |
| `GND` | `GND` |
| `SHIELD` | `SHIELD`/chassis entry |
