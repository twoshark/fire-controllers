# mp-input — LID

Export: `mp-input-lid.stl` · Print: groove **up**.  
Shared: [`../CAD_PARTS.md`](../CAD_PARTS.md) · [`../MOUNTING.md`](../MOUNTING.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W | **200 × 160** |
| Thickness | **4–5** |
| Button holes | **Ø28** (verify ring) |

Ring center ≈ outer **(100, 80)**. Pitch **≥50**.

## Build steps

1. **Plate** 200 × 160 × 4–5.  
2. **Gasket groove** 3.5 × 2.0 — FRONT/LEFT/RIGHT continuous; BACK inboard of hinges.  
3. **Hinge lid leaf** pins:

| Hinge | (X, Y) |
| --- | ---: |
| A | **(35, 150)** |
| B | **(159, 150)** |

M2 mid-pin · 7.5 mm into cavity.  
4. **Latch** M3 through @ ≈ **(8, 8)** / **(186, 8)**.  
5. **Arcade** — 4× Ø28: ALL center + triangle

| Button | ΔX | ΔY |
| --- | ---: | ---: |
| ALL | 0 | 0 |
| CH1 | 0 | −50 |
| CH2 | +43.3 | +25 |
| CH3 | −43.3 | +25 |

```text
            [CH1]
      [CH3]  [ALL]  [CH2]
```

6. Optional buttons-PCB M2 bosses underside.

## Mate check

Same as sign-input: hinge clear, ≥90° open, gasket crush 20–30%.
