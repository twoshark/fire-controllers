# Enclosures (v1.0.0)

**Four printed boxes** + **two OTS outdoor PSUs**. Bed ≤ **256 × 256 mm**.

| Unit | Role | Docs |
| --- | --- | --- |
| [`sign-input/`](sign-input/) | Arcade hex · input PCB · RS-15-12 | CAD · BOM · Wiring |
| [`mp-input/`](mp-input/) | Arcade + ALL center · input PCB · RS-15-12 | CAD · BOM · Wiring |
| [`sign-output/`](sign-output/) | Output PCB · SOL0..4 · 12 V from HLG | CAD · BOM · Wiring |
| [`mp-output/`](mp-output/) | Output PCB · SOL0..2 · 12 V from HLG | CAD · BOM · Wiring |
| HLG-240H-12 ×2 | IP67 120 VAC→12 V/16 A (5+3 SOL) | [`POWER_OTS.md`](POWER_OTS.md) |

| Root | Contents |
| --- | --- |
| [`POWER_OTS.md`](POWER_OTS.md) | Outdoor PSU |
| [`SHOPPING_LIST.md`](SHOPPING_LIST.md) | Cart |
| [`SEALING.md`](SEALING.md) | Dust/rain |
| [`MOUNTING.md`](MOUNTING.md) | Inserts · pillars |
| [`CAD_VERIFICATION.md`](CAD_VERIFICATION.md) | Size / bed / DFM |
| [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md) | Cutouts · keep-outs |
| [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) | Ratings |

## System

```text
sign-input ──RS-485──► sign-output ◄──DTP── HLG-240H-12 ◄── 120 VAC
mp-input   ──RS-485──► mp-output   ◄──DTP── HLG-240H-12 ◄── 120 VAC
```

## Orientation

| Face | Contents |
| --- | --- |
| **FRONT** | LED window |
| **LEFT** | Inputs: C14 + POWER + M12 · Outputs: SOL + M12 |
| **BACK** | Outputs: DTP 12 V (near `J1`) · CL **40** |
| **Top** | Arcade (inputs) or lid · **internal bociloy 1"** hinges on BACK |
| **Service** | Open lid (front latches) → USB-C / RESET·BOOT / `F9` |

| Unit | Power control |
| --- | --- |
| Inputs | KCD4+boot on AC → RS-15 |
| Outputs | Unplug HLG AC |
| HLG | Outdoor cord → waterproof splice → L/N/FG |
