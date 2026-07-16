# Shopping list — 4 enclosures (triple-checked 2026-07-15)

Build kit: **sign-input**, **mp-input**, **sign-output**, **mp-output**.

Vendors: **Bravo → DigiKey/Mouser → Adafruit → LCSC → Amazon (HangTon USB only) → McMaster**.

---

## Audit findings (what changed vs prior list)

| # | Issue | Severity | Fix |
| ---: | --- | --- | --- |
| 1 | Adafruit **558 = white**, not red. **557 = earrings** | High | RESET = **559** red; BOOT = **481** blue (or **558** white) |
| 2 | Adafruit **560** green **out of stock** | Med | Do not order green as primary |
| 3 | LRS-200 AC draw **4 A @ 115 VAC** full load (datasheet) | High | Sign fuse → **T5 A** (`BK/GMC-5-R`), not 3.15 A |
| 4 | GX16-6/8 contacts ≈ **5 A/pin**; single `+12V` pin cannot carry 10 A | **Critical** | Parallel **two** `+12V` pins on solenoid multipin (see pin map) |
| 5 | DigiKey rocker URL `/4655/` unreliable | Low | Use MPN **WRG32F2FBBNN** / **CH784-ND** |
| 6 | DigiKey G5LE URL was wrong vendor slug | Low | Search MPN **G5LE-14-DC12**; Mouser also fine (~$1.25) |
| 7 | GX16 often **IP55**, not true IP67 | Med | Buy metal + rubber gasket variants; hose-test; claim IP65-class |
| 8 | USB was deferred for cost | — | **Re-locked HangTon** — lid-closed DFU required |
| 9 | Bravo IRM blurb says “1 A” but table/datasheet = **1.25 A** | Low | Trust **1.25 A** |
| 11 | SIGN_OUTPUT fuse / diagram still said 3.15 A / LRS-150 | High | Synced → **5 A** fuse + LRS-200; mp → **3.15 A** |
| 12 | Solenoid pin maps still single +12 V in box docs | Critical | Synced paralleled pins in SIGN/MP_OUTPUT + WIRING |
| 13 | Adafruit **559** only **5 in stock** | Med | Order soon; need 4 RESET |

---

## PSU sizing (re-verified)

| Box | 12 V load | PSU | Rating | AC @115 V (datasheet typ.) | Headroom |
| --- | --- | --- | ---: | --- | --- |
| sign-output | 5×2 A + ~0.3 A ≈ **10.3 A** | **LRS-200-12** | **17 A / 204 W** | **4.0 A** full | ~65% on DC |
| mp-output | 3×2 A + ~0.3 A ≈ **6.3 A** | **LRS-150-12** | **12.5 A / 150 W** | **2.8 A** full | ~50% on DC |
| each input | PCB+LEDs &lt; 0.8 A | **IRM-15-12** | **1.25 A / 15 W** | ~0.2 A | large |

LRS: set selector **115 V**, trim **12.0 V**. Cold-start inrush on LRS-200 is **60 A** — use **time-delay** (GMC) fuses.

---

## Cart 1 — Bravo Electro (PSUs)

