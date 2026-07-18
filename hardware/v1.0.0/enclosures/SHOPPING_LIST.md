# Shopping list — 4 enclosures + 2 HLG

**Printed:** sign-input · mp-input · sign-output · mp-output  
**PSU:** 2× **HLG-185H-12** (IP67, 12 V / 13 A) — sized for **sign 5 SOL** + **mp 3 SOL** (no 8-ch expand).  
**RS-485:** runs up to **~30 ft** and **~50 ft** → order **100 ft** spool.  
**Review:** [`PARTS_REVIEW.md`](PARTS_REVIEW.md) · hinge CAD: [`MOUNTING.md`](MOUNTING.md).  
Ratings / seal: [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) · [`SEALING.md`](SEALING.md).

**Rule:** every line has a real product URL and a listed USD price. Prices checked **2026-07-18**. Re-check at checkout.

**Est. checkout:** **~$534** (+ Lowdoller ship if under $100).

---

## Cables (lengths)

| Run | Qty | Length |
| --- | ---: | ---: |
| C13→C14 (inputs) | 2 | **6 ft** |
| SJTW to each HLG | 2 | **6 ft** · 16/3 |
| HLG DC → output DTP | 2 | **≤ 4 ft** · 12 AWG |
| RS-485 interconnect | 2 | up to **~30 ft** + **~50 ft** → M12-5 |

---

## Per unit

| Unit | AC | POWER | 12 V | RS-485 | SOL | PSU |
| --- | --- | --- | --- | --- | --- | --- |
| sign-input | C14 | KCD4 IP65 | — | M12-5 | — | RS-15-12 |
| mp-input | C14 | KCD4 IP65 | — | M12-5 | — | RS-15-12 |
| sign-output | — | — | DTP IN | M12-5 | **5× DT** | ← HLG-185H |
| mp-output | — | — | DTP IN | M12-5 | **3× DT** | ← HLG-185H |
| HLG ×2 | SJTW + adhesive HS butt (outdoor) | unplug to kill | DTP on pigtail | — | — | HLG-185H-12 |

### Pin maps

| Interface | Pins |
| --- | --- |
| DTP 12 V | 1=`+12V` · 2=`GND` · box **DTP04-2P** · HLG **DTP06-2S** |
| M12-5 RS-485 | 1–4=TX+/TX−/RX+/RX− · 5=GND · shell=SHIELD |
| DT SOL | 1=`+12V` (star from `J6`) · 2=`OUTn` |

---

## Cart 1 — Bravo Electro · **$132.00**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | Mean Well **RS-15-12** | $8.60 | $17.20 | https://www.bravoelectro.com/rs-15-12.html |
| 2 | Mean Well **HLG-185H-12** (IP67, 12 V / 13 A) | $57.40 | $114.80 | https://www.bravoelectro.com/hlg-185h-12.html |

13 A covers sign **10 A** peak (5×2 A) and mp **6 A** peak (3×2 A). Do **not** load unused PCB channels without a larger PSU + more DT.

---

## Cart 2 — Lid hinges (Amazon) · **$5.29**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | bociloy **1"** SS butt hinge · **10-pack** (use 8) | $5.29 | $5.29 | https://www.amazon.com/bociloy-Cabinet-Rectangle-Stainless-Folding/dp/B0D43MBGXC |

