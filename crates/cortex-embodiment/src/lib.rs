//! Embodiment: the frame ABI and the ring protocol between the engine and a plant.
//!
//! The frame ABI and the single-producer single-consumer ring protocol that couple the engine to
//! a physics engine or robot controller over shared memory (whitepaper §5.2.4, §6.4, ADR-0015).
//!
//! This crate defines the records and the cursor protocol only. The shared-memory mapping, the
//! frame storage it contains and the 1 ms loop are the runtime's; the protocol here is index-only
//! and contains no `unsafe`, so it can be exercised over any storage a caller provides. The
//! two-thread exchange test lives in `tests/spsc.rs`.

#![no_std]
use core::sync::atomic::{AtomicU64, Ordering};

/// Degrees of freedom carried by one frame. Unused entries are zero.
pub const DOF: usize = 12;

/// Frames per ring. A power of two; sixteen frames is 16 ms of backpressure at the 1 ms period,
/// three times the watchdog's five-period window.
pub const CAPACITY: u64 = 16;

/// Version of the frame ABI (the two frame records, the control block and this protocol).
/// Bumped on any change to any of them.
pub const FRAME_ABI_VERSION: u32 = 1;

const INDEX_MASK: u64 = CAPACITY - 1;

/// One period's motor command, engine → plant. 64 B, align 64.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct TorqueFrame {
    pub epoch: u64,              // [0..8] Simulation epoch this command belongs to
    pub torques_q16: [i32; DOF], // [8..56] Joint torques, Q16.16, in joint order
    pub _reserved: [u8; 8],      // [56..64] Reserved; MUST be zero
}

impl TorqueFrame {
    /// Decodes one period's layer-5 activity into torques by a push-pull rate code (whitepaper
    /// section 6.4 step 2): for each joint, `(agonist - antagonist) x gain_q16`, widened to `i64`
    /// and clamped to the `i32` range, so an agonist and its antagonist cancel, an empty period
    /// is the zero frame, and a runaway count saturates instead of wrapping. `gain_q16` is the
    /// torque one net burst is worth. Population-vector and learned decoders are Specified.
    pub fn from_burst_counts(
        epoch: u64,
        agonist: &[u32; DOF],
        antagonist: &[u32; DOF],
        gain_q16: i32,
    ) -> Self {
        let mut torques_q16 = [0i32; DOF];
        for (j, torque) in torques_q16.iter_mut().enumerate() {
            let net = agonist[j] as i64 - antagonist[j] as i64;
            *torque = (net * gain_q16 as i64).clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        }
        Self {
            epoch,
            torques_q16,
            _reserved: [0; 8],
        }
    }
}

/// One period's observation, plant → engine. 64 B, align 64. Velocities are not carried: at a
/// fixed period they are the finite difference of consecutive positions (ADR-0015).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct JointStateFrame {
    pub epoch: u64,                // [0..8] Plant epoch this observation was taken at
    pub positions_q16: [i32; DOF], // [8..56] Joint positions, Q16.16, in joint order
    pub _reserved: [u8; 8],        // [56..64] Reserved; MUST be zero
}

/// The control block of one ring: two cursors, the last published epoch, the producer's
/// heartbeat, and the ABI parameters the consumer checks before trusting the mapping.
///
/// Cursors are monotonic 64-bit counters; the slot of a cursor value is `cursor & (CAPACITY - 1)`.
/// The ring is empty when the cursors are equal and full when they differ by `CAPACITY`.
///
/// Ordering: the producer writes a frame's bytes, then release-stores the write cursor; the
/// consumer acquire-loads the write cursor, then reads the frame. The release/acquire pair on the
/// cursor orders the plain frame writes before the plain frame reads, so no other synchronisation
/// is needed on the payload (whitepaper §8.5). The consumer releases a slot by release-storing
/// the read cursor after its last read of the frame; the producer acquire-loads it before reusing
/// the slot.
// Control record (whitepaper §8.2, rule L-5): holds atomics, so it is Sync but not Copy.
#[derive(Debug)]
#[repr(C, align(64))]
pub struct EmbodimentRingBuffer {
    pub write_cursor: AtomicU64, // [0..8] Frames published by the producer
    pub read_cursor: AtomicU64,  // [8..16] Frames released by the consumer
    pub epoch_id: AtomicU64,     // [16..24] Epoch of the last published frame
    pub heartbeat_ms: AtomicU64, // [24..32] Producer's monotonic clock, ms, at the last publish
    pub abi_version: u32,        // [32..36] FRAME_ABI_VERSION; written once at initialisation
    pub capacity: u32,           // [36..40] CAPACITY; written once at initialisation
    pub reserved: [u8; 24],      // [40..64] Reserved; MUST be zero
}

impl EmbodimentRingBuffer {
    /// An empty ring at epoch 0, carrying the ABI version and capacity.
    pub const fn new() -> Self {
        Self {
            write_cursor: AtomicU64::new(0),
            read_cursor: AtomicU64::new(0),
            epoch_id: AtomicU64::new(0),
            heartbeat_ms: AtomicU64::new(0),
            abi_version: FRAME_ABI_VERSION,
            capacity: CAPACITY as u32,
            reserved: [0; 24],
        }
    }

