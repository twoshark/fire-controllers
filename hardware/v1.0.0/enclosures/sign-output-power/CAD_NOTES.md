# sign-output-power — CAD

Refs: [`../CAD_VERIFICATION.md`](../CAD_VERIFICATION.md) · [`../MOUNTING.md`](../MOUNTING.md) · [`../SEALING.md`](../SEALING.md).

## Envelope (grown for LRS + wires · bed-max L)

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **256 × 200 × 100** |
| Inner | **250 × 194 × 94** |

**Print note:** L = 256 — use **no brim** on length, or rotate 90° if brim required.

## Layout (critical)

```text
SHORT mate end:     [DTP OUT] CL 40     ← faces sign-output
LONG wall (term):   [C14] CL 35  [POWER+boot] CL 50   near LRS terminals
LONG walls:         fans recessed  CL 48
LRS:                215×115×30 on Al plate; terminals → AC long wall
                    far end clearance ≥5; terminal-end wire bay ≥30
```

| Feature | Cutout | CL |
| --- | --- | ---: |
| DTP OUT | pocket ≈18×22 | **40** |
| C14 | 27.5×20 | **35** |
| POWER | 30×22 | **50** |
| Fan | Ø57 + 50×50 | **48** |

## Internal

| Item | Spec |
| --- | --- |
| Al plate | **215×115×3** · 4× M4 · hole span verify on LRS |
| LRS-200-12 | on plate · M4 L≤5 into PSU |
| Fans ×2 | wall pockets · foam · on LRS +V/−V |
| Lid | 6× M3 · gasket |

## Cables

| Cable | Length |
| --- | ---: |
| C13→C14 | **6 ft** |
| DTP → sign-output | **≤ 4 ft** |

LRS: **115 V**, trim **12.0 V**.
