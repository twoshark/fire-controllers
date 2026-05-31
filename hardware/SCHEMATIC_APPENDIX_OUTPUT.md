# Output Board Schematic Appendix (Executable Mapping)

This appendix is the pin/net/junction reference for `hardware/SCHEMATIC_GUIDE.md`.

Review-related additions (per `hardware/REVIEW_RESOLUTION.md`):

- New designators: `F9` (12V input ATO blade fuse holder), `C30` (2nd 22uF on `3V3`), `C31` (`U2A.VCC`), `C32` (`U2B.VCC`), `C33` (`U5.VCC`), `C34` (`U6.VCC`), `D20` (USB ESD).
- Net naming: `3V3` only - never `3.3V`, `+3.3V`, etc.
- All `GND`/`3V3`/`12V_MAIN`/`VIN_12V_IN` use Power Symbol Net Flags, not Net Ports.
- Polarized cap polarity is locked: see Section 8 below.

## 1) MCU used-pin map (output board)

| MCU GPIO/net | Direction | Function | Endpoint |
| --- | --- | --- | --- |
| `PA0` | In | `OVR_CH0_SENSE` | CH0 override Schmitt output |
| `PA1` | In | `OVR_CH1_SENSE` | CH1 override Schmitt output |
| `PA4` | In | `OVR_CH2_SENSE` | CH2 override Schmitt output |
| `PA5` | In | `OVR_CH3_SENSE` | CH3 override Schmitt output |
| `PA6` | In | `OVR_CH4_SENSE` | CH4 override Schmitt output |
| `PA7` | In | `OVR_CH5_SENSE` | CH5 override Schmitt output |
| `PB0` | In | `OVR_CH6_SENSE` | CH6 override Schmitt output |
| `PB1` | In | `OVR_CH7_SENSE` | CH7 override Schmitt output |
| `PB2` | Out | `GATE_CH0` | MOSFET CH0 gate |
| `PB10` | Out | `GATE_CH1` | MOSFET CH1 gate |
| `PB11` | Out | `GATE_CH2` | MOSFET CH2 gate |
| `PB12` | Out | `GATE_CH3` | MOSFET CH3 gate |
| `PB13` | Out | `GATE_CH4` | MOSFET CH4 gate |
| `PA8` | Out | `GATE_CH5` | MOSFET CH5 gate (USB pin conflict avoided) |
| `PA15` | Out | `GATE_CH6` | MOSFET CH6 gate |
| `PB3` | Out | `GATE_CH7` | MOSFET CH7 gate |
| `PA9` | Out | `USART1_TX` | RS-485 TX transceiver `U2B.DI` (heartbeat) |
| `PA10` | In | `USART1_RX` | RS-485 RX transceiver `U2A.RO` |
| `PA11` | BiDir | `USB_DM` | USB-C D- through 22R |
| `PA12` | BiDir | `USB_DP` | USB-C D+ through 22R |
| `PA13` | SWD | `SWDIO` | SWD header |
| `PA14-BOOT0` | SWD/In | `SWCLK` + BOOT0 | SWD header + BOOT0 button + `R57` 10k pulldown |
| `NRST` | In | Reset | NRST button + SWD header |
| `PB4` | Out | `LED_CH0_N` | CH0 output LED cathode |
| `PB5` | Out | `LED_CH1_N` | CH1 output LED cathode |
| `PB6` | Out | `LED_CH2_N` | CH2 output LED cathode |
| `PB7` | Out | `LED_CH3_N` | CH3 output LED cathode |
| `PB8` | Out | `LED_CH4_N` | CH4 output LED cathode |
| `PB9` | Out | `LED_CH5_N` | CH5 output LED cathode |
| `PC6` | Out | `LED_CH6_N` | CH6 output LED cathode |
| `PC7` | Out | `LED_CH7_N` | CH7 output LED cathode |
| `PC14` | Out | `LED_LINK_N` | Link LED cathode |

Note: power LED is not MCU-driven; wire as always-on from `3V3` through resistor to LED.

## 2) Connector pin map (output board)

