# Connector ratings audit

Loads: **2 A / channel** · sign peak **10 A** · mp peak **6 A** · LRS-200-12 **17 A** max.

| Interface | Part | Rating | Load | Termination | Verdict |
| --- | --- | --- | ---: | --- | --- |
| 12 V box link | PanelPole2 + PP30 contacts | **30 A** · 12–14 AWG | ≤10 A (sign) / ≤17 A PSU | **Crimp** (not solder) | OK |
| 12 V field cable | EX-12-5 | 12 AWG · PP30 | same | Pre-made | OK — do **not** use EX-10-5 (needs 45 A contacts) |
| Output PCB `J1`/`J6` | KF128-7.5-2P | LCSC **24 A** (some KEFA sheets **10 A**) | 6–10 A | **Screw** | OK at sign/mp; parallel both `J6` poles for +12V |
| Output `J5a`/`J5b` | DB128V-5.08 | **16 A**/pin | 2 A/pin | **Screw** | OK |
| Input `J1` | DB128V-5.08 | **16 A** | ≪1 A | **Screw** | OK |
| RS-485 panel | M12-5 A-code | **4 A**/pin · ≤**22 AWG** / 0.75 mm² | signal ≪1 A | Solder cup or field screw (see pack) | OK for signal only |
| SOL panel | AT 2-pin (AT04/AT06) | **13 A** · **16–18 AWG** | 2 A | **Crimp** size-16 | OK |
| AC inlet | IEC C14 | typically **10–15 A** | LRS ~2 A @115 V | QC / screw | OK |
| POWER rocker | WRG32F2FBBNN | **16 A** @125/250 VAC | ~2 A | **0.250" QC** tabs | OK |
| Fans | MF60151V2 leads | ≪0.2 A each | on LRS DC | splice / QC | OK |
| USB HangTon | USB-C | data / 5 V debug | not in 12 V path | Plug jumper | OK |
| Channel PTC | 1812L200/16GR | **2 A** hold | 2 A design | SMD | At rating — no margin |
| `F9` | 25 A ATO | holder ~22.5 A cont. | 6–10 A | Blade | OK |

## Assembly (no mystery)

| Joint | Method |
| --- | --- |
| PCB power / SOL / RS-485 | Screw into `DB128V` / `KF128` |
| PanelPole ↔ LRS / `J1` | Crimp PP30 contacts (12 AWG) |
| AT SOL contacts | Crimp size-16 (16–18 AWG) — Deutsch/AT tool |
| M12 RS-485 rear | Solder cups **or** field-wireable screw (per purchased pack) |
| C14 / WRG32 | 0.250" female QC on 18 AWG |
| HangTon ↔ board USB | Pre-made USB-C M–M jumper |

## Rejected / fixed

| Was | Problem | Now |
| --- | --- | --- |
| M12-8 for 12 V bus | Signal connector, pin-paralleling for 10–17 A | PanelPole2 |
| M12-5 for SOL | 4 A OK but ~22 AWG max; signal family | AT 2-pin 13 A / 18 AWG |
| EX-10-5 + PP30 | 10 AWG needs 45 A contacts | EX-12-5 |
| Docs “M12 screw-terminal” | B0CFFX6JW4 listing is cable set, not proven screw panel | RS-485 only; verify pack or use panel pigtail M12 |
