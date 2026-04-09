# Output Board PCB Appendix (Executable Constraints)

This appendix defines board-specific placement/routing/test constraints for the output board.

## 1) Placement zone map (output board)

| Zone | Required content | Notes |
| --- | --- | --- |
| Power-entry zone | `J1`, bulk caps, LDO chain (`U4`) | Keep `12V_MAIN` entry and `3V3` generation compact. |
| High-current output zone | `Q1..Q8`, `F1..F8`, flyback diodes, `J5a/J5b`, `J6` | Dedicated corridor; keep away from USB and MCU. |
| Override-input zone | `J3a/J3b`, `J4`, RC cells, `U5/U6` | Uniform per-channel topology. |
| RS-485 zone | `J2`, `U2A/U2B`, `R52`, 2x SM712 | Keep near cable entry. |
| USB/debug zone | `J7`, D+/D- resistors, `J8`, `SW1/SW2` | Keep accessible near board edge. |
| MCU core zone | `U1` + decoupling | Between noisy/high-current and sensitive interface zones. |
| LED edge zone | `LED1..LED10` + resistors | Single edge-aligned status window. |

## 2) Net-class and routing rules (output board)

| Net class | Nets | Min width | Min clearance | Via guidance |
| --- | --- | --- | --- | --- |
| Main high-current trunk | `12V_MAIN`, `LOAD_12V`, `LOAD_GND_RTN` | 300 mil pour-equivalent for up to 16A peak | >=1.0 mm from USB/MCU traces | Dense stitching vias every 5-10 mm if both layers used. |
| Per-channel branches | `F(n)` to channel load path (2A/channel) | 60 mil | >=1.0 mm from sensitive traces | >=3 vias in parallel at layer transitions. |
| Mid-current shared branches | shared 4A to 8A segments | 150 mil to 300 mil | >=1.0 mm | >=5 vias (4A), >=9 vias (8A). |
| Logic 3V3 | GPIO, Schmitt, LED controls | 8 mil | 0.5 mm | Standard 0.3 mm via allowed. |
| USB pair | `USB_DP`, `USB_DM` | per impedance target | keep clear of high-current zone | Max one via/line. |
| RS-485 pairs | `RS485_TX+/-`, `RS485_RX+/-` | 10 mil baseline | >=1.0 mm from high di/dt nodes | Length match within each pair. |

## 3) Output-stage routing constraints

Per channel loop:

```text
12V_MAIN -> PTC(Fn) -> load connector pin -> external load -> OUT_CHn_SW -> MOSFET -> LOAD_GND_RTN
```

Rules:

- Keep each channel loop compact and independent.
- Place flyback diode close to switched node and local 12V tie.
- Keep gate loop (`MCU -> Rg -> gate -> Rpd -> GND`) short and quiet.
- Avoid routing logic traces through MOSFET/PTC hot corridor.

## 4) Thermal execution checklist (output board)

- Provide >=100 mm^2 combined local copper (both layers preferred) around each MOSFET drain/source network.
- Keep PTCs at least 3 mm apart edge-to-edge.
- Reserve >=250 mm^2 copper for AMS1117 thermal spreading.
- Do not place electrolytics directly against regulator body.
- If projected steady-state LDO dissipation >0.8W, treat as design risk and rework copper/airflow/load budget.

## 5) RS-485 and USB constraints (output board)

- `J2.1/J2.2` = TX pair, `J2.3/J2.4` = RX pair, `J2.5` GND, `J2.6` shield.
- `R52` 120R only on RX pair.
- Place SM712 parts at connector entry and keep return path short.
- USB pair targets:
  - `90R +/-15%` differential impedance
  - <=50 mm routed length
  - <=2 mm mismatch
  - no branch stubs

## 6) Test-point matrix (output board)

| Test point | Net | Expected state |
| --- | --- | --- |
| TP1 | `VIN_12V_IN` | 12V nominal |
| TP2 | `12V_MAIN` | 12V bulk rail |
| TP3 | `3V3` | 3.3V logic rail |
| TP4 | `GND` | 0V |
| TP5 | `USART1_RX` | valid state frame stream |
| TP6 | `USART1_TX` | heartbeat frame stream |
| TP7 | `RS485_RX+/-` | differential serial activity |
| TP8 | `RS485_TX+/-` | differential serial activity |
| TP9 | `PA14-BOOT0` | LOW default |
| TP10 | `NRST` | HIGH default |
| TP11 | `GATE_CH0..CH7` | follows serial/override/failsafe logic |
| TP12 | `OUT_CH0..CH7` | switched low-side outputs |

## 7) Bring-up probe order (output board)

1. Validate `VIN_12V_IN`, `12V_MAIN`, and `3V3`.
2. Verify BOOT0/NRST behavior and SWD programming access.
3. Confirm receive path (`USART1_RX`) with input-board traffic.
4. Confirm heartbeat transmit (`USART1_TX`) at 10Hz frame cadence.
5. Force failsafe and verify all `GATE_CHn` and `OUT_CHn` go OFF.
6. Validate override priority channel-by-channel.
7. Validate output LEDs and link LED fault/healthy indications.
