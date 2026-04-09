# Input Board Design

## Overview

The input board reads 8 switch inputs and transmits a channel bitfield to the output board at 1kHz over full-duplex RS-485.

| Item | Value |
| --- | --- |
| MCU | STM32G0B1CBT6 |
| Inputs | 8x switch-to-GND channels |
| Serial | RS-485 full duplex, 115200 |
| Debug/programming | USB DFU + SWD |

## 3.3V current estimate and margin target

Estimated `3V3` draw (input board):

- MCU (`STM32G0B1`) active + USB-capable clocking: ~20mA typical, ~35mA worst-case budget
- RS-485 (`2x SP3485EN`): ~10mA typical combined, ~20mA worst-case budget
- LEDs (power + link + up to 8 channel LEDs): ~20mA typical, ~38mA peak
- Input pull-up network and logic overhead: ~5mA peak budget
- Miscellaneous digital overhead: ~2mA peak budget

Resulting rail budget:

- Typical: ~55mA
- Peak: ~120mA
- Regulator requirement: at least 300mA continuous on `3V3` to maintain >=2.5x peak margin and low thermal stress from `12V_MAIN`.

Regulator decision for this board:

- `LMZM23601V33` was the preferred option in plan, but assembly availability is currently not production-friendly.
- Selected fallback: `AP63203WU-7` (`C780769`), fixed 3.3V, synchronous buck, 2A rating.
- This keeps the rail well inside safe operating area and avoids the linear-regulator heat path.

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
- Full connector pin map is defined in `hardware/SCHEMATIC_APPENDIX_INPUT.md`

## Layout and implementation notes

- Keep the input RC cells physically uniform across all 8 channels.
- Place Schmitt ICs between input-channel RC blocks and MCU GPIO fan-in.
- Keep USB D+/D- short and routed together; place 22R resistors close to MCU.
- Place RS-485 TVS close to connector entry and 120R close to RX transceiver pins.

See also:

- `hardware/SCHEMATIC_GUIDE.md`
- `hardware/SCHEMATIC_APPENDIX_INPUT.md`
- `hardware/PCB_LAYOUT_GUIDE.md`
- `hardware/PCB_APPENDIX_INPUT.md`
