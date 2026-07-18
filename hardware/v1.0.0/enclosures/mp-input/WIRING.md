# mp-input — wiring

```text
C14 → arcade POWER → RS-15-12 AC L/N · PE → FG
RS-15-12 +V/−V → PCB J1
M12-5 RS-485 → CN2
HangTon → J5
CH1..CH3 + ALL → J2 / buttons PCB D1..D3
RESET → NRST · BOOT → BOOT0
```

### Front → MCU

| Front | PCB | Notes |
| --- | --- | --- |
| CH1..CH3 | `J2a.1`..`J2a.3` | MCU CH0..CH2 |
| ALL | D1..D3 only | leave D4..D8 open |
| Commons | `J3` GND | |

### M12-5 → `CN2`

| M12 | `CN2` | Net |
| ---: | ---: | --- |
| 1 | 1 | TX+ |
| 2 | 2 | TX− |
| 3 | 3 | RX+ |
| 4 | 4 | RX− |
| 5 | 5 | GND |
| Shell | 6 | SHIELD |

| Path | Gauge |
| --- | --- |
| AC | 18 AWG |
| RS-15 → J1 | 18 AWG |
| Buttons / RS-485 | 22–28 AWG |
