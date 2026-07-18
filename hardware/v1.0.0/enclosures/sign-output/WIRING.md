# sign-output — wiring

```text
DTP IN → J1     [seat F9]     (no local POWER rocker)
M12-5 → J2
SOL0..SOL4 (DT) → J6 + J5a/b
Kill 12 V: AC POWER on sign-output-power
```

| Path | Net | Gauge |
| --- | --- | --- |
| DTP pin1 → J1.1 | +12V | 12 AWG |
| DTP pin2 → J1.2 | GND | 12 AWG |
| J6 both poles → star bus → DT pin1 each | +12V | 12 AWG stub / 18 AWG/ch |
| DT pin2 → OUTn | low-side | 18 AWG |

| SOL | OUT |
| --- | --- |
| 0..3 | J5a.1..4 |
| 4 | J5b.1 |

External: DTP jumper **≤ 4 ft** from sign-output-power.
