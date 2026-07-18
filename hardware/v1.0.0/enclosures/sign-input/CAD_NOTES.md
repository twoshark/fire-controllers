# sign-input — CAD

Orientation: [`../README.md`](../README.md). Cutouts: [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md). Bed ≤ **256 × 256**.

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 180 × 90** |
| Wall | 2.5–3.0 |
| Lid | openable (top) |

## Top — EG STARTS triangle hex (ALL at top)

```text
              [ALL]
         [CH1]     [CH2]
         [CH5]     [CH3]
              [CH4]
```

| Dim | Value |
| --- | ---: |
| Button hole | **Ø28** (adapter ring — verify before freeze; some rings use Ø24) |
| Center-to-center pitch | **≥45** |
| Hex bounding box (6× centers) | ~**110 × 100** |
| Panel thickness at buttons | ≤8 (nut + ring) |

LED each button: **12 V** from RS-15. Part: [B07R565HM6](https://www.amazon.com/dp/B07R565HM6) (`5/SJX-5C-BUT`).

## Sides (elevations)

Heights = hole center from **outer bottom**.

```text
FRONT (long, LED):     [LED window 40×10]  CL height 50
LEFT  (short, AC):     [C14] CL 35     [POWER KCD4] CL 50
RIGHT (short, serial): [M12-5 Ø16.2] CL 50
BACK  (long):          blank / hinge
```

| Feature | Cutout | CL height | Spacing notes |
| --- | --- | ---: | --- |
| C14 | 27.5×20 (snap) or flange 27×19 | **35** | ≥12 from corners |
| POWER KCD4 | 30×22 | **50** | ≥15 clear of C14 |
| M12-5 | Ø16.2 | **50** | — |
| LED window | 40×10 | **50** | Align to PCB LED edge (31 mm row + margin) |

No panel RESET/BOOT/USB — open lid → `J5` / `SW1` / `SW2`.

## Internal keep-outs

| Item | Size / place |
| --- | --- |
| Input PCB | **86 × 83** keep-out; LED edge → LED window; `CN2` → M12; `J1` → RS-15 |
| RS-15-12 | **62.5 × 51 × 28**; near C14/POWER; 2× M3 |
| Buttons daughter | near top; flying leads to `J2` / `J3` |
| Air | ≥10 above RS-15 |

## Cables (external)

| Cable | Length |
| --- | ---: |
| C13→C14 mains | **6 ft** |
