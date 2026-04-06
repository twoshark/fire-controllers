# Schematic Entry Guide

Step-by-step instructions for entering the fire controller schematics into EasyEDA (or KiCad 8). All pin assignments, component values, and net connections are defined here and in the unified BOMs.

**Critical:** All physical LQFP48 pin numbers below have been verified against the **STM32G070CBT6 datasheet (DS12766 Rev 3, Table 11/12/13/14)**.

---

## STM32G070CBT6 LQFP48 Pin Map (Used Pins)

| Pin | Function |
| --- | -------- |
| 4   | VBAT     |
| 5   | VREF+    |
| 6   | VDD/VDDA |
| 7   | VSS/VSSA |
| 10  | NRST     |
| 11  | PA0      |
| 12  | PA1      |
| 13  | PA2      |
| 14  | PA3      |
| 15  | PA4      |
| 16  | PA5      |
| 17  | PA6      |
| 18  | PA7      |
| 19  | PB0      |
| 20  | PB1      |
| 21  | PB2      |
| 22  | PB10     |
| 23  | PB11     |
| 24  | PB12     |
| 25  | PB13     |
| 26  | PB14     |
| 27  | PB15     |
| 29  | PA9      |
| 30  | PC6      |
| 31  | PC7      |
| 32  | PA10     |
| 33  | PA11     |
| 34  | PA12     |
| 35  | PA13     |
| 36  | PA14/BOOT0 |
| 37  | PA15     |
| 42  | PB3      |
| 43  | PB4      |
| 44  | PB5      |
| 45  | PB6      |
| 46  | PB7      |
| 47  | PB8      |
| 48  | PB9      |


---

## Input Board Schematic

### Sheet 1: Power


| From              | To                                | Via | Notes                                   |
| ----------------- | --------------------------------- | --- | --------------------------------------- |
| J1 pin 1 (12V)    | D1 anode (SS34 SMA)               | —   | Reverse polarity protection             |
| D1 cathode        | F1 input (3A PTC)                 | —   | Input fuse                              |
| F1 output         | +12V rail net                     | —   | Main 12V bus                            |
| +12V rail         | C17+ (100µF/25V SMD electrolytic) | —   | Bulk capacitor (observe polarity)       |
| C17-              | GND                               | —   |                                         |
| +12V rail         | C6 (100nF 0603)                   | —   | HF decoupling                           |
| +12V rail         | U4 input (AMS1117-3.3 pin 3)      | —   | LDO input                               |
| U4 output (pin 2) | +3V3 rail                         | —   |                                         |
| U4 GND (pin 1)    | GND                               | —   | Ensure thermal pad → copper pour → GND  |
| +3V3 rail         | C18+ (22µF tantalum)              | —   | LDO output stability (observe polarity) |
| +3V3 rail         | C19 (100nF 0603)                  | —   | LDO output HF decoupling                |
| J1 pin 2          | GND                               | —   |                                         |


### Sheet 2: MCU (STM32G070CBT6)


