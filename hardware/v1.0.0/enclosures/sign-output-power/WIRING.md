# sign-output-power — wiring

```text
C14 L → POWER → LRS AC/L (pins 1)
C14 N → LRS AC/N (pins 2)
C14 PE → LRS FG (pin 3)
LRS +V (7–9) / −V (4–6) → both fans + PanelPole2 12V OUT
```

### PanelPole2 12V OUT

| Pole | Net |
| --- | --- |
| Red | LRS +V |
| Black | LRS −V |

### Fans

| Fan | + | − |
| --- | --- | --- |
| Intake + exhaust | LRS +V | LRS −V |

On with POWER (same DC rail as PanelPole).

### Setup

1. Selector **115 V**, trim **12.0 V**.  
2. Intake: 09250-F/60 + fan. Exhaust: 8147 + fan.  
3. Field cable to sign-output: Powerwerx **EX-12-5**.

| Path | Gauge |
| --- | --- |
| AC | 18 AWG |
| LRS → PanelPole / fans | **12 AWG** (PP30) |
