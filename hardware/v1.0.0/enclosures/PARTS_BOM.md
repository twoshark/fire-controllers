# Enclosure parts BOM — budget rebuild (≤ $50 / box)

**Cost target:** ≤ **$50 USD** per box excluding filament, arcade buttons, and PCBs.  
**Inter-box cable plant** (Belden run, spare field plugs bought in bulk) is **not** in the per-box $50.

Prices captured **2026-07-15**. Amazon ASINs rotate — search the quoted title if a link 404s. Prefer Prime / ≤2-week ship.

---

## Why the old BOM was ~$190–$295

| Old line | Old $ | Problem |
| --- | ---: | --- |
| Buccaneer 400 AC in+out+contacts | ~70–80 | Correct IP68, wrong price tier for this project |
| Phoenix `1513758` M12 | ~32 | Industrial pigtail tax |
| `MUSBR-M5C1-M0` | 16.66 | DigiKey harsh-environment USB |
| `PV1F640SS` ×2 | 15.72 | DigiKey anti-vandal |
| `MP0045` POWER | ~38 | Overkill stainless latch |
| Amphenol AT PanelMate ×5 | ~25+ | Fine parts, add up fast |
| Pololu 2482 | 7.95 | Nice, not necessary |

Those choices were high-confidence industrial. They are **incompatible with a $50 box**.

---

## Locked budget platform (all boxes)

| Role | Choice | Why | Rejected |
| --- | --- | --- | --- |
| AC / glow / RS-485 / solenoids | **GX16** via **LCSC** (~$2/set) | Cheap; 3P **7 A**, 6/8P **~5 A/pin** | Buccaneer / Phoenix M12 / AT |
| Solenoid `+12V` | **2 pins paralleled** on multipin | 5×2 A needs &gt;5 A on feed | Single pin (overload) |
| USB-C panel | **HangTon** F–F bulkhead + cap + short M–M jumper | Lid-closed DFU; **IP65** with cap; ~$8.50 | DigiKey MUSBR ~$17 |
| RESET / BOOT | Adafruit **559** red / **481** blue | **$4.95**; gasketed Ø16 | DigiKey PV1F; SKU 558=white, 557≠button |
| POWER (output) | DigiKey **WRG32F2FBBNN** | **$2.88**, 16 A AC | Bulgin MP0045; not face-IP67 |
| POWER (input) | Your arcade | $0 | — |
| Glow relay | **G5LE-14-DC12** | DigiKey/Mouser ~$1.25–1.73; 10 A @ 120 VAC | Pololu 2482 |
| Input PSU | **IRM-15-12** | Bravo **$8.60**; **1.25 A** | — |
| sign-output PSU | **LRS-200-12** | **17 A**; Bravo/DigiKey **$26** | LRS-150 too tight |
| mp-output PSU | **LRS-150-12** | **12.5 A**; Bravo **$19.80** | LRS-100 too tight |
| Sign AC fuse | **BK/GMC-5-R** (5 A TD) | LRS-200 draws **4 A** @115 V full | 3.15 A undersized |
| Mp AC fuse | **BK/GMC-3.15-R** | LRS-150 draws **2.8 A** @115 V full | 2.5 A too tight |

**Order sheet:** [`SHOPPING_LIST.md`](SHOPPING_LIST.md) (triple-checked).

**IP honesty:** GX16 often IP55–IP65 mated — hose-test. HangTon USB **IP65** with cap. Rocker not face-IP67.

---

## A. Shared catalog (buy links + qty‑1)

### A1. Power supplies

