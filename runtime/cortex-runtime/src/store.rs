//! The engine's term arena and clause store (whitepaper §5.2.30, §6.10; ADR-0052): what the
//! executor owns so that the discovery loop of ADR-0045 runs inside the tick without a
//! caller and the image carries it. The nodes, the store's indices, the induction record of
//! `cortex-reasoning`, the affect state of `cortex-affect` and the scratch the operators
//! need (the binding table, the trail, the work stack, the pair table) are allocated once in
//! `Executor::new`, sized from the arena's capacity; the inputs that build a term and assert
//! a clause run between ticks; a search runs from the record's cursor with the record's
//! budget, so that a bounded budget makes progress over the pairs instead of repeating its
//! first ones; a commit's outputs are instantiated through the bindings and the table
//! unbound, so that between searches the table is empty and the store reads without it.
//! The affect state is primed to the store's description length at every change (ADR-0043),
//! so the length is read from it and kept nowhere else. Since ADR-0056 the arena is
//! compacted onto the store's clauses, between ticks on a call and inside the tick at every
//! slow-wave onset: the nodes the store does not reach (a commit's replaced inputs, the
//! operators' intermediate nodes) are reclaimed, the store's indices and the record's
//! cursor moved, the store reading the same node for node. Nothing here allocates after
//! `new`.

use crate::discovery::{
    Discovery, DiscoveryError, SearchReport, description_length, free_energy_q16, prime,
    search_from,
};
use cortex_affect::InteroceptiveState;
use cortex_reasoning::{
    Binding, Compaction, InduceScratch, InductionState, TERM_EMPTY, TERM_VARIABLE, TermNode,
    clause, compact, is_clause, size,
};

/// Why a term or a clause was refused (ADR-0052). Nothing changes on a refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TermError {
    /// The engine keeps no arena (`Config::terms` is 0).
    NoArena,
    /// The arena has no free node.
    ArenaFull,
    /// The store has no free slot (`Config::clauses`).
    StoreFull,
    /// The node is not well formed, is empty, names a child at or beyond the arena's cursor
    /// or is a variable outside the binding table; or a clause would hold more literals than
    /// `MAX_BODY`.
    Malformed,
    /// The clause's size could not be measured (a walk past its bound), so the store's
    /// length could not be primed.
    Unmeasurable,
}

/// The executor's induction state: the arena, the store, the scratch, the two records and
/// the loop's counters.
pub(crate) struct Induction {
    terms: Vec<TermNode>,
    store: Vec<u32>,
    bindings: Vec<Binding>,
    trail: Vec<u32>,
    stack: Vec<u32>,
    pairs: Vec<[u32; 3]>,
    /// The engine's affect state (ADR-0043): primed to the store's length; the valence rule
    /// reads the drop.
    pub(crate) affect: InteroceptiveState,
    /// The engine's induction record (ADR-0052).
    pub(crate) record: InductionState,
    /// The discoveries of the last search, one slot per store slot: a commit needs a store
    /// slot, so the buffer never fills before the store does.
    discoveries: Vec<Discovery>,
    /// How many of them the last search committed; none after a search that ended in an
    /// error, whose commits stand in the store without a report.
    last_commits: usize,
    /// Searches run, inside the tick and between ticks.
    pub(crate) searches: u64,
    /// Inventions committed by them.
    pub(crate) inventions: u64,
    /// Rewarded searches whose moment the ledger or the train refused to tag.
    pub(crate) untagged: u64,
    /// Searches that ended in an error of their own (the store or the arena full, a bound).
    pub(crate) failures: u64,
    /// Compactions run, between ticks and at slow-wave onsets (ADR-0056).
    pub(crate) compactions: u64,
    /// Nodes the compactions reclaimed.
    pub(crate) reclaimed: u64,
}