    /// True when the control block was initialised by this ABI version with this capacity; a
    /// consumer MUST check it before reading any frame.
    #[inline]
    pub fn is_compatible(&self) -> bool {
        self.abi_version == FRAME_ABI_VERSION && self.capacity == CAPACITY as u32
    }

    /// Frames published and not yet released.
    #[inline]
    pub fn len(&self) -> u64 {
        self.write_cursor.load(Ordering::Acquire) - self.read_cursor.load(Ordering::Acquire)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() >= CAPACITY
    }

    /// Producer: the slot to write the next frame into, or `None` when the ring is full. The
    /// producer owns the write cursor, so it reads it relaxed; the read cursor is acquire-loaded
    /// so that the consumer's reads of the slot being reused happen-before the producer's writes.
    #[inline]
    pub fn producer_claim(&self) -> Option<usize> {
        let w = self.write_cursor.load(Ordering::Relaxed);
        let r = self.read_cursor.load(Ordering::Acquire);
        if w - r >= CAPACITY {
            None
        } else {
            Some((w & INDEX_MASK) as usize)
        }
    }

    /// Producer: publishes the frame written into the claimed slot. `epoch` is the frame's
    /// epoch; `now_ms` is the producer's monotonic clock in milliseconds, which the watchdog
    /// compares with its own (whitepaper §8.9). The release-store of the write cursor is what
    /// makes the frame visible; the epoch and heartbeat are stored before it so that a consumer
    /// that sees the new cursor also sees them.
    #[inline]
    pub fn producer_publish(&self, epoch: u64, now_ms: u64) {
        self.epoch_id.store(epoch, Ordering::Relaxed);
        self.heartbeat_ms.store(now_ms, Ordering::Relaxed);
        let w = self.write_cursor.load(Ordering::Relaxed);
        self.write_cursor.store(w + 1, Ordering::Release);
    }

    /// Consumer: the slot of the oldest unreleased frame, or `None` when the ring is empty. The
    /// consumer owns the read cursor, so it reads it relaxed; the write cursor is acquire-loaded
    /// so that the producer's writes of that frame happen-before the consumer's reads.
    #[inline]
    pub fn consumer_peek(&self) -> Option<usize> {
        let r = self.read_cursor.load(Ordering::Relaxed);
        let w = self.write_cursor.load(Ordering::Acquire);
        if r == w {
            None
        } else {
            Some((r & INDEX_MASK) as usize)
        }
    }

