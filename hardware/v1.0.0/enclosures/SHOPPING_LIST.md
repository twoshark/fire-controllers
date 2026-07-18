# Shopping list — 4 enclosures + 2 HLG

**Printed:** sign-input · mp-input · sign-output · mp-output  
**PSU:** 2× **HLG-240H-12** (IP67, 12 V / 16 A) — full PCB headroom; day-1 panels still **sign 5** + **mp 3** DT.  
**RS-485:** runs up to **~30 ft** and **~50 ft** → order **100 ft** spool.  
**Review:** [`PARTS_REVIEW.md`](PARTS_REVIEW.md) · hinge CAD: [`MOUNTING.md`](MOUNTING.md).  
Ratings / seal: [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) · [`SEALING.md`](SEALING.md).

**Rule:** every line has a real product URL and a listed USD price. Prices checked **2026-07-18**. Re-check at checkout.

**Est. checkout:** **~$484** (+ Lowdoller ship if under $100).

---

## On hand — do not buy

| Item | Use |
| --- | --- |
| M2 + M3 heat-set inserts | PCB · hinges · latches · RS-15 |
| M2 + M3 screws | same |
| C13–C14 (PC/monitor IEC) cords ×2 | Input box mains → C14 inlet |
| EG STARTS triangle arcade buttons | sign-input + mp-input lids |

Still need outdoor **SJTW** (or equivalent NEMA 5-15 outdoor cord) for HLG AC — not the same as C13–C14.

---

## Cables (lengths)

| Run | Qty | Length | Source |
| --- | ---: | ---: | --- |
| C13→C14 (inputs) | 2 | **6 ft** | **OWNED** |
| SJTW to each HLG | 2 | **6 ft** · 16/3 | buy |
| HLG DC → output DTP | 2 | **≤ 4 ft** · 12 AWG | buy |
| RS-485 interconnect | 2 | up to **~30 ft** + **~50 ft** → M12-5 | buy |

---

## Per unit

| Unit | AC | POWER | 12 V | RS-485 | SOL | PSU |
| --- | --- | --- | --- | --- | --- | --- |
| sign-input | C14 | KCD4 IP65 | — | M12-5 | — | RS-15-12 |
| mp-input | C14 | KCD4 IP65 | — | M12-5 | — | RS-15-12 |
| sign-output | — | — | DTP IN | M12-5 | **5× DT** | ← HLG-240H |
| mp-output | — | — | DTP IN | M12-5 | **3× DT** | ← HLG-240H |
| HLG ×2 | SJTW + adhesive HS butt (outdoor) | unplug to kill | DTP on pigtail | — | — | HLG-240H-12 |

### Pin maps

| Interface | Pins |
| --- | --- |
| DTP 12 V | 1=`+12V` · 2=`GND` · box **DTP04-2P** · HLG **DTP06-2S** |
| M12-5 RS-485 | 1–4=TX+/TX−/RX+/RX− · 5=GND · shell=SHIELD |
| DT SOL | 1=`+12V` (star from `J6`) · 2=`OUTn` |

---

## Cart 1 — Bravo Electro · **$159.80**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 2 | Mean Well **RS-15-12** | $8.60 | $17.20 | https://www.bravoelectro.com/rs-15-12.html |
| 2 | Mean Well **HLG-240H-12** (IP67, 12 V / 16 A) | $71.30 | $142.60 | https://www.bravoelectro.com/hlg-240h-12.html |

16 A covers PCB ceiling (8×2 A). Day-1 DT stays **5** + **3** — buy more DT later if you panel unused channels.

---

## Cart 2 — Lid hinges (Amazon) · **$5.29**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | bociloy **1"** SS butt hinge · **10-pack** (use 8) | $5.29 | $5.29 | https://www.amazon.com/bociloy-Cabinet-Rectangle-Stainless-Folding/dp/B0D43MBGXC |

