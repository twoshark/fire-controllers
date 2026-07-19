# LED window (lid) — dividers + clear inserts

**All sizes in mm.** LEDs face **+Z (up)**. Window is in the **lid**, not the FRONT wall.

PCB LED edge sits toward the **FRONT** of the box so the window strip is along the front of the lid — clear of arcade buttons (inputs).

Print the **lid** (opaque) with baffle dividers. Print **clear** cover inserts separately (clear PETG / clear PLA). One insert per box.

Shared recipe for all four lids. Per-box pocket centers: each unit’s `LID.md`.

---

## Geometry (10 channels)

LED pitch ≈ **3.50** along **+X** (left→right). Row at nearly constant **Y** (front of lid).

| Dim | Value | Notes |
| --- | ---: | --- |
| Channels | **10** | Input: `LED1`…`LED10` · Output: `PWR`, `LINK`, `CH0`…`CH7` |
| Pitch | **3.50** | Center-to-center along X |
| Cell aperture | **2.8 × 2.8** | Through-cut over each LED |
| Divider thickness | **0.70** | Opaque wall between cells |
| Baffle depth | lid full thick + **2.0** hang | Extrude dividers **−2 mm** below lid underside |
| Clear insert plate | **38.0 × 9.0 × 1.2** | Long axis **+X** |
| Recess (lid top) | **38.4 × 9.4 × 1.5** | Foam optional under clear plate |
| Overall through block | **36.0 × 6.0** | Bounding box of the 10 apertures |

Label order along **+X** (left → right):

| Box type | Left-most | … | Right-most |
| --- | --- | --- | --- |
| Input | `LED1` | … | `LED10` |
| Output | `PWR` | … | `CH7` |

---

## Lid CAD steps (after lid plate exists)

Use pocket **center (Xc, Yc)** from the unit `LID.md`.

1. **Top recess** (for clear insert): sketch on lid **top** rectangle **38.4 × 9.4**, centered at **(Xc, Yc)**. Long axis along **+X**. Extrude **−cut** **1.5** deep.
2. **Through apertures:** sketch **10** squares **2.8 × 2.8**, centers at:
   - `Y = Yc`
   - `X_i = Xc − 15.75 + i × 3.50` for `i = 0…9`
3. Extrude **−cut** through remaining lid thickness (and through recess floor).
4. **Dividers:** between each pair of cells, a wall **0.70** thick × **6.0** deep (Y), height = lid thick + **2.0** hanging into cavity. Or model a baffle comb **36.0 × 6.0 × (lid+2)** and cut the 10 tunnels.
5. Optional: thin **foam** under the clear plate before seating.

---

## Clear insert (separate print)

| | |
| --- | --- |
| Material | Clear PETG or clear PLA |
| Size | **38.0 × 9.0 × 1.2** (slightly under recess) |
| Qty | **1 per box** · **4** total |
| Finish | Print face-down on clean PEI for optical face |
| Fit | Press into recess; foam crush or clear silicone at corners |

---

## Pocket centers (outer XY) — must match floor placement

CW LED-edge→FRONT map in [`MOUNTING.md`](MOUNTING.md):

| Unit | Xc | Yc | Notes |
| --- | ---: | ---: | --- |
| sign-input | **118.4** | **34.8** | Front strip; arcade hex centered farther back |
| mp-input | **113.4** | **31.8** | Front strip; arcade moved back |
| sign-output | **70.9** | **34.7** | |
| mp-output | **70.9** | **34.7** | Same PCB map |

After importing the board STEP, nudge **(Xc, Yc)** so cell centers sit on LED midpoints (±0.5 mm OK).