impl Induction {
    /// Every buffer allocated once: the arena of `terms` nodes, the store of `clauses` slots,
    /// a binding table and a trail of the arena's size, a work stack of twice it (two entries
    /// per pending pair of unification) and a pair table of it.
    pub(crate) fn new(
        terms: usize,
        clauses: usize,
        search_shift: u8,
        search_budget: u32,
        tag: u8,
    ) -> Self {
        Self {
            terms: vec![TermNode::default(); terms],
            store: vec![0; clauses],
            bindings: vec![Binding::UNBOUND; terms],
            trail: vec![0; terms],
            stack: vec![0; terms.saturating_mul(2)],
            pairs: vec![[0; 3]; terms],
            affect: InteroceptiveState::default(),
            record: InductionState {
                search_shift,
                search_budget,
                tag,
                ..InductionState::new()
            },
            discoveries: vec![Discovery::default(); clauses],
            last_commits: 0,
            searches: 0,
            inventions: 0,
            untagged: 0,
            failures: 0,
            compactions: 0,
            reclaimed: 0,
        }
    }

    /// The arena's nodes in use.
    pub(crate) fn terms(&self) -> &[TermNode] {
        let free = (self.record.free as usize).min(self.terms.len());
        &self.terms[..free]
    }

    /// The store's clauses, as arena indices, in order.
    pub(crate) fn clauses(&self) -> &[u32] {
        let len = (self.record.clauses as usize).min(self.store.len());
        &self.store[..len]
    }

    /// The arena's and the store's capacities.
    pub(crate) fn capacity(&self) -> (usize, usize) {
        (self.terms.len(), self.store.len())
    }

    /// The store's description length in nodes, as the affect state is primed to it: read
    /// from there and never kept twice.
    fn length(&self) -> u32 {
        self.affect.free_energy_prev_q16 >> 16
    }

    /// The last search's commits, in order: what it invented, at what lengths, for what
    /// reward.
    pub(crate) fn discoveries(&self) -> &[Discovery] {
        &self.discoveries[..self.last_commits.min(self.discoveries.len())]
    }

    /// The discovery of the last search's first commit, if it committed one.
    pub(crate) fn first_invention(&self) -> Option<&Discovery> {
        self.discoveries().first()
    }

    /// The checks a node passes into the arena: well formed and not empty, every child below
    /// `free` (a host builds bottom-up, so the arena stays acyclic), a variable's number
    /// inside the binding table.
    fn admits(&self, node: &TermNode, free: usize) -> bool {
        node.is_well_formed()
            && node.kind != TERM_EMPTY
            && (node.kind != TERM_VARIABLE || (node.functor as usize) < self.bindings.len())
            && (0..node.arity as usize)
                .all(|slot| matches!(node.child(slot), Some(child) if (child as usize) < free))
    }

    /// Appends `node` at the arena's cursor and returns its index. Refused for an engine
    /// without an arena, a full arena and a node `admits` refuses. A variable moves
    /// `next_variable` above its number, so that the operators never reuse it.
    pub(crate) fn term(&mut self, node: TermNode) -> Result<u32, TermError> {
        if self.terms.is_empty() {
            return Err(TermError::NoArena);
        }
        let free = self.record.free as usize;
        if !self.admits(&node, free) {
            return Err(TermError::Malformed);
        }
        let Some(slot) = self.terms.get_mut(free) else {
            return Err(TermError::ArenaFull);
        };
        *slot = node;
        // Below the arena's length, which `Executor::new` bounded below `u32::MAX`.
        self.record.free = self.record.free.wrapping_add(1);
        if node.kind == TERM_VARIABLE {
            // Below the table's length, which is below `u32::MAX`: the step cannot wrap.
            self.record.next_variable = self.record.next_variable.max(node.functor.wrapping_add(1));
        }
        Ok(free as u32)
    }

    /// The clause `head ← body` into the arena and its index into the store, the affect
    /// state primed to the store's new length. Refused for a full store (before anything
    /// is written), for a body longer than `MAX_BODY` or a term `term` refuses, and for a
    /// clause whose size cannot be measured (then the node is taken back).
    pub(crate) fn assert_clause(&mut self, head: u32, body: &[u32]) -> Result<u32, TermError> {
        if self.terms.is_empty() {
            return Err(TermError::NoArena);
        }
        let len = self.record.clauses as usize;
        if len >= self.store.len() {
            return Err(TermError::StoreFull);
        }
        let node = clause(head, body).ok_or(TermError::Malformed)?;
        let index = self.term(node)?;
        let Ok(nodes) = size(index, &self.terms, &self.bindings, &mut self.stack) else {
            // The node is taken back: `term` appended it at the cursor and moved the cursor.
            self.terms[index as usize] = TermNode::default();
            self.record.free = index;
            return Err(TermError::Unmeasurable);
        };
        self.store[len] = index;
        // Below the store's length, which `Executor::new` bounded below `u32::MAX`.
        self.record.clauses = self.record.clauses.wrapping_add(1);
        let length = self.length().saturating_add(nodes);
        prime(&mut self.affect, length);
        Ok(index)
    }

