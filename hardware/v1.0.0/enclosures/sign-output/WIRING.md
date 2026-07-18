# sign-output — wiring

```text
PanelPole → POWER (KCD4) → J1     [seat F9]
M12-5 → J2
SOL0..SOL4 (DT 2-pin each) → J6 + J5a/b
DFU / RESET / BOOT: open lid → J7 / SW
```

| Path | Net | Gauge |
| --- | --- | --- |
| PanelPole red → KCD4 → J1.1 | +12V | 12 AWG |
| PanelPole black → J1.2 | GND | 12 AWG |

### DT SOL (each)

| Pin | Net |
| ---: | --- |
| 1 | J6 +12V |
| 2 | OUTn |

| SOL | OUT |
| --- | --- |
| 0..3 | J5a.1..4 |
| 4 | J5b.1 |

External 12 V from sign-output-power: Powerpole jumper **≤ 4 ft**, 12 AWG.
