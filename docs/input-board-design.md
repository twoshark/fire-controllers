# Input Board Design

## 1. Overview

The input board reads 8 digital input channels (3.3V–12V range) and continuously transmits their state over RS-485 at 1kHz using the **Hotline v2** protocol. It is the "sender" side of the fire controller system.


| Parameter        | Value                                                |
| ---------------- | ---------------------------------------------------- |
| MCU              | STM32G070CBT6 (Cortex-M0+, 64MHz, 128KB flash, 36KB SRAM, LQFP48) |
| Input channels   | 8 (3.3V–12V, independently per channel)              |
| Output           | RS-485 serial (3-byte frame at 1kHz)                 |
| Power input      | 12V DC via screw terminal (from Mean Well IRM-15-12) |
| Logic rail       | 3.3V via AMS1117-3.3 LDO                             |
| Status LEDs      | 10 (8 channel + 1 power + 1 link)                    |
| Debug            | USB-C (CH340C UART bridge) + SWD header              |
| Total power draw | ~1W                                                  |


---

## 2. Power Section

### Power Supply Chain

```
120V AC (IEC C14 inlet on enclosure)
    │
    ▼
Mean Well IRM-15-12 (off-board, inside enclosure)
    120V AC → 12V DC, 15W (1.25A), PCB-mount enclosed module
    │
    ▼ (wire to PCB screw terminal)
12V DC Screw Terminal (J_PWR)
    │
    ├──► D_RPP (SS34 Schottky, SMA) — reverse polarity protection
    │        │  (drops ~0.5V, giving ~11.5V rail — above LDO dropout)
    │        ▼
    ├──► F_INPUT (3A PTC fuse, 1812 SMD) — input rail overcurrent protection
    │        │
    │        ▼
    ├──► 100μF/25V electrolytic (C_BULK) — bulk energy storage
    ├──► 100nF/50V ceramic 0603 (C_IN) — high-frequency decoupling
    │
    ├──► 12V Rail — input conditioning reference
    │
    └──► AMS1117-3.3 (U_LDO, SOT-223)
            │
            ├──► 22μF/10V tantalum (C_OUT1) — LDO output stability
            ├──► 100nF ceramic 0603 (C_OUT2) — high-frequency decoupling
            │
            └──► 3.3V Rail — MCU, RS-485, LEDs
```

### Power Budget


| Consumer                     | Rail   | Current (typ) | Power                       |
| ---------------------------- | ------ | ------------- | --------------------------- |
| STM32G070CBT6                | 3.3V   | 15mA          | 50mW                        |
| SP3485EN (TX mode)           | 3.3V   | 0.5mA         | 2mW                         |
| CH340C (USB connected)       | 5V USB | 10mA          | — (from USB host)           |
| 10x LEDs @ 5mA               | 3.3V   | 50mA          | 165mW                       |
| Input dividers (8x @ ~0.1mA) | 12V    | 0.8mA         | 10mW                        |
| **3.3V rail total**          | 3.3V   | **~66mA**     | 217mW                       |
| **LDO dissipation**          | —      | —             | (12−3.3) × 66mA = **574mW** |
| **Total from 12V**           | 12V    | **~67mA**     | **~801mW**                  |


The IRM-15-12 (15W, 1.25A) has massive headroom for the input board's ~1W draw.

The CH340C is powered from USB VBUS (5V) through its VCC pin, and generates 3.3V internally on its V3 pin for I/O level shifting. It does not draw from the board's 3.3V LDO rail.

### LDO Thermal

The AMS1117-3.3 in SOT-223 dissipates ~574mW at worst case. **Mandatory layout requirement**: a copper pour under the SOT-223 thermal pad is required (reduces θ_JA from ~90°C/W to ~46°C/W). At 574mW with copper pour, junction temperature rise is ~26°C above ambient (safe). Without copper pour, the rise would be ~52°C (at 90°C/W), still manageable but with less margin at elevated ambient. No external heatsink needed with adequate copper pour.

### Decoupling