### `J1` (2-pos, 7.5 mm, 24A, board 12V input)

| Pin | Net |
| --- | --- |
| 1 | `VIN_12V_IN` |
| 2 | `GND` |

### `J2` (6-pos, RS-485 terminal)

| Pin | Net |
| --- | --- |
| 1 | `RS485_TX+` |
| 2 | `RS485_TX-` |
| 3 | `RS485_RX+` |
| 4 | `RS485_RX-` |
| 5 | `GND` |
| 6 | `SHIELD` |

### `J3a` (4-pos, OVR0..OVR3)

| Pin | Net |
| --- | --- |
| 1 | `OVR_CH0_RAW` |
| 2 | `OVR_CH1_RAW` |
| 3 | `OVR_CH2_RAW` |
| 4 | `OVR_CH3_RAW` |

### `J3b` (4-pos, OVR4..OVR7)

| Pin | Net |
| --- | --- |
| 1 | `OVR_CH4_RAW` |
| 2 | `OVR_CH5_RAW` |
| 3 | `OVR_CH6_RAW` |
| 4 | `OVR_CH7_RAW` |

### `J4` (2-pos, override GND/GND)

| Pin | Net |
| --- | --- |
| 1 | `GND` |
| 2 | `GND` |

### `J5a` (4-pos, OUT0..OUT3 load low-side terminals)

The load low side connects here; the net is `OUT_CHk` (before the per-channel fuse). On the board: `OUT_CHk` -> fuse -> `OUT_CHk_SW` (MOSFET drain).

| Pin | Net |
| --- | --- |
| 1 | `OUT_CH0` |
| 2 | `OUT_CH1` |
| 3 | `OUT_CH2` |
| 4 | `OUT_CH3` |

### `J5b` (4-pos, OUT4..OUT7 load low-side terminals)

| Pin | Net |
| --- | --- |
| 1 | `OUT_CH4` |
| 2 | `OUT_CH5` |
| 3 | `OUT_CH6` |
| 4 | `OUT_CH7` |

### `J6` (2-pos, 7.5 mm, 24A, load +12V supply out)

Both poles are the **same** net, `12V_MAIN` (paralleled), giving two heavy +12V terminals for distributing the load feed to multiple loads. (There is no separate `LOAD_12V` net - the load +12V feed is just `12V_MAIN` brought out at `J6`.) The load ground return is **internal only** - the MOSFET sources tie to board `GND` (net `LOAD_GND_RTN`) and return to the PSU negative via `J1.2`; it is intentionally not exposed on `J6`, so a load can never be wired to bypass its low-side switch.

| Pin | Net |
| --- | --- |
| 1 | `12V_MAIN` |
| 2 | `12V_MAIN` |

### `J7` (USB-C receptacle)

| Connector function | Net |
| --- | --- |
| D+ | `USB_DP` (clamped by `D20.IO1` before MCU) |
| D- | `USB_DM` (clamped by `D20.IO2` before MCU) |
| CC1 | `USB_CC1` (5.1k pulldown to GND via `R55`) |
| CC2 | `USB_CC2` (5.1k pulldown to GND via `R56`) |
| VBUS | `USB_VBUS` (also clamped by `D20.VBUS`) |
| GND/shell | `GND` / chassis policy per PCB appendix |

`D20` (`USBLC6-2SC6`, LCSC `C7519`, SOT-23-6) pinout (per ST datasheet, top view: `1=I/O1, 2=GND, 3=I/O2, 4=I/O2, 5=VBUS, 6=I/O1`; pins 1&6 are internally common I/O1, pins 3&4 are internally common I/O2):

| Pin | Signal | Net |
| --- | --- | --- |
| 1 | `I/O1` | `USB_DP` (on the `J7.D+` side of `R53`) |
| 2 | `GND` | `GND` |
| 3 | `I/O2` | `USB_DM` (on the `J7.D-` side of `R54`) |
| 4 | `I/O2` | `USB_DM` (internally common with pin 3; route to the same net or leave open) |
| 5 | `VBUS` | `USB_VBUS` |
| 6 | `I/O1` | `USB_DP` (internally common with pin 1; route to the same net or leave open) |

