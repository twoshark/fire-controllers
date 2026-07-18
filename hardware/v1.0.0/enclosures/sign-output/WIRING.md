# sign-output — wiring

```text
DTP IN → J1     [seat F9]     (no local POWER rocker)
M12-5 → J2
SOL0..SOL4 (DT) → J6 + J5a/b
Kill 12 V: unplug HLG-240H-12 AC cord
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

External: DTP from **HLG-240H-12** DC · **≤ 4 ft** · [`../POWER_OTS.md`](../POWER_OTS.md).
