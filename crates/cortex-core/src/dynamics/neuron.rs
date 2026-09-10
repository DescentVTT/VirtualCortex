use core::sync::atomic::{AtomicU8, AtomicU32, AtomicU64, Ordering};

/// The turn gate of axiom A3 (whitepaper §8.5, ADR-0006, ADR-0017), stored as the byte value of
/// `DendriticSuperNeuron::gate_state`. Idle is zero, so an image at rest (§8.7) is idle.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateState {
    /// No worker holds or has claimed the unit.
    Idle = 0,
    /// A pusher claimed the unit and enqueued it; a worker will claim the turn.
    Scheduled = 1,
    /// A worker holds the turn and is the only writer of the plain fields.
    Running = 2,
}

impl GateState {
    /// The state a byte encodes, or `None` for a byte no state has.
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Idle),
            1 => Some(Self::Scheduled),
            2 => Some(Self::Running),
            _ => None,
        }
    }
}

/// The empty mailbox: zero, so that an image at rest (§8.7) holds empty mailboxes with no
/// fix-up. A non-zero head or `next` value `v` names node `v - 1` of the caller's arena.
pub const MAILBOX_EMPTY: u64 = 0;
/// A node's `next` when it is the last of its list.
pub const MAILBOX_NIL: u32 = 0;

/// One mailbox node, 8 bytes, in an arena the caller owns (a per-worker pool, §8.5). The
/// payload is a spike message (`spike_message`, ADR-0022) for a delivery; the executor may put
/// any token there. Both
/// fields are atomics so that a producer can write them through a shared reference; their
/// stores are relaxed and are ordered by the compare-exchange on the mailbox head.
// Holds atomics: `Debug` only (whitepaper §8.2, rule L-5).
#[repr(C)]
#[derive(Debug)]
pub struct MailboxNode {
    pub next: AtomicU32,    // [0..4] Next node + 1, or MAILBOX_NIL
    pub payload: AtomicU32, // [4..8] The message: a spike message, efficacy and compartment (ADR-0022)
}

impl MailboxNode {
    /// A free node.
    pub const fn new() -> Self {
        Self {
            next: AtomicU32::new(MAILBOX_NIL),
            payload: AtomicU32::new(0),
        }
    }
}

impl Default for MailboxNode {
    fn default() -> Self {
        Self::new()
    }
}

/// The nodes drained from a mailbox by [`DendriticSuperNeuron::mailbox_drain`], most recently
/// pushed first. The iteration stops at the end of the list, at an index outside the arena
/// (a corrupt list) and after `arena.len()` nodes (a cyclic list), so it always terminates.
#[derive(Debug)]
pub struct MailboxDrain<'a> {
    nodes: &'a [MailboxNode],
    next: u64,
    remaining: usize,
}

impl Iterator for MailboxDrain<'_> {
    /// `(node index, payload)`; the node is the consumer's until it returns it to the pool.
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == MAILBOX_EMPTY || self.remaining == 0 {
            return None;
        }
        let idx = self.next.wrapping_sub(1);
        let Some(node) = usize::try_from(idx).ok().and_then(|i| self.nodes.get(i)) else {
            self.next = MAILBOX_EMPTY;
            return None;
        };
        self.next = node.next.load(Ordering::Relaxed) as u64;
        self.remaining = self.remaining.saturating_sub(1);
        Some((idx as u32, node.payload.load(Ordering::Relaxed)))
    }
}

