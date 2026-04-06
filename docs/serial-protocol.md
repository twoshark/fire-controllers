# Hotline v2 Serial Protocol Specification

## 1. Overview

The fire controller uses **Hotline v2**, a half-duplex bidirectional serial protocol over RS-485. The primary direction transmits 8 digital input states from the input board to the output board at 1kHz. A reverse heartbeat from the output board (at 10Hz) confirms link health. The protocol is designed for minimum latency, maximum simplicity, and high reliability.

| Parameter | Value |
|-----------|-------|
| Direction | Half-duplex bidirectional (primary: input → output state at 1kHz, reverse: output → input heartbeat at 10Hz) |
| Physical layer | RS-485 half-duplex |
| Baud rate | 115200 |
| UART framing | 8N1 (8 data bits, no parity, 1 stop bit) |
| Protocol frame size | 3 bytes |
| Transmission rate | 1kHz (one frame per millisecond) |
| Bandwidth utilization | ~26% of available RS-485 bandwidth |

---

## 2. Physical Layer

### RS-485 Signaling

Both boards use the **SP3485EN** RS-485 transceiver (MaxLinear, SOIC-8 package):
- 3.3V supply, half-duplex, 32-node bus capability
- Up to 10 Mbps (115200 baud is ~1% of capacity)
- ±15kV ESD protection on bus pins
- Supply current: ~0.5mA typical (2mA max active TX)

Both boards toggle the DE/RE pin via firmware to switch between TX and RX modes. The input board is the bus master: it transmits state frames, then briefly switches to RX mode to listen for heartbeat responses. The output board defaults to RX mode and briefly switches to TX mode to transmit heartbeat frames (every ~100ms).

### Cable

**Belden 9841**: 1-pair 24AWG shielded twisted pair, 120Ω characteristic impedance, specifically designed for RS-485/EIA-485. Maximum run length: 200ft (61m).

**Termination**: A 120Ω resistor at the receiver end (output board) matches the cable characteristic impedance and prevents signal reflections. No termination at the transmitter end (short stub).

**Shield grounding**: The cable shield/drain wire is grounded at one end only (output board) to prevent ground loops while still providing EMI shielding.

### Connector

**M12 4-pin** industrial connectors (panel-mount, IP67) at each enclosure:

| Pin | Signal |
|-----|--------|
| 1 | RS-485 A (+) |
| 2 | RS-485 B (−) |
| 3 | Signal GND |
| 4 | Shield/Drain |

### Propagation Delay

At 200ft with typical twisted-pair velocity (~5ns/ft), cable propagation delay is ~1μs — negligible relative to the 10ms latency budget.

### Bus Turnaround Protocol

The input board acts as **bus master** and controls all turnaround timing:

1. **Input board transmits** state frame (3 bytes, ~260μs) with DE/RE HIGH (TX mode)
2. **Input board switches to RX**: DE/RE LOW, waits for heartbeat response
3. **Guard time**: 50μs minimum after DE/RE transition to allow transceiver settling
4. **Output board transmits** heartbeat (if due): DE/RE HIGH for 3 bytes (~260μs), then DE/RE LOW
5. **Input board RX window**: ~500μs total (50μs guard + 260μs frame + 190μs margin)
6. **Input board switches back to TX**: DE/RE HIGH, ready for next state frame

The heartbeat is only transmitted once per ~100ms (every ~100th state frame cycle). On non-heartbeat cycles, the input board still briefly enters RX mode but sees no response — this keeps timing predictable. The bus turnaround overhead is ~500μs per cycle, leaving ~240μs idle time within each 1ms polling interval.

```
Timeline (1ms cycle, heartbeat cycle):
|--TX state frame--|--guard--|--RX heartbeat--|--guard--|--idle--|
0              260μs   310μs            570μs    620μs     1000μs
                         ▲                        ▲
                    DE/RE LOW               DE/RE HIGH
```

---

## 3. Frame Format

