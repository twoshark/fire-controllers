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
| [`SHOPPING_LIST.md`](SHOPPING_LIST.md) | Cart · ~$282 |
| [`COST_OPTIONS.md`](COST_OPTIONS.md) | Add-backs / cuts |
| [`SEALING.md`](SEALING.md) | Dust/rain strategy |
| [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md) | Diameters · pitches · keep-outs |
| [`MOUNTING.md`](MOUNTING.md) | Inserts · pillars · lid |
| [`CAD_VERIFICATION.md`](CAD_VERIFICATION.md) | Size / bed / DFM audit |
| [`PARTS_BOM.md`](PARTS_BOM.md) | Shared catalog |
| [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) | Current / seal audit |

## System

```text
sign-input ──RS-485──► sign-output ◄──12 V DTP── sign-output-power ◄── 120 VAC
mp-input   ──RS-485──► mp-output   ◄──12 V DTP── mp-output-power   ◄── 120 VAC
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
| **Top** | Arcade (inputs) or openable lid |
| **Sides** | C14 + POWER (AC boxes), DTP 12 V, M12, DT SOL, LED window |
| **12V mate face** | Short end with DTP — power OUT faces controller IN |
| **Service** | Open lid → USB-C / on-board RESET·BOOT |

### 12 V alignment

| Dim | Value |
| --- | --- |
| DTP center height from outer bottom | **40 mm** |
| DTP horizontal | centered on short-end wall |
| Cable | DIY 12 AWG DTP jumper · **≤ 4 ft** |

### Cable lengths

| Cable | Qty | Length |
| --- | ---: | ---: |
| IEC C13→C14 | 4 | **6 ft** |
| 12 V DTP | 2 | **≤ 4 ft** |

### POWER rocker (AC boxes only)

| Box | Switches |
| --- | --- |
| sign-input / mp-input | AC via DPST (C14 → KCD4+boot → RS-15; switch L, optionally N) |
| sign-output-power / mp-output-power | AC via DPST (C14 → KCD4+boot → LRS) — **also kills 12 V to output box** |
| sign-output / mp-output | **No rocker** — kill via matching power box |

Part: **KCD4** DPST · **30 × 22 mm** · side wall · silicone boot.

## Power path

| Box | Path |
| --- | --- |
| Inputs | C14 → POWER → RS-15-12 → PCB `J1` |
| Output controllers | DTP IN → PCB `J1` (`F9` seated) |
| Output power | C14 → POWER → LRS-200-12 → DTP OUT · fans on LRS +V/−V |

## CAD status

| Item | Status |
| --- | --- |
| Envelopes | Locked in [`CAD_VERIFICATION.md`](CAD_VERIFICATION.md) (power **256×200×100**) |
| Mounting | [`MOUNTING.md`](MOUNTING.md) — M3/M4 heat-set + clamp pillars |
| PCB outline | PnP keep-outs in [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md) |
| LED window | **40 × 10** + PC lens + gasket |
| EG STARTS / C14 | Verify ring Ø and snap vs flange before freeze |
| Print | Body open-up · lid groove up · power box no brim on 256 edge |