### `J8` (2x5 SWD, 1.27 mm ARM convention)

| Pin | Net |
| --- | --- |
| 1 | `VTREF_3V3` |
| 2 | `SWDIO` |
| 3 | `GND` |
| 4 | `PA14-BOOT0` (`SWCLK`) |
| 5 | `GND` |
| 6 | `SWO_NC` |
| 7 | `KEY_NC` |
| 8 | `NC` |
| 9 | `GND` |
| 10 | `NRST` |

## 3) Power/junction endpoint map (output board)

| Source | Through | Destination |
| --- | --- | --- |
| `VIN_12V_IN` (`J1.1`) | `F9` pins 1-4 (input blade) -> pins 5-8 (output blade) | `D1` anode |
| `D1` anode | `D1` (`SS34`) reverse-polarity stage | `12V_MAIN` |
| `12V_MAIN` | `C17` + `C18` (both polarized, `+` to `12V_MAIN`) bulk + `C29` HF bypass | `GND` return |
| `12V_MAIN` | `U4.VIN` (`AP63203WU-7`) | buck input stage |
| `U4.SW` | `L1` | `3V3` |
| `U4.BST` | `C20` to `U4.SW` | bootstrap drive loop |
| `3V3` | `C19` + `C30` (both 22uF, polarized `+` to `3V3`) + `C1` + `C2-C7` + `C15` decoupling network | digital logic domain |
| `3V3` | `C31` (`U2A.VCC`), `C32` (`U2B.VCC`), `C33` (`U5.VCC`), `C34` (`U6.VCC`) - 1uF each | `GND` return at each IC |
| `12V_MAIN` | direct feed (same net) | `J6.1` + `J6.2` (paralleled +12V load-feed terminals) |
| MOSFET sources (`LOAD_GND_RTN`, internal) | board `GND` tie, wide copper | `J1.2` (`GND`) -> PSU negative |

## 4) RS-485 transceiver endpoint map (output board)

### `U2A` (RX path, always receive)

| Transceiver pin role | Net |
| --- | --- |
| `RO` | `USART1_RX` (`PA10`) |
| `DE` | `GND` |
| `/RE` | `GND` |
| `A/B` | `RS485_RX+` / `RS485_RX-` via `J2.3/J2.4` |
| Termination | `R52 = 120R` across `RS485_RX+/-` |

### `U2B` (TX path, always transmit)

| Transceiver pin role | Net |
| --- | --- |
| `DI` | `USART1_TX` (`PA9`) |
| `DE` | `3V3` |
| `/RE` | `3V3` |
| `A/B` | `RS485_TX+` / `RS485_TX-` via `J2.1/J2.2` |

Protection devices:

- `D18` (`SM712`) on TX pair.
- `D19` (`SM712`) on RX pair.

## 5) Override CH0..CH7 junction map

Schmitt stage pin convention for `U5/U6` (`SN74LV14APWR`):

- `A` pins are inputs (from RC node), `Y` pins are outputs (to `_SENSE` net).
- Pin map: `1A/1Y=1/2`, `2A/2Y=3/4`, `3A/3Y=5/6`, `4A/4Y=9/8`, `5A/5Y=11/10`, `6A/6Y=13/12`.

Per channel override chain:

```text
J3x.OVR_CHn -> OVR_CHn_RAW -> pull-up resistor from `R17-R32` set to 3V3, switch to GND
              -> series resistor from `R17-R32` set -> RC node with channel capacitor `C21..C28` to GND
              -> SN74LV14 A-input -> SN74LV14 Y-output -> OVR_CHn_SENSE (MCU GPIO)
```

| Channel | Override connector pin | MCU GPIO |
| --- | --- | --- |
| CH0 | `J3a.1` | `PA0` |
| CH1 | `J3a.2` | `PA1` |
| CH2 | `J3a.3` | `PA4` |
| CH3 | `J3a.4` | `PA5` |
| CH4 | `J3b.1` | `PA6` |
| CH5 | `J3b.2` | `PA7` |
| CH6 | `J3b.3` | `PB0` |
| CH7 | `J3b.4` | `PB1` |

