# Shopping list — 4 enclosures + 2 HLG

**Printed:** sign-input · mp-input · sign-output · mp-output  
**PSU:** 2× **HLG-240H-12** — [`POWER_OTS.md`](POWER_OTS.md).  
Ratings / seal: [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) · [`SEALING.md`](SEALING.md).

Prices USD 2026-07-18.

---

## Cables

| Run | Qty | Length |
| --- | ---: | ---: |
| C13→C14 (inputs) | 2 | **6 ft** |
| SJTW to each HLG | 2 | **6 ft** · 16/3 |
| HLG DC → output DTP | 2 | **≤ 4 ft** · 12 AWG |
| RS-485 interconnect | 2 | **Belden 9842** (or equiv 120 Ω 2-pair STP) · length per install · terminate in M12-5 |

---

## Per unit

| Unit | AC | POWER | 12 V | RS-485 | SOL | PSU |
| --- | --- | --- | --- | --- | --- | --- |
| sign-input | C14 | KCD4+boot | — | M12-5 | — | RS-15-12 |
| mp-input | C14 | KCD4+boot | — | M12-5 | — | RS-15-12 |
| sign-output | — | — | DTP IN | M12-5 | 5× DT | ← HLG |
| mp-output | — | — | DTP IN | M12-5 | 3× DT | ← HLG |
| HLG ×2 | SJTW splice | unplug to kill | DTP on pigtail | — | — | HLG-240H-12 |

---

## Pin maps

### 12 V — DTP 2-pin

| Pin | Net |
| ---: | --- |
| 1 | `+12V` |
| 2 | `GND` |

Output panel: **DTP04-2P**. HLG DC: **DTP06-2S**.

### RS-485 M12-5

| Pin | Signal |
| ---: | --- |
| 1–4 | TX+/TX−/RX+/RX− |
| 5 | GND |
| Shell | SHIELD |

### SOL — DT 2-pin

| Pin | Net |
| ---: | --- |
| 1 | `+12V` (`J6`, star) |
| 2 | `OUTn` |

---

## Cart 1 — Bravo · $159.80

| Qty | MPN | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | RS-15-12 | $8.60 | $17.20 | [bravoelectro.com](https://www.bravoelectro.com/rs-15-12.html) |
| 2 | HLG-240H-12 | $71.30 | $142.60 | [bravoelectro.com](https://www.bravoelectro.com/hlg-240h-12.html) |

Budget: 2× [HLG-185H-12](https://www.bravoelectro.com/hlg-185h-12.html) @ $57.40 → Bravo $131.80.

---

## Cart 2 — Amazon / Digi-Key · ~$230

| Qty | Item | ASIN / search | Ext |
| ---: | --- | --- | ---: |
| 2 | EG STARTS triangle LED ×5 | [B07R565HM6](https://www.amazon.com/dp/B07R565HM6) | $23.98 |
| 4 | M12-5 panel receptacles | confirm A-code 5-pin | ~$20 |
| 4 | M12-5 field plugs (cable ends) | for 2× RS-485 cables | ~$16 |
| 1 | M12 dust caps (≥4) | “M12 dust cap panel” | ~$8 |
| 50 ft | **Belden 9842** (or equiv) | Digi-Key / Newark | ~$45 |
| 1 | C14 inlet ×2–4 | [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) | ~$8 |
| 1 | C14 covers ×2 | “IEC C14 inlet cover” | ~$5 |
| 2 | C13–C14 cord **6 ft** | “C13 C14 6ft” | ~$8 |
| 2 | Outdoor SJTW **6 ft** 16/3 | “SJTW 16/3 outdoor” | ~$16 |
| 1 | Waterproof splice / Wago Gelbox | HLG AC | ~$12 |
| 8 | DT04-2P + DT06-2S sets | TE preferred; kit OK if complete | ~$28 |
| 4 | DTP04-2P + DTP06-2S sets | TE preferred · **25 A** | ~$30 |
| 1 | DT/DTP wedges + cavity plugs | match housing | ~$10 |
| 1 | KCD4 **DPST** 125 VAC (use 2) | verify marking | ~$8 |
| 1 | Silicone rocker boots | “KCD4 rocker boot” | ~$8 |
| 1 | 12 AWG red/black ≥10 ft | HLG→DTP + `J6` stub | ~$8 |
| 1 | 18 AWG red/black ≥20 ft | SOL channels | ~$6 |
| 1 | Adhesive heatshrink / gel | HLG DC re-term | ~$6 |
| 2 | ATO **20 A** blade | board `F9` | ~$3 |
| 1 | PC lens + foam gasket stock | LED ×4 · lids | ~$8 |
| 1 | M2×4–5 heat-set ≥20 | PCB mounts | ~$8 |
| 1 | M3×5.7 heat-set ≥24 | lids + RS-15 | ~$8 |
| 1 | M2×6–8 pan · M3×10–12 pan | screws | ~$6 |

TE flange upgrade: [`COST_OPTIONS.md`](COST_OPTIONS.md) **C**.

---

## Total · ~$390

| Cart | Ext |
| --- | ---: |
| Bravo | $159.80 |
| Amazon / Digi-Key | ~$230 |
| **Sum** | **~$390** |

---

## Checklist

- [ ] Bravo: RS-15×2, HLG-240H-12×2  
- [ ] Belden 9842 + M12 panel×4 + field plugs×4 + dust caps  
- [ ] DT/DTP with contacts, wedges, cavity plugs (TE or complete kits)  
- [ ] C14+covers, cords, SJTW+splices, KCD4+boots, 12/18 AWG, **M2+M3 inserts**  
- [ ] Crimp DTP06-2S on each HLG DC · **seal re-term** · ≤4 ft  
- [ ] Mount HLGs shaded · FG earthed  
- [ ] `F9` **20 A** ATO seated  
- [ ] Measure SOL current @ 12 V before loading all channels  
