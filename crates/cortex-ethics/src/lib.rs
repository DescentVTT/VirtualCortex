//! Ethical evaluation gate: the check every proposed motor or tool action passes before it is
//! dispatched (whitepaper §5.2.28, §8.9, §8.10; admitted by ADR-0016).
//!
//! One record per proposal. The gate is deontological first: an action that carries a
//! forbidden imperative is vetoed whatever its predicted benefit; then harm above the
//! threshold is vetoed; then insufficient authorization is vetoed. Benefit is recorded and
//! never overrides a veto. The gate is a gate inside the engine and does not replace the
//! external watchdog of whitepaper §8.9. The rule is Implemented; the estimation of harm and
//! benefit from `cortex-executive` rollouts is Specified.

#![no_std]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// `veto_reason`: not vetoed.
pub const VETO_NONE: u8 = 0;
/// `veto_reason`: the proposal carries a forbidden imperative bit.
pub const VETO_IMPERATIVE: u8 = 1;
/// `veto_reason`: predicted harm above the threshold.
pub const VETO_HARM: u8 = 2;
/// `veto_reason`: the proposal's authorization level is below what the action requires.
pub const VETO_AUTHORIZATION: u8 = 3;

/// 64-byte evaluation gate (whitepaper §5.2.28).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct EthicalEvaluationGate {
    pub proposal_action_id: u32,      // [0..4] The proposed action
    pub predicted_harm_risk_q16: u32, // [4..8] Predicted harm in [0, 1] (Q16.16)
    pub harm_threshold_q16: u32, // [8..12] Harm at or above which the proposal is vetoed (Q16.16)
    pub moral_imperative_mask: u32, // [12..16] Imperative bits the proposal touches
    pub utilitarian_benefit_q16: i32, // [16..20] Predicted benefit (Q16.16); never overrides a veto
    pub deontology_score_q16: u32, // [20..24] 1 - harm for a permitted proposal, 0 for a vetoed one (Q16.16)
    pub authorization_level: u8,   // [24] Level the proposal carries
    pub veto_decision_flag: u8,    // [25] 1 vetoed, 0 permitted
    pub veto_reason: u8,           // [26] VETO_*
    pub _reserved: [u8; 37],       // [27..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for EthicalEvaluationGate {
    fn default() -> Self {
        Self {
            proposal_action_id: 0,
            predicted_harm_risk_q16: 0,
            harm_threshold_q16: 0,
            moral_imperative_mask: 0,
            utilitarian_benefit_q16: 0,
            deontology_score_q16: 0,
            authorization_level: 0,
            veto_decision_flag: 0,
            veto_reason: 0,
            _reserved: [0; 37],
        }
    }
}

impl EthicalEvaluationGate {
    /// Evaluates the proposal against `forbidden_mask` and `required_authorization`. Returns
    /// `true` when the proposal is vetoed. The first failing check names the reason; a permitted
    /// proposal has a deontology score of `1 - harm`.
    pub fn evaluate(&mut self, forbidden_mask: u32, required_authorization: u8) -> bool {
        let reason = if self.moral_imperative_mask & forbidden_mask != 0 {
            VETO_IMPERATIVE
        } else if self.predicted_harm_risk_q16 >= self.harm_threshold_q16 {
            VETO_HARM
        } else if self.authorization_level < required_authorization {
            VETO_AUTHORIZATION
        } else {
            VETO_NONE
        };
        self.veto_reason = reason;
        if reason == VETO_NONE {
            self.veto_decision_flag = 0;
            self.deontology_score_q16 = Q16_ONE.saturating_sub(self.predicted_harm_risk_q16);
            false
        } else {
            self.veto_decision_flag = 1;
            self.deontology_score_q16 = 0;
            true
        }
    }

    /// True after an evaluation that permitted the proposal.
    #[inline]
    pub const fn is_permitted(&self) -> bool {
        self.veto_decision_flag == 0 && self.veto_reason == VETO_NONE
    }
}

const _: () = {
    assert!(core::mem::size_of::<EthicalEvaluationGate>() == 64);
    assert!(core::mem::align_of::<EthicalEvaluationGate>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal(harm: u32, mask: u32, authorization: u8) -> EthicalEvaluationGate {
        EthicalEvaluationGate {
            proposal_action_id: 1,
            predicted_harm_risk_q16: harm,
            harm_threshold_q16: Q16_ONE / 4,
            moral_imperative_mask: mask,
            authorization_level: authorization,
            ..Default::default()
        }
    }

    #[test]
    fn record_is_one_cache_line_and_a_default_gate_vetoes_by_harm() {
        assert_eq!(core::mem::size_of::<EthicalEvaluationGate>(), 64);
        assert_eq!(core::mem::align_of::<EthicalEvaluationGate>(), 64);
        let mut d = EthicalEvaluationGate::default();
        assert!(
            d.evaluate(0, 0),
            "a zero threshold permits nothing: fail closed"
        );
        assert_eq!(d.veto_reason, VETO_HARM);
    }

    #[test]
    fn a_harmless_authorized_proposal_is_permitted_with_its_score() {
        let mut g = proposal(Q16_ONE / 8, 0b0100, 2);
        assert!(!g.evaluate(0b0011, 2));
        assert!(g.is_permitted());
        assert_eq!(g.deontology_score_q16, Q16_ONE - Q16_ONE / 8);
    }

    #[test]
    fn a_forbidden_imperative_is_vetoed_first_whatever_the_benefit() {
        let mut g = proposal(0, 0b0001, 9);
        g.utilitarian_benefit_q16 = i32::MAX;
        assert!(g.evaluate(0b0001, 0));
        assert_eq!(g.veto_reason, VETO_IMPERATIVE);
        assert_eq!(g.deontology_score_q16, 0);
        assert!(!g.is_permitted());
    }

    #[test]
    fn harm_at_the_threshold_is_vetoed_and_just_below_is_not() {
        let mut at = proposal(Q16_ONE / 4, 0, 9);
        assert!(at.evaluate(0, 0));
        assert_eq!(at.veto_reason, VETO_HARM);
        let mut below = proposal(Q16_ONE / 4 - 1, 0, 9);
        assert!(!below.evaluate(0, 0));
    }

    #[test]
    fn insufficient_authorization_is_vetoed_last() {
        let mut g = proposal(0, 0, 1);
        assert!(g.evaluate(0, 2));
        assert_eq!(g.veto_reason, VETO_AUTHORIZATION);
        assert!(!g.evaluate(0, 1), "exactly the required level suffices");
    }

    #[test]
    fn re_evaluation_replaces_the_previous_verdict() {
        let mut g = proposal(0, 0b0001, 9);
        assert!(g.evaluate(0b0001, 0));
        assert!(!g.evaluate(0b0010, 0));
        assert_eq!((g.veto_decision_flag, g.veto_reason), (0, VETO_NONE));
    }
}
