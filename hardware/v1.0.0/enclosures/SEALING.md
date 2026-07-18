# Sealing (dust + rain)

Target: **playa-practical**.

| Class | Interfaces | Claim |
| --- | --- | --- |
| **IP67 (mated)** | DT · DTP · M12-5 | Connector face when correctly assembled with seals/wedges |
| **IP67 (unit)** | HLG-240H-12 | Factory; mount shaded; FG earthed |
| **Splash / dust** | Printed lids + gasket · LED PC+foam · KCD4 boot · C14 cover | Not certified whole-box IP67 |
| **Not outdoor alone** | Arcade EG STARTS · C14 when unmated | Prefer inputs under canopy / shade |

Do **not** claim the printed PETG/ABS shell is IP67.

| Interface | Part | Mated | Unmated | Mitigation |
| --- | --- | --- | --- | --- |
| 12 V | DTP 2-pin | IP67 at connector face | cavity | Pocket+foam or **flange**; leave mated; cavity plugs |
| SOL | DT 2-pin | IP67 at connector face | same | Same; flange = cost option **C** |
| RS-485 | M12-5 | IP67 (gasketed nut) | open | Dust caps both ends |
| AC (inputs) | C14 | cord helps | ~IP20 | Cover when unmated · canopy |
| POWER (inputs) | KCD4 + boot | splash | boot | Silicone boot · AC only |
| 12 V PSU | HLG-240H-12 | **IP67 unit** | — | Shade · FG |
| HLG DC → DTP | cut pigtail + crimp | — | — | **Adhesive heatshrink / gel** over re-term |
| LED window | PC + foam | splash/dust | — | Not an open slot |
| Lid | silicone gasket + **internal** KHA-25C hinges | splash/dust | — | Gasket FRONT/L/R continuous; BACK inboard of knuckles · 20–30% crush |
| Arcade | EG STARTS | not certified | nut | Silicone under bezel · canopy |

## Rules

1. Outdoor **power and signal links** use DT / DTP / M12 only.
2. Kill 12 V by unplugging HLG AC (no AC-rated rocker on the 12 V path).
3. Printed pockets: compression lip + foam — **not** panel IP67 without flanged TE parts.
4. Cap unused M12; DT/DTP **cavity plugs** on open cavities.
5. LED = PC lens + foam. Lid = silicone gasket; hinges **internal** (Sugatsune KHA-25C) — no external hinge cutouts.
6. After crimping DTP on HLG DC: seal the jacket cut (adhesive HS or gel).
7. Park **input** boxes under canopy when possible (arcade + C14 are the weak faces).