    /// One search over the store from the record's cursor with the record's budget
    /// (ADR-0045, ADR-0052): the commits stand whatever the outcome, the cursor and the
    /// arena's counters move with them, and the affect state is primed to the store's
    /// length after. The cursor is the pair after which the next search resumes, or the
    /// start after a commit, a complete pass or an error.
    pub(crate) fn search(&mut self) -> Result<SearchReport, DiscoveryError> {
        let after = self.record.resume();
        let mut len = (self.record.clauses as usize).min(self.store.len());
        let mut scratch = InduceScratch {
            arena: &mut self.terms,
            free: self.record.free as usize,
            bindings: &mut self.bindings,
            trail: &mut self.trail,
            trail_len: 0,
            stack: &mut self.stack,
            pairs: &mut self.pairs,
            next_variable: self.record.next_variable,
            next_invented: self.record.next_invented,
        };
        let out = search_from(
            &mut self.store,
            &mut len,
            &mut scratch,
            &mut self.affect,
            self.record.search_budget,
            after,
            &mut self.discoveries,
        );
        // Below the arena's length, which `Executor::new` bounded below `u32::MAX`.
        self.record.free = scratch.free as u32;
        self.record.next_variable = scratch.next_variable;
        self.record.next_invented = scratch.next_invented;
        self.record.clauses = len as u32;
        let (report, resume) = match out {
            Ok(found) => found,
            Err(e) => {
                self.last_commits = 0;
                self.record.set_resume(None);
                return Err(e);
            }
        };
        self.last_commits = report.commits as usize;
        // A pair the search returns is one of the store's, `i < j` below its length: the
        // cursor takes it; a refusal here would be a bug of the search, and the start is
        // the safe cursor either way.
        if !self.record.set_resume(resume) {
            self.record.set_resume(None);
        }
        Ok(report)
    }

    /// A compaction of the arena onto the store's clauses (ADR-0056): the nodes no clause
    /// reaches are reclaimed, the store's indices and the record's cursor moved, the last
    /// search's discoveries cleared (their indices moved), the counters stepped. The store
    /// reads the same node for node, so the affect state, primed to its description
    /// length, stands, and the search's cursor, which names store positions, stands too.
    /// The work stack is the rule's scratch: twice the arena's size, and empty between
    /// walks. `NoArena` for an engine without one; `Malformed` if a binding is bound (the
    /// table is empty between searches, and a binding names an index, so a bound one would
    /// name a moved node) or if the rule refuses, which the engine's own arena, bottom-up by
    /// construction, cannot make it do.
    pub(crate) fn compact(&mut self) -> Result<Compaction, TermError> {
        if self.terms.is_empty() {
            return Err(TermError::NoArena);
        }
        if self.bindings.iter().any(|b| *b != Binding::UNBOUND) {
            return Err(TermError::Malformed);
        }
        let free = (self.record.free as usize).min(self.terms.len());
        let len = (self.record.clauses as usize).min(self.store.len());
        let report = compact(
            &mut self.terms,
            free,
            &mut self.store[..len],
            &mut self.stack,
        )
        .map_err(|_| TermError::Malformed)?;
        self.record.free = report.live;
        self.last_commits = 0;
        self.compactions = self.compactions.saturating_add(1);
        self.reclaimed = self.reclaimed.saturating_add(u64::from(report.reclaimed));
        Ok(report)
    }

    /// The loader's: a node from an image, appended at the cursor. Refused as `term` refuses,
    /// so that no loaded child points at or beyond its parent and no loaded arena is cyclic.
    pub(crate) fn load_term(&mut self, node: TermNode) -> bool {
        self.term(node).is_ok()
    }

