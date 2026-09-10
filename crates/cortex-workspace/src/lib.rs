//! Global workspace: competitive broadcast slots with threshold ignition (whitepaper §5.2.8).
//! Decay and competition are Specified.

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct GlobalWorkspaceSlot {
    pub slot_id: u32,                // [0..4] Active workspace slot index (0..7)
    pub binding_hash: u32,           // [4..8] Hash of bound cortical concept
    pub ignition_potential: i32,     // [8..12] Non-linear threshold accumulator (Q16.16)
    pub persistence_ticks: u32,      // [12..16] Working memory reverberation counter
    pub confidence_q16: u32,         // [16..20] Metacognitive certainty metric (0..1.0 in Q16.16)
    pub broadcast_channel_mask: u32, // [20..24] Target cortical recipient mask
    pub p300_wave_phase: u32,        // [24..28] Ignition oscillation phase angle
    pub is_ignited: u32,             // [28..32] 1 if consciously ignited, 0 if subliminal
    pub _reserved: [u8; 32],         // [32..64] Strict 64-byte cache-line alignment padding
}

impl GlobalWorkspaceSlot {
    pub const IGNITION_THRESHOLD: i32 = 0x0001_8000; // 1.5 in Q16.16

    /// Accumulates evidence and ignites at or above the threshold. The accumulator
    /// saturates (whitepaper §8.1), so sustained evidence can never wrap to a negative
    /// potential and silently un-ignite the slot. Decay and competition are Specified.
    #[inline(always)]
    pub fn step_ignition(&mut self, bottom_up_evidence: i32) -> bool {
        self.ignition_potential = self.ignition_potential.saturating_add(bottom_up_evidence);
        if self.ignition_potential >= Self::IGNITION_THRESHOLD {
            self.is_ignited = 1;
            self.persistence_ticks = 300; // 300ms conscious persistence window
            true
        } else {
            self.is_ignited = 0;
            false
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<GlobalWorkspaceSlot>() == 64);
    assert!(core::mem::align_of::<GlobalWorkspaceSlot>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

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
            _reserved: [0; 32],
        }
    }

    #[test]
    fn ignites_at_exactly_the_threshold() {
        let mut s = slot(0);
        assert!(s.step_ignition(GlobalWorkspaceSlot::IGNITION_THRESHOLD));
        assert_eq!(s.is_ignited, 1);
        assert_eq!(s.persistence_ticks, 300);
    }

    #[test]
    fn one_lsb_below_the_threshold_stays_subliminal() {
        let mut s = slot(0);
        assert!(!s.step_ignition(GlobalWorkspaceSlot::IGNITION_THRESHOLD - 1));
        assert_eq!(s.is_ignited, 0);
        assert_eq!(s.persistence_ticks, 0);
    }

    #[test]
    fn evidence_accumulates_across_steps() {
        let mut s = slot(0);
        assert!(!s.step_ignition(ONE));
        assert!(s.step_ignition(HALF));
        assert_eq!(s.ignition_potential, ONE + HALF);
    }

    #[test]
    fn sustained_evidence_saturates_instead_of_wrapping_negative() {
        let mut s = slot(i32::MAX - 1);
        assert!(s.step_ignition(ONE));
        assert_eq!(s.ignition_potential, i32::MAX);
        assert!(s.step_ignition(i32::MAX));
        assert_eq!(s.ignition_potential, i32::MAX);
        assert_eq!(s.is_ignited, 1);
    }
}