Override RC capacitor assignment (output board):

| Channel | RC capacitor |
| --- | --- |
| CH0 | `C21` |
| CH1 | `C22` |
| CH2 | `C23` |
| CH3 | `C24` |
| CH4 | `C25` |
| CH5 | `C26` |
| CH6 | `C27` |
| CH7 | `C28` |

## 6) Output-channel endpoint map (serial/override to load terminal)

| CH | Gate GPIO | MOSFET | Load terminal (`OUT_CHk`) | LED GPIO |
| --- | --- | --- | --- | --- |
| CH0 | `PB2` | `Q1` | `J5a.1` | `PB4` |
| CH1 | `PB10` | `Q2` | `J5a.2` | `PB5` |
| CH2 | `PB11` | `Q3` | `J5a.3` | `PB6` |
| CH3 | `PB12` | `Q4` | `J5a.4` | `PB7` |
| CH4 | `PB13` | `Q5` | `J5b.1` | `PB8` |
| CH5 | `PA8` | `Q6` | `J5b.2` | `PB9` |
| CH6 | `PA15` | `Q7` | `J5b.3` | `PC6` |
| CH7 | `PB3` | `Q8` | `J5b.4` | `PC7` |

Per-channel power path (low-side switch, shared +12V feed):

- High side (shared): `12V_MAIN` -> `J6.1`/`J6.2` (both poles) -> external load high side. One common feed for all eight loads; **not** fused per channel.
- Low side (per channel): external load low side -> `J5a/J5b.k` terminal (net `OUT_CHk`) -> `F(k)` -> `OUT_CHk_SW` (`Q(k).D`) -> `Q(k)` -> `Q(k).S` -> `LOAD_GND_RTN` (internal MOSFET-source ground; ties to board `GND` and returns to the PSU via `J1.2`).
- Flyback `D(k)` is across the load: anode -> `OUT_CHk` (terminal, load side of the fuse); cathode -> `12V_MAIN`. Freewheel current bypasses the fuse and MOSFET.

**How a load is wired (important):** connect the load between a `J6` `12V_MAIN` pole (+12V) and its channel terminal `J5a/J5b.k` (switched low side). The load's return is its `J5` pin - it goes through the channel MOSFET to ground inside the box. Loads must NEVER be returned to a ground terminal; there is no load-ground terminal, precisely so a load cannot bypass its switch.

So each channel has two nets separated by its fuse: `OUT_CHk` (the `J5` terminal / load low side / flyback anode) and `OUT_CHk_SW` (the MOSFET drain).

Channel power-part mapping:

| Channel | Terminal | Fuse | MOSFET | Flyback diode |
| --- | --- | --- | --- | --- |
| CH0 | `J5a.1` | `F1` | `Q1` | `D2` |
| CH1 | `J5a.2` | `F2` | `Q2` | `D3` |
| CH2 | `J5a.3` | `F3` | `Q3` | `D4` |
| CH3 | `J5a.4` | `F4` | `Q4` | `D5` |
| CH4 | `J5b.1` | `F5` | `Q5` | `D6` |
| CH5 | `J5b.2` | `F6` | `Q6` | `D7` |
| CH6 | `J5b.3` | `F7` | `Q7` | `D8` |
| CH7 | `J5b.4` | `F8` | `Q8` | `D9` |

`J6.1` and `J6.2` are both `12V_MAIN` (paralleled poles; the load +12V feed), together delivering up to 16A. The load ground return is internal only: the MOSFET sources (net `LOAD_GND_RTN`) tie to board `GND` and return to the PSU negative via `J1.2` - it is not exposed on any terminal (`J6` = `KF128-7.5-2P`, see section 2).

## 7) Cable crossover map (output side view)

| Output board net | Input board destination |
| --- | --- |
| `RS485_RX+` | `RS485_TX+` |
| `RS485_RX-` | `RS485_TX-` |
| `RS485_TX+` | `RS485_RX+` |
| `RS485_TX-` | `RS485_RX-` |
| `GND` | `GND` |
| `SHIELD` | `SHIELD`/chassis entry |