**CAD locked** (open **25×28** mm): pin centers unchanged; **M2** boss at mid-pin, **7.5 mm** into cavity — [`MOUNTING.md`](MOUNTING.md). Caliper on arrival if 2 holes/leaf.  
Premium: [Alema KHA-25C](https://www.alema.com/kha-25c-25mm-stainless-steel-butt-hinge-with-screw-holes.html) @ **$4.34** ×8 (M3 pattern).

---

## Cart 3 — CNC Kitchen USA · **$22.40**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | Heat-set **M3×5.7** · 100 pcs | $10.90 | $10.90 | https://cnckitchenus.store/products/heat-set-insert-m3-x-5-7-100-pieces |
| 1 | Heat-set **M2×3.0** · 100 pcs | $11.50 | $11.50 | https://cnckitchenus.store/products/heat-set-insert-m2-x-3-100-pieces |

**M2:** PCB + hinge leaves. **M3:** front latches + RS-15 only (~20 needed; 100-pack is cheapest break).

---

## Cart 4 — Lowdoller Motorsports (Deutsch) · **$50.00** (+ ship)

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | **DTP** 2-pin plug+receptacle kit · PN 35570 | $9.00 | $18.00 | https://lowdoller-motorsports.com/products/deutsch-dt-2-kit-pn-35557 |
| 8 | **DT** 2-pin plug+receptacle kit · PN 35557 | $4.00 | $32.00 | https://lowdoller-motorsports.com/products/deutsch-dt-2-kit-dt06-2s-dt04-2p-pn-35557 |

| Kit | Why |
| --- | --- |
| DTP ×**2** | One mating pair per HLG→box 12 V link (2 links) |
| DT ×**8** | sign **5** + mp **3** (day-1 panel layout) |

Free ship was @ $100 — this cart is under; pay Lowdoller shipping at checkout.

---

## Cart 5 — Cable · **$197.07**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | **KWANGIL** RS-485 24 AWG 2-pr · Belden **9842 equal** · **100 ft** | $138.99 | $138.99 | https://kwangilwire.com/products/24awg-2pr-rs-485-shielded-cable-equal-to-belden-9842-str-tc-pe-ins-os-drain-tc-brd-pvc-jkt-120-ohm-for-low-voltage-analog-signals-digital-control |
| 1 | Cable Matters **C13–C14** 6 ft · **2-pack** | $11.99 | $11.99 | https://www.amazon.com/Cable-Matters-2-Pack-Computer-Extension/dp/B01CH25VQQ |
| 2 | ELECTERY **16/3 SJTW** outdoor 6 ft | $10.55 | $21.10 | https://www.walmart.com/ip/6-Ft-Black-Indoor-Outdoor-Extension-Cord-Heavy-Duty-Flexible-16-AWG-Power-Cable-SJTW-Waterproof-Weather-Resistant-Wire-3-Prong-Grounded-Wire-ELECTERY/2088408818 |
| 1 | Powerwerx **12 AWG** red/black bonded copper · **25 ft** | $24.99 | $24.99 | https://www.valley-ent.com/store/red-black-bonded-zip-cord-easy-id-low-voltage-cable-gauge-12-length-25-feet.html |

Select **24AWG / 100ft**. Do **not** use CAT5/6 for these runs.

---

## Cart 6 — Panel / hardware · **$127.55**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | EG STARTS triangle LED ×5 | $11.99 | $23.98 | https://www.amazon.com/dp/B07R565HM6 |
| 1 | MECCANIXITY M12 5-pin M+F · **4-set** IP67 | $22.39 | $22.39 | https://www.amazon.com/dp/B0CFFX6JW4 |
| 1 | Elecbee M12 dust caps · **5-pack** | $12.69 | $12.69 | https://www.amazon.com/Elecbee-Protective-Plastic-Panel-Mount-Sockets/dp/B0B6HYB4KM |
| 1 | QTEATAK C14 panel inlet **with cover** · **2-pack** | $5.99 | $5.99 | https://www.amazon.com/gp/product/B081C32LWG |
| 2 | KCD4 DPST **IP65** rocker 30×22 (Metabee) | $1.20 | $2.40 | https://www.metabee.com/waterproof-baot-rocker-switch-kcd4-4pin-on-off-16a-250vac-dpst-square-green-led.html |
| 1 | Wirefy adhesive heatshrink · **200 pcs** 3:1 (HLG AC outdoors + DC re-term) | $14.99 | $14.99 | https://www.amazon.com/dp/B089D82FLG |
| 1 | ATO **20 A** blade fuses · **20 pcs** | $4.39 | $4.39 | https://www.amazon.com/20pcs-Standard-Fuses-Blade-Automotive/dp/B0CLGTWZHZ |
| 1 | Clear acrylic 8×10 × **1 mm** · **2-pack** (LED windows) | $6.99 | $6.99 | https://www.amazon.com/Extruded-Plexiglass-Protective-Projects-Watercolor/dp/B0D4QVJRLT |
| 1 | WOD closed-cell foam tape 1/8"×1/4"×75' (lid gaskets) | $13.79 | $13.79 | https://www.amazon.com/WOD-Tape-Weatherstrip-Insulation-Available/dp/B07H33916Y |
| 1 | M3×6 pan Phillips · **100 pcs** | $9.95 | $9.95 | https://skycraftsurplus.com/products/m3-x-6mm-zinc-plated-steel-pan-head-screws-100-piece-lot.html |
| 1 | M2/M2.5 pan Phillips assort · **200 pcs** (PCB + hinges) | $9.99 | $9.99 | https://www.amazon.com/Phillips-Machine-200-piece-Assortmrnt-M2-5x10mm/dp/B078ZJ3CML |

Inside boxes: screw / solder / adhesive butt only. Outdoor HLG AC: adhesive HS (Wirefy).

---

## Total (checked 2026-07-18)

| Cart | Ext |
| --- | ---: |
| 1 Bravo PSU (HLG-185H) | $132.00 |
| 2 Amazon 1" hinges | $5.29 |
| 3 CNC Kitchen inserts | $22.40 |
| 4 Lowdoller DT/DTP (+ ship) | $50.00 |
| 5 Cable (100 ft RS-485) | $197.07 |
| 6 Panel / hardware | $127.55 |
| **Est. checkout** | **~$534.31** (+ Lowdoller ship) |

Cuts vs prior 8-ch cart: −8 DT (−$32) · −2 DTP spare (−$18) · HLG-240→185 (−$27.80).  
TE flange: [`COST_OPTIONS.md`](COST_OPTIONS.md) **C**. Full necessity table: [`PARTS_REVIEW.md`](PARTS_REVIEW.md).

---

## Checklist

- [ ] Bravo: RS-15×2, **HLG-185H-12×2**  
- [ ] bociloy 1" ×8 · caliper holes vs [`MOUNTING.md`](MOUNTING.md)  
- [ ] CNC Kitchen: M2 + M3 inserts  
- [ ] Lowdoller: **2× DTP** + **8× DT** (5 sign + 3 mp)  
- [ ] KWANGIL **100 ft** + M12 4-set + dust caps + C14 + C13–C14 + SJTW + 12 AWG  
- [ ] KCD4 IP65, adhesive HS, fuse, acrylic, foam, screws  
- [ ] Crimp DTP06-2S on each HLG DC · HS seal · ≤4 ft  
- [ ] Mount HLGs shaded · FG earthed · `F9` **20 A** seated  
- [ ] Measure SOL current @ 12 V before loading all channels  
