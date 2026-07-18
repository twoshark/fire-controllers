# Parts review — session audit

Cart truth: [`SHOPPING_LIST.md`](SHOPPING_LIST.md). Audited **2026-07-18** against this session’s locks.

## Session criteria

| # | Criterion | Status |
| ---: | --- | --- |
| 1 | Real product URL + listed USD on every buy line | **PASS** |
| 2 | Cost-disciplined; keep function | **PASS** · est. **~$469** |
| 3 | RS-485 runs ~30 ft + ~50 ft → **100 ft** 120 Ω 2-pr STP | **PASS** · KWANGIL (not CAT5) |
| 4 | Outdoor PSU **HLG-240H-12** · **16 A** headroom | **PASS** |
| 5 | M12 dust caps kept | **PASS** |
| 6 | No 8-ch expand buy — panels **sign 5** + **mp 3** | **PASS** · DT ×8 |
| 7 | DTP = one kit per power link (not 4) | **PASS** · DTP ×2 |
| 8 | Owned stock not re-bought | **PASS** · see below |
| 9 | C13–C14 ≠ outdoor HLG AC (SJTW still bought) | **PASS** |
| 10 | Inside boxes: screw/solder/butt · no DryConn | **PASS** |
| 11 | Outdoor HLG AC + DC re-term: adhesive HS | **PASS** · **owned** |
| 12 | Lid hinges Amazon 1" · CAD M2 bosses | **PASS** · [`MOUNTING.md`](MOUNTING.md) |

## Cart + owned

| # | Item | Ext | Function | Verdict |
| ---: | --- | ---: | --- | --- |
| 1 | RS-15-12 ×2 | $17.20 | Input 12 V electronics | **KEEP** |
| 2 | HLG-240H-12 ×2 | $142.60 | Outdoor 12 V / 16 A | **KEEP** (locked) |
| 3 | bociloy 1" hinges 10pk | $5.29 | Internal lids (use 8) | **KEEP** |
| 4 | M2/M3 heat-sets | — | PCB · hinge · latch · RS-15 | **OWNED** |
| 5 | M2/M3 screws | — | same | **OWNED** |
| 6 | DTP ×2 | $18.00 | 2× HLG↔box 12 V | **KEEP** |
| 7 | DT ×8 | $32.00 | SOL 5+3 | **KEEP** |
| 8 | KWANGIL RS-485 100 ft | $138.99 | Full-duplex 120 Ω | **KEEP** |
| 9 | C13–C14 | — | Input IEC mains | **OWNED** |
| 10 | SJTW 6 ft ×2 | $21.10 | HLG AC outdoor NEMA | **KEEP** |
| 11 | 12 AWG 25 ft | $24.99 | HLG DC ≤4 ft ×2 | **KEEP*** (~8 ft needed) |
| 12 | EG STARTS | — | Arcade lids | **OWNED** |
| 13 | M12 5-pin 4-set | $22.39 | 2 RS-485 runs | **KEEP** |
| 14 | M12 dust caps | $12.69 | Unmated outdoor | **KEEP** (locked) |
| 15 | C14 + cover 2pk | $5.99 | Input inlets | **KEEP** |
| 16 | KCD4 IP65 ×2 | $2.40 | Input POWER | **KEEP** |
| 17 | Adhesive heatshrink | — | HLG AC + DC re-term | **OWNED** |
| 18 | ATO 20 A 20pc | $4.39 | `F9` ×2 + spare | **KEEP*** |
| 19 | Acrylic 8×10 2pk | $6.99 | LED windows | **KEEP** |
| 20 | Foam tape 75 ft | $13.79 | Lid gaskets | **KEEP*** |

\*Cheapest break / length overbuy; still buy once.

## Already cut this session

| Cut | Why |
| --- | --- |
| DT 16→8 · DTP 4→2 | Day-1 channels only |
| HLG-185 (brief) → back to **240** | User wants 16 A |
| CNC Kitchen inserts · screw packs | Owned |
| C13–C14 · EG STARTS · Wirefy HS | Owned |
| Quickcar · Belden-by-ft · KHA-25C · DryConn · GATSUN | Cheaper/equal or wrong |

## Do not cut further (without new info)

| Item | Why |
| --- | --- |
| HLG-240 / DTP / DT / M12+caps | Outdoor interfaces |
| KWANGIL 2-pr 120 Ω | Spec cable @ 30–50 ft |
| SJTW | Outdoor NEMA ≠ monitor IEC |
| bociloy hinges | CAD locked |
| C14 · KCD4 · F9 · acrylic · foam | Required assembly |

## Optional later (not in cart)

| Idea | Δ | Note |
| --- | ---: | --- |
| Stage 50 ft RS-485 first | −~$60 | Second order for long run |
| Drop SJTW if you have outdoor 16/3 | −$21 | Confirm NEMA outdoor, not C13 |
| TE flange DT/DTP | +$40–55 | Better panel seal |
| +8 DT later | +$32 | Unused PCB channels |
