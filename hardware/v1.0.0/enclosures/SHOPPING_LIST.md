# Shopping list — 6 enclosures (revised · ≤ $300)

**sign-input**, **mp-input**, **sign-output**, **sign-output-power**, **mp-output**, **mp-output-power**.

Prices USD 2026-07-18. See [`COST_OPTIONS.md`](COST_OPTIONS.md) · [`SEALING.md`](SEALING.md) · [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) · [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md).

No external RESET/BOOT/USB. No PanelPole. No 12 V rocker on output boxes (kill via AC POWER on power boxes).

---

## Cable assumptions

| Run | Qty | Length | Notes |
| --- | ---: | ---: | --- |
| Mains C13→C14 | **4** | **6 ft** | Both inputs + both power boxes |
| 12 V DTP jumper | **2** | **≤ 4 ft** | Each power box ↔ its output box · 12 AWG |

---

## Per box

| Box | AC | POWER | 12 V | RS-485 | SOL | PSU |
| --- | --- | --- | --- | --- | --- | --- |
| sign-input | C14 | KCD4+boot | — | M12-5 | — | RS-15-12 |
| mp-input | C14 | KCD4+boot | — | M12-5 | — | RS-15-12 |
| sign-output | — | — (kill on power box) | DTP IN | M12-5 | 5× DT | — |
| mp-output | — | — (kill on power box) | DTP IN | M12-5 | 3× DT | — |
| sign-output-power | C14 | KCD4+boot | DTP OUT | — | — | LRS-200-12 |
| mp-output-power | C14 | KCD4+boot | DTP OUT | — | — | LRS-200-12 |

12 V mate faces: DTP pocket **H = 40 mm**.

---

## Pin maps

### RS-485 M12-5

| Pin | Signal |
| ---: | --- |
| 1–4 | TX+/TX−/RX+/RX− |
| 5 | GND |
| Shell | SHIELD |

### 12 V — DTP 2-pin

| Pin | Net |
| ---: | --- |
| 1 | `+12V` |
| 2 | `GND` |

Panel: **DTP04-2P** (receptacle) in printed pocket. Field jumper: **DTP06-2S** both ends · 12 AWG · ≤4 ft.

### SOL — DT 2-pin each channel

| Pin | Net |
| ---: | --- |
| 1 | `+12V` (`J6`, star feed) |
| 2 | `OUTn` |

---

## Cart 1 — Bravo Electro · $69.20

| Qty | MPN | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | RS-15-12 | $8.60 | $17.20 | [bravoelectro.com](https://www.bravoelectro.com/rs-15-12.html) |
| 2 | LRS-200-12 | $26.00 | $52.00 | [bravoelectro.com](https://www.bravoelectro.com/lrs-200-12.html) |

LRS: **115 V**, trim **12.0 V**. Mech: RS-15 **62.5×51×28** · LRS **215×115×30**.

---

## Cart 2 — Amazon

| Qty | Item | ASIN / search | Ext |
| ---: | --- | --- | ---: |
| 2 | EG STARTS triangle LED ×5 | [B07R565HM6](https://www.amazon.com/dp/B07R565HM6) | $23.98 |
| 1 | M12-5 4-set | [B0CFFX6JW4](https://www.amazon.com/dp/B0CFFX6JW4) | $23.79 |
| 1 | M12 dust caps (match panel gender) | search “M12 dust cap panel” 5–10 pc | ~$8 |
| 1 | C14 inlet ×4 | [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) | $8.19 |
| 1 | C14 rubber inlet covers ×4 | search “IEC C14 inlet waterproof cover” | ~$8 |
| 4 | IEC C13–C14 cord **6 ft** | search “C13 C14 6ft” | ~$16 |
| 1 | JRready **DT** 2-pin **10 sets** (SOL) | [B0BKRRGRQY](https://www.amazon.com/dp/B0BKRRGRQY) | ~$25 |
| 1 | JRready / DX **DTP** 2-pin **10 sets** 25 A (12 V) | search “DTP 2 pin connector kit 10 sets 25A” | ~$28 |
| 1 | KCD4 DPST rocker **6-pack** (use 4) · 30×22 | search “VEXUNGA KCD4 6pcs” | ~$8 |
| 1 | Silicone rocker boots (fit 30×22) | search “KCD4 rocker switch cover boot” | ~$8 |
| 1 | 60 mm 12 V fan **4-pack** | search “60mm 12V fan 4 pack” | ~$12 |
| 1 | 60 mm fan filter/grill **4-pack** | search “60mm fan filter 4 pack” | ~$10 |
| 1 | 12 AWG red/black zip **≥10 ft** | 2× ≤4 ft DTP jumpers + internal | ~$8 |
| 1 | Clear PC sheet scrap + foam gasket strip | LED lenses ×6 · lid gasket material | ~$8 |
| 1 | M3×5.7 heat-set inserts **≥50** | Ruthex / CNC Kitchen | ~$12 |
| 1 | M4×6–8 heat-set inserts **≥12** | for LRS plates | ~$6 |
| | | **Subtotal** | **~$213** |

DTP kit: 4 panel + 4 cable ends + spare; use included cavity plugs. DT kit: 8 SOL + spare + plugs. No Powerwerx cart.

---

## Total

| Cart | Ext |
| --- | ---: |
| Bravo | $69.20 |
| Amazon | ~$213 |
| **Sum** | **~$282** |

(+ Al plate scrap · Deutsch crimp tool if not owned)

Was ~$530 → lean PanelPole ~$297 → **this ~$256** with better sealing.

---

## Checklist

- [ ] Bravo: RS-15×2, LRS-200×2  
- [ ] Amazon: EG STARTS×2, M12+caps, C14+covers, 4×6 ft cords, DT 10-set, **DTP 10-set**, KCD4+boots, fans/filters, 12 AWG, **M3/M4 inserts**  
- [ ] DIY two DTP jumpers **≤ 4 ft** (DTP06-2S both ends)  
- [ ] Al plates · `F9` seated  
- [ ] Verify C14 cutout style + EG STARTS ring Ø before CAD freeze  
