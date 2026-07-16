# Enclosure parts BOM (corrected)

Prices are **DigiKey qty‑1 USD** captured **2026-07-15** from live DigiKey product pages (or Findchips mirror for IRM). Re-check the linked page before ordering — DigiKey changes price/stock.

**Locked decisions:** AC-hot POWER; true IP67/IP68 mains; arcade buttons are yours; unused channels open.

---

## Why the old BOM was wrong

| Claim | Reality |
| --- | --- |
| “Bulgin `PX0580` = IP68 power inlet” | **False.** `PX0580` is a normal **IEC C14** flange inlet (not Buccaneer sealed). That link/choice was incorrect. |
| “~$15–30 / search DigiKey” | Vague because many rows were **category searches**, not buyable SKUs — so price could not be fixed. |
| Binder USA product page for M12 | Manufacturer page, not a DigiKey cart line — price unknown from that link alone. |
| “USB-C IP67 bulkhead” with only a search string | Not a part number — uncertainty was self-inflicted. |

This file replaces those with **exact MPNs + DigiKey product URLs + qty‑1 prices + why chosen**.

---

## A. Power — AC-DC

| Qty | Role | Why this part | MPN | DigiKey | Qty‑1 | Conf. |
| ---: | --- | --- | --- | --- | ---: | --- |
| 1 | 12 V rail for PCB + arcade LEDs | Project standard; 12 V / **1.25 A / 15 W**; Class II; 85–305 VAC; small; cheap enough. Measured PCB ≤0.12 A; arcade LEDs must stay well under remaining ~1 A. | **IRM-15-12** | [1866-3033-ND](https://www.digikey.com/en/products/detail/mean-well-usa-inc/IRM-15-12/7704553) · [MW PDF](https://www.meanwell.com/Upload/PDF/IRM-15/IRM-15-SPEC.PDF) | **$8.60** (Findchips DigiKey row; page also shows ~$10.63 in some mirrors — use cart) | High |

**Rejected alternate:** larger LRS/enclosed supplies — overkill cost/size for input box. **HDR-15-12** only if you refuse soldering IRM pins (screw terminals; same power class).

**Wire:** 18 AWG 300 V to AC pins; 20 AWG to `+V`/`−V`.

---

## B. Power — IP68 panel inlet (CORRECTED)

Must be sealed when mated. Use **Bulgin Buccaneer 400** (IP68/IP69K when mated), **not** PX0580.

| Qty | Role | Why | MPN | DigiKey | Qty‑1 | Conf. |
| ---: | --- | --- | --- | --- | ---: | --- |
| 1 | Panel chassis plug (pins toward outside, like C14) | Front-panel single-hole; IP68 mated; 3 poles L/N/E; 8 A contacts class for 2/3-pole | **PX0412/03P** | [708-1059-ND](https://www.digikey.com/en/products/detail/bulgin/PX0412-03P/1625824) | **$11.56** | High |
| 3 | Pin contacts for panel (solder) | Bodies ship **without** contacts | **SA3350/1** | [SA3350/1](https://www.digikey.com/en/products/detail/bulgin/SA3350-1/1625857) | ~**$0.93–1.50** ea (use page) | High |
| 1 | Flex cable connector (sockets) | Mates PX0412; pick gland for your cord OD | **PX0410/03S/5560** (5.5–6.0 mm) | [PX0410-03S-5560](https://www.digikey.com/en/products/detail/bulgin/PX0410-03S-5560/1625803) | **$13.32** | High |
| 3 | Socket contacts for flex (solder) | Required | **SA3349/1** | [SA3349/1](https://www.digikey.com/en/products/detail/bulgin/SA3349-1/1625856) | ~**$0.93–1.50** ea | High |
| 1 | Sealing cap (unmated panel) | IP when cord unplugged | **PX0484** | DigiKey search `PX0484` Bulgin | ~**$3–5** | High |
| 1 | Cord | SJTW 18/3 outdoor, OD matching gland (**5560** ⇒ ~5.5–6.0 mm) | — | any electrical supplier | ~**$1/ft** | High |
| 1 | Fuse on AC hot | Protect IRM; 1 A slow for ~0.35 A@115 V inrush | **BK/GMC-1-R** (5×20 mm 1 A T) | DigiKey `BK/GMC-1-R` | ~**$1** | High |
| 1 | Fuse holder | Prefer in-line IP67 holder **or** internal holder (inside sealed box) | DigiKey IP67 in-line fuse holder | search `fuse holder IP67 inline` | ~**$5–12** | Med |

**Gland size:** If your cord OD differs, swap the `PX0410/03S/xxxx` suffix (3035…6570). DigiKey lists each; prices ~$9.70–$13.36.

**PE / earth:** 3rd pole is PE. IRM-15 is Class II (no PE pin). On a plastic print: land PE on a marked PE terminal block bonded only if you add a metal plate; otherwise terminate PE in the inlet and do not float a touchable metal part. Conf. **high** for plastic box with no exposed metal.

**Rejected:** Schurter/open C14, and **PX0580** (wrong family).

---

## C. RS-485 panel (M12)

| Qty | Role | Why | MPN | DigiKey / link | Qty‑1 | Conf. |
| ---: | --- | --- | --- | --- | ---: | --- |
| 1 | Panel female M12 A-coded 8-pin + 0.5 m leads | Commodity industrial; IP67; 8 pins map to `CN2` (6 used, 2 NC); A-code; PG9 panel | **Phoenix `1513758`** (`SACC-E-M12FS-8CON-PG9/0.5`) | DigiKey search MPN `1513758` (Phoenix PDF on DigiKey CDN) | typically **$25–40** (confirm cart) | High |
| 1 | Flat nut if required | PG9 finish | **1504084** (`SACC-E-MU-PG9`) | DigiKey with 1513758 accessories | ~**$1–3** | High |
| AR | Field cable | Spec cable for Hotline | **Belden 9842** (2×24 AWG pair, shielded, 120 Ω) | DigiKey search `9842` Belden (sold by foot) | ~**$2–4/ft** | High |
| 1 | Cordset or field-attach M12 male 8-pin A | Mate to panel | DigiKey `M12 A 8 male cordset` | pick length | ~**$15–35** | High |
| 1 | M12 protective cap | IP when unmated | Matching Phoenix/TE cap | with series | ~**$3–6** | High |

**Why M12 not Amphenol AT here:** Field patch cords are cheaper/faster for M12; AT remains valid if you already standardize AT across the show. Conf. **high** either way if one family is used end-to-end.

**Wire gauge on pigtail:** 24 AWG (0.25 mm²) — adequate for RS-485.

---

## D. USB DFU (IP67)

Board `J5` is USB-C. Need sealed panel USB that can reach `J5`.

| Qty | Role | Why | MPN | DigiKey | Qty‑1 | Conf. |
| ---: | --- | --- | --- | --- | ---: | --- |
| 1 | IP67 USB‑C panel receptacle | Verified DigiKey USB‑C IP67, die-cast, panel gasket | **MUSBR-4593-M0** | [MUSBR-4593-M0-ND](https://www.digikey.com/en/products/detail/amphenol-cs-commercial-products/MUSBR-4593-M0/7386150) | **$21.33** | High |

**Wiring reality check (important):** `MUSBR-4593-M0` is a **panel/PCB right-angle receptacle**, not a female↔male bulkhead coupler. You must either:

1. Mount it in the panel and **wire USB2 D+/D−/VBUS/GND** to the input PCB USB nets / `J5` pads with a short harness, **or**
2. Add a tiny adapter PCB.

Do **not** expect a USB-C plug on the inside without a custom pigtail.

**Rejected:** `MUSBRA11145` — that DigiKey sibling is **USB-A**, not USB-C ($11.43) — wrong connector for `J5` unless you accept A→C adapter inside (extra failure point).

**Cap:** use Amphenol MUSBR dust cover accessory from the same series page (~$2–5).

---

## E. RESET / BOOT (buy)

Arcade buttons cover CH/ALL/POWER. Still need sealed service buttons unless you also arcade these.

| Qty | Role | Why | Buy guidance | Qty‑1 target | Conf. |
| ---: | --- | --- | --- | ---: | --- |
| 2 | RESET + BOOT | Momentary NO to GND; IP67 front; 16 mm common | DigiKey filter: Pushbutton, Panel Mount, 16 mm, IP67, Momentary, SPST-NO — pick two colors (e.g. red RESET, blue BOOT). Example family on DigiKey: Bulgin MPI / anti-vandal 16 mm | **$4–12** ea | Med |

No single forced MPN here until you prefer illuminated vs not; electrical requirement is only **momentary NO**.

---

## F. Internal wire / hardware (fixed gauges)

| Use | Gauge / part | Why | DigiKey / source | Qty‑1 |
| --- | --- | --- | --- | ---: |
| AC L/N | **18 AWG** 300 V hook-up | Ampacity + IRM AC pins | DigiKey hook-up 18 AWG | ~$0.30/ft |
| 12 V + LED bus | **20 AWG** | IRM 1.25 A headroom | DigiKey | ~$0.25/ft |
| Switch NO / LED | **22–24 AWG** | Signal + LED tens of mA | DigiKey | ~$0.20/ft |
| NRST/BOOT | **26–28 AWG** | Low current, keep short | DigiKey | ~$0.15/ft |
| Ferrules | 18–22 AWG bootlace | Reliable in screw terminals | DigiKey ferrules | ~$0.10 ea |
| Lid gasket | 2–3 mm EPDM cord | Print IP strategy | McMaster gasket cord | ~$5–10 |
| M3 heat-sets | M3-0.5 | PCB/PSU mounts | DigiKey / Amazon kit | ~$3 |

---

## G. Cost roll-up per box (DigiKey-ish, excl. arcade + PCB + filament)

| Line | Qty‑1 USD |
| --- | ---: |
| IRM-15-12 | 8.60 |
| PX0412/03P + 3× pin contacts | ≈ 11.56 + 3×1.20 ≈ **15.20** |
| PX0410/03S/5560 + 3× socket contacts | ≈ 13.32 + 3×1.20 ≈ **16.90** |
| Cap + fuse + holder | ≈ 10 |
| M12 panel 1513758 + nut + cap | ≈ 35 |
| Belden 9842 (assume 25 ft shared later) | allocate **15** |
| M12 cordset | ≈ 25 |
| MUSBR-4593-M0 + cover | ≈ 24 |
| RESET + BOOT | ≈ 16 |
| Wire/ferrules/gasket/inserts | ≈ 20 |
| **Total hardware (approx.)** | **≈ $185** |

Sign and MP are the **same** buy list (arcade qty differs only).

---

## H. Selection rationale summary

| Decision | Why | Rejected |
| --- | --- | --- |
| IRM-15-12 | Spec’d, enough amps, cheap, small | Bigger Mean Well bricks |
| Buccaneer 400 (PX0412/PX0410) | Actual IP68 when mated; DigiKey stock; 3-pole | PX0580 (not sealed), open C14 |
| M12-8 A Phoenix pigtail | Matches `CN2`, IP67, stock industrial | DIY gland + loose wires |
| MUSBR-4593-M0 | Real DigiKey USB-C IP67 | Vague “search USB-C IP67”; USB-A MUSBR |
| Your arcade buttons | You already own them | Buying duplicate 22 mm metal switches |
| Unused CH open | You confirmed | Dummy loads / firmware masks |

---

## I. Still open (honest)

1. **Arcade LED current** — measure; BOM assumes LED not filament.
2. **POWER arcade AC rating** — contacts must be ≥5 A @ 125 VAC or use a separate AC-rated latch.
3. **MUSBR → J5 harness** — you must design the short USB wiring (pinout from Amphenol MUSBR datasheet).
4. **Phoenix 1513758 cart price** — DigiKey blocked automated fetch; confirm in cart (typically mid-$20s–$40).
5. **Daughter PCB** — still diodes-only; flying leads or rev with terminals.