The protocol uses a fixed-length 3-byte frame:

```
Byte:    0          1            2
       ┌──────┐  ┌──────────┐  ┌──────┐
       │ 0xAA │  │ Payload  │  │ CRC8 │
       │ SYNC │  │ 8-bit    │  │      │
       └──────┘  └──────────┘  └──────┘
```

### Byte 0 — Sync Marker (0xAA)

Fixed value `0xAA` (binary `10101010`). The receiver scans the incoming byte stream for this value to align to frame boundaries.

The value 0xAA was chosen because:
- Its alternating bit pattern is statistically unlikely to appear as a valid CRC of common payloads
- It provides good clock recovery properties for the UART receiver
- It is a widely used sync byte in embedded protocols

### Byte 1 — Payload

An 8-bit value where each bit represents one input channel:

| Bit | Channel | Meaning when 1 | Meaning when 0 |
|-----|---------|-----------------|-----------------|
| 0 | CH0 | Input active (HIGH) | Input inactive (LOW) |
| 1 | CH1 | Input active (HIGH) | Input inactive (LOW) |
| 2 | CH2 | Input active (HIGH) | Input inactive (LOW) |
| 3 | CH3 | Input active (HIGH) | Input inactive (LOW) |
| 4 | CH4 | Input active (HIGH) | Input inactive (LOW) |
| 5 | CH5 | Input active (HIGH) | Input inactive (LOW) |
| 6 | CH6 | Input active (HIGH) | Input inactive (LOW) |
| 7 | CH7 | Input active (HIGH) | Input inactive (LOW) |

Bit 0 is the LSB. A payload of `0x00` means all inputs inactive; `0xFF` means all inputs active.

### Byte 2 — CRC8 Checksum

CRC-8/MAXIM computed over the **payload byte only** (byte 1). The sync byte is excluded from the CRC computation because it is a fixed constant — including it would add computation with no error-detection benefit.

### Heartbeat Frame (Output → Input)

The output board periodically transmits a heartbeat frame back to the input board to confirm link health:

```
Byte:    0          1            2
       ┌──────┐  ┌──────────┐  ┌──────┐
       │ 0x55 │  │ Status   │  │ CRC8 │
       │ SYNC │  │ 8-bit    │  │      │
       └──────┘  └──────────┘  └──────┘
```

**Byte 0 — Heartbeat sync marker (0x55)**: Fixed value `0x55` (binary `01010101`). Distinct from the state frame's `0xAA`, allowing both frame types to coexist on the same bus without ambiguity.

**Byte 1 — Status payload**:

| Bit | Meaning |
|-----|---------|
| 0 | Output board healthy (1 = OK) |
| 1 | Watchdog state (1 = failsafe active) |
| 2–7 | Reserved (set to 0) |

**Byte 2 — CRC8**: Same CRC-8/MAXIM algorithm as state frames, computed over the status byte only.

**Transmission rate**: 10Hz (every ~100ms). This provides timely link health feedback without consuming significant bandwidth (3 bytes × 10 Hz = 30 bytes/sec, <0.3% of bus capacity).

---

## 4. CRC-8/MAXIM Algorithm

### Parameters

| Parameter | Value |
|-----------|-------|
| Algorithm | CRC-8/MAXIM (CRC-8/Dallas) |
| Polynomial | 0x31 (x⁸ + x⁵ + x⁴ + 1) |
| Initial value | 0x00 |
| Reflect input | Yes |
| Reflect output | Yes |
| Final XOR | 0x00 |
| Check value | CRC8("123456789") = 0xA1 (standard check value per CRC catalogue) |

### Lookup Table

For firmware implementation, a 256-byte lookup table provides O(1) CRC computation per byte:

