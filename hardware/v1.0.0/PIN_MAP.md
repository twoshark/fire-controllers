# MCU Pin Map (Netlist-Verified)

Sources: `exports/Netlist_input.asc`, `exports/Netlist_output.asc` (2026-07-15).
GPIO names use STM32G0B1CBT6 LQFP48 package pins.

Polarity: switch/override closed to GND → Schmitt HIGH → active.
LEDs active-low. MOSFET gate HIGH = ON.

## Input board

| CH | Connector | Schmitt | LQFP48 | GPIO | Wake |
| --- | --- | --- | --- | --- | --- |
| 0 | `J2a.1` | `U5.2` | 11 | `PA0` | EXTI0 |
| 1 | `J2a.2` | `U5.4` | 12 | `PA1` | EXTI1 |
| 2 | `J2a.3` | `U5.6` | 15 | `PA4` | EXTI4 |
| 3 | `J2a.4` | `U5.8` | 16 | `PA5` | EXTI5 |
| 4 | `J2b.1` | `U6.2` | 17 | `PA6` | EXTI6 |
| 5 | `J2b.2` | `U6.4` | 18 | `PA7` | EXTI7 |
| 6 | `J2b.3` | `U6.6` | 19 | `PB0` | polled 1 ms (EXTI0 clash) |
| 7 | `J2b.4` | `U6.8` | 20 | `PB1` | polled 1 ms (EXTI1 clash) |

`PA2`/`PA3` unused. RC caps: CH0=`C20`, CH1=`C23`, CH2=`C24`, CH3=`C25`, CH4=`C26`, CH5=`C27`, CH6=`C4`, CH7=`C29`.

| Function | Ref | GPIO |
| --- | --- | --- |
| CH0..CH7 LEDs | `LED3`..`LED10` | `PB10`, `PA15`, `PB3`..`PB8` |
| LINK | `LED2` | `PB9` |
| POWER | `LED1` | always-on via `R25` |
| USART1 TX/RX | — | `PA9` → `U2A.DI` / `PA10` ← `U2B.RO` |
| USB DM/DP | — | `PA11`/`PA12` |
| SWD | `J6` | `PA13`/`PA14` + `NRST` |

## Output board

| CH | Override | OVR GPIO | Gate | MOSFET | Load | LED | LED GPIO |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | `J3a.1` | `PA0` | `PB0` | `Q1` | `J5a.1` | `CH0` | `PB3` |
| 1 | `J3a.2` | `PA1` | `PB1` | `Q2` | `J5a.2` | `CH1` | `PB4` |
| 2 | `J3a.3` | `PA2` | `PB2` | `Q3` | `J5a.3` | `CH2` | `PB5` |
| 3 | `J3a.4` | `PA3` | `PB10` | `Q4` | `J5a.4` | `CH3` | `PB6` |
| 4 | `J3b.1` | `PA4` | `PB11` | `Q5` | `J5b.1` | `CH4` | `PB7` |
| 5 | `J3b.2` | `PA5` | `PB12` | `Q6` | `J5b.2` | `CH5` | `PB8` |
| 6 | `J3b.3` | `PA6` | `PB13` | `Q7` | `J5b.3` | `CH6` | `PB9` |
| 7 | `J3b.4` | `PA7` | `PB14` | `Q8` | `J5b.4` | `CH7` | `PC6` |

LINK=`PC7`. POWER=`PWR` always-on via `R41`. Gates: direct MCU + `R9`..`R16` 10k PD (no series Rg).

USART1: `PA9`→`U2A.DI`, `PA10`←`U2B.RO` (same TX/RX roles as input).

## RS-485 crossover cable

| Input `CN2` | Output `J2` |
| --- | --- |
| TX+/- (1/2) | RX+/- (3/4) |
| RX+/- (3/4) | TX+/- (1/2) |
| GND (5) | GND (5) |
| SHIELD (6) | SHIELD (6) |

On both PCBs, connector pin 6 (SHIELD) is bonded to GND in the netlist.

Load wiring: `J6` (+12V) → load → `J5x.k`. Never return load to board GND.
