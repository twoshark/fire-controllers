# CAD parts — what to model

Every enclosure is **two printed solids**. Model and export them separately.

| Part | File stem | What it is |
| --- | --- | --- |
| **BODY** | `{unit}-body` | Open-top box · floor · walls · panel cutouts · internal bosses |
| **LID** | `{unit}-lid` | Flat lid · gasket groove · hinge bosses · latches · (inputs: arcade holes) |

| Unit | BODY | LID |
| --- | --- | --- |
| sign-input | [`sign-input/BODY.md`](sign-input/BODY.md) | [`sign-input/LID.md`](sign-input/LID.md) |
| mp-input | [`mp-input/BODY.md`](mp-input/BODY.md) | [`mp-input/LID.md`](mp-input/LID.md) |
| sign-output | [`sign-output/BODY.md`](sign-output/BODY.md) | [`sign-output/LID.md`](sign-output/LID.md) |
| mp-output | [`mp-output/BODY.md`](mp-output/BODY.md) | [`mp-output/LID.md`](mp-output/LID.md) |

Optional third print (not lid/body): **DT/DTP retainer clips** — separate small parts; see [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md). Do **not** print hinges (prefab bociloy 1").

Shared numbers: [`MOUNTING.md`](MOUNTING.md) · [`PANEL_CUTOUTS.md`](PANEL_CUTOUTS.md) · [`SEALING.md`](SEALING.md) · [`CAD_VERIFICATION.md`](CAD_VERIFICATION.md).

---

## Coordinate frame (all units)

```text
Outer envelope: L × W × H  =  +X × +Y × +Z
Origin: outer front-left-bottom corner (unless a BODY/LID doc says inner floor)
FRONT = Y≈0 = LED window wall
BACK  = Y = W
LEFT  = X≈0
RIGHT = X = L
UP    = +Z
```

Inner cavity ≈ outer − **2× wall** (wall **3.0** mm → −6 mm on L and W).

---

## Modeling order (any CAD tool)

### BODY

1. Outer solid L×W×H  
2. Shell / subtract inner (wall 3.0; floor 3.0) → open top  
3. Floor bosses (PCB M2, RS-15 M3 if input)  
4. Side cutouts (panel features)  
5. LED window pocket on FRONT  
6. Body-side hinge bosses (BACK, internal)  
7. Front flange / latch M3 inserts (mate with lid)  
8. Export `{unit}-body.stl` · print **open face up**

### LID

1. Outer plate L×W × thickness (typ. **4–5** mm)  
2. Underside gasket groove (3.5 W × 2.0 D) — continuous FRONT/LEFT/RIGHT; BACK inboard of hinges  
3. Lid-side hinge bosses (BACK)  
4. Latch clearance / through for M3 at FRONT  
5. Inputs only: arcade button holes  
6. Optional: buttons-PCB M2 bosses underside  
7. Export `{unit}-lid.stl` · print **groove up**

---

## Common feature recipes

| Feature | Recipe |
| --- | --- |
| Wall | **3.0** mm |
| PCB standoff | Boss H **≥12** · hole Ø**3.2** · OD **≥7** · M2 heat-set |
| M3 boss (latch / RS-15) | Hole Ø**4.2** · OD **≥9** · H **≥6** |
| bociloy hinge (body + lid) | Pin at BACK · M2 boss mid-pin · **Y = 7.5** into cavity · [`MOUNTING.md`](MOUNTING.md) |
| LED window | **40 × 10** aperture · recess for 1 mm acrylic + foam |
| M12-5 | Through **Ø16.2** |
| C14 | **27.5 × 20** (or flange pack — measure) |
| KCD4 + boot | **30 × 22** |
| DT pocket | ≈**16 × 18** + lip · clip retain (separate part) |
| DTP pocket | ≈**18 × 22** + lip · clip retain |
| Gasket | Groove in **lid** · 20–30% crush on foam/silicone tape |

---

## Print

| Part | Orientation | Notes |
| --- | --- | --- |
| BODY | Cavity open **up** | No supports if DFAM OK |
| LID | Groove **up** | Arcade holes vertical |
| Material | PETG or ABS | 0.2 mm · 3–4 perimeters · 25–40% |

Bed ≤ **256 × 256**. Largest body footprint **220 × 180** (sign-input).
