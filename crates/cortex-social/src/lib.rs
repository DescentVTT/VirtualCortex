//! Social perspective nodes: the engine's model of another agent, its inferred intention and
//! belief, the trust placed in it, the affective resonance it evokes and, since ADR-0021, the
//! state of the dialogue with it (whitepaper §5.2.27, §8.8, §8.13; admitted by ADR-0016).
//!
//! `cortex-agency` decides whether a sensory change was caused by the self; this crate holds
//! one record per *other* agent. Resonance, the trust update, the turn-taking machine, common
//! ground and the register the trust selects are Implemented; intention inference and
//! false-belief tracking are Specified.

#![no_std]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// Trust gains 2^-TRUST_GAIN_SHIFT of the remaining distance to 1.0 per confirmation.
pub const TRUST_GAIN_SHIFT: u32 = 4;
/// Trust loses 2^-TRUST_LOSS_SHIFT of itself per disconfirmation: it breaks faster than it builds.
pub const TRUST_LOSS_SHIFT: u32 = 3;

/// `dialogue_turn_state`: no exchange in progress.
pub const TURN_IDLE: u16 = 0;
/// `dialogue_turn_state`: the self holds the floor.
pub const TURN_SELF: u16 = 1;
/// `dialogue_turn_state`: the other agent holds the floor.
pub const TURN_OTHER: u16 = 2;
/// `dialogue_turn_state`: the self has asked the other to repair (clarify) its last turn.
pub const TURN_REPAIR: u16 = 3;

