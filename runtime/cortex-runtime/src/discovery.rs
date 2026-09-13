//! The discovery path (whitepaper §5.2.23, §5.2.29, §6.10, §8.8; ADR-0043; brief 021): the
//! place an invention of `cortex-reasoning` meets the valence of `cortex-affect`, the reward
//! of `cortex-neuromod` and the certification of `cortex-knowledge`, between ticks, with no
//! executor field. A clause store's description length in nodes is the free energy the
//! valence rule reads (minimum description length and Helmholtz free energy are one quantity:
//! Hinton and Zemel 1994); an intra-construction that shortens the store is a drop, the drop
//! is the valence, and a quarter of the valence, clamped to $[-1, 1]$, is the
//! reward-prediction error `Executor::reward` takes, at the scale the mirth already arrives
//! at (compression progress as the reward: Schmidhuber 2009). What the modulator then
//! consolidates is whatever eligibility trace is pending; whether that is anything a later
//! behaviour reads is hypothesis H-11. A conjecture leaves as a prover frame whose parameter
//! hash is the statement's; a node is certified only by a completed prover frame whose
//! payload carries that statement hash and a certificate hash, every other frame leaving the
//! node untouched (§6.10, fail-closed). The clause store, the scratch and the affect state are
//! the caller's, as the language module's codebook is; which clauses to try is the caller's
//! search (Specified). Nothing here allocates.

use cortex_affect::{InteroceptiveState, Q16_ONE};
use cortex_knowledge::SemanticOntologyNode;
use cortex_reasoning::{
    Binding, INVENTED_LIMIT, InduceError, InduceScratch, Invention, TermNode, intra_construct,
    next_pair, size,
};
use cortex_tools::{
    ACTION_SOLVE_CONSTRAINTS, ACTION_VERIFY_PROOF, STATUS_COMPLETED, TOOL_CATEGORY_FORMAL_PROVER,
    ToolInvocationFrame,
};

use crate::language::ROLE_CONCEPT_BASE;

/// The valence shifted right by this many bits is the reward: a saving of four nodes is a
/// full reward, the quarter at which the mirth of `cortex-affect` arrives at the modulator.
pub const COMPRESSION_REWARD_SHIFT: u32 = 2;
/// The bytes of a prover's payload: `[0..4)` the statement hash it certifies and `[4..8)`
/// the certificate hash, both little-endian; a certificate hash of zero is none.
pub const CERTIFICATE_BYTES: usize = 8;
/// The most nodes a description length reads as: the integer ceiling of Q16.16.
pub const LENGTH_CEILING: u32 = 0xFFFF;

/// Why an invention was not made into a discovery. The scratch and the affect state are as
/// they were.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscoveryError {
    /// The operator's reason.
    Induce(InduceError),
    /// A clause of the store could not be measured (an index outside the arena, an empty
    /// node, a stack too small).
    Length,
    /// The affect state's previous free energy is not the store's length: `prime` first.
    NotPrimed,
    /// One of the two clauses is not in the store.
    NotInStore,
    /// The search committed an invention and the store has no slot for its third clause
    /// (ADR-0045): the store's slice is its capacity.
    StoreFull,
    /// The search committed more inventions than `out` holds (ADR-0045).
    OutFull,
}

impl From<InduceError> for DiscoveryError {
    fn from(error: InduceError) -> Self {
        Self::Induce(error)
    }
}

/// Why a frame certified nothing. The node is as it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CertifyError {
    /// The frame is pending, running, failed or denied.
    NotCompleted,
    /// The frame's category is not the formal prover.
    NotAProver,
    /// The frame's action is neither a proof check nor a constraint solve.
    NotAVerification,
    /// The frame's parameter hash or the payload's statement hash is not the statement's.
    Mismatch,
    /// The payload is shorter than `CERTIFICATE_BYTES` or its certificate hash is zero.
    NoCertificate,
}

/// What an invention did to the store and to the affect state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Discovery {
    pub invention: Invention,
    pub length_before: u32,
    pub length_after: u32,
    pub valence_q16: i32,
    pub reward_q16: i32,
}

/// The description length of a clause store: the sum of its clauses' sizes through the
/// bindings, saturating.
pub fn description_length(
    store: &[u32],
    arena: &[TermNode],
    bindings: &[Binding],
    stack: &mut [u32],
) -> Result<u32, DiscoveryError> {
    let mut total = 0u32;
    for &clause in store {
        let nodes = size(clause, arena, bindings, stack).map_err(|_| DiscoveryError::Length)?;
        total = total.saturating_add(nodes);
    }
    Ok(total)
}

/// A description length as the free energy the valence rule reads: whole nodes in Q16.16,
/// saturating at the format's ceiling of `LENGTH_CEILING` nodes.
pub fn free_energy_q16(length: u32) -> u32 {
    // At most sixteen bits, so the shift stays within the width.
    length.min(LENGTH_CEILING) << 16
}

/// Writes the store's length into the affect state as the free energy the next discovery
/// is measured against.
pub fn prime(affect: &mut InteroceptiveState, length: u32) {
    affect.free_energy_prev_q16 = free_energy_q16(length);
}

