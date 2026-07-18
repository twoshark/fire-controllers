# mp-output — wiring

```text
PanelPole → POWER (KCD4) → J1     [seat F9]
M12-5 → J2
SOL0..SOL2 (DT) → J6 + J5a
DFU / RESET / BOOT: open lid
```

| SOL | OUT |
| --- | --- |
| 0..2 | J5a.1..3 |

DT pin1 = +12V, pin2 = OUTn. 18 AWG.

External 12 V from mp-output-power: Powerpole jumper **≤ 4 ft**, 12 AWG.
