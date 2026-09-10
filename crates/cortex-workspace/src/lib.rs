//! Global workspace: competitive broadcast slots with threshold ignition (whitepaper §5.2.8),
//! and, since ADR-0020, an ignition threshold scaled by the distance from criticality and an
//! attention-schema hash that models what the slot is broadcasting (§8.12). Decay and
//! competition are Specified.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct GlobalWorkspaceSlot {
    pub slot_id: u32,                    // [0..4] Active workspace slot index (0..7)
    pub binding_hash: u32,               // [4..8] Hash of bound cortical concept
    pub ignition_potential: i32,         // [8..12] Non-linear threshold accumulator (Q16.16)
    pub persistence_ticks: u32,          // [12..16] Working memory reverberation counter
    pub confidence_q16: u32, // [16..20] Metacognitive certainty metric (0..1.0 in Q16.16)
    pub broadcast_channel_mask: u32, // [20..24] Target cortical recipient mask
    pub p300_wave_phase: u32, // [24..28] Ignition oscillation phase angle
    pub is_ignited: u32,     // [28..32] 1 if ignited and broadcasting, 0 if subliminal
    pub attention_schema_meta_hash: u32, // [32..36] Hash of the broadcast state: the slot's model of its own attention (ADR-0020)
    pub criticality_distance_q16: u32, // [36..40] |sigma - 1| the last step was gated with (Q16.16, ADR-0020)
    pub _reserved: [u8; 24],           // [40..64] Strict 64-byte cache-line alignment padding
}

impl GlobalWorkspaceSlot {
    pub const IGNITION_THRESHOLD: i32 = 0x0001_8000; // 1.5 in Q16.16
    /// Ticks a slot stays ignited after crossing the threshold.
    pub const PERSISTENCE_TICKS: u32 = 300;

    /// Accumulates evidence and ignites at or above the threshold. The accumulator
    /// saturates (whitepaper §8.1), so sustained evidence can never wrap to a negative
    /// potential and silently un-ignite the slot. Returns whether the slot is ignited after the
    /// step: it crossed the threshold now, or the hold of an earlier crossing has not run out;
    /// the hold is `PERSISTENCE_TICKS` steps and counts down one per sub-threshold step. Decay
    /// and competition are Specified.
    #[inline(always)]
    pub fn step_ignition(&mut self, bottom_up_evidence: i32) -> bool {
        self.criticality_distance_q16 = 0;
        self.ignite_at(bottom_up_evidence, Self::IGNITION_THRESHOLD)
    }

    /// `step_ignition` under the criticality of the tissue (ADR-0020, whitepaper §8.12): with
    /// `sigma_q16` the branching ratio `cortex-homeostasis` measured, the threshold is scaled
    /// by `1 + min(|sigma - 1|, 1)`, so at criticality ignition is as cheap as it can be and a
    /// subcritical or supercritical tissue needs up to twice the evidence. The distance used is
    /// stored. Returns whether the slot ignited.
    pub fn step_ignition_at(&mut self, bottom_up_evidence: i32, sigma_q16: u32) -> bool {
        let distance = sigma_q16.abs_diff(Q16_ONE).min(Q16_ONE);
        self.criticality_distance_q16 = distance;
        // At most twice the threshold, formed in `i64` and clamped on the way back (§8.1).
        let scaled = (Self::IGNITION_THRESHOLD as i64).saturating_add(
            (Self::IGNITION_THRESHOLD as i64).saturating_mul(distance as i64) >> 16,
        );
        self.ignite_at(bottom_up_evidence, scaled.min(i32::MAX as i64) as i32)
    }

    /// The attention schema (ADR-0020): a hash of what the slot broadcasts, recomputed after
    /// every step, so that a second-order consumer (`cortex-attention`) can tell a change of
    /// broadcast from a change of evidence without reading the slot's fields. Returns it.
    pub fn update_attention_schema(&mut self) -> u32 {
        let mut h = 0x811C_9DC5u32;
        for word in [
            self.slot_id,
            self.binding_hash,
            self.is_ignited,
            self.persistence_ticks,
            self.broadcast_channel_mask,
        ] {
            h = (h ^ word).wrapping_mul(0x0100_0193);
        }
        self.attention_schema_meta_hash = h;
        h
    }

