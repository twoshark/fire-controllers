# Shopping list — 4 enclosures

Build kit: **sign-input**, **mp-input**, **sign-output**, **mp-output**.

Prices USD as of 2026-07-18. Re-check at checkout.

---

## Per box

| Box | AC | RS-485 | SOL | USB | PSU |
| --- | --- | --- | --- | --- | --- |
| sign-input | C14 | M12-5 | — | HangTon | IRM-15-12 |
| mp-input | C14 | M12-5 | — | HangTon | IRM-15-12 |
| sign-output | C14 | M12-5 | M12-8 | HangTon | LRS-200-12 |
| mp-output | C14 | M12-5 | M12-5 | HangTon | LRS-150-12 |

| Box | 12 V load | AC @115 V |
| --- | --- | --- |
| sign-output | ≈10.3 A | ~4 A |
| mp-output | ≈6.2 A | ~2.8 A |
| inputs | &lt;0.8 A | ~0.2 A |

---

## Pin maps

### RS-485 M12-5

| Pin | Signal |
| ---: | --- |
| 1 | TX+ |
| 2 | TX− |
| 3 | RX+ |
| 4 | RX− |
| 5 | GND |
| Shell | SHIELD → chassis PE |

Field cable: male–male, TX↔RX crossover.

### Sign SOL M12-8

| Pin | Net |
| ---: | --- |
| 1–2 | `+12V` paralleled |
| 3–7 | OUT0..4 |
| 8 | NC |

### Mp SOL M12-5

| Pin | Net |
| ---: | --- |
| 1–2 | `+12V` paralleled |
| 3–5 | OUT0..2 |

---

## Cart 1 — Bravo Electro

| Qty | MPN | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | IRM-15-12 | $8.60 | $17.20 | [bravoelectro.com/irm-15-12](https://www.bravoelectro.com/irm-15-12.html) |
| 1 | LRS-200-12 | $26.00 | $26.00 | [bravoelectro.com/lrs-200-12](https://www.bravoelectro.com/lrs-200-12.html) |
| 1 | LRS-150-12 | $19.80 | $19.80 | [bravoelectro.com/lrs-150-12](https://www.bravoelectro.com/lrs-150-12.html) |
| | | **Subtotal** | **$63.00** | |

LRS: selector **115 V**, trim **12.0 V**.

---

## Cart 2 — Newark

| Qty | MPN | Role | Unit | Ext | Link |
| ---: | --- | --- | ---: | ---: | --- |
| 2 | WRG32F2FBBNN | Output POWER rocker | $2.80 | $5.60 | [05M1943](https://www.newark.com/zf-electronics/wrg32f2fbbnn/switch-rocker-dpst-16a-250vac/dp/05M1943) |
| | | | **Subtotal** | **$5.60** | |

---

## Cart 3 — Adafruit

| Qty | SKU | Role | Unit | Ext | Link |
| ---: | --- | --- | ---: | ---: | --- |
| 4 | 559 | RESET red 16 mm | $4.95 | $19.80 | [559](https://www.adafruit.com/product/559) |
| 4 | 481 | BOOT blue 16 mm | $4.95 | $19.80 | [481](https://www.adafruit.com/product/481) |
| | | | **Subtotal** | **$39.60** | |

Arcade POWER / CH / ALL FIRE buttons: supplied by builder.

---

## Cart 4 — Amazon connectors

| Qty | Item | ASIN | Pack $ | Ext |
| ---: | --- | --- | ---: | ---: |
| 2 | M12-5 4-set (RS-485×4 + mp SOL + spares) | [B0CFFX6JW4](https://www.amazon.com/dp/B0CFFX6JW4) | $23.79 | $47.58 |
| 1 | M12-8 4-set (sign SOL + spares) | [B0CFFY9X93](https://www.amazon.com/gp/product/B0CFFY9X93) | $36.49 | $36.49 |
| 1 | C14 inlet pack of 4 | [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) | $8.19 | $8.19 |
| | | | **Subtotal** | **$92.26** |

M12 listings must be screw-terminal / field-wireable. Panel half = female.

---

## Cart 5 — Amazon USB + cords

| Qty | Item | ASIN | Unit | Ext |
| ---: | --- | --- | ---: | ---: |
| 2 | HangTon USB-C F–F 2-pack | [B0CDPR3BJS](https://www.amazon.com/dp/B0CDPR3BJS) | $14.44 | $28.88 |
| 1 | USB-C M–M 6″ 4-pack | [B0CJY9PRNY](https://www.amazon.com/dp/B0CJY9PRNY) | $8.98 | $8.98 |
| 4 | NEMA 5-15P → C13 6 ft | [B000067SLV](https://www.amazon.com/dp/B000067SLV) | $8.83 | $35.32 |
| | | | **Subtotal** | **$73.18** |

---

## Total

| Cart | Ext |
| --- | ---: |
| Bravo | $63.00 |
| Newark | $5.60 |
| Adafruit | $39.60 |
| Amazon connectors | $92.26 |
| Amazon USB + cords | $73.18 |
| **Sum** | **$273.64** |

Wire, ferrules, gasket, heat-set inserts: as needed for print/assembly.

---

## Checklist

- [ ] Bravo PSUs  
- [ ] Newark WRG×2  
- [ ] Adafruit 559×4, 481×4  
- [ ] Amazon M12-5×2, M12-8×1, C14×4  
- [ ] Amazon HangTon×2, USB jumpers, C13×4  
- [ ] Output PCB `F9` 25 A blade seated before load  

[`WIRING.md`](WIRING.md) · [`SIGN_OUTPUT.md`](SIGN_OUTPUT.md) · [`MP_OUTPUT.md`](MP_OUTPUT.md)
