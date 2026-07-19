# Enclosures (v1.0.0)

**Four printed boxes** + **two OTS outdoor PSUs**. Bed ≤ **256 × 256 mm**.

| Unit | Role | CAD (body + lid) | Other |
| --- | --- | --- | --- |
| [`sign-input/`](sign-input/) | Arcade hex · input PCB · RS-15 | [`BODY`](sign-input/BODY.md) · [`LID`](sign-input/LID.md) | BOM · Wiring |
| [`mp-input/`](mp-input/) | Arcade + ALL · input PCB · RS-15 | [`BODY`](mp-input/BODY.md) · [`LID`](mp-input/LID.md) | BOM · Wiring |
| [`sign-output/`](sign-output/) | SOL0..4 · 12 V from HLG | [`BODY`](sign-output/BODY.md) · [`LID`](sign-output/LID.md) | BOM · Wiring |
| [`mp-output/`](mp-output/) | SOL0..2 · 12 V from HLG | [`BODY`](mp-output/BODY.md) · [`LID`](mp-output/LID.md) | BOM · Wiring |
| HLG-240H-12 ×2 | IP67 120 VAC→12 V/16 A (panels 5+3 SOL) | OTS — not printed | [`POWER_OTS.md`](POWER_OTS.md) |

| Root | Contents |
| --- | --- |
| [`CAD_PARTS.md`](CAD_PARTS.md) | **What to model** — body + lid workflow for all 4 |
| [`LED_WINDOW.md`](LED_WINDOW.md) | Lid LED pockets · dividers · clear inserts |
| [`POWER_OTS.md`](POWER_OTS.md) | Outdoor PSU |
| [`SHOPPING_LIST.md`](SHOPPING_LIST.md) | Cart |
| [`SEALING.md`](SEALING.md) | Dust/rain |
| [`MOUNTING.md`](MOUNTING.md) | Inserts · pillars · flat LED-up board map |
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
| **FRONT** | Lid latches only (no LED cutout) |
| **LEFT** | Inputs: C14 + POWER + M12 · Outputs: SOL + M12 |
| **BACK** | Outputs: DTP 12 V · CL **40** |
| **Top / lid** | Front LED strip + dividers + clear insert · arcade farther back (inputs) · **internal bociloy 1"** hinges on BACK |
| **Service** | Open lid (front latches) → USB-C / RESET·BOOT / `F9` |

| Unit | Power control |
| --- | --- |
| Inputs | KCD4+boot on AC → RS-15 |
| Outputs | Unplug HLG AC |
| HLG | Outdoor cord → waterproof splice → L/N/FG |
