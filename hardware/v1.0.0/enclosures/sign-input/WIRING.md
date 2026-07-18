# sign-input — wiring

```text
C14 → POWER (KCD4+boot) → RS-15-12 AC
RS-15 → J1
M12-5 → CN2
Top EG STARTS → J2 / buttons PCB
DFU / RESET / BOOT: open lid
```

| Top | PCB |
| --- | --- |
| CH1..CH5 NO | `J2a.1`..`J2b.1` |
| ALL | D1..D5 |
| LED each | RS-15 +12V / GND |

| M12 | CN2 |
| ---: | ---: |
| 1–5 | TX+ TX− RX+ RX− GND |
| Shell | pin 6 SHIELD |

External: **6 ft** C13→C14. Cap M12 when unused.
