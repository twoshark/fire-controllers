# Sealing strategy (dust + rain)

Target: **playa-practical** — IP67 on mated signal/power connectors, gasketed lids, dust-managed cooling.  
Whole-box IP67 is **not** claimed (forced-air LRS fans + C14 when unmated).

---

## Interface IP table

| Interface | Part | Mated | Unmated | Mitigation |
| --- | --- | --- | --- | --- |
| 12 V bus | DTP 2-pin (25 A) | **IP67 at connector face** | dust/rain into cavity | Printed pocket + lip is **not** OEM flange IP67 for the panel hole — add foam around housing; leave mated in field; cavity plugs |
| SOL | DT 2-pin (13 A) | **IP67 at connector face** | same | Same pocket caveat; prefer flange (COST_OPTIONS C) if budget allows |
| RS-485 | M12-5 | **IP67** (if gasketed panel nut) | open | M12 dust caps on panel when unused |
| AC inlet | IEC C14 | cord helps | **~IP20** | Rubber inlet cover when unmated; keep cord plugged in weather |
| POWER | KCD4 + silicone boot | splash | boot limits dust | Boots on all 4 AC rockers |
| Fans (power boxes) | 60 mm + filter | open by design | — | Intake filter; foam gasket under fan; shade box |
| LED window | PC lens + foam gasket | splash/dust | — | Do **not** leave open slot |
| Lid | printed + silicone gasket | splash/dust | — | Continuous gasket; compress with screws/latches |
| Arcade (inputs) | EG STARTS | not certified | nut/LED path | Silicone under bezel; leave lit/used |

---

## Rules

1. Prefer **Deutsch DT / DTP** (or Amphenol AT / ATP) for outdoor power and loads — not Anderson Powerpole.
2. Kill 12 V loads with the **AC rocker on `*-output-power`** — no AC-rated rocker in the 12 V path.
3. Print connector pockets with a compression lip + foam against the housing; do not claim panel IP67 unless using flange parts.
4. Cap unused M12. Use cavity plugs from DT/DTP kits (or buy plugs) for open cavities.
5. Fans are **dust-managed**, not sealed.
6. LED window = PC lens + foam gasket (not an open slot). Lid = continuous silicone gasket.
