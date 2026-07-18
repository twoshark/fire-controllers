# Shopping list — 4 enclosures + 2 HLG

**Printed:** sign-input · mp-input · sign-output · mp-output  
**PSU:** 2× **HLG-240H-12** — [`POWER_OTS.md`](POWER_OTS.md).

Prices USD 2026-07-18.

---

## Cables

| Run | Qty | Length |
| --- | ---: | ---: |
| C13→C14 (inputs) | 2 | **6 ft** |
| SJTW to each HLG | 2 | **6 ft** |
| HLG DC → output DTP | 2 | **≤ 4 ft** · 12 AWG |

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

## Cart 2 — Amazon · ~$199

| Qty | Item | ASIN / search | Ext |
| ---: | --- | --- | ---: |
| 2 | EG STARTS triangle LED ×5 | [B07R565HM6](https://www.amazon.com/dp/B07R565HM6) | $23.98 |
| 1 | M12-5 4-set | [B0CFFX6JW4](https://www.amazon.com/dp/B0CFFX6JW4) | $23.79 |
| 1 | M12 dust caps | search “M12 dust cap panel” | ~$8 |
| 1 | C14 inlet ×2–4 | [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) | ~$8 |
| 1 | C14 covers ×2 | search “IEC C14 inlet cover” | ~$5 |
| 2 | C13–C14 cord **6 ft** | search “C13 C14 6ft” | ~$8 |
| 2 | Outdoor SJTW **6 ft** 16/3 | search “SJTW 16/3 outdoor” | ~$16 |
| 1 | Waterproof splice / Wago Gelbox | HLG AC | ~$12 |
| 1 | DT 2-pin **10 sets** | [B0BKRRGRQY](https://www.amazon.com/dp/B0BKRRGRQY) | ~$25 |
| 1 | DTP 2-pin **10 sets** 25 A | search “DTP 2 pin kit 10 sets” | ~$28 |
| 1 | KCD4 DPST 6-pack (use 2) | search “VEXUNGA KCD4 6pcs” | ~$8 |
| 1 | Silicone rocker boots | search “KCD4 rocker boot” | ~$8 |
| 1 | 12 AWG red/black zip ≥10 ft | HLG→DTP + internals | ~$8 |
| 1 | PC lens + foam gasket stock | LED ×4 · lids | ~$8 |
| 1 | M2×4–5 heat-set ≥20 | PCB mounts (100 mil holes) | ~$8 |
| 1 | M3×5.7 heat-set ≥24 | lids + RS-15 | ~$8 |
| 1 | M2×6–8 pan screws ≥20 | through PCB | ~$4 |

---

## Total · ~$359

| Cart | Ext |
| --- | ---: |
| Bravo | $159.80 |
| Amazon | ~$199 |
| **Sum** | **~$359** |

---

## Checklist

- [ ] Bravo: RS-15×2, HLG-240H-12×2  
- [ ] Amazon: EG STARTS, M12+caps, C14, cords, SJTW+splices, DT, DTP, KCD4+boots, 12 AWG, **M2+M3 inserts**  

- [ ] Crimp DTP06-2S on each HLG DC pigtail (≤4 ft)  
- [ ] Mount HLGs shaded · FG earthed  
- [ ] `F9` seated  