```rust
const CRC8_TABLE: [u8; 256] = [
    0x00, 0x5E, 0xBC, 0xE2, 0x61, 0x3F, 0xDD, 0x83,
    0xC2, 0x9C, 0x7E, 0x20, 0xA3, 0xFD, 0x1F, 0x41,
    0x9D, 0xC3, 0x21, 0x7F, 0xFC, 0xA2, 0x40, 0x1E,
    0x5F, 0x01, 0xE3, 0xBD, 0x3E, 0x60, 0x82, 0xDC,
    0x23, 0x7D, 0x9F, 0xC1, 0x42, 0x1C, 0xFE, 0xA0,
    0xE1, 0xBF, 0x5D, 0x03, 0x80, 0xDE, 0x3C, 0x62,
    0xBE, 0xE0, 0x02, 0x5C, 0xDF, 0x81, 0x63, 0x3D,
    0x7C, 0x22, 0xC0, 0x9E, 0x1D, 0x43, 0xA1, 0xFF,
    0x46, 0x18, 0xFA, 0xA4, 0x27, 0x79, 0x9B, 0xC5,
    0x84, 0xDA, 0x38, 0x66, 0xE5, 0xBB, 0x59, 0x07,
    0xDB, 0x85, 0x67, 0x39, 0xBA, 0xE4, 0x06, 0x58,
    0x19, 0x47, 0xA5, 0xFB, 0x78, 0x26, 0xC4, 0x9A,
    0x65, 0x3B, 0xD9, 0x87, 0x04, 0x5A, 0xB8, 0xE6,
    0xA7, 0xF9, 0x1B, 0x45, 0xC6, 0x98, 0x7A, 0x24,
    0xF8, 0xA6, 0x44, 0x1A, 0x99, 0xC7, 0x25, 0x7B,
    0x3A, 0x64, 0x86, 0xD8, 0x5B, 0x05, 0xE7, 0xB9,
    0x8C, 0xD2, 0x30, 0x6E, 0xED, 0xB3, 0x51, 0x0F,
    0x4E, 0x10, 0xF2, 0xAC, 0x2F, 0x71, 0x93, 0xCD,
    0x11, 0x4F, 0xAD, 0xF3, 0x70, 0x2E, 0xCC, 0x92,
    0xD3, 0x8D, 0x6F, 0x31, 0xB2, 0xEC, 0x0E, 0x50,
    0xAF, 0xF1, 0x13, 0x4D, 0xCE, 0x90, 0x72, 0x2C,
    0x6D, 0x33, 0xD1, 0x8F, 0x0C, 0x52, 0xB0, 0xEE,
    0x32, 0x6C, 0x8E, 0xD0, 0x53, 0x0D, 0xEF, 0xB1,
    0xF0, 0xAE, 0x4C, 0x12, 0x91, 0xCF, 0x2D, 0x73,
    0xCA, 0x94, 0x76, 0x28, 0xAB, 0xF5, 0x17, 0x49,
    0x08, 0x56, 0xB4, 0xEA, 0x69, 0x37, 0xD5, 0x8B,
    0x57, 0x09, 0xEB, 0xB5, 0x36, 0x68, 0x8A, 0xD4,
    0x95, 0xCB, 0x29, 0x77, 0xF4, 0xAA, 0x48, 0x16,
    0xE9, 0xB7, 0x55, 0x0B, 0x88, 0xD6, 0x34, 0x6A,
    0x2B, 0x75, 0x97, 0xC9, 0x4A, 0x14, 0xF6, 0xA8,
    0x74, 0x2A, 0xC8, 0x96, 0x15, 0x4B, 0xA9, 0xF7,
    0xB6, 0xE8, 0x0A, 0x54, 0xD7, 0x89, 0x6B, 0x35,
];

pub fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0x00;
    for &byte in data {
        crc = CRC8_TABLE[(crc ^ byte) as usize];
    }
    crc
}
```

### Verification Vectors

| Payload byte | Expected CRC8 |
|-------------|---------------|
| 0x00 | 0x00 |
| 0xFF | 0x35 |
| 0x55 | 0xE4 |
| 0xAA | 0xD1 |
| 0x01 | 0x5E |

