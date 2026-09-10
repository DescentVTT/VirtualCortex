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

/// `insincerity_q16` moves by $2^{-3}$ of its gap to the latest stated-versus-outcome gap, and by
/// at least one LSB (ADR-0026).
pub const INSINCERITY_SHIFT: u32 = 3;
/// A stated-versus-outcome gap at or above 0.5 disconfirms the agent's stated intent.
pub const SINCERITY_GAP_THRESHOLD_Q16: u32 = Q16_ONE / 2;
/// An insincerity average at or above 0.25 makes the agent suspect.
pub const SUSPICION_THRESHOLD_Q16: u32 = Q16_ONE / 4;

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
    pub expected_of_self_hash: u32, // [40..44] What the agent expects the self to do next: the self's model of the agent's model of the self; 0 = none (ADR-0026)
    pub insincerity_q16: u32, // [44..48] Slow average of the gap between what the agent stated and what followed (Q16.16, ADR-0026)
    pub _reserved: [u8; 16],  // [48..64] Reserved; MUST be zero
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

    /// The other opens the exchange and the self listens: from idle to the other's turn.
    /// Refused once an exchange is under way, where the floor passes by `yield_turn`.
    pub fn listen(&mut self) -> bool {
        self.transition(&[TURN_IDLE], TURN_OTHER)
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

    /// Ends the exchange: the floor is idle, the repair count and the agent's expectation of the
    /// self are cleared (both belong to the exchange), the common ground is kept, since it was
    /// earned.
    pub fn close_exchange(&mut self) {
        self.dialogue_turn_state = TURN_IDLE;
        self.turn_repair_count = 0;
        self.expected_of_self_hash = 0;
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

    /// Records what the agent expects the self to do next, the second level of the model
    /// (the self's model of the agent's model of the self; ADR-0026): read from a directive
    /// the agent addressed to the self, or from the agent's stated prediction. Refused for
    /// zero, which means no expectation.
    pub fn expect_of_self(&mut self, action_hash: u32) -> bool {
        if action_hash == 0 {
            return false;
        }
        self.expected_of_self_hash = action_hash;
        true
    }

    /// Whether `planned_action_hash` departs from what the agent expects of the self; `None`
    /// when nothing is expected.
    pub const fn would_surprise(&self, planned_action_hash: u32) -> Option<bool> {
        if self.expected_of_self_hash == 0 {
            None
        } else {
            Some(self.expected_of_self_hash != planned_action_hash)
        }
    }

    /// The deepest level of the agent's mind the record holds: 2 when the agent's expectation of
    /// the self is held (`expected_of_self_hash`), else 1 when a belief is attributed
    /// (`belief_state_hash`), else 0.
    pub const fn tom_depth(&self) -> u8 {
        if self.expected_of_self_hash != 0 {
            2
        } else if self.belief_state_hash != 0 {
            1
        } else {
            0
        }
    }

    /// The sincerity check (ADR-0026): the agent stated a valence for what it would do (praise,
    /// a promise, a threat) and `outcome_valence_q16` is what followed. The gap, clamped to 1.0,
    /// moves `insincerity_q16` by $2^{-3}$ of its distance and at least one LSB; a gap at or
    /// above 0.5 is a disconfirmed prediction about the agent (`update_trust(false)`), a smaller
    /// one a confirmed one. Returns the gap.
    pub fn assess_sincerity(&mut self, stated_valence_q16: i32, outcome_valence_q16: i32) -> u32 {
        let gap = (stated_valence_q16 as i64 - outcome_valence_q16 as i64)
            .unsigned_abs()
            .min(Q16_ONE as u64) as u32;
        let current = self.insincerity_q16;
        self.insincerity_q16 = if gap > current {
            current.saturating_add(((gap - current) >> INSINCERITY_SHIFT).max(1))
        } else if gap < current {
            current - ((current - gap) >> INSINCERITY_SHIFT).max(1)
        } else {
            current
        };
        self.update_trust(gap < SINCERITY_GAP_THRESHOLD_Q16);
        gap
    }

    /// True when the agent's stated intents have diverged from its outcomes often enough: the
    /// register goes formal with the trust it costs, and the runtime weighs what the agent
    /// asks for as a harm risk in the veto gate (Specified).
    pub const fn is_suspect(&self) -> bool {
        self.insincerity_q16 >= SUSPICION_THRESHOLD_Q16
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

    #[test]
    fn the_sincerity_gap_is_the_distance_its_average_moves_by_an_eighth_and_a_half_is_a_break() {
        let mut n = node(Q16_ONE);
        assert_eq!(
            n.assess_sincerity(Q16_ONE as i32 / 2, Q16_ONE as i32 / 4),
            Q16_ONE / 4
        );
        assert_eq!(n.insincerity_q16, Q16_ONE / 32, "an eighth of the quarter");
        assert_eq!(
            n.assess_sincerity(Q16_ONE as i32 / 4, Q16_ONE as i32 / 2),
            Q16_ONE / 4
        );
        assert_eq!(
            n.insincerity_q16,
            Q16_ONE / 32 + (Q16_ONE / 4 - Q16_ONE / 32) / 8,
            "an eighth of what remains"
        );
        assert_eq!(n.assess_sincerity(0, 0), 0);
        assert_eq!(
            n.insincerity_q16,
            3_840 - (3_840 >> INSINCERITY_SHIFT),
            "a closed gap: it falls by an eighth"
        );
        let mut trusted = node(Q16_ONE);
        trusted.trust_score_q16 = Q16_ONE / 2;
        assert_eq!(trusted.assess_sincerity(Q16_ONE as i32 / 2, 0), Q16_ONE / 2);
        assert_eq!(
            trusted.trust_score_q16,
            Q16_ONE / 2 - ((Q16_ONE / 2) >> TRUST_LOSS_SHIFT),
            "a gap of exactly a half is a disconfirmed prediction"
        );
        let mut kept = node(Q16_ONE);
        kept.trust_score_q16 = Q16_ONE / 2;
        assert_eq!(
            kept.assess_sincerity(Q16_ONE as i32 / 2 - 1, 0),
            Q16_ONE / 2 - 1
        );
        assert!(
            kept.trust_score_q16 > Q16_ONE / 2,
            "one LSB less is confirmed"
        );
        let mut far = node(Q16_ONE);
        assert_eq!(
            far.assess_sincerity(i32::MAX, i32::MIN),
            Q16_ONE,
            "clamped to 1.0"
        );
    }

    #[test]
    fn the_common_ground_folds_each_referent_and_is_pinned() {
        let mut n = node(Q16_ONE);
        assert!(n.listen());
        assert_eq!(n.ground(0x1234), Some(0x341c_a7dc));
        assert_eq!(
            n.ground(0x1234),
            Some(0xf132_5c38),
            "the same referent twice is not once"
        );
        assert_eq!(n.shared_intentionality_hash, 0xf132_5c38);
    }

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
    fn trust_above_one_is_clamped_back_to_one_by_a_confirmation() {
        let mut n = node(0);
        n.trust_score_q16 = Q16_ONE + 5;
        assert_eq!(n.update_trust(true), Q16_ONE);
        let mut m = node(0);
        m.trust_score_q16 = u32::MAX;
        assert_eq!(m.update_trust(true), Q16_ONE, "no wrap on a corrupt value");
    }

    #[test]
    fn the_floor_passes_by_the_rules_and_refuses_the_rest() {
        let mut n = node(0);
        assert!(!n.yield_turn(), "nothing to yield while idle");
        assert!(!n.request_repair(), "nothing to repair while idle");
        assert!(n.listen(), "the other may open the exchange");
        assert_eq!(n.dialogue_turn_state, TURN_OTHER);
        assert!(!n.listen(), "but only from idle");
        assert!(n.take_turn());
        assert!(!n.listen(), "not while the self holds the floor");
        n.close_exchange();
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

    #[test]
    fn the_sincerity_gap_moves_insincerity_and_trust_and_reaches_zero_when_words_match_deeds() {
        let mut n = node(0);
        n.trust_score_q16 = Q16_ONE / 2;
        assert!(!n.is_suspect());
        // Praise that ended badly: a gap of 1.0 (clamped), a disconfirmation.
        assert_eq!(
            n.assess_sincerity(Q16_ONE as i32, -(Q16_ONE as i32)),
            Q16_ONE
        );
        assert_eq!(n.insincerity_q16, Q16_ONE >> INSINCERITY_SHIFT);
        assert!(n.trust_score_q16 < Q16_ONE / 2, "trust broke");
        for _ in 0..3 {
            n.assess_sincerity(Q16_ONE as i32, -(Q16_ONE as i32));
        }
        assert!(n.is_suspect(), "four broken promises");
        // A small gap confirms; insincerity decays to exactly zero.
        let before = n.trust_score_q16;
        assert_eq!(n.assess_sincerity(1000, 900), 100);
        assert!(n.trust_score_q16 > before, "a kept word rebuilds");
        for _ in 0..200 {
            n.assess_sincerity(0, 0);
        }
        assert_eq!(n.insincerity_q16, 0);
        assert!(!n.is_suspect());
    }

    #[test]
    fn the_second_level_of_the_model_says_what_the_agent_expects_of_the_self() {
        let mut n = node(0);
        assert_eq!(n.tom_depth(), 0);
        assert_eq!(n.would_surprise(7), None, "nothing expected");
        n.belief_state_hash = 0xB1;
        assert_eq!(n.tom_depth(), 1);
        assert!(!n.expect_of_self(0));
        assert!(n.expect_of_self(0xA1));
        assert_eq!(n.tom_depth(), 2);
        assert_eq!(n.would_surprise(0xA1), Some(false), "doing the expected");
        assert_eq!(n.would_surprise(0xA2), Some(true), "doing otherwise");
        n.close_exchange();
        assert_eq!(
            n.would_surprise(0xA1),
            None,
            "an expectation belongs to the exchange"
        );
        assert_eq!(n.tom_depth(), 1, "the belief outlives it");
    }
}
