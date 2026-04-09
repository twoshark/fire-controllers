# PCB Layout Guide

This guide is the board-layout counterpart to `hardware/SCHEMATIC_GUIDE.md`.
It assumes the current architecture:

- STM32G0B1CBT6 MCU
- Full-duplex RS-485 (2x SP3485EN per board)
- Native USB on PA11/PA12
- RC + Schmitt digital input front-end
- SWD + NRST + BOOT0 debug/recovery controls

## 1) Placement order (high-confidence workflow)

1. Place connectors and mounting holes (mechanical constraints first).
2. Place power entry/protection parts and bulk capacitors.
3. Place MCU and local decoupling.
4. Place RS-485 transceivers + termination + TVS near RS-485 terminal.
5. Place USB connector + D+/D- series resistors near MCU pins.
6. Place Schmitt input conditioning channels as repeated blocks.
7. Place output MOSFET/PTC/flyback blocks (output board).
8. Place status LED block along one enclosure-visible edge.
9. Place SWD/NRST/BOOT0 where probe/tool access is easy.

## 2) Layer stack and trace guidance

Recommended minimum:

- 2-layer is feasible if current paths are short/wide and grounding is disciplined.
- 4-layer is preferred for cleaner return paths and easier EMI control.

For 2-layer:

- Top: component + critical routing
- Bottom: near-continuous GND plane with minimal slotting

## 3) Power routing

### Input board

- Keep the 12V entry path compact: `J1 -> D1 -> F1 -> bulk/HF caps`.
- Place `C17` and `C6` adjacent to the 12V entry node.
- Place AMS1117 close to output caps (`C18`, `C19`).

### Output board

- Route high-current output path first:
  - 12V rail trunk -> per-channel PTC -> output terminal
  - return path to load ground terminal with low impedance
- Use wide copper for all high-current segments (size by your current/temperature rise target).
- Keep MOSFET + flyback + terminal loop tight per channel.

## 4) MCU + decoupling

- Place each 100nF decoupler at its IC supply pin with very short loop to GND.
- Keep VREF/analog decoupling traces short and quiet.
- Keep BOOT0 pulldown physically close to MCU pin.
- Keep NRST network close to MCU NRST pin and SWD header route short.

## 5) RS-485 full-duplex layout

- Place both SP3485EN parts close to the RS-485 terminal connector.
- Keep each differential pair (`TX+/-`, `RX+/-`) tightly coupled and length-matched within each pair.
- Place 120R termination directly across receiver A/B pins.
- Place SM712 as close as practical to connector entry on each pair.
- Keep pair reference over continuous ground; avoid crossing plane gaps.

## 6) USB layout

- Route PA11/PA12 to USB connector as a coupled differential pair.
- Place 22R D+/D- resistors near MCU pins.
- Keep stubs and vias to a minimum.
- Keep noisy switching nodes (MOSFET drains, PTC branches) away from USB pair.

## 7) RC + Schmitt input/override channels

Per-channel block:

- Place `R_pullup`, `R_series`, and `C` as a compact repeated cell.
- Place Schmitt ICs (`SN74LVC14A`) near MCU-side routing transition.
- Keep channel routing topologically identical across channels where possible.
- Route switch return currents to ground without long shared narrow bottlenecks.

## 8) Output stage (output board)

- Keep each gate loop short (`MCU -> R_gate -> MOSFET gate -> R_pd -> GND`).
- Place flyback diode physically close to load switching node and 12V rail tie point.
- Separate high di/dt output-current loops from MCU/USB/RS-485 regions.

## 9) Grounding and return paths

- Maintain a continuous ground reference under MCU, USB, and RS-485.
- Connect noisy power-return regions with low-impedance stitching to global ground.
- Avoid narrow necks in return path that force unrelated currents through one segment.

## 10) Debuggability and DFM

- Keep SWD header unobstructed by tall components.
- Expose NRST and BOOT0 controls without removing enclosure internals.
- Add test points for:
  - 12V, 3V3, GND
  - USART1 TX/RX logic nodes
  - RS-485 differential pairs (if space allows)
- Ensure silkscreen includes connector pin labels and channel numbering.

## 11) Pre-release layout checklist

- [ ] Power-entry polarity/protection path matches schematic.
- [ ] Every IC has local decoupling placed and routed.
- [ ] RS-485 TVS + termination are on the correct pair and close enough.
- [ ] USB pair is short, coupled, and clear of noisy nodes.
- [ ] BOOT0 defaults LOW and DFU button wiring is correct.
- [ ] SWD header pinout/orientation matches documentation.
- [ ] Output current paths sized for expected load current and temperature rise.
- [ ] Mounting holes and connector clearances fit enclosure plan.
