# Shopping list — 6 enclosures (lean · ≤ $300)

**sign-input**, **mp-input**, **sign-output**, **sign-output-power**, **mp-output**, **mp-output-power**.

Prices USD 2026-07-18. Options: [`COST_OPTIONS.md`](COST_OPTIONS.md). Cutouts: [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md). Ratings: [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md).

No external RESET/BOOT — open lid, use on-board tactiles. No panel USB — open lid for DFU.

---

## Cable assumptions

| Run | Qty | Length | Notes |
| --- | ---: | ---: | --- |
| Mains C13→C14 | **4** | **6 ft** | Both inputs + both power boxes |
| 12 V Powerpole jumper | **2** | **≤ 4 ft** | Each power box ↔ its output box |

---

## Per box

| Box | AC | POWER | 12 V | RS-485 | SOL | PSU |
| --- | --- | --- | --- | --- | --- | --- |
| sign-input | C14 | KCD4 | — | M12-5 | — | RS-15-12 |
| mp-input | C14 | KCD4 | — | M12-5 | — | RS-15-12 |
| sign-output | — | KCD4 (12 V) | PanelPole IN | M12-5 | 5× DT | — |
| mp-output | — | KCD4 (12 V) | PanelPole IN | M12-5 | 3× DT | — |
| sign-output-power | C14 | KCD4 | PanelPole OUT | — | — | LRS-200-12 |
| mp-output-power | C14 | KCD4 | PanelPole OUT | — | — | LRS-200-12 |

Top: EG STARTS triangle. Sides: POWER + I/O. PanelPole **H = 40 mm** mate faces.

---

## Pin maps

### RS-485 M12-5

| Pin | Signal |
| ---: | --- |
| 1–4 | TX+/TX−/RX+/RX− |
| 5 | GND |
| Shell | SHIELD |

### 12 V — Powerpole

| Pole | Net |
| --- | --- |
| Red | `+12V` |
| Black | `GND` |

Panel: PanelPole2. Field: DIY 12 AWG + PP30, **≤ 4 ft**.

### SOL — DT 2-pin each channel

| Pin | Net |
| ---: | --- |
| 1 | `+12V` (`J6`) |
| 2 | `OUTn` |

---

## Cart 1 — Bravo Electro · $69.20

| Qty | MPN | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | RS-15-12 | $8.60 | $17.20 | [bravoelectro.com](https://www.bravoelectro.com/rs-15-12.html) |
| 2 | LRS-200-12 | $26.00 | $52.00 | [bravoelectro.com](https://www.bravoelectro.com/lrs-200-12.html) |

LRS: **115 V**, trim **12.0 V**. Mech: RS-15 **62.5×51×28** · LRS **215×115×30**.

---

## Cart 2 — Powerwerx · $79.96

| Qty | MPN | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 4 | PanelPole2 | $19.99 | $79.96 | [powerwerx.com](https://powerwerx.com/panelpole-panel-mount-powerpole-black-dual) |

Panel hole **Ø28.6** (1-1/8″). Rear depth **22.2**.

---

## Cart 3 — Amazon

| Qty | Item | ASIN / search | Ext |
| ---: | --- | --- | ---: |
| 2 | EG STARTS triangle LED ×5 | [B07R565HM6](https://www.amazon.com/dp/B07R565HM6) | $23.98 |
| 1 | M12-5 4-set | [B0CFFX6JW4](https://www.amazon.com/dp/B0CFFX6JW4) | $23.79 |
| 1 | C14 inlet ×4 | [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) | $8.19 |
| 4 | IEC C13–C14 cord **6 ft** | search “C13 C14 6ft” | ~$16 |
| 1 | JRready DT 2-pin **10 sets** | [B0BKRRGRQY](https://www.amazon.com/dp/B0BKRRGRQY) | ~$25 |
| 1 | KCD4 DPST rocker **6-pack** 20A/125V · 30×22 | search “VEXUNGA KCD4 6pcs” | ~$8 |
| 1 | 60 mm 12 V fan **4-pack** | search “60mm 12V fan 4 pack” | ~$12 |
| 1 | 60 mm fan filter/grill **4-pack** | search “60mm fan filter 4 pack” | ~$10 |
| 1 | PP30 Powerpole **10-set** | Valley / Amazon PP30-10 | ~$13 |
| 1 | 12 AWG red/black zip **≥10 ft** | 2× ≤4 ft jumpers + slack | ~$8 |
| | | **Subtotal** | **~$148** |

DT kit covers SOL panel + cable ends (8 channels + spare). One M12 pack = 4 panel + 2 RS-485 field cables.

---

## Total (lean)

| Cart | Ext |
| --- | ---: |
| Bravo | $69.20 |
| Powerwerx PanelPole | $79.96 |
| Amazon bundle | ~$148 |
| **Sum** | **~$297** |

(+ Al plate scrap)

Was ~$530. Cuts listed in [`COST_OPTIONS.md`](COST_OPTIONS.md).

---

## Checklist

- [ ] Bravo: RS-15×2, LRS-200×2  
- [ ] Powerwerx: PanelPole2 ×4  
- [ ] Amazon: EG STARTS×2, M12×1, C14, **4× 6 ft C13–C14**, DT 10-set, KCD4×6, fans×4, filters, PP30-10, 12 AWG ≥10 ft  
- [ ] DIY two Powerpole jumpers **≤ 4 ft** each  
- [ ] Al plates · `F9` seated  
- [ ] Verify C14 pack cutout (snap vs flange) + EG STARTS ring Ø before CAD freeze  

Add-backs (optional): HangTon USB, Newark WRG32, OEM AT flange — see COST_OPTIONS.
