# CAD / 3D-print notes (budget GX16)

Shells for sign-input, mp-input, sign-output, mp-output.

## IP strategy

Gasketed lid + GX16 panel seals + caps + HangTon USB **IP65** with weather cap. Expect **IP55–IP65** after hose test — not lab-certified IP67/IP68.

## Outer sizes (start)

| Box | L × W × H mm | Driver |
| --- | --- | --- |
| sign-input | 240 × 160 × 90 | Arcade row |
| mp-input | 200 × 140 × 90 | Fewer buttons |
| sign-output | 300 × 220 × 110 | LRS-200 (215×115×30) |
| mp-output | 280 × 200 × 100 | LRS-150 (159×97×30) |

Wall 3.0–3.5 mm PETG/ABS; lid 3 mm; gasket groove 2–3 mm.

## Cutouts

| Feature | Hole | Notes |
| --- | --- | --- |
| GX16 panel | **Ø16 mm** | All power/signal GX16 |
| USB-C HangTon D-type | D-cut per datasheet | Cap clearance; nut inside |
| RESET/BOOT Adafruit 16 mm | **Ø16 mm** | Gasket included |
| POWER rocker | ~30 × 22 mm | `WRG32F2FBBNN`; seal cutout |
| Arcade (input) | per your buttons | — |
| LED window | ~80×8 mm | Match PCB LED row |

### sign-output / mp-output edge layout

```text
[AC IN GX16-3 male] [GLOW GX16-3 female] [RS-485 GX16-6] [SOL multipin] [USB-C]
```

Put AC/glow on bottom drip edge. Label `120VAC IN`, `GLOW 120VAC`, `SOLENOID 12V`, `RS-485`.

## Keep-outs

| Item | Size |
| --- | --- |
| LRS-200 | 215 × 115 × 30 mm + terminals + 10 mm airflow |
| LRS-150 | 159 × 97 × 30 mm + terminals + 10 mm airflow |
| IRM-15-12 | 53 × 28 × 25 mm |
| G5LE | ~22 × 16 × 19 mm |

## Print

PETG; ≥4 walls; 40–60% gyroid; M3 heat-sets; lid screws 6–10 places.

## Checklist

- [ ] Continuous gasket  
- [ ] Nut clearance behind every GX16  
- [ ] AC loom isolated from 12 V / RS-485  
- [ ] LRS selector accessible before final close  
- [ ] Caps for unmated connectors  
