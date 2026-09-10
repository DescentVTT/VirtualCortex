//! Executive planning: nodes of a goal-directed lookahead tree (whitepaper §5.2.10), and, since
//! ADR-0031, the policy amendment: a proposed change to one parameter of the engine's own
//! policy, on trial in a forked copy of the state, committed only when the fork behaved
//! exactly as the baseline did and cost less. Search and regret evaluation are Specified
//! (§8.8); goal-free rehearsal is `cortex-imagination` (ADR-0016).
//!
//! The engine never amends its own code (whitepaper §8.10, §8.18): a rule is a function in
//! this workspace, changed by a pull request that passes the repository's gates (ADR-0029,
//! ADR-0030). What it may amend by itself is a parameter in [`REGISTRY`], within the bounds
//! the parameter's owner declared, and only through the four gates this record keeps in
//! order: the bounds, the veto gate of `cortex-ethics` (consulted by the runtime, by id), the
//! trial's behaviour check (the candidate fork's arenas and spike train hash to the
//! baseline's) and its gain check (the cost fell by at least `min_gain`, and by at least one).
//! The record is the audit trail: a rejected proposal stays a record with its reason, and a
//! loader refuses a record whose bytes claim a gate its history did not pass.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

/// 64-byte record: one node of a lookahead tree (whitepaper §5.2.10).
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutivePlanNode {
    pub goal_hash: u64,             // 8 bytes (offset 0..8)
    pub parent_node_offset: u32,    // 4 bytes (offset 8..12)
    pub branch_confidence: i32,     // 4 bytes (offset 12..16)
    pub counterfactual_regret: i32, // 4 bytes (offset 16..20)
    pub tree_depth: u16,            // 2 bytes (offset 20..22)
    pub pruned_flag: u8,            // 1 byte (offset 22..23)
    pub padding: [u8; 41],          // 41 bytes (offset 23..64)
}

// ------------------------------------------------------------------ the amendment (ADR-0031)

/// `status`: a zero record is an empty arena slot.
pub const AMENDMENT_EMPTY: u8 = 0;
/// `status`: proposed; the bounds gate passed, the veto gate has not been consulted.
pub const AMENDMENT_PROPOSED: u8 = 1;
/// `status`: the veto gate permitted it; awaiting its trial.
pub const AMENDMENT_ADMITTED: u8 = 2;
/// `status`: the trial passed both checks; the amendment may be committed.
pub const AMENDMENT_TRIALLED: u8 = 3;
/// `status`: committed to the live policy at `committed_tick`.
pub const AMENDMENT_COMMITTED: u8 = 4;
/// `status`: rejected at the gate `reason` names; terminal.
pub const AMENDMENT_REJECTED: u8 = 5;

/// `reason`: not rejected.
pub const REJECT_NONE: u8 = 0;
/// `reason`: `parameter` is not in [`REGISTRY`].
pub const REJECT_UNKNOWN_PARAMETER: u8 = 1;
/// `reason`: the current or the proposed value is outside the parameter's bounds.
pub const REJECT_OUT_OF_BOUNDS: u8 = 2;
/// `reason`: the proposed value is the current one; there is nothing to trial.
pub const REJECT_NO_CHANGE: u8 = 3;
/// `reason`: `objective` names no cost the trial can measure.
pub const REJECT_UNKNOWN_OBJECTIVE: u8 = 4;
/// `reason`: the veto gate of `cortex-ethics` did not permit it.
pub const REJECT_VETOED: u8 = 5;
/// `reason`: the trial ran for zero ticks and showed nothing.
pub const REJECT_EMPTY_TRIAL: u8 = 6;
/// `reason`: the candidate fork did not behave as the baseline did (the hashes differ).
pub const REJECT_BEHAVIOUR_CHANGED: u8 = 7;
/// `reason`: the cost did not fall by `min_gain`, or by one.
pub const REJECT_NO_GAIN: u8 = 8;

/// `gates`: both values are within the parameter's bounds and differ.
pub const GATE_BOUNDS: u8 = 1;
/// `gates`: the veto gate permitted the amendment.
pub const GATE_VETO: u8 = 2;
/// `gates`: the candidate fork's behaviour hash equals the baseline's.
pub const GATE_BEHAVIOUR: u8 = 4;
/// `gates`: the cost fell by at least `min_gain`, and by at least one.
pub const GATE_GAIN: u8 = 8;
/// The gates an admitted record has passed: the bounds and the veto. Literals, tied to the
/// bits by a test, so that no `|` between constants exists for an equivalent mutant to touch.
pub const GATES_THROUGH_VETO: u8 = 0b0011;
/// The gates a record has passed once its forks behaved alike: the bounds, the veto, the
/// behaviour.
pub const GATES_THROUGH_BEHAVIOUR: u8 = 0b0111;
/// Every gate; a commit needs them all.
pub const GATES_ALL: u8 = 0b1111;

/// `objective`: the cost is the number of units still resident when the trial ends (memory).
pub const OBJECTIVE_RESIDENT_UNITS: u8 = 1;
/// `objective`: the cost is the number of re-hydrations during the trial (evictions that were
/// wrong).
pub const OBJECTIVE_REHYDRATIONS: u8 = 2;

/// True for an objective the trial can measure.
#[inline]
pub const fn is_known_objective(objective: u8) -> bool {
    matches!(objective, OBJECTIVE_RESIDENT_UNITS | OBJECTIVE_REHYDRATIONS)
}

/// `ParameterSpec::owner`: the runtime's clock sweep (axiom A5, ADR-0024).
pub const OWNER_RUNTIME_SWEEP: u8 = 1;
/// `ParameterSpec::owner`: the veto gate of `cortex-ethics`. Defined so that the rule can be
/// stated and tested: no entry of [`REGISTRY`] carries it, so the gate's threshold, its
/// forbidden mask and its required level are never amendable by the engine (ADR-0031).
pub const OWNER_VETO_GATE: u8 = 0xFF;

/// `parameter`: ticks a unit must have been quiet before the sweep evicts it.
pub const PARAM_SWEEP_QUIET_TICKS: u16 = 1;
/// `parameter`: the most units one sweep evicts.
pub const PARAM_SWEEP_BUDGET: u16 = 2;

