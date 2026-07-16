# Input Buttons Daughter PCB

EasyEDA export 2026-07-15. Diode-OR board for the enclosure **ALL FIRE** button.

## What it does

Eight `1N4148W` diodes form a wired-OR so one **ALL FIRE** switch can pull multiple channel sense lines to GND at once (same polarity as the per-channel buttons).

| Net | Role |
| --- | --- |
| `IN_CH0_BUTTON_A` … `IN_CH7_BUTTON_A` | To input-board `J2a`/`J2b` channel terminals (and to each channel button NO contact) |
| `ALL_BUTTON_A` | Common cathode side → ALL FIRE switch → `GND` |

Diode orientation (from netlist): `D*.1` on `ALL_BUTTON_A`, `D*.2` on `IN_CHx_BUTTON_A`. With SOD-123 cathode on pin 1 this pulls channels **low** when `ALL_BUTTON_A` is grounded.

## Exports

| File | Contents |
| --- | --- |
| [`exports/BOM.csv`](exports/BOM.csv) | 8× `1N4148W` (LCSC `C22374707`) |
| [`exports/PnP.csv`](exports/PnP.csv) | Pick-and-place |
| [`exports/Netlist.asc`](exports/Netlist.asc) | PADS netlist |

## Gaps / wiring note

This export contains **only the diodes** — no connectors in the BOM/PnP. For enclosure builds:

1. Solder flying leads to the diode pads / add a through-hole header on a board rev, **or**
2. Prefer a small screw-terminal carrier in a future PCB rev (`ALL`, `CH0`…`CH7`, `GND`).

Until then, treat pads as solder posts and strain-relieve every wire.

## Channel usage by enclosure

| Enclosure | Wired channels | Unused diodes |
| --- | --- | --- |
| **sign-input** | CH0..CH4 + ALL | Leave CH5..CH7 pads open (or omit those diodes) |
| **mp-input** | CH0..CH2 + ALL | Leave CH3..CH7 open |

See [`../enclosures/`](../enclosures/) for full box designs.