| Box | MPN | Buy | Qty‑1 |
| --- | --- | ---: |
| Each **input** | **IRM-15-12** | [Bravo $8.60](https://www.bravoelectro.com/irm-15-12.html) · DigiKey | **$8.60** |
| **sign-output** | **LRS-200-12** | [Bravo $26.00](https://www.bravoelectro.com/lrs-200-12.html) · [DigiKey $26.00](https://www.digikey.com/en/products/detail/mean-well-usa-inc/LRS-200-12/7705022) | **$26.00** |
| **mp-output** | **LRS-150-12** | [Bravo $19.80](https://www.bravoelectro.com/lrs-150-12.html) | **$19.80** |

LRS: selector **115 V**, trim **12.0 V**.

### A2. GX16 connectors — **LCSC** (not Amazon; cheaper than DigiKey circular)

| Use | Pins | Buy | Typical $ |
| --- | ---: | --- | ---: |
| AC mains in | **3** | LCSC search `GX16-3` set | **~$2** / set |
| Glowflies out | **3** | same | **~$2** |
| RS-485 | **6** | [KH-GX16-6P C18203269](https://www.lcsc.com/product-detail/C18203269.html) | **$2.13** / set |
| Solenoids | sign **8** / mp **6** | LCSC `GX16-8` / `KH-GX16-6P` | **~$2–2.50** |

Example 3P female: [XD-GX16-3P-F C22392638](https://www.lcsc.com/product-detail/C22392638.html) @ **$0.44** (+ matching male).

### A3. USB / buttons / POWER / relay / consumables

| Item | Spec | Buy | Qty‑1 |
| --- | --- | --- | ---: |
| USB-C bulkhead | HangTon D-type F–F + cap | [Amazon B0CDPR3BJS](https://www.amazon.com/HangTon-Waterproof-Connector-Extension-Bulkhead/dp/B0CDPR3BJS) | **~$8.50** |
| USB-C jumper | M–M ≤50 cm | Amazon / DigiKey | **~$2** |
| RESET | Adafruit **559** red | [adafruit.com/product/559](https://www.adafruit.com/product/559) | **$4.95** |
| BOOT | Adafruit **481** blue | [adafruit.com/product/481](https://www.adafruit.com/product/481) | **$4.95** |
| POWER (output) | **WRG32F2FBBNN** | DigiKey **CH784-ND** | **$2.88** |
| Glow relay | **G5LE-14-DC12** | DigiKey/Mouser | **~$1.25–1.73** |
| Sign fuse | **BK/GMC-5-R** | DigiKey | ~$1.30 |
| Mp fuse | **BK/GMC-3.15-R** | DigiKey | ~$1.20 |
| Input fuse | **BK/GMC-1-R** | DigiKey | ~$1.00 |
| Wire + ferrules | DigiKey hook-up | DigiKey | **~$4**/box |
| Gasket + inserts | McMaster | McMaster | **~$4**/box |

### A4. Wire gauges

| Net | Gauge |
| --- | --- |
| AC L/N/PE, glow AC | **16–18 AWG** 300 V |
| Solenoid +12 V / returns | **18 AWG** |
| Relay coil, OVR strap | **22 AWG** |
| RS-485, NRST, BOOT, USB | **24–28 AWG** |

---

## B. Per-box roll-up (≤ $50 target)

### sign-input / mp-input — **≈ $45**

| Line | $ |
| --- | ---: |
| IRM-15-12 | 8.60 |
| GX16-3 + GX16-6 (LCSC) | ~4 |
| HangTon USB + jumper | ~10.50 |
| Adafruit RESET + BOOT | 9.90 |
| Fuse + holder | 3 |
| Wire / gasket / inserts | 8 |
| Arcade POWER / CH / ALL | 0 |
| **Total** | **≈ $45** |

### sign-output — **≈ $65**

| Line | $ |
| --- | ---: |
| LRS-200-12 | 26.00 |
| GX16 AC + glow + RS-485 + SOL8 | ~9 |
| HangTon USB + jumper | ~10.50 |
| Adafruit RESET + BOOT | 9.90 |
| WRG32F2FBBNN | 2.88 |
| G5LE-14-DC12 | ~1.50 |
| Fuse 5 A + holder + wire/gasket | ~8 |
| **Total** | **≈ $65** |

### mp-output — **≈ $58**

| Line | $ |
| --- | ---: |
| LRS-150-12 | 19.80 |
| GX16 set (LCSC) | ~8 |
| HangTon USB + jumper | ~10.50 |
| Adafruit RESET + BOOT | 9.90 |
| POWER + G5LE + fuse + consumables | ~11 |
| **Total** | **≈ $58** |

---

## C. Channel / connector maps (budget)

### Solenoid multipin — **parallel +12 V** (5 A/pin limit)

#### sign-output GX16-8

| Pin | Net |
| ---: | --- |
| **1** | `+12V` (`J6`) |
| **2** | `+12V` (`J6`) — **must parallel pin 1** |
| 3 | OUT0 |
| 4 | OUT1 |
| 5 | OUT2 |
| 6 | OUT3 |
| 7 | OUT4 |
| 8 | NC |

#### mp-output GX16-6

| Pin | Net |
| ---: | --- |
| **1–2** | `+12V` paralleled |
| 3 | OUT0 |
| 4 | OUT1 |
| 5 | OUT2 |
| 6 | NC |

Field cables later (out of scope).

### RS-485 GX16-6

| Pin | Signal | Board |
| ---: | --- | --- |
| 1 | TX+ | `CN2`/`J2` pin 1 |
| 2 | TX− | 2 |
| 3 | RX+ | 3 |
| 4 | RX− | 4 |
| 5 | GND | 5 |
| 6 | SHIELD | 6 |

Crossover cable still required between input and output (swap TX↔RX pairs).

### Glowflies (both output boxes)

- MCU **CH7** (“CH8”); **OVR7 → GND** strap  
- Coil: `J6` → G5LE coil → `J5b.4`  
- Contacts: COM ← AC hot after POWER; NO → GX16 glow hot; N → glow N  

---

## D. Confidence

| Part class | Conf. | Note |
| --- | --- | --- |
| Mean Well PSU | High | Bravo prices verified live |
| Omron G5LE | High | Confirm stock; 10 A @ 120 VAC OK for glow |
| WRG32F2FBBNN | High electrical / Med seal | Not face-IP67 |
| Adafruit 559/481 | High | Correct SKUs; gasketed |
| GX16 LCSC | Med | Parallel +12 V; IP often 55–65 |
| HangTon USB-C | Med | IP65 with cap; Amazon price floats |
| Print + gasket | Med | Hose-test |

---

## E. Still open

1. Confirm LCSC cart stock for KH-GX16-8P / 3P sets before ordering.  
2. Arcade POWER ≥5 A @ 125 VAC on input boxes.  
3. Glowflies AC current ≤ ~5 A (GX16-3 @ 7 A).  
4. Optional: DigiKey-only path ≈ $150+/box — not pursued.