    fn ignite_at(&mut self, bottom_up_evidence: i32, threshold: i32) -> bool {
        self.ignition_potential = self.ignition_potential.saturating_add(bottom_up_evidence);
        if self.ignition_potential >= threshold {
            self.is_ignited = 1;
            self.persistence_ticks = Self::PERSISTENCE_TICKS;
        } else if self.persistence_ticks > 0 {
            // Within the hold of an earlier crossing: the slot stays ignited and the window
            // counts down one tick per step.
            self.persistence_ticks = self.persistence_ticks.saturating_sub(1);
            self.is_ignited = 1;
        } else {
            self.is_ignited = 0;
        }
        self.update_attention_schema();
        self.is_ignited == 1
    }
}

const _: () = {
    assert!(core::mem::size_of::<GlobalWorkspaceSlot>() == 64);
    assert!(core::mem::align_of::<GlobalWorkspaceSlot>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_attention_schema_hash_is_pinned_and_sees_every_word() {
        let mut s = slot(0);
        s.slot_id = 1;
        s.binding_hash = 2;
        assert_eq!(s.update_attention_schema(), 0x6bc2_066e);
        assert_eq!(s.attention_schema_meta_hash, 0x6bc2_066e);
        s.is_ignited = 1;
        assert_eq!(
            s.update_attention_schema(),
            0xaaa8_b9b9,
            "the ignition bit changes the hash"
        );
        let mut previous = s.attention_schema_meta_hash;
        s.slot_id = 2;
        assert_ne!(
            s.update_attention_schema(),
            previous,
            "the slot id is folded"
        );
        previous = s.attention_schema_meta_hash;
        s.binding_hash = 3;
        assert_ne!(
            s.update_attention_schema(),
            previous,
            "the binding is folded"
        );
        previous = s.attention_schema_meta_hash;
        s.persistence_ticks = 4;
        assert_ne!(
            s.update_attention_schema(),
            previous,
            "the persistence is folded"
        );
        previous = s.attention_schema_meta_hash;
        s.broadcast_channel_mask = 5;
        assert_ne!(
            s.update_attention_schema(),
            previous,
            "the channel mask is folded"
        );
        previous = s.attention_schema_meta_hash;
        s.ignition_potential = 6;
        assert_eq!(
            s.update_attention_schema(),
            previous,
            "the potential is not a word of the schema"
        );
    }

    const ONE: i32 = 0x0001_0000;
    const HALF: i32 = 0x0000_8000;

    fn slot(ignition_potential: i32) -> GlobalWorkspaceSlot {
        GlobalWorkspaceSlot {
            slot_id: 0,
            binding_hash: 0,
            ignition_potential,
            persistence_ticks: 0,
            confidence_q16: 0,
            broadcast_channel_mask: 0,
            p300_wave_phase: 0,
            is_ignited: 0,
            attention_schema_meta_hash: 0,
            criticality_distance_q16: 0,
            _reserved: [0; 24],
        }
    }

    #[test]
    fn record_is_one_cache_line() {
        assert_eq!(core::mem::size_of::<GlobalWorkspaceSlot>(), 64);
        assert_eq!(core::mem::align_of::<GlobalWorkspaceSlot>(), 64);
    }

    #[test]
    fn ignites_at_the_threshold_and_holds_for_the_persistence_window() {
        let mut s = slot(0);
        assert!(!s.step_ignition(ONE));
        assert!(s.step_ignition(HALF), "1.0 + 0.5 reaches 1.5");
        assert_eq!(s.is_ignited, 1);
        assert_eq!(s.persistence_ticks, GlobalWorkspaceSlot::PERSISTENCE_TICKS);
        assert_eq!(
            s.ignition_potential,
            GlobalWorkspaceSlot::IGNITION_THRESHOLD
        );
    }

    #[test]
    fn sustained_evidence_saturates_instead_of_wrapping_to_a_negative_potential() {
        let mut s = slot(i32::MAX - 1);
        assert!(s.step_ignition(ONE));
        assert_eq!(s.ignition_potential, i32::MAX);
        assert!(s.step_ignition(ONE), "still ignited, not wrapped negative");
        let mut n = slot(i32::MIN + 1);
        assert!(!n.step_ignition(-ONE));
        assert_eq!(n.ignition_potential, i32::MIN);
    }

    #[test]
    fn evidence_below_the_threshold_leaves_the_slot_subliminal() {
        let mut s = slot(ONE);
        assert!(!s.step_ignition(-HALF));
        assert_eq!(s.is_ignited, 0);
        assert_eq!(s.persistence_ticks, 0);
    }

    #[test]
    fn an_ignited_slot_is_held_for_the_persistence_window_and_then_released() {
        let mut s = slot(ONE);
        assert!(s.step_ignition(HALF));
        for i in 1..=GlobalWorkspaceSlot::PERSISTENCE_TICKS {
            assert!(s.step_ignition(-1), "held at step {i}");
            assert_eq!(s.is_ignited, 1);
            assert_eq!(
                s.persistence_ticks,
                GlobalWorkspaceSlot::PERSISTENCE_TICKS - i
            );
        }
        assert!(!s.step_ignition(-1), "the hold has run out");
        assert_eq!((s.is_ignited, s.persistence_ticks), (0, 0));
        assert!(s.step_ignition(2 * ONE), "and a new crossing reloads it");
        assert_eq!(s.persistence_ticks, GlobalWorkspaceSlot::PERSISTENCE_TICKS);
    }

    #[test]
    fn at_criticality_the_gated_step_is_the_plain_step_and_far_from_it_costs_twice() {
        let mut a = slot(ONE);
        let mut b = slot(ONE);
        assert_eq!(a.step_ignition(HALF), b.step_ignition_at(HALF, Q16_ONE));
        assert_eq!(b.criticality_distance_q16, 0);
        let mut far = slot(ONE);
        assert!(
            !far.step_ignition_at(HALF, 2 * Q16_ONE),
            "1.5 is not enough at sigma = 2"
        );
        assert_eq!(
            far.criticality_distance_q16, Q16_ONE,
            "the distance is capped at 1.0"
        );
        assert!(
            far.step_ignition_at(ONE + HALF, 2 * Q16_ONE),
            "3.0 is: the threshold doubled"
        );
        let mut sub = slot(ONE);
        assert!(
            !sub.step_ignition_at(HALF, Q16_ONE / 2),
            "subcritical tissue is as hard as supercritical"
        );
        assert_eq!(sub.criticality_distance_q16, Q16_ONE / 2);
        let mut extreme = slot(0);
        assert!(
            !extreme.step_ignition_at(0, u32::MAX),
            "a huge sigma clamps the distance rather than wrapping"
        );
        assert_eq!(extreme.criticality_distance_q16, Q16_ONE);
    }

    #[test]
    fn the_attention_schema_changes_with_the_broadcast_and_is_stable_without_it() {
        let mut s = slot(0);
        s.binding_hash = 0xBEEF;
        let quiet = s.update_attention_schema();
        assert_eq!(
            s.update_attention_schema(),
            quiet,
            "deterministic and stable"
        );
        assert!(!s.step_ignition(0));
        assert_eq!(
            s.attention_schema_meta_hash, quiet,
            "a step that changes no broadcast keeps the schema"
        );
        assert!(s.step_ignition(2 * ONE));
        let lit = s.attention_schema_meta_hash;
        assert_ne!(lit, quiet, "ignition is a change of broadcast");
        s.binding_hash = 0xCAFE;
        assert_ne!(s.update_attention_schema(), lit, "so is a new binding");
        let mut other = slot(0);
        other.binding_hash = 0xBEEF;
        assert_eq!(
            other.update_attention_schema(),
            quiet,
            "the same broadcast state hashes the same"
        );
    }
}
