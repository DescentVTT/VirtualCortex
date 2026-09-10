//! Two-tier flat timing wheel: O(1) scheduling of delayed spike delivery without allocation
//! (ADR-0004, geometry and slot representation decided by ADR-0013).
//!
//! Time is counted in fine ticks (10 µs). The fine ring has [`FINE_SLOTS`] slots, one per
//! tick; the coarse ring has [`COARSE_SLOTS`] slots, one per [`FINE_PER_COARSE`] ticks. A
//! delay below the fine ring's length lands directly in the fine ring; a longer delay lands in
//! the coarse ring with its residual fine offset packed into the token, and is cascaded into
//! the fine ring when its coarse window begins. Both ring lengths are powers of two, so slot
//! selection is a mask. Each slot is a fixed-capacity list of opaque 28-bit tokens (a
//! `SynapseBlock` offset or a unit index; the wheel does not interpret them). Delivery order is
//! deterministic: the tokens already in a fine slot come first, then the tokens cascaded into it,
//! each group in scheduling order; two wheels fed the same sequence produce identical slots.

/// Fine slots: 256 × 10 µs = 2.56 ms.
pub const FINE_SLOTS: usize = 256;
/// Coarse slots: 256 × 100 µs = 25.6 ms.
pub const COARSE_SLOTS: usize = 256;
/// Fine ticks per coarse slot.
pub const FINE_PER_COARSE: u64 = 10;
/// Largest token: 28 bits, leaving 4 bits for the fine residual in the coarse ring.
pub const MAX_TOKEN: u32 = (1 << 28) - 1;

const FINE_MASK: u64 = FINE_SLOTS as u64 - 1;
const COARSE_MASK: u64 = COARSE_SLOTS as u64 - 1;
const RESIDUAL_SHIFT: u32 = 28;

/// Why a delay could not be scheduled. None of these mutate the wheel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScheduleError {
    /// A zero delay is delivered through the mailbox directly (whitepaper §6.1, R-1), never
    /// through the wheel.
    ZeroDelay,
    /// The delay is at or beyond [`FlatTimingWheel::horizon_ticks`].
    BeyondHorizon,
    /// The target slot already holds `CAP` tokens; the worker's wheel is undersized.
    SlotFull,
    /// The token exceeds [`MAX_TOKEN`].
    TokenTooLarge,
}

/// The per-worker wheel. `CAP` is the number of tokens one slot can hold.
// Not a record and deliberately not Copy: a multi-megabyte value per worker.
#[derive(Debug)]
pub struct FlatTimingWheel<const CAP: usize> {
    tick: u64,
    fine: [[u32; CAP]; FINE_SLOTS],
    fine_len: [u16; FINE_SLOTS],
    coarse: [[u32; CAP]; COARSE_SLOTS],
    coarse_len: [u16; COARSE_SLOTS],
}

/// The production geometry: 2 048 tokens per slot, 4 195 336 bytes per worker (ADR-0013).
pub type WorkerWheel = FlatTimingWheel<2048>;

impl<const CAP: usize> FlatTimingWheel<CAP> {
    /// A wheel at tick 0 with every slot empty.
    pub const fn new() -> Self {
        assert!(CAP > 0 && CAP <= u16::MAX as usize);
        Self {
            tick: 0,
            fine: [[0; CAP]; FINE_SLOTS],
            fine_len: [0; FINE_SLOTS],
            coarse: [[0; CAP]; COARSE_SLOTS],
            coarse_len: [0; COARSE_SLOTS],
        }
    }

    /// Delays at or beyond this many fine ticks cannot be scheduled (2 560 ticks, 25.6 ms).
    pub const fn horizon_ticks() -> u64 {
        COARSE_SLOTS as u64 * FINE_PER_COARSE
    }

    /// The current tick; the slot returned by the last [`advance`](Self::advance) is due now.
    pub const fn tick(&self) -> u64 {
        self.tick
    }

