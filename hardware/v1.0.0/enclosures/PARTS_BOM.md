# Enclosure parts BOM

Order: [`SHOPPING_LIST.md`](SHOPPING_LIST.md). Ratings: [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md).

Prices USD 2026-07-18.

---

## Power supplies

| Use | MPN | Size | Buy | Qty‑1 |
| --- | --- | --- | ---: |
| Input box | RS-15-12 | 62.5 × 51 × 28 mm | [Bravo](https://www.bravoelectro.com/rs-15-12.html) | $8.60 |
| Output-power box | LRS-200-12 | 215 × 115 × 30 mm | [Bravo](https://www.bravoelectro.com/lrs-200-12.html) | $26.00 |

LRS: selector **115 V**, trim **12.0 V**.

## Connectors

| Use | Part | Rating | Buy |
| --- | --- | --- | --- |
| AC IN | C14 | 10–15 A | [B07PVP7XB7](https://www.amazon.com/dp/B07PVP7XB7) |
| RS-485 | M12-5 | 4 A · ≤22 AWG · signal | [B0CFFX6JW4](https://www.amazon.com/dp/B0CFFX6JW4) |
| 12 V link | PanelPole2 | 30 A · 12–14 AWG | [B097QDKJJ2](https://www.amazon.com/dp/B097QDKJJ2) |
| 12 V field | EX-12-5 | 12 AWG | [Powerwerx](https://powerwerx.com/powerpole-connector-extension-cable) |
| SOL per CH | AT04-2P-KIT01 + AT06-2S-KIT01 | 13 A · 16–18 AWG | Amphenol / DigiKey |
| USB-C | HangTon | debug | [B0CDPR3BJS](https://www.amazon.com/dp/B0CDPR3BJS) |

## Controls

| Item | Buy | Qty‑1 |
| --- | --- | ---: |
| RESET | Adafruit [559](https://www.adafruit.com/product/559) | $4.95 |
| BOOT | Adafruit [481](https://www.adafruit.com/product/481) | $4.95 |
| POWER | WRG32F2FBBNN · 16 A AC · 0.250" QC · [05M1943](https://www.newark.com/zf-electronics/wrg32f2fbbnn/switch-rocker-dpst-16a-250vac/dp/05M1943) | $2.80 |

## Cooling (output-power)

| Item | MPN | Qty‑1 |
| --- | --- | ---: |
| Fan 60×60×15 12 V | MF60151V2-1000U-A99 | $7.63 |
| Intake filter | 09250-F/60 | $1.68 |
| Exhaust guard | 8147 | $0.54 |
| Spreader | 6061 Al 115 × 215 × 3 mm | — |

## Protection

| Layer | Part |
| --- | --- |
| Output 12 V | `F9` 25 A ATO |
| Channels | PTC `F1`..`F8` 2 A hold |
| AC | RS-15 / LRS internal |

## Wire

| Net | Gauge | Why |
| --- | --- | --- |
| AC L/N/PE | 18 AWG | C14 / WRG32 QC |
| 12 V link | **12 AWG** | matches PP30 + EX-12-5 |
| SOL | **18 AWG** | matches AT 16–18 AWG · 2 A |
| RS-485 | 22–24 AWG | M12 max ~22 AWG |