**CAD locked** (open **25×28** mm): pin centers unchanged; **M2** boss at mid-pin, **7.5 mm** into cavity — [`MOUNTING.md`](MOUNTING.md). Caliper on arrival if 2 holes/leaf.  
Premium: [Alema KHA-25C](https://www.alema.com/kha-25c-25mm-stainless-steel-butt-hinge-with-screw-holes.html) @ **$4.34** ×8 (M3 pattern).

---

## Cart 3 — Lowdoller Motorsports (Deutsch) · **$50.00** (+ ship)

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

## Cart 4 — Cable · **$185.08**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | **KWANGIL** RS-485 24 AWG 2-pr · Belden **9842 equal** · **100 ft** | $138.99 | $138.99 | https://kwangilwire.com/products/24awg-2pr-rs-485-shielded-cable-equal-to-belden-9842-str-tc-pe-ins-os-drain-tc-brd-pvc-jkt-120-ohm-for-low-voltage-analog-signals-digital-control |
| 2 | ELECTERY **16/3 SJTW** outdoor 6 ft | $10.55 | $21.10 | https://www.walmart.com/ip/6-Ft-Black-Indoor-Outdoor-Extension-Cord-Heavy-Duty-Flexible-16-AWG-Power-Cable-SJTW-Waterproof-Weather-Resistant-Wire-3-Prong-Grounded-Wire-ELECTERY/2088408818 |
| 1 | Powerwerx **12 AWG** red/black bonded copper · **25 ft** | $24.99 | $24.99 | https://www.valley-ent.com/store/red-black-bonded-zip-cord-easy-id-low-voltage-cable-gauge-12-length-25-feet.html |

Select **24AWG / 100ft**. Do **not** use CAT5/6 for these runs.  
C13–C14: **owned** — not in cart.

---

## Cart 5 — Panel / hardware · **$83.63**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | MECCANIXITY M12 5-pin M+F · **4-set** IP67 | $22.39 | $22.39 | https://www.amazon.com/dp/B0CFFX6JW4 |
| 1 | Elecbee M12 dust caps · **5-pack** | $12.69 | $12.69 | https://www.amazon.com/Elecbee-Protective-Plastic-Panel-Mount-Sockets/dp/B0B6HYB4KM |
| 1 | QTEATAK C14 panel inlet **with cover** · **2-pack** | $5.99 | $5.99 | https://www.amazon.com/gp/product/B081C32LWG |
| 2 | KCD4 DPST **IP65** rocker 30×22 (Metabee) | $1.20 | $2.40 | https://www.metabee.com/waterproof-baot-rocker-switch-kcd4-4pin-on-off-16a-250vac-dpst-square-green-led.html |
| 1 | Wirefy adhesive heatshrink · **200 pcs** 3:1 (HLG AC outdoors + DC re-term) | $14.99 | $14.99 | https://www.amazon.com/dp/B089D82FLG |
| 1 | ATO **20 A** blade fuses · **20 pcs** | $4.39 | $4.39 | https://www.amazon.com/20pcs-Standard-Fuses-Blade-Automotive/dp/B0CLGTWZHZ |
| 1 | Clear acrylic 8×10 × **1 mm** · **2-pack** (LED windows) | $6.99 | $6.99 | https://www.amazon.com/Extruded-Plexiglass-Protective-Projects-Watercolor/dp/B0D4QVJRLT |
| 1 | WOD closed-cell foam tape 1/8"×1/4"×75' (lid gaskets) | $13.79 | $13.79 | https://www.amazon.com/WOD-Tape-Weatherstrip-Insulation-Available/dp/B07H33916Y |

Heat-sets · M2/M3 screws · arcade buttons: **owned**. Inside boxes: screw / solder / adhesive butt only. Outdoor HLG AC: adhesive HS (Wirefy).

---

## Total (checked 2026-07-18)

| Cart | Ext |
| --- | ---: |
| 1 Bravo PSU (HLG-240H) | $159.80 |
| 2 Amazon 1" hinges | $5.29 |
| 3 Lowdoller DT/DTP (+ ship) | $50.00 |
| 4 Cable | $185.08 |
| 5 Panel / hardware | $83.63 |
| **Est. checkout** | **~$483.80** (+ Lowdoller ship) |

Owned: inserts · screws · C13–C14 · arcade buttons. DT still day-1 **8** only.  

TE flange: [`COST_OPTIONS.md`](COST_OPTIONS.md) **C**. Full necessity table: [`PARTS_REVIEW.md`](PARTS_REVIEW.md).

---

## Checklist

- [ ] Bravo: RS-15×2, **HLG-240H-12×2**  
- [ ] bociloy 1" ×8 · caliper holes vs [`MOUNTING.md`](MOUNTING.md)  
- [ ] Lowdoller: **2× DTP** + **8× DT** (5 sign + 3 mp)  
- [ ] KWANGIL **100 ft** + M12 4-set + dust caps + C14 + SJTW + 12 AWG  
- [ ] KCD4 IP65, adhesive HS, fuse, acrylic, foam  
- [ ] On hand: heat-sets · M2/M3 screws · C13–C14 ×2 · arcade buttons  
- [ ] Crimp DTP06-2S on each HLG DC · HS seal · ≤4 ft  
- [ ] Mount HLGs shaded · FG earthed · `F9` **20 A** seated  
- [ ] Measure SOL current @ 12 V before loading all channels  