/// The neural unit: a two-compartment record (basal, apical, soma) with short-term-plasticity
/// state and the virtual-actor control fields (whitepaper §5.2.1). Not plain old data: it
/// holds atomics and is a control record (rule L-5).
// Control record (whitepaper §8.2, rule L-5): holds atomics, so it is Sync but not Copy.
#[derive(Debug)]
#[repr(C, align(64))]
pub struct DendriticSuperNeuron {
    pub id: u64,                     // [0..8] Global neuron ID
    pub mailbox_head_ptr: AtomicU64, // [8..16] Mailbox head: node index + 1, MAILBOX_EMPTY when empty (an index despite the name; L-3 reserves the suffix for it)
    pub mailbox_reserved: u64, // [16..24] Reserved; MUST be zero (the ABA tag of ADR-0006, removed by ADR-0017: a push-and-drain-whole stack needs none)
    pub v_soma: i32,           // [24..28] Soma potential (Q16.16)
    pub v_basal: i32,          // [28..32] Basal feedforward potential (Q16.16)
    pub v_apical: i32,         // [32..36] Apical contextual potential (Q16.16)
    pub v_thresh: i32,         // [36..40] Dynamic adaptive threshold (Q16.16)
    pub bac_plateau_ticks: u16, // [40..42] Larkum BAC calcium burst countdown
    pub refractory_ticks: u16, // [42..44] Absolute refractory countdown
    pub last_soma_spike_tick: u32, // [44..48] Somatic action potential timestamp
    pub synapse_slab_idx: u32, // [48..52] First SynapseBlock of the fan-out, as index + 1; 0 = no fan-out (ADR-0022)
    pub _reserved: u16, // [52..54] Reserved; MUST be zero (the 16-bit delta head lived here until ADR-0024)
    pub spatial_voxel_morton: u16, // [54..56] 16-bit Morton spatial voxel code
    pub gate_state: AtomicU8, // [56] Turn gate: a GateState byte
    pub flags: u8,      // [57] BURST_MODE / Inhibitory Flags
    pub stp_r_ves: u8,  // [58] Tsodyks-Markram vesicle pool (STD), Q0.8
    pub stp_u_rel: u8,  // [59] Tsodyks-Markram release fraction (STF), Q0.8
    pub plastic_delta_head: u32, // [60..64] First PlasticDelta of the unit's list, as index + 1; 0 = none (ADR-0024, finding F-20)
}

impl DendriticSuperNeuron {
    /// A unit at rest: every field zero apart from `id`, which is an idle gate and an empty
    /// mailbox, the state an image at rest holds (§8.7).
    pub const fn new(id: u64) -> Self {
        Self {
            id,
            mailbox_head_ptr: AtomicU64::new(MAILBOX_EMPTY),
            mailbox_reserved: 0,
            v_soma: 0,
            v_basal: 0,
            v_apical: 0,
            v_thresh: 0,
            bac_plateau_ticks: 0,
            refractory_ticks: 0,
            last_soma_spike_tick: 0,
            synapse_slab_idx: 0,
            _reserved: 0,
            spatial_voxel_morton: 0,
            gate_state: AtomicU8::new(GateState::Idle as u8),
            flags: 0,
            stp_r_ves: 0,
            stp_u_rel: 0,
            plastic_delta_head: 0,
        }
    }

    /// The gate's state, or `None` for a byte no state has (a corrupt record).
    #[inline]
    pub fn gate(&self) -> Option<GateState> {
        GateState::from_u8(self.gate_state.load(Ordering::Acquire))
    }

