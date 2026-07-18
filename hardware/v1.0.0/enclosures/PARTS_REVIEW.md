# Parts review — necessity · cost · function

Cart truth: [`SHOPPING_LIST.md`](SHOPPING_LIST.md). Reviewed **2026-07-18**.  
**Scope lock:** sign-output **5 SOL** · mp-output **3 SOL** — **no** 8-channel expand buy.

Confidence: **high** = keep · **med** = keep with note · **cut** = remove/reduce.

| # | Item | Ext | Necessary? | Function | Verdict |
| ---: | --- | ---: | --- | --- | --- |
| 1 | RS-15-12 ×2 | $17.20 | Yes | 12 V for input electronics | **KEEP** |
| 2 | HLG-240H-12 ×2 | $142.60 | Yes | Outdoor 12 V / 16 A · day-1 peaks | **KEEP** |
| 3 | bociloy 1" hinges 10pk | $5.29 | Yes | Internal lid hinge | **KEEP** |
| 4 | M3 / M2 heat-sets | — | Yes | Latches · RS-15 · PCB · hinges | **OWNED** |
| 5 | M2 / M3 screws | — | Yes | same | **OWNED** |
| 6 | DTP kit ×**2** | $18.00 | Yes | 2× HLG↔box 12 V | **KEEP** |
| 7 | DT kit ×**8** | $32.00 | Yes | sign 5 + mp 3 | **KEEP** |
| 8 | KWANGIL RS-485 100 ft | $138.99 | Yes for 30+50 ft | Full-duplex 120 Ω STP | **KEEP** |
| 9 | C13–C14 cords | — | Yes | Input mains | **OWNED** |
| 10 | SJTW 6 ft ×2 | $21.10 | Yes | HLG AC outdoors (NEMA, not IEC) | **KEEP** |
| 11 | 12 AWG 25 ft | $24.99 | Partial | Only ~8 ft needed | **KEEP*** |
| 12 | EG STARTS ×2 | $23.98 | Yes | Arcade inputs | **KEEP** |
| 13 | M12 5-pin 4-set | $22.39 | Yes | 2 RS-485 runs | **KEEP** |
| 14 | M12 dust caps | $12.69 | Yes (locked) | Unmated outdoor | **KEEP** |
| 15 | C14 + cover 2pk | $5.99 | Yes | Input AC inlet | **KEEP** |
| 16 | KCD4 IP65 ×2 | $2.40 | Yes | Input POWER | **KEEP** |
| 17 | Wirefy HS 200pc | $14.99 | Yes | Outdoor HLG AC + DC re-term | **KEEP*** |
| 18 | ATO 20 A 20pc | $4.39 | Yes | `F9` ×2 + spare | **KEEP** |
| 19 | Acrylic 8×10 2pk | $6.99 | Yes | LED windows | **KEEP** |
| 20 | Foam tape 75 ft | $13.79 | Yes | Lid gasket | **KEEP*** |

\*Overbuy still cheaper than a second specialty order.

## Cuts applied

| Cut | Δ | Why |
| --- | ---: | --- |
| DT 16→**8** · DTP 4→**2** | −$50 | Day-1 5+3 panels only |
| HLG kept at **240** | — | 16 A headroom (user lock) |
| CNC Kitchen inserts | −$22.40 | Owned |
| M2/M3 screw packs | −$19.94 | Owned |
| C13–C14 2pk | −$11.99 | Owned PC/monitor cords |

## Do not cut

| Item | Why |
| --- | --- |
| HLG / DTP / DT / M12 | Outdoor interfaces for day-1 |
| SJTW (or outdoor NEMA cord) | HLG AC — different from C13–C14 |
| 2-pair 120 Ω STP | Full-duplex @ 30–50 ft |
| M12 caps | Unmated outdoor |

## Not expand-related (left alone)

| Item | Note |
| --- | --- |
| Output PCB (8 FETs) | Hardware still 8-ch; enclosure only panels **5** / **3** |
| M12 4-set | Needed for **2** RS-485 links |
| RS-485 100 ft | Run length, not channel count |