- **MCU**: 100nF ceramic (0603) on each VDD pin + 1μF ceramic (0603) shared across VDD pins, placed as close to pins as possible
- **SP3485EN**: 100nF ceramic (0603) on VCC pin
- **CH340C**: 100nF ceramic (0603) on VCC pin
- **LDO output**: 22μF tantalum + 100nF ceramic
- **12V input**: 100μF electrolytic + 100nF ceramic

---

## 3. Input Conditioning

### Circuit (Per Channel)

Each of the 8 input channels uses a resistor divider to scale the 3.3V–12V input range into the MCU's 0–3.3V ADC range, with Zener clamp protection:

```
External Input (3.3V–12V)
    │
    ├──► R_TOP (27kΩ, 0603)
    │        │
    │        ├──► Divider Node
    │        │        │
    │        │        ├──► R_BOT (10kΩ, 0603) ──► GND
    │        │        │
    │        │        ├──► D_CLAMP (BZT52C3V3-7-F, SOD-123) ──► GND
    │        │        │      3.3V Zener clamp
    │        │        │
    │        │        ├──► C_FILT (10nF, 0603 ceramic) ──► GND
    │        │        │      ADC anti-aliasing filter
    │        │        │
    │        │        └──► MCU ADC Pin (PA0–PA7)
    │        │
    │        └──► (also filtered by parasitic + PCB capacitance)
    │
Screw Terminal (J_INPUT, per channel)
```

### Divider Math

With R_top = 27kΩ, R_bot = 10kΩ:


| Input Voltage   | V_adc = V_in × R_bot / (R_top + R_bot) | Status                            |
| --------------- | -------------------------------------- | --------------------------------- |
| 0V              | 0V                                     | LOW (inactive)                    |
| 3.3V            | 3.3 × 10 / 37 = **0.89V**              | HIGH (active, minimum detectable) |
| 5V              | 5.0 × 10 / 37 = **1.35V**              | HIGH (active)                     |
| 12V             | 12.0 × 10 / 37 = **3.24V**             | HIGH (active, maximum)            |
| 15V (transient) | Clamped to ~3.3V by Zener              | Protected                         |


### Firmware Threshold

- **Threshold**: ~0.45V (ADC count ~558 at 12-bit, 3.3V reference)
- Halfway between 0V (inactive) and 0.89V (minimum active input at 3.3V)
- Provides ~0.45V noise margin on both sides

### Input Impedance

Z_in = R_top + R_bot = 37kΩ. Low enough to reject noise pickup, high enough to avoid loading external signal sources.

### Protection

The BZT52C3V3-7-F Zener diode (SOD-123) clamps the divider node to ~3.3V (Zener breakdown voltage, 3.1–3.5V tolerance range). This protects the STM32's ADC input (absolute max VDD + 0.3V = 3.6V). Under normal operating conditions (≤12V input), the divider output stays at ≤3.24V and the Zener does not conduct.

---

## 4. RS-485 Interface

### Circuit

```
STM32 USART1_TX (PA9) ──────► SP3485EN DI (pin 4)
STM32 GPIO (PA12) ────────────► SP3485EN DE (pin 3) + RE̅ (pin 2)
                                    (tied together, active HIGH = TX enable)
SP3485EN RO (pin 1) ──────► STM32 USART1_RX (PA10)
                                    (heartbeat reception from output board)

SP3485EN VCC (pin 8) ──► 3.3V + 100nF decoupling
SP3485EN GND (pin 5) ──► GND

SP3485EN A (pin 6) ──────► RS-485 screw terminal A (+)
SP3485EN B (pin 7) ──────► RS-485 screw terminal B (−)

SM712 TVS diode (SOT-23) across A and B lines for transient voltage suppression
    Asymmetric clamping: −7V to +12V (matches RS-485 common-mode range)
```

### Operating Mode

The DE/RE pin defaults to HIGH (TX mode) at boot. After each state frame transmission, firmware briefly toggles DE/RE LOW to switch to RX mode, listening for heartbeat responses from the output board. After the RX window (~500μs), firmware toggles DE/RE back to HIGH for the next state frame. The input board is the bus master and controls all turnaround timing. USART1_RX receives heartbeat frames from the output board.