    /// Pusher, after [`mailbox_push`](Self::mailbox_push): claims the right to enqueue the unit
    /// on a worker deque (idle → scheduled). `true` means the caller MUST enqueue it; `false`
    /// means it is already scheduled or running and the message will be drained by that turn or
    /// caught by [`end_turn`](Self::end_turn). Sequentially consistent, with the head
    /// compare-exchange before it, the idle store and the head load of `end_turn`: the four
    /// operations that make a lost wakeup impossible (ADR-0017).
    #[inline]
    pub fn try_schedule(&self) -> bool {
        self.gate_state
            .compare_exchange(
                GateState::Idle as u8,
                GateState::Scheduled as u8,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
    }

    /// Worker: claims the turn of a scheduled unit (scheduled → running). Acquire on success,
    /// so that every plain-field write of the previous turn is visible to this one.
    #[inline]
    pub fn begin_turn(&self) -> bool {
        self.gate_state
            .compare_exchange(
                GateState::Scheduled as u8,
                GateState::Running as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    /// Worker: ends the turn (running → idle) and closes the lost-wakeup window: after storing
    /// idle it re-reads the mailbox head, and if a message arrived while the unit was running
    /// it re-claims the schedule itself. `true` means the caller MUST enqueue the unit again.
    /// The idle store releases every plain-field write of this turn.
    #[inline]
    pub fn end_turn(&self) -> bool {
        self.gate_state
            .store(GateState::Idle as u8, Ordering::SeqCst);
        if self.mailbox_head_ptr.load(Ordering::SeqCst) != MAILBOX_EMPTY {
            self.try_schedule()
        } else {
            false
        }
    }

    /// True when no message waits.
    #[inline]
    pub fn mailbox_is_empty(&self) -> bool {
        self.mailbox_head_ptr.load(Ordering::SeqCst) == MAILBOX_EMPTY
    }

    /// Any worker: pushes `payload` in node `node` of the caller's arena onto the mailbox. One
    /// compare-exchange on the head; the payload and link are stored before it and are ordered
    /// by it. Refused, with nothing changed, for a node outside the arena, for node
    /// `u32::MAX` (whose `+ 1` encoding does not fit) and for a corrupt head (read before the
    /// payload is stored; a head that turns corrupt during the retry leaves the payload in the
    /// caller's own node). A pusher calls
    /// [`try_schedule`](Self::try_schedule) after a successful push, never before.
    pub fn mailbox_push(&self, nodes: &[MailboxNode], node: u32, payload: u32) -> bool {
        if node == u32::MAX {
            return false;
        }
        let Some(n) = nodes.get(node as usize) else {
            return false;
        };
        let mut head = self.mailbox_head_ptr.load(Ordering::Relaxed);
        let encoded = (node as u64).wrapping_add(1);
        loop {
            // One check for the first attempt and every retry: a corrupt head refuses before
            // anything is stored on the first attempt, and leaves the payload in the caller's own
            // node on a retry.
            if head > u32::MAX as u64 {
                return false;
            }
            n.payload.store(payload, Ordering::Relaxed);
            n.next.store(head as u32, Ordering::Relaxed);
            match self.mailbox_head_ptr.compare_exchange_weak(
                head,
                encoded,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(current) => head = current,
            }
        }
    }

    /// The worker holding the turn: takes the whole mailbox in one swap and walks it, most
    /// recently pushed first. The swap synchronises with every push it took, so the payloads
    /// and links read during the walk are the ones the pushers stored. The nodes yielded are
    /// the consumer's until it returns them to their pool. The order is reverse arrival, which
    /// under concurrent pushers is not deterministic; the executor orders a batch before
    /// integrating it (whitepaper §8.3, Specified).
    pub fn mailbox_drain<'a>(&self, nodes: &'a [MailboxNode]) -> MailboxDrain<'a> {
        let head = self.mailbox_head_ptr.swap(MAILBOX_EMPTY, Ordering::SeqCst);
        MailboxDrain {
            nodes,
            next: head,
            remaining: nodes.len(),
        }
    }
}

impl Default for DendriticSuperNeuron {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Four outgoing synapses of one unit, 64 bytes; blocks chain by index (whitepaper §5.2.1,
/// ADR-0022). Every index stored here is `index + 1`, so zero is an empty slot or the end of
/// the chain and a zeroed block is a valid empty one. The walk, the plasticity rule and the
/// delivery encodings are in `dynamics/synapse.rs`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SynapseBlock {
    pub target_neuron_ids: [u32; 4], // [0..16] Post-synaptic unit index + 1 per slot; 0 = empty slot
    pub weights_q1_15: [i16; 4], // [16..24] Base weights, Q1.15 (ADR-0012), moved by STDP (ADR-0022)
    pub delays_ticks: [u16; 4], // [24..32] Conduction delay per slot; 0 = mailbox now, else the wheel
    pub chain: u32, // [32..36] Bits 0–27: next block index + 1, 0 = end of chain; bits 28–31: bit 28 + k set when slot k lands in the apical compartment (ADR-0032)
    pub last_spike_tick: u32, // [36..40] Last presynaptic spike; 0 = none on record (STDP)
    pub last_release_q16: [i32; 4], // [40..56] Efficacy released by the last presynaptic spike per slot (Q16.16), read at delayed delivery
    pub eligibility_q1_15: [i16; 4], // [56..64] Eligibility trace per slot (Q1.15): the pairing amounts not yet consolidated into the weight (ADR-0032)
}

/// Synaptic efficacy in Q16.16 from a Q1.15 base weight and the two Q0.8 short-term
/// plasticity factors (ADR-0012).
///
/// The product carries 15 + 8 + 8 = 31 fractional bits and is formed exactly in `i64`; one
/// arithmetic shift by 15 yields Q16.16. No intermediate rounding occurs, so every bit of the
/// Q1.15 weight survives into the result, scaled by the coarser STP factors. The magnitude is
/// bounded by 1.0 (65 536), so the result always fits an `i32`; the shift floors toward
/// negative infinity, consistent with whitepaper §8.1.
#[inline(always)]
pub const fn synaptic_efficacy_q16(w_q1_15: i16, u_q0_8: u8, r_q0_8: u8) -> i32 {
    ((w_q1_15 as i64)
        .saturating_mul(u_q0_8 as i64)
        .saturating_mul(r_q0_8 as i64)
        >> 15) as i32
}

const _: () = {
    assert!(core::mem::size_of::<DendriticSuperNeuron>() == 64);
    assert!(core::mem::align_of::<DendriticSuperNeuron>() == 64);
    assert!(core::mem::size_of::<SynapseBlock>() == 64);
    assert!(core::mem::align_of::<SynapseBlock>() == 64);
    assert!(core::mem::size_of::<MailboxNode>() == 8);
    assert!(core::mem::align_of::<MailboxNode>() == 4);
};

#[cfg(test)]
mod tests {
    use super::*;

