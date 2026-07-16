# Hotline v2 Serial Protocol Specification

## Overview

Hotline v2 uses **full-duplex** RS-485.

- Input board -> Output board: state frame on every input edge, plus 25 ms idle keepalive (~40 Hz when quiet)
- Output board -> Input board: heartbeat frame at 10Hz
- Both directions run continuously on separate differential pairs

| Parameter | Value |
| --- | --- |
| Physical layer | RS-485 full-duplex (2 pairs) |
| UART | 115200 baud, 8N1 |
| Frame length | 3 bytes |
| State frame rate | Edge-driven + 25 ms keepalive |
| Heartbeat frame rate | 10Hz |

## Physical layer

- Two SP3485EN transceivers per board:
  - TX transceiver (`U2A`): DE/RE# strapped to 3V3 (always transmit)
  - RX transceiver (`U2B`): DE/RE# strapped to GND (always receive)
- Cable: Belden 9842 (or equivalent), 2-pair shielded twisted pair, 120R impedance
- Signals: `TX+`, `TX-`, `RX+`, `RX-`, `GND`, `SHIELD`
- One 120R termination on each receiver pair
- One SM712 TVS per differential pair
- Point-to-point wiring crossover:
  - Input `TX+/-` -> Output `RX+/-`
  - Output `TX+/-` -> Input `RX+/-`
- Terminal pin maps: [`hardware/v1.0.0/PIN_MAP.md`](../hardware/v1.0.0/PIN_MAP.md)
  - Input RS-485 connector: **`CN2`**
  - Output RS-485 connector: **`J2`**
  - Both boards: `U2A`=TX, `U2B`=RX

No DE/RE firmware bus-turnaround logic is used.

## Frame formats

### State frame (input -> output)

```text
[0xAA] [channels] [crc8(channels)]
```

- `channels` is 8 bits (`bit0=CH0 ... bit7=CH7`)
- `1` means channel active
- Frame represents current channel state only (no edge/event history)

### Heartbeat frame (output -> input)

```text
[0x55] [status] [crc8(status)]
```

- `status bit0`: output board healthy
- `status bit1`: failsafe active
- `status bit2..7`: reserved (0)

## CRC

CRC-8/MAXIM (Dallas):

- Polynomial: `0x31`
- Init: `0x00`
- Reflect in/out: yes
- Final xor: `0x00`

Validation vector: `CRC8("123456789") = 0xA1`.

## Timing and latency

- One 3-byte UART frame at 115200 takes about 260us on wire.
- Edge-triggered state TX keeps end-to-end latency in the low milliseconds; 25 ms keepalive keeps the output watchdog fed when inputs are idle.
- 10Hz heartbeat provides fast link-health feedback.

## Failsafe

- Output board enables failsafe if no valid state frame is received for 100ms.
- In failsafe, serial-derived channels are forced OFF; local override inputs still force their channels ON.
- Heartbeat status reports the failsafe bit back to input board.
