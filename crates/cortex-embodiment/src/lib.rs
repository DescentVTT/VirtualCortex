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

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::Ordering;

    #[test]
    fn control_block_is_one_cache_line() {
        assert_eq!(core::mem::size_of::<EmbodimentRingBuffer>(), 64);
        assert_eq!(core::mem::align_of::<EmbodimentRingBuffer>(), 64);
    }

    #[test]
    fn new_and_default_are_all_zero() {
        for b in [EmbodimentRingBuffer::new(), EmbodimentRingBuffer::default()] {
            assert_eq!(b.write_cursor.load(Ordering::Relaxed), 0);
            assert_eq!(b.read_cursor.load(Ordering::Relaxed), 0);
            assert_eq!(b.epoch_id.load(Ordering::Relaxed), 0);
            assert_eq!(b.heartbeat_ms.load(Ordering::Relaxed), 0);
            assert_eq!(b.reserved, [0; 32]);
        }
    }

    #[test]
    fn cursors_are_independent_atomics_with_release_acquire() {
        let b = EmbodimentRingBuffer::new();
        b.write_cursor.store(5, Ordering::Release);
        b.heartbeat_ms.fetch_add(1, Ordering::AcqRel);
        assert_eq!(b.write_cursor.load(Ordering::Acquire), 5);
        assert_eq!(b.read_cursor.load(Ordering::Acquire), 0);
        assert_eq!(b.heartbeat_ms.load(Ordering::Acquire), 1);
    }
}
