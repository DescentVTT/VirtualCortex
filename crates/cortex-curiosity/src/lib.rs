//! Epistemic exploration vectors: per-target novelty, uncertainty and the intrinsic urgency
//! they produce (whitepaper §5.2.26, §8.8; admitted by ADR-0016).
//!
//! `cortex-homeostasis` holds the one scalar `curiosity_drive` of the whole organism; this
//! crate holds one record per candidate target, so that the drive can be pointed at something.
//! The visit rule is Implemented; how urgency competes in `cortex-basal-ganglia` is Specified.

#![no_std]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// Novelty loses 2^-NOVELTY_DECAY_SHIFT of itself per visit.
pub const NOVELTY_DECAY_SHIFT: u32 = 2;

/// 64-byte exploration vector (whitepaper §5.2.26).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct CuriosityExplorationVector {
    pub target_state_hash: u32,       // [0..4] The state this vector points at
    pub novelty_magnitude_q16: u32, // [4..8] 1.0 before the first visit, decaying with each (Q16.16)
    pub epistemic_entropy_q16: u32, // [8..12] Uncertainty of the prediction about the target (Q16.16)
    pub exploration_urgency_q16: u32, // [12..16] Intrinsic drive toward the target (Q16.16)
    pub visited_count: u32,         // [16..20] Visits so far
    pub prediction_error_q16: u32, // [20..24] Error of the last prediction about the target (Q16.16)
    pub _reserved: [u8; 40],       // [24..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for CuriosityExplorationVector {
    fn default() -> Self {
        Self {
            target_state_hash: 0,
            novelty_magnitude_q16: 0,
            epistemic_entropy_q16: 0,
            exploration_urgency_q16: 0,
            visited_count: 0,
            prediction_error_q16: 0,
            _reserved: [0; 40],
        }
    }
}

impl CuriosityExplorationVector {
    /// A vector for an unvisited target: full novelty, no urgency yet.
    pub const fn new(target_state_hash: u32) -> Self {
        Self {
            target_state_hash,
            novelty_magnitude_q16: Q16_ONE,
            epistemic_entropy_q16: 0,
            exploration_urgency_q16: 0,
            visited_count: 0,
            prediction_error_q16: 0,
            _reserved: [0; 40],
        }
    }

    /// One visit. Novelty loses a quarter of itself, and at least one LSB so that it reaches
    /// zero rather than stalling at three; the visit is counted (saturating), the prediction error and
    /// entropy are stored, and urgency becomes `novelty/2 + error/4 + entropy/4`, which cannot
    /// overflow. Returns the urgency.
    pub fn visit(&mut self, prediction_error_q16: u32, entropy_q16: u32) -> u32 {
        let decay = (self.novelty_magnitude_q16 >> NOVELTY_DECAY_SHIFT).max(1);
        self.novelty_magnitude_q16 = self.novelty_magnitude_q16.saturating_sub(decay);
        self.visited_count = self.visited_count.saturating_add(1);
        self.prediction_error_q16 = prediction_error_q16;
        self.epistemic_entropy_q16 = entropy_q16;
        self.exploration_urgency_q16 = (self.novelty_magnitude_q16 >> 1)
            .saturating_add(prediction_error_q16 >> 2)
            .saturating_add(entropy_q16 >> 2);
        self.exploration_urgency_q16
    }
}

const _: () = {
    assert!(core::mem::size_of::<CuriosityExplorationVector>() == 64);
    assert!(core::mem::align_of::<CuriosityExplorationVector>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_is_one_cache_line_and_default_is_an_exhausted_target() {
        assert_eq!(core::mem::size_of::<CuriosityExplorationVector>(), 64);
        assert_eq!(core::mem::align_of::<CuriosityExplorationVector>(), 64);
        assert_eq!(
            CuriosityExplorationVector::default().novelty_magnitude_q16,
            0
        );
        assert_eq!(
            CuriosityExplorationVector::new(9).novelty_magnitude_q16,
            Q16_ONE
        );
    }

    #[test]
    fn the_first_visit_of_a_predictable_target_is_driven_by_novelty_alone() {
        let mut v = CuriosityExplorationVector::new(1);
        let urgency = v.visit(0, 0);
        assert_eq!(
            v.novelty_magnitude_q16,
            Q16_ONE - (Q16_ONE >> NOVELTY_DECAY_SHIFT)
        );
        assert_eq!(urgency, v.novelty_magnitude_q16 >> 1);
        assert_eq!(v.visited_count, 1);
    }

    #[test]
    fn novelty_decays_monotonically_toward_zero() {
        let mut v = CuriosityExplorationVector::new(1);
        let mut last = v.novelty_magnitude_q16;
        for _ in 0..100 {
            v.visit(0, 0);
            assert!(v.novelty_magnitude_q16 < last || last == 0);
            last = v.novelty_magnitude_q16;
        }
        assert_eq!(last, 0, "a hundred visits exhaust a target");
    }

    #[test]
    fn error_and_entropy_keep_a_visited_target_interesting() {
        let mut v = CuriosityExplorationVector::new(1);
        for _ in 0..100 {
            v.visit(0, 0);
        }
        assert_eq!(
            v.visit(Q16_ONE, Q16_ONE),
            Q16_ONE / 2,
            "error and entropy each add a quarter"
        );
    }

    #[test]
    fn urgency_and_the_visit_count_cannot_overflow() {
        let mut v = CuriosityExplorationVector::new(1);
        v.visited_count = u32::MAX;
        v.novelty_magnitude_q16 = u32::MAX;
        assert_eq!(
            v.visit(u32::MAX, u32::MAX),
            (u32::MAX - (u32::MAX >> 2)) / 2 + 2 * (u32::MAX >> 2)
        );
        assert_eq!(v.visited_count, u32::MAX);
    }
}