### Bus Biasing

No bus bias resistors on the input board (transmitter). The 120Ω termination resistor is at the receiver end (output board) only.

---

## 5. USB Debug Interface

### Circuit

```
USB-C Connector
    │
    ├── VBUS (5V) ───────► CH340C VCC (pin 16)
    ├── D+ ──────────────► CH340C UD+ (pin 5)
    ├── D− ──────────────► CH340C UD− (pin 6)
    ├── GND ─────────────► CH340C GND (pin 1)
    │
    └── CC1, CC2 ────────► 5.1kΩ pull-down to GND each
                           (USB-C source detection, signals device role)

CH340C V3 (pin 4) ──► 100nF to GND (internal 3.3V regulator decoupling)
CH340C TXD (pin 2) ──► STM32 USART2_RX (PA3)
CH340C RXD (pin 3) ──► STM32 USART2_TX (PA2)
```

The CH340C is powered entirely from USB VBUS (5V). Its internal regulator generates 3.3V on the V3 pin, which sets the I/O voltage level — no external 3.3V connection needed. When USB is disconnected, the CH340C is unpowered and its UART lines go high-impedance.

---

## 6. Status LEDs

### LED Circuit (Per LED)

```
3.3V ──► LED (0603) ──► R_LED (330Ω, 0603) ──► MCU GPIO Pin
                                                  (active LOW: GPIO LOW = LED ON)
```

Current per LED: I = (3.3V − V_f) / 330Ω ≈ (3.3 − 2.0) / 330 ≈ **3.9mA** (green/yellow)

For blue LEDs (V_f ≈ 3.0V): I ≈ (3.3 − 3.0) / 330 ≈ **0.9mA** — dimmer, but adequate for status indication. Use 150Ω for blue if brighter output needed. Let's use the 150 ohm / brighter blue option.

### LED Assignments

All LEDs are surface-mounted in a single row along one PCB edge (the "LED edge"):


| Position | Label | Color  | Function                  | Drive Logic                                    |
| -------- | ----- | ------ | ------------------------- | ---------------------------------------------- |
| 1        | PWR   | Green  | Power indicator           | Always ON (tied to 3.3V via resistor, no GPIO) |
| 2        | LINK  | Blue   | End-to-end link health    | ON solid when heartbeat received from output board; blinks during TX-only (no heartbeat) |
| 3        | CH0   | Yellow | Channel 0 input state     | ON when input active                           |
| 4        | CH1   | Yellow | Channel 1 input state     | ON when input active                           |
| 5        | CH2   | Yellow | Channel 2 input state     | ON when input active                           |
| 6        | CH3   | Yellow | Channel 3 input state     | ON when input active                           |
| 7        | CH4   | Yellow | Channel 4 input state     | ON when input active                           |
| 8        | CH5   | Yellow | Channel 5 input state     | ON when input active                           |
| 9        | CH6   | Yellow | Channel 6 input state     | ON when input active                           |
| 10       | CH7   | Yellow | Channel 7 input state     | ON when input active                           |


The power LED is wired directly to the 3.3V rail (no GPIO needed), saving one GPIO pin. It turns on whenever the board is powered.

---

## 7. SWD Debug Header

Standard ARM 10-pin (2×5, 1.27mm pitch) Cortex Debug connector:


| Pin | Signal | Connection           |
| --- | ------ | -------------------- |
| 1   | VTref  | 3.3V                 |
| 2   | SWDIO  | STM32 PA13           |
| 3   | GND    | GND                  |
| 4   | SWCLK  | STM32 PA14           |
| 5   | GND    | GND                  |
| 6   | SWO    | NC (not used on M0+) |
| 7   | —      | NC                   |
| 8   | —      | NC                   |
| 9   | GND    | GND                  |
| 10  | NRST   | STM32 NRST           |


---

## 8. Complete GPIO Pin Assignment

### STM32G070CBT6 (LQFP48)