---

## 5. Transmitter State Machine (Input Board)

The input board transmits continuously at 1kHz using a simple periodic loop:

```
┌─────────────┐
│   IDLE      │ ◄── Timer fires every 1ms
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ ADC SCAN    │  Read 8 analog channels
│             │  Apply thresholds → 8-bit state
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ BUILD FRAME │  [0xAA] [state] [CRC8(state)]
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ UART TX     │  Async write 3 bytes to USART1
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ UPDATE LEDs │  Set 8 channel LEDs + link LED
└──────┬──────┘
       │
       ▼ (loop)
```

The RS-485 transceiver DE/RE pin is set HIGH for TX before each frame transmission. After the frame completes, firmware toggles DE/RE LOW for a brief RX window (~500μs) to listen for heartbeat responses from the output board, then toggles back to HIGH for the next cycle.

---

## 6. Receiver State Machine (Output Board)

The output board receives and validates frames using a byte-level state machine:

```
                    ┌───────────────────────────────┐
                    │                               │
                    ▼                               │
             ┌────────────┐                         │
      ┌──────│ WAIT_SYNC  │◄──────────┐            │
      │      └─────┬──────┘           │            │
      │            │ byte == 0xAA     │            │
      │            ▼                  │            │
      │      ┌────────────┐           │            │
      │      │ READ_DATA  │           │            │
      │      │ (1 byte)   │           │            │
      │      └─────┬──────┘           │            │
      │            │ store payload    │            │
      │            ▼                  │            │
      │      ┌────────────┐           │            │
      │      │ READ_CRC   │           │            │
      │      │ (1 byte)   │           │            │
      │      └─────┬──────┘           │            │
      │            │                  │            │
      │      ┌─────┴─────┐           │            │
      │      │ CRC valid? │           │            │
      │      └─────┬──────┘           │            │
      │       yes  │    no            │            │
      │            │    └─────────────┘            │
      │            ▼                               │
      │      ┌────────────┐                        │
      │      │ ACCEPT     │  Update channel state  │
      │      │ FRAME      │  Reset watchdog timer  │
      │      └─────┬──────┘                        │
      │            │                               │
      │            └───────────────────────────────┘
      │
      │ byte != 0xAA
      │ (discard, stay in WAIT_SYNC)
      └──────────────────────┘
```

### Key Behaviors

**Byte-at-a-time processing**: The receiver processes one byte per UART interrupt/DMA event. No buffering of a full frame before processing — the state machine advances incrementally.

**False sync recovery**: If a non-0xAA byte arrives in WAIT_SYNC state, it is silently discarded. This handles mid-stream startup and re-synchronization after errors.

**CRC failure**: On CRC mismatch, the receiver returns to WAIT_SYNC and begins scanning for the next 0xAA. The failed payload is discarded. No retry or NAK is sent.

**Payload byte == 0xAA**: If the payload byte happens to be 0xAA (all odd channels active), the state machine handles this correctly because it only looks for 0xAA in the WAIT_SYNC state. Once in READ_DATA, the next byte is always treated as payload regardless of value.

---

## 7. Timing Analysis

### Per-Frame Timing

| Stage | Duration | Cumulative |
|-------|----------|------------|
| ADC scan (8 channels, 12-bit SAR) | ~10μs | 10μs |
| Frame construction + CRC lookup | ~1μs | 11μs |
| UART TX (3 bytes × 10 bits @ 115200) | 260μs | 271μs |
| Cable propagation (200ft) | ~1μs | 272μs |
| UART RX + interrupt latency | ~15μs | 287μs |
| Frame validation + CRC check | ~2μs | 289μs |
| Override ADC check (8 channels) | ~10μs | 299μs |
| GPIO output write | ~0.1μs | ~300μs |

### End-to-End Latency

