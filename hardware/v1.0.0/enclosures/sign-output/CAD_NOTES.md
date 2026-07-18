# sign-output — CAD

Printer bed ≤ 256 × 256 mm.

## Size

**220 × 160 × 80 mm** (L×W×H)

## Connector walls

```text
WALL A (power / data):
  [PanelPole2 12V IN] [M12-5 RS-485] [HangTon USB]

WALL B (solenoids — long face):
  [SOL0] [SOL1] [SOL2] [SOL3] [SOL4]   ← AT04-2P each
```

| Feature | Cutout |
| --- | --- |
| PanelPole2 | **Ø1-1/8" (28.6 mm)** |
| M12-5 RS-485 | Ø16.2 mm |
| AT04-2P SOL0..SOL4 | rectangular per AT housing · printed flange/pocket |
| HangTon USB | D-cut |
| RESET / BOOT | Ø16 mm |
| LED window | **40 × 10 mm** vertical |

SOL pitch ≥30 mm. Label SOL0..SOL4.

## Internal layout

```text
PanelPole2 ──12 AWG──► J1
M12 RS-485 ──22–24 AWG──► J2
AT SOL pin1 ──18 AWG──► J6 (both poles)
AT SOL pin2 ──18 AWG──► J5a.n / J5b.1
HangTon ──jumper──► J7
```

PCB orientation: [`PnP`](../../output/eda-exports/PnP_output.csv) — `J1` left, `J5`/`J6` top toward SOL wall, `J2`/LEDs right, USB bottom.

Keep-outs: PanelPole ≥25 mm · M12 ≥35 mm · AT rear clearance for wedges · `F9` access · PCB ~120×110 +3 mm.

## Print

ASA or PETG · gasketed lid · AT housings captured so mating face is flush/proud of outer wall.
