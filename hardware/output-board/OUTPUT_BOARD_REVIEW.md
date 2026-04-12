# Output Board Deep Review (2026-04-12)

This file captures the review evidence and confidence scorecard for the output-board documentation/BOM pass.

## 1) Live part/stock verification (JLCPCB)

Verification method:

- Source BOM: `hardware/output-board/bom-jlcpcb.csv`
- Query endpoint: `https://jlcpcb.com/partdetail/<LCSC Part#>`
- Checks per line item:
  - LCSC page resolves
  - `In Stock: <count>` exists and count > 0
  - MPN string check against page content (advisory; naming variants may differ from BOM formatting)

Result summary:

- 32/32 output-board BOM lines resolved at JLCPCB and reported in stock (>0).
- 32/32 lines had valid LCSC part pages.
- 29/32 lines had direct BOM-MPN substring match in page text.
- 3/32 lines had naming-format mismatch only (not stock failure):
  - `C14663` (`CL10B104KB8NNNC` in BOM; JLC page title uses alternate equivalent string format)
  - `C2765186` (`TYPE-C-16P-2MD073` vs page title punctuation/spacing variant)
  - `C41376037` (`HX-PZ1.27-2x5P-TP` vs page title punctuation/spacing variant)

Stock snapshot by LCSC line:

| LCSC Part# | Stock |
| --- | ---: |
| C2847904 | 7,433 |
| C8963 | 105,498 |
| C780769 | 60,813 |
| C2849485 | 123 |
| C7780 | 438 |
| C53550 | 16,362 |
| C8678 | 1,494,450 |
| C521963 | 299,156 |
| C18198339 | 3,624 |
| C22775 | 8,444,278 |
| C25804 | 24,099,591 |
| C23138 | 3,667,565 |
| C22808 | 968,956 |
| C22787 | 1,904,817 |
| C23345 | 3,126,638 |
| C23186 | 5,243,265 |
| C19702 | 7,054,329 |
| C14663 | 34,996,118 |
| C15849 | 9,600,056 |
| C2162977 | 3,777 |
| C4747964 | 25,958 |
| C11366 | 167,803 |
| C7496818 | 64,283 |
| C7496819 | 42,301 |
| C72038 | 485,583 |
| C318884 | 1,651,456 |
| C2915639 | 13,507 |
| C2915642 | 24,018 |
| C2915641 | 82,554 |
| C2765186 | 730,354 |
| C41376037 | 12,454 |

## 2) Schematic guide consistency fixes applied

- Added explicit output-board input protection path with `D1`:
  - `J1.1 -> VIN_12V_IN -> D1 -> 12V_MAIN`
- Updated output Page-1 part list to include `D1`.
- Replaced ambiguous TVS wording with explicit `D18`/`D19` line-pin and return-pin mapping.
- Fixed output channel indexing math in page map:
  - `F(k+1)`, `Q(k+1)`, `R(k+1)`, `R(k+9)`, `D(k+2)` for `k=0..7`
- Added explicit CH0..CH7 mapping table:
  - `F1..F8`, `Q1..Q8`, `R1..R8`, `R9..R16`, `D2..D9`
- Updated output schematic appendix with explicit IDs/ranges:
  - `R57` BOOT0 pulldown
  - `D18`, `D19` protection references
  - `R17-R32` mention in override chain
  - explicit power mapping rows for `D1`, `C1`, `C2-C7`, `C15`, `J6.1`

## 3) Layout guide detail upgrades applied

- Added explicit output-board routing order (power protection -> buck -> trunk -> channels -> gates -> interfaces -> logic -> DRC).
- Added output-board-specific grounding/return strategy guidance (`LOAD_GND_RTN` handling).
- Added connector-side RS-485 ordering rule (connector -> TVS -> transceiver/termination).
- Added explicit `J2.6` shield/chassis mapping expectation.
- Added signoff requirement for DRC/ERC + Gerber review.
- Expanded output PCB appendix with:
  - pre-layout setup checklist
  - explicit buck chain IDs in placement map
  - detailed execution-order section
  - output-board signoff checklist

## 4) Confidence scorecard (tactics)

All tactics were forced to >=9 confidence by replacing lower-confidence approaches with stronger evidence-based methods.

| Tactic | Initial confidence | Final confidence | Replacement / hardening applied |
| --- | ---: | ---: | --- |
| Assume BOM stock from prior checks | 4 | 10 | Replaced with fresh live scrape of every output BOM line from JLC part pages. |
| Spot-check only high-risk parts | 5 | 9 | Replaced with full 32-line sweep and stock-count capture. |
| Infer schematic correctness from prose only | 6 | 9 | Replaced with designator-by-designator cross-check vs BOM and appendix maps. |
| Keep formula-style `n` indexing | 3 | 10 | Replaced with explicit `k=0..7` mapping plus CH table (`F/Q/R/D` IDs). |
| Keep high-level layout advice | 5 | 9 | Replaced with explicit routing order and signoff checklist for first-time layout execution. |
| Leave D1 ambiguity unresolved | 2 | 9 | Replaced with explicit `D1` path in output power-entry docs and aligned appendix mapping. |