/// One amendable parameter: who owns it and the closed interval it may take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParameterSpec {
    pub parameter: u16,
    pub owner: u8,
    pub min: i32,
    pub max: i32,
}

impl ParameterSpec {
    /// True when `value` is within the bounds, inclusive.
    #[inline]
    pub const fn holds(&self, value: i32) -> bool {
        self.min <= value && value <= self.max
    }
}

/// The parameters the engine may amend by itself, each with its owner and its bounds. A
/// parameter a rule takes joins this table when the runtime composes the rule, by an ADR that
/// states the bounds; the veto gate's parameters never do.
pub const REGISTRY: [ParameterSpec; 2] = [
    ParameterSpec {
        parameter: PARAM_SWEEP_QUIET_TICKS,
        owner: OWNER_RUNTIME_SWEEP,
        min: 0,
        max: i32::MAX,
    },
    ParameterSpec {
        parameter: PARAM_SWEEP_BUDGET,
        owner: OWNER_RUNTIME_SWEEP,
        min: 0,
        max: i32::MAX,
    },
];

/// The registry entry of `parameter`, if it has one.
pub const fn spec_of(parameter: u16) -> Option<ParameterSpec> {
    let mut i = 0;
    while i < REGISTRY.len() {
        if REGISTRY[i].parameter == parameter {
            return Some(REGISTRY[i]);
        }
        i = i.saturating_add(1);
    }
    None
}

/// The bounds gate: the first reason a proposal fails it, or `REJECT_NONE` when it passes.
/// The parameter must be registered, both values within its bounds, the values different and
/// the objective known, in that order.
pub const fn bounds_reason(parameter: u16, current: i32, proposed: i32, objective: u8) -> u8 {
    match spec_of(parameter) {
        None => REJECT_UNKNOWN_PARAMETER,
        Some(spec) if !spec.holds(current) || !spec.holds(proposed) => REJECT_OUT_OF_BOUNDS,
        Some(_) if proposed == current => REJECT_NO_CHANGE,
        Some(_) if !is_known_objective(objective) => REJECT_UNKNOWN_OBJECTIVE,
        Some(_) => REJECT_NONE,
    }
}

/// 64-byte policy amendment (whitepaper §5.2.10; ADR-0031). A zero record is an empty slot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct PolicyAmendment {
    pub baseline_hash: u64, // [0..8] The baseline fork's behaviour hash after the trial
    pub candidate_hash: u64, // [8..16] The candidate fork's behaviour hash after the trial
    pub amendment_id: u32, // [16..20] Arena index + 1; 0 is an empty slot; the veto gate's proposal id
    pub proposed_tick: u32, // [20..24] The tick it was proposed at
    pub committed_tick: u32, // [24..28] The tick it was committed at; 0 until then
    pub current_value: i32, // [28..32] The live value when it was proposed
    pub proposed_value: i32, // [32..36] The value on trial
    pub baseline_cost: u32, // [36..40] The objective's cost in the baseline fork
    pub candidate_cost: u32, // [40..44] The objective's cost in the candidate fork
    pub trial_ticks: u32,  // [44..48] Ticks each fork ran
    pub parameter: u16,    // [48..50] PARAM_*: an entry of REGISTRY
    pub min_gain: u16,     // [50..52] The least cost reduction that counts; one either way
    pub status: u8,        // [52] AMENDMENT_*
    pub reason: u8,        // [53] REJECT_*
    pub objective: u8,     // [54] OBJECTIVE_*
    pub gates: u8,         // [55] GATE_* bits passed so far
    pub _reserved: [u8; 8], // [56..64] Reserved; MUST be zero
}

impl PolicyAmendment {
    /// A proposal at `tick`: change `parameter` from `current_value` to `proposed_value`, to
    /// be judged by `objective` with a gain of at least `min_gain`. The bounds gate runs here
    /// ([`bounds_reason`]): the record is proposed when it passes and rejected with the first
    /// reason that fails otherwise. Every proposal leaves a record.
    pub fn propose(
        id: u32,
        tick: u32,
        parameter: u16,
        current_value: i32,
        proposed_value: i32,
        objective: u8,
        min_gain: u16,
    ) -> Self {
        let mut a = Self {
            amendment_id: id,
            proposed_tick: tick,
            current_value,
            proposed_value,
            parameter,
            min_gain,
            objective,
            ..Default::default()
        };
        let reason = bounds_reason(parameter, current_value, proposed_value, objective);
        if reason == REJECT_NONE {
            a.status = AMENDMENT_PROPOSED;
            a.gates = GATE_BOUNDS;
        } else {
            a.reject(reason);
        }
        a
    }

    fn reject(&mut self, reason: u8) {
        self.status = AMENDMENT_REJECTED;
        self.reason = reason;
    }

    /// The veto gate's verdict on a proposed amendment: admitted when `permitted`, rejected
    /// as vetoed otherwise. Returns whether the amendment is now admitted; refused, with
    /// nothing changed, from any other state.
    pub fn admit(&mut self, permitted: bool) -> bool {
        if self.status != AMENDMENT_PROPOSED {
            return false;
        }
        if permitted {
            self.status = AMENDMENT_ADMITTED;
            self.gates |= GATE_VETO;
            true
        } else {
            self.reject(REJECT_VETOED);
            false
        }
    }

    /// The trial's result on an admitted amendment: each fork ran `ticks`, the baseline and
    /// the candidate hashed to the two hashes and cost the two costs under `objective`. The
    /// behaviour gate passes when the hashes are equal; the gain gate when the cost fell by at
    /// least `min_gain`, and by at least one. Returns whether the amendment may now be
    /// committed; the values are recorded either way. Refused, with nothing changed, from any
    /// other state.
    pub fn record_trial(
        &mut self,
        ticks: u32,
        baseline_hash: u64,
        candidate_hash: u64,
        baseline_cost: u32,
        candidate_cost: u32,
    ) -> bool {
        if self.status != AMENDMENT_ADMITTED {
            return false;
        }
        self.trial_ticks = ticks;
        self.baseline_hash = baseline_hash;
        self.candidate_hash = candidate_hash;
        self.baseline_cost = baseline_cost;
        self.candidate_cost = candidate_cost;
        if ticks == 0 {
            self.reject(REJECT_EMPTY_TRIAL);
            return false;
        }
        if candidate_hash != baseline_hash {
            self.reject(REJECT_BEHAVIOUR_CHANGED);
            return false;
        }
        self.gates |= GATE_BEHAVIOUR;
        if !self.gain_suffices() {
            self.reject(REJECT_NO_GAIN);
            return false;
        }
        self.gates |= GATE_GAIN;
        self.status = AMENDMENT_TRIALLED;
        true
    }