| MCU Pin (LQFP48) | Port       | Net         | Function                                           |
| ---------------- | ---------- | ----------- | -------------------------------------------------- |
| 6                | VDD/VDDA   | +3V3        | C1 (10µF) + C2 (100nF) to GND nearby               |
| 5                | VREF+      | +3V3        | C15 (1µF) to GND                                   |
| 4                | VBAT       | +3V3        |                                                    |
| 7                | VSS/VSSA   | GND         |                                                    |
| 10               | NRST       | NRST net    | C3 (100nF) to GND + SWD header pin 10 + SW1 to GND |
| 11               | PA0        | ADC_CH0     | Input conditioning channel 0                       |
| 12               | PA1        | ADC_CH1     | Input conditioning channel 1                       |
| 15               | PA4        | ADC_CH2     | Input conditioning channel 2                       |
| 16               | PA5        | ADC_CH3     | Input conditioning channel 3                       |
| 17               | PA6        | ADC_CH4     | Input conditioning channel 4                       |
| 18               | PA7        | ADC_CH5     | Input conditioning channel 5                       |
| 19               | PB0        | ADC_CH6     | Input conditioning channel 6                       |
| 20               | PB1        | ADC_CH7     | Input conditioning channel 7                       |
| 29               | PA9        | USART1_TX   | SP3485EN DI (pin 4)                                |
| 32               | PA10       | USART1_RX   | SP3485EN RO (pin 1)                                |
| 34               | PA12       | RS485_DE_RE | SP3485EN DE (pin 3) + RE (pin 2)                   |
| 13               | PA2        | USART2_TX   | CH340C RXD (pin 3)                                 |
| 14               | PA3        | USART2_RX   | CH340C TXD (pin 2)                                 |
| 21               | PB2        | LED_CH0     | Channel 0 LED (via 330Ω to +3V3)                   |
| 37               | PA15       | LED_CH1     | Channel 1 LED                                      |
| 42               | PB3        | LED_CH2     | Channel 2 LED                                      |
| 43               | PB4        | LED_CH3     | Channel 3 LED                                      |
| 44               | PB5        | LED_CH4     | Channel 4 LED                                      |
| 45               | PB6        | LED_CH5     | Channel 5 LED                                      |
| 46               | PB7        | LED_CH6     | Channel 6 LED                                      |
| 47               | PB8        | LED_CH7     | Channel 7 LED                                      |
| 48               | PB9        | LED_LINK    | Link LED (via 150Ω to +3V3 for blue)               |
| 35               | PA13       | SWDIO       | SWD header pin 2                                   |
| 36               | PA14/BOOT0 | SWCLK       | SWD header pin 4; BOOT0 jumper pad to +3V3         |


### Sheet 3: Input Conditioning (x8)

Per channel (CH0–CH7), repeat this sub-circuit connected to J2a/J2b:

```
J2_CHx ──► R_top (27kΩ) ──► divider_node ──► R_bot (10kΩ) ──► GND
                                   ├──► D_clamp (BZT52C3V3 cathode, anode to GND)
                                   ├──► C_filt (10nF) ──► GND
                                   └──► MCU ADC_CHx pin
```

Channel-to-connector mapping:

- J2a pins 1–4 → CH0–CH3
- J2b pins 1–4 → CH4–CH7
- J3 pin 1 → COM/GND (common ground for input sources)
- J3 pin 2 → GND

### Sheet 4: RS-485


| From                 | To                           | Notes                                                   |
| -------------------- | ---------------------------- | ------------------------------------------------------- |
| SP3485EN VCC (pin 8) | +3V3 + 100nF decoupling      |                                                         |
| SP3485EN GND (pin 5) | GND                          |                                                         |
| SP3485EN DI (pin 4)  | MCU PA9 (pin 29, USART1_TX)  |                                                         |
| SP3485EN RO (pin 1)  | MCU PA10 (pin 32, USART1_RX) |                                                         |
| SP3485EN DE (pin 3)  | RS485_DE_RE net              | Directly tied to RE                                     |
| SP3485EN RE (pin 2)  | RS485_DE_RE net              | MCU PA12 (pin 34) drives both                           |
| SP3485EN A (pin 6)   | J4 pin A + SM712             |                                                         |
| SP3485EN B (pin 7)   | J4 pin B + SM712             |                                                         |
| SM712 common         | Between A/B and GND          | Asymmetric TVS                                          |
| R27 (120Ω, DNP)      | Between A and B              | Termination resistor — **not populated** on input board |


### Sheet 5: USB Debug (CH340C)


