# Enclosure parts BOM

Order sheet with pack quantities: [`SHOPPING_LIST.md`](SHOPPING_LIST.md).  
Prices USD as of 2026-07-18.

---

## Catalog

### Power supplies

| Box | MPN | Buy | Qty‑1 |
| --- | --- | ---: |
| Each input | IRM-15-12 | [Bravo](https://www.bravoelectro.com/irm-15-12.html) | $8.60 |
| sign-output | LRS-200-12 | [Bravo](https://www.bravoelectro.com/lrs-200-12.html) | $26.00 |
| mp-output | LRS-150-12 | [Bravo](https://www.bravoelectro.com/lrs-150-12.html) | $19.80 |

LRS: selector **115 V**, trim **12.0 V**.

### Panel connectors

| Use | Part | Pack $ |
| --- | --- | ---: |
| AC IN | C14 [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) ×4 | $8.19 |
| RS-485 + mp SOL | M12-5 [B0CFFX6JW4](https://www.amazon.com/dp/B0CFFX6JW4) 4-set | $23.79 |
| Sign SOL | M12-8 [B0CFFY9X93](https://www.amazon.com/gp/product/B0CFFY9X93) 4-set | $36.49 |
| USB-C bulkhead | HangTon [B0CDPR3BJS](https://www.amazon.com/dp/B0CDPR3BJS) 2-pack | $14.44 |
| USB-C jumper | [B0CJY9PRNY](https://www.amazon.com/dp/B0CJY9PRNY) 4-pack | $8.98 |

M12: screw-terminal, panel female. IEC C14: panel male.

### Controls

| Item | Buy | Qty‑1 |
| --- | --- | ---: |
| RESET | Adafruit [559](https://www.adafruit.com/product/559) | $4.95 |
| BOOT | Adafruit [481](https://www.adafruit.com/product/481) | $4.95 |
| POWER (outputs) | WRG32F2FBBNN · Newark [05M1943](https://www.newark.com/zf-electronics/wrg32f2fbbnn/switch-rocker-dpst-16a-250vac/dp/05M1943) | $2.80 |
| POWER / CH / ALL (inputs) | Builder arcade parts | — |

### Protection (on PCB / PSU)

| Layer | Part |
| --- | --- |
| Output 12 V feed | `F9` 25 A ATO |
| Output channels | PTC `F1`..`F8` |
| AC primary | IRM / LRS internal fuse |

### Wire

| Net | Gauge |
| --- | --- |
| AC L/N/PE | 18 AWG 300 V |
| Solenoid +12 V / returns | 18 AWG |
| RS-485, NRST, BOOT, USB | 24–28 AWG |

---

## Per-box estimate (ex filament / arcade / PCB)

| Box | ≈$ |
| --- | ---: |
| sign-input / mp-input | 42 |
| sign-output | 64 |
| mp-output | 56 |

Kit total **$273.64** — [`SHOPPING_LIST.md`](SHOPPING_LIST.md).

Pin maps: [`WIRING.md`](WIRING.md).
