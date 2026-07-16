# CAD / 3D-print notes

Printable shells for **sign-input** and **mp-input**. Dimensions are starting recommendations — adjust after measuring your actual PCB + PSU stack.

## Target IP strategy

3D prints are porous unless sealed. Treat IP67 as **panel components + gasketed lid**, not raw PETG alone:

1. 2–3 mm deep lid groove + continuous EPDM/silicone O-ring / gasket cord
2. All cutouts use the component’s panel gasket + correct torque
3. Optional: brush-on epoxy / polyurethane coat on exterior
4. Orient seams away from spray; drain lip under lid

Expect **IP65–IP67 with care**; validate with hose test before field use.

## Suggested outer sizes

| Box | Outer L × W × H | Why |
| --- | --- | --- |
| **sign-input** | **240 × 160 × 90 mm** | 6× 22 mm + POWER + 2× 16 mm + connectors |
| **mp-input** | **200 × 140 × 90 mm** | Fewer channel buttons |

Wall: **3.0–3.5 mm** PETG/ABS. Lid: **3 mm**. Corner radius ≥3 mm for strength.

## Internal keep-outs

| Item | Envelope (approx.) | Mount |
| --- | --- | --- |
| Input PCB | check your panel; leave 10 mm margin | M3 standoffs 8–10 mm |
| IRM-15-12 | 53 × 28 × 25 mm + wiring | Insulated carrier; AC clearance ≥5 mm to plastic if no coating |
| Diode daughter | ~70 × 25 × 10 mm (from PnP span) | Standoffs or adhesive standoff |
| Cable bend (M12/USB) | 30 mm behind panel | — |

## Front panel cutouts

| Feature | Hole | Qty sign / mp |
| --- | --- | --- |
| Channel + ALL + POWER (arcade) | **per your arcade datasheet** (often Ø24–30 mm) | 7 / 5 |
| RESET, BOOT | **Ø16.0 mm** (or arcade if you prefer) | 2 / 2 |
| M12-8 panel | per datasheet (often **Ø16 mm** / M16 thread) | 1 |
| USB-C bulkhead | per datasheet (often rectangular + 2 screws) | 1 |
| Bulgin / IP68 Buccaneer 400 | **Ø16 mm** class single-hole (confirm `PX0412` datasheet) | 1 |
| LED window | slot matching PCB LED row (~80×8 mm) | 1 |

Arcade spacing: leave **≥ finger width** between bezels; verify nut clearance behind the panel.

### Suggested front layout — sign-input

```text
┌──────────────────────────────────────────────┐
│  [S1] [S2] [S3] [S4] [S5]     [ALL FIRE]     │
│                                              │
│  [POWER]              [RESET] [BOOT]         │
│                                              │
│  ░░░░░ LED WINDOW ░░░░░                      │
│                                              │
│  (USB-C)     (M12 RS-485)     (IP67 AC)      │
└──────────────────────────────────────────────┘
```

### Suggested front layout — mp-input

```text
┌────────────────────────────────────┐
│  [M1] [M2] [M3]      [ALL FIRE]    │
│                                    │
│  [POWER]        [RESET] [BOOT]     │
│                                    │
│  ░░░ LED WINDOW ░░░                │
│                                    │
│  (USB-C)  (M12)  (IP67 AC)         │
└────────────────────────────────────┘
```

Put **IP67 AC inlet on the bottom or rear** if you want the front cleaner; keep AC wiring short inside. Cap all connectors when unmated.

## Print settings (starting point)

| Parameter | Value |
| --- | --- |
| Material | PETG (outdoor) or ABS (acetone-smoothable) |
| Infill | 40–60% gyroid for walls |
| Walls | ≥4 perimeters |
| Layer | 0.2 mm |
| Orientation | Lid flat; box open-face up |
| Inserts | M3 heat-set after print |

## Fasteners

- Lid: M3×10 with washers into heat-sets (6–8 places)
- PCB: M3×8 + 6–8 mm nylon/brass standoffs
- Do not metal-standoff directly under AC traces without insulation

## Labeling

Laser/engrave or vinyl:

- Channel names, ALL FIRE, POWER, RESET, BOOT
- `RS-485`, `USB DFU`, `120 VAC`
- Warning: mains inside

## CAD checklist before slicing

- [ ] Gasket groove continuous (no gaps at corners)
- [ ] Nut clearance behind every panel connector
- [ ] IRM airflow: ≥10 mm free above module
- [ ] Strain relief bosses for internal harness
- [ ] Drain / weep optional on bottom (trade vs IP)