| From               | To                         | Notes                          |
| ------------------ | -------------------------- | ------------------------------ |
| USB-C VBUS (J5)    | CH340C VCC (pin 16)        | 5V from USB host               |
| USB-C D+ (J5)      | CH340C UD+ (pin 5)         |                                |
| USB-C D- (J5)      | CH340C UD- (pin 6)         |                                |
| USB-C GND (J5)     | GND + CH340C GND (pin 1)   |                                |
| USB-C CC1 (J5)     | R29 (5.1kΩ) → GND          | Device role detection          |
| USB-C CC2 (J5)     | R30 (5.1kΩ) → GND          |                                |
| CH340C V3 (pin 4)  | C16 (100nF) → GND          | Internal 3.3V regulator output |
| CH340C TXD (pin 2) | MCU PA3 (pin 14, USART2_RX) | Cross-connected                |
| CH340C RXD (pin 3) | MCU PA2 (pin 13, USART2_TX) | Cross-connected                |


### Sheet 6: LEDs

Power LED: +3V3 → LED1 (green) → 330Ω → GND (no GPIO, always on).

Channel/Link LEDs: +3V3 → LED → resistor → MCU GPIO (active LOW).

- 330Ω for green/yellow LEDs (R17–R26)
- 150Ω for blue LED (R28, LINK)

### Sheet 7: SWD Debug Header (J6)


| J6 Pin | Signal | Connection            |
| ------ | ------ | --------------------- |
| 1      | VTref  | +3V3                  |
| 2      | SWDIO  | MCU PA13 (pin 35)     |
| 3      | GND    | GND                   |
| 4      | SWCLK  | MCU PA14 (pin 36)     |
| 5      | GND    | GND                   |
| 6      | NC     | —                     |
| 7      | NC     | —                     |
| 8      | NC     | —                     |
| 9      | GND    | GND                   |
| 10     | NRST   | MCU NRST (pin 10)     |


---

## Output Board Schematic

### Sheet 1: Power

Same structure as input board but with larger bulk capacitors and no input blade fuse (use an inline 20A fuse in enclosure wiring between PSU and board):


| Addition vs Input Board          | Notes                                             |
| -------------------------------- | ------------------------------------------------- |
| C17 = 470µF/25V SMD electrolytic | Higher bulk for output switching                  |
| C18 = 100µF/25V SMD electrolytic | Additional bulk                                   |
| No on-board input fuse           | User installs 20A inline fuse in enclosure wiring |


### Sheet 2: MCU (STM32G070CBT6)


| MCU Pin (LQFP48) | Port       | Net         | Function                        |
| ---------------- | ---------- | ----------- | ------------------------------- |
| 6                | VDD/VDDA   | +3V3        | C1 (10µF) + C2 (100nF) to GND   |
| 5                | VREF+      | +3V3        | C15 (1µF) to GND                |
| 4                | VBAT       | +3V3        |                                 |
| 7                | VSS/VSSA   | GND         |                                 |
| 10               | NRST       | NRST net    | 100nF to GND + SWD header + SW1 |
| 11               | PA0        | OVR_CH0     | Override ADC channel 0          |
| 12               | PA1        | OVR_CH1     | Override ADC channel 1          |
| 15               | PA4        | OVR_CH2     | Override ADC channel 2          |
| 16               | PA5        | OVR_CH3     | Override ADC channel 3          |
| 17               | PA6        | OVR_CH4     | Override ADC channel 4          |
| 18               | PA7        | OVR_CH5     | Override ADC channel 5          |
| 19               | PB0        | OVR_CH6     | Override ADC channel 6          |
| 20               | PB1        | OVR_CH7     | Override ADC channel 7          |
| 21               | PB2        | GATE_CH0    | MOSFET gate CH0                 |
| 22               | PB10       | GATE_CH1    | MOSFET gate CH1                 |
| 23               | PB11       | GATE_CH2    | MOSFET gate CH2                 |
| 24               | PB12       | GATE_CH3    | MOSFET gate CH3                 |
| 25               | PB13       | GATE_CH4    | MOSFET gate CH4                 |
| 33               | PA11       | GATE_CH5    | MOSFET gate CH5                 |
| 37               | PA15       | GATE_CH6    | MOSFET gate CH6                 |
| 42               | PB3        | GATE_CH7    | MOSFET gate CH7                 |
| 29               | PA9        | USART1_TX   | SP3485EN DI (heartbeat TX)      |
| 32               | PA10       | USART1_RX   | SP3485EN RO (state frame RX)    |
| 34               | PA12       | RS485_DE_RE | SP3485EN DE+RE (LOW=RX default) |
| 13               | PA2        | USART2_TX   | CH340C RXD                      |
| 14               | PA3        | USART2_RX   | CH340C TXD                      |
| 43               | PB4        | LED_CH0     | Output state LED CH0            |
| 44               | PB5        | LED_CH1     | Output state LED CH1            |
| 45               | PB6        | LED_CH2     | Output state LED CH2            |
| 46               | PB7        | LED_CH3     | Output state LED CH3            |
| 47               | PB8        | LED_CH4     | Output state LED CH4            |
| 48               | PB9        | LED_CH5     | Output state LED CH5            |
| 30               | PC6        | LED_CH6     | Output state LED CH6            |
| 31               | PC7        | LED_CH7     | Output state LED CH7            |
| 2                | PC14       | LED_LINK    | Link LED                        |
| 35               | PA13       | SWDIO       | SWD header                      |
| 36               | PA14/BOOT0 | SWCLK       | SWD header + BOOT0 jumper       |


