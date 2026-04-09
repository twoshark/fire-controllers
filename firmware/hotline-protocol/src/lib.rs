#![no_std]
#![doc = "Hotline v2 protocol crate for the fire controller system."]
#![doc = ""]
#![doc = "Provides CRC-8/MAXIM, state frame and heartbeat frame encoding/decoding,"]
#![doc = "and a byte-at-a-time frame decoder state machine. This crate has no"]
#![doc = "hardware dependencies and is fully testable on the host."]

#[cfg(feature = "defmt")]
use defmt::Format;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const STATE_SYNC: u8 = 0xAA;
pub const HEARTBEAT_SYNC: u8 = 0x55;
pub const FRAME_LEN: usize = 3;

/// Communication watchdog timeout in milliseconds.
pub const COMM_WATCHDOG_TIMEOUT_MS: u64 = 100;

/// Heartbeat transmission interval in milliseconds.
pub const HEARTBEAT_INTERVAL_MS: u64 = 100;

/// Maximum allowed age of the latest heartbeat before link is considered unhealthy.
pub const HEARTBEAT_LOSS_TIMEOUT_MS: u64 = 500;

/// Polling interval in milliseconds.
pub const POLL_INTERVAL_MS: u64 = 1;

// ---------------------------------------------------------------------------
// CRC-8/MAXIM
// ---------------------------------------------------------------------------

/// CRC-8/MAXIM (Dallas) lookup table.
/// Polynomial 0x31, init 0x00, reflect-in, reflect-out.
const CRC8_TABLE: [u8; 256] = [
    0x00, 0x5E, 0xBC, 0xE2, 0x61, 0x3F, 0xDD, 0x83, 0xC2, 0x9C, 0x7E, 0x20, 0xA3, 0xFD, 0x1F, 0x41,
    0x9D, 0xC3, 0x21, 0x7F, 0xFC, 0xA2, 0x40, 0x1E, 0x5F, 0x01, 0xE3, 0xBD, 0x3E, 0x60, 0x82, 0xDC,
    0x23, 0x7D, 0x9F, 0xC1, 0x42, 0x1C, 0xFE, 0xA0, 0xE1, 0xBF, 0x5D, 0x03, 0x80, 0xDE, 0x3C, 0x62,
    0xBE, 0xE0, 0x02, 0x5C, 0xDF, 0x81, 0x63, 0x3D, 0x7C, 0x22, 0xC0, 0x9E, 0x1D, 0x43, 0xA1, 0xFF,
    0x46, 0x18, 0xFA, 0xA4, 0x27, 0x79, 0x9B, 0xC5, 0x84, 0xDA, 0x38, 0x66, 0xE5, 0xBB, 0x59, 0x07,
    0xDB, 0x85, 0x67, 0x39, 0xBA, 0xE4, 0x06, 0x58, 0x19, 0x47, 0xA5, 0xFB, 0x78, 0x26, 0xC4, 0x9A,
    0x65, 0x3B, 0xD9, 0x87, 0x04, 0x5A, 0xB8, 0xE6, 0xA7, 0xF9, 0x1B, 0x45, 0xC6, 0x98, 0x7A, 0x24,
    0xF8, 0xA6, 0x44, 0x1A, 0x99, 0xC7, 0x25, 0x7B, 0x3A, 0x64, 0x86, 0xD8, 0x5B, 0x05, 0xE7, 0xB9,
    0x8C, 0xD2, 0x30, 0x6E, 0xED, 0xB3, 0x51, 0x0F, 0x4E, 0x10, 0xF2, 0xAC, 0x2F, 0x71, 0x93, 0xCD,
    0x11, 0x4F, 0xAD, 0xF3, 0x70, 0x2E, 0xCC, 0x92, 0xD3, 0x8D, 0x6F, 0x31, 0xB2, 0xEC, 0x0E, 0x50,
    0xAF, 0xF1, 0x13, 0x4D, 0xCE, 0x90, 0x72, 0x2C, 0x6D, 0x33, 0xD1, 0x8F, 0x0C, 0x52, 0xB0, 0xEE,
    0x32, 0x6C, 0x8E, 0xD0, 0x53, 0x0D, 0xEF, 0xB1, 0xF0, 0xAE, 0x4C, 0x12, 0x91, 0xCF, 0x2D, 0x73,
    0xCA, 0x94, 0x76, 0x28, 0xAB, 0xF5, 0x17, 0x49, 0x08, 0x56, 0xB4, 0xEA, 0x69, 0x37, 0xD5, 0x8B,
    0x57, 0x09, 0xEB, 0xB5, 0x36, 0x68, 0x8A, 0xD4, 0x95, 0xCB, 0x29, 0x77, 0xF4, 0xAA, 0x48, 0x16,
    0xE9, 0xB7, 0x55, 0x0B, 0x88, 0xD6, 0x34, 0x6A, 0x2B, 0x75, 0x97, 0xC9, 0x4A, 0x14, 0xF6, 0xA8,
    0x74, 0x2A, 0xC8, 0x96, 0x15, 0x4B, 0xA9, 0xF7, 0xB6, 0xE8, 0x0A, 0x54, 0xD7, 0x89, 0x6B, 0x35,
];

