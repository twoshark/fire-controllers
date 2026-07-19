# LED window (lid) — dividers + clear inserts

**All sizes in mm.** LEDs face **+Z (up)**. Window is in the **lid**, not the FRONT wall.

Print the **lid** (opaque) with baffle dividers. Print **clear** cover inserts separately (clear PETG / clear PLA). One insert per box (or cut from a strip).

Shared recipe for all four lids. Per-box pocket centers: each unit’s [`LID.md`](sign-input/LID.md).

---

## Geometry (10 channels)

LED pitch ≈ **3.50** along **+Y** (front→back). Column at nearly constant **X**.

| Dim | Value | Notes |
| --- | ---: | --- |
| Channels | **10** | Input: `LED1`…`LED10` · Output: `PWR`, `LINK`, `CH0`…`CH7` |
| Pitch | **3.50** | Center-to-center along Y |
| Cell aperture | **2.8 × 2.8** | Through-cut over each LED |
| Divider thickness | **0.70** | Opaque wall between cells (pitch − aperture = 0.70) |
| Baffle depth | lid full thick + **2.0** hang | Extrude dividers **−2 mm** below lid underside |
| Clear insert plate | **9.0 × 38.0 × 1.2** | Sits in top recess |
| Recess (lid top) | **9.4 × 38.4 × 1.5** | Foam optional under clear plate |
| Overall through block | **6.0 × 36.0** | Bounding box of the 10 apertures |

Label order along **+Y** (front → back):

| Box type | Front-most | … | Back-most |
| --- | --- | --- | --- |
| Input | `LED10` | … | `LED1` |
| Output | `CH7` | … | `PWR` |

---

## Lid CAD steps (after lid plate exists)

Use pocket **center (Xc, Yc)** from the unit `LID.md`.

1. **Top recess** (for clear insert): sketch on lid **top** (Z = lid thick) rectangle **9.4 × 38.4**, centered at **(Xc, Yc)**. Long axis along **+Y**. Extrude **−cut** **1.5** deep.
2. **Through apertures:** sketch **10** squares **2.8 × 2.8**, centers at:
   - `X = Xc`
   - `Y_i = Yc − 15.75 + i × 3.50` for `i = 0…9`  
     (span 9×3.50 = 31.5; first/last centers ±15.75 from Yc)
3. Extrude **−cut** through remaining lid thickness (and through recess floor).
4. **Dividers:** between each pair of cells, a wall **0.70** thick × **6.0** wide (X), height = lid thick + **2.0** hanging into cavity. Easiest path:
   - Model as a single **baffle comb**: start from a solid **6.0 × 36.0 × (lid+2)** block under the window, then cut the 10 cell tunnels through it; or
   - Extrude **9** divider ribs from the underside between apertures.
5. Optional: thin **foam** strip under the clear plate before seating.

---

## Clear insert (separate print)

| | |
| --- | --- |
| Material | Clear PETG or clear PLA |
| Size | **9.0 × 38.0 × 1.2** (slightly under recess) |
| Qty | **1 per box** · **4** total |
| Finish | Print face-down on clean PEI for optical face; or sand/polish top |
| Fit | Press into recess; retain with foam crush or a drop of clear silicone at corners |

Do **not** buy sheet acrylic for v1 — printed clear covers are the plan.

---

## Pocket centers (outer XY) — must match floor placement

From flat LED-up mount in [`MOUNTING.md`](MOUNTING.md):

| Unit | Xc | Yc | Notes |
| --- | ---: | ---: | --- |
| sign-input | **192.3** | **81.5** | Right of arcade hex |
| mp-input | **177.3** | **71.5** | Right of arcade |
| sign-output | **165.2** | **102.5** | |
| mp-output | **165.2** | **102.5** | Same PCB map |

After importing the board STEP, nudge **(Xc, Yc)** so cell centers sit on LED midpoints (±0.5 mm OK).
