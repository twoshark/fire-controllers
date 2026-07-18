# Connector ratings

## Load assumptions (verify before field)

| Load | Assumed | Notes |
| --- | ---: | --- |
| Per SOL / relay | **≤1.5 A continuous** hot · **2 A** short duty | Confirm MPN + measure hold/inrush @ 12 V |
| sign-output (5 ch) | **≤7.5 A** cont · **10 A** peak | Must stay in HLG CV region |
| mp-output (3 ch) | **≤4.5 A** cont · **6 A** peak | |
| PCB absolute ceiling | 8×2 A = 16 A | MOSFET + wiring; not PTC continuous |

Channel PTC `1812L200/16GR`: **2.0 A hold @ ~20 °C** — derates to ~**1.4–1.6 A @ 50–60 °C**. Do **not** plan continuous 2 A/ch in a hot sealed box. Field continuous budget **~1.2–1.5 A/ch** unless measured otherwise.

---

## Interface ratings

| Interface | Part | Rating | Load | Seal | Joint | Verdict |
| --- | --- | --- | ---: | --- | --- | --- |
| 12 V | **DTP04-2P** / **DTP06-2S** (+ size-12 contacts, wedges) | **25 A** · 10–14 AWG | ≤10 / 6 A peak | IP67 **mated face** | Crimp | **OK** |
| SOL | **DT04-2P** / **DT06-2S** (+ size-16 contacts, wedges) | 13 A · 14–18 AWG | ≤1.5–2 A | IP67 **mated face** | Crimp | **OK** |
| RS-485 | M12-5 A-code | signal · ≤22–24 AWG | ≪1 A | IP67 mated | Solder/field | **OK** |
| POWER (inputs) | KCD4 **DPST** + silicone boot · **125 VAC** | 16–20 A class | ~0.2 A AC | splash w/ boot | QC | **OK on AC only** |
| AC inlet (inputs) | IEC C14 + cover | 10–15 A | ≪1 A | cover when unmated | QC | **OK** · not IP67 |
| Outdoor PSU | HLG-185H-12 | 13 A @ 12 V | ≤10 / 6 A peak | **IP67 unit** | factory + sealed DTP re-term | **OK** |

### Preferred TE Deutsch (over Amazon assortments)

| Role | Receptacle (box) | Plug (cable) | Notes |
| --- | --- | --- | --- |
| 12 V | DTP04-2P (flange `-L012` if panel) | DTP06-2S | Size-12 contacts · wedges · seals |
| SOL | DT04-2P (flange `-L012` if panel) | DT06-2S | Size-16 contacts · wedges · cavity plugs |

Amazon “10-set” kits are acceptable only if they include **correct contacts, seals, and wedges**. Prefer Digi-Key / Mouser TE line items when possible. Flanged receptacles = [`COST_OPTIONS.md`](COST_OPTIONS.md) **C**.

---

## PCB / fuse notes

| Item | Note |
| --- | --- |
| `J1` / `J6` KF128-7.5 | KEFA C474954 (listed 24 A; treat conservatively). Parallel **both** `J6` poles |
| SOL +12 V | **Star** from `J6` · 12 AWG stub → **18 AWG/ch** |
| `F9` ATO | Field fuse **20 A** (was 25 A). Hard-fault / reverse; **HLG CC (~13 A)** is continuous overload limit |
| Kill 12 V | Unplug HLG AC cord |

No panel USB / RESET / BOOT. Seal detail: [`SEALING.md`](SEALING.md).