## 8) Polarized capacitor polarity reference (review fix OP2/OP3)

Reverse polarity will cause venting/rupture at first power-on. Confirmed orientations on the output board:

| Designator | Value | `+` net | `-` net |
| --- | --- | --- | --- |
| `C17` | bulk on `12V_MAIN` | `12V_MAIN` | `GND` |
| `C18` | bulk on `12V_MAIN` | `12V_MAIN` | `GND` |
| `C19` | 22uF on `3V3` | `3V3` | `GND` |
| `C30` | 22uF on `3V3` (added) | `3V3` | `GND` |

PCB silkscreen `+` mark must agree with schematic `+` pin before fab release.

## 9) Per-IC `1uF` VCC bypass map (review fix OP4)

Each cap goes from the IC VCC pin DIRECTLY to local GND, supplementing the existing 100nF HF caps (do not replace, place in parallel).

| Designator | IC | Function |
| --- | --- | --- |
| `C31` | `U2A` | RS-485 RX transceiver bypass |
| `C32` | `U2B` | RS-485 TX transceiver bypass |
| `C33` | `U5` | Schmitt OVR0..OVR5 bypass |
| `C34` | `U6` | Schmitt OVR6..OVR7 bypass |

## 10) USB ESD protection map (review fix OP5)

| Designator | Part | Pin map |
| --- | --- | --- |
| `D20` | `USBLC6-2SC6` (LCSC `C7519`, SOT-23-6) | IO1->`USB_DP`, IO2->`USB_DM`, VBUS->`USB_VBUS`, GND->`GND` |

Insertion topology (clamp on the connector side of the series resistor): `J7.D+` -> `D20.IO1` (clamp) -> `R53` (22R) -> `U1.PA12` (and same for the D-/IO2 path through `R54`). The ESD diode must be upstream of the series resistor so the strike is shunted before reaching `R53`/the MCU. Place within 5 mm of `J7` per `PCB_APPENDIX_OUTPUT.md`.

## 11) Output board input fuse (review fix OP1)

| Designator | Part | Wiring |
| --- | --- | --- |
| `F9` | Littelfuse `178.6165.0002` (LCSC `C207061`), 4-pin PCB-mount ATO blade fuse holder, 22.5A continuous / 30A max, 32V | `VIN_12V_IN` -> `F9` (pins 1-4) ... (pins 5-8) -> `D1.A`, in series before the reverse-polarity diode |
| `F9` fuse element | **25A ATO blade fuse**, 32V (any brand; field-inserted accessory, not placed by PCBA) | inserted into `F9` holder |

### `F9` pin map (EasyEDA `C207061` footprint = 8 symbol pins)

The EasyEDA footprint exposes 8 symbol pins that are doubled onto the holder's 4 physical solder legs (2 symbol pins per leg), and the 4 legs are 2 per fuse-blade contact. So the part presents exactly **two electrical nodes** - the two blade contacts:

| EasyEDA pin | Physical leg | Blade contact | Net |
| ---: | ---: | --- | --- |
| 1 | leg 1 | input blade | `VIN_12V_IN` |
| 2 | leg 1 | input blade | `VIN_12V_IN` |
| 3 | leg 2 | input blade | `VIN_12V_IN` |
| 4 | leg 2 | input blade | `VIN_12V_IN` |
| 5 | leg 3 | output blade | `F9_OUT` -> `D1.A` |
| 6 | leg 3 | output blade | `F9_OUT` -> `D1.A` |
| 7 | leg 4 | output blade | `F9_OUT` -> `D1.A` |
| 8 | leg 4 | output blade | `F9_OUT` -> `D1.A` |

Rules:

- All four pins of a blade contact carry the same net and **must all be soldered** and joined with wide copper (they exist to share the 16A current). Pins 1-4 = `VIN_12V_IN`; pins 5-8 = the fused output to `D1` anode.
- The holder is **non-polarized** (the two blade contacts are interchangeable), so input/output may be swapped, but pins 1-4 must never share a net with pins 5-8 (that would short the fuse).
- **Verify against the footprint before routing:** confirm that pins 1-4 land on one blade slot and pins 5-8 on the other. If the EasyEDA symbol numbers the legs in a different order, group by physical blade slot, not by pin number.

