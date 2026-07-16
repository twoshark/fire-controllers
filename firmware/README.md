# Firmware

Embassy / Rust `no_std` firmware for the v1.0.0 STM32G0B1 boards.

| Crate | Board | Binary |
| --- | --- | --- |
| `hotline-protocol` | shared | library + host tests |
| `input-controller` | input PCB | `target/thumbv6m-none-eabi/release/input-controller` |
| `output-controller` | output PCB | `target/thumbv6m-none-eabi/release/output-controller` |

Pin map: [`../hardware/v1.0.0/PIN_MAP.md`](../hardware/v1.0.0/PIN_MAP.md)
Bring-up: [`../hardware/v1.0.0/BRINGUP.md`](../hardware/v1.0.0/BRINGUP.md)

## Quick start

```bash
# From repo root
./scripts/test.sh
./scripts/flash-input.sh    # SWD probe on input board
./scripts/flash-output.sh   # SWD probe on output board
```

Chip target for probe-rs: `STM32G0B1CBTx`.

## Behavior summary

### Input controller

- Netlist pin map: CH0..5 on `PA0`/`PA1`/`PA4`..`PA7` (EXTI); CH6..7 on `PB0`/`PB1` (1 ms poll)
- Packs 8 channels into Hotline v2 state frame `[0xAA][state][CRC8]`
- Transmits on every input edge, plus 25 ms idle keepalive (`STATE_KEEPALIVE_MS`)
- Mirrors channel state on LEDs (`PB10`,`PA15`,`PB3`..`PB8`, active-low)
- Monitors heartbeat on USART1 RX; drives link LED (`PB9`)

### Output controller

- Receives state frames on USART1 RX
- Overrides on `PA0`..`PA7` force channel ON
- Gates on `PB0`,`PB1`,`PB2`,`PB10`..`PB14` (default low / OFF)
- Comm watchdog ~100ms -> failsafe all-OFF
- Heartbeat TX at 10Hz; link LED on `PC7`

## Protocol

See [`../docs/serial-protocol.md`](../docs/serial-protocol.md).

- UART 115200 8N1
- CRC-8/MAXIM
- State keepalive and heartbeat intervals defined in `hotline-protocol`
