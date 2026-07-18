# Shopping list — 6 enclosures

**sign-input**, **mp-input**, **sign-output**, **sign-output-power**, **mp-output**, **mp-output-power**.

Prices USD 2026-07-18. Ratings: [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md).

---

## Per box

| Box | AC | 12 V link | RS-485 | SOL | USB | PSU |
| --- | --- | --- | --- | --- | --- | --- |
| sign-input | C14 | — | M12-5 | — | HangTon | RS-15-12 |
| mp-input | C14 | — | M12-5 | — | HangTon | RS-15-12 |
| sign-output | — | PanelPole2 IN | M12-5 | 5× AT 2-pin | HangTon | — |
| mp-output | — | PanelPole2 IN | M12-5 | 3× AT 2-pin | HangTon | — |
| sign-output-power | C14 | PanelPole2 OUT | — | — | — | LRS-200-12 |
| mp-output-power | C14 | PanelPole2 OUT | — | — | — | LRS-200-12 |

---

## Pin maps

### RS-485 M12-5 (signal only · 4 A · ≤22 AWG)

| Pin | Signal |
| ---: | --- |
| 1 | TX+ |
| 2 | TX− |
| 3 | RX+ |
| 4 | RX− |
| 5 | GND |
| Shell | SHIELD → chassis PE |

Field cable: male–male, TX↔RX crossover. **Not** for solenoid power.

### 12 V link — Anderson Powerpole (PanelPole2)

| Pole | Net |
| --- | --- |
| Red | `+12V` |
| Black | `GND` |

Panel: **PanelPole2** (PP30 contacts, 12–14 AWG, 30 A). Field: **EX-12-5** (12 AWG). Do not use 10 AWG EX-10 with PP30 contacts.

### SOL — one AT 2-pin per channel (13 A · 16–18 AWG)

| Pin | Net |
| ---: | --- |
| 1 | `+12V` (`J6`) |
| 2 | `OUTn` |

| Box | Panel receptacles | Plugs (cable) | `OUTn` |
| --- | --- | --- | --- |
| sign-output | 5× AT04-2P-KIT01 | 5× AT06-2S-KIT01 | `J5a.1`..`J5a.4`, `J5b.1` |
| mp-output | 3× AT04-2P-KIT01 | 3× AT06-2S-KIT01 | `J5a.1`..`J5a.3` |

Field: 18 AWG zip · pin1 → solenoid → pin2. Crimp size-16 contacts (AT tool).

---

## Cart 1 — Bravo Electro

