# Output Board Design

## 1. Overview

The output board receives 8 channel states over RS-485 (via the **Hotline v2** protocol) and drives 8 high-current 12V MOSFET outputs. Each channel has an optional override input for local control. A communication watchdog turns all outputs off if the serial link is lost.

| Parameter | Value |
|-----------|-------|
| MCU | STM32G070CBT6 (Cortex-M0+, 64MHz, 128KB flash, 36KB SRAM, LQFP48) |
| Output channels | 8 (12V, up to 2A per channel) |
| Input | RS-485 serial (3-byte frame at 1kHz) |
| Override inputs | 8 (3.3V–12V, three-state detection) |
| Power input | 12V DC via screw terminal (from Mean Well LRS-200-12) |
| Logic rail | 3.3V via AMS1117-3.3 LDO |
| Status LEDs | 10 (8 channel + 1 power + 1 link) |
| Debug | USB-C (CH340C UART bridge) + SWD header |
| Failsafe | All outputs OFF after 100ms with no valid frames |

---

## 2. Power Section

### Power Supply Chain

```
120V AC (IEC C14 inlet on enclosure)
    │
    ▼
Mean Well LRS-200-12 (off-board, chassis-mount inside enclosure)
    120V AC → 12V DC, 200W (16.7A), enclosed, fanless, ~$28
    │
    ▼ (heavy-gauge wire to PCB screw terminal)
12V DC Screw Terminal (J_PWR)
    │
    ├──► D_RPP (SS34 Schottky, SMA) — reverse polarity protection
    │        │  (drops ~0.5V, giving ~11.5V rail — above LDO dropout)
    │        ▼
    ├──► F_INPUT (20A automotive blade fuse holder) — input rail overcurrent protection
    │        │
    │        ▼
    ├──► 470μF/25V electrolytic (C_BULK1) — bulk energy for load switching
    ├──► 100μF/25V electrolytic (C_BULK2) — additional bulk storage
    ├──► 100nF/50V ceramic 0603 (C_IN) — high-frequency decoupling
    │
    ├──► 12V Rail ──► Output MOSFET drains (via PTC fuses + load terminals)
    │
    └──► AMS1117-3.3 (U_LDO, SOT-223)
            │
            ├──► 22μF/10V tantalum (C_OUT1) — LDO output stability
            ├──► 100nF ceramic 0603 (C_OUT2) — high-frequency decoupling
            │
            └──► 3.3V Rail — MCU, RS-485, LEDs, override bias
```

### Why LRS-200-12

The output board drives loads up to 8 × 2A = 16A at 12V = **192W peak**. A 15W supply (IRM-15-12) cannot power these loads. The LRS-200-12 provides 200W (17A), giving 8W headroom above worst case. It is a single supply that powers both the board electronics (~4.3W) and all load channels.

**Important: Voltage selector switch**: The LRS-200-12 has a manual 115V/230V input voltage selector switch on its case. It must be set to the 115V position for 120V North American mains. If set to 230V and powered with 120V, the PSU may not start or will output insufficient voltage. The IRM-15-12 (input board) is auto-ranging (85–264VAC) and needs no switch.

**Important: Output voltage trim**: The LRS-200-12 output voltage trim pot must NOT be adjusted above 12.0V (factory default). The per-channel PTC fuses (1812L200/12GR) are rated for a maximum of 12V; exceeding this risks the tripped fuse not limiting current properly.

### Power Budget

| Consumer | Rail | Current | Power |
|----------|------|---------|-------|
| STM32G070CBT6 | 3.3V | 20mA | 66mW |
| SP3485EN (RX mode) | 3.3V | 0.5mA | 2mW |
| CH340C (USB connected) | 5V USB | 10mA | — (from USB host) |
| 10x LEDs @ 5mA | 3.3V | 50mA | 165mW |
| Override bias resistors (8x) | 3.3V | 0.3mA | 1mW |
| **3.3V rail total** | 3.3V | **~71mA** | 234mW |
| **LDO dissipation** | — | — | (12−3.3) × 71mA = **618mW** |
| 8x outputs @ 2A (through loads) | 12V | 16A max | 192W peak |
| MOSFET losses (8x × 2A × 37mΩ) | — | — | 8 × 0.148W = **1.18W** |
| PTC fuse losses (8x × 2A × ~70mΩ worst case) | — | — | 8 × 0.28W = **~2.24W** |
| **Total from 12V (electronics)** | 12V | ~350mA | ~4.3W |
| **Total from 12V (with loads)** | 12V | ~16.35A | ~197W |

