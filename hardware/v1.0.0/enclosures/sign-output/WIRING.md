# sign-output — wiring

```text
PanelPole2 12V IN → J1 (+12V / GND)     [seat F9 first]
M12-5 RS-485 → J2
SOL0..SOL4 (each AT 2-pin) → J6 + J5a/J5b
HangTon → J7
RESET → NRST · BOOT → BOOT0
```

### PanelPole2 12V IN → `J1`

| Pole | Net | Gauge |
| --- | --- | --- |
| Red | `J1.1` +12V | 12 AWG |
| Black | `J1.2` GND | 12 AWG |

Crimp PP30 contacts (PanelPole2 includes 30 A / 12–14 AWG).

### M12-5 RS-485 → `J2`

| M12 | `J2` | Net |
| ---: | ---: | --- |
| 1 | 1 | TX+ |
| 2 | 2 | TX− |
| 3 | 3 | RX+ |
| 4 | 4 | RX− |
| 5 | 5 | GND |
| Shell | 6 | SHIELD |

≤22 AWG. Signal only.

### SOL0..SOL4 — one AT 2-pin each

Panel: **AT04-2P-KIT01**. Cable: **AT06-2S-KIT01**.

| Pin | Net |
| ---: | --- |
| 1 | `J6` +12V (use **both** `J6` poles paralleled) |
| 2 | `OUTn` |

| Connector | `OUTn` |
| --- | --- |
| SOL0 | `J5a.1` |
| SOL1 | `J5a.2` |
| SOL2 | `J5a.3` |
| SOL3 | `J5a.4` |
| SOL4 | `J5b.1` |

18 AWG · crimp size-16. Load: pin1 → solenoid → pin2.

| Path | Gauge | Joint |
| --- | --- | --- |
| PanelPole → J1 | 12 AWG | Crimp PP → screw `J1` |
| SOL (each) | 18 AWG | Crimp AT → screw `J6`/`J5` |
| RS-485 | 22–24 AWG | M12 rear → screw `J2` |
| USB | jumper | Plug |