/// The reward-prediction error a valence becomes: the valence shifted right by
/// `COMPRESSION_REWARD_SHIFT` (an arithmetic shift, so a negative valence stays negative),
/// clamped to $[-1, 1]$.
pub fn reward_q16(valence_q16: i32) -> i32 {
    let one = Q16_ONE as i32;
    (valence_q16 >> COMPRESSION_REWARD_SHIFT).clamp(one.wrapping_neg(), one)
}

/// Intra-constructs `ca` and `cb` into a discovery: the store's length before, its length
/// after (the two inputs replaced by the three outputs, every clause measured through the
/// bindings the matching made), the valence `update_valence` returns for the length after,
/// and the reward. `NotInStore` unless both clauses are in `store`; `NotPrimed` unless the
/// affect state's previous free energy is the length before; every error leaves the scratch
/// and the affect state as they were (an after-measure that does not fit restores what the
/// invention did).
pub fn invent(
    ca: u32,
    cb: u32,
    store: &[u32],
    scratch: &mut InduceScratch,
    affect: &mut InteroceptiveState,
) -> Result<Discovery, DiscoveryError> {
    if !store.contains(&ca) || !store.contains(&cb) {
        return Err(DiscoveryError::NotInStore);
    }
    let length_before = description_length(store, scratch.arena, scratch.bindings, scratch.stack)?;
    if affect.free_energy_prev_q16 != free_energy_q16(length_before) {
        return Err(DiscoveryError::NotPrimed);
    }
    let mark = scratch.mark();
    let invention = intra_construct(ca, cb, scratch)?;
    let outputs = [
        invention.common,
        invention.definitions[0],
        invention.definitions[1],
    ];
    let mut length_after = 0u32;
    for &clause in store
        .iter()
        .filter(|&&c| c != ca && c != cb)
        .chain(outputs.iter())
    {
        // The store measured before; a deeper term through the new bindings may still
        // exceed the stack, and then the invention is undone.
        let Ok(nodes) = size(clause, scratch.arena, scratch.bindings, scratch.stack) else {
            scratch.restore(&mark);
            return Err(DiscoveryError::Length);
        };
        length_after = length_after.saturating_add(nodes);
    }
    let valence_q16 = affect.update_valence(free_energy_q16(length_after));
    Ok(Discovery {
        invention,
        length_before,
        length_after,
        valence_q16,
        reward_q16: reward_q16(valence_q16),
    })
}

/// What a search over a clause store came to (ADR-0045).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SearchReport {
    /// Pairs the search tried.
    pub attempts: u32,
    /// Inventions committed: those whose reward was positive.
    pub commits: u32,
    /// Inventions undone: those whose reward was not positive, and pairs the operator
    /// refused for a reason of their own (no common literal, nothing to invent, a bound of
    /// the clause).
    pub rejections: u32,
    /// The store's description length before the first attempt and after the last commit.
    pub length_before: u32,
    pub length_after: u32,
    /// The committed rewards, summed, saturating: what the caller passes to the modulator.
    pub reward_total_q16: i32,
}

