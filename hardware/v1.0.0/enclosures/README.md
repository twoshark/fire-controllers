# Enclosures (v1.0.0)

Six printed boxes (bed ≤ **256 × 256 mm**).

| Enclosure | Role | Docs |
| --- | --- | --- |
| [`sign-input/`](sign-input/) | Triangle arcade hex · input PCB · RS-15-12 | CAD · BOM · Wiring |
| [`mp-input/`](mp-input/) | Triangle arcade + ALL center · input PCB · RS-15-12 | CAD · BOM · Wiring |
| [`sign-output/`](sign-output/) | Output PCB · SOL0..SOL4 · 12 V from power box | CAD · BOM · Wiring |
| [`sign-output-power/`](sign-output-power/) | 120 VAC → 12 V · LRS-200-12 · sign-output | CAD · BOM · Wiring |
| [`mp-output/`](mp-output/) | Output PCB · SOL0..SOL2 · 12 V from power box | CAD · BOM · Wiring |
| [`mp-output-power/`](mp-output-power/) | 120 VAC → 12 V · LRS-200-12 · mp-output | CAD · BOM · Wiring |

| Root | Contents |
| --- | --- |
| [`SHOPPING_LIST.md`](SHOPPING_LIST.md) | Lean cart · ≤ $300 |
| [`COST_OPTIONS.md`](COST_OPTIONS.md) | Add-backs / further cuts |
| [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md) | Diameters · pitches · PCB keep-outs · cable lengths |
| [`PARTS_BOM.md`](PARTS_BOM.md) | Shared catalog |
| [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) | Current / termination audit |

## System

```text
sign-input ──RS-485──► sign-output ◄──12 V── sign-output-power ◄── 120 VAC
mp-input   ──RS-485──► mp-output   ◄──12 V── mp-output-power   ◄── 120 VAC
```

## Orientation (all boxes)

```text
        TOP = arcade (inputs) / lid
              ┌──────────────┐
   LEFT       │              │       RIGHT
   short end  │              │       short end
              └──────────────┘
        BOTTOM = floor / feet
```

| Face | Contents |
| --- | --- |
| **Top** | Arcade buttons only (input boxes). No power rocker, no I/O. |
| **Sides** | POWER rocker, C14, PanelPole, M12, DT SOL, LED window |
| **12V mate face** | Short end with PanelPole — same end on power box (OUT) and output box (IN) |
| **Service** | Open lid → board USB-C / on-board RESET·BOOT tactiles |

### 12 V alignment (power box ↔ output box)

Place boxes with **12V mate faces toward each other**.

| Dim | Value |
| --- | --- |
| PanelPole hole center height from outer bottom | **40 mm** |
| PanelPole horizontal | centered on short-end wall |
| Cable | DIY 12 AWG Powerpole jumper · **≤ 4 ft** between mate faces |

Same **40 mm** height on sign-output, sign-output-power, mp-output, mp-output-power.

### Cable lengths (site)

| Cable | Qty | Length |
| --- | ---: | ---: |
| IEC C13→C14 (each AC box) | 4 | **6 ft** |
| 12 V Powerpole (each power↔output pair) | 2 | **≤ 4 ft** |

Full cutout table: [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md).

### POWER rocker (every box)

| Box | Switches |
| --- | --- |
| sign-input / mp-input | AC L (C14 → rocker → RS-15) |
| sign-output-power / mp-output-power | AC L (C14 → rocker → LRS) |
| sign-output / mp-output | 12 V + (PanelPole red → rocker → `J1.1`) |

Part: **KCD4** DPST (lean) or WRG32 (optional UL) · ~**30 × 22 mm** · **side** wall.

## Power path

| Box | Path |
| --- | --- |
| Inputs | C14 → POWER → RS-15-12 → PCB `J1` |
| Output controllers | PanelPole IN → POWER → PCB `J1` (`F9` seated) |
| Output power | C14 → POWER → LRS-200-12 → PanelPole OUT · fans on LRS +V/−V |

## CAD gaps (v1.0.0 PCB exports)

| Item | Status |
| --- | --- |
| PCB outline W×H | PnP center span + margin → keep-outs in [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md) |
| PCB mounting holes | None in v1.0.0 — corner clips / standoffs |
| LED window | Side wall · **40 × 10 mm** (covers ~31 mm LED row) |
| EG STARTS hole | Measure adapter ring before freeze (Ø28 vs Ø24) |
| C14 pack style | Confirm snap-in vs flange cutout from Amazon pack |