/// Registers the trust selects (ADR-0021): the politeness level `cortex-linguistic` realises.
/// Trust at or above 0.75: plain, banter allowed.
pub const REGISTER_FAMILIAR: u8 = 0;
/// Trust at or above 0.25: courteous.
pub const REGISTER_COURTEOUS: u8 = 1;
/// Trust below 0.25: formal, hedged.
pub const REGISTER_FORMAL: u8 = 2;

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
    pub turn_repair_count: u8, // [33] Repairs the self has requested in this exchange, saturating (ADR-0021)
    pub dialogue_turn_state: u16, // [34..36] TURN_* (ADR-0021)
    pub shared_intentionality_hash: u32, // [36..40] Common ground: a hash accumulating every grounded referent (ADR-0021)
    pub _reserved: [u8; 24],             // [40..64] Reserved; MUST be zero
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

    /// The self takes the floor: from idle or from the other's turn (a repair request is still
    /// the other's floor and is refused). Returns whether the turn was taken.
    pub fn take_turn(&mut self) -> bool {
        self.transition(&[TURN_IDLE, TURN_OTHER], TURN_SELF)
    }

    /// The self yields the floor to the other. Refused unless the self holds it.
    pub fn yield_turn(&mut self) -> bool {
        self.transition(&[TURN_SELF], TURN_OTHER)
    }

    /// The self asks the other to repair its last turn (a clarification request). Refused
    /// unless the other holds the floor; counted, saturating.
    pub fn request_repair(&mut self) -> bool {
        if self.transition(&[TURN_OTHER], TURN_REPAIR) {
            self.turn_repair_count = self.turn_repair_count.saturating_add(1);
            true
        } else {
            false
        }
    }

    /// A referent both parties now share is mixed into the common ground; a pending repair is
    /// resolved by it (the floor returns to the other). Refused while idle: nothing is grounded
    /// outside an exchange. Returns the new common-ground hash.
    pub fn ground(&mut self, referent_hash: u32) -> Option<u32> {
        if self.dialogue_turn_state == TURN_IDLE {
            return None;
        }
        self.shared_intentionality_hash =
            (self.shared_intentionality_hash ^ referent_hash).wrapping_mul(0x0100_0193);
        if self.dialogue_turn_state == TURN_REPAIR {
            self.dialogue_turn_state = TURN_OTHER;
        }
        Some(self.shared_intentionality_hash)
    }

    /// Ends the exchange: the floor is idle, the repair count is cleared, the common ground is
    /// kept, since it was earned.
    pub fn close_exchange(&mut self) {
        self.dialogue_turn_state = TURN_IDLE;
        self.turn_repair_count = 0;
    }

    /// The register the trust selects (ADR-0021): familiar at or above 0.75, courteous at or
    /// above 0.25, formal below. `cortex-linguistic` takes it as the politeness level.
    #[inline]
    pub const fn register(&self) -> u8 {
        if self.trust_score_q16 >= 3 * (Q16_ONE / 4) {
            REGISTER_FAMILIAR
        } else if self.trust_score_q16 >= Q16_ONE / 4 {
            REGISTER_COURTEOUS
        } else {
            REGISTER_FORMAL
        }
    }

    fn transition(&mut self, from: &[u16], to: u16) -> bool {
        if !from.contains(&self.dialogue_turn_state) {
            return false;
        }
        self.dialogue_turn_state = to;
        true
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
        let d = SocialPerspectiveNode::default();
        assert_eq!(d.trust_score_q16, 0, "trust is earned");
        assert_eq!(d.dialogue_turn_state, TURN_IDLE);
        assert_eq!(
            d.register(),
            REGISTER_FORMAL,
            "a stranger is addressed formally"
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

    #[test]
    fn the_floor_passes_by_the_rules_and_refuses_the_rest() {
        let mut n = node(0);
        assert!(!n.yield_turn(), "nothing to yield while idle");
        assert!(!n.request_repair(), "nothing to repair while idle");
        assert!(n.take_turn());
        assert!(!n.take_turn(), "already holding the floor");
        assert!(n.yield_turn());
        assert_eq!(n.dialogue_turn_state, TURN_OTHER);
        assert!(n.request_repair());
        assert_eq!(
            (n.dialogue_turn_state, n.turn_repair_count),
            (TURN_REPAIR, 1)
        );
        assert!(
            !n.take_turn(),
            "a pending repair keeps the floor with the other"
        );
        assert_eq!(n.ground(0xF00D), Some(n.shared_intentionality_hash));
        assert_eq!(
            n.dialogue_turn_state, TURN_OTHER,
            "a grounded referent resolves the repair"
        );
        assert!(n.take_turn());
        n.close_exchange();
        assert_eq!((n.dialogue_turn_state, n.turn_repair_count), (TURN_IDLE, 0));
        assert_ne!(
            n.shared_intentionality_hash, 0,
            "common ground survives the exchange"
        );
    }

    #[test]
    fn common_ground_accumulates_deterministically_and_only_inside_an_exchange() {
        let mut a = node(0);
        assert_eq!(a.ground(1), None, "idle: nothing is grounded");
        assert!(a.take_turn());
        let mut b = a;
        for r in [10u32, 20, 30] {
            assert_eq!(a.ground(r), b.ground(r));
        }
        assert_eq!(a.shared_intentionality_hash, b.shared_intentionality_hash);
        let mut c = node(0);
        c.take_turn();
        c.ground(30);
        c.ground(20);
        c.ground(10);
        assert_ne!(
            c.shared_intentionality_hash, a.shared_intentionality_hash,
            "order is part of the ground"
        );
        let mut r = node(0);
        r.take_turn();
        r.yield_turn();
        r.turn_repair_count = u8::MAX;
        assert!(r.request_repair());
        assert_eq!(r.turn_repair_count, u8::MAX, "the count saturates");
    }

    #[test]
    fn the_register_follows_the_trust() {
        let mut n = node(0);
        n.trust_score_q16 = Q16_ONE / 4 - 1;
        assert_eq!(n.register(), REGISTER_FORMAL);
        n.trust_score_q16 = Q16_ONE / 4;
        assert_eq!(n.register(), REGISTER_COURTEOUS);
        n.trust_score_q16 = 3 * (Q16_ONE / 4);
        assert_eq!(n.register(), REGISTER_FAMILIAR);
    }
}
