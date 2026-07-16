# Input-board enclosures

3D-printed waterproof control boxes that host the v1.0.0 **input PCB**, Mean Well **IRM-15-12**, and the **input-buttons** diode-OR daughter PCB.

| Enclosure | Operator buttons | Channels used | Doc |
| --- | --- | --- | --- |
| **sign-input** | 5× channel + ALL FIRE | CH0..CH4 | [`SIGN_INPUT.md`](SIGN_INPUT.md) |
| **mp-input** | 3× channel + ALL FIRE | CH0..CH2 | [`MP_INPUT.md`](MP_INPUT.md) |

Shared:

| Doc | Contents |
| --- | --- |
| [`PARTS_BOM.md`](PARTS_BOM.md) | Buy list with links, gauges, IP ratings (excludes your arcade buttons) |
| [`WIRING.md`](WIRING.md) | Interconnect diagram, net map, AC/DC segregation |
| [`CAD_NOTES.md`](CAD_NOTES.md) | Printable shell dimensions, cutouts, gasket strategy |

Daughter PCB: [`../input-buttons-pcb/`](../input-buttons-pcb/).
Input PCB: [`../INPUT_BOARD.md`](../INPUT_BOARD.md).

## Design goals

- **IP67/IP68** at every panel interface (arcade seals, USB-C, M12, **Bulgin Buccaneer 400** power — **not** open C14 / not PX0580)
- POWER latch breaks **AC hot** after the fuse
- Channel / ALL / POWER use **your arcade buttons**; BOM covers PSU, sealed connectors, RESET/BOOT
- Unused input channels stay **open / disconnected**
- Keep IRM-15-12 well under rating once arcade LED currents are measured
- CAD/print-friendly gasketed shell

## Feature ↔ enclosure map (both boxes)

```text
┌──────────────────────────── FRONT / TOP PANEL ────────────────────────────┐
│  [arcade CH]  [arcade ALL]  [arcade POWER]   [RESET] [BOOT]               │
│       │             │            │               │      │                 │
│       ▼             ▼            │               ▼      ▼                 │
│   switch→GND    ALL_BUTTON_A     │            NRST   BOOT0                │
│       │         (diode OR PCB)   │               │      │                 │
│       └─────────────┴──► input J2a/J2b + J3      └──┬───┘→ MCU pads/SW    │
│                                                      │                    │
│  [USB-C IP67 bulkhead] ──cable──► input J5                                │
│  [M12-8 A-code panel] ──wires──► input CN2 (RS-485)                       │
│  [Bulgin 400 PX0412] ──fuse──► POWER (AC hot) ──► IRM-15-12               │
└───────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────── INSIDE ───────────────────────────────────────┐
│  IRM-15-12 +12V/GND ──► input J1                                          │
│                 └──────► arcade button LEDs (+12V / LED− → GND)           │
│  input-buttons PCB (diodes) between ALL FIRE and channel nets             │
│  Status LEDs on input PCB → light pipes / clear window strip              │
└───────────────────────────────────────────────────────────────────────────┘
```

## Power summary

| Load | Guidance |
| --- | --- |
| Input PCB @12 V | ≤120 mA |
| Arcade button LEDs | **Measure** your lamps; budget total 12 V load **&lt; 800 mA** |
| IRM-15-12 | 1250 mA max |

Reject filament bulbs. LED arcade lamps are expected to leave large headroom.

## Locked decisions

1. Channel / ALL FIRE: momentary NO, switch-to-GND (arcade).
2. POWER: latching, switches **AC hot** (confirmed).
3. Mains: **Bulgin Buccaneer 400** (`PX0412` + `PX0410`) IP68 when mated (confirmed). **Not** IEC C14 / **not** `PX0580`.
4. Unused channels: left open.
5. RS-485 via M12 to the output enclosure ([`../PIN_MAP.md`](../PIN_MAP.md) crossover).
