# Cost options → under $300

Current kit is **6** boxes (2 inputs + 2 outputs + 2 power).  
Lean cart below targets **≤ $300 for all 6**. If you meant only **4** controller boxes (no power shells), see Option D.

Removed by default (no quality loss for field use):
- External RESET/BOOT (Adafruit) — open the lid, use on-board tactiles
- HangTon panel USB — open lid / grommet for DFU
- DigiKey Amphenol AT kits — same DT 2-pin family via Amazon kit
- Amazon markup on PanelPole2 — buy from Powerwerx.com
- Pre-made EX-12-5 — DIY 12 AWG + PP30 (**≤ 4 ft** each link)

Kept (site assumptions):
- **4× IEC C13–C14 6 ft** cords — every AC box to a 120 V receptacle
- **2× Powerpole jumpers ≤ 4 ft** — power box ↔ matching output box

---

## Recommended lean cart (~$297)

| Keep | Why | Ext |
| --- | --- | ---: |
| Bravo 2× RS-15-12 + 2× LRS-200-12 | Already cheap / correct PSUs | $69.20 |
| EG STARTS triangle ×2 | Locked part | $23.98 |
| C14 pack ×4 | Needed | $8.19 |
| C13–C14 cords **4× 6 ft** | Mains to each AC box | ~$16 |
| PanelPole2 ×4 @ Powerwerx **$19.99** | Same part, not Amazon $27.78 | $79.96 |
| PP30-10 + 12 AWG zip (DIY 2× ≤4 ft) | Same Powerpole quality | ~$21 |
| M12-5 **1×** 4-set | 4 panel + 2 field cables | $23.79 |
| JRready DT 2-pin **10 sets** [B0BKRRGRQY](https://www.amazon.com/dp/B0BKRRGRQY) | IP67 · 13 A · replaces DigiKey AT | ~$25 |
| KCD4 DPST rocker **6-pack** (30×22 class) | Same job as WRG32 | ~$8 |
| 60 mm 12 V fans **4-pack** + filter/grill pack | Adequate cooling | ~$22 |
| | **Sum** | **~$297** |

(+ scrap Al for LRS plates · crimp tools if needed)

---

## Option menu (pick any)

| ID | Change | Saves | Tradeoff |
| --- | ---: | --- |
| **A** | Keep HangTon USB on 4 controller boxes | **−$0** / costs **+$29** | Convenient DFU without opening |
| **B** | Keep Newark WRG32 (UL) instead of KCD4 | **−$0** / costs **+$9** | Name-brand UL rocker |
| **C** | Keep DigiKey Amphenol AT kits | costs **+$55** | OEM AT vs DT-compatible kit |
| **D** | **4 boxes only** — drop both `*-output-power` shells, fans, 2× LRS, 2× PanelPole OUT | **−~$160** | Need another 12 V source for outputs (not a complete system) |
| **E** | Print Powerpole panel mounts + PP30 only (no PanelPole2) | **−~$60** | You design/print weather seal |
| **F** | Skip purchased fans — convection + shade only | **−~$22** | Hotter LRS; derate load |
| **G** | One shared LRS-200 for both outputs (one power box) | **−~$40** | One cable run / single point of failure |

---

## Do not cut (false economy)

| Item | Why |
| --- | --- |
| LRS-200-12 / RS-15-12 | Load and form-factor locked |
| Powerpole (genuine PP30) for 12 V bus | Current rating |
| DT/AT 2-pin for SOL | 2 A × outdoor |
| M12 for RS-485 | Signal integrity / outdoor |

---

## vs previous ~$530

| Cut | Saved |
| --- | ---: |
| Adafruit RESET/BOOT | $39.60 |
| HangTon + USB pack | $37.86 |
| Long/premium C13 cords → 4× 6 ft basic | ~$20 |
| PanelPole Amazon→Powerwerx | $31.16 |
| DigiKey AT → Amazon DT | ~$55 |
| EX-12-5 → DIY | ~$15 |
| Extra M12 pack | $23.79 |
| Newark rocker/filter premium | ~$12 |
| DigiKey fans → Amazon | ~$10 |
| **Total cut** | **~$250** |
