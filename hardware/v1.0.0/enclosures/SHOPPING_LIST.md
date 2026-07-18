# Shopping list — 4 enclosures + 2 HLG

**Printed:** sign-input · mp-input · sign-output · mp-output  
**PSU:** 2× **HLG-240H-12** (IP67, 12 V / 16 A) — headroom for each output box at **8 channels**.  
**RS-485:** runs up to **~30 ft** and **~50 ft** → order **100 ft** spool.  
Ratings / seal: [`CONNECTOR_RATINGS.md`](CONNECTOR_RATINGS.md) · [`SEALING.md`](SEALING.md).

**Rule:** every line has a real product URL and a listed USD price. Prices checked **2026-07-18**. Re-check at checkout.

**Est. checkout:** **~$660** (Lowdoller hits free ship).

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
| sign-output | — | — | DTP IN | M12-5 | up to 8× DT | ← HLG-240H |
| mp-output | — | — | DTP IN | M12-5 | up to 8× DT | ← HLG-240H |
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

---

## Cart 2 — Lid hinges (Amazon) · **$5.99**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | bociloy **1"** SS butt hinge · **10-pack** (use 8) | $5.99 | $5.99 | https://www.amazon.com/bociloy-Cabinet-Rectangle-Stainless-Folding/dp/B0D43MBGXC |

