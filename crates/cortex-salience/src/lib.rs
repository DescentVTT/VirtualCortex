//! Salience: threat valuation, the fast subcortical route and emotional tagging (whitepaper
//! §5.2.7). Contextual suppression is Specified; the target that a peak selects is
//! `cortex-attention`'s.

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SalienceNodeState {
    pub node_id: u32,                // [0..4] Salience node index
    pub threat_valence: i32,         // [4..8] Threat intensity (-1.0..+1.0 in Q16.16)
    pub low_road_ticks: u32,         // [8..12] Countdown for 12ms emergency reflex
    pub fear_conditioning_w: i32,    // [12..16] Conditioned stimulus weight (Q16.16)
    pub defense_mode_flags: u32,     // [16..20] 0: None, 1: Freeze, 2: Flight, 3: Fight
    pub emotional_tag_priority: u32, // [20..24] Priority boost for hippocampal SWR replay
    pub unconditioned_stimulus: i32, // [24..28] Pain / shock immediate input (Q16.16)
    pub override_active: u32,        // [28..32] 1 if motor override engaged, 0 otherwise
    pub _reserved: [u8; 32],         // [32..64] Strict 64-byte cache-line alignment padding
}

impl SalienceNodeState {
    /// Threshold rule only: a shock strictly above 2.0, or a conditioned weight strictly
    /// above 1.0, engages the freeze reflex; when neither holds, the reflex, the valence, the
    /// override and the replay tag are released together, and what persists is the conditioned
    /// weight (ADR-0028). No arithmetic is performed, so there is nothing to saturate
    /// (whitepaper §8.1).
    #[inline(always)]
    pub fn evaluate_threat(&mut self, sensory_shock: i32) -> bool {
        self.unconditioned_stimulus = sensory_shock;
        if sensory_shock > 0x0002_0000 || self.fear_conditioning_w > 0x0001_0000 {
            self.threat_valence = 0x0001_0000;
            self.defense_mode_flags = 1; // Freeze reflex
            self.override_active = 1;
            self.emotional_tag_priority = 255;
            true
        } else {
            self.threat_valence = 0;
            self.defense_mode_flags = 0;
            self.override_active = 0;
            self.emotional_tag_priority = 0;
            false
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<SalienceNodeState>() == 64);
    assert!(core::mem::align_of::<SalienceNodeState>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: i32 = 0x0001_0000;
    const TWO: i32 = 0x0002_0000;

    fn node(fear_conditioning_w: i32) -> SalienceNodeState {
        SalienceNodeState {
            node_id: 0,
            threat_valence: 0,
            low_road_ticks: 0,
            fear_conditioning_w,
            defense_mode_flags: 0,
            emotional_tag_priority: 0,
            unconditioned_stimulus: 0,
            override_active: 0,
            _reserved: [0; 32],
        }
    }

    #[test]
    fn shock_at_exactly_two_does_not_trigger() {
        let mut n = node(0);
        assert!(!n.evaluate_threat(TWO));
        assert_eq!(n.unconditioned_stimulus, TWO);
        assert_eq!(n.override_active, 0);
        assert_eq!(n.defense_mode_flags, 0);
    }

    #[test]
    fn shock_one_lsb_above_two_triggers_freeze() {
        let mut n = node(0);
        assert!(n.evaluate_threat(TWO + 1));
        assert_eq!(n.threat_valence, ONE);
        assert_eq!(n.defense_mode_flags, 1);
        assert_eq!(n.override_active, 1);
        assert_eq!(n.emotional_tag_priority, 255);
    }

    #[test]
    fn conditioned_weight_at_exactly_one_does_not_trigger() {
        let mut n = node(ONE);
        assert!(!n.evaluate_threat(0));
    }

    #[test]
    fn conditioned_weight_above_one_triggers_without_a_shock() {
        let mut n = node(ONE + 1);
        assert!(n.evaluate_threat(0));
        assert_eq!(n.override_active, 1);
    }

    #[test]
    fn the_reflex_is_released_with_the_override_and_the_conditioned_weight_persists() {
        let mut n = node(ONE / 2);
        assert!(n.evaluate_threat(i32::MAX));
        assert_eq!(
            (
                n.defense_mode_flags,
                n.threat_valence,
                n.emotional_tag_priority
            ),
            (1, ONE, 255)
        );
        assert!(!n.evaluate_threat(0));
        assert_eq!(n.override_active, 0);
        assert_eq!(
            (
                n.defense_mode_flags,
                n.threat_valence,
                n.emotional_tag_priority
            ),
            (0, 0, 0),
            "released together"
        );
        assert_eq!(n.fear_conditioning_w, ONE / 2, "what was learned stays");
        assert_eq!(n.unconditioned_stimulus, 0);
    }
}