### Sheet 3: Output Stage (x8)

Per channel, repeat this sub-circuit:

```
+12V rail ──► PTC fuse (1812L200/12GR) ──► OUT_CHx net ──► J5a/J5b OUTx terminal
                                                │
                                          MOSFET drain (IRLML6344)
                                          MOSFET gate ◄── R_gate (100Ω) ◄── MCU GATE_CHx
                                          MOSFET gate ──► R_pull (10kΩ) ──► GND
                                          MOSFET source ──► GND
                                                │
                                          SS34 flyback: anode=drain, cathode=+12V rail
```

Output connector mapping:

- J5a pins 1–4 → OUT0–OUT3
- J5b pins 1–4 → OUT4–OUT7
- J6 pin 1 → 12V LOAD (direct to +12V rail, for powering external loads)
- J6 pin 2 → GND RTN (ground return for external loads)

### Sheet 4: Override Detection (x8)

Per channel, repeat:

```
J3a/J3b OVRx ──► R_series (10kΩ) ──► adc_node
                                          ├──► BZT52C3V3 (cathode→adc_node, anode→GND)
                                          ├──► C_filt (10nF) ──► GND
                                          └──► ADC pin ◄──┬── R_high (100kΩ) ──► +3V3
                                                          └── R_low (100kΩ) ──► GND
```

Override connector mapping:

- J3a pins 1–4 → OVR0–OVR3
- J3b pins 1–4 → OVR4–OVR7
- J4 pin 1 → Override COM/GND
- J4 pin 2 → GND

### Sheet 5: RS-485

Same as input board, plus at the output board:

- R52 (120Ω) termination resistor between A and B — **populated** on output board
- R53 (390Ω) bias: A to +3V3
- R54 (390Ω) bias: B to GND

### Sheet 6: USB Debug + LEDs + SWD

Identical to input board (sheets 5, 6, 7). Use J7 for USB-C and J8 for SWD header.

---

## PCB Layout Checklist

- AMS1117 SOT-223: copper pour under thermal pad to GND plane
- Output board: wide traces (≥2mm) from +12V rail to PTC fuses to MOSFET drains
- RS-485 differential pair: route A/B close together, short to SP3485EN
- USB D+/D-: controlled impedance (90Ω differential)
- ADC traces: short, away from switching noise
- Decoupling caps: as close to IC VDD pins as possible
- All LEDs in one row along the LED edge
- 4× M3 mounting holes, 5mm inset from corners
- Test points: accessible from board edge
- BOOT0 solder jumper pad near PA14 (pin 36)
- NRST tactile switch accessible (connects NRST pin 10 to GND)
- Ground plane on bottom layer (or inner layer for 4-layer)
- Silkscreen: board name, rev, date, all connector labels, LED labels, test point labels
- SMD electrolytic caps (C17, C18): check polarity markings on footprint
- Tantalum cap: check polarity band on footprint