    /// Consumer: releases the slot returned by the last `consumer_peek` after its last read of
    /// the frame. The release-store lets the producer reuse the slot.
    #[inline]
    pub fn consumer_release(&self) {
        let r = self.read_cursor.load(Ordering::Relaxed);
        self.read_cursor.store(r + 1, Ordering::Release);
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
    assert!(core::mem::size_of::<TorqueFrame>() == 64);
    assert!(core::mem::align_of::<TorqueFrame>() == 64);
    assert!(core::mem::size_of::<JointStateFrame>() == 64);
    assert!(core::mem::align_of::<JointStateFrame>() == 64);
    assert!(CAPACITY.is_power_of_two());
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_are_one_cache_line() {
        assert_eq!(core::mem::size_of::<EmbodimentRingBuffer>(), 64);
        assert_eq!(core::mem::align_of::<EmbodimentRingBuffer>(), 64);
        assert_eq!(core::mem::size_of::<TorqueFrame>(), 64);
        assert_eq!(core::mem::align_of::<TorqueFrame>(), 64);
        assert_eq!(core::mem::size_of::<JointStateFrame>(), 64);
        assert_eq!(core::mem::align_of::<JointStateFrame>(), 64);
    }

    #[test]
    fn new_and_default_carry_the_abi_and_zero_cursors() {
        for b in [EmbodimentRingBuffer::new(), EmbodimentRingBuffer::default()] {
            assert_eq!(b.write_cursor.load(Ordering::Relaxed), 0);
            assert_eq!(b.read_cursor.load(Ordering::Relaxed), 0);
            assert_eq!(b.epoch_id.load(Ordering::Relaxed), 0);
            assert_eq!(b.heartbeat_ms.load(Ordering::Relaxed), 0);
            assert_eq!(b.abi_version, FRAME_ABI_VERSION);
            assert_eq!(b.capacity, 16);
            assert_eq!(b.reserved, [0; 24]);
            assert!(b.is_compatible());
            assert!(b.is_empty());
        }
        let mut foreign = EmbodimentRingBuffer::new();
        foreign.abi_version = 2;
        assert!(!foreign.is_compatible());
    }

    #[test]
    fn default_frames_are_all_zero() {
        assert_eq!(TorqueFrame::default().torques_q16, [0; DOF]);
        assert_eq!(JointStateFrame::default().positions_q16, [0; DOF]);
        assert_eq!(TorqueFrame::default()._reserved, [0; 8]);
    }

    #[test]
    fn an_empty_period_decodes_to_the_zero_frame_with_its_epoch() {
        let f = TorqueFrame::from_burst_counts(9, &[0; DOF], &[0; DOF], 0x0001_0000);
        assert_eq!(f.epoch, 9);
        assert_eq!(f.torques_q16, [0; DOF]);
        assert_eq!(f._reserved, [0; 8]);
    }

    #[test]
    fn balanced_bursts_cancel_and_the_sign_follows_the_dominant_side() {
        let mut agonist = [3u32; DOF];
        let mut antagonist = [3u32; DOF];
        agonist[0] = 5;
        antagonist[1] = 7;
        let f = TorqueFrame::from_burst_counts(1, &agonist, &antagonist, 0x0000_8000);
        assert_eq!(
            f.torques_q16[0],
            2 * 0x0000_8000,
            "two net agonist bursts at 0.5 each"
        );
        assert_eq!(
            f.torques_q16[1],
            -4 * 0x0000_8000,
            "four net antagonist bursts"
        );
        assert!(
            f.torques_q16[2..].iter().all(|&t| t == 0),
            "balanced joints are still"
        );
    }

    #[test]
    fn the_gain_scales_linearly_and_the_extremes_clamp() {
        let one = [1u32; DOF];
        let zero = [0u32; DOF];
        let a = TorqueFrame::from_burst_counts(0, &one, &zero, 0x0000_1000);
        let b = TorqueFrame::from_burst_counts(0, &one, &zero, 0x0000_2000);
        assert_eq!(b.torques_q16[0], 2 * a.torques_q16[0]);
        let hi = TorqueFrame::from_burst_counts(0, &[u32::MAX; DOF], &zero, i32::MAX);
        assert_eq!(hi.torques_q16, [i32::MAX; DOF], "clamps rather than wraps");
        let lo = TorqueFrame::from_burst_counts(0, &zero, &[u32::MAX; DOF], i32::MAX);
        assert_eq!(lo.torques_q16, [i32::MIN; DOF]);
        let neg = TorqueFrame::from_burst_counts(0, &one, &zero, -0x0001_0000);
        assert_eq!(
            neg.torques_q16[0], -0x0001_0000,
            "a negative gain inverts the joint"
        );
    }

    #[test]
    fn two_frames_from_the_same_counts_are_equal() {
        let agonist: [u32; DOF] = core::array::from_fn(|j| (j * 7 % 5) as u32);
        let antagonist: [u32; DOF] = core::array::from_fn(|j| (j * 3 % 4) as u32);
        let a = TorqueFrame::from_burst_counts(4, &agonist, &antagonist, 0x0000_C000);
        let b = TorqueFrame::from_burst_counts(4, &agonist, &antagonist, 0x0000_C000);
        assert_eq!(a, b);
    }

    #[test]
    fn empty_ring_has_nothing_to_read_and_a_slot_to_write() {
        let b = EmbodimentRingBuffer::new();
        assert_eq!(b.consumer_peek(), None);
        assert_eq!(b.producer_claim(), Some(0));
    }

    #[test]
    fn a_full_ring_refuses_the_producer_until_the_consumer_releases() {
        let b = EmbodimentRingBuffer::new();
        for i in 0..16u64 {
            assert_eq!(b.producer_claim(), Some(i as usize));
            b.producer_publish(i, i);
        }
        assert!(b.is_full());
        assert_eq!(b.len(), 16);
        assert_eq!(b.producer_claim(), None);
        assert_eq!(b.consumer_peek(), Some(0));
        b.consumer_release();
        assert_eq!(b.producer_claim(), Some(0), "the released slot is reused");
        assert_eq!(b.len(), 15);
    }

    #[test]
    fn slots_wrap_at_capacity_and_cursors_keep_counting() {
        let b = EmbodimentRingBuffer::new();
        for i in 0..40u64 {
            assert_eq!(b.producer_claim(), Some((i % 16) as usize), "frame {i}");
            b.producer_publish(i, i);
            assert_eq!(b.consumer_peek(), Some((i % 16) as usize));
            b.consumer_release();
        }
        assert_eq!(b.write_cursor.load(Ordering::Relaxed), 40);
        assert_eq!(b.read_cursor.load(Ordering::Relaxed), 40);
        assert!(b.is_empty());
    }

    #[test]
    fn epoch_and_heartbeat_follow_the_last_publish() {
        let b = EmbodimentRingBuffer::new();
        let mut last = 0;
        for (epoch, now) in [(1u64, 100u64), (2, 101), (3, 103)] {
            b.producer_claim().unwrap();
            b.producer_publish(epoch, now);
            assert_eq!(b.epoch_id.load(Ordering::Acquire), epoch);
            let hb = b.heartbeat_ms.load(Ordering::Acquire);
            assert_eq!(hb, now);
            assert!(hb >= last, "heartbeat is monotonic");
            last = hb;
        }
    }
}
