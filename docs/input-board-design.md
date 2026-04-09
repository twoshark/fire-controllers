# Input Board Design

## Overview

The input board reads 8 switch inputs and transmits a channel bitfield to the output board at 1kHz over full-duplex RS-485.

| Item | Value |
| --- | --- |
| MCU | STM32G0B1CBT6 |
| Inputs | 8x switch-to-GND channels |
| Serial | RS-485 full duplex, 115200 |
| Debug/programming | USB DFU + SWD |

## Input conditioning

Each channel uses RC conditioning and Schmitt cleanup before GPIO read:

```text
3V3 -> 10k pull-up -> switch_node -> 10k series -> rc_node -> SN74LVC14A input
                             |                             |
                           switch                        100nF
                             |                             |
                            GND                           GND
```

- `tau = 10k * 100nF = 1ms`
- `SN74LVC14A` provides Schmitt-trigger input thresholding
- Output is inverted; firmware interprets buffered HIGH as active/pressed

## RS-485 architecture

Two SP3485EN devices:

- TX path: `PA9 (USART1_TX)` -> transceiver -> cable TX pair
- RX path: cable RX pair -> transceiver -> `PA10 (USART1_RX)`
- Receiver pair has populated 120R termination
- One SM712 TVS per differential pair

Point-to-point cable crossover at installation (required for full duplex):

- Input `TX+` -> Output `RX+`
- Input `TX-` -> Output `RX-`
- Input `RX+` <- Output `TX+`
- Input `RX-` <- Output `TX-`

## USB DFU and SWD

- Native USB data:
  - `PA11 = USB_DM`
  - `PA12 = USB_DP`
- 22R series resistors on D+ and D-
- 5.1k pull-downs on CC1/CC2
- SWD 10-pin header retained for bring-up/recovery

BOOT0:

- Net naming follows `PA14-BOOT0` (shared with SWCLK during SWD attach)
- 10k pulldown keeps normal boot default
- `SW2` pushes BOOT0 high
- `SW1` is NRST
- DFU entry: hold SW2, reset via SW1

## LEDs and UI

- 8 channel LEDs mirror transmitted channel state
- 1 power LED
- 1 link LED indicates heartbeat health

## Firmware behavior summary

- Poll digital inputs every 1ms
- Pack into 8-bit state payload
- Payload is current-state only (no edge/event counters)
- Transmit `[0xAA][state][CRC8]`
- Continuously receive heartbeat frames in parallel

## Connector strategy

- PCB uses screw terminals
- Enclosure uses panel-mounted waterproof connectors wired back to PCB terminals
- RS-485 terminal exposes TX pair, RX pair, GND, shield

## Layout and implementation notes

- Keep the input RC cells physically uniform across all 8 channels.
- Place Schmitt ICs between input-channel RC blocks and MCU GPIO fan-in.
- Keep USB D+/D- short and routed together; place 22R resistors close to MCU.
- Place RS-485 TVS close to connector entry and 120R close to RX transceiver pins.

See also:

- `hardware/SCHEMATIC_GUIDE.md`
- `hardware/PCB_LAYOUT_GUIDE.md`
