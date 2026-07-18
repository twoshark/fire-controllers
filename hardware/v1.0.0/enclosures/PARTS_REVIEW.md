# Parts review — necessity · cost · function

Cart truth: [`SHOPPING_LIST.md`](SHOPPING_LIST.md). Reviewed **2026-07-18**.  
**Scope lock:** sign-output **5 SOL** · mp-output **3 SOL** — **no** 8-channel expand buy.

Confidence: **high** = keep · **med** = keep with note · **cut** = remove/reduce.

| # | Item | Ext | Necessary? | Function | Verdict |
| ---: | --- | ---: | --- | --- | --- |
| 1 | RS-15-12 ×2 | $17.20 | Yes | 12 V for input electronics | **KEEP** |
| 2 | HLG-185H-12 ×2 | $114.80 | Yes | Outdoor 12 V / 13 A · day-1 peaks | **KEEP** (was 240 for expand) |
| 3 | bociloy 1" hinges 10pk | $5.29 | Yes | Internal lid hinge | **KEEP** |
| 4 | M3 heat-set 100 | $10.90 | Yes | Latches + RS-15 | **KEEP** (overpack OK) |
| 5 | M2 heat-set 100 | $11.50 | Yes | PCB + hinge leaves | **KEEP** |
| 6 | DTP kit ×**2** | $18.00 | Yes | 2× HLG↔box 12 V | **KEEP** (was 4 spare) |
| 7 | DT kit ×**8** | $32.00 | Yes | sign 5 + mp 3 | **KEEP** (was 16 expand) |
| 8 | KWANGIL RS-485 100 ft | $138.99 | Yes for 30+50 ft | Full-duplex 120 Ω STP | **KEEP** |
| 9 | C13–C14 6 ft 2pk | $11.99 | Yes | Input mains cords | **KEEP** |
| 10 | SJTW 6 ft ×2 | $21.10 | Yes | HLG AC outdoors | **KEEP** |
| 11 | 12 AWG 25 ft | $24.99 | Partial | Only ~8 ft needed | **KEEP*** · overbuy length |
| 12 | EG STARTS ×2 | $23.98 | Yes | Arcade inputs | **KEEP** |
| 13 | M12 5-pin 4-set | $22.39 | Yes | 2 RS-485 runs (panel+field) | **KEEP** |
| 14 | M12 dust caps | $12.69 | Yes (locked) | Unmated outdoor | **KEEP** |
| 15 | C14 + cover 2pk | $5.99 | Yes | Input AC inlet | **KEEP** |
| 16 | KCD4 IP65 ×2 | $2.40 | Yes | Input POWER | **KEEP** |
| 17 | Wirefy HS 200pc | $14.99 | Yes | Outdoor HLG AC + DC re-term | **KEEP*** |
| 18 | ATO 20 A 20pc | $4.39 | Yes | `F9` ×2 + spare | **KEEP** |
| 19 | Acrylic 8×10 2pk | $6.99 | Yes | LED windows | **KEEP** |
| 20 | Foam tape 75 ft | $13.79 | Yes | Lid gasket | **KEEP*** |
| 21 | M3×6 screws 100 | $9.95 | Partial | RS-15 / latches | **KEEP** |
| 22 | M2 assort 200 | $9.99 | Yes | PCB + hinge screws | **KEEP** |

\*Overbuy still cheaper than a second specialty order.

## Cuts this pass (no 8-ch expand)

| Cut | Δ | Why |
| --- | ---: | --- |
| DT 16→**8** | −$32 | Only SOL0..4 + SOL0..2 |
| DTP 4→**2** | −$18 | One kit = one full power link |
| HLG-240→**185** | −$27.80 | 13 A enough for 10 A / 6 A peaks |
| Free-ship padding | — | Cart under $100; pay Lowdoller ship |

## Do not cut

| Item | Why |
| --- | --- |
| HLG / DTP / DT / M12 | Outdoor interfaces for day-1 |
| 2-pair 120 Ω STP | Full-duplex @ 30–50 ft |
| Heat-sets | Repeated lid open + PCB service |
| M12 caps | Unmated outdoor |

## Not expand-related (left alone)

| Item | Note |
| --- | --- |
| Output PCB (8 FETs) | Hardware still 8-ch; enclosure only panels **5** / **3** |
| M12 4-set | Needed for **2** RS-485 links, not SOL count |
| RS-485 100 ft | Run length, not channel count |