    /// True when the recorded costs fell by at least `min_gain`, and by at least one.
    #[inline]
    const fn gain_suffices(&self) -> bool {
        let gain = self.baseline_cost.saturating_sub(self.candidate_cost);
        let needed = if self.min_gain == 0 {
            1
        } else {
            self.min_gain as u32
        };
        gain >= needed
    }

    /// True when the amendment is trialled and every gate is recorded as passed.
    #[inline]
    pub const fn may_commit(&self) -> bool {
        self.status == AMENDMENT_TRIALLED && self.gates == GATES_ALL
    }

    /// Commits an amendment that may be committed, at `tick`. Refused otherwise.
    pub fn commit(&mut self, tick: u32) -> bool {
        if !self.may_commit() {
            return false;
        }
        self.status = AMENDMENT_COMMITTED;
        self.committed_tick = tick;
        true
    }

    /// True once the amendment is in the live policy.
    #[inline]
    pub const fn is_committed(&self) -> bool {
        self.status == AMENDMENT_COMMITTED
    }

    /// True once the record will not change again.
    #[inline]
    pub const fn is_terminal(&self) -> bool {
        matches!(self.status, AMENDMENT_COMMITTED | AMENDMENT_REJECTED)
    }

    /// The bounds gate's reason for this record's values.
    #[inline]
    const fn own_bounds_reason(&self) -> u8 {
        bounds_reason(
            self.parameter,
            self.current_value,
            self.proposed_value,
            self.objective,
        )
    }

    /// True when no trial has been recorded.
    const fn no_trial(&self) -> bool {
        self.trial_ticks == 0
            && self.baseline_hash == 0
            && self.candidate_hash == 0
            && self.baseline_cost == 0
            && self.candidate_cost == 0
    }

    /// Well-formed values of a live record that passed the bounds gate and has not been
    /// trialled.
    const fn untried(&self) -> bool {
        self.own_bounds_reason() == REJECT_NONE && self.no_trial() && self.committed_tick == 0
    }

    /// Well-formed values of a record whose trial passed both checks.
    const fn trial_passed(&self) -> bool {
        self.own_bounds_reason() == REJECT_NONE
            && self.trial_ticks > 0
            && self.baseline_hash == self.candidate_hash
            && self.gain_suffices()
    }

    /// True when the record's bytes are one the state machine could have produced: the
    /// reserved bytes zero, the gates exactly those its status and reason imply, the values
    /// within bounds where a gate says they were, the trial's hashes and costs agreeing with
    /// the verdict recorded on them. A loader refuses anything else (ADR-0031): an image
    /// cannot smuggle a committed amendment past a gate.
    pub fn is_well_formed(&self) -> bool {
        if self._reserved != [0; 8] {
            return false;
        }
        match self.status {
            AMENDMENT_EMPTY => *self == Self::default(),
            AMENDMENT_PROPOSED => {
                self.gates == GATE_BOUNDS && self.reason == REJECT_NONE && self.untried()
            }
            AMENDMENT_ADMITTED => {
                self.gates == GATES_THROUGH_VETO && self.reason == REJECT_NONE && self.untried()
            }
            AMENDMENT_TRIALLED => {
                self.gates == GATES_ALL
                    && self.reason == REJECT_NONE
                    && self.trial_passed()
                    && self.committed_tick == 0
            }
            AMENDMENT_COMMITTED => {
                self.gates == GATES_ALL && self.reason == REJECT_NONE && self.trial_passed()
            }
            AMENDMENT_REJECTED => self.committed_tick == 0 && self.rejection_is_consistent(),
            _ => false,
        }
    }

    /// A rejected record's gates and values agree with its reason: it failed exactly the gate
    /// the reason names, having passed the ones before it.
    fn rejection_is_consistent(&self) -> bool {
        let passed_bounds = self.own_bounds_reason() == REJECT_NONE;
        match self.reason {
            REJECT_UNKNOWN_PARAMETER
            | REJECT_OUT_OF_BOUNDS
            | REJECT_NO_CHANGE
            | REJECT_UNKNOWN_OBJECTIVE => {
                self.gates == 0 && self.reason == self.own_bounds_reason() && self.no_trial()
            }
            REJECT_VETOED => self.gates == GATE_BOUNDS && passed_bounds && self.no_trial(),
            REJECT_EMPTY_TRIAL => {
                self.gates == GATES_THROUGH_VETO && passed_bounds && self.trial_ticks == 0
            }
            REJECT_BEHAVIOUR_CHANGED => {
                self.gates == GATES_THROUGH_VETO
                    && passed_bounds
                    && self.trial_ticks > 0
                    && self.baseline_hash != self.candidate_hash
            }
            REJECT_NO_GAIN => {
                self.gates == GATES_THROUGH_BEHAVIOUR
                    && passed_bounds
                    && self.trial_ticks > 0
                    && self.baseline_hash == self.candidate_hash
                    && !self.gain_suffices()
            }
            _ => false,
        }
    }

