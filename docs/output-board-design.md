# Output Board Design

## Overview

The output board receives channel states over full-duplex RS-485 and controls 8x 12V MOSFET outputs (up to 2A/channel). Local override switches can force a channel ON.

| Item | Value |
| --- | --- |
| MCU | STM32G0B1CBT6 |
| Outputs | 8x low-side MOSFET channels |
| Overrides | 8x switch-to-GND inputs |
| Serial | RS-485 full duplex, 115200 |

## 3.3V current estimate and margin target

Estimated `3V3` draw (output board):

- MCU (`STM32G0B1`) active while receiving frames and driving outputs: ~22mA typical, ~35mA worst-case budget
- RS-485 (`2x SP3485EN`): ~10mA typical combined, ~20mA worst-case budget
- LEDs (power + link + up to 8 output LEDs): ~22mA typical, ~38mA peak
- Override pull-up network and gate-drive static components: ~7mA peak budget
- Miscellaneous digital overhead: ~3mA peak budget

Resulting rail budget:

- Typical: ~65mA
- Peak: ~140mA
- Regulator requirement: at least 350mA continuous on `3V3` to preserve >=2.5x margin for startup/transient conditions.

Regulator decision for this board:

- `LMZM23601V33` was checked first per plan, but direct assembly availability is currently constrained.
- Selected fallback: `AP63203WU-7` (`C780769`), fixed 3.3V synchronous buck with 2A rating and broad stock availability.
- This materially improves thermal behavior versus linear drop from 12V while keeping cost within target.

## Output power stage

Per channel:

- IRLML6344 MOSFET low-side switch
- 100R gate resistor
- 10k gate pulldown
- 2A PTC fuse in series with load supply
- SS34 flyback diode for inductive load protection

## Override front-end

Per override channel:

```text
3V3 -> 10k pull-up -> switch_node -> 10k series -> rc_node -> SN74LV14A input
                             |                             |
                           switch                        100nF
                             |                             |
                            GND                           GND
```

- Schmitt trigger cleans RC edges/noise
- Buffered HIGH means local override active
- Buffered LOW means serial controls output

## RS-485 architecture

Two SP3485EN devices:

- RX path: cable RX pair -> transceiver -> `PA10 (USART1_RX)`
- TX path: `PA9 (USART1_TX)` -> transceiver -> cable TX pair (heartbeat)
- One 120R termination on receiver pair
- One SM712 TVS per pair

Point-to-point cable crossover at installation (required for full duplex):

- Input `TX+` -> Output `RX+`
- Input `TX-` -> Output `RX-`
- Output `TX+` -> Input `RX+`
- Output `TX-` -> Input `RX-`

## USB DFU and SWD

- Native USB:
  - `PA11 = USB_DM`
  - `PA12 = USB_DP`
- 22R D+/D- series resistors and CC pull-downs retained
- SWD header retained for debug/recovery
- Full SWD/connector pin map is defined in `hardware/SCHEMATIC_APPENDIX_OUTPUT.md`

BOOT0:

- Net naming follows `PA14-BOOT0` (shared with SWCLK during SWD attach)
- 10k pulldown default
- `SW2` asserts BOOT0 high
- `SW1` resets NRST

## Pin updates

- `GATE_CH5` is on `PA8` to keep `PA11` available for USB
- Override channels remain on `PA0`, `PA1`, `PA4`, `PA5`, `PA6`, `PA7`, `PB0`, `PB1`

## Failsafe behavior

- If no valid state frame is received for 100ms, all outputs force OFF
- Link LED indicates healthy/failsafe state
- Heartbeat frame reports failsafe flag back to input board

## Layout and implementation notes

- Route high-current output paths first (12V trunk, PTC branches, load returns).
- Use the quantified width/via targets from `hardware/PCB_LAYOUT_GUIDE.md` for 2A branches and shared rails.
- Keep each MOSFET + flyback + terminal loop compact to minimize switching noise.
- Keep RS-485 and USB routing away from high di/dt output switching regions.
- Keep SWD/NRST/BOOT0 physically accessible for bring-up and field recovery.

See also:

- `hardware/SCHEMATIC_GUIDE.md`
- `hardware/SCHEMATIC_APPENDIX_OUTPUT.md`
- `hardware/PCB_LAYOUT_GUIDE.md`
- `hardware/PCB_APPENDIX_OUTPUT.md`
