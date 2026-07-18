# mp-input — CAD

Refs: [`../CAD_VERIFICATION.md`](../CAD_VERIFICATION.md) · [`../MOUNTING.md`](../MOUNTING.md) · [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md) · [`../SEALING.md`](../SEALING.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **200 × 160 × 90** |
| Inner | **194 × 154 × 84** |

## Top — triangle + ALL center

Pitch **≥50**. Centers in [`../MOUNTING.md`](../MOUNTING.md). Hole **Ø28** (verify).

```text
            [CH1]
      [CH3]  [ALL]  [CH2]
```

## Sides

C14 + POWER + **M12 on LEFT** · LED **FRONT**. Y: C14 **30** · KCD4 **55** · M12 **80**.

## Internal

| Item | Spec |
| --- | --- |
| RS-15-12 | Body (10,15)–(61,77.5) · M3 @ (35.5, 26.5)/(35.5, 65.6) |
| Input PCB | `(ox,oy)=(70,30)` · H1–H4 · Z=12 |
| Channels | D4..D8 unused |
| Lid | **2× KHA-25C** BACK · 2× M3 latch FRONT |

## DFAM

Body/lid support-free; retainer clips for M12. Bed OK with brim.