    /// Schedules `token` for delivery on the `delay_ticks`-th subsequent
    /// [`advance`](Self::advance). O(1); never allocates; on error the wheel is unchanged.
    #[inline]
    pub fn schedule(&mut self, delay_ticks: u32, token: u32) -> Result<(), ScheduleError> {
        if delay_ticks == 0 {
            return Err(ScheduleError::ZeroDelay);
        }
        if delay_ticks as u64 >= Self::horizon_ticks() {
            return Err(ScheduleError::BeyondHorizon);
        }
        if token > MAX_TOKEN {
            return Err(ScheduleError::TokenTooLarge);
        }
        let due = self.tick + delay_ticks as u64;
        if (delay_ticks as usize) < FINE_SLOTS {
            let slot = (due & FINE_MASK) as usize;
            Self::push(&mut self.fine[slot], &mut self.fine_len[slot], token)
        } else {
            let slot = ((due / FINE_PER_COARSE) & COARSE_MASK) as usize;
            let residual = (due % FINE_PER_COARSE) as u32;
            Self::push(
                &mut self.coarse[slot],
                &mut self.coarse_len[slot],
                token | (residual << RESIDUAL_SHIFT),
            )
        }
    }

    /// Moves to the next tick and returns the tokens due at it: those scheduled directly into
    /// the fine slot first, then those cascaded from the coarse ring, each group in scheduling
    /// order. The slice is valid until the next call. The slot consumed by the previous call is cleared first;
    /// at every coarse boundary the next coarse slot is cascaded into the fine ring before the
    /// due slot is returned. O(1) plus the length of the cascaded slot.
    #[inline]
    pub fn advance(&mut self) -> &[u32] {
        let consumed = (self.tick & FINE_MASK) as usize;
        self.fine_len[consumed] = 0;
        self.tick += 1;
        // `%` rather than `is_multiple_of` (Rust 1.87+): the MSRV is 1.85 (ADR-0009). Clippy
        // reads `rust-version` from the manifest and does not suggest the newer method.
        if self.tick % FINE_PER_COARSE == 0 {
            self.cascade();
        }
        let due = (self.tick & FINE_MASK) as usize;
        &self.fine[due][..self.fine_len[due] as usize]
    }

    /// Moves every token of the coarse slot whose window begins now into the fine slot of
    /// its exact due tick. The fine slots it writes are the current one and the next nine,
    /// none of which is the consumed slot, so nothing is lost or delivered late.
    fn cascade(&mut self) {
        let window = ((self.tick / FINE_PER_COARSE) & COARSE_MASK) as usize;
        let n = self.coarse_len[window] as usize;
        for i in 0..n {
            let packed = self.coarse[window][i];
            let residual = (packed >> RESIDUAL_SHIFT) as u64;
            let token = packed & MAX_TOKEN;
            let slot = ((self.tick + residual) & FINE_MASK) as usize;
            // A full fine slot here is the same capacity fault as at schedule time; the coarse
            // entry was accepted, so the token is dropped rather than the tick loop aborted.
            // The runtime sizes CAP so that this cannot happen (whitepaper §8.9).
            let _ = Self::push(&mut self.fine[slot], &mut self.fine_len[slot], token);
        }
        self.coarse_len[window] = 0;
    }

    #[inline(always)]
    fn push(slot: &mut [u32; CAP], len: &mut u16, token: u32) -> Result<(), ScheduleError> {
        let n = *len as usize;
        if n >= CAP {
            return Err(ScheduleError::SlotFull);
        }
        slot[n] = token;
        *len = (n + 1) as u16;
        Ok(())
    }
}

impl<const CAP: usize> Default for FlatTimingWheel<CAP> {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::size_of::<WorkerWheel>() == 4_195_336);
    assert!(FINE_SLOTS.is_power_of_two());
    assert!(COARSE_SLOTS.is_power_of_two());
    assert!(FINE_PER_COARSE < 16); // the residual must fit in the four bits above MAX_TOKEN
};

#[cfg(test)]
mod tests {
    use super::*;

    type Wheel = FlatTimingWheel<16>;

    /// Advances until `token` is delivered; returns how many advances it took.
    fn advances_until(w: &mut Wheel, token: u32, limit: u64) -> Option<u64> {
        (1..=limit).find(|_| w.advance().contains(&token))
    }

