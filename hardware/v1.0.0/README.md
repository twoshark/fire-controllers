# Hardware v1.0.0 (2026-07-15)

Board truth for printed EasyEDA PCBs (**v1.0.0**). Derived only from EasyEDA exports in [`exports/`](exports/).

| Doc | Purpose |
| --- | --- |
| [`PIN_MAP.md`](PIN_MAP.md) | MCU GPIO ↔ channels / LEDs / UART (firmware truth) |
| [`INPUT_BOARD.md`](INPUT_BOARD.md) | Input connectors, power, RS-485, channels |
| [`OUTPUT_BOARD.md`](OUTPUT_BOARD.md) | Output connectors, power, gates, loads |
| [`BRINGUP.md`](BRINGUP.md) | First power-on, flash, link test |
| [`input-buttons-pcb/`](input-buttons-pcb/) | ALL FIRE diode-OR daughter PCB |
| [`enclosures/`](enclosures/) | sign/mp input+output; budget BOM + [`SHOPPING_LIST.md`](enclosures/SHOPPING_LIST.md) |
| [`exports/`](exports/) | BOM, Pick-and-Place, Netlist (PADS ASC) |

Firmware: `firmware/input-controller`, `firmware/output-controller`.
Protocol: [`../../docs/serial-protocol.md`](../../docs/serial-protocol.md).

## Exports present

| Path | EasyEDA export |
| --- | --- |
| `input/easyeda/BOM_input.csv`, `PnP_input.csv`, `Netlist_input.asc` | Input BOM / CPL / PADS |
| `output/eda-exports/BOM_output.csv`, `PnP_output.csv`, `Netlist_output.asc` | Output BOM / CPL / PADS |
| `input/easyeda/gerber/`, `output/eda-exports/gerber/` | Gerber + Excellon (2026-07-18) |
| `input-buttons-pcb/exports/` (+ `gerber/`) | Daughter BOM / CPL / netlist / Gerber |

Mounting-hole XY for enclosures is derived from Gerber drills — see [`enclosures/MOUNTING.md`](enclosures/MOUNTING.md).