    /// The loader's: a store index from an image, appended to the store. Refused for an
    /// index at or beyond the cursor, a node that is not a clause, an index the store
    /// already holds, or a full store.
    pub(crate) fn load_clause(&mut self, index: u32) -> bool {
        let len = (self.record.clauses as usize).min(self.store.len());
        let is_clause_node = self.terms().get(index as usize).is_some_and(is_clause);
        if !is_clause_node || self.store[..len].contains(&index) || len >= self.store.len() {
            return false;
        }
        self.store[len] = index;
        self.record.clauses = self.record.clauses.wrapping_add(1);
        true
    }

    /// The loader's: the induction record an image holds, after the arena and the store were
    /// loaded. Refused for a record that is not well formed, whose cursor and length are not
    /// the loaded counts, or whose next variable is at or below a loaded variable's number
    /// (the operators would reuse it).
    pub(crate) fn set_record(&mut self, record: InductionState) -> bool {
        let variables_below = self
            .terms()
            .iter()
            .filter(|node| node.kind == TERM_VARIABLE)
            .all(|node| node.functor < record.next_variable);
        if !record.is_well_formed()
            || record.free != self.record.free
            || record.clauses != self.record.clauses
            || !variables_below
        {
            return false;
        }
        self.record = record;
        true
    }