    const Q16_ONE: i32 = 0x0001_0000;

    fn arena<const N: usize>() -> [MailboxNode; N] {
        core::array::from_fn(|_| MailboxNode::new())
    }

    #[test]
    fn full_weight_and_full_stp_is_just_below_one() {
        // 32767 × 255 × 255 = 2 130 674 175; >> 15 floors to 65 023 (0.99217 in Q16.16), below 1.0.
        let e = synaptic_efficacy_q16(i16::MAX, 255, 255);
        assert_eq!(e, 65_023);
        assert!(e < Q16_ONE);
    }

    #[test]
    fn most_negative_weight_stays_in_range() {
        // −1.0 × 255/256 × 255/256 = −0.99221 → −65 025; the i64 product cannot overflow.
        assert_eq!(synaptic_efficacy_q16(i16::MIN, 255, 255), -65_025);
    }

    #[test]
    fn depleted_resource_or_zero_release_silences_the_synapse() {
        assert_eq!(synaptic_efficacy_q16(i16::MAX, 255, 0), 0);
        assert_eq!(synaptic_efficacy_q16(i16::MAX, 0, 255), 0);
    }

    #[test]
    fn half_weight_half_release_full_resource() {
        // 0.5 × 0.5 × 0.996 = 0.249 → 16 320 in Q16.16, exactly.
        assert_eq!(synaptic_efficacy_q16(0x4000, 128, 255), 16_320);
    }

    #[test]
    fn one_lsb_of_weight_survives_at_full_stp_and_vanishes_at_half() {
        assert_eq!(synaptic_efficacy_q16(1, 255, 255), 1);
        assert_eq!(synaptic_efficacy_q16(1, 128, 128), 0);
    }

    #[test]
    fn flooring_is_toward_negative_infinity() {
        // −65 025 >> 15 = −1.98 floors to −2, not −1.
        assert_eq!(synaptic_efficacy_q16(-1, 255, 255), -2);
    }

    #[test]
    fn efficacy_is_monotonic_in_each_factor_in_the_direction_of_the_weight_s_sign() {
        for w in [i16::MIN, -0x2000, -1, 1, 0x2000, i16::MAX] {
            for (u, r) in [
                (0u8, 0u8),
                (1, 1),
                (51, 200),
                (100, 100),
                (254, 254),
                (255, 255),
            ] {
                let base = synaptic_efficacy_q16(w, u, r);
                let up_u = synaptic_efficacy_q16(w, u.saturating_add(1), r);
                let up_r = synaptic_efficacy_q16(w, u, r.saturating_add(1));
                if w > 0 {
                    assert!(up_u >= base && up_r >= base, "{w} {u} {r}");
                } else {
                    assert!(up_u <= base && up_r <= base, "{w} {u} {r}");
                }
            }
            assert!(
                synaptic_efficacy_q16(w.saturating_add(1), 100, 100)
                    >= synaptic_efficacy_q16(w, 100, 100)
            );
        }
    }

    #[test]
    fn a_new_unit_is_at_rest_and_the_gate_byte_encodes_three_states() {
        let u = DendriticSuperNeuron::new(7);
        assert_eq!(u.id, 7);
        assert_eq!(u.gate(), Some(GateState::Idle));
        assert!(u.mailbox_is_empty());
        assert_eq!(u.mailbox_reserved, 0);
        assert_eq!(GateState::from_u8(0), Some(GateState::Idle));
        assert_eq!(GateState::from_u8(1), Some(GateState::Scheduled));
        assert_eq!(GateState::from_u8(2), Some(GateState::Running));
        assert_eq!(GateState::from_u8(3), None);
        assert_eq!(DendriticSuperNeuron::default().id, 0);
    }