| Pin | Port.Bit | Function      | Peripheral      | Notes                         |
| --- | -------- | ------------- | --------------- | ----------------------------- |
| 11  | PA0      | ADC input CH0 | ADC1_IN0        | Input conditioning channel 0  |
| 12  | PA1      | ADC input CH1 | ADC1_IN1        | Input conditioning channel 1  |
| 13  | PA2      | USART2 TX     | USART2_TX (AF1) | CH340C RXD (debug UART)       |
| 14  | PA3      | USART2 RX     | USART2_RX (AF1) | CH340C TXD (debug UART)       |
| 15  | PA4      | ADC input CH2 | ADC1_IN4        | Input conditioning channel 2  |
| 16  | PA5      | ADC input CH3 | ADC1_IN5        | Input conditioning channel 3  |
| 17  | PA6      | ADC input CH4 | ADC1_IN6        | Input conditioning channel 4  |
| 18  | PA7      | ADC input CH5 | ADC1_IN7        | Input conditioning channel 5  |
| 19  | PB0      | ADC input CH6 | ADC1_IN8        | Input conditioning channel 6  |
| 20  | PB1      | ADC input CH7 | ADC1_IN9        | Input conditioning channel 7  |
| 21  | PB2      | LED CH0       | GPIO output     | Channel 0 state LED           |
| 29  | PA9      | USART1 TX     | USART1_TX (AF1) | RS-485 data out (SP3485EN DI) |
| 32  | PA10     | USART1 RX     | USART1_RX (AF1) | RS-485 data in (heartbeat RX) |
| 34  | PA12     | RS-485 DE/RE  | GPIO output     | Transceiver TX enable (HIGH)  |
| 35  | PA13     | SWDIO         | SWD             | Debug programming             |
| 36  | PA14     | SWCLK         | SWD             | Debug programming             |
| 37  | PA15     | LED CH1       | GPIO output     | Channel 1 state LED           |
| 42  | PB3      | LED CH2       | GPIO output     | Channel 2 state LED           |
| 43  | PB4      | LED CH3       | GPIO output     | Channel 3 state LED           |
| 44  | PB5      | LED CH4       | GPIO output     | Channel 4 state LED           |
| 45  | PB6      | LED CH5       | GPIO output     | Channel 5 state LED           |
| 46  | PB7      | LED CH6       | GPIO output     | Channel 6 state LED           |
| 47  | PB8      | LED CH7       | GPIO output     | Channel 7 state LED           |
| 48  | PB9      | LED LINK      | GPIO output     | Serial link activity LED      |
| 2   | PC14     | —             | —               | Reserved (OSC32 if needed)    |
| 3   | PC15     | —             | —               | Reserved (OSC32 if needed)    |
| 30  | PC6      | —             | —               | Unused                        |
| 31  | PC7      | —             | —               | Unused                        |
| —   | NRST     | Reset         | System          | Exposed on SWD header         |
| —   | VDD      | 3.3V          | Power           | Decoupled with 100nF + 1μF    |
| —   | VSS      | GND           | Power           | —                             |
| —   | VDDA     | 3.3V          | ADC reference   | Decoupled with 1μF + 100nF    |


**Total active GPIO: 24** (8 ADC + 9 LED + 1 DE/RE + 4 UART + 2 SWD). Well within the LQFP48's ~44 usable GPIO.

### Pin Assignment Rationale

- **PA0–PA1, PA4–PA7, PB0–PB1**: All on ADC1 channels, grouped together for efficient sequential ADC scanning
- **PA2–PA3**: USART2 alternate function for debug UART
- **PA9–PA10**: USART1 alternate function for RS-485
- **PA13–PA14**: Dedicated SWD pins (cannot be reassigned without losing debug)
- **PB2–PB9, PA15**: General-purpose GPIO for LEDs, grouped on one port where possible for efficient register writes
- **PA12**: Arbitrary GPIO for RS-485 direction control

---

## 9. Screw Terminal Layout

All external connections use 5.08mm pitch screw terminals for easy enclosure wiring:

