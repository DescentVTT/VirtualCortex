//! Sub-Millisecond Closed-Loop Physical Embodiment Bridge

#![no_std]
use core::sync::atomic::AtomicU64;

// Control record (whitepaper §8.2, rule L-5): holds atomics, so it is Sync but not Copy.
#[derive(Debug)]
#[repr(C, align(64))]
pub struct EmbodimentRingBuffer {
    pub write_cursor: AtomicU64,
    pub read_cursor: AtomicU64,
    pub epoch_id: AtomicU64,
    pub heartbeat_ms: AtomicU64,
    pub reserved: [u8; 32],
}

impl EmbodimentRingBuffer {
    pub const fn new() -> Self {
        Self {
            write_cursor: AtomicU64::new(0),
            read_cursor: AtomicU64::new(0),
            epoch_id: AtomicU64::new(0),
            heartbeat_ms: AtomicU64::new(0),
            reserved: [0; 32],
        }
    }
}

impl Default for EmbodimentRingBuffer {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::size_of::<EmbodimentRingBuffer>() == 64);
    assert!(core::mem::align_of::<EmbodimentRingBuffer>() == 64);
};
