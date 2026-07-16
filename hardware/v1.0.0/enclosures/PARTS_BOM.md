# Enclosure parts BOM (shared + per-box)

Target: cheapest credible **IP67** panel parts, typically ship ≤2 weeks from DigiKey / Mouser / Amazon US.

Prices are approximate USD (2026-07) one-off; verify stock before ordering.

**Decisions locked:** POWER switches **AC hot**; mains entry must be **true IP67** (no open C14/C13 face); channel/ALL/POWER operator buttons are **user-supplied arcade buttons** (not in this buy list); unused input channels stay **open**.

## Confidence legend

| Level | Meaning |
| --- | --- |
| **High** | Standard catalog part, clear datasheet, usual stock |
| **Med** | Generic / house-brand OK if reviews + IP marking match |
| **Low** | Avoid; listed only as last-resort alternate |

---

## A. Power conversion & AC entry (IP67 required)

| Qty (per box) | Item | Suggested P/N | Where | Est. | IP / notes | Conf. |
| --- | --- | ---: | --- | ---: | --- | --- |
| 1 | AC-DC 12 V 15 W | **Mean Well `IRM-15-12`** | [DigiKey](https://www.digikey.com/en/products/detail/mean-well-usa-inc/IRM-15-12/7704553) · [MW spec](https://www.meanwell.com/Upload/PDF/IRM-15/IRM-15-SPEC.PDF) | $8–12 | PCB-mount; solder 18–20 AWG flying leads to AC/L, AC/N, +V, −V | High |
| 1 | **IP67/IP68 panel power inlet** | **Bulgin Buccaneer `PX0580`** (or current Buccaneer 400/600 power inlet equiv.) | [Bulgin power](https://www.bulgin.com/products/power) · DigiKey search `PX0580` / `Buccaneer` | $15–30 | **Primary choice** — sealed appliance inlet; mate with matching Bulgin IP68 free plug + strain relief | High |
| 1 | Matching free plug + cable | Bulgin mate for above + SJTW 18/3 outdoor cord | DigiKey / Bulgin | $15–40 | Cord face stays sealed when mated; cap inlet when unplugged | High |
| 1 | Inline or holder fuse | 5×20 mm **1 A T** (slow) 250 V in sealed holder **or** in-line waterproof fuse holder | DigiKey `BK/GMC-1-R` + IP67 fuse holder | $3–8 | On AC hot before POWER switch | High |
| 1 | Optional: DIN/chassis PSU | Mean Well **`HDR-15-12`** | DigiKey | $12–15 | Screw terminals; same 12 V/1.25 A class | High |

**Rejected for this project:** Schurter/open **IEC C14** panel inlets — the C13 cord mating face is not IP67.

**AC wiring:** 18 AWG SJT/SJTW / hook-up, 300 V, black=hot, white=neutral. Keep AC ≥8 mm from low-voltage bundles; heat-shrink every joint.

**POWER switch:** Latching, contacts **≥5 A @ 125 VAC**, breaks **AC hot** after the fuse. LED (if present) still from +12 V after IRM. Use your arcade latching switch if it is AC-rated; otherwise buy a separate AC-rated latch (panel or internal).

---

## B. Operator controls

### Arcade buttons (user-supplied)

Channel, ALL FIRE, and (if used) POWER arcade buttons are **yours** — not purchased in this BOM.

Wiring contract (must match input PCB):

| Function | Switch type | Electrical |
| --- | --- | --- |
| Channel / ALL FIRE | Momentary **NO** | NO → channel net or `ALL_BUTTON_A`; COM → `GND` |
| Button lamps | 12 V LED preferred | LED+ → `+12V`; LED− → `GND` (measure mA) |
| POWER | Latching, **AC-rated** contacts | Pole on fused hot → IRM `AC/L` |

CAD: cut panel holes to **your** arcade button diameters (often 24–30 mm) — see [`CAD_NOTES.md`](CAD_NOTES.md).

### Still buy (service / DFU)

| Role | Type | Cutout | Suggested | Est. | Conf. |
| --- | --- | --- | ---: | --- | --- |
| RESET | Momentary NO, IP67 | **16 mm** (or arcade if preferred) | Stainless IP67 16 mm momentary | $3–8 | High |
| BOOT | Momentary NO, IP67, distinct color | **16 mm** | Same family, different color | $3–8 | High |

| Part | sign-input | mp-input |
| --- | ---: | ---: |
| Arcade CH buttons | 5 | 3 |
| Arcade ALL FIRE | 1 | 1 |
| Arcade / latch POWER (AC-rated) | 1 | 1 |
| RESET / BOOT | 1 / 1 | 1 / 1 |

**LED budget:** measure your arcade lamps. IRM-15-12 has 1.25 A; keep total 12 V load (PCB + all lamps) **&lt; 800 mA** for margin. Reject filament bulbs.

---

## C. External I/O connectors (panel) — IP67

| Qty | Function | Part | Link | Est. | Wire | Conf. |
| --- | --- | --- | ---: | --- | --- | --- |
| 1 | RS-485 to output box | **M12 A-coded 8-pin female panel mount**, rear mount, pigtails | Binder / Amphenol / Phoenix — DigiKey `M12 A 8 panel` e.g. [Binder IP68 8-pin panel](https://www.binder-usa.com/us-en/products/automation-technology/m12-a/76-2632-1111-00008-0200-m12-a-female-panel-mount-connector-8-unshielded-single-wires-ip68-ul-m16x15-front-fastened-stainless-steel) | $12–25 | 24 AWG → `CN2` | High |
| 1 | Mating field cable | Belden **9842** (or equiv.) + M12 8-pin male cordset | DigiKey | cable + $15–30 | Twisted pairs; shield → pin 6 | High |
| 1 | USB DFU / debug | **IP67 USB-C bulkhead** | DigiKey/Mouser `USB-C IP67 panel` | $15–35 | Short pigtail to `J5` | Med |
| 1 | Dust/water cap | Cap for M12 + USB + power inlet when unused | With connector series | $5–15 | Required for IP when unmated | High |
| 0–2 | Spare penetrations | Nylon **M16/M12 IP68 glands** + blanking plugs | DigiKey | $2–4 | Cap unused | High |

---

## D. Internal interconnect

| Use | Gauge | Type | Notes |
| --- | --- | --- | --- |
| AC L/N (inlet → fuse → POWER → IRM) | **18 AWG** | 300 V hook-up / SJTW | Heat-shrink every joint |
| 12 V PSU → board `J1`, LED bus | **20 AWG** | red/black | Star from PSU; common GND |
| Arcade switch NO → channel / ALL | **22–24 AWG** | hook-up | Switch-to-GND; unused CH left open |
| Arcade LED +/− | **24 AWG** | hook-up | Confirm polarity / current |
| RS-485 panel → `CN2` | **24 AWG** | twisted pairs | Match TX± / RX± |
| NRST / BOOT0 / GND | **26–28 AWG** | PTFE/silicone | Short; away from AC |
| USB bulkhead → `J5` | USB 2.0 pigtail | prefab | Prefer not DIY differentials |

Ferrules: **22–18 AWG** bootlace for `J1`/`J2`/`CN2`.

---

## E. Mechanical / print / seal

| Qty | Item | Notes | Est. | Conf. |
| --- | --- | ---: | --- | --- |
| 1 | 3D-printed shell + lid | [`CAD_NOTES.md`](CAD_NOTES.md); cutouts sized for **arcade** buttons | filament | — |
| 1 | Lid gasket | 2–3 mm EPDM/silicone cord in groove | $5–10 | High |
| 4–6 | M3 heat-set inserts + screws | PCB + PSU standoffs | $3 | High |
| 1 | LED window / light pipes | Over input-PCB LED row | $2–5 | Med |
| 1 | PSU insulating carrier | Creepage clearances for AC | — | High |
| AR | RTV around panel nuts | Backup seal | — | High |

---

## F. Rough cost per box (excludes arcade buttons & PCBs)

| Tier | sign-input | mp-input |
| --- | ---: | ---: |
| IP67 (Bulgin power + M12 + USB-C + RESET/BOOT + seals) | ~$80–160 | ~$80–160 |

---

## Rejected options

| Option | Why rejected |
| --- | --- |
| Open IEC C14 / C13 inlet | Not IP67 at the cord face |
| Filament 12 V pilot lamps | Can starve IRM-15 headroom |
| Board USB-C cutout with no bulkhead | Not waterproof |
| Power switch on 12 V only (IEC always live) | Rejected — AC hot latch preferred and confirmed |
