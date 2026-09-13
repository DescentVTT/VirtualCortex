//! Brief 021's exit test (ADR-0041, ADR-0043; whitepaper §6.10, §8.8): the discovery path as
//! the runtime composes it. Two clauses of one head, three shared literals and one differing
//! literal each (ids from a vocabulary that exists only in this test) are intra-constructed
//! on a primed affect state: the store's description length falls from thirty nodes to
//! twenty-five, the valence is five nodes and the reward a full 1.0, pinned; that reward into
//! the modulator of a network with a pending eligibility trace (the shape of
//! `modulation.rs`) consolidates the trace at the next presynaptic spike, the weight gaining
//! what the trace lost; an invention that does not pay (one shared literal) is a negative
//! valence, a negative reward, and, under the test's baseline of zero (the modulation is the
//! baseline plus the signal, clamped to $[0, 1]$), consolidates nothing; the invention's
//! statement hash is
//! pinned, and a prover frame completed with that hash and a certificate certifies a node,
//! while the pending frame does not.

#![deny(clippy::arithmetic_side_effects)]

use cortex_affect::InteroceptiveState;
use cortex_core::{
    MODULATION_ONE_Q16, STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16,
};
use cortex_knowledge::SemanticOntologyNode;
use cortex_reasoning::{Binding, INVENTED_BASE, InduceScratch, TermNode, clause, term_hash};
use cortex_runtime::{
    CertifyError, Config, Executor, certify_from_frame, conjecture_frame, invent, prime,
};

/// The vocabulary: predicates and the head, as concept ids; no word enters the runtime.
const P: u32 = 0x100;
const R: u32 = 0x101;
const S: u32 = 0x102;
const T: u32 = 0x103;
const U: u32 = 0x104;
const W: u32 = 0x105;

/// The statement hash of `p(X, Z) ← r(X, Y), s(Y, Z), t(X, Z), q(X)` through the bindings the
/// invention made, computed by an oracle outside the tree.
const STATEMENT: u32 = 0x21a1_c619;
const ONE: i32 = MODULATION_ONE_Q16;

/// Unit 0 fans out to unit 1 through slot 0 of block 0: weight 1 000, one tick of delay.
const WEIGHT: i16 = 1000;
/// Messages that make an armed unit at rest fire.
const KICK: usize = 13;

struct Arena {
    nodes: [TermNode; 64],
    len: usize,
    vars: u32,
}

impl Arena {
    fn new() -> Self {
        Self {
            nodes: [TermNode::default(); 64],
            len: 0,
            vars: 0,
        }
    }

    fn push(&mut self, node: TermNode) -> u32 {
        let i = self.len;
        self.nodes[i] = node;
        self.len = self.len.wrapping_add(1);
        i as u32
    }

    fn var(&mut self) -> u32 {
        let v = self.vars;
        self.vars = self.vars.wrapping_add(1);
        self.push(TermNode::variable(v))
    }

    fn compound(&mut self, f: u32, args: &[u32]) -> u32 {
        self.push(TermNode::compound(f, args).unwrap())
    }

    /// `p(X, Z) ← r(X, Y), s(Y, Z), t(X, Z), own(X)` over fresh variables.
    fn wide(&mut self, own: u32) -> u32 {
        let (x, y, z) = (self.var(), self.var(), self.var());
        let head = self.compound(P, &[x, z]);
        let body = [
            self.compound(R, &[x, y]),
            self.compound(S, &[y, z]),
            self.compound(T, &[x, z]),
            self.compound(own, &[x]),
        ];
        self.push(clause(head, &body).unwrap())
    }

    /// `p(X, Z) ← r(X, Y), own(X)` over fresh variables.
    fn narrow(&mut self, own: u32) -> u32 {
        let (x, y, z) = (self.var(), self.var(), self.var());
        let head = self.compound(P, &[x, z]);
        let body = [self.compound(R, &[x, y]), self.compound(own, &[x])];
        self.push(clause(head, &body).unwrap())
    }
}

struct Slices {
    bindings: [Binding; 32],
    trail: [u32; 64],
    stack: [u32; 128],
    pairs: [[u32; 3]; 8],
}

impl Slices {
    fn new() -> Self {
        Self {
            bindings: [Binding::UNBOUND; 32],
            trail: [0; 64],
            stack: [0; 128],
            pairs: [[0; 3]; 8],
        }
    }

    fn scratch<'a>(&'a mut self, arena: &'a mut Arena) -> InduceScratch<'a> {
        InduceScratch {
            arena: &mut arena.nodes,
            free: arena.len,
            bindings: &mut self.bindings,
            trail: &mut self.trail,
            trail_len: 0,
            stack: &mut self.stack,
            pairs: &mut self.pairs,
            next_variable: arena.vars,
            next_invented: INVENTED_BASE,
        }
    }
}

fn config() -> Config {
    Config {
        workers: 2,
        units: 2,
        blocks: 1,
        nodes_per_worker: 64,
        injector_capacity: 64,
        modulation_baseline_q16: 0,
        ..Config::default()
    }
}

fn network() -> Executor<64> {
    let mut exec = Executor::<64>::new(config()).expect("a valid configuration");
    assert!(exec.blocks_mut()[0].set_synapse(0, 1, WEIGHT, 1, false));
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
    assert!(exec.units_mut()[0].set_first_block(0));
    exec
}

/// Makes `unit` fire within the next few ticks, and runs until it has.
fn fire(exec: &mut Executor<64>, unit: u32) -> u32 {
    let before = exec.units()[unit as usize].last_soma_spike_tick;
    let inject = exec.injector();
    let strong = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..KICK {
        inject.inject(unit, spike_message(strong, false)).unwrap();
    }
    for _ in 0..40 {
        exec.tick();
        let now = exec.units()[unit as usize].last_soma_spike_tick;
        if now != before {
            return now;
        }
    }
    panic!("unit {unit} did not fire");
}

