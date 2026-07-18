# CAD / 3D-print notes

## IP

Gasketed lid, M12 panel seals, HangTon USB with cap. Seal the C14 cutout (inlet is not weatherproof). Hose-test.

## Outer sizes (start)

| Box | L × W × H mm |
| --- | --- |
| sign-input | 240 × 160 × 90 |
| mp-input | 200 × 140 × 90 |
| sign-output | 300 × 220 × 110 |
| mp-output | 280 × 200 × 100 |

## Cutouts

| Feature | Notes |
| --- | --- |
| IEC C14 | All four boxes |
| M12 | Female front; 1 on inputs, 2 on outputs |
| HangTon USB | D-cut |
| RESET / BOOT | Ø16 mm |
| POWER rocker (outputs) | ~30 × 22 mm |
| LED window | ~80 × 8 mm |

```text
sign-output: [C14] [M12 RS-485] [M12 SOL] [USB]
mp-output:   [C14] [M12 RS-485] [M12 SOL] [USB]
sign-input:  [C14] [M12 RS-485] [USB]  + arcade face
mp-input:    [C14] [M12 RS-485] [USB]  + arcade face
```

Leave clearance behind each M12 for screw-terminal assembly.

## Keep-outs

| Item | Size |
| --- | --- |
| LRS-200 | 215 × 115 × 30 + airflow |
| LRS-150 | 159 × 97 × 30 + airflow |
| IRM-15-12 | PCB-mount + lid clearance |