| Qty | MPN | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | RS-15-12 | $8.60 | $17.20 | [bravoelectro.com/rs-15-12](https://www.bravoelectro.com/rs-15-12.html) |
| 2 | LRS-200-12 | $26.00 | $52.00 | [bravoelectro.com/lrs-200-12](https://www.bravoelectro.com/lrs-200-12.html) |
| | | **Subtotal** | **$69.20** | |

LRS: selector **115 V**, trim **12.0 V**.

---

## Cart 2 — Newark

| Qty | MPN | Role | Unit | Ext | Link |
| ---: | --- | --- | ---: | ---: | --- |
| 2 | WRG32F2FBBNN | POWER (0.250" QC tabs) | $2.80 | $5.60 | [05M1943](https://www.newark.com/zf-electronics/wrg32f2fbbnn/switch-rocker-dpst-16a-250vac/dp/05M1943) |
| 2 | 09250-F/60 | 60 mm intake filter | $1.68 | $3.36 | [17B6480](https://www.newark.com/qualtek/09250-f-60/fan-filter-assembly-plastic-60mm/dp/17B6480) |
| | | | **Subtotal** | **$8.96** | |

---

## Cart 3 — Adafruit

| Qty | SKU | Role | Unit | Ext | Link |
| ---: | --- | --- | ---: | ---: | --- |
| 4 | 559 | RESET | $4.95 | $19.80 | [559](https://www.adafruit.com/product/559) |
| 4 | 481 | BOOT | $4.95 | $19.80 | [481](https://www.adafruit.com/product/481) |
| | | | **Subtotal** | **$39.60** | |

Arcade POWER / CH / ALL: builder-supplied (**AC-rated** for POWER).

---

## Cart 4 — Amazon connectors

| Qty | Item | ASIN | Pack $ | Ext |
| ---: | --- | --- | ---: | ---: |
| 2 | M12-5 4-set (RS-485 only) | [B0CFFX6JW4](https://www.amazon.com/dp/B0CFFX6JW4) | $23.79 | $47.58 |
| 1 | C14 inlet pack of 4 | [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) | $8.19 | $8.19 |
| 4 | Powerwerx PanelPole2 | [B097QDKJJ2](https://www.amazon.com/dp/B097QDKJJ2) | $27.78 | $111.12 |
| | | | **Subtotal** | **$166.89** |

M12-5: **4 A**/pin, signal use, ≤22 AWG. Listing is cable M/F sets — confirm panel-nut female + rear termination before relying on it; 4 panel + field ends fit in 2 packs with spare.  
PanelPole2: Ø**1-1/8"**, PP30 (12–14 AWG), weather cover.

---

## Cart 5 — Amazon USB + cords

| Qty | Item | ASIN | Unit | Ext |
| ---: | --- | --- | ---: | ---: |
| 2 | HangTon USB-C F–F 2-pack | [B0CDPR3BJS](https://www.amazon.com/dp/B0CDPR3BJS) | $14.44 | $28.88 |
| 1 | USB-C M–M 6″ 4-pack | [B0CJY9PRNY](https://www.amazon.com/dp/B0CJY9PRNY) | $8.98 | $8.98 |
| 4 | NEMA 5-15P → C13 6 ft | [B000067SLV](https://www.amazon.com/dp/B000067SLV) | $8.83 | $35.32 |
| | | | **Subtotal** | **$73.18** |

---

## Cart 6 — DigiKey / Sager (fans + SOL AT kits)

| Qty | MPN | Role | Unit | Ext | Link |
| ---: | --- | --- | ---: | ---: | --- |
| 4 | MF60151V2-1000U-A99 | 60 mm fan | $7.63 | $30.52 | [DigiKey](https://www.digikey.com/en/products/detail/sunon-fans/MF60151V2-1000U-A99/2770973) |
| 2 | 8147 | Exhaust guard | $0.54 | $1.08 | [CR221-ND](https://www.digikey.com/en/products/detail/qualtek/8147/312840) |
| 8 | AT04-2P-KIT01 | SOL panel receptacle + contacts | $4.99 | $39.92 | [Amphenol](https://www.amphenol-sine.com/AT04-2P-KIT01-2-Way-Pin-Receptacle-Wedge-and-Contacts-Kit_p_5768.html) |
| 8 | AT06-2S-KIT01 | SOL cable plug + contacts | $4.99 | $39.92 | [Amphenol](https://www.amphenol-sine.com/AT06-2S-KIT01-2-Socket-Plug-Wedge-and-Contacts-Kit_p_5769.html) |
| | | | **Subtotal** | **$111.44** | |

AT kit unit $ from DigiKey listing class (~$5); confirm cart before order. Free-hanging housings — capture in printed panel pocket / flange (IP67 when mated).

---

## Cart 7 — Powerwerx (12 V field cables)

| Qty | MPN | Role | Unit | Ext | Link |
| ---: | --- | --- | ---: | ---: | --- |
| 2 | EX-12-5 | 12 AWG Powerpole extension 5 ft | $12.99 | $25.98 | [powerwerx.com](https://powerwerx.com/powerpole-connector-extension-cable) |
| | | | **Subtotal** | **$25.98** | |

---

## Total

| Cart | Ext |
| --- | ---: |
| Bravo | $69.20 |
| Newark | $8.96 |
| Adafruit | $39.60 |
| Amazon connectors | $166.89 |
| Amazon USB + cords | $73.18 |
| DigiKey / Sager | $111.44 |
| Powerwerx cables | $25.98 |
| **Sum** | **$495.25** |

(+ aluminum plate · AT/Powerpole crimp tools if not owned)

---

## Checklist

- [ ] Bravo: 2× RS-15-12, 2× LRS-200-12  
- [ ] Newark: WRG×2, 09250-F/60 ×2  
- [ ] DigiKey: fans×4, 8147×2, AT04-2P-KIT01×8, AT06-2S-KIT01×8  
- [ ] Adafruit: 559×4, 481×4  
- [ ] Amazon: M12-5×2, C14×4, PanelPole2×4, HangTon×2, USB×1, C13×4  
- [ ] Powerwerx: EX-12-5 ×2  
- [ ] 2× Al plates 115 × 215 × 3 mm  
- [ ] Output PCB `F9` seated  
- [ ] AT + Powerpole crimp tools  

Per-box docs in each folder.
