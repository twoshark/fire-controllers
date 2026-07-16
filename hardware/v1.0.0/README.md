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

| File | EasyEDA export |
| --- | --- |
| `exports/BOM_input.csv` / `BOM_output.csv` | Bill of Materials (CSV) |
| `exports/PnP_input.csv` / `PnP_output.csv` | Pick and Place / CPL |
| `exports/Netlist_input.asc` / `Netlist_output.asc` | PADS netlist |
| `input-buttons-pcb/exports/` | Daughter PCB BOM / PnP / netlist |

## Optional next export

If you can add one more file, export **Schematic PDF** (File → Export → PDF) for each board.
Easy in EasyEDA, useful for visual silk/net cross-check. Drop as:

- `exports/Schematic_input.pdf`
- `exports/Schematic_output.pdf`
- `input-buttons-pcb/exports/Schematic.pdf` (and ideally a connectorized board rev)