/// Compute CRC-8/MAXIM over a byte slice.
pub fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0x00;
    for &byte in data {
        crc = CRC8_TABLE[(crc ^ byte) as usize];
    }
    crc
}

// ---------------------------------------------------------------------------
// State frame (input -> output, 0xAA sync)
// ---------------------------------------------------------------------------

/// An 8-channel state frame transmitted from input board to output board at 1 kHz.
///
/// Wire format: `[0xAA][payload][CRC8]` (3 bytes total).
/// Each bit in `channels` maps to one channel (bit 0 = CH0, bit 7 = CH7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct StateFrame {
    pub channels: u8,
}

impl StateFrame {
    pub fn new(channels: u8) -> Self {
        Self { channels }
    }

    /// Encode into a 3-byte wire frame.
    pub fn encode(&self) -> [u8; FRAME_LEN] {
        let crc = crc8(&[self.channels]);
        [STATE_SYNC, self.channels, crc]
    }

    /// Decode from a 3-byte buffer. Returns `None` on sync or CRC mismatch.
    pub fn decode(buf: &[u8; FRAME_LEN]) -> Option<Self> {
        if buf[0] != STATE_SYNC {
            return None;
        }
        let expected_crc = crc8(&[buf[1]]);
        if buf[2] != expected_crc {
            return None;
        }
        Some(Self { channels: buf[1] })
    }

    /// Read a single channel state (true = active/HIGH).
    pub fn channel(&self, ch: u8) -> bool {
        debug_assert!(ch < 8);
        self.channels & (1 << ch) != 0
    }
}

// ---------------------------------------------------------------------------
// Heartbeat frame (output -> input, 0x55 sync)
// ---------------------------------------------------------------------------

/// Heartbeat status bits.
pub const STATUS_HEALTHY: u8 = 1 << 0;
pub const STATUS_FAILSAFE_ACTIVE: u8 = 1 << 1;

/// A heartbeat frame transmitted from output board to input board at 10 Hz.
///
/// Wire format: `[0x55][status][CRC8]` (3 bytes total).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct HeartbeatFrame {
    pub status: u8,
}

impl HeartbeatFrame {
    pub fn new(healthy: bool, failsafe_active: bool) -> Self {
        let mut status = 0u8;
        if healthy {
            status |= STATUS_HEALTHY;
        }
        if failsafe_active {
            status |= STATUS_FAILSAFE_ACTIVE;
        }
        Self { status }
    }

    pub fn encode(&self) -> [u8; FRAME_LEN] {
        let crc = crc8(&[self.status]);
        [HEARTBEAT_SYNC, self.status, crc]
    }

    pub fn decode(buf: &[u8; FRAME_LEN]) -> Option<Self> {
        if buf[0] != HEARTBEAT_SYNC {
            return None;
        }
        let expected_crc = crc8(&[buf[1]]);
        if buf[2] != expected_crc {
            return None;
        }
        Some(Self { status: buf[1] })
    }

    pub fn is_healthy(&self) -> bool {
        self.status & STATUS_HEALTHY != 0
    }

    pub fn is_failsafe_active(&self) -> bool {
        self.status & STATUS_FAILSAFE_ACTIVE != 0
    }
}

// ---------------------------------------------------------------------------
// Frame decoder state machine
// ---------------------------------------------------------------------------

/// Result from feeding a byte into the decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub enum DecoderEvent {
    /// No complete frame yet; keep feeding bytes.
    None,
    /// A valid state frame was decoded.
    State(StateFrame),
    /// A valid heartbeat frame was decoded.
    Heartbeat(HeartbeatFrame),
    /// A frame with correct sync was received but CRC failed.
    CrcError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DecoderState {
    WaitSync,
    ReadData { sync: u8 },
    ReadCrc { sync: u8, payload: u8 },
}

/// Byte-at-a-time frame decoder for both state and heartbeat frames.
///
/// Feed bytes from the UART one at a time via [`FrameDecoder::feed`].
/// The decoder recognises both `0xAA` (state) and `0x55` (heartbeat) sync
/// bytes and validates the CRC before emitting a frame event.
#[derive(Clone, Debug)]
pub struct FrameDecoder {
    state: DecoderState,
}

