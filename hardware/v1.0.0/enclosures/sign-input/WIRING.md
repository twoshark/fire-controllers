# sign-input — wiring

```text
C14 → arcade POWER → RS-15-12 AC L/N · PE → FG
RS-15-12 +V/−V → PCB J1.1 / J1.2
M12-5 RS-485 → CN2
HangTon → J5 (USB-C jumper)
CH + ALL → J2 / buttons PCB
RESET → NRST · BOOT → BOOT0
```

### Front → MCU

| Front | PCB | Notes |
| --- | --- | --- |
| CH1..CH5 NO → GND | `J2a.1`..`J2b.1` | MCU CH0..CH4 |
| ALL NO → buttons PCB | D1..D5 cathodes | assert CH0..CH4 only |
| Switch commons | `J3` GND | |

Leave `J2b.2`..`J2b.4` and D6..D8 open.

### M12-5 → `CN2`

| M12 | `CN2` | Net |
| ---: | ---: | --- |
| 1 | 1 | TX+ |
| 2 | 2 | TX− |
| 3 | 3 | RX+ |
| 4 | 4 | RX− |
| 5 | 5 | GND |
| Shell | 6 | SHIELD (PCB ties pin 6 to GND) |

| Path | Gauge |
| --- | --- |
| AC | 18 AWG |
| RS-15 → J1 | 18 AWG |
| Buttons / RS-485 | 22–28 AWG |