`F9` sits upstream of `12V_MAIN` and therefore carries the **entire board current** (buck ~0.15A + up to 8 x 2A = 16A of channel load). Sizing rationale:

- The 1-pin ATO PCB holder is only rated 11A continuous, so the **4-pin** `178.6165.0002` (22.5A continuous / 30A max) is required to carry 16A without overheating.
- Automotive blade fuses derate to roughly 70-80% for continuous load, so a **25A** fuse carries 16A continuous comfortably (~18A continuous capability) while still opening on a hard fault. A 20A fuse would derate to ~14-16A continuous and risks nuisance fatigue at full load.
- The `LRS-200-12` PSU folds back near ~17-18A, covering the 16-25A band; `F9` is the field-replaceable backstop for a catastrophic short (e.g. shorted bulk cap or `12V_MAIN` copper).

Layout/routing implications (see `PCB_APPENDIX_OUTPUT.md`):

- `F9` is a THT part with a sizeable footprint; reserve power-entry-zone area. JLCPCB may require hand-soldering for this holder.
- The `VIN_12V_IN -> F9 -> D1 -> 12V_MAIN` trunk must be sized for 16A continuous (wide pour / heavy copper).
- `J1` and `J6` are both `KF128-7.5-2P` (LCSC `C474954`), 7.5mm pitch, **24A / 450V, 12-22 AWG** screw terminals. `J1` carries the 16A board input trunk (`VIN_12V_IN` + `GND` return); `J6` carries the load +12V feed on both poles (`12V_MAIN` on `J6.1` and `J6.2`, paralleled), up to 16A total. Use >=14 AWG (12 AWG preferred) wiring on both. Their footprints are larger than the 5.08mm `DB128V` terminals - reserve area in the power-entry and load zones.

## 12) MCU GPIO pin sink-current verification (review fix OP6)

Per-channel LED current calculation:

- `I_pin = (3.3V - V_F_LED) / R_series`
- For 330R + V_F = 2.0V: `I_pin = 1.3 / 330 ~= 3.94mA` per CH LED
- For 150R + V_F = 2.0V: `I_pin = 1.3 / 150 ~= 8.67mA` for LINK LED (`R51`)
- STM32G0B1 datasheet limits: `Iol_max = 20mA` per pin; recommended <=8mA for digital integrity
- Sustained worst-case: 8 channel LEDs + 1 LINK LED = `8 * 3.94 + 8.67 ~= 40.2mA` group total (within typ. 80mA per port group)

Independent of LEDs, the gate-drive GPIOs (`PB2`, `PB10`, `PB11`, `PB12`, `PB13`, `PA8`, `PA15`, `PB3`) drive only MOSFET gate capacitance through a series gate resistor; no DC current beyond switching transients, so no per-pin sink-current concern beyond LED budget.

## 13) BOM additions summary (review fixes)

| Designator | Value | LCSC / MPN | BOM line |
| --- | --- | --- | --- |
| `C30` | 22uF 10V tantalum | `C11366` / TAJA226K010RNJ | merged onto the `C19` line (qty 2) |
| `C31` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `C32` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `C33` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `C34` | 1uF 0603 | `C15849` / CL10A105KB8NNNC | merged onto the `C15` line (qty 5) |
| `D20` | `USBLC6-2SC6` SOT-23-6 | `C7519` / USBLC6-2SC6 | own line (qty 1) |
| `F9` | ATO blade fuse holder, 4-pin THT | `C207061` / Littelfuse 178.6165.0002 | own line (qty 1) |

The capacitor additions reuse part numbers already on the BOM; `D20` uses `C7519` (USBLC6-2SC6, ST) and `F9` uses `C207061`, both confirmed in LCSC/JLCPCB stock. The `F9` 25A ATO blade fuse element is a separately-procured accessory (not placed by PCBA). See section 11 for sizing rationale.