    /// The loader's: the affect state an image holds. Refused for a state that is not well
    /// formed or whose free energy is not the store's description length (the invariant
    /// `prime` keeps, which the next search requires).
    pub(crate) fn set_affect(&mut self, state: InteroceptiveState) -> bool {
        let len = (self.record.clauses as usize).min(self.store.len());
        let length = description_length(
            &self.store[..len],
            &self.terms,
            &self.bindings,
            &mut self.stack,
        );
        let primed = length.is_ok_and(|nodes| state.free_energy_prev_q16 == free_energy_q16(nodes));
        if !state.is_well_formed() || !primed {
            return false;
        }
        self.affect = state;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortex_reasoning::{INVENTED_BASE, MAX_BODY};

    #[test]
    fn a_term_is_admitted_bottom_up_within_the_table_and_a_clause_primes_the_affect_state() {
        let mut none = Induction::new(0, 0, 0, 0, 0);
        assert_eq!(none.term(TermNode::constant(1)), Err(TermError::NoArena));
        assert_eq!(none.assert_clause(0, &[]), Err(TermError::NoArena));
        assert!(none.terms().is_empty() && none.clauses().is_empty());
        let mut ind = Induction::new(8, 2, 0, 0, 3);
        assert_eq!(ind.capacity(), (8, 2));
        let a = ind.term(TermNode::constant(0x200)).unwrap();
        let x = ind.term(TermNode::variable(5)).unwrap();
        assert_eq!((a, x), (0, 1));
        assert_eq!(ind.record.next_variable, 6, "above the host's variable");
        assert_eq!(
            ind.term(TermNode::variable(8)),
            Err(TermError::Malformed),
            "outside a table of eight"
        );
        assert_eq!(ind.term(TermNode::default()), Err(TermError::Malformed));
        assert_eq!(
            ind.term(TermNode::compound(0x100, &[2]).unwrap()),
            Err(TermError::Malformed),
            "a child at the cursor"
        );
        let mut bad = TermNode::constant(1);
        bad._pad = 1;
        assert_eq!(ind.term(bad), Err(TermError::Malformed));
        let px = ind.term(TermNode::compound(0x100, &[x]).unwrap()).unwrap();
        let qa = ind.term(TermNode::compound(0x101, &[a]).unwrap()).unwrap();
        assert_eq!(ind.length(), 0, "no clause: nothing primed");
        let c1 = ind.assert_clause(px, &[qa]).unwrap();
        assert_eq!(ind.clauses(), &[c1]);
        assert_eq!(ind.length(), 5, "the clause, p, x, q, a");
        assert_eq!(ind.affect.free_energy_prev_q16, 5 << 16);
        assert_eq!(
            ind.assert_clause(qa, &[px; MAX_BODY + 1]),
            Err(TermError::Malformed)
        );
        let c2 = ind.assert_clause(qa, &[]).unwrap();
        assert_eq!(ind.length(), 5 + 3, "the fact: the clause, q, a");
        assert_eq!(ind.clauses(), &[c1, c2]);
        assert_eq!(
            ind.assert_clause(qa, &[]),
            Err(TermError::StoreFull),
            "before anything is written"
        );
        assert_eq!(ind.terms().len(), 6);
        ind.record.clauses = 0;
        for _ in 0..2 {
            ind.term(TermNode::constant(9)).unwrap();
        }
        assert_eq!(ind.term(TermNode::constant(9)), Err(TermError::ArenaFull));
        assert_eq!(ind.assert_clause(qa, &[]), Err(TermError::ArenaFull));
    }

    #[test]
    fn the_loader_s_hooks_refuse_what_the_record_and_the_store_could_not_hold() {
        let mut ind = Induction::new(8, 2, 0, 0, 3);
        assert!(ind.load_term(TermNode::constant(1)));
        assert!(ind.load_term(TermNode::variable(2)));
        assert!(
            !ind.load_term(TermNode::compound(5, &[2]).unwrap()),
            "a child at its own index"
        );
        assert!(ind.load_term(TermNode::compound(5, &[0]).unwrap()));
        assert!(ind.load_term(clause(2, &[]).unwrap()));
        assert!(!ind.load_clause(2), "not a clause");
        assert!(!ind.load_clause(4), "at the cursor");
        assert!(ind.load_clause(3));
        assert!(!ind.load_clause(3), "twice");
        assert!(ind.load_term(clause(2, &[2]).unwrap()));
        assert!(ind.load_clause(4));
        assert!(!ind.load_clause(4), "the store is full");
        let good = InductionState {
            free: 5,
            next_variable: 3,
            next_invented: INVENTED_BASE + 1,
            clauses: 2,
            search_budget: 8,
            search_shift: 4,
            tag: 9,
            ..InductionState::new()
        };
        for (what, patch) in [
            (
                "a cursor that is not the arena's",
                (|r| r.free = 4) as fn(&mut InductionState),
            ),
            ("a length that is not the store's", |r| r.clauses = 1),
            ("a next variable at a loaded one", |r| r.next_variable = 2),
            ("a record that is not well formed", |r| r._pad = 1),
        ] {
            let mut bad = good;
            patch(&mut bad);
            assert!(!ind.set_record(bad), "{what}");
        }
        assert!(ind.set_record(good));
        assert_eq!(ind.record, good);
        // The affect state must be primed to the store's length: the fact is three nodes
        // (the clause, the literal, its constant) and the rule five (the clause, then the
        // shared literal and its constant twice, once as the head and once as the body).
        let mut affect = InteroceptiveState::default();
        assert!(!ind.set_affect(affect), "not primed");
        prime(&mut affect, 7);
        assert!(!ind.set_affect(affect), "primed to another length");
        prime(&mut affect, 8);
        affect._reserved[0] = 1;
        assert!(!ind.set_affect(affect), "not well formed");
        affect._reserved[0] = 0;
        assert!(ind.set_affect(affect));
        assert_eq!(ind.length(), 8);
    }

    #[test]
    fn a_compaction_refuses_a_bound_table_and_reclaims_nothing_from_a_host_s_store() {
        let mut none = Induction::new(0, 0, 0, 0, 0);
        assert_eq!(none.compact(), Err(TermError::NoArena));
        let mut ind = Induction::new(8, 2, 0, 0, 3);
        let a = ind.term(TermNode::constant(0x200)).unwrap();
        let x = ind.term(TermNode::variable(0)).unwrap();
        let px = ind.term(TermNode::compound(0x100, &[x]).unwrap()).unwrap();
        let _ = ind.term(TermNode::compound(0x101, &[a]).unwrap()).unwrap();
        ind.assert_clause(px, &[]).unwrap();
        // A binding names an index: a bound table refuses, and nothing moves.
        ind.bindings[0] = Binding(2);
        assert_eq!(ind.compact(), Err(TermError::Malformed));
        assert_eq!((ind.record.free, ind.compactions), (5, 0));
        ind.bindings[0] = Binding::UNBOUND;
        // The constant `a` and `q(a)` are reached by no clause: reclaimed; the clause, the
        // literal and the variable stay, in order, and the length stands.
        let report = ind.compact().unwrap();
        assert_eq!((report.live, report.reclaimed), (3, 2));
        assert_eq!(ind.clauses(), &[2]);
        assert_eq!(ind.terms()[0], TermNode::variable(0));
        assert_eq!(ind.length(), 3);
        assert_eq!((ind.compactions, ind.reclaimed), (1, 2));
    }
}
