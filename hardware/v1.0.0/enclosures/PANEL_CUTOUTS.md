# Panel cutouts & as-built dimensions

Sources: v1.0.0 EasyEDA PnP exports, Mean Well / Powerwerx / TE / IEC datasheets for selected cart parts ([`SHOPPING_LIST.md`](SHOPPING_LIST.md)).

All dimensions **mm** unless noted. Hole centers measured from **outer bottom** of enclosure unless noted.

---

## PCB keep-outs (from PnP midpoints + terminal body margin)

v1.0.0 boards have **no mounting holes** — use corner clips / 3D printed standoffs. Keep-out = part-center span + ~6 mm each side for screw-terminal / USB bodies.

| Board | PnP center span | Recommended floor keep-out | LED block (PnP) |
| --- | ---: | ---: | --- |
| Input PCB | 74.1 × 71.0 | **86 × 83** | `LED1`…`LED10` on edge X≈79.3; Y span **31.3**; pitch ≈**3.5** |
| Output PCB | 113.4 × 101.9 | **126 × 114** | `PWR`/`LINK`/`CH0`…`CH7` on edge X≈117.2; Y span **31.5**; pitch ≈**3.5** |
| Buttons daughter | diodes only ~45 span | flying leads / tiny board | — |

### Input connector mids (PnP, mm)

| Ref | Mid X | Mid Y | Notes |
| --- | ---: | ---: | --- |
| `J1` | 5.6 | −12.8 | 12 V in · face left edge |
| `J3` | 5.5 | −24.5 | switch GND |
| `J2b` | 6.0 | −40.8 | CH4..7 |
| `J2a` | 6.0 | −64.1 | CH0..3 |
| `CN2` | 51.8 | −5.2 | RS-485 · top edge |
| `J5` USB-C | 48.9 | −73.8 | bottom edge · open lid |
| `J6` SWD | 63.5 | −74.3 | bottom edge |
| `SW1`/`SW2` | ≈77.7 | −64 / −57 | on-board RESET/BOOT |

Orient PCB so LED edge (`X≈79`) faces the **LED window** wall.

### Output connector mids (PnP, mm)

| Ref | Mid X | Mid Y | Notes |
| --- | ---: | ---: | --- |
| `J1` | 5.5 | −19.9 | 12 V in |
| `F9` | 10.5 | −5.5 | ATO fuse · access via open lid |
| `J6` | 40.3 | −5.2 | load +12 V (both poles) |
| `J5a` | 60.6 | −5.3 | OUT0..3 |
| `J5b` | 90.0 | −5.2 | OUT4..7 |
| `J2` | 118.6 | −33.7 | RS-485 · right edge |
| `J7` USB-C | 70.0 | −107.1 | bottom · open lid |
| LEDs | ≈117.2 | −85.2…−53.7 | face LED window |

Orient so LED edge faces the **LED window** wall; `J5`/`J6` toward SOL side.

---

## Panel hardware cutouts (selected parts)

| Feature | Part | Cutout | Notes |
| --- | --- | --- | --- |
| POWER rocker | KCD4 DPST (lean) | **30.0 × 22.0** snap-in | Body ~31×25; 20 A / 125 VAC |
| AC inlet | IEC C14 panel (Amazon pack) | **27.5 × 20.0** snap-in *or* flange **27 × 19** + 2× Ø3.2 @ **42.5** pitch | Verify which style in pack before cutting |
| RS-485 | M12-5 panel (PG9 / M16) | **Ø16.2** (PG9) or **Ø16.1** (M16×1.5) | Flat for anti-rotation if keyed; panel ≤4.5 typical |
| 12 V | PanelPole2 | **Ø28.6** (1-1/8″) | Rear nut; housing depth behind panel **22.2** (7/8″) excl. wire |
| SOL | DT 2-pin (lean free-hanging kit) | Print pocket **≈16 × 18** + retention lip | Housing ~15×17; 13 A; 14–18 AWG |
| SOL (OEM option) | DT04-2P-**L012** flange | **≈22.5 × 17.2** + 2× M4 | Confirm TE customer drawing; max panel 6.35 |
| Arcade | EG STARTS `5/SJX-5C-BUT` | **Ø28** with included adapter ring (verify) | Equivalent FILN triangle often **Ø24**; measure ring before CAD freeze |
| LED window | — | **40 × 10** | Covers 10× LED row (~31 mm) + margin |
| Fan | 60 mm 12 V | **Ø57** airflow + 4× **Ø3.5** on **50 × 50** | Frame 60×60; thickness 15–25 |
| Fan filter/grill | 60 mm | match fan hole pattern | |

### Fixed heights / pitches (all boxes)

| Dim | Value |
| --- | ---: |
| PanelPole center height (outer bottom → hole CL) | **40** |
| PanelPole horizontal | centered on short-end mate face |
| SOL connector pitch (center-to-center) | **≥25** |
| Arcade button pitch (sign hex) | **≥45** |
| Arcade button pitch (mp triangle) | **≥50** |
| Side-wall feature clearance (edge of cutout → corner) | **≥12** |
| Wall thickness (printed) | **2.5–3.0** (nuts/rockers) |

---

## PSU mechanical (Mean Well datasheets)

| PSU | L × W × H | Mount | Clearance |
| --- | ---: | --- | --- |
| RS-15-12 | **62.5 × 51 × 28** | 2× M3 (L≤3–4) | Terminals + trim side; convection |
| LRS-200-12 | **215 × 115 × 30** | bottom / side 4× M4 (L≤5) | Al spreader **215 × 115 × 3**; fans on long walls |

LRS case drawing (typical): length-wise mount hole span **150**; end offset **32.5** — verify on unit before drilling plate.

---

## Cable length assumptions

| Cable | Qty | Length | Build |
| --- | ---: | ---: | --- |
| IEC C13 → C14 (mains) | **4** | **6 ft** (1.8 m) | One per AC box: both inputs + both power boxes |
| 12 V Powerpole jumper | **2** | **≤ 4 ft** (1.2 m) | sign-power↔sign-output · mp-power↔mp-output |
| RS-485 M12 field | 2 | site-run | Not length-capped here |
| SOL DT field | 8 ch | site-run | — |

Wire gauges inside boxes: AC **18 AWG** · 12 V link **12 AWG** · SOL **18 AWG** · RS-485 **22–24 AWG**.