    #[test]
    fn delivered_on_exactly_the_requested_advance() {
        // Both rings, both wrap boundaries, and the extremes of each range.
        for &d in &[1u32, 2, 9, 10, 11, 255, 256, 257, 2559] {
            let mut w = Wheel::new();
            w.schedule(d, 7).unwrap();
            assert_eq!(advances_until(&mut w, 7, 3000), Some(d as u64), "delay {d}");
        }
    }

    #[test]
    fn delivery_is_exact_from_any_starting_tick() {
        for start in [0u64, 250, 255, 256, 2555, 2559, 2560, 5119] {
            for &d in &[1u32, 255, 256, 1000, 2559] {
                let mut w = Wheel::new();
                for _ in 0..start {
                    w.advance();
                }
                w.schedule(d, 42).unwrap();
                assert_eq!(
                    advances_until(&mut w, 42, 3000),
                    Some(d as u64),
                    "start {start} delay {d}"
                );
            }
        }
    }

    #[test]
    fn a_coarse_event_is_never_delivered_early() {
        let mut w = Wheel::new();
        w.schedule(1000, 5).unwrap();
        for n in 1..1000 {
            assert!(!w.advance().contains(&5), "delivered early at advance {n}");
        }
        assert!(w.advance().contains(&5));
    }

    #[test]
    fn rejections_do_not_mutate_the_wheel() {
        let mut w = Wheel::new();
        assert_eq!(w.schedule(0, 1), Err(ScheduleError::ZeroDelay));
        assert_eq!(w.schedule(2560, 1), Err(ScheduleError::BeyondHorizon));
        assert_eq!(w.schedule(u32::MAX, 1), Err(ScheduleError::BeyondHorizon));
        assert_eq!(
            w.schedule(5, MAX_TOKEN + 1),
            Err(ScheduleError::TokenTooLarge)
        );
        assert_eq!(w.schedule(5, MAX_TOKEN), Ok(()));
        let mut delivered = 0;
        for _ in 0..3000 {
            delivered += w.advance().len();
        }
        assert_eq!(delivered, 1);
    }

    #[test]
    fn a_full_slot_is_reported_and_the_rest_still_arrives() {
        let mut w = FlatTimingWheel::<4>::new();
        for t in 0..4 {
            w.schedule(3, t).unwrap();
        }
        assert_eq!(w.schedule(3, 99), Err(ScheduleError::SlotFull));
        w.advance();
        w.advance();
        assert_eq!(w.advance(), &[0, 1, 2, 3]);
    }

    #[test]
    fn delivery_order_is_deterministic_across_both_rings() {
        // Tokens 1 and 2 go to the coarse ring (due 260, window 26); token 3 is fine-scheduled
        // for the same tick from tick 10. The window is cascaded at tick 260, after token 3 is
        // already in the fine slot, so the order is: fine tokens present before the cascade,
        // then the cascaded tokens, each group in scheduling order.
        let mut w = Wheel::new();
        w.schedule(260, 1).unwrap();
        w.schedule(260, 2).unwrap();
        for _ in 0..10 {
            w.advance();
        }
        w.schedule(250, 3).unwrap();
        for _ in 0..249 {
            w.advance();
        }
        assert_eq!(w.advance(), &[3, 1, 2]);
    }

    #[test]
    fn two_wheels_fed_the_same_sequence_agree() {
        let mut a = FlatTimingWheel::<64>::new();
        let mut b = FlatTimingWheel::<64>::new();
        let mut seed = 0x9E37_79B9u32;
        for step in 0..4000u32 {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let delay = 1 + (seed >> 8) % 2559;
            let token = step & MAX_TOKEN;
            let ra = a.schedule(delay, token);
            let rb = b.schedule(delay, token);
            assert_eq!(ra, rb);
            assert_eq!(a.advance(), b.advance(), "step {step}");
        }
    }

    #[test]
    fn horizon_and_geometry_constants() {
        assert_eq!(Wheel::horizon_ticks(), 2560);
        assert_eq!(FINE_SLOTS, 256);
        assert_eq!(COARSE_SLOTS, 256);
        let w = Wheel::default();
        assert_eq!(w.tick(), 0);
    }
}