| Scenario | Latency | Notes |
|----------|---------|-------|
| Best case | ~300μs | Input changes just before ADC scan |
| Typical (mid-poll) | ~800μs | Input changes midway through polling interval |
| Worst case | ~1.3ms | Input changes just after a poll completes |
| Worst + 1 dropped frame | ~2.3ms | One frame corrupted, next frame valid |
| **Design target** | **<10ms** | **7.7ms margin in worst case** |

### Bandwidth Budget

- Wire capacity at 115200 baud: 11520 bytes/sec
- Frame consumption: 3 bytes × 1000 Hz = 3000 bytes/sec
- **Utilization: 26%**
- Remaining 74% is available for future protocol extensions (ACK frames, diagnostics, configuration commands)

---

## 8. Error Handling

### Communication Errors Detected

| Error Type | Detection Method | Recovery |
|------------|-----------------|----------|
| Bit errors in payload | CRC8 mismatch | Discard frame, resync |
| Bit errors in sync byte | Sync byte not found | Stay in WAIT_SYNC |
| Bit errors in CRC byte | CRC8 mismatch | Discard frame, resync |
| UART framing error | UART peripheral flag | Discard byte, resync |
| Cable disconnection | Watchdog timeout (100ms) | All outputs OFF (failsafe) |
| Noise burst (multi-byte) | CRC + resync | Skip corrupted region, recover on next valid frame |

### CRC8 Detection Capability

CRC-8/MAXIM with polynomial 0x31 detects:
- All single-bit errors
- All odd-count bit errors
- All burst errors ≤ 8 bits
- Most burst errors > 8 bits (undetected error probability ~1/256 for random errors)

For a 1-byte payload (8 bits of data protected), this provides very strong coverage.

### Undetectable Errors

If the sync byte (0xAA) is corrupted to look like a different byte, the frame is lost (not mis-decoded). If a random byte stream happens to form a valid [0xAA][payload][correct CRC] sequence, a false frame could be accepted — but the probability is approximately 1/(256 × 256) = 1/65536 per byte. At 115200 baud, this is roughly once per 9.5 minutes of continuous random noise, and the output would be corrected by the next valid frame 1ms later.

---

## 9. Watchdog / Failsafe

The output board implements a **communication watchdog**:

| Parameter | Value |
|-----------|-------|
| Timeout | 100ms |
| Missed frames at timeout | ~100 (at 1kHz TX rate) |
| Failsafe action | All 8 outputs driven LOW (OFF) |
| LED indication | Link LED shows error pattern (fast blink or off) |
| Recovery | Automatic on next valid frame received |

The 100ms timeout balances responsiveness with tolerance for transient errors:
- **Short enough**: Outputs turn off within 100ms of cable disconnection or input board failure
- **Long enough**: Rides through burst interference that might corrupt several consecutive frames without triggering a false failsafe

### Watchdog State Transitions

```
                    Valid frame received
              ┌──────────────────────────┐
              │                          │
              ▼                          │
       ┌────────────┐            ┌──────┴─────┐
       │   HEALTHY  │──────────►│  COUNTING  │
       │            │  Timer    │  (no valid  │
       │ Outputs    │  starts   │   frames)   │
       │ follow     │           └──────┬──────┘
       │ serial     │                  │
       └────────────┘           100ms elapsed
              ▲                        │
              │                        ▼
              │                 ┌────────────┐
              │  Valid frame    │  FAILSAFE  │
              └─────────────── │            │
                               │ All outputs│
                               │ OFF        │
                               └────────────┘
```

---

## 10. Future Protocol Extensions

The current protocol reserves capacity for future enhancements. These can be implemented without hardware changes:

**Extended frame format**: A longer frame format (different sync byte, e.g., 0xBB) could add payload bytes for diagnostics, firmware version, error counters, or per-channel configuration. The heartbeat frame (0x55) demonstrates this extensibility pattern.

**Variable transmission rate**: Reduce to lower rate (e.g., 100Hz) during idle periods to save power, increase to 1kHz+ when input changes are detected. The watchdog timeout would need to be adjusted accordingly.
