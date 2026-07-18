# sign-input — LID

Export: `sign-input-lid.stl` · Print: gasket groove **up**.  
Shared: [`../CAD_PARTS.md`](../CAD_PARTS.md) · button XY: [`../MOUNTING.md`](../MOUNTING.md).

## Envelope

| Dim | mm |
| --- | ---: |
| Outer L × W | **220 × 180** (match body) |
| Thickness | **4–5** (solid plate) |
| Arcade holes | **Ø28** (verify EG STARTS ring; may be Ø24) |

Lid XY origin = same as body outer (front-left). Center for hex ring ≈ outer **(110, 90)**.

## Build steps

1. **Plate** 220 × 180 × 4–5.  
2. **Gasket groove** (underside when printed groove-up = top of print):  
   - **3.5** W × **2.0** D  
   - Continuous on FRONT / LEFT / RIGHT  
   - BACK: run **inboard of hinge knuckles** (hinges stay internal/dry)  
3. **Hinge — lid leaf** (BACK underside) · match body pins:

| Hinge | Pin mid (X, Y) |
| --- | ---: |
| A | **(40, 170)** |
| B | **(174, 170)** |

M2 boss: mid-pin · **7.5 mm** toward FRONT (+ into lid cavity side) · Ø3.2 · OD≥7.  
4. **Latch** — clearance / counterbore for M3×10 at FRONT corners ≈ **(8, 8)** and **(206, 8)** (screws into body inserts).  
5. **Arcade hex ring** — 6× Ø28 through. Lid center **(110, 90)** + Δ:

| Button | ΔX | ΔY |
| --- | ---: | ---: |
| ALL | 0 | 0 |
| CH1 | 0 | −38.3 |
| CH2 | +36.4 | −11.8 |
| CH3 | +22.5 | +31.0 |
| CH4 | −22.5 | +31.0 |
| CH5 | −36.4 | −11.8 |

Margins ≥12 from lid edge. Pitch **45** on R≈38.3.  
6. **Optional** — M2 bosses for buttons daughter PCB underside (2–4 diagonal). Orient harness toward `J2`.  
7. **No** panel cutouts on lid (C14 / KCD4 / M12 / LED are on BODY).

## Mate check

- Closed lid: hinge leaves clear PCB / RS-15 / wires  
- Open ≥90° toward FRONT without hitting BACK wall  
- Gasket crush 20–30% on foam/silicone tape  