/// The synapse's weight and trace, between ticks.
fn synapse(exec: &Executor<64>) -> (i16, i16) {
    let block = &exec.blocks()[0];
    (block.weights_q1_15[0], block.eligibility_q1_15[0])
}

/// Three pairings, presynaptic then postsynaptic twenty ticks later, then quiet: a trace
/// pending and the weight unmoved under a baseline of zero.
fn pending(exec: &mut Executor<64>) -> i16 {
    for _ in 0..3 {
        let pre = fire(exec, 0);
        exec.run(20);
        let post = fire(exec, 1);
        assert!(post > pre);
        exec.run(4000);
    }
    let (weight, trace) = synapse(exec);
    assert_eq!(
        weight, WEIGHT,
        "with the baseline at 0 no pairing reaches the weight"
    );
    assert!(
        trace > 300,
        "three presynaptic spikes, two pairings: {trace}"
    );
    trace
}

#[test]
fn an_invention_that_shortens_the_store_is_a_full_reward_that_consolidates_the_trace() {
    let mut arena = Arena::new();
    let ca = arena.wide(U);
    let cb = arena.wide(W);
    let store = [ca, cb];
    let mut slices = Slices::new();
    let mut scratch = slices.scratch(&mut arena);
    let mut affect = InteroceptiveState::default();
    prime(&mut affect, 30);
    let d = invent(ca, cb, &store, &mut scratch, &mut affect).unwrap();
    assert_eq!((d.length_before, d.length_after), (30, 25));
    assert_eq!(d.valence_q16, 5 * ONE, "five nodes saved");
    assert_eq!(d.reward_q16, ONE, "five over four, clamped: a full reward");
    assert_eq!(
        (
            d.invention.predicate,
            d.invention.arguments,
            d.invention.shared
        ),
        (INVENTED_BASE, 1, 3)
    );
    assert_eq!(
        term_hash(
            d.invention.common,
            scratch.arena,
            scratch.bindings,
            scratch.stack
        ),
        Ok(STATEMENT)
    );
    assert_eq!(affect.valence_df_dt_q16, 5 * ONE);
    assert_eq!(
        affect.existential_stake_q16,
        (5 * ONE as u32) >> 4,
        "one step toward |ΔF|"
    );
    assert_eq!(affect.free_energy_prev_q16, 25 << 16);

    let mut exec = network();
    let trace = pending(&mut exec);
    assert_eq!(exec.modulator().dopamine_rpe, 0, "no reward yet");
    assert_eq!(
        exec.reward(d.reward_q16),
        ONE,
        "the discovery's reward is the signal"
    );
    fire(&mut exec, 0);
    let (weight_after, trace_after) = synapse(&exec);
    assert!(
        trace_after <= 1 && weight_after > WEIGHT.wrapping_add(300),
        "consolidated nearly whole: weight {weight_after}, trace {trace_after}"
    );
    assert!(
        i32::from(weight_after).wrapping_sub(i32::from(WEIGHT)) >= i32::from(trace).wrapping_sub(2),
        "the weight gained at least the trace that was pending: {weight_after} from {trace}"
    );
}

#[test]
fn an_invention_that_does_not_pay_is_a_negative_reward_that_consolidates_nothing() {
    let mut arena = Arena::new();
    let ca = arena.narrow(U);
    let cb = arena.narrow(W);
    let store = [ca, cb];
    let mut slices = Slices::new();
    let mut scratch = slices.scratch(&mut arena);
    let mut affect = InteroceptiveState::default();
    prime(&mut affect, 18);
    let d = invent(ca, cb, &store, &mut scratch, &mut affect).unwrap();
    assert_eq!((d.length_before, d.length_after), (18, 19));
    assert_eq!(d.valence_q16, -ONE, "one node longer");
    assert_eq!(d.reward_q16, -ONE / 4);

    let mut exec = network();
    let trace = pending(&mut exec);
    assert_eq!(exec.reward(d.reward_q16), -ONE / 4, "a negative signal");
    fire(&mut exec, 0);
    let (weight_after, trace_after) = synapse(&exec);
    assert_eq!(
        weight_after, WEIGHT,
        "a modulation clamped at zero moves nothing"
    );
    assert!(
        trace_after >= trace,
        "the trace only grew with the new spike: {trace_after}"
    );
}

#[test]
fn the_invention_leaves_as_a_conjecture_and_returns_as_a_theorem_only_when_certified() {
    const CERTIFICATE: u32 = 0x5EA1_ED01;
    let mut node = SemanticOntologyNode::default();
    let mut frame = conjecture_frame(21, STATEMENT, 1).unwrap();
    assert_eq!(
        certify_from_frame(&frame, STATEMENT, &mut node),
        Err(CertifyError::NotCompleted),
        "pending: nothing yet"
    );
    assert!(!node.is_certified_theorem());
    assert!(frame.start());
    let mut payload = [0u8; 8];
    payload[..4].copy_from_slice(&STATEMENT.to_le_bytes());
    payload[4..].copy_from_slice(&CERTIFICATE.to_le_bytes());
    assert!(frame.complete(&payload));
    assert_eq!(
        certify_from_frame(&frame, STATEMENT, &mut node),
        Ok(CERTIFICATE)
    );
    assert!(node.is_certified_theorem());
    assert_eq!(node.property_vector_hash, STATEMENT);
    assert_eq!(
        certify_from_frame(&frame, STATEMENT ^ 1, &mut node),
        Err(CertifyError::Mismatch),
        "the frame certifies its own statement only"
    );
}