/// The executive search over a clause store (ADR-0045): the pairs `next_pair` yields, in
/// order, each intra-constructed by [`invent`]; an invention whose reward is positive is
/// committed (the two inputs replaced by the common clause and the first definition, the
/// second definition appended at `len`, the discovery written to `out`, the walk restarted
/// from the first pair), one whose reward is not positive is undone (the scratch restored to
/// before it, the affect state put back as it was), as is a pair the operator refuses for a
/// reason of its own; the walk ends when no pair is left or `budget` attempts are spent.
/// The affect state must be primed to the store's length, as `invent` requires; after a
/// commit it is primed to the new length by the valence update itself. `StoreFull` when a
/// commit finds no slot, `OutFull` when it finds no room in `out`, either with the invention
/// undone; the operator's own bounds (an arena or a band exhausted, a walk past its limit, a
/// malformed store) end the search as errors with the store as it was before that attempt.
/// Returns the report; `len` is the store's length after.
pub fn search(
    store: &mut [u32],
    len: &mut usize,
    scratch: &mut InduceScratch,
    affect: &mut InteroceptiveState,
    budget: u32,
    out: &mut [Discovery],
) -> Result<SearchReport, DiscoveryError> {
    let mut report = SearchReport {
        length_before: description_length(
            store.get(..*len).ok_or(DiscoveryError::Length)?,
            scratch.arena,
            scratch.bindings,
            scratch.stack,
        )?,
        ..SearchReport::default()
    };
    report.length_after = report.length_before;
    let mut pair = next_pair(&store[..*len], None, scratch)?;
    while let Some((i, j)) = pair {
        if report.attempts >= budget {
            break;
        }
        report.attempts = report.attempts.wrapping_add(1);
        let saved = *affect;
        let mark = scratch.mark();
        match invent(store[i], store[j], &store[..*len], scratch, affect) {
            Ok(discovery) if discovery.reward_q16 > 0 => {
                let Some(slot) = store.get_mut(*len) else {
                    scratch.restore(&mark);
                    *affect = saved;
                    return Err(DiscoveryError::StoreFull);
                };
                let Some(record) = out.get_mut(report.commits as usize) else {
                    scratch.restore(&mark);
                    *affect = saved;
                    return Err(DiscoveryError::OutFull);
                };
                *slot = discovery.invention.definitions[1];
                store[i] = discovery.invention.common;
                store[j] = discovery.invention.definitions[0];
                // Below the store's length, which a slice bounds.
                *len = len.wrapping_add(1);
                *record = discovery;
                report.commits = report.commits.wrapping_add(1);
                report.length_after = discovery.length_after;
                report.reward_total_q16 =
                    report.reward_total_q16.saturating_add(discovery.reward_q16);
                pair = next_pair(&store[..*len], None, scratch)?;
            }
            Ok(_) => {
                scratch.restore(&mark);
                *affect = saved;
                report.rejections = report.rejections.wrapping_add(1);
                pair = next_pair(&store[..*len], Some((i, j)), scratch)?;
            }
            Err(DiscoveryError::Induce(
                InduceError::NoMatch
                | InduceError::NothingToInvent
                | InduceError::TooManyArguments
                | InduceError::BodyFull
                | InduceError::PairsFull,
            )) => {
                // The operator restored the scratch and touched no affect state.
                report.rejections = report.rejections.wrapping_add(1);
                pair = next_pair(&store[..*len], Some((i, j)), scratch)?;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(report)
}

/// The frame a conjecture leaves in: a pending proof check whose parameter hash is the
/// statement's, at the authorization level the veto gate assigned.
pub fn conjecture_frame(
    call_id: u64,
    statement_hash: u32,
    authorization_level: u8,
) -> Option<ToolInvocationFrame> {
    ToolInvocationFrame::new_call(
        call_id,
        TOOL_CATEGORY_FORMAL_PROVER,
        ACTION_VERIFY_PROOF,
        statement_hash,
        authorization_level,
    )
}

/// Certifies `node` with `statement_hash` from a prover's completed frame and returns the
/// certificate hash: the frame must be completed, its category the prover, its action a
/// proof check or a constraint solve, its parameter hash the statement's, and its payload
/// at least `CERTIFICATE_BYTES` with the statement hash first and a non-zero certificate
/// hash second. Every other frame leaves the node untouched.
pub fn certify_from_frame(
    frame: &ToolInvocationFrame,
    statement_hash: u32,
    node: &mut SemanticOntologyNode,
) -> Result<u32, CertifyError> {
    if frame.execution_status != STATUS_COMPLETED {
        return Err(CertifyError::NotCompleted);
    }
    if frame.tool_category != TOOL_CATEGORY_FORMAL_PROVER {
        return Err(CertifyError::NotAProver);
    }
    if !matches!(
        frame.action_opcode,
        ACTION_VERIFY_PROOF | ACTION_SOLVE_CONSTRAINTS
    ) {
        return Err(CertifyError::NotAVerification);
    }
    if frame.param_hash != statement_hash {
        return Err(CertifyError::Mismatch);
    }
    let payload = frame.payload();
    let (Some(statement), Some(certificate)) = (payload.get(0..4), payload.get(4..8)) else {
        return Err(CertifyError::NoCertificate);
    };
    let (Ok(statement), Ok(certificate)) = (
        <[u8; 4]>::try_from(statement),
        <[u8; 4]>::try_from(certificate),
    ) else {
        return Err(CertifyError::NoCertificate);
    };
    let certificate = u32::from_le_bytes(certificate);
    if certificate == 0 {
        return Err(CertifyError::NoCertificate);
    }
    if u32::from_le_bytes(statement) != statement_hash {
        return Err(CertifyError::Mismatch);
    }
    node.certify(statement_hash);
    Ok(certificate)
}

const _: () = {
    // An invented predicate's id never reaches the grammar's role band.
    assert!(INVENTED_LIMIT <= ROLE_CONCEPT_BASE);
    assert!(CERTIFICATE_BYTES <= cortex_tools::PAYLOAD_BYTES);
    assert!(COMPRESSION_REWARD_SHIFT < 32);
};

#[cfg(test)]
mod tests {
    use super::*;
    use cortex_knowledge::AFFORDANCE_CERTIFIED_THEOREM;
    use cortex_reasoning::{INVENTED_BASE, clause};
    use cortex_tools::{
        ACTION_PARSE_STRUCTURE, ACTION_SYMBOLIC_EVAL, PAYLOAD_BYTES, TOOL_CATEGORY_DOC_ENGINE,
    };

    const ONE: i32 = Q16_ONE as i32;

    #[test]
    fn the_reward_is_a_quarter_of_the_valence_clamped() {
        assert_eq!(reward_q16(0), 0);
        assert_eq!(reward_q16(ONE), ONE / 4);
        assert_eq!(reward_q16(3 * ONE), 3 * ONE / 4);
        assert_eq!(reward_q16(4 * ONE), ONE, "four nodes are a full reward");
        assert_eq!(reward_q16(5 * ONE), ONE, "and more is clamped");
        assert_eq!(reward_q16(-ONE), -ONE / 4);
        assert_eq!(reward_q16(-4 * ONE), -ONE);
        assert_eq!(reward_q16(i32::MIN), -ONE);
        assert_eq!(reward_q16(i32::MAX), ONE);
        assert_eq!(
            reward_q16(-1),
            -1,
            "an arithmetic shift keeps a negative negative"
        );
    }

    #[test]
    fn a_length_is_a_free_energy_up_to_the_ceiling() {
        assert_eq!(free_energy_q16(0), 0);
        assert_eq!(free_energy_q16(30), 30 << 16);
        assert_eq!(free_energy_q16(LENGTH_CEILING), 0xFFFF_0000);
        assert_eq!(
            free_energy_q16(LENGTH_CEILING + 1),
            0xFFFF_0000,
            "saturates"
        );
        assert_eq!(free_energy_q16(u32::MAX), 0xFFFF_0000);
        let mut affect = InteroceptiveState::default();
        prime(&mut affect, 30);
        assert_eq!(affect.free_energy_prev_q16, 30 << 16);
        assert_eq!(affect.valence_df_dt_q16, 0, "priming is not an update");
    }

    /// A store of two clauses `p(X) ← r(X), u(X)` and `p(Y) ← r(Y), w(Y)`: one shared
    /// literal, so the invention does not pay (eighteen nodes become nineteen).
    fn small_store(arena: &mut [TermNode]) -> (u32, u32) {
        let nodes = [
            TermNode::variable(0),
            TermNode::compound(0x100, &[0]).unwrap(),
            TermNode::compound(0x101, &[0]).unwrap(),
            TermNode::compound(0x104, &[0]).unwrap(),
            clause(1, &[2, 3]).unwrap(),
            TermNode::variable(1),
            TermNode::compound(0x100, &[5]).unwrap(),
            TermNode::compound(0x101, &[5]).unwrap(),
            TermNode::compound(0x105, &[5]).unwrap(),
            clause(6, &[7, 8]).unwrap(),
        ];
        arena[..nodes.len()].copy_from_slice(&nodes);
        (4, 9)
    }

    #[test]
    fn a_store_is_measured_and_an_invention_that_does_not_pay_is_a_negative_valence() {
        let mut arena = [TermNode::default(); 32];
        let (ca, cb) = small_store(&mut arena);
        let mut bindings = [Binding::UNBOUND; 8];
        let (mut trail, mut stack, mut pairs) = ([0u32; 16], [0u32; 32], [[0u32; 3]; 4]);
        let store = [ca, cb];
        assert_eq!(
            description_length(&store, &arena, &bindings, &mut stack),
            Ok(14)
        );
        assert_eq!(
            description_length(&[], &arena, &bindings, &mut stack),
            Ok(0)
        );
        assert_eq!(
            description_length(&[ca, 99], &arena, &bindings, &mut stack),
            Err(DiscoveryError::Length)
        );
        let mut one = [0u32; 1];
        assert_eq!(
            description_length(&store, &arena, &bindings, &mut one),
            Err(DiscoveryError::Length),
            "a stack too small"
        );
        let mut scratch = InduceScratch {
            arena: &mut arena,
            free: 10,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            pairs: &mut pairs,
            next_variable: 2,
            next_invented: INVENTED_BASE,
        };
        let mut affect = InteroceptiveState::default();
        assert_eq!(
            invent(ca, cb, &store, &mut scratch, &mut affect),
            Err(DiscoveryError::NotPrimed),
            "a fresh affect state reads zero, not fourteen"
        );
        assert_eq!(
            (scratch.free, scratch.trail_len),
            (10, 0),
            "nothing happened"
        );
        prime(&mut affect, 14);
        for wrong in [&[][..], &[ca][..], &[cb][..], &[cb, 99][..]] {
            let mut primed = InteroceptiveState::default();
            prime(&mut primed, 0);
            assert_eq!(
                invent(ca, cb, wrong, &mut scratch, &mut primed),
                Err(DiscoveryError::NotInStore),
                "{wrong:?}"
            );
        }
        assert_eq!(
            invent(ca, ca, &store, &mut scratch, &mut affect),
            Err(DiscoveryError::Induce(InduceError::NothingToInvent)),
            "a clause with itself shares everything"
        );
        assert_eq!(
            affect.valence_df_dt_q16, 0,
            "the affect state is untouched by a refusal"
        );
        assert_eq!(affect.free_energy_prev_q16, 14 << 16);
        let d = invent(ca, cb, &store, &mut scratch, &mut affect).unwrap();
        assert_eq!((d.length_before, d.length_after), (14, 17));
        assert_eq!(d.valence_q16, -3 * ONE, "three nodes longer");
        assert_eq!(d.reward_q16, -3 * ONE / 4);
        assert_eq!(d.invention.predicate, INVENTED_BASE);
        assert_eq!(
            affect.free_energy_prev_q16,
            17 << 16,
            "the next discovery measures from here"
        );
        assert_eq!(affect.valence_df_dt_q16, -3 * ONE);
        assert_eq!(
            invent(
                d.invention.common,
                d.invention.definitions[0],
                &store,
                &mut scratch,
                &mut affect
            ),
            Err(DiscoveryError::NotInStore),
            "the outputs are not in the store"
        );
        assert_eq!(
            invent(ca, cb, &store, &mut scratch, &mut affect),
            Err(DiscoveryError::NotPrimed),
            "the store still reads fourteen, the affect state seventeen"
        );
    }

    #[test]
    fn an_after_measure_that_does_not_fit_restores_the_scratch() {
        // ca = p(W) ← r(a), u(a); cb = p(f(X1..X8)) ← r(a), w(a); c3 = s(W, ..., W), seven
        // times. The heads bind W to f(X1..X8): before, c3 walks with seven entries and cb
        // with ten; after, c3 needs fourteen. A stack of ten measures before, invents, and
        // cannot measure after; the invention is undone. Fourteen entries measure both.
        let mut nodes = [TermNode::default(); 48];
        nodes[0] = TermNode::variable(0);
        nodes[1] = TermNode::constant(0x200);
        nodes[2] = TermNode::compound(0x100, &[0]).unwrap();
        nodes[3] = TermNode::compound(0x101, &[1]).unwrap();
        nodes[4] = TermNode::compound(0x104, &[1]).unwrap();
        nodes[5] = clause(2, &[3, 4]).unwrap();
        for (k, slot) in nodes[6..14].iter_mut().enumerate() {
            *slot = TermNode::variable(k.wrapping_add(1) as u32);
        }
        nodes[14] = TermNode::compound(0x300, &[6, 7, 8, 9, 10, 11, 12, 13]).unwrap();
        nodes[15] = TermNode::compound(0x100, &[14]).unwrap();
        nodes[16] = TermNode::compound(0x105, &[1]).unwrap();
        nodes[17] = clause(15, &[3, 16]).unwrap();
        nodes[18] = TermNode::compound(0x102, &[0; 7]).unwrap();
        nodes[19] = clause(18, &[]).unwrap();
        let (ca, cb, c3) = (5u32, 17u32, 19u32);
        let store = [ca, cb, c3];
        let snapshot = nodes;
        let mut bindings = [Binding::UNBOUND; 12];
        let (mut trail, mut pairs) = ([0u32; 16], [[0u32; 3]; 4]);
        for (entries, expected) in [(10usize, Err(DiscoveryError::Length)), (14, Ok((31, 87)))] {
            let mut stack = [0u32; 14];
            let mut scratch = InduceScratch {
                arena: &mut nodes,
                free: 20,
                bindings: &mut bindings,
                trail: &mut trail,
                trail_len: 0,
                stack: &mut stack[..entries],
                pairs: &mut pairs,
                next_variable: 9,
                next_invented: INVENTED_BASE,
            };
            let mut affect = InteroceptiveState::default();
            prime(&mut affect, 31);
            let outcome = invent(ca, cb, &store, &mut scratch, &mut affect)
                .map(|d| (d.length_before, d.length_after));
            assert_eq!(outcome, expected, "{entries} entries");
            if outcome.is_err() {
                assert_eq!(
                    (scratch.free, scratch.trail_len),
                    (20, 0),
                    "the invention undone"
                );
                assert_eq!(scratch.next_invented, INVENTED_BASE);
                assert!(scratch.bindings.iter().all(|&b| b == Binding::UNBOUND));
                assert_eq!(&scratch.arena[..], &snapshot[..]);
                assert_eq!(affect.valence_df_dt_q16, 0, "the affect state untouched");
                assert_eq!(affect.free_energy_prev_q16, 31 << 16);
            } else {
                assert_eq!(
                    affect.valence_df_dt_q16,
                    -56 * ONE,
                    "thirty-one nodes became eighty-seven"
                );
            }
        }
    }

    #[test]
    fn a_conjecture_frame_is_a_pending_proof_check_with_the_statement_as_its_parameter() {
        let frame = conjecture_frame(7, 0xABCD, 3).unwrap();
        assert_eq!(frame.call_id, 7);
        assert_eq!(frame.tool_category, TOOL_CATEGORY_FORMAL_PROVER);
        assert_eq!(frame.action_opcode, ACTION_VERIFY_PROOF);
        assert_eq!(frame.param_hash, 0xABCD);
        assert_eq!(frame.authorization_level, 3);
        assert_eq!(frame.execution_status, cortex_tools::STATUS_PENDING);
    }

    fn completed(
        category: u16,
        opcode: u16,
        param_hash: u32,
        payload: &[u8],
    ) -> ToolInvocationFrame {
        let mut frame = ToolInvocationFrame::new_call(1, category, opcode, param_hash, 1).unwrap();
        assert!(frame.start());
        assert!(frame.complete(payload));
        frame
    }

    fn payload(statement: u32, certificate: u32) -> [u8; 8] {
        let mut out = [0u8; 8];
        out[..4].copy_from_slice(&statement.to_le_bytes());
        out[4..].copy_from_slice(&certificate.to_le_bytes());
        out
    }

    #[test]
    fn a_completed_prover_frame_certifies_and_every_other_frame_does_not() {
        const STATEMENT: u32 = 0x21a1_c619;
        const CERTIFICATE: u32 = 0xC0FF_EE01;
        let fresh = SemanticOntologyNode::default();
        let mut node = fresh;
        let good = completed(
            TOOL_CATEGORY_FORMAL_PROVER,
            ACTION_VERIFY_PROOF,
            STATEMENT,
            &payload(STATEMENT, CERTIFICATE),
        );
        assert_eq!(
            certify_from_frame(&good, STATEMENT, &mut node),
            Ok(CERTIFICATE)
        );
        assert!(node.is_certified_theorem());
        assert_eq!(
            node.property_vector_hash, STATEMENT,
            "the node holds the statement's hash"
        );
        assert_eq!(node.consolidation_count, 1);
        assert_eq!(node.affordance_action_mask, AFFORDANCE_CERTIFIED_THEOREM);
        let solved = completed(
            TOOL_CATEGORY_FORMAL_PROVER,
            ACTION_SOLVE_CONSTRAINTS,
            STATEMENT,
            &payload(STATEMENT, CERTIFICATE),
        );
        let mut node = fresh;
        assert_eq!(
            certify_from_frame(&solved, STATEMENT, &mut node),
            Ok(CERTIFICATE)
        );
        let mut longer = payload(STATEMENT, CERTIFICATE).to_vec();
        longer.extend_from_slice(&[9; PAYLOAD_BYTES - 8]);
        let padded = completed(
            TOOL_CATEGORY_FORMAL_PROVER,
            ACTION_VERIFY_PROOF,
            STATEMENT,
            &longer,
        );
        let mut node = fresh;
        assert_eq!(
            certify_from_frame(&padded, STATEMENT, &mut node),
            Ok(CERTIFICATE),
            "more bytes are ignored"
        );

        let refusals: [(ToolInvocationFrame, CertifyError); 9] = [
            (
                conjecture_frame(1, STATEMENT, 1).unwrap(),
                CertifyError::NotCompleted,
            ),
            (
                {
                    let mut f = conjecture_frame(1, STATEMENT, 1).unwrap();
                    assert!(f.start());
                    f
                },
                CertifyError::NotCompleted,
            ),
            (
                {
                    let mut f = conjecture_frame(1, STATEMENT, 1).unwrap();
                    assert!(f.start() && f.fail());
                    f
                },
                CertifyError::NotCompleted,
            ),
            (
                {
                    let mut f = conjecture_frame(1, STATEMENT, 1).unwrap();
                    assert!(f.deny());
                    f
                },
                CertifyError::NotCompleted,
            ),
            (
                completed(
                    TOOL_CATEGORY_DOC_ENGINE,
                    ACTION_PARSE_STRUCTURE,
                    STATEMENT,
                    &payload(STATEMENT, CERTIFICATE),
                ),
                CertifyError::NotAProver,
            ),
            (
                completed(
                    TOOL_CATEGORY_FORMAL_PROVER,
                    ACTION_SYMBOLIC_EVAL,
                    STATEMENT,
                    &payload(STATEMENT, CERTIFICATE),
                ),
                CertifyError::NotAVerification,
            ),
            (
                completed(
                    TOOL_CATEGORY_FORMAL_PROVER,
                    ACTION_VERIFY_PROOF,
                    STATEMENT ^ 1,
                    &payload(STATEMENT, CERTIFICATE),
                ),
                CertifyError::Mismatch,
            ),
            (
                completed(
                    TOOL_CATEGORY_FORMAL_PROVER,
                    ACTION_VERIFY_PROOF,
                    STATEMENT,
                    &payload(STATEMENT ^ 1, CERTIFICATE),
                ),
                CertifyError::Mismatch,
            ),
            (
                completed(
                    TOOL_CATEGORY_FORMAL_PROVER,
                    ACTION_VERIFY_PROOF,
                    STATEMENT,
                    &payload(STATEMENT, 0),
                ),
                CertifyError::NoCertificate,
            ),
        ];
        for (frame, expected) in refusals {
            let mut node = fresh;
            assert_eq!(
                certify_from_frame(&frame, STATEMENT, &mut node),
                Err(expected)
            );
            assert_eq!(node, fresh, "{expected:?} leaves the node untouched");
        }
        for short in [0usize, 3, 4, 7] {
            let frame = completed(
                TOOL_CATEGORY_FORMAL_PROVER,
                ACTION_VERIFY_PROOF,
                STATEMENT,
                &payload(STATEMENT, CERTIFICATE)[..short],
            );
            let mut node = fresh;
            assert_eq!(
                certify_from_frame(&frame, STATEMENT, &mut node),
                Err(CertifyError::NoCertificate),
                "{short} bytes"
            );
            assert_eq!(node, fresh);
        }
    }

    /// `p(X) ← a(X), b(X), c(X), d(X), l(X)` over a fresh variable, `l` the last literal.
    fn wide(arena: &mut [TermNode], free: &mut usize, next_var: &mut u32, last: u32) -> u32 {
        let base = *free as u32;
        let at = |k: u32| base.wrapping_add(k);
        let nodes = [
            TermNode::variable(*next_var),
            TermNode::compound(0x100, &[base]).unwrap(),
            TermNode::compound(0x200, &[base]).unwrap(),
            TermNode::compound(0x201, &[base]).unwrap(),
            TermNode::compound(0x202, &[base]).unwrap(),
            TermNode::compound(0x203, &[base]).unwrap(),
            TermNode::compound(last, &[base]).unwrap(),
            clause(at(1), &[at(2), at(3), at(4), at(5), at(6)]).unwrap(),
        ];
        let end = free.wrapping_add(nodes.len());
        arena[*free..end].copy_from_slice(&nodes);
        *free = end;
        *next_var = next_var.wrapping_add(1);
        at(7)
    }

    #[test]
    fn the_search_commits_the_inventions_that_pay_restores_the_rest_and_stops_at_its_bounds() {
        let mut arena = [TermNode::default(); 512];
        let (mut free, mut vars) = (0usize, 0u32);
        // Three clauses of one head sharing four literals, and one of another head.
        let c0 = wide(&mut arena, &mut free, &mut vars, 0x204);
        let c1 = wide(&mut arena, &mut free, &mut vars, 0x205);
        let c2 = wide(&mut arena, &mut free, &mut vars, 0x206);
        let base = free as u32;
        let at = |k: u32| base.wrapping_add(k);
        arena[free] = TermNode::variable(vars);
        arena[free.wrapping_add(1)] = TermNode::compound(0x101, &[base]).unwrap();
        arena[free.wrapping_add(2)] = TermNode::compound(0x200, &[base]).unwrap();
        arena[free.wrapping_add(3)] = TermNode::compound(0x207, &[base]).unwrap();
        arena[free.wrapping_add(4)] = clause(at(1), &[at(2), at(3)]).unwrap();
        let c3 = at(4);
        free = free.wrapping_add(5);
        vars = vars.wrapping_add(1);
        let mut bindings = [Binding::UNBOUND; 64];
        let (mut trail, mut stack, mut pairs) = ([0u32; 128], [0u32; 256], [[0u32; 3]; 32]);
        let mut scratch = InduceScratch {
            arena: &mut arena,
            free,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            pairs: &mut pairs,
            next_variable: vars,
            next_invented: INVENTED_BASE,
        };
        let mut store = [c0, c1, c2, c3, 0, 0, 0, 0];
        let mut len = 4;
        let mut affect = InteroceptiveState::default();
        // Each wide clause is 13 nodes (the clause, the head of two, five literals of two),
        // the narrow one 7: forty-six in all. Two inventions of four shared literals save
        // three nodes each: the common clause of thirteen and two definitions of five
        // against two of thirteen.
        prime(&mut affect, 46);
        let mut out = [Discovery::default(); 4];
        let report = search(
            &mut store,
            &mut len,
            &mut scratch,
            &mut affect,
            16,
            &mut out,
        )
        .unwrap();
        assert_eq!(
            report,
            SearchReport {
                attempts: 2,
                commits: 2,
                rejections: 0,
                length_before: 46,
                length_after: 40,
                reward_total_q16: 3 * ONE / 2,
            },
            "two commits of three nodes each, a reward of three quarters each"
        );
        assert_eq!(
            len, 6,
            "two inputs became three outputs, twice: four clauses become six"
        );
        assert_eq!(out[0].invention.predicate, INVENTED_BASE);
        assert_eq!(out[1].invention.predicate, INVENTED_BASE + 1);
        assert_eq!((out[0].length_before, out[0].length_after), (46, 43));
        assert_eq!((out[1].length_before, out[1].length_after), (43, 40));
        assert_eq!(
            affect.free_energy_prev_q16,
            40 << 16,
            "primed to the store after"
        );
        assert_eq!(
            description_length(
                &store[..len],
                scratch.arena,
                scratch.bindings,
                scratch.stack
            ),
            Ok(40)
        );
        // The store's shape after: the common clause at 0, the first definition at 1, the
        // second appended; then the second invention over the common clause and clause 2.
        assert_eq!(store[0], out[1].invention.common);
        assert_eq!(store[1], out[0].invention.definitions[0]);
        assert_eq!(store[4], out[0].invention.definitions[1]);
        assert_eq!(store[2], out[1].invention.definitions[0]);
        assert_eq!(store[5], out[1].invention.definitions[1]);
        assert_eq!(store[3], c3, "the other head's clause stays");
        // Nothing left to pair: a second search changes nothing.
        let again = search(
            &mut store,
            &mut len,
            &mut scratch,
            &mut affect,
            16,
            &mut out,
        )
        .unwrap();
        assert_eq!(
            (again.attempts, again.commits, again.length_after),
            (0, 0, 40)
        );
        assert_eq!(len, 6);
    }

    #[test]
    fn the_search_undoes_an_invention_that_does_not_pay_and_refuses_what_it_cannot_hold() {
        let mut arena = [TermNode::default(); 64];
        let (ca, cb) = small_store(&mut arena);
        let mut bindings = [Binding::UNBOUND; 16];
        let (mut trail, mut stack, mut pairs) = ([0u32; 32], [0u32; 64], [[0u32; 3]; 8]);
        let mut scratch = InduceScratch {
            arena: &mut arena,
            free: 10,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            pairs: &mut pairs,
            next_variable: 2,
            next_invented: INVENTED_BASE,
        };
        let mut affect = InteroceptiveState::default();
        prime(&mut affect, 14);
        let mut store = [ca, cb, 0];
        let mut len = 2;
        let mut out = [Discovery::default(); 1];
        // One shared literal: fourteen become seventeen, undone.
        let report = search(
            &mut store,
            &mut len,
            &mut scratch,
            &mut affect,
            16,
            &mut out,
        )
        .unwrap();
        assert_eq!(
            report,
            SearchReport {
                attempts: 1,
                commits: 0,
                rejections: 1,
                length_before: 14,
                length_after: 14,
                reward_total_q16: 0,
            }
        );
        assert_eq!(
            (len, scratch.free, scratch.trail_len, scratch.next_invented),
            (2, 10, 0, INVENTED_BASE)
        );
        assert_eq!(
            affect,
            {
                let mut a = InteroceptiveState::default();
                prime(&mut a, 14);
                a
            },
            "the affect state as it was"
        );
        assert_eq!(out[0], Discovery::default(), "nothing written");
        // A budget of zero tries nothing; an unprimed state is the operator's refusal.
        let report = search(&mut store, &mut len, &mut scratch, &mut affect, 0, &mut out).unwrap();
        assert_eq!((report.attempts, report.length_before), (0, 14));
        prime(&mut affect, 1);
        assert_eq!(
            search(
                &mut store,
                &mut len,
                &mut scratch,
                &mut affect,
                16,
                &mut out
            ),
            Err(DiscoveryError::NotPrimed)
        );
        // A length past the store is a length error.
        prime(&mut affect, 14);
        let mut too_long = 4;
        assert_eq!(
            search(
                &mut store,
                &mut too_long,
                &mut scratch,
                &mut affect,
                16,
                &mut out
            ),
            Err(DiscoveryError::Length)
        );
    }

    #[test]
    fn a_commit_with_no_slot_in_the_store_or_in_the_output_is_refused_and_undone() {
        let mut arena = [TermNode::default(); 256];
        let (mut free, mut vars) = (0usize, 0u32);
        let c0 = wide(&mut arena, &mut free, &mut vars, 0x204);
        let c1 = wide(&mut arena, &mut free, &mut vars, 0x205);
        let mut bindings = [Binding::UNBOUND; 32];
        let (mut trail, mut stack, mut pairs) = ([0u32; 64], [0u32; 128], [[0u32; 3]; 16]);
        let mut scratch = InduceScratch {
            arena: &mut arena,
            free,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            pairs: &mut pairs,
            next_variable: vars,
            next_invented: INVENTED_BASE,
        };
        let mut affect = InteroceptiveState::default();
        prime(&mut affect, 26);
        let mut out = [Discovery::default(); 1];
        let mut full = [c0, c1];
        let mut len = 2;
        assert_eq!(
            search(&mut full, &mut len, &mut scratch, &mut affect, 16, &mut out),
            Err(DiscoveryError::StoreFull)
        );
        assert_eq!(
            (len, scratch.free, scratch.next_invented),
            (2, free, INVENTED_BASE),
            "undone"
        );
        assert_eq!(affect.free_energy_prev_q16, 26 << 16);
        let mut room = [c0, c1, 0];
        let mut none: [Discovery; 0] = [];
        assert_eq!(
            search(
                &mut room,
                &mut len,
                &mut scratch,
                &mut affect,
                16,
                &mut none
            ),
            Err(DiscoveryError::OutFull)
        );
        assert_eq!((len, scratch.free, room[2]), (2, free, 0), "undone");
        let report = search(&mut room, &mut len, &mut scratch, &mut affect, 16, &mut out).unwrap();
        assert_eq!((report.commits, len), (1, 3));
    }

    /// Two clauses `p(X) ← a(X, c), b(X), e(X)` and `p(Y) ← a(Y, c), b(Y), f(Y)`: the
    /// invention saves nothing (twenty nodes become twenty), so its reward is zero and the
    /// search undoes it: a commit needs a reward above zero, not at it.
    #[test]
    fn an_invention_that_saves_nothing_is_not_committed() {
        let mut arena = [TermNode::default(); 64];
        let nodes = [
            TermNode::variable(0),
            TermNode::constant(0x300),
            TermNode::compound(0x200, &[0, 1]).unwrap(),
            TermNode::compound(0x201, &[0]).unwrap(),
            TermNode::compound(0x204, &[0]).unwrap(),
            TermNode::compound(0x100, &[0]).unwrap(),
            clause(5, &[2, 3, 4]).unwrap(),
            TermNode::variable(1),
            TermNode::constant(0x300),
            TermNode::compound(0x200, &[7, 8]).unwrap(),
            TermNode::compound(0x201, &[7]).unwrap(),
            TermNode::compound(0x205, &[7]).unwrap(),
            TermNode::compound(0x100, &[7]).unwrap(),
            clause(12, &[9, 10, 11]).unwrap(),
        ];
        arena[..nodes.len()].copy_from_slice(&nodes);
        let mut bindings = [Binding::UNBOUND; 16];
        let (mut trail, mut stack, mut pairs) = ([0u32; 32], [0u32; 64], [[0u32; 3]; 8]);
        let mut scratch = InduceScratch {
            arena: &mut arena,
            free: nodes.len(),
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            pairs: &mut pairs,
            next_variable: 2,
            next_invented: INVENTED_BASE,
        };
        let mut store = [6u32, 13, 0];
        let mut len = 2;
        assert_eq!(
            description_length(
                &store[..len],
                scratch.arena,
                scratch.bindings,
                scratch.stack
            ),
            Ok(20)
        );
        let mut affect = InteroceptiveState::default();
        prime(&mut affect, 20);
        let mut out = [Discovery::default(); 1];
        let report = search(
            &mut store,
            &mut len,
            &mut scratch,
            &mut affect,
            16,
            &mut out,
        )
        .unwrap();
        assert_eq!(
            (
                report.attempts,
                report.commits,
                report.rejections,
                report.length_after
            ),
            (1, 0, 1, 20)
        );
        assert_eq!(
            (len, scratch.free, scratch.next_invented),
            (2, nodes.len(), INVENTED_BASE)
        );
        assert_eq!(out[0], Discovery::default());
    }
}