impl FrameDecoder {
    pub const fn new() -> Self {
        Self {
            state: DecoderState::WaitSync,
        }
    }

    /// Reset the decoder to the initial sync-hunting state.
    pub fn reset(&mut self) {
        self.state = DecoderState::WaitSync;
    }

    /// Feed one received byte and return any resulting event.
    pub fn feed(&mut self, byte: u8) -> DecoderEvent {
        match self.state {
            DecoderState::WaitSync => {
                if byte == STATE_SYNC || byte == HEARTBEAT_SYNC {
                    self.state = DecoderState::ReadData { sync: byte };
                }
                DecoderEvent::None
            }
            DecoderState::ReadData { sync } => {
                self.state = DecoderState::ReadCrc {
                    sync,
                    payload: byte,
                };
                DecoderEvent::None
            }
            DecoderState::ReadCrc { sync, payload } => {
                self.state = DecoderState::WaitSync;
                let expected_crc = crc8(&[payload]);
                if byte != expected_crc {
                    return DecoderEvent::CrcError;
                }
                if sync == STATE_SYNC {
                    DecoderEvent::State(StateFrame { channels: payload })
                } else {
                    DecoderEvent::Heartbeat(HeartbeatFrame { status: payload })
                }
            }
        }
    }
}

impl Default for FrameDecoder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests (host-runnable)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- CRC verification vectors from the protocol spec --

    #[test]
    fn crc8_verification_vectors() {
        assert_eq!(crc8(&[0x00]), 0x00);
        assert_eq!(crc8(&[0xFF]), 0x35);
        assert_eq!(crc8(&[0x55]), 0xE4);
        assert_eq!(crc8(&[0xAA]), 0xD1);
        assert_eq!(crc8(&[0x01]), 0x5E);
    }

    #[test]
    fn crc8_standard_check_value() {
        // CRC-8/MAXIM standard check: CRC of ASCII "123456789" = 0xA1
        assert_eq!(crc8(b"123456789"), 0xA1);
    }

    #[test]
    fn crc8_empty() {
        assert_eq!(crc8(&[]), 0x00);
    }

    // -- State frame round-trips --

    #[test]
    fn state_frame_round_trip_all_zero() {
        let frame = StateFrame::new(0x00);
        let encoded = frame.encode();
        assert_eq!(encoded[0], STATE_SYNC);
        let decoded = StateFrame::decode(&encoded).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn state_frame_round_trip_all_ones() {
        let frame = StateFrame::new(0xFF);
        let encoded = frame.encode();
        let decoded = StateFrame::decode(&encoded).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn state_frame_round_trip_alternating() {
        for payload in [0x55u8, 0xAA, 0x0F, 0xF0, 0x01, 0x80] {
            let frame = StateFrame::new(payload);
            let decoded = StateFrame::decode(&frame.encode()).unwrap();
            assert_eq!(decoded, frame, "failed for payload 0x{payload:02X}");
        }
    }

    #[test]
    fn state_frame_payload_equals_sync() {
        // Payload 0xAA must not confuse the decoder
        let frame = StateFrame::new(0xAA);
        let decoded = StateFrame::decode(&frame.encode()).unwrap();
        assert_eq!(decoded.channels, 0xAA);
    }

    #[test]
    fn state_frame_bad_sync() {
        assert!(StateFrame::decode(&[0x00, 0x00, 0x00]).is_none());
        assert!(StateFrame::decode(&[0x55, 0x00, 0x00]).is_none());
    }

    #[test]
    fn state_frame_bad_crc() {
        let mut encoded = StateFrame::new(0x42).encode();
        encoded[2] ^= 0xFF; // corrupt CRC
        assert!(StateFrame::decode(&encoded).is_none());
    }

    #[test]
    fn state_frame_channel_access() {
        let frame = StateFrame::new(0b1010_0101);
        assert!(frame.channel(0));
        assert!(!frame.channel(1));
        assert!(frame.channel(2));
        assert!(!frame.channel(3));
        assert!(!frame.channel(4));
        assert!(frame.channel(5));
        assert!(!frame.channel(6));
        assert!(frame.channel(7));
    }

    // -- Heartbeat frame round-trips --

    #[test]
    fn heartbeat_frame_round_trip() {
        let frame = HeartbeatFrame::new(true, false);
        let encoded = frame.encode();
        assert_eq!(encoded[0], HEARTBEAT_SYNC);
        let decoded = HeartbeatFrame::decode(&encoded).unwrap();
        assert_eq!(decoded, frame);
        assert!(decoded.is_healthy());
        assert!(!decoded.is_failsafe_active());
    }

    #[test]
    fn heartbeat_frame_failsafe() {
        let frame = HeartbeatFrame::new(false, true);
        let decoded = HeartbeatFrame::decode(&frame.encode()).unwrap();
        assert!(!decoded.is_healthy());
        assert!(decoded.is_failsafe_active());
    }

    #[test]
    fn heartbeat_frame_bad_sync() {
        assert!(HeartbeatFrame::decode(&[0xAA, 0x01, 0x5E]).is_none());
    }

    #[test]
    fn heartbeat_payload_equals_sync() {
        // Status byte 0x55 must not confuse anything
        let frame = HeartbeatFrame { status: 0x55 };
        let decoded = HeartbeatFrame::decode(&frame.encode()).unwrap();
        assert_eq!(decoded.status, 0x55);
    }

    // -- Frame decoder state machine --

    #[test]
    fn decoder_state_frame() {
        let mut dec = FrameDecoder::new();
        let wire = StateFrame::new(0x42).encode();
        assert_eq!(dec.feed(wire[0]), DecoderEvent::None);
        assert_eq!(dec.feed(wire[1]), DecoderEvent::None);
        assert_eq!(
            dec.feed(wire[2]),
            DecoderEvent::State(StateFrame::new(0x42))
        );
    }

    #[test]
    fn decoder_heartbeat_frame() {
        let mut dec = FrameDecoder::new();
        let wire = HeartbeatFrame::new(true, false).encode();
        assert_eq!(dec.feed(wire[0]), DecoderEvent::None);
        assert_eq!(dec.feed(wire[1]), DecoderEvent::None);
        assert_eq!(
            dec.feed(wire[2]),
            DecoderEvent::Heartbeat(HeartbeatFrame::new(true, false))
        );
    }

    #[test]
    fn decoder_crc_error() {
        let mut dec = FrameDecoder::new();
        dec.feed(STATE_SYNC);
        dec.feed(0x42);
        assert_eq!(dec.feed(0xFF), DecoderEvent::CrcError);
    }

    #[test]
    fn decoder_garbage_before_sync() {
        let mut dec = FrameDecoder::new();
        for &garbage in &[0x00, 0x01, 0xFF, 0x42] {
            assert_eq!(dec.feed(garbage), DecoderEvent::None);
        }
        let wire = StateFrame::new(0x0F).encode();
        assert_eq!(dec.feed(wire[0]), DecoderEvent::None);
        assert_eq!(dec.feed(wire[1]), DecoderEvent::None);
        assert_eq!(
            dec.feed(wire[2]),
            DecoderEvent::State(StateFrame::new(0x0F))
        );
    }

    #[test]
    fn decoder_resyncs_after_crc_error() {
        let mut dec = FrameDecoder::new();
        // Bad frame
        dec.feed(STATE_SYNC);
        dec.feed(0x42);
        assert_eq!(dec.feed(0x00), DecoderEvent::CrcError); // bad CRC
                                                            // Good frame immediately after
        let wire = StateFrame::new(0xBB).encode();
        assert_eq!(dec.feed(wire[0]), DecoderEvent::None);
        assert_eq!(dec.feed(wire[1]), DecoderEvent::None);
        assert_eq!(
            dec.feed(wire[2]),
            DecoderEvent::State(StateFrame::new(0xBB))
        );
    }

    #[test]
    fn decoder_back_to_back_frames() {
        let mut dec = FrameDecoder::new();
        let w1 = StateFrame::new(0x01).encode();
        let w2 = StateFrame::new(0x80).encode();
        for &b in &w1 {
            dec.feed(b);
        }
        assert_eq!(dec.feed(w2[0]), DecoderEvent::None);
        assert_eq!(dec.feed(w2[1]), DecoderEvent::None);
        assert_eq!(dec.feed(w2[2]), DecoderEvent::State(StateFrame::new(0x80)));
    }

    #[test]
    fn decoder_mixed_frame_types() {
        let mut dec = FrameDecoder::new();
        let state_wire = StateFrame::new(0xAB).encode();
        let hb_wire = HeartbeatFrame::new(true, true).encode();

        for &b in &state_wire {
            dec.feed(b);
        }
        assert_eq!(dec.feed(hb_wire[0]), DecoderEvent::None);
        assert_eq!(dec.feed(hb_wire[1]), DecoderEvent::None);
        assert_eq!(
            dec.feed(hb_wire[2]),
            DecoderEvent::Heartbeat(HeartbeatFrame::new(true, true))
        );
    }
}
