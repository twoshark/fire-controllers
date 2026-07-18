# sign-input — CAD

Refs: [`../CAD_VERIFICATION.md`](../CAD_VERIFICATION.md) · [`../MOUNTING.md`](../MOUNTING.md) · [`../PANEL_CUTOUTS.md`](../PANEL_CUTOUTS.md) · [`../SEALING.md`](../SEALING.md).

Bed ≤ **256 × 256**. Print body open-up; lid groove up.

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W × H | **220 × 180 × 90** |
| Inner (3 mm wall) | **214 × 174 × 84** |
| Wall | 3.0 |

## Top — EG STARTS hex (ALL at top)

```text
              [ALL]
         [CH1]     [CH2]
         [CH5]     [CH3]
              [CH4]
```

| Dim | Value |
| --- | ---: |
| Button hole | **Ø28** (verify ring) |
| Pitch | **≥45** |
| Hex bbox | ~**110 × 100** |

## Sides (CL from outer bottom)

| Feature | Cutout | CL | Wall |
| --- | --- | ---: | --- |
| C14 | 27.5×20 | **35** | LEFT |
| POWER KCD4+boot | 30×22 | **50** | LEFT |
| M12-5 | Ø16.2 | **50** | RIGHT |
| LED 40×10 + PC lens | — | **50** | FRONT |

## Internal (see MOUNTING.md)

| Item | Spec |
| --- | --- |
| Input PCB | Outline **83.1×79.0** · KO **86×83** · LED→FRONT · 4× M2 bosses ([`../MOUNTING.md`](../MOUNTING.md) H1–H4) · Z=12 |
| RS-15-12 | **62.5×51×28** · LEFT near C14 · 2× M3 |
| Buttons PCB | Outline **71.0×44.7** · under top · 2–4× M2 |
| Lid | 4× M3 · gasket groove |

## Cables

C13→C14 **6 ft**.

## Supports

None if DFM followed (chamfered pockets, groove up). Brim optional ≤5 mm.
