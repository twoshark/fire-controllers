# mp-output — wiring

```text
PanelPole2 12V IN → J1     [seat F9 first]
M12-5 RS-485 → J2
SOL0..SOL2 (each AT 2-pin) → J6 + J5a
HangTon → J7
RESET → NRST · BOOT → BOOT0
```

### PanelPole2 12V IN → `J1`

| Pole | Net | Gauge |
| --- | --- | --- |
| Red | +12V | 12 AWG |
| Black | GND | 12 AWG |

### M12-5 RS-485 → `J2`

| M12 | `J2` | Net |
| ---: | ---: | --- |
| 1 | 1 | TX+ |
| 2 | 2 | TX− |
| 3 | 3 | RX+ |
| 4 | 4 | RX− |
| 5 | 5 | GND |
| Shell | 6 | SHIELD |

### SOL0..SOL2 — one AT 2-pin each

| Pin | Net |
| ---: | --- |
| 1 | `J6` +12V (both poles paralleled) |
| 2 | `OUTn` |

| Connector | `OUTn` |
| --- | --- |
| SOL0 | `J5a.1` |
| SOL1 | `J5a.2` |
| SOL2 | `J5a.3` |

18 AWG · AT04-2P panel / AT06-2S cable · crimp size-16.

| Path | Gauge |
| --- | --- |
| PanelPole → J1 | 12 AWG |
| SOL (each) | 18 AWG |
| RS-485 | 22–24 AWG |
