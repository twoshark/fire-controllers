# Connector ratings (lean cart)

Loads: **2 A/ch** · sign **10 A** · mp **6 A** · LRS **17 A** max.

| Interface | Part | Rating | Load | Joint | Verdict |
| --- | --- | --- | ---: | --- | --- |
| 12 V panel | PanelPole2 + PP30 | 30 A · 12–14 AWG | ≤10–17 A | Crimp | OK |
| 12 V field | DIY 12 AWG + PP30 | same | same | Crimp | OK |
| SOL | DT 2-pin (JRready kit) | 13 A · 14–18 AWG | 2 A | Crimp | OK |
| RS-485 | M12-5 | 4 A · ≤22 AWG | signal | Solder/field | OK |
| POWER | KCD4 DPST | 20 A/125 VAC | AC ~2 A / DC ≤10 A | QC | OK at 12 V |
| AC inlet | C14 | 10–15 A | ~2 A | QC | OK |
| Fans | 60 mm 12 V | ≪0.2 A | on LRS DC | splice | OK |

PCB terminals remain screw (`J1`/`J5`/`J6`/`CN2`). No panel USB / RESET / BOOT.
