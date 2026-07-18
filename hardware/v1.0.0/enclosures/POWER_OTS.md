# Outdoor 12 V supply

120 VAC → 12 V DC for each output box: Mean Well **HLG-240H-12** (OTS, IP67).

## Loads

Assumes ≤**1.5 A continuous** / ch hot (PTC derate) · **2 A** short duty. **Measure SOL MPN** before freeze.

| Load | Current @ 12 V | Power |
| --- | ---: | ---: |
| sign-output (**5× SOL**) | **≤7.5 A** cont · **10 A** peak | ≤120 W |
| mp-output (**3× SOL**) | **≤4.5 A** cont · **6 A** peak | ≤72 W |
| PCB ceiling (8×2 A) | 16 A | 192 W |

Day-1 cart panels **5** / **3** DT only; PSU rated for full board if channels are added later.

## Part

| Qty | MPN | Rating | IP | Unit | Ext | Buy |
| ---: | --- | --- | --- | ---: | ---: | --- |
| **2** | **HLG-240H-12** | **12 V · 16 A · 192 W** · 90–305 VAC | **IP67** | $71.30 | **$142.60** | [Bravo](https://www.bravoelectro.com/hlg-240h-12.html) |

Fanless · metal case · Class I (earth FG) · CV+CC · 7 yr warranty · **244 × 68 × 39 mm**.

| Alt | When |
| --- | --- |
| HLG-185H-12 ×2 · 13 A · $57.40 | Cost cut; OK for day-1 10 A / 6 A only |
| HLG-240H (sign) + HLG-185H (mp) | Mix |
| XLG-200-12-A · 16 A · IP67 | Same class |

Use Mean Well (or equivalent OEM). Avoid no-name IP67 bricks.

## Hookup

```text
120 VAC (6 ft SJTW)
    → waterproof splice → HLG AC L/N/FG
HLG DC +/−
    → 12 AWG ≤4 ft → DTP06-2S → DTP04-2P on output box → J1
         (adhesive heatshrink/gel over pigtail re-term)
```

| Rule | Detail |
| --- | --- |
| Mount | To structure; **shade** (avoid direct sun) |
| Earth | Connect FG |
| Output V | Fixed ~12 V (blank type) |
| Kill | Unplug AC; optional in-line IP67 switch on cord |
| Loads | Board flybacks handle SOL; run in **CV** region |
| Fuse | Board `F9` **20 A** ATO; HLG CC (~**16 A**) limits continuous overload |

## System

```text
sign-input ──RS-485──► sign-output ◄──DTP── HLG-240H-12 ◄── 120 VAC
mp-input   ──RS-485──► mp-output   ◄──DTP── HLG-240H-12 ◄── 120 VAC
```

Printed boxes: **4** (2 input + 2 output).