```
                      ┌──────────────────────────────────────┐
                      │              INPUT BOARD PCB          │
    ┌─────────┐       │                                      │       ┌──────────┐
    │  12V IN │──J1───│                                      │───J3──│ RS-485 A │
    │   GND   │       │         [component area]             │       │ RS-485 B │
    └─────────┘       │                                      │       │   GND    │
                      │                                      │       │  SHIELD  │
    ┌─────────┐       │                                      │       └──────────┘
    │   CH0   │──J2───│                                      │
    │   CH1   │       │                                      │       ┌──────────┐
    │   CH2   │       │                                      │───J4──│  USB-C   │
    │   CH3   │       │  [PWR][LINK][0][1][2][3][4][5][6][7] │       └──────────┘
    │   CH4   │       │              LED Edge                │
    │   CH5   │       └──────────────────────────────────────┘
    │   CH6   │
    │   CH7   │
    │   COM   │
    └─────────┘
```


| Connector | Position   | Terminals         | Purpose                                   |
| --------- | ---------- | ----------------- | ----------------------------------------- |
| J1        | Top-left   | 12V, GND          | Power input from AC-DC module             |
| J2        | Left edge  | CH0–CH7, COM      | 8 signal inputs + common ground           |
| J3        | Right edge | A, B, GND, SHIELD | RS-485 output to cable                    |
| J4        | Right edge | USB-C             | Debug/programming (panel-mount or direct) |


---

## 10. PCB Layout Notes

### Mounting

- 4x M3 mounting holes in rectangular pattern, inset 5mm from PCB edges
- Compatible with M3 × 6mm hex standoffs
- Board dimensions: approximately 100mm × 65mm (to be finalized in KiCad)

### Layout Guidelines

- **Power**: LDO and bulk caps placed near 12V input terminal. Short, wide traces for power rails. Ground plane on inner/bottom layer.
- **ADC inputs**: Route ADC traces away from switching/digital noise. Keep traces short between divider output and MCU pin. Place divider resistors close to MCU.
- **RS-485**: SP3485EN placed close to RS-485 screw terminal. Differential pair routed as close together as possible. 100nF decoupling cap directly at SP3485EN VCC pin.
- **USB**: CH340C placed close to USB-C connector. D+/D− routed as differential pair with controlled impedance (90Ω differential).
- **LEDs**: All 10 LEDs in a single row along bottom PCB edge. Series resistors placed directly adjacent to LEDs.
- **Decoupling**: 100nF caps placed as close as physically possible to each IC's VDD pin, with short return path to ground plane.
- **SWD header**: Placed at PCB edge for easy probe access. Keep away from noisy circuits.

### Test Points


| Label | Signal    | Purpose                                       |
| ----- | --------- | --------------------------------------------- |
| TP1   | 3.3V      | Verify LDO output                             |
| TP2   | 12V       | Verify input power                            |
| TP3   | GND       | Probe ground reference                        |
| TP4   | USART1_TX | RS-485 data out (pre-transceiver)             |
| TP5   | ADC_CH0   | First ADC channel (post-divider)              |
| TP6   | USART1_RX | Heartbeat reception debugging (PA10)          |
| TP7   | RS-485 A  | Differential bus signal (oscilloscope probe)  |
| TP8   | DE/RE     | Bus turnaround timing verification (PA12)     |


### Debug Enhancements

**BOOT0 Access**: A solder-bridge jumper pad connects to the PA14-BOOT0 pin. Normally held LOW by internal pull-down. When jumpered HIGH before reset, the STM32 enters its built-in UART bootloader, allowing recovery programming via the CH340C USB-UART even if firmware is bricked or SWD is locked out. Cost: $0 (PCB pad/trace only). On the STM32G070, BOOT0 shares PA14 with SWCLK; the option byte nBOOT_SEL must be cleared to enable hardware BOOT0 pin sensing.

**NRST Tactile Button**: A small SMD tactile switch provides hardware reset without requiring a debugger. The SWD header has NRST but it requires manual shorting. Cost: ~$0.05/board.


### Silkscreen

- Board name, revision, date
- All connector labels (J1–J4) with pin names
- All LED labels (PWR, LINK, CH0–CH7)
- Test point labels
- Orientation marks for ICs
- Mounting hole labels

