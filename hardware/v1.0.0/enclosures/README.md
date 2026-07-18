# Enclosures (v1.0.0)

Six printed boxes (bed ≤ **256 × 256 mm**).

| Enclosure | Role | Docs |
| --- | --- | --- |
| [`sign-input/`](sign-input/) | Buttons CH1..CH5 + ALL · input PCB · RS-15-12 | CAD · BOM · Wiring |
| [`mp-input/`](mp-input/) | Buttons CH1..CH3 + ALL · input PCB · RS-15-12 | CAD · BOM · Wiring |
| [`sign-output/`](sign-output/) | Output PCB · SOL0..SOL4 (AT 2-pin each) · 12 V from power box | CAD · BOM · Wiring |
| [`sign-output-power/`](sign-output-power/) | 120 VAC → 12 V · LRS-200-12 · sign-output | CAD · BOM · Wiring |
| [`mp-output/`](mp-output/) | Output PCB · SOL0..SOL2 (AT 2-pin each) · 12 V from power box | CAD · BOM · Wiring |
| [`mp-output-power/`](mp-output-power/) | 120 VAC → 12 V · LRS-200-12 · mp-output | CAD · BOM · Wiring |

| Root | Contents |
| --- | --- |
| [`SHOPPING_LIST.md`](SHOPPING_LIST.md) | Order carts · pin maps |
| [`PARTS_BOM.md`](PARTS_BOM.md) | Shared catalog |
| [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) | Current / termination audit |

## System

```text
sign-input ──RS-485──► sign-output ◄──12 V── sign-output-power ◄── 120 VAC
mp-input   ──RS-485──► mp-output   ◄──12 V── mp-output-power   ◄── 120 VAC
```

## Power

| Box | Path |
| --- | --- |
| Inputs | C14 → POWER → RS-15-12 → PCB `J1` |
| Output controllers | PanelPole2 12V IN → PCB `J1` (`F9` seated) |
| Output power | C14 → POWER → LRS-200-12 → PanelPole2 12V OUT · fans on LRS +V/−V |

## CAD gaps (v1.0.0 PCB exports)

| Item | Status |
| --- | --- |
| PCB outline W×H | Not in EasyEDA exports — keep-outs use PnP bbox + margin |
| PCB mounting holes | None in v1.0.0 BOM/PnP — use corner clips / adhesive standoffs until rev |
| LED window | Size to LED column (~32 mm), not 80 mm |
| Power-box fans | On **long walls** (across LRS width) — end-stack with 215 mm LRS does not fit ≤256 mm bed |
