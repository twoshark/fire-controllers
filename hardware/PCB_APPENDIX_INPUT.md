# Input Board PCB Appendix (Executable Constraints)

This appendix defines board-specific placement/routing/test constraints for the input board.

## 1) Placement zone map (input board)

Use edge zoning below for deterministic assembly and enclosure wiring.

| Zone | Required content | Notes |
| --- | --- | --- |
| Power-entry zone | `J1`, `D1`, `F1`, `C17`, `C6`, `U4`, `L1`, `C18`, `C19` | Keep as compact buck chain from `VIN_12V_IN` to `3V3`; keep `SW` node short. |
| RS-485 zone | `J4`, `U2A`, `U2B`, `R27`, 2x SM712 | TVS closest to connector. Termination at RX IC pins. |
| USB/debug zone | `J5`, `R31/R32`, `R29/R30`, `J6`, `SW1`, `SW2` | Keep SWD probe clearance. Keep BOOT0/NRST reachable. |
| Input-channel zone | `J2a`, `J2b`, RC cells, `U5/U6` | Replicate geometry CH0..CH7 uniformly. |
| MCU core zone | `U1` + local decoupling | Keep centered between RS-485/USB/input zones. |
| LED edge zone | `LED1..LED10` + series resistors | Single edge-aligned block for window/light-pipe. |

## 2) Net-class and routing rules (input board)

| Net class | Nets | Min width | Min clearance | Via guidance |
| --- | --- | --- | --- | --- |
| Low-current power | `VIN_12V_IN`, `12V_MAIN` (input board only) | 40 mil | 1.0 mm to USB/MCU traces | Avoid unnecessary transitions. |
| Logic 3V3 | `3V3`, GPIO, Schmitt I/O | 8 mil | 0.5 mm | Standard 0.3 mm via allowed. |
| USB pair | `USB_DP`, `USB_DM` | per impedance target | pair-to-other >=1.0 mm near noisy zones | Max one via/line. |
| RS-485 TX pair | `RS485_TX+/-` | 10 mil baseline | >=1.0 mm from switching nodes | Keep tightly coupled and same layer where possible. |
| RS-485 RX pair | `RS485_RX+/-` | 10 mil baseline | >=1.0 mm from switching nodes | Place 120R directly at RX transceiver pins. |

## 3) Input-channel routing template (repeat CH0..CH7)

```text
TerminalPin -> pull-up/switch node -> series R -> RC node -> Schmitt input -> Schmitt output -> MCU GPIO
```

Rules:

- Keep CH topology identical for all 8 channels.
- Match RC node parasitics by similar trace lengths/shapes.
- Route switch-return currents directly to local ground (avoid thin shared bottlenecks).

## 4) USB constraints (input board)

- USB differential impedance target: `90R +/-15%`.
- D+/D- routed length target: <=50 mm.
- Intra-pair mismatch: <=2 mm.
- No branch stubs; use in-line test pads only if needed.
- Place USB ESD (if populated) within 5 mm of connector pins.
- Keep D+/D- and RS-485 pairs from crossing split planes.

## 5) RS-485 constraints (input board)

- Pair assignment at connector:
  - `J4.1/J4.2` = TX pair
  - `J4.3/J4.4` = RX pair
- Keep TX and RX pairs physically separated at the connector breakout.
- Place SM712 return path to local reference near connector entry.
- Shield handling follows top-level shield/chassis policy.

## 6) Test-point matrix (input board)

| Test point | Net | Expected state |
| --- | --- | --- |
| TP1 | `VIN_12V_IN` | 12V nominal |
| TP2 | `12V_MAIN` | 12V after polarity/fuse path |
| TP3 | `3V3` | 3.3V regulator output |
| TP4 | `GND` | 0V |
| TP5 | `USART1_TX` | active 1kHz frame stream |
| TP6 | `USART1_RX` | active heartbeat stream |
| TP7 | `RS485_TX+/-` | differential serial activity |
| TP8 | `RS485_RX+/-` | differential serial activity |
| TP9 | `PA14-BOOT0` | LOW default, HIGH only while SW2 pressed |
| TP10 | `NRST` | HIGH default, LOW pulse on SW1 press |

## 7) Bring-up probe order (input board)

1. Validate `VIN_12V_IN` and `12V_MAIN`.
2. Validate `3V3`.
3. Confirm `PA14-BOOT0` default LOW and NRST behavior.
4. Attach SWD and program firmware.
5. Verify `USART1_TX` periodic state frames.
6. Verify `USART1_RX` heartbeat reception.
7. Validate channel LEDs CH0..CH7 and link LED behavior.
