# Panel cutouts & as-built dimensions

Sources: v1.0.0 EasyEDA PnP, Mean Well RS-15 / HLG, DT·DTP, IEC C14, M12 PG9.

All dimensions **mm**. Heights = hole center from **outer bottom**.

---

## PCB keep-outs (Gerber outline + margin)

PCB mount pads: hole **100 mil (Ø2.54)** · pad **160 mil (Ø4.06)** → **M2** — boss XY in [`MOUNTING.md`](MOUNTING.md). Gerbers: `../input/easyeda/gerber/`, `../output/eda-exports/gerber/`.

| Board | Outline (GKO) | Floor keep-out | LED block |
| --- | ---: | ---: | --- |
| Input PCB | **83.1 × 79.0** | **86 × 83** | `LED1`…`LED10` X≈79.3; Y span **31.3**; pitch ≈**3.5** |
| Output PCB | **124.0 × 112.4** | **126 × 114** | `PWR`/`LINK`/`CH0`…`CH7` X≈117.2; Y span **31.5**; pitch ≈**3.5** |
| Buttons daughter | **71.0 × 44.7** | under lid | 4× Ø2.54 corners |

### Input connector mids

| Ref | Mid X | Mid Y | Notes |
| --- | ---: | ---: | --- |
| `J1` | 5.6 | −12.8 | 12 V in |
| `J3` | 5.5 | −24.5 | switch GND |
| `J2b` | 6.0 | −40.8 | CH4..7 |
| `J2a` | 6.0 | −64.1 | CH0..3 |
| `CN2` | 51.8 | −5.2 | RS-485 |
| `J5` USB-C | 48.9 | −73.8 | open lid |
| `SW1`/`SW2` | ≈77.7 | −64 / −57 | RESET/BOOT |

LED edge (`X≈79`) → LED window wall.

### Output connector mids

| Ref | Mid X | Mid Y | Notes |
| --- | ---: | ---: | --- |
| `J1` | 5.5 | −19.9 | 12 V in |
| `F9` | 10.5 | −5.5 | ATO · open lid |
| `J6` | 40.3 | −5.2 | load +12 V (both poles) |
| `J5a`/`J5b` | 60.6 / 90.0 | −5.3 / −5.2 | OUT0..7 |
| `J2` | 118.6 | −33.7 | RS-485 |
| `J7` USB-C | 70.0 | −107.1 | open lid |
| LEDs | ≈117.2 | −85.2…−53.7 | LED window |

---

## Panel hardware cutouts

| Feature | Part | Cutout | Notes |
| --- | --- | --- | --- |
| POWER rocker | KCD4 + boot | **30.0 × 22.0** | **Inputs only** (2×) |
| AC inlet | IEC C14 | **27.5 × 20.0** snap or flange **27 × 19** + Ø3.2 @ **42.5** | + rubber cover |
| RS-485 | M12-5 PG9/M16 | **Ø16.2** / **Ø16.1** | + dust cap |
| 12 V | DTP04-2P in pocket | pocket **≈18 × 22** + lip | **25 A**; IP67 mated |
| SOL | DT04-2P in pocket | pocket **≈16 × 18** + lip | 13 A; pitch ≥25 |
| Arcade | EG STARTS | **Ø28** (verify ring; may be Ø24) | — |
| LED window | PC + foam gasket | **40 × 10** | not open slot |

### Fixed heights / pitches

| Dim | Value |
| --- | ---: |
| DTP 12 V center height | **40** |
| DTP horizontal | centered on short-end mate face |
| Output outer | **220 × 170 × 85** |
| SOL pitch (CL) | **≥25** |
| Arcade pitch sign / mp | **≥45** / **≥50** |
| Cutout → corner | **≥12** |
| Wall thickness | **2.5–3.0** |

---

## PSU mechanical

| PSU | L × W × H | Mount |
| --- | ---: | --- |
| RS-15-12 (in input box) | **62.5 × 51 × 28** | 2× M3 L≤3–4 |
| HLG-240H-12 (OTS) | **244 × 68 × 39** | structure / shade — [`POWER_OTS.md`](POWER_OTS.md) |

---

## Cable lengths

| Cable | Qty | Length |
| --- | ---: | ---: |
| C13→C14 (inputs) | 2 | **6 ft** |
| SJTW → HLG | 2 | **6 ft** |
| HLG DC → DTP | 2 | **≤ 4 ft** · 12 AWG |

Internal: AC 18 AWG · 12 V 12 AWG · SOL 18 AWG (star from `J6`) · RS-485 22–24 AWG.
