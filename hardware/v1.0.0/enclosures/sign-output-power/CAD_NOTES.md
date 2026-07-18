# sign-output-power — CAD

Printer bed ≤ 256 × 256 mm.

## Size

**240 × 180 × 95 mm** (L×W×H)

LRS body is **215 × 115 × 30 mm**. Fans on the **long walls** (across the 115 mm width). Do not put fan stacks on the short ends — 215 + 2×(~25 mm fan/filter) exceeds a ≤256 mm print axis.

## Layout

```text
TOP VIEW (lid removed):

  W=180
  ┌────────────────────────────────────────────┐
  │ ▓ intake          LRS-200-12               │
  │ ▓ 09250-F/60      215 × 115 on Al plate    │ exhaust ▓
  │ ▓ + fan           terminals → short end A  │ 8147+fan ▓
  ├────────────────────────────────────────────┤
  │ short end A:  [C14]  [POWER WRG32]  [PanelPole2]│
  └────────────────────────────────────────────┘
         L = 240
```

| Zone | Contents |
| --- | --- |
| Short end A | C14, POWER, PanelPole2 12V OUT — wire path ≤80 mm to LRS terminals |
| Floor center | Al plate 115 × 215 × 3 mm · LRS bolted 4×M4 · 10 mm air gap under plate (standoffs) |
| Long wall −Y | Intake: panel 09250-F/60 + MF60151V2 inside, blowing **in** |
| Long wall +Y | Exhaust: panel 8147 + MF60151V2 inside, blowing **out** |
| Short end B | Closed / grill optional — no fan stack |

Air path: intake → across LRS long faces → exhaust. Keep ≥5 mm free around LRS mesh sides.

## Cutouts

| Feature | Cutout | Notes |
| --- | --- | --- |
| IEC C14 | per inlet drawing (snap-in pack B07PVP7XB7) | Hot through POWER only |
| WRG32F2FBBNN | **30 × 22 mm** | DPST · switches AC L |
| PanelPole2 | **Ø1-1/8" (28.6 mm)** | 12V OUT · red=+ · black=GND |
| Intake | Qualtek **09250-F/60** pattern (60 mm fan) | Filter outside, fan inside |
| Exhaust | Qualtek **8147** + fan hole pattern **50 mm** pitch | Guard outside, fan inside |
| Lid | gasket groove · 4–6× M3 lid screws | No vents in lid |

## Cooling stack

| Item | Spec |
| --- | --- |
| PSU | LRS-200-12 · 215 × 115 × 30 mm · mount **4×M4** (datasheet Case 207) |
| Spreader | 6061 Al · **115 × 215 × 3 mm** · thermal pad optional under LRS |
| Fans | 2× MF60151V2-1000U-A99 · 60×60×15 · 12 V · hole pitch 50 mm |
| Intake | 09250-F/60 |
| Exhaust | 8147 |

## Internal keep-outs

| Item | Clearance |
| --- | --- |
| LRS terminal screws | finger access from short end A |
| PanelPole2 rear | ≥25 mm (nut + contacts) |
| Fan depth | 15 mm + filter/guard into interior from each long wall |
| Wire duct | AC along short-end A; 12 AWG DC along floor to PanelPole2 |

## Print

| Param | Value |
| --- | --- |
| Material | ASA or PETG |
| Wall | ≥3 mm |
| Lid | separate · gasket |
| Orientation | print shell open-up; avoid supports in fan openings |

LRS: selector **115 V**, trim **12.0 V** before close-up.