### LDO Thermal

Same as input board: AMS1117-3.3 in SOT-223 dissipates ~618mW. **Mandatory layout requirement**: copper pour under the SOT-223 thermal pad is required (reduces θ_JA from ~90°C/W to ~46°C/W). At 618mW with copper pour, junction temperature rise is ~28°C above ambient (safe). Without copper pour, the rise would be ~56°C (at 90°C/W), still manageable but with less margin at elevated ambient.

### Bulk Capacitance

The output board uses 470μF + 100μF electrolytic capacitors on the 12V rail. The extra capacitance (vs. input board's 100μF) absorbs current transients when multiple output channels switch simultaneously, preventing voltage droop that could cause LDO brownout or affect MOSFET gate drive.

---

## 3. Output Stage

### Circuit (Per Channel)

Each of the 8 output channels uses a low-side N-channel MOSFET switch with per-channel PTC fuse protection:

```
12V Rail
    │
    ├──► PTC Fuse (1812L200/12GR, 1812 SMD)
    │        2A hold, 3.5A trip, 12V rated
    │        │
    │        └──► Output Screw Terminal (to load +)
    │                  │
    │                  ▼
    │             External Load
    │             (solenoid / relay)
    │                  │
    │                  ▼
    │             MOSFET Drain ◄──── SS34 Flyback Diode (cathode → 12V)
    │                  │                  3A 40V Schottky, SMA
    │                  │
    │             IRLML6344TRPbF (SOT-23)
    │             N-Channel MOSFET
    │             R_DS(on) = 37mΩ @ V_GS = 2.5V
    │             V_DS(max) = 30V
    │                  │
    │             MOSFET Source ──► GND
    │
    │             MOSFET Gate ◄──── R_GATE (100Ω, 0603)
    │                  │                    │
    │                  │               MCU GPIO Pin
    │                  │
    │                  └──► R_PULL (10kΩ, 0603) ──► GND
    │                        Gate pull-down (default off)
```

### MOSFET Selection: IRLML6344TRPbF

| Parameter | Value |
|-----------|-------|
| Manufacturer | Infineon |
| Package | SOT-23 |
| V_DS(max) | 30V |
| V_GS(th) | 0.8V typical |
| R_DS(on) @ V_GS = 2.5V | 37mΩ |
| R_DS(on) @ V_GS = 4.5V | 29mΩ (max) |
| I_D continuous | 5A (package limited) |
| Power dissipation @ 2A | I²R = 4 × 0.037 = **0.148W** |

Compared to the previously considered SI2302CDS (80mΩ, V_DS=20V), the IRLML6344 provides:
- **2.2x lower on-resistance** → 2.2x less heat (0.148W vs 0.32W at 2A)
- **50% more drain voltage headroom** (30V vs 20V)
- Same SOT-23 package and approximately the same price (~$0.15)
- Verified on Infineon datasheet and DigiKey availability

### PTC Fuse: 1812L200/12GR

| Parameter | Value |
|-----------|-------|
| Hold current (I_hold) | 2.0A |
| Trip current (I_trip) | 3.50A |
| Maximum voltage | 12V |
| Package | 1812 SMD |
| Typical resistance | 20–70mΩ (datasheet range; worst case 70mΩ) |

Each output channel has its own PTC fuse between the 12V rail and the load terminal. This protects:
- The MOSFET from sustained overcurrent if the load shorts
- The PCB trace from overheating
- The power supply from overload

The PTC fuse is resettable — it trips at ~3.5A, increasing resistance dramatically to limit current, and resets when the fault is removed and the fuse cools.

**Important**: The 12V PTC fuse voltage rating means the LRS-200-12 output voltage trim pot must NOT be adjusted above 12.0V (factory default). The SS34 flyback diode absorbs inductive spikes before they reach the PTC.

### Flyback Diode: SS34

The SS34 (3A 40V Schottky, SMA package) clamps the inductive kickback voltage when a solenoid or relay coil is de-energized. The diode is placed across the load terminals (cathode to 12V, anode to MOSFET drain) so the collapsing magnetic field energy dissipates through the diode rather than through the MOSFET.

### Gate Drive

- **R_GATE (100Ω)**: Limits gate charge current to ~33mA peak (3.3V / 100Ω), reducing EMI from fast switching edges while still providing sub-microsecond switching times (IRLML6344 total gate charge Qg ≈ 7nC).
- **R_PULL (10kΩ)**: Pulls the gate to GND when the MCU GPIO is in high-impedance state (during reset, boot, or fault). This ensures all outputs default to OFF.

### Thermal Analysis

At 2A continuous per channel with IRLML6344:
- MOSFET power: 0.148W per device, SOT-23 θ_JA = 100°C/W → ΔT = 14.8°C above ambient
- PTC fuse power: ~0.28W per device (worst case 70mΩ), self-heating is by design (it's how the fuse operates)
- Total per channel: ~0.43W dissipated on PCB
- All 8 channels: ~3.4W total — manageable with adequate copper pour

---

## 4. Override Detection (Corrected)

### Why the Original Design Was Broken

The original design ran override inputs through the same 27k/10k voltage divider as the input board, then added 100k/100k mid-rail bias resistors. This created a circuit interaction:

| Override Input | Expected ADC | Actual ADC (broken) | Problem |
|---------------|-------------|---------------------|---------|
| Floating | 1.65V | 1.65V | OK |
| 3.3V HIGH | >2.4V | ~0.99V | Falls in dead zone or reads as LOW |
| 5V HIGH | >2.4V | ~1.39V | Falls in dead zone |
| 12V HIGH | >2.4V | ~2.45V | Barely works |

The voltage divider attenuated the override signal before it reached the mid-rail bias network, making low-voltage overrides undetectable.

### Corrected Circuit (Per Channel)

The corrected design removes the voltage divider entirely. Instead, it uses:
1. **10kΩ series resistor** — current limiting (protects ADC pin and Zener from high-current drive)
2. **BZT52C3V3-7-F Zener clamp** — overvoltage protection (clamps to ~3.3V for inputs >3.3V)
3. **100k/100k mid-rail bias** — floating state detection (on the ADC pin, not after a divider)

```
External Override Input (3.3V–12V)
    │
    ├──► R_SERIES (10kΩ, 0603) — current limiting
    │        │
    │        ├──► Zener Node
    │        │        │
    │        │        ├──► D_ZENER (BZT52C3V3-7-F, SOD-123) ──► GND
    │        │        │      Clamps to 3.3V for overvoltage
    │        │        │
    │        │        ├──► C_FILT (10nF, 0603 ceramic) ──► GND
    │        │        │      ADC noise filter
    │        │        │
    │        │        └──► ADC Pin ◄──┬── R_HIGH (100kΩ, 0603) ──► 3.3V
    │        │                        │
    │        │                        └── R_LOW (100kΩ, 0603) ──► GND
    │        │                             Mid-rail bias: ~1.65V when floating
    │
Override Screw Terminal (per channel)
```

### Corrected ADC Math

**Floating (disconnected)**: Only the 100k/100k divider matters.
- V_adc = 3.3V × 100k / (100k + 100k) = **1.65V** (clean mid-rail)

**Driven HIGH at 3.3V**: The 10k series resistor + 100k||100k (50k) parallel bias form a divider:
- V_adc = 3.3V × 50k / (10k + 50k) = **2.75V** (well above HIGH threshold)
- More precise with Thevenin: bias Thevenin = 1.65V with 50k impedance. 3.3V through 10k into 50k Thevenin:
  V_adc = (3.3V/10k + 1.65V/50k) / (1/10k + 1/50k) = (0.33mA + 0.033mA) / (0.1mS + 0.02mS) = **3.03V**

**Driven HIGH at 5V**: 5V through 10k, clamped by Zener at ~3.3V.
- Zener conducts, clamping the node to **~3.3V**. Excess current (5−3.3)/10k = 0.17mA through Zener.

**Driven HIGH at 12V**: Same Zener clamping.
- V_adc = **~3.3V**. Excess current (12−3.3)/10k = 0.87mA through Zener (well within BZT52C3V3-7-F rating of 300mW).

**Driven LOW at 0V**: External source pulls through 10k against the 100k bias.
- V_adc = (0V/10k + 1.65V/50k) / (1/10k + 1/50k) = (0 + 0.033mA) / 0.12mS = **0.275V** (well below LOW threshold)

### ADC Threshold Summary

| State | ADC Voltage | Detection |
|-------|-------------|-----------|
| Floating (disconnected) | ~1.65V | Mid-band → serial controls output |
| Driven LOW (0V) | ~0.275V | Below LOW threshold → override OFF |
| Driven HIGH (3.3V) | ~3.03V | Above HIGH threshold → override ON |
| Driven HIGH (5V+) | ~3.3V (clamped) | Above HIGH threshold → override ON |

All three states are cleanly separated with generous margins.

### Firmware Hysteresis Bands

| Transition | Threshold | Band |
|-----------|-----------|------|
| Enter HIGH | >2.4V | — |
| Exit HIGH | <2.1V | 0.3V hysteresis |
| Enter LOW | <0.9V | — |
| Exit LOW | >1.2V | 0.3V hysteresis |
| Mid-band (floating) | 1.2V – 2.1V | 0.9V dead zone |

The 0.3V hysteresis on each threshold prevents noise-triggered oscillation between states.

---

## 5. RS-485 Interface

### Circuit

```
SP3485EN RO (pin 1) ──────► STM32 USART1_RX (PA10)
STM32 GPIO (PA12) ────────────► SP3485EN DE (pin 3) + RE̅ (pin 2)
                                    (tied together, active LOW = RX enable)
STM32 USART1_TX (PA9) ──────► SP3485EN DI (pin 4)
                                    (heartbeat TX to input board)

SP3485EN VCC (pin 8) ──► 3.3V + 100nF decoupling
SP3485EN GND (pin 5) ──► GND

SP3485EN A (pin 6) ──────► RS-485 screw terminal A (+)
SP3485EN B (pin 7) ──────► RS-485 screw terminal B (−)

SM712 TVS diode (SOT-23) across A and B lines for transient voltage suppression
    Asymmetric clamping: −7V to +12V (matches RS-485 common-mode range)

120Ω termination resistor between A and B (on PCB, at receiver end)
```

### Operating Mode

The DE/RE pin defaults to LOW (RX mode) at boot. When a heartbeat is due (~every 100ms), firmware briefly asserts DE/RE HIGH to transmit the 3-byte heartbeat frame ([0x55][status][CRC8]) during the bus turnaround window after receiving a state frame, then returns DE/RE to LOW. USART1_TX drives the heartbeat via SP3485EN DI.

### Bus Termination

A 120Ω resistor (0603) is placed between the A and B lines directly at the SP3485EN pins. This matches the Belden 9841 cable's 120Ω characteristic impedance and prevents signal reflections at the end of the cable run.

### Fail-Safe Bus Biasing

Two 390Ω fail-safe bias resistors are placed at the receiver end (output board):

```
3.3V ──► R_BIAS_A (390Ω, 0603) ──► RS-485 A (+)
GND  ◄── R_BIAS_B (390Ω, 0603) ◄── RS-485 B (−)
```

During bus idle periods (when neither transceiver drives the bus — e.g., during turnaround windows), these bias resistors pull A high and B low, creating a positive differential (V_A − V_B > 200mV). This ensures the receiver interprets idle as logic HIGH (mark/idle state), preventing false byte reception from floating bus noise.

Per TIA/EIA-485, the combined common-mode loading (~375Ω equivalent) stays within the standard's receiver input impedance specification. No bias resistors are placed on the input board (transmitter end).

---

## 6. USB Debug Interface

Identical circuit to the input board:

```
USB-C Connector
    │
    ├── VBUS (5V) ───────► CH340C VCC (pin 16)
    ├── D+ ──────────────► CH340C UD+ (pin 5)
    ├── D− ──────────────► CH340C UD− (pin 6)
    ├── GND ─────────────► CH340C GND (pin 1)
    │
    └── CC1, CC2 ────────► 5.1kΩ pull-down to GND each

CH340C V3 (pin 4) ──► 100nF to GND
CH340C TXD (pin 2) ──► STM32 USART2_RX (PA3)
CH340C RXD (pin 3) ──► STM32 USART2_TX (PA2)
```

CH340C powered from USB VBUS (5V), not the board's 3.3V rail.

---

## 7. Status LEDs

### LED Circuit (Per LED)

Same as input board:

```
3.3V ──► LED (0603) ──► R_LED (330Ω, 0603) ──► MCU GPIO Pin
                                                  (active LOW)
```

### LED Assignments

| Position | Label | Color | Function | Drive Logic |
|----------|-------|-------|----------|-------------|
| 1 | PWR | Green | Power indicator | Always ON (tied to 3.3V, no GPIO) |
| 2 | LINK | Blue | RX activity / link health | Blinks on valid frame; error pattern on watchdog timeout |
| 3 | CH0 | Yellow | Channel 0 output state | ON when output is active (regardless of source: serial or override) |
| 4 | CH1 | Yellow | Channel 1 output state | ON when output is active |
| 5 | CH2 | Yellow | Channel 2 output state | ON when output is active |
| 6 | CH3 | Yellow | Channel 3 output state | ON when output is active |
| 7 | CH4 | Yellow | Channel 4 output state | ON when output is active |
| 8 | CH5 | Yellow | Channel 5 output state | ON when output is active |
| 9 | CH6 | Yellow | Channel 6 output state | ON when output is active |
| 10 | CH7 | Yellow | Channel 7 output state | ON when output is active |

The channel LEDs show the **actual output state** after override merge — not just the serial state. If an override is forcing a channel ON but serial says OFF, the LED shows ON.

---

## 8. SWD Debug Header

Identical to input board. Standard ARM 10-pin (2×5, 1.27mm pitch):

| Pin | Signal | Connection |
|-----|--------|------------|
| 1 | VTref | 3.3V |
| 2 | SWDIO | STM32 PA13 |
| 3 | GND | GND |
| 4 | SWCLK | STM32 PA14 |
| 5 | GND | GND |
| 6 | SWO | NC |
| 7 | — | NC |
| 8 | — | NC |
| 9 | GND | GND |
| 10 | NRST | STM32 NRST |

---

## 9. Complete GPIO Pin Assignment

### STM32G070CBT6 (LQFP48)

The output board uses more GPIO than the input board (34 pins vs 24) due to the 8 override ADC inputs and 8 MOSFET gate drives in addition to LEDs and peripherals.

| Pin | Port.Bit | Function | Peripheral | Notes |
|-----|----------|----------|------------|-------|
| 11  | PA0 | Override ADC CH0 | ADC1_IN0 | Override detection channel 0 |
| 12  | PA1 | Override ADC CH1 | ADC1_IN1 | Override detection channel 1 |
| 13  | PA2 | USART2 TX | USART2_TX (AF1) | CH340C RXD (debug UART) |
| 14  | PA3 | USART2 RX | USART2_RX (AF1) | CH340C TXD (debug UART) |
| 15  | PA4 | Override ADC CH2 | ADC1_IN4 | Override detection channel 2 |
| 16  | PA5 | Override ADC CH3 | ADC1_IN5 | Override detection channel 3 |
| 17  | PA6 | Override ADC CH4 | ADC1_IN6 | Override detection channel 4 |
| 18  | PA7 | Override ADC CH5 | ADC1_IN7 | Override detection channel 5 |
| 19  | PB0 | Override ADC CH6 | ADC1_IN8 | Override detection channel 6 |
| 20  | PB1 | Override ADC CH7 | ADC1_IN9 | Override detection channel 7 |
| 21  | PB2 | MOSFET gate CH0 | GPIO output | Output channel 0 drive |
| 22  | PB10 | MOSFET gate CH1 | GPIO output | Output channel 1 drive |
| 23  | PB11 | MOSFET gate CH2 | GPIO output | Output channel 2 drive |
| 24  | PB12 | MOSFET gate CH3 | GPIO output | Output channel 3 drive |
| 25  | PB13 | MOSFET gate CH4 | GPIO output | Output channel 4 drive |
| 29  | PA9 | USART1 TX | USART1_TX (AF1) | RS-485 data out (heartbeat TX) |
| 32  | PA10 | USART1 RX | USART1_RX (AF1) | RS-485 data in (SP3485EN RO) |
| 33  | PA11 | MOSFET gate CH5 | GPIO output | Output channel 5 drive |
| 34  | PA12 | RS-485 DE/RE | GPIO output | Transceiver RX enable (LOW) |
| 35  | PA13 | SWDIO | SWD | Debug programming |
| 36  | PA14 | SWCLK | SWD | Debug programming |
| 37  | PA15 | MOSFET gate CH6 | GPIO output | Output channel 6 drive |
| 42  | PB3 | MOSFET gate CH7 | GPIO output | Output channel 7 drive |
| 43  | PB4 | LED CH0 | GPIO output | Channel 0 output state LED |
| 44  | PB5 | LED CH1 | GPIO output | Channel 1 output state LED |
| 45  | PB6 | LED CH2 | GPIO output | Channel 2 output state LED |
| 46  | PB7 | LED CH3 | GPIO output | Channel 3 output state LED |
| 47  | PB8 | LED CH4 | GPIO output | Channel 4 output state LED |
| 48  | PB9 | LED CH5 | GPIO output | Channel 5 output state LED |
| 30  | PC6 | LED CH6 | GPIO output | Channel 6 output state LED |
| 31  | PC7 | LED CH7 | GPIO output | Channel 7 output state LED |
| 2   | PC14 | LED LINK | GPIO output | Serial link activity LED |
| 1   | PC13 | — | — | Reserved |
| 3   | PC15 | — | — | Reserved |
| — | NRST | Reset | System | Exposed on SWD header |
| — | VDD | 3.3V | Power | Decoupled with 100nF + 1μF |
| — | VSS | GND | Power | — |
| — | VDDA | 3.3V | ADC reference | Decoupled with 1μF + 100nF |

**Total active GPIO: 32** (8 ADC + 8 MOSFET + 9 LED + 1 DE/RE + 4 UART + 2 SWD) plus 3 reserved/unused (PC13, PC15, PA8). Well within the LQFP48's ~44 usable GPIO — this validates the choice of LQFP48 over LQFP32.

### Pin Assignment Rationale

- **PA0–PA1, PA4–PA7, PB0–PB1**: ADC channels for override detection (same ADC pin mapping as input board for consistency)
- **PA2–PA3**: USART2 for debug UART
- **PA9–PA10**: USART1 for RS-485
- **PA11, PA15, PB2–PB3, PB10–PB13**: MOSFET gate drives — spread across available GPIO, no special peripheral requirements
- **PB4–PB9, PC6–PC7, PC14**: LED drives — grouped where possible for efficient register writes
- **PA13–PA14**: Dedicated SWD pins

---

## 10. Screw Terminal Layout

```
                      ┌──────────────────────────────────────────────┐
                      │              OUTPUT BOARD PCB                │
    ┌─────────┐       │                                              │       ┌──────────┐
    │  12V IN │──J1───│                                              │───J4──│  OUT0    │
    │   GND   │       │                                              │       │  OUT1    │
    └─────────┘       │         [component area]                     │       │  OUT2    │
                      │                                              │       │  OUT3    │
    ┌─────────┐       │                                              │       │  OUT4    │
    │ RS485 A │──J2───│                                              │       │  OUT5    │
    │ RS485 B │       │                                              │       │  OUT6    │
    │   GND   │       │                                              │       │  OUT7    │
    │ SHIELD  │       │                                              │       │ 12V LOAD │
    └─────────┘       │                                              │       │ GND RTN  │
                      │                                              │       └──────────┘
    ┌─────────┐       │                                              │
    │  OVR0   │──J3───│                                              │       ┌──────────┐
    │  OVR1   │       │                                              │───J5──│  USB-C   │
    │  OVR2   │       │                                              │       └──────────┘
    │  OVR3   │       │  [PWR][LINK][0][1][2][3][4][5][6][7]        │
    │  OVR4   │       │              LED Edge                        │
    │  OVR5   │       └──────────────────────────────────────────────┘
    │  OVR6   │
    │  OVR7   │
    │   COM   │
    └─────────┘
```

| Connector | Position | Terminals | Purpose |
|-----------|----------|-----------|---------|
| J1 | Top-left | 12V, GND | Power input from AC-DC module |
| J2 | Left edge | A, B, GND, SHIELD | RS-485 input from cable |
| J3 | Left edge | OVR0–OVR7, COM | 8 override inputs + common ground |
| J4 | Right edge | OUT0–OUT7, 12V LOAD, GND RTN | 8 load outputs + load power + return |
| J5 | Right edge | USB-C | Debug/programming |

### Load Wiring

The **12V LOAD** and **GND RTN** terminals on J4 provide direct connection points for the load power loop. External loads connect between an OUTx terminal and GND RTN. The OUTx terminal is the MOSFET drain side (after the load); the 12V LOAD terminal connects directly to the 12V rail for load sourcing. This arrangement allows:

```
12V LOAD terminal ──► Load (+) ──► Load (−) ──► OUTx terminal (MOSFET drain)
                                                        │
                                                   MOSFET switches to GND
```

---

## 11. PCB Layout Notes

### Board Dimensions

Approximately 120mm × 80mm (larger than input board due to output stage and extra connectors). To be finalized in KiCad.

### Mounting

- 4x M3 mounting holes in rectangular pattern, inset 5mm from PCB edges
- Compatible with M3 × 6mm hex standoffs

### Layout Guidelines

- **Power**: Wide 12V traces (or copper pours) from J1 to MOSFET drains. Each output channel carries up to 2A — traces should be at minimum 1mm wide (preferably 2mm+ or use poured copper). Bulk capacitors placed close to J1.
- **Output MOSFETs**: Group MOSFET + gate resistor + pull-down + PTC fuse + flyback diode for each channel in a tight cluster. Place near the output screw terminals (J4) to minimize high-current trace length.
- **Override inputs**: Override conditioning components (series resistor, Zener, bias resistors) placed close to MCU ADC pins. Keep analog traces away from switching MOSFET gate traces.
- **RS-485**: SP3485EN placed close to J2. Termination resistor directly at SP3485EN A/B pins.
- **Ground plane**: Solid ground plane on inner/bottom layer. Star ground topology: analog ground (ADC, override) and digital ground (MCU, UART) share the plane but avoid routing high-current MOSFET return paths under sensitive analog areas.

### Test Points

| Label | Signal | Purpose |
|-------|--------|---------|
| TP1 | 3.3V | Verify LDO output |
| TP2 | 12V | Verify input power |
| TP3 | GND | Probe ground reference |
| TP4 | USART1_RX | RS-485 data in (post-transceiver) |
| TP5 | OVR_CH0 (ADC pin) | Override channel 0 voltage |
| TP6 | GATE_CH0 | MOSFET gate 0 drive signal |
| TP7 | DRAIN_CH0 | MOSFET drain 0 (load-side) |
| TP8 | RS-485 A | Differential bus signal (oscilloscope probe) |
| TP9 | DE/RE | Bus turnaround timing verification (PA12) |
| TP10 | GATE_CH7 | Last-channel gate drive (catches pin mapping bugs) |
| TP11 | 12V post-PTC CH0 | Verify fuse is not tripped |

### Debug Enhancements

**BOOT0 Access**: A solder-bridge jumper pad connects to the PA14-BOOT0 pin. Normally held LOW by internal pull-down. When jumpered HIGH before reset, the STM32 enters its built-in UART bootloader, allowing recovery programming via the CH340C USB-UART even if firmware is bricked or SWD is locked out. Cost: $0 (PCB pad/trace only). On the STM32G070, BOOT0 shares PA14 with SWCLK; the option byte nBOOT_SEL must be cleared to enable hardware BOOT0 pin sensing.

**NRST Tactile Button**: A small SMD tactile switch provides hardware reset without requiring a debugger. The SWD header has NRST but it requires manual shorting. Cost: ~$0.05/board.

### Silkscreen

- Board name, revision, date
- All connector labels (J1–J5) with pin names
- All LED labels (PWR, LINK, CH0–CH7)
- Test point labels
- Per-channel labeling near each MOSFET cluster (CH0–CH7)
- Override input labels matching output channel numbers
- Orientation marks for all ICs, diodes, and polarized capacitors
- "12V MAX 2A/CH" warning near output terminals