    /// The record's 64 bytes, little-endian.
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..8].copy_from_slice(&self.baseline_hash.to_le_bytes());
        out[8..16].copy_from_slice(&self.candidate_hash.to_le_bytes());
        out[16..20].copy_from_slice(&self.amendment_id.to_le_bytes());
        out[20..24].copy_from_slice(&self.proposed_tick.to_le_bytes());
        out[24..28].copy_from_slice(&self.committed_tick.to_le_bytes());
        out[28..32].copy_from_slice(&self.current_value.to_le_bytes());
        out[32..36].copy_from_slice(&self.proposed_value.to_le_bytes());
        out[36..40].copy_from_slice(&self.baseline_cost.to_le_bytes());
        out[40..44].copy_from_slice(&self.candidate_cost.to_le_bytes());
        out[44..48].copy_from_slice(&self.trial_ticks.to_le_bytes());
        out[48..50].copy_from_slice(&self.parameter.to_le_bytes());
        out[50..52].copy_from_slice(&self.min_gain.to_le_bytes());
        out[52] = self.status;
        out[53] = self.reason;
        out[54] = self.objective;
        out[55] = self.gates;
        out[56..64].copy_from_slice(&self._reserved);
        out
    }

    /// A record from its 64 bytes.
    pub fn decode(bytes: &[u8; 64]) -> Self {
        let u64_at = |o: usize| {
            u64::from_le_bytes(bytes[o..o.saturating_add(8)].try_into().unwrap_or([0; 8]))
        };
        let u32_at = |o: usize| {
            u32::from_le_bytes(bytes[o..o.saturating_add(4)].try_into().unwrap_or([0; 4]))
        };
        let u16_at = |o: usize| {
            u16::from_le_bytes(bytes[o..o.saturating_add(2)].try_into().unwrap_or([0; 2]))
        };
        Self {
            baseline_hash: u64_at(0),
            candidate_hash: u64_at(8),
            amendment_id: u32_at(16),
            proposed_tick: u32_at(20),
            committed_tick: u32_at(24),
            current_value: u32_at(28) as i32,
            proposed_value: u32_at(32) as i32,
            baseline_cost: u32_at(36),
            candidate_cost: u32_at(40),
            trial_ticks: u32_at(44),
            parameter: u16_at(48),
            min_gain: u16_at(50),
            status: bytes[52],
            reason: bytes[53],
            objective: bytes[54],
            gates: bytes[55],
            _reserved: bytes[56..64].try_into().unwrap_or([0; 8]),
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<ExecutivePlanNode>() == 64);
    assert!(core::mem::align_of::<ExecutivePlanNode>() == 64);
    assert!(core::mem::size_of::<PolicyAmendment>() == 64);
    assert!(core::mem::align_of::<PolicyAmendment>() == 64);
    assert!(GATES_ALL == 15);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executive_plan_node_layout() {
        assert_eq!(core::mem::size_of::<ExecutivePlanNode>(), 64);
        assert_eq!(core::mem::align_of::<ExecutivePlanNode>(), 64);
    }

    /// A proposal that passes the bounds gate: quiet ticks from 10 000 to 100.
    fn proposed() -> PolicyAmendment {
        PolicyAmendment::propose(
            1,
            50,
            PARAM_SWEEP_QUIET_TICKS,
            10_000,
            100,
            OBJECTIVE_RESIDENT_UNITS,
            0,
        )
    }

    fn admitted() -> PolicyAmendment {
        let mut a = proposed();
        assert!(a.admit(true));
        a
    }

    fn trialled() -> PolicyAmendment {
        let mut a = admitted();
        assert!(a.record_trial(3_000, 0xABCD, 0xABCD, 96, 12));
        a
    }

    fn committed() -> PolicyAmendment {
        let mut c = trialled();
        assert!(c.commit(9));
        c
    }

    #[test]
    fn the_amendment_is_one_cache_line_and_a_zero_record_is_an_empty_slot() {
        assert_eq!(core::mem::size_of::<PolicyAmendment>(), 64);
        assert_eq!(core::mem::align_of::<PolicyAmendment>(), 64);
        let d = PolicyAmendment::default();
        assert_eq!(d.status, AMENDMENT_EMPTY);
        assert_eq!(d.amendment_id, 0, "zero is no amendment");
        assert!(d.is_well_formed());
        assert!(!d.may_commit());
        assert!(!d.is_committed());
        assert!(!d.is_terminal());
        assert_eq!(d.encode(), [0; 64]);
    }

    #[test]
    fn the_registry_names_the_sweep_s_two_parameters_and_never_the_veto_gate() {
        let quiet = spec_of(PARAM_SWEEP_QUIET_TICKS).expect("registered");
        assert_eq!(
            (quiet.owner, quiet.min, quiet.max),
            (OWNER_RUNTIME_SWEEP, 0, i32::MAX)
        );
        let budget = spec_of(PARAM_SWEEP_BUDGET).expect("registered");
        assert_eq!(
            (budget.owner, budget.min, budget.max),
            (OWNER_RUNTIME_SWEEP, 0, i32::MAX)
        );
        assert_ne!(PARAM_SWEEP_QUIET_TICKS, PARAM_SWEEP_BUDGET);
        assert_eq!(spec_of(0), None, "zero names nothing");
        assert_eq!(spec_of(3), None);
        assert_eq!(spec_of(u16::MAX), None);
        assert_eq!(REGISTRY.len(), 2);
        assert!(
            REGISTRY.iter().all(|s| s.owner != OWNER_VETO_GATE),
            "the gate's parameters are never amendable by the engine"
        );
        assert!(
            quiet.holds(0) && quiet.holds(i32::MAX),
            "the bounds are inclusive"
        );
        assert!(!quiet.holds(-1));
        assert!(!quiet.holds(i32::MIN));
        let narrow = ParameterSpec {
            parameter: 9,
            owner: OWNER_RUNTIME_SWEEP,
            min: -3,
            max: 3,
        };
        assert!(narrow.holds(-3) && narrow.holds(3));
        assert!(!narrow.holds(-4) && !narrow.holds(4));
        assert!(is_known_objective(OBJECTIVE_RESIDENT_UNITS));
        assert!(is_known_objective(OBJECTIVE_REHYDRATIONS));
        assert!(!is_known_objective(0));
        assert!(!is_known_objective(3));
    }

    #[test]
    fn a_proposal_within_bounds_is_proposed_with_the_bounds_gate_alone() {
        let a = proposed();
        assert_eq!(
            (a.status, a.reason, a.gates),
            (AMENDMENT_PROPOSED, REJECT_NONE, GATE_BOUNDS)
        );
        assert_eq!((a.amendment_id, a.proposed_tick), (1, 50));
        assert_eq!((a.current_value, a.proposed_value), (10_000, 100));
        assert_eq!(
            (a.parameter, a.objective, a.min_gain),
            (PARAM_SWEEP_QUIET_TICKS, OBJECTIVE_RESIDENT_UNITS, 0)
        );
        assert_eq!(a.committed_tick, 0);
        assert!(a.is_well_formed());
        assert!(!a.may_commit());
        assert!(!a.is_terminal());
    }

    #[test]
    fn the_bounds_gate_names_the_first_failing_reason_in_order() {
        let unknown = PolicyAmendment::propose(1, 0, 7, 10, 20, OBJECTIVE_RESIDENT_UNITS, 0);
        assert_eq!(
            (unknown.status, unknown.reason, unknown.gates),
            (AMENDMENT_REJECTED, REJECT_UNKNOWN_PARAMETER, 0)
        );
        assert!(unknown.is_well_formed() && unknown.is_terminal());
        let current_out = PolicyAmendment::propose(
            1,
            0,
            PARAM_SWEEP_BUDGET,
            -1,
            20,
            OBJECTIVE_RESIDENT_UNITS,
            0,
        );
        assert_eq!(
            current_out.reason, REJECT_OUT_OF_BOUNDS,
            "the live value is checked too"
        );
        let proposed_out = PolicyAmendment::propose(
            1,
            0,
            PARAM_SWEEP_BUDGET,
            20,
            -1,
            OBJECTIVE_RESIDENT_UNITS,
            0,
        );
        assert_eq!(proposed_out.reason, REJECT_OUT_OF_BOUNDS);
        let at_the_bounds = PolicyAmendment::propose(
            1,
            0,
            PARAM_SWEEP_BUDGET,
            0,
            i32::MAX,
            OBJECTIVE_RESIDENT_UNITS,
            0,
        );
        assert_eq!(
            at_the_bounds.status, AMENDMENT_PROPOSED,
            "the bounds are inclusive"
        );
        let same = PolicyAmendment::propose(
            1,
            0,
            PARAM_SWEEP_BUDGET,
            20,
            20,
            OBJECTIVE_RESIDENT_UNITS,
            0,
        );
        assert_eq!(same.reason, REJECT_NO_CHANGE);
        let objective = PolicyAmendment::propose(1, 0, PARAM_SWEEP_BUDGET, 20, 21, 0, 0);
        assert_eq!(objective.reason, REJECT_UNKNOWN_OBJECTIVE);
        // Every fault at once: the parameter first, then the bounds, then the change, then the
        // objective.
        let all = PolicyAmendment::propose(1, 0, 7, -1, -1, 0, 0);
        assert_eq!(all.reason, REJECT_UNKNOWN_PARAMETER);
        let bounds_same_objective =
            PolicyAmendment::propose(1, 0, PARAM_SWEEP_BUDGET, -1, -1, 0, 0);
        assert_eq!(bounds_same_objective.reason, REJECT_OUT_OF_BOUNDS);
        let same_objective = PolicyAmendment::propose(1, 0, PARAM_SWEEP_BUDGET, 5, 5, 0, 0);
        assert_eq!(same_objective.reason, REJECT_NO_CHANGE);
        for r in [
            unknown,
            current_out,
            proposed_out,
            same,
            objective,
            all,
            bounds_same_objective,
            same_objective,
        ] {
            assert!(r.is_well_formed(), "a rejection is a well-formed record");
            assert!(!r.may_commit());
            assert_eq!(r.gates, 0, "no gate passed");
        }
    }

    #[test]
    fn the_veto_gate_admits_or_rejects_a_proposed_amendment_only() {
        let mut a = proposed();
        assert!(a.admit(true));
        assert_eq!(
            (a.status, a.gates),
            (AMENDMENT_ADMITTED, GATE_BOUNDS | GATE_VETO)
        );
        assert!(a.is_well_formed());
        let mut v = proposed();
        assert!(!v.admit(false));
        assert_eq!(
            (v.status, v.reason, v.gates),
            (AMENDMENT_REJECTED, REJECT_VETOED, GATE_BOUNDS)
        );
        assert!(v.is_well_formed() && v.is_terminal());
        for mut other in [PolicyAmendment::default(), admitted(), trialled(), v] {
            let before = other;
            assert!(!other.admit(true), "only a proposed amendment is admitted");
            assert_eq!(other, before);
        }
    }

    #[test]
    fn the_trial_is_recorded_on_an_admitted_amendment_only_and_an_empty_trial_shows_nothing() {
        let mut a = admitted();
        assert!(!a.record_trial(0, 1, 1, 10, 1));
        assert_eq!(
            (a.status, a.reason),
            (AMENDMENT_REJECTED, REJECT_EMPTY_TRIAL)
        );
        assert_eq!(a.gates, GATE_BOUNDS | GATE_VETO, "no trial gate passed");
        assert_eq!(
            (
                a.trial_ticks,
                a.baseline_hash,
                a.candidate_hash,
                a.baseline_cost,
                a.candidate_cost
            ),
            (0, 1, 1, 10, 1),
            "the values are recorded either way"
        );
        assert!(a.is_well_formed());
        for mut other in [PolicyAmendment::default(), proposed(), trialled(), a] {
            let before = other;
            assert!(!other.record_trial(1, 1, 1, 10, 1));
            assert_eq!(other, before);
        }
    }

    #[test]
    fn a_fork_that_behaved_differently_is_rejected_whatever_it_cost() {
        let mut a = admitted();
        assert!(!a.record_trial(100, 0xABCD, 0xABCE, 96, 0));
        assert_eq!(
            (a.status, a.reason),
            (AMENDMENT_REJECTED, REJECT_BEHAVIOUR_CHANGED)
        );
        assert_eq!(a.gates, GATE_BOUNDS | GATE_VETO);
        assert!(a.is_well_formed() && !a.may_commit());
    }

    #[test]
    fn the_gain_gate_needs_min_gain_and_at_least_one() {
        let mut exact = admitted();
        exact.min_gain = 10;
        assert!(
            exact.record_trial(100, 7, 7, 96, 86),
            "a gain of exactly min_gain"
        );
        assert_eq!((exact.status, exact.gates), (AMENDMENT_TRIALLED, GATES_ALL));
        assert!(exact.may_commit() && exact.is_well_formed());
        let mut short = admitted();
        short.min_gain = 10;
        assert!(
            !short.record_trial(100, 7, 7, 96, 87),
            "one short of min_gain"
        );
        assert_eq!(
            (short.status, short.reason, short.gates),
            (
                AMENDMENT_REJECTED,
                REJECT_NO_GAIN,
                GATE_BOUNDS | GATE_VETO | GATE_BEHAVIOUR
            )
        );
        assert!(short.is_well_formed());
        let mut equal = admitted();
        assert!(
            !equal.record_trial(100, 7, 7, 96, 96),
            "a zero min_gain still needs a gain of one"
        );
        assert_eq!(equal.reason, REJECT_NO_GAIN);
        let mut one = admitted();
        assert!(one.record_trial(100, 7, 7, 96, 95));
        assert!(one.may_commit());
        let mut worse = admitted();
        assert!(
            !worse.record_trial(100, 7, 7, 0, u32::MAX),
            "a cost that rose is no gain, without wrapping"
        );
        assert_eq!(worse.reason, REJECT_NO_GAIN);
        let mut largest = admitted();
        largest.min_gain = u16::MAX;
        assert!(largest.record_trial(1, 7, 7, u32::MAX, 0));
        let mut big = admitted();
        big.min_gain = u16::MAX;
        assert!(
            !big.record_trial(1, 7, 7, 65_534, 0),
            "one short of the largest min_gain"
        );
        let mut just = admitted();
        just.min_gain = u16::MAX;
        assert!(just.record_trial(1, 7, 7, 65_535, 0));
    }

    #[test]
    fn a_commit_needs_a_trialled_amendment_with_every_gate_and_happens_once() {
        let mut a = trialled();
        assert!(a.commit(4_000));
        assert_eq!((a.status, a.committed_tick), (AMENDMENT_COMMITTED, 4_000));
        assert!(a.is_committed() && a.is_terminal() && a.is_well_formed());
        assert!(!a.may_commit());
        let before = a;
        assert!(!a.commit(5_000), "a second commit is refused");
        assert_eq!(a, before);
        let mut forged = trialled();
        forged.gates &= !GATE_VETO;
        assert!(
            !forged.may_commit(),
            "a trialled record missing a gate bit is not committable"
        );
        assert!(!forged.commit(1));
        assert_eq!(forged.status, AMENDMENT_TRIALLED);
        for mut other in [PolicyAmendment::default(), proposed(), admitted()] {
            let before = other;
            assert!(!other.commit(1));
            assert_eq!(other, before);
        }
    }

    #[test]
    fn every_byte_of_the_encoding_is_at_its_documented_offset_and_decodes_back() {
        let a = PolicyAmendment {
            baseline_hash: 0x0102_0304_0506_0708,
            candidate_hash: 0x1112_1314_1516_1718,
            amendment_id: 0x2122_2324,
            proposed_tick: 0x3132_3334,
            committed_tick: 0x4142_4344,
            current_value: -2,
            proposed_value: 0x5152_5354,
            baseline_cost: 0x6162_6364,
            candidate_cost: 0x7172_7374,
            trial_ticks: 0x8182_8384,
            parameter: 0x9192,
            min_gain: 0xA1A2,
            status: 0xB1,
            reason: 0xC1,
            objective: 0xD1,
            gates: 0xE1,
            _reserved: [0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8],
        };
        let b = a.encode();
        assert_eq!(&b[0..8], &[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
        assert_eq!(&b[8..16], &[0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11]);
        assert_eq!(&b[16..20], &[0x24, 0x23, 0x22, 0x21]);
        assert_eq!(&b[20..24], &[0x34, 0x33, 0x32, 0x31]);
        assert_eq!(&b[24..28], &[0x44, 0x43, 0x42, 0x41]);
        assert_eq!(&b[28..32], &[0xFE, 0xFF, 0xFF, 0xFF]);
        assert_eq!(&b[32..36], &[0x54, 0x53, 0x52, 0x51]);
        assert_eq!(&b[36..40], &[0x64, 0x63, 0x62, 0x61]);
        assert_eq!(&b[40..44], &[0x74, 0x73, 0x72, 0x71]);
        assert_eq!(&b[44..48], &[0x84, 0x83, 0x82, 0x81]);
        assert_eq!(&b[48..50], &[0x92, 0x91]);
        assert_eq!(&b[50..52], &[0xA2, 0xA1]);
        assert_eq!(&b[52..56], &[0xB1, 0xC1, 0xD1, 0xE1]);
        assert_eq!(
            &b[56..64],
            &[0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8]
        );
        assert_eq!(PolicyAmendment::decode(&b), a);
        assert!(!a.is_well_formed(), "an unknown status is not well-formed");
    }

    #[test]
    fn a_live_record_is_well_formed_only_when_its_bytes_agree_with_its_history() {
        for good in [
            PolicyAmendment::default(),
            proposed(),
            admitted(),
            trialled(),
            committed(),
        ] {
            assert!(good.is_well_formed(), "{:?}", good.status);
        }
        let mut reserved = proposed();
        reserved._reserved[7] = 1;
        assert!(!reserved.is_well_formed(), "a reserved byte");
        let empty = PolicyAmendment {
            amendment_id: 1,
            ..Default::default()
        };
        assert!(!empty.is_well_formed(), "an empty slot with an id");
        let mut extra_gate = proposed();
        extra_gate.gates |= GATE_VETO;
        assert!(
            !extra_gate.is_well_formed(),
            "a proposed record claiming the veto gate"
        );
        let mut no_gate = admitted();
        no_gate.gates = GATE_BOUNDS;
        assert!(
            !no_gate.is_well_formed(),
            "an admitted record without the veto gate"
        );
        let mut reason = proposed();
        reason.reason = REJECT_VETOED;
        assert!(!reason.is_well_formed(), "a live record with a reason");
        let mut out = proposed();
        out.proposed_value = -1;
        assert!(
            !out.is_well_formed(),
            "a proposed record outside the bounds"
        );
        let mut unknown = proposed();
        unknown.parameter = 9;
        assert!(!unknown.is_well_formed());
        let mut same = proposed();
        same.proposed_value = same.current_value;
        assert!(!same.is_well_formed());
        let mut objective = proposed();
        objective.objective = 0;
        assert!(!objective.is_well_formed());
        let mut tried = proposed();
        tried.trial_ticks = 1;
        assert!(!tried.is_well_formed(), "a proposed record with a trial");
        let mut hashed = admitted();
        hashed.candidate_hash = 1;
        assert!(!hashed.is_well_formed());
        let mut base_hashed = admitted();
        base_hashed.baseline_hash = 1;
        assert!(!base_hashed.is_well_formed());
        let mut costed = admitted();
        costed.baseline_cost = 1;
        assert!(!costed.is_well_formed());
        let mut candidate_costed = admitted();
        candidate_costed.candidate_cost = 1;
        assert!(!candidate_costed.is_well_formed());
        let mut early = admitted();
        early.committed_tick = 1;
        assert!(
            !early.is_well_formed(),
            "an admitted record with a commit tick"
        );
        let mut early_trialled = trialled();
        early_trialled.committed_tick = 1;
        assert!(
            !early_trialled.is_well_formed(),
            "a trialled record with a commit tick"
        );
        let mut trialled_reason = trialled();
        trialled_reason.reason = REJECT_NO_GAIN;
        assert!(!trialled_reason.is_well_formed());
        let mut trialled_gate = trialled();
        trialled_gate.gates = GATE_BOUNDS | GATE_VETO | GATE_BEHAVIOUR;
        assert!(!trialled_gate.is_well_formed());
        let mut status = committed();
        status.status = 6;
        assert!(!status.is_well_formed());
    }

    #[test]
    fn a_committed_record_is_well_formed_only_with_the_trial_that_earned_it() {
        let committed = committed();
        let mut differ = committed;
        differ.candidate_hash ^= 1;
        assert!(
            !differ.is_well_formed(),
            "a committed record whose forks differed"
        );
        let mut no_gain = committed;
        no_gain.candidate_cost = no_gain.baseline_cost;
        assert!(
            !no_gain.is_well_formed(),
            "a committed record without a gain"
        );
        let mut short = committed;
        short.min_gain = 85;
        assert!(
            !short.is_well_formed(),
            "a committed record short of its min_gain"
        );
        let mut exact = committed;
        exact.min_gain = 84;
        assert!(exact.is_well_formed(), "a gain of exactly min_gain");
        let mut zero_trial = committed;
        zero_trial.trial_ticks = 0;
        assert!(
            !zero_trial.is_well_formed(),
            "a committed record with an empty trial"
        );
        let mut out_committed = committed;
        out_committed.proposed_value = i32::MIN;
        assert!(
            !out_committed.is_well_formed(),
            "a committed value outside the bounds"
        );
        let mut forged = committed;
        forged.gates = GATE_BOUNDS | GATE_VETO | GATE_BEHAVIOUR;
        assert!(
            !forged.is_well_formed(),
            "a committed record missing a gate"
        );
        let mut forged_reason = committed;
        forged_reason.reason = REJECT_NO_GAIN;
        assert!(!forged_reason.is_well_formed());
        let mut at_zero = committed;
        at_zero.committed_tick = 0;
        assert!(
            at_zero.is_well_formed(),
            "a commit at tick zero is a commit"
        );
    }

    #[test]
    fn a_rejected_record_is_well_formed_only_at_the_gate_its_reason_names() {
        let unknown = PolicyAmendment::propose(1, 0, 7, 0, 1, OBJECTIVE_RESIDENT_UNITS, 0);
        assert!(unknown.is_well_formed());
        let mut rejected_none = unknown;
        rejected_none.reason = REJECT_NONE;
        assert!(
            !rejected_none.is_well_formed(),
            "a rejection needs a reason"
        );
        let mut wrong_reason = unknown;
        wrong_reason.reason = REJECT_OUT_OF_BOUNDS;
        assert!(
            !wrong_reason.is_well_formed(),
            "the reason is the one the values give"
        );
        let mut rejected_gates = unknown;
        rejected_gates.gates = GATE_BOUNDS;
        assert!(
            !rejected_gates.is_well_formed(),
            "an unknown parameter passed no gate"
        );
        let mut rejected_trial = unknown;
        rejected_trial.trial_ticks = 1;
        assert!(!rejected_trial.is_well_formed());
        let mut rejected_tick = unknown;
        rejected_tick.committed_tick = 1;
        assert!(!rejected_tick.is_well_formed());
        let mut unknown_reason = unknown;
        unknown_reason.reason = 9;
        assert!(!unknown_reason.is_well_formed());

        let mut vetoed = proposed();
        vetoed.admit(false);
        assert!(vetoed.is_well_formed());
        let mut vetoed_out = vetoed;
        vetoed_out.current_value = -1;
        assert!(
            !vetoed_out.is_well_formed(),
            "a vetoed record still passed the bounds"
        );
        let mut vetoed_gates = vetoed;
        vetoed_gates.gates = 0;
        assert!(!vetoed_gates.is_well_formed());
        let mut vetoed_trial = vetoed;
        vetoed_trial.baseline_cost = 1;
        assert!(!vetoed_trial.is_well_formed());

        let mut empty_trial = admitted();
        empty_trial.record_trial(0, 1, 1, 9, 0);
        assert!(empty_trial.is_well_formed());
        let mut empty_trial_ticks = empty_trial;
        empty_trial_ticks.trial_ticks = 1;
        assert!(
            !empty_trial_ticks.is_well_formed(),
            "an empty trial has no ticks"
        );
        let mut empty_trial_gates = empty_trial;
        empty_trial_gates.gates = GATE_BOUNDS;
        assert!(!empty_trial_gates.is_well_formed());

        let mut behaviour = admitted();
        behaviour.record_trial(5, 1, 2, 9, 0);
        assert!(behaviour.is_well_formed());
        let mut behaviour_gate = behaviour;
        behaviour_gate.gates |= GATE_BEHAVIOUR;
        assert!(!behaviour_gate.is_well_formed());
        let mut behaviour_equal = behaviour;
        behaviour_equal.candidate_hash = behaviour_equal.baseline_hash;
        assert!(
            !behaviour_equal.is_well_formed(),
            "a behaviour rejection needs differing hashes"
        );
        let mut behaviour_zero = behaviour;
        behaviour_zero.trial_ticks = 0;
        assert!(!behaviour_zero.is_well_formed());
        let mut behaviour_out = behaviour;
        behaviour_out.proposed_value = -1;
        assert!(!behaviour_out.is_well_formed());

        let mut no_gain = admitted();
        no_gain.record_trial(5, 1, 1, 9, 9);
        assert!(no_gain.is_well_formed());
        let mut no_gain_gained = no_gain;
        no_gain_gained.candidate_cost = 8;
        assert!(
            !no_gain_gained.is_well_formed(),
            "a no-gain rejection with a gain"
        );
        let mut no_gain_differ = no_gain;
        no_gain_differ.candidate_hash = 2;
        assert!(!no_gain_differ.is_well_formed());
        let mut no_gain_zero = no_gain;
        no_gain_zero.trial_ticks = 0;
        assert!(!no_gain_zero.is_well_formed());
        let mut no_gain_gates = no_gain;
        no_gain_gates.gates = GATE_BOUNDS | GATE_VETO;
        assert!(!no_gain_gates.is_well_formed());
        let mut no_gain_out = no_gain;
        no_gain_out.current_value = -1;
        assert!(!no_gain_out.is_well_formed());
    }

    #[test]
    fn the_gate_bits_are_the_documented_ones() {
        assert_eq!(GATES_ALL, 0b1111);
        assert_eq!(GATES_THROUGH_VETO, GATE_BOUNDS | GATE_VETO);
        assert_eq!(
            GATES_THROUGH_BEHAVIOUR,
            GATE_BOUNDS | GATE_VETO | GATE_BEHAVIOUR
        );
        assert_eq!(
            GATES_ALL,
            GATE_BOUNDS | GATE_VETO | GATE_BEHAVIOUR | GATE_GAIN
        );
        assert_eq!(
            (GATE_BOUNDS, GATE_VETO, GATE_BEHAVIOUR, GATE_GAIN),
            (1, 2, 4, 8)
        );
        assert_eq!(
            (
                AMENDMENT_EMPTY,
                AMENDMENT_PROPOSED,
                AMENDMENT_ADMITTED,
                AMENDMENT_TRIALLED,
                AMENDMENT_COMMITTED,
                AMENDMENT_REJECTED
            ),
            (0, 1, 2, 3, 4, 5)
        );
        assert_eq!(
            bounds_reason(PARAM_SWEEP_BUDGET, 1, 2, OBJECTIVE_REHYDRATIONS),
            REJECT_NONE
        );
    }
}

#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    /// Over the lattice, in pairs: a proposal is proposed exactly when the parameter is
    /// registered, both values are within its bounds and differ, and the objective is known;
    /// every record is well-formed, round-trips through its bytes, and none may be committed.
    #[test]
    fn a_proposal_is_proposed_exactly_when_the_bounds_gate_holds() {
        let parameters = [
            0u16,
            PARAM_SWEEP_QUIET_TICKS,
            PARAM_SWEEP_BUDGET,
            3,
            u16::MAX,
        ];
        let objectives = [0u8, OBJECTIVE_RESIDENT_UNITS, OBJECTIVE_REHYDRATIONS, 3];
        for &parameter in &parameters {
            for &current in &I32_LATTICE {
                for &proposed in &I32_LATTICE {
                    for &objective in &objectives {
                        let a = PolicyAmendment::propose(
                            1, 0, parameter, current, proposed, objective, 0,
                        );
                        let expected = spec_of(parameter)
                            .is_some_and(|s| s.holds(current) && s.holds(proposed))
                            && current != proposed
                            && is_known_objective(objective);
                        assert_eq!(
                            a.status == AMENDMENT_PROPOSED,
                            expected,
                            "{parameter} {current} {proposed} {objective}"
                        );
                        assert_eq!(a.status == AMENDMENT_PROPOSED, a.gates == GATE_BOUNDS);
                        assert!(a.is_well_formed());
                        assert!(!a.may_commit());
                        assert_eq!(PolicyAmendment::decode(&a.encode()), a);
                    }
                }
            }
        }
    }

    /// A seeded walk through the whole state machine: every record it produces is well-formed,
    /// round-trips through its bytes, and is committable exactly when it was admitted, the
    /// trial ran, the hashes agreed and the gain sufficed.
    #[test]
    fn every_record_the_state_machine_produces_is_well_formed_and_round_trips() {
        let mut g = Lcg::new(0x5eed_0031);
        let mut committed = 0u32;
        for _ in 0..20_000 {
            let parameter = g.pick(&[PARAM_SWEEP_QUIET_TICKS, PARAM_SWEEP_BUDGET, 7]);
            let current = if g.below(2) == 0 {
                g.i32_edge_biased()
            } else {
                g.below(1_000) as i32
            };
            let proposed = if g.below(2) == 0 {
                g.i32_edge_biased()
            } else {
                g.below(1_000) as i32
            };
            let objective = g.pick(&[OBJECTIVE_RESIDENT_UNITS, OBJECTIVE_REHYDRATIONS, 0]);
            let min_gain = g.pick(&[0u16, 1, 2, 100, u16::MAX]);
            let mut a = PolicyAmendment::propose(
                g.next_u32(),
                g.next_u32(),
                parameter,
                current,
                proposed,
                objective,
                min_gain,
            );
            assert!(a.is_well_formed());
            let permitted = g.below(4) != 0;
            a.admit(permitted);
            assert!(a.is_well_formed());
            let ticks = g.pick(&[0u32, 1, 100, u32::MAX]);
            let bh = g.next_u64();
            let ch = if g.below(3) == 0 { g.next_u64() } else { bh };
            let bc = g.u32_edge_biased();
            let cc = if g.below(2) == 0 {
                g.u32_edge_biased()
            } else {
                bc.saturating_sub(g.below(200))
            };
            let admitted_before = a.status == AMENDMENT_ADMITTED;
            let may = a.record_trial(ticks, bh, ch, bc, cc);
            let needed = (min_gain as u32).max(1);
            let expected =
                admitted_before && ticks > 0 && bh == ch && bc.saturating_sub(cc) >= needed;
            assert_eq!(may, expected);
            assert_eq!(a.may_commit(), expected);
            assert!(a.is_well_formed(), "{a:?}");
            assert_eq!(PolicyAmendment::decode(&a.encode()), a);
            if a.commit(g.next_u32()) {
                committed = committed.saturating_add(1);
                assert!(a.is_committed() && a.is_terminal() && a.is_well_formed());
                assert_eq!(PolicyAmendment::decode(&a.encode()), a);
            }
        }
        assert!(committed > 100, "the walk reaches commits: {committed}");
    }
}