| Qty | MPN | For | Link | Unit | Ext |
| ---: | --- | --- | --- | ---: | ---: |
| 2 | **IRM-15-12** | inputs | [bravoelectro.com/irm-15-12](https://www.bravoelectro.com/irm-15-12.html) | **$8.60** | **$17.20** |
| 1 | **LRS-200-12** | sign-output | [bravoelectro.com/lrs-200-12](https://www.bravoelectro.com/lrs-200-12.html) | **$26.00** | **$26.00** |
| 1 | **LRS-150-12** | mp-output | [bravoelectro.com/lrs-150-12](https://www.bravoelectro.com/lrs-150-12.html) | **$19.80** | **$19.80** |
| | | | **Subtotal** | | **$63.00** |

DigiKey LRS-200 also **$26.00** if consolidating POs. Conf: **High**.

---

## Cart 2 — DigiKey / Mouser

| Qty | MPN | Role | Where | Unit (typ.) | Ext | Conf |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 2 | **G5LE-14-DC12** | Glow relay | DigiKey/Mouser search MPN (coil 12 V, **33 mA**, contacts **10 A @ 120 VAC**) | **~$1.25–1.73** | **~$3.00** | High |
| 2 | **WRG32F2FBBNN** | POWER rocker | DigiKey **CH784-ND** — DPST **16 A @ 125/250 VAC** | **$2.88** | **$5.76** | High |
| 2 | **BK/GMC-1-R** | Input AC fuse 1 A TD | DigiKey | ~$1.00 | ~$2.00 | High |
| 1 | **BK/GMC-5-R** | Sign-output AC fuse **5 A** TD | DigiKey | ~$1.30 | ~$1.30 | High |
| 1 | **BK/GMC-3.15-R** | Mp-output AC fuse 3.15 A TD | DigiKey | ~$1.20 | ~$1.20 | High |
| 4 | 5×20 fuse holder (internal) | all boxes | DigiKey `holder 5x20` | ~$2–4 | ~$12 | Med |
| 1 spool | **18 AWG** 300 V red ~25 ft | AC / +12 V | DigiKey | ~$8 | ~$8 | High |
| 1 spool | **18 AWG** 300 V black ~25 ft | N / returns | DigiKey | ~$8 | ~$8 | High |
| 1 spool | **16 AWG** 300 V ~10 ft | LRS AC hot/N (optional stiffer) | DigiKey | ~$5 | ~$5 | High |
| 1 spool | **22 AWG** ~25 ft | coil / OVR / buttons | DigiKey | ~$5 | ~$5 | High |
| 1 spool | **24 AWG** ~25 ft | RS-485 | DigiKey | ~$5 | ~$5 | High |
| 1 | Ferrule kit 16–22 AWG | screw terminals | DigiKey | ~$8 | ~$8 | High |
| | | | **Subtotal** | | **~$64** | |

**POWER seal:** `WRG32F2FBBNN` is **not** face-IP67. Silicone the bezel + rely on lid gasket, or put rocker on an inner panel.

**Do not buy on DigiKey for this budget:** Buccaneer, Phoenix 1513758, MUSBR USB-C, MP0045, PV1F640SS×8.

**G5LE note:** Confirm DigiKey/Mouser **in stock** (some G5LE variants EOL). Equivalent: Omron G5LE-14 DC12 sealed SPDT. Coil between `J6` and `J5b.4` — **do not** need Pololu carrier.

---

## Cart 3 — Adafruit (RESET / BOOT)

| Qty | ID | Color | Role | Link | Unit | Ext | Stock note |
| ---: | ---: | --- | --- | --- | ---: | ---: | --- |
| 4 | **559** | Red | RESET | [adafruit.com/product/559](https://www.adafruit.com/product/559) | $4.95 | **$19.80** | **5 in stock** — order promptly |
| 4 | **481** | Blue | BOOT | [adafruit.com/product/481](https://www.adafruit.com/product/481) | $4.95 | **$19.80** | In stock |
| | | | | **Subtotal** | | **$39.60** | |

Alternates same price: **558** white. Avoid **560** green (was OOS). Avoid **557** (not a button).

Wiring: COM→GND, NO→`NRST`/`BOOT0`. LED ring optional (3–6 V; for 12 V add **470 Ω** series). Leave LED open if unused.

Cutout **Ø16 mm**. Gasket included — good for panel seal. Switch rating 3 A — **signal only**, not AC mains. Conf: **High**.

Also sold via DigiKey/Mouser as Adafruit distributors if you want one cart.

---

## Cart 4 — LCSC (GX16)

Prefer **sets** (male+female). Verify photo = panel flange + cable plug.

| Qty | MPN / LCSC # | Role | Rating | Unit | Ext |
| ---: | --- | --- | --- | ---: | ---: |
| 4 | **KH-GX16-3P** set *or* `XD-GX16-3P-M` [C22392642](https://jlcpcb.com/partdetail/chxunda-XD_GX16_3PM/C22392642) + `XD-GX16-3P-F` [C22392638](https://www.lcsc.com/product-detail/C22392638.html) | AC IN ×4 | **7 A / 220 V** | ~$1.50–2.00 | **~$7** |
| 2 | same 3P set | GLOW OUT ×2 | 7 A | ~$2 | **~$4** |
| 4 | **KH-GX16-6P** [C18203269](https://www.lcsc.com/product-detail/C18203269.html) | RS-485 ×4 | **5 A / 125 V** | **$2.13** | **$8.52** |
| 1 | **KH-GX16-8P** [C19269294](https://jlcpcb.com/partdetail/Shenzhen_KinghelmElec-KH_GX168P/C19269294) | sign solenoids | **5 A/pin** | **~$2.01** | **~$2** |
| 1 | **KH-GX16-6P** | mp solenoids | 5 A/pin | $2.13 | **$2.13** |
| AR | dust caps | unmated | — | ~$0.30 | **~$3** |
| | | | **Subtotal** | | **~$27** |

LCSC 3P-F live price was **~$0.63** (promo); male **~$1.03** — confirm cart.

### Gender (do not mix up)

| Port | Panel half | Why |
| --- | --- | --- |
| AC IN | **Male pins** on panel | Appliance convention |
| GLOW OUT | **Female sockets** on panel | Wall cord cannot mate |
| RS-485 / SOL | Either — stay consistent box-to-box | Document in silk |

### Solenoid pin map — **paralleled +12 V** (mandatory)

**sign-output GX16-8** (≤5 A/pin):

| Pin | Net |
| ---: | --- |
| **1** | `+12V` (`J6`) |
| **2** | `+12V` (`J6`) — **parallel with pin 1** |
| 3 | OUT0 |
| 4 | OUT1 |
| 5 | OUT2 |
| 6 | OUT3 |
| 7 | OUT4 |
| 8 | NC |

Max continuous on paralleled +12 V ≈ **8–10 A** if both pins share current — OK for 5×2 A with even split. Use **18 AWG** into both pins.

**mp-output GX16-6:**

| Pin | Net |
| ---: | --- |
| **1–2** | `+12V` paralleled |
| 3 | OUT0 |
| 4 | OUT1 |
| 5 | OUT2 |
| 6 | NC |

Conf: **Med** (generic aviation; verify gasketed listing).

---

## Cart 5 — Amazon HangTon USB-C (lid-closed DFU)

**Locked:** panel F–F bulkhead + short internal USB-C M–M jumper → board `J5` (input) / `J7` (output). Cap on when unused → **IP65**.

| Qty | Item | Link | Unit | Ext |
| ---: | --- | --- | ---: | ---: |
| 2 packs (4 pcs) | HangTon USB-C **D-type** F–F bulkhead + weather cap | [Amazon B0CDPR3BJS](https://www.amazon.com/HangTon-Waterproof-Connector-Extension-Bulkhead/dp/B0CDPR3BJS) (~$16.99 / 2-pack) | **~$8.50**/ea | **~$34** |
| *or* 1× 5-pack | same (1 spare) | [Amazon B0CDPT5YT3](https://www.amazon.com/HangTon-Waterproof-Connector-Bulkhead-Extension/dp/B0CDPT5YT3) | ~$8.18 | **~$41** |
| 4 | USB-C M–M jumper **≤30–50 cm** (DFU data OK) | Amazon short pack *or* DigiKey `USB-C male male` | ~$2 | **~$8** |
| | | **Subtotal (2×2-pack path)** | | **~$42** |

**Reject:** DigiKey `MUSBR-M5C1-M0` (~$17/ea). Circular HangTon ([B0CDPVS7V4](https://www.amazon.com/HangTon-Waterproof-Circular-Pass-Through-Connector/dp/B0CDPVS7V4)) is fine if you prefer round cutout — same price class.

**Install:** D-cut per HangTon drawing; nut from inside; short jumper board-side only; keep USB loom away from AC.

---

## Cart 6 — McMaster / hardware

| Item | ~$ |
| --- | ---: |
| EPDM gasket cord 2–3 mm × ~10 ft | ~8 |
| M3 heat-set inserts kit | ~8 |
| M3 screws | ~5 |
| **Subtotal** | **~$21** |

---

## Optional Pololu

[2482](https://www.pololu.com/product/2482) @ $7.95 ×2 — only if you want screw terminals. **Default: bare G5LE.**

---

## System interconnect (not in per-box $50)

| Item | Vendor |
| --- | --- |
| Belden **9842** | DigiKey |
| GX16-6 field jumpers with **TX↔RX crossover** | Build from LCSC mates |

---

## Corrected kit total (HangTon USB included)

| Cart | ~$ |
| --- | ---: |
| Bravo | 63 |
| DigiKey | 64 |
| Adafruit | 40 |
| LCSC | 27 |
| HangTon USB + jumpers | 42 |
| McMaster | 21 |
| **Total** | **≈ $257** |
| **Avg / box** | **≈ $64** |

| Box | ≈$ |
| --- | ---: |
| Each input | **~$45** |
| mp-output | **~$58** |
| sign-output | **~$65** (LRS-200 $26) |

Per-box $50 target slips on outputs because of PSU + USB; inputs stay under.

---

## Order checklist

- [ ] Bravo: 2×IRM-15-12, 1×LRS-200-12, 1×LRS-150-12  
- [ ] DigiKey: 2×G5LE-14-DC12, 2×WRG32F2FBBNN, fuses **1 A / 5 A / 3.15 A**, holders, wire, ferrules  
- [ ] Adafruit: 4×**559** + 4×**481**  
- [ ] LCSC: GX16 3/6/8 per table; **parallel +12 V pins** on SOL  
- [ ] Amazon: HangTon USB-C ×4 + 4× short USB-C M–M jumpers  
- [ ] McMaster: gasket + inserts  
- [ ] Skip DigiKey MUSBR / Buccaneer / Phoenix M12  
- [ ] Silk-label AC IN vs GLOW gender; USB cap when unused  

Wiring detail: [`WIRING.md`](WIRING.md) · BOM: [`PARTS_BOM.md`](PARTS_BOM.md).
