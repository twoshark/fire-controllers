# Cost options → under $300

Six boxes. Revised cart (~$256): [`SHOPPING_LIST.md`](SHOPPING_LIST.md).

### Locked changes (vs PanelPole lean)

| Change | Why |
| --- | --- |
| Drop PanelPole2 ×4 (−$80) | Not IP67 when mated; too expensive |
| Add DTP 2-pin 10-set (~+$28) | **25 A · IP67 mated** · 12 AWG |
| Drop 12 V KCD4 on output boxes | AC-only rocker must not switch 10 A DC |
| Add boots / M12 caps / C14 covers (~+$24) | Dust/rain on remaining weak points |
| Keep DT for SOL · M12 for RS-485 · Bravo PSUs | Correct families |

---

## Recommended cart (~$282)

| Keep | Ext |
| --- | ---: |
| Bravo 2× RS-15 + 2× LRS-200 | $69.20 |
| Amazon (connectors, seal extras, fans, cords, inserts, lens/gasket) | ~$213 |
| **Sum** | **~$282** |

---

## Option menu

| ID | Change | Δ | Tradeoff |
| --- | --- | ---: | --- |
| **A** | HangTon USB on 4 controllers | +$29 | DFU without opening |
| **B** | UL AC rocker (WRG32) on 4 AC boxes | +$9 | Still AC-only — OK on AC path only |
| **C** | OEM DigiKey AT flange for SOL | +$40–55 | Better panel seal than printed pocket |
| **D** | 4 boxes only (drop power shells) | −~$160 | Incomplete system |
| **E** | Printed Powerpole instead of DTP | −~$20 vs DTP | **Reject for outdoor** — unsealed when mated |
| **F** | Skip fans | −$22 | Hotter LRS; more sealable power box |
| **G** | One shared LRS for both | −~$40 | 16 A on 17 A — **reject** without derate |

---

## Do not cut

| Item | Why |
| --- | --- |
| LRS-200-12 / RS-15-12 | Load locked |
| DTP (or ATP) for 12 V bus | Current + IP67 |
| DT/AT for SOL | 2 A outdoor |
| M12 for RS-485 | Signal / outdoor |
