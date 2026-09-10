//! Social perspective nodes: the engine's model of another agent, its inferred intention and
//! belief, the trust placed in it, and the affective resonance it evokes (whitepaper §5.2.27,
//! §8.8; admitted by ADR-0016).
//!
//! `cortex-agency` decides whether a sensory change was caused by the self; this crate holds
//! one record per *other* agent. Resonance and the trust update are Implemented; intention
//! inference and false-belief tracking are Specified.

#![no_std]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// Trust gains 2^-TRUST_GAIN_SHIFT of the remaining distance to 1.0 per confirmation.
pub const TRUST_GAIN_SHIFT: u32 = 4;
/// Trust loses 2^-TRUST_LOSS_SHIFT of itself per disconfirmation: it breaks faster than it builds.
pub const TRUST_LOSS_SHIFT: u32 = 3;

/// 64-byte social perspective node (whitepaper §5.2.27).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SocialPerspectiveNode {
    pub target_agent_id: u32, // [0..4] The other agent; the self is cortex-agency's agent 0
    pub inferred_intention_id: u32, // [4..8] Intention attributed to the agent (Specified)
    pub attention_focus_hash: u32, // [8..12] What the agent is attending to (Specified)
    pub belief_state_hash: u32, // [12..16] What the agent is believed to believe (Specified)
    pub emotional_valence_q16: i32, // [16..20] The agent's last observed valence (Q16.16)
    pub trust_score_q16: u32, // [20..24] Trust in [0, 1] (Q16.16)
    pub empathy_gain_q16: u32, // [24..28] Fraction of the agent's valence mirrored (Q16.16)
    pub resonance_q16: i32,   // [28..32] Mirrored valence from the last resonate() (Q16.16)
    pub false_belief_flag: u8, // [32] 1 when the agent's belief is known to be false (Specified)
    pub _reserved: [u8; 31],  // [33..64] Reserved; MUST be zero
}

impl SocialPerspectiveNode {
    /// Mirrors an observed valence through the empathy gain: `resonance = observed × gain`,
    /// widened and clamped to the `i32` range. Stores the observation and returns the
    /// resonance.
    pub fn resonate(&mut self, observed_valence_q16: i32) -> i32 {
        self.emotional_valence_q16 = observed_valence_q16;
        let product = (observed_valence_q16 as i64 * self.empathy_gain_q16 as i64) >> 16;
        self.resonance_q16 = product.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        self.resonance_q16
    }

    /// Moves trust after a prediction about the agent was confirmed or not. Confirmation closes
    /// 1/16 of the distance to 1.0; disconfirmation removes 1/8 of the trust; each step is at
    /// least one LSB, so trust reaches its bounds instead of stalling short of them. Trust
    /// stays in [0, 1].
    pub fn update_trust(&mut self, prediction_confirmed: bool) -> u32 {
        if prediction_confirmed {
            let remaining = Q16_ONE.saturating_sub(self.trust_score_q16);
            let gain = (remaining >> TRUST_GAIN_SHIFT).max(1).min(remaining);
            self.trust_score_q16 = (self.trust_score_q16 + gain).min(Q16_ONE);
        } else {
            let loss = (self.trust_score_q16 >> TRUST_LOSS_SHIFT).max(1);
            self.trust_score_q16 = self.trust_score_q16.saturating_sub(loss);
        }
        self.trust_score_q16
    }
}

const _: () = {
    assert!(core::mem::size_of::<SocialPerspectiveNode>() == 64);
    assert!(core::mem::align_of::<SocialPerspectiveNode>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    fn node(gain: u32) -> SocialPerspectiveNode {
        SocialPerspectiveNode {
            target_agent_id: 7,
            empathy_gain_q16: gain,
            ..Default::default()
        }
    }

    #[test]
    fn record_is_one_cache_line_and_default_is_zero() {
        assert_eq!(core::mem::size_of::<SocialPerspectiveNode>(), 64);
        assert_eq!(core::mem::align_of::<SocialPerspectiveNode>(), 64);
        assert_eq!(
            SocialPerspectiveNode::default().trust_score_q16,
            0,
            "trust is earned"
        );
    }

    #[test]
    fn unit_gain_mirrors_the_valence_and_zero_gain_mirrors_nothing() {
        let mut n = node(Q16_ONE);
        assert_eq!(n.resonate(-0x0000_8000), -0x0000_8000);
        assert_eq!(n.emotional_valence_q16, -0x0000_8000);
        let mut m = node(0);
        assert_eq!(m.resonate(i32::MAX), 0);
    }

    #[test]
    fn half_gain_halves_and_a_large_gain_clamps() {
        let mut n = node(Q16_ONE / 2);
        assert_eq!(n.resonate(Q16_ONE as i32), (Q16_ONE / 2) as i32);
        let mut m = node(u32::MAX);
        assert_eq!(m.resonate(i32::MAX), i32::MAX);
        assert_eq!(m.resonate(i32::MIN), i32::MIN);
    }

    #[test]
    fn trust_builds_slowly_and_converges_to_one() {
        let mut n = node(0);
        assert_eq!(n.update_trust(true), Q16_ONE >> TRUST_GAIN_SHIFT);
        for _ in 0..500 {
            n.update_trust(true);
        }
        assert_eq!(
            n.trust_score_q16, Q16_ONE,
            "full trust is reached, not approached"
        );
    }

    #[test]
    fn trust_breaks_faster_than_it_builds_and_never_goes_negative() {
        let mut n = node(0);
        n.trust_score_q16 = Q16_ONE;
        let after_one_loss = n.update_trust(false);
        assert_eq!(after_one_loss, Q16_ONE - (Q16_ONE >> TRUST_LOSS_SHIFT));
        let mut m = node(0);
        m.update_trust(true);
        assert!(
            Q16_ONE - after_one_loss > m.trust_score_q16,
            "one loss costs more than one gain earns"
        );
        for _ in 0..200 {
            n.update_trust(false);
        }
        assert_eq!(n.trust_score_q16, 0);
    }
}