**CAD note:** bosses were laid out for Sugatsune **KHA-25C** (25 mm). These ~1" Amazon hinges need boss hole spacing matched to the leaf you buy (measure before printing). Same job: internal lid hinge.  
Premium keep: [Alema KHA-25C](https://www.alema.com/kha-25c-25mm-stainless-steel-butt-hinge-with-screw-holes.html) @ **$4.34** ×8 = $34.72.

---

## Cart 3 — CNC Kitchen USA · **$22.40**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | Heat-set **M3×5.7** · 100 pcs | $10.90 | $10.90 | https://cnckitchenus.store/products/heat-set-insert-m3-x-5-7-100-pieces |
| 1 | Heat-set **M2×3.0** · 100 pcs | $11.50 | $11.50 | https://cnckitchenus.store/products/heat-set-insert-m2-x-3-100-pieces |

**M3 uses (not optional if you want reusable threads in PETG/ASA):** lid hinge leaves · front lid latches · RS-15-12 feet (2 inputs). M2 = PCB bosses. Pack of 100 is cheapest break; you need ~40–50 M3.

---

## Cart 4 — Lowdoller Motorsports (Deutsch) · **$100.00** (free ship)

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 4 | **DTP** 2-pin plug+receptacle kit · PN 35570 | $9.00 | $36.00 | https://lowdoller-motorsports.com/products/deutsch-dt-2-kit-pn-35557 |
| 16 | **DT** 2-pin plug+receptacle kit · PN 35557 | $4.00 | $64.00 | https://lowdoller-motorsports.com/products/deutsch-dt-2-kit-dt06-2s-dt04-2p-pn-35557 |

DTP: 2× box + 2× HLG. DT: **16 mated pairs** = both output boxes at **8 SOL channels** (sign now 5, mp now 3, room to expand) + wedges/contacts. IP67 mated.  
Order total **$100** → free US ship (site: free over $100).

---

## Cart 5 — Cable · **$197.07**

| Qty | Item | Unit | Ext | Link |
| ---: | --- | ---: | ---: | --- |
| 1 | **KWANGIL** RS-485 24 AWG 2-pr · Belden **9842 equal** · **100 ft** | $138.99 | $138.99 | https://kwangilwire.com/products/24awg-2pr-rs-485-shielded-cable-equal-to-belden-9842-str-tc-pe-ins-os-drain-tc-brd-pvc-jkt-120-ohm-for-low-voltage-analog-signals-digital-control |
| 1 | Cable Matters **C13–C14** 6 ft · **2-pack** | $11.99 | $11.99 | https://www.amazon.com/Cable-Matters-2-Pack-Computer-Extension/dp/B01CH25VQQ |
| 2 | ELECTERY **16/3 SJTW** outdoor 6 ft | $10.55 | $21.10 | https://www.walmart.com/ip/6-Ft-Black-Indoor-Outdoor-Extension-Cord-Heavy-Duty-Flexible-16-AWG-Power-Cable-SJTW-Waterproof-Weather-Resistant-Wire-3-Prong-Grounded-Wire-ELECTERY/2088408818 |
| 1 | Powerwerx **12 AWG** red/black bonded copper · **25 ft** | $24.99 | $24.99 | https://www.valley-ent.com/store/red-black-bonded-zip-cord-easy-id-low-voltage-cable-gauge-12-length-25-feet.html |

Select **24AWG / 100ft** on the KWANGIL page ($138.99). Covers ~30 ft + ~50 ft with ~20 ft spare. UL-listed 120 Ω foil+braid Belden-9842 equal (~**$1.39/ft** vs ProWire Belden **$5.03/ft**).  
C13–C14: confirm Amazon lists **6 ft** before buy. Alt: [CablesAndKits 6 ft](https://www.amazon.com/Cablesandkits-Heavy-Duty-Power-Cord/dp/B0D12BPBSF) @ **$16.13** ×2.

**Cable cost levers** (see notes in chat / below checklist): buy **80 ft** only if a vendor sells it; stage one run first; do **not** substitute CAT5/6 (wrong Z₀ for long full-duplex). C13–C14 / SJTW / 12 AWG are already cheap — the RS-485 spool is the lever.

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
| 1 | M2/M2.5 pan Phillips assort · **200 pcs** (incl. M2×8) | $9.99 | $9.99 | https://www.amazon.com/Phillips-Machine-200-piece-Assortmrnt-M2-5x10mm/dp/B078ZJ3CML |

**No DryConn / waterproof wire nuts** — inside boxes: screw terminals / solder / adhesive-lined butt only. Outdoor HLG AC pigtail: adhesive HS butt (Wirefy) or a small outdoor junction box with screw terminals.  
**M12 note:** ASIN B0CFFX6JW4 last tracked US buybox **$22.39** (confirm). Alt: [Lonlonty 4-set](https://lonlonty.com/products/m12-5-pin-male-female-socket-panel-aviation-wire-connector-12mm-industrial-circular-connector-outdoor-waterproof-ip67-4-sets) @ **$33.99**.  
**C14 note:** if B081C32LWG has no offer, [Newegg C14+cover](https://www.newegg.com/p/1ET-0002-00D99) @ **$6.99** each ×2.

---

## Total (checked 2026-07-18)

| Cart | Ext |
| --- | ---: |
| 1 Bravo PSU (HLG-240H) | $159.80 |
| 2 Amazon 1" hinges | $5.99 |
| 3 CNC Kitchen inserts | $22.40 |
| 4 Lowdoller DT/DTP (free ship) | $100.00 |
| 5 Cable (100 ft RS-485) | $197.07 |
| 6 Panel / hardware | $127.55 |
| **Est. checkout** | **$612.81** |

TE flange upgrade: [`COST_OPTIONS.md`](COST_OPTIONS.md) **C**.

---

## Checklist

- [ ] Bravo: RS-15×2, **HLG-240H-12×2**  
- [ ] Amazon 1" hinges ×8 (measure leaf → CAD bosses) **or** Alema KHA-25C  
- [ ] CNC Kitchen: M2 + M3 inserts  
- [ ] Lowdoller: 4× DTP + **16× DT** ($100 free ship)  
- [ ] KWANGIL **100 ft** + M12 4-set + **dust caps** + C14 + C13–C14 + SJTW + 12 AWG  
- [ ] KCD4 IP65, adhesive HS (outdoor HLG AC + DC), fuse, acrylic, foam, screws  
- [ ] Crimp DTP06-2S on each HLG DC · HS seal · ≤4 ft  
- [ ] Mount HLGs shaded · FG earthed · `F9` **20 A** seated  
- [ ] Measure SOL current @ 12 V before loading all channels  