    #[test]
    fn the_gate_admits_one_scheduler_and_one_worker_per_turn() {
        let u = DendriticSuperNeuron::new(1);
        assert!(!u.begin_turn(), "nothing to claim while idle");
        assert!(u.try_schedule());
        assert!(!u.try_schedule(), "scheduled at most once between turns");
        assert_eq!(u.gate(), Some(GateState::Scheduled));
        assert!(u.begin_turn());
        assert!(!u.begin_turn(), "one worker holds the turn");
        assert!(!u.try_schedule(), "a pusher cannot schedule a running unit");
        assert!(!u.end_turn(), "an empty mailbox ends the turn idle");
        assert_eq!(u.gate(), Some(GateState::Idle));
    }

    #[test]
    fn push_then_drain_yields_every_node_once_most_recent_first_and_empties_the_mailbox() {
        let nodes = arena::<4>();
        let u = DendriticSuperNeuron::new(1);
        for i in 0..4u32 {
            assert!(u.mailbox_push(&nodes, i, 100 + i));
        }
        assert!(!u.mailbox_is_empty());
        let drained: [(u32, u32); 4] = {
            let mut it = u.mailbox_drain(&nodes);
            let d = [
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ];
            assert_eq!(it.next(), None);
            d
        };
        assert_eq!(drained, [(3, 103), (2, 102), (1, 101), (0, 100)]);
        assert!(u.mailbox_is_empty());
        assert_eq!(
            u.mailbox_drain(&nodes).next(),
            None,
            "a second drain is empty"
        );
    }

    #[test]
    fn a_corrupt_head_refuses_a_push_before_the_payload_is_stored() {
        let nodes = [MailboxNode::new()];
        let u = DendriticSuperNeuron::new(1);
        u.mailbox_head_ptr.store(u64::MAX, Ordering::SeqCst);
        assert!(!u.mailbox_push(&nodes, 0, 0xABCD));
        assert_eq!(
            nodes[0].payload.load(Ordering::Relaxed),
            0,
            "nothing changed"
        );
        assert_eq!(u.mailbox_head_ptr.load(Ordering::SeqCst), u64::MAX);
    }

    #[test]
    fn pushes_outside_the_arena_are_refused_unchanged() {
        let nodes = arena::<2>();
        let u = DendriticSuperNeuron::new(1);
        assert!(!u.mailbox_push(&nodes, 2, 9));
        assert!(!u.mailbox_push(&nodes, u32::MAX, 9));
        assert!(u.mailbox_is_empty());
        assert_eq!(nodes[0].payload.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn a_corrupt_or_cyclic_list_terminates_the_drain() {
        let nodes = arena::<2>();
        let u = DendriticSuperNeuron::new(1);
        u.mailbox_head_ptr.store(7, Ordering::SeqCst);
        assert_eq!(
            u.mailbox_drain(&nodes).next(),
            None,
            "node 6 is outside the arena"
        );
        nodes[0].next.store(2, Ordering::Relaxed);
        nodes[1].next.store(1, Ordering::Relaxed);
        u.mailbox_head_ptr.store(1, Ordering::SeqCst);
        assert_eq!(
            u.mailbox_drain(&nodes).count(),
            2,
            "a cycle yields at most the arena"
        );
        u.mailbox_head_ptr.store(u64::MAX, Ordering::SeqCst);
        assert!(
            !u.mailbox_push(&nodes, 0, 1),
            "a corrupt head refuses the push"
        );
    }

    #[test]
    fn end_turn_catches_a_message_pushed_during_the_turn() {
        let nodes = arena::<2>();
        let u = DendriticSuperNeuron::new(1);
        assert!(u.try_schedule());
        assert!(u.begin_turn());
        let _ = u.mailbox_drain(&nodes).count();
        // A message arrives while the unit runs: the pusher cannot schedule it.
        assert!(u.mailbox_push(&nodes, 0, 42));
        assert!(!u.try_schedule());
        // A worker that merely stored idle would leave the lost-wakeup state behind: a message
        // waits and nobody is scheduled.
        u.gate_state.store(GateState::Idle as u8, Ordering::SeqCst);
        assert!(!u.mailbox_is_empty());
        assert_eq!(u.gate(), Some(GateState::Idle));
        // The rule: end_turn re-reads the head and re-claims the schedule.
        u.gate_state
            .store(GateState::Running as u8, Ordering::SeqCst);
        assert!(u.end_turn(), "the caller must enqueue the unit again");
        assert_eq!(u.gate(), Some(GateState::Scheduled));
        assert!(u.begin_turn());
        assert_eq!(u.mailbox_drain(&nodes).next(), Some((0, 42)));
        assert!(!u.end_turn());
    }
}
