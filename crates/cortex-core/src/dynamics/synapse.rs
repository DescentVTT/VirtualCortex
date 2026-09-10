//! Synaptic fan-out and pair-based spike-timing-dependent plasticity on `SynapseBlock`
//! (whitepaper §5.2.1, §6.1 step 6, §8.8; ADR-0022; brief 013).
//!
//! A unit's outgoing synapses live in 64-byte blocks of four, chained by index. Every index a
//! block stores is `index + 1`, so that zero is "nothing": an empty slot, the end of a chain, a
//! unit with no fan-out. A zeroed arena is therefore a valid arena of empty blocks and an image
//! at rest (§8.7) needs no fix-up. The walk is bounded by the arena, so a corrupt or cyclic
//! chain terminates. A delayed delivery travels through the wheel as a 28-bit token naming the
//! synapse (`block × 4 + slot`); the efficacy it releases is stored in the block at the spike
//! and read back at delivery, so the delivering worker needs nothing but the block. A delivery
//! reaches the target's mailbox as a 32-bit message carrying the efficacy and the compartment.
//! STDP is the nearest-neighbour pair rule applied at the presynaptic spike, with the window
//! $(1 - 2^{-11})^{\Delta t}$ from `stp_decay_factor_q16` and amplitudes in Q1.15, saturating.

use super::neuron::{DendriticSuperNeuron, SynapseBlock, synaptic_efficacy_q16};
use super::plasticity::stp_decay_factor_q16;
use crate::dispatch::wheel::MAX_TOKEN;

/// Synapses per block.
pub const SYNAPSES_PER_BLOCK: usize = 4;
/// `next_block_idx` of the last block of a chain, and `synapse_slab_idx` of a unit without
/// fan-out: zero, so that a record at rest names nothing.
pub const CHAIN_END: u32 = 0;
/// `target_neuron_ids[k]` of an empty slot.
pub const SLOT_EMPTY: u32 = 0;
/// A `last_spike_tick` or `last_soma_spike_tick` of zero: no spike on record. A spike at a
/// tick that is zero modulo $2^{32}$ leaves no record; the next one does (§8.4).
pub const NO_SPIKE_ON_RECORD: u32 = 0;

/// STDP window $\tau_+ = \tau_- = 2^{11}$ ticks (20.48 ms).
pub const STDP_TAU_SHIFT: u32 = 11;
/// Potentiation amplitude $A_+$ = 328/32768 ≈ 0.0100 in Q1.15.
pub const STDP_A_PLUS_Q1_15: i16 = 328;
/// Depression amplitude $A_-$ = 344/32768 ≈ 0.0105 in Q1.15, so $A_- / A_+ = 1.05$ and the
/// rule is depression-dominant at equal windows.
pub const STDP_A_MINUS_Q1_15: i16 = 344;

/// Largest block index a delayed-delivery token can name: the 28-bit token holds a 26-bit
/// block index and a 2-bit slot (finding F-23).
pub const MAX_TOKEN_BLOCK: u32 = (1 << 26) - 1;
/// Message bit 18: the efficacy lands in the apical compartment; clear, the basal one.
pub const MESSAGE_APICAL: u32 = 1 << 18;
const MESSAGE_EFFICACY_BITS: u32 = 18;
const MESSAGE_EFFICACY_MASK: u32 = (1 << MESSAGE_EFFICACY_BITS) - 1;
const MESSAGE_EFFICACY_MAX: i32 = (1 << (MESSAGE_EFFICACY_BITS - 1)) - 1;

/// The wheel token of one synapse: `block_idx << 2 | slot`, 28 bits. `None` when the block
/// index exceeds [`MAX_TOKEN_BLOCK`] or the slot is not one of four.
pub const fn synapse_token(block_idx: u32, slot: u8) -> Option<u32> {
    if block_idx > MAX_TOKEN_BLOCK || slot as usize >= SYNAPSES_PER_BLOCK {
        return None;
    }
    Some((block_idx << 2) | slot as u32)
}

/// The block a token names.
#[inline]
pub const fn token_block(token: u32) -> u32 {
    (token & MAX_TOKEN) >> 2
}

/// The slot a token names.
#[inline]
pub const fn token_slot(token: u32) -> u8 {
    (token & 0b11) as u8
}

/// The mailbox message of one delivery: the efficacy in 18-bit two's complement (bits 0–17;
/// `synaptic_efficacy_q16` is below 1.0 in magnitude, so it fits; a wider value is clamped) and
/// the compartment in bit 18. Bits 19–31 are zero. Messages sort by value, which is the batch
/// order the executor uses (§8.3).
pub const fn spike_message(efficacy_q16: i32, apical: bool) -> u32 {
    let e = if efficacy_q16 > MESSAGE_EFFICACY_MAX {
        MESSAGE_EFFICACY_MAX
    } else if efficacy_q16 < -MESSAGE_EFFICACY_MAX - 1 {
        -MESSAGE_EFFICACY_MAX - 1
    } else {
        efficacy_q16
    };
    ((e as u32) & MESSAGE_EFFICACY_MASK) | if apical { MESSAGE_APICAL } else { 0 }
}

/// The efficacy a message carries, sign-extended from 18 bits.
#[inline]
pub const fn message_efficacy_q16(message: u32) -> i32 {
    ((message << (32 - MESSAGE_EFFICACY_BITS)) as i32) >> (32 - MESSAGE_EFFICACY_BITS)
}

/// True when the message lands in the apical compartment.
#[inline]
pub const fn message_is_apical(message: u32) -> bool {
    message & MESSAGE_APICAL != 0
}

/// One synapse as the walk yields it: where it is and what it carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Synapse {
    /// Index of the block in the arena.
    pub block_idx: u32,
    /// Slot within the block, 0 to 3.
    pub slot: u8,
    /// The efficacy lands in the target's apical compartment (else basal).
    pub apical: bool,
    /// The post-synaptic unit index (decoded).
    pub target: u32,
    /// The base weight, Q1.15.
    pub weight_q1_15: i16,
    /// The conduction delay; 0 delivers through the mailbox, else through the wheel. The walk
    /// yields a delay beyond the wheel's horizon unchanged: rejecting it is the loader's job
    /// (§6.2).
    pub delay_ticks: u16,
}

/// The synapses of a chain, in block order and slot order, empty slots skipped. Stops at the
/// end of the chain, at an index outside the arena (a corrupt chain) and after `arena.len()`
/// blocks (a cyclic chain), so it always terminates.
#[derive(Debug)]
pub struct FanOut<'a> {
    blocks: &'a [SynapseBlock],
    next: u32,
    slot: usize,
    remaining: usize,
}

impl Iterator for FanOut<'_> {
    type Item = Synapse;

    fn next(&mut self) -> Option<Synapse> {
        loop {
            if self.next == CHAIN_END {
                return None;
            }
            let idx = self.next - 1;
            let Some(block) = self.blocks.get(idx as usize) else {
                self.next = CHAIN_END;
                return None;
            };
            if self.slot == 0 {
                if self.remaining == 0 {
                    self.next = CHAIN_END;
                    return None;
                }
                self.remaining -= 1;
            }
            while self.slot < SYNAPSES_PER_BLOCK {
                let slot = self.slot;
                self.slot += 1;
                let encoded = block.target_neuron_ids[slot];
                if encoded != SLOT_EMPTY {
                    return Some(Synapse {
                        block_idx: idx,
                        slot: slot as u8,
                        apical: block.apical_mask & (1 << slot) != 0,
                        target: encoded - 1,
                        weight_q1_15: block.weights_q1_15[slot],
                        delay_ticks: block.delays_ticks[slot],
                    });
                }
            }
            self.slot = 0;
            self.next = block.next_block_idx;
        }
    }
}

/// The block indices of a chain, in order, with the same termination as [`FanOut`]. For a
/// caller that must mutate the blocks it walks.
#[derive(Debug)]
pub struct Chain<'a> {
    blocks: &'a [SynapseBlock],
    next: u32,
    remaining: usize,
}

impl Iterator for Chain<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.next == CHAIN_END || self.remaining == 0 {
            return None;
        }
        let idx = self.next - 1;
        let Some(block) = self.blocks.get(idx as usize) else {
            self.next = CHAIN_END;
            return None;
        };
        self.remaining -= 1;
        self.next = block.next_block_idx;
        Some(idx)
    }
}

/// $A \cdot (1 - 2^{-11})^{\Delta t}$ in Q1.15, rounded to nearest.
#[inline]
fn window(amplitude_q1_15: i16, delta_ticks: u32) -> i16 {
    let factor = stp_decay_factor_q16(delta_ticks, STDP_TAU_SHIFT) as i64;
    ((amplitude_q1_15 as i64 * factor + 0x8000) >> 16) as i16
}

impl SynapseBlock {
    /// An empty block: four empty slots, the end of a chain, no spike on record. Equal to
    /// `Default`, and a zeroed image holds nothing else.
    pub const fn new() -> Self {
        Self {
            target_neuron_ids: [SLOT_EMPTY; SYNAPSES_PER_BLOCK],
            weights_q1_15: [0; SYNAPSES_PER_BLOCK],
            delays_ticks: [0; SYNAPSES_PER_BLOCK],
            next_block_idx: CHAIN_END,
            last_spike_tick: NO_SPIKE_ON_RECORD,
            last_release_q16: [0; SYNAPSES_PER_BLOCK],
            apical_mask: 0,
            _reserved: [0; 7],
        }
    }

    /// True for the last block of a chain.
    #[inline]
    pub const fn is_end(&self) -> bool {
        self.next_block_idx == CHAIN_END
    }

    /// The next block of the chain, decoded, or `None` at the end.
    #[inline]
    pub const fn next(&self) -> Option<u32> {
        if self.next_block_idx == CHAIN_END {
            None
        } else {
            Some(self.next_block_idx - 1)
        }
    }

    /// Chains `next_block_idx` after this block. Refused for `u32::MAX`, whose `+ 1` encoding
    /// does not fit.
    pub fn link(&mut self, next_block_idx: u32) -> bool {
        if next_block_idx == u32::MAX {
            return false;
        }
        self.next_block_idx = next_block_idx + 1;
        true
    }

    /// Makes this block the last of its chain.
    pub fn unlink(&mut self) {
        self.next_block_idx = CHAIN_END;
    }

    /// The target of a slot, decoded, or `None` for an empty slot or a slot that does not exist.
    #[inline]
    pub const fn target(&self, slot: usize) -> Option<u32> {
        if slot >= SYNAPSES_PER_BLOCK {
            return None;
        }
        match self.target_neuron_ids[slot] {
            SLOT_EMPTY => None,
            encoded => Some(encoded - 1),
        }
    }

    /// True when the slot's efficacy lands in the apical compartment.
    #[inline]
    pub const fn is_apical(&self, slot: usize) -> bool {
        slot < SYNAPSES_PER_BLOCK && self.apical_mask & (1 << slot) != 0
    }

    /// Fills a slot: target, base weight, delay and compartment; the last release is cleared.
    /// Refused for a slot that does not exist and for target `u32::MAX`, whose `+ 1` encoding
    /// does not fit.
    pub fn set_synapse(
        &mut self,
        slot: usize,
        target: u32,
        weight_q1_15: i16,
        delay_ticks: u16,
        apical: bool,
    ) -> bool {
        if slot >= SYNAPSES_PER_BLOCK || target == u32::MAX {
            return false;
        }
        self.target_neuron_ids[slot] = target + 1;
        self.weights_q1_15[slot] = weight_q1_15;
        self.delays_ticks[slot] = delay_ticks;
        if apical {
            self.apical_mask |= 1 << slot;
        } else {
            self.apical_mask &= !(1 << slot);
        }
        self.last_release_q16[slot] = 0;
        true
    }

    /// Empties a slot. Refused for a slot that does not exist.
    pub fn clear_synapse(&mut self, slot: usize) -> bool {
        if slot >= SYNAPSES_PER_BLOCK {
            return false;
        }
        self.target_neuron_ids[slot] = SLOT_EMPTY;
        self.weights_q1_15[slot] = 0;
        self.delays_ticks[slot] = 0;
        self.apical_mask &= !(1 << slot);
        self.last_release_q16[slot] = 0;
        true
    }

    /// The synapses of the chain that starts at `head`, an encoded index (`index + 1`; 0 is
    /// no chain), as a unit stores it in `synapse_slab_idx`.
    pub fn fan_out(blocks: &[SynapseBlock], head: u32) -> FanOut<'_> {
        FanOut {
            blocks,
            next: head,
            slot: 0,
            remaining: blocks.len(),
        }
    }

    /// The block indices of the chain that starts at `head` (encoded as for
    /// [`fan_out`](Self::fan_out)).
    pub fn chain(blocks: &[SynapseBlock], head: u32) -> Chain<'_> {
        Chain {
            blocks,
            next: head,
            remaining: blocks.len(),
        }
    }

    /// The release of one presynaptic spike through one slot: the efficacy of the slot's weight
    /// under the presynaptic unit's short-term-plasticity factors ([ADR-0012], [ADR-0019]),
    /// stored in `last_release_q16` so that a delayed delivery reads it back, and returned. Zero,
    /// and nothing stored, for an empty slot or a slot that does not exist.
    ///
    /// [ADR-0012]: https://github.com/DescentVTT/VirtualCortex/blob/main/docs/adr/0012-synaptic-weight-q1-15.md
    /// [ADR-0019]: https://github.com/DescentVTT/VirtualCortex/blob/main/docs/adr/0019-short-term-plasticity.md
    pub fn release(&mut self, slot: usize, u_q0_8: u8, r_q0_8: u8) -> i32 {
        if slot >= SYNAPSES_PER_BLOCK || self.target_neuron_ids[slot] == SLOT_EMPTY {
            return 0;
        }
        let efficacy = synaptic_efficacy_q16(self.weights_q1_15[slot], u_q0_8, r_q0_8);
        self.last_release_q16[slot] = efficacy;
        efficacy
    }

    /// [`release`](Self::release) for every slot; empty slots give zero.
    pub fn release_all(&mut self, u_q0_8: u8, r_q0_8: u8) -> [i32; SYNAPSES_PER_BLOCK] {
        let mut out = [0; SYNAPSES_PER_BLOCK];
        for (slot, e) in out.iter_mut().enumerate() {
            *e = self.release(slot, u_q0_8, r_q0_8);
        }
        out
    }

    /// The nearest-neighbour pair rule at a presynaptic spike, for one slot (ADR-0022,
    /// whitepaper §8.8). With $p$ this block's previous presynaptic stamp, $q$ the target's last
    /// somatic spike and $t$ the spike now: if $p < q \le t$ the target fired after the previous
    /// presynaptic spike, and the weight gains $A_+ (1 - 2^{-11})^{q - p}$; then, if $q < t$,
    /// the target fired before this one and the weight loses $A_- (1 - 2^{-11})^{t - q}$. Every
    /// comparison is a wrapping difference read as signed (§8.4), so a stamp older than $2^{31}$
    /// ticks reads as future and pairs with nothing; a stamp of zero is no spike on record.
    /// Saturating in $[-1, 1)$. Does not stamp: the caller stamps once per block after every
    /// slot ([`stamp_presynaptic`](Self::stamp_presynaptic)), or uses
    /// [`step_stdp_all`](Self::step_stdp_all). Returns the weight; zero for an empty slot.
    pub fn step_stdp(&mut self, slot: usize, pre_now_tick: u32, post_last_tick: u32) -> i16 {
        if slot >= SYNAPSES_PER_BLOCK || self.target_neuron_ids[slot] == SLOT_EMPTY {
            return 0;
        }
        let mut w = self.weights_q1_15[slot];
        if post_last_tick != NO_SPIKE_ON_RECORD {
            let since_post = pre_now_tick.wrapping_sub(post_last_tick) as i32;
            if self.last_spike_tick != NO_SPIKE_ON_RECORD {
                let post_after_prev = post_last_tick.wrapping_sub(self.last_spike_tick) as i32;
                if post_after_prev > 0 && since_post >= 0 {
                    w = w.saturating_add(window(STDP_A_PLUS_Q1_15, post_after_prev as u32));
                }
            }
            if since_post > 0 {
                w = w.saturating_sub(window(STDP_A_MINUS_Q1_15, since_post as u32));
            }
        }
        self.weights_q1_15[slot] = w;
        w
    }

    /// Records the presynaptic spike the block just carried, after every slot has been
    /// updated against the previous one.
    #[inline]
    pub fn stamp_presynaptic(&mut self, now_tick: u32) {
        self.last_spike_tick = now_tick;
    }

    /// [`step_stdp`](Self::step_stdp) for every slot against its target's last somatic spike,
    /// then the stamp. Returns the weights.
    pub fn step_stdp_all(
        &mut self,
        now_tick: u32,
        post_last_ticks: [u32; SYNAPSES_PER_BLOCK],
    ) -> [i16; SYNAPSES_PER_BLOCK] {
        let mut out = [0; SYNAPSES_PER_BLOCK];
        for (slot, w) in out.iter_mut().enumerate() {
            *w = self.step_stdp(slot, now_tick, post_last_ticks[slot]);
        }
        self.stamp_presynaptic(now_tick);
        out
    }
}

impl DendriticSuperNeuron {
    /// The first block of the unit's fan-out, decoded, or `None` for a unit without one.
    #[inline]
    pub const fn first_block(&self) -> Option<u32> {
        if self.synapse_slab_idx == CHAIN_END {
            None
        } else {
            Some(self.synapse_slab_idx - 1)
        }
    }

    /// Names the first block of the unit's fan-out. Refused for `u32::MAX`, whose `+ 1`
    /// encoding does not fit.
    pub fn set_first_block(&mut self, block_idx: u32) -> bool {
        if block_idx == u32::MAX {
            return false;
        }
        self.synapse_slab_idx = block_idx + 1;
        true
    }

    /// The unit's synapses, walking its chain in `blocks` (R-1 step 6).
    pub fn fan_out<'a>(&self, blocks: &'a [SynapseBlock]) -> FanOut<'a> {
        SynapseBlock::fan_out(blocks, self.synapse_slab_idx)
    }

    /// The block indices of the unit's chain in `blocks`.
    pub fn chain<'a>(&self, blocks: &'a [SynapseBlock]) -> Chain<'a> {
        SynapseBlock::chain(blocks, self.synapse_slab_idx)
    }
}

const _: () = {
    assert!(MAX_TOKEN == (MAX_TOKEN_BLOCK << 2) | 0b11);
};

#[cfg(test)]
mod tests {
    use super::super::plasticity::{STP_MAX, STP_U};
    use super::*;

    fn full_block(first_target: u32, delay: u16) -> SynapseBlock {
        let mut b = SynapseBlock::new();
        for slot in 0..SYNAPSES_PER_BLOCK {
            assert!(b.set_synapse(
                slot,
                first_target + slot as u32,
                0x1000 * (slot as i16 + 1),
                delay,
                slot & 1 == 1
            ));
        }
        b
    }

    fn collect<const N: usize>(it: FanOut<'_>) -> ([Option<Synapse>; N], usize) {
        let mut out = [None; N];
        let mut n = 0;
        for s in it {
            assert!(n < N, "more synapses than expected");
            out[n] = Some(s);
            n += 1;
        }
        (out, n)
    }

    #[test]
    fn a_zeroed_block_is_empty_and_ends_its_chain() {
        let b = SynapseBlock::new();
        assert_eq!(b, SynapseBlock::default());
        assert!(b.is_end());
        assert_eq!(b.next(), None);
        for slot in 0..SYNAPSES_PER_BLOCK {
            assert_eq!(b.target(slot), None);
            assert!(!b.is_apical(slot));
        }
        assert_eq!(b.target(4), None);
        let arena = [b];
        assert_eq!(SynapseBlock::fan_out(&arena, 1).count(), 0);
        assert_eq!(SynapseBlock::fan_out(&arena, CHAIN_END).count(), 0);
        assert_eq!(SynapseBlock::chain(&arena, 1).count(), 1, "one empty block");
        let unit = DendriticSuperNeuron::new(1);
        assert_eq!(unit.first_block(), None, "a unit at rest has no fan-out");
        assert_eq!(unit.fan_out(&arena).count(), 0);
    }

    #[test]
    fn the_walk_yields_every_synapse_of_a_three_block_chain_once_and_stops_at_the_end() {
        // Chain: block 0 (four synapses) → block 2 (slots 0 and 2) → block 1 (slot 3 only).
        let mut arena = [full_block(10, 5), SynapseBlock::new(), SynapseBlock::new()];
        assert!(arena[2].set_synapse(0, 20, 0x2000, 60_000, true));
        assert!(arena[2].set_synapse(2, 22, -0x2000, 0, false));
        assert!(arena[1].set_synapse(3, 33, i16::MIN, 2559, false));
        assert!(arena[0].link(2));
        assert!(arena[2].link(1));
        let (got, n) = collect::<8>(SynapseBlock::fan_out(&arena, 1));
        assert_eq!(n, 7);
        let targets: [u32; 7] = core::array::from_fn(|i| got[i].unwrap().target);
        assert_eq!(targets, [10, 11, 12, 13, 20, 22, 33]);
        let fifth = got[4].unwrap();
        assert_eq!(
            fifth,
            Synapse {
                block_idx: 2,
                slot: 0,
                apical: true,
                target: 20,
                weight_q1_15: 0x2000,
                delay_ticks: 60_000,
            },
            "a delay beyond the horizon is yielded unchanged: the loader rejects it"
        );
        assert_eq!(
            (got[1].unwrap().apical, got[6].unwrap().weight_q1_15),
            (true, i16::MIN)
        );
        let chain: [u32; 3] = {
            let mut c = SynapseBlock::chain(&arena, 1);
            let out = [c.next().unwrap(), c.next().unwrap(), c.next().unwrap()];
            assert_eq!(c.next(), None);
            out
        };
        assert_eq!(chain, [0, 2, 1]);
        let mut unit = DendriticSuperNeuron::new(1);
        assert!(unit.set_first_block(0));
        assert_eq!(unit.first_block(), Some(0));
        assert_eq!(unit.fan_out(&arena).count(), 7);
        assert_eq!(unit.chain(&arena).count(), 3);
    }

    #[test]
    fn a_cyclic_or_out_of_arena_chain_terminates() {
        let mut arena = [full_block(0, 1), full_block(4, 1)];
        assert!(arena[0].link(1));
        assert!(arena[1].link(0));
        assert_eq!(
            SynapseBlock::fan_out(&arena, 1).count(),
            8,
            "a cycle yields at most the arena's blocks"
        );
        assert_eq!(SynapseBlock::chain(&arena, 1).count(), 2);
        assert!(arena[1].link(7));
        assert_eq!(
            SynapseBlock::fan_out(&arena, 1).count(),
            8,
            "a link outside the arena ends the walk after the block that holds it"
        );
        assert_eq!(
            SynapseBlock::fan_out(&arena, 9).count(),
            0,
            "a head outside the arena"
        );
        assert_eq!(SynapseBlock::chain(&arena, 9).count(), 0);
        let empty: [SynapseBlock; 0] = [];
        assert_eq!(SynapseBlock::fan_out(&empty, 1).count(), 0);
    }

    #[test]
    fn set_synapse_link_and_the_unit_refuse_what_the_encoding_cannot_hold() {
        let mut b = SynapseBlock::new();
        assert!(!b.set_synapse(4, 1, 1, 1, false));
        assert!(!b.set_synapse(0, u32::MAX, 1, 1, false));
        assert_eq!(b, SynapseBlock::new(), "refused unchanged");
        assert!(b.set_synapse(1, 0, 7, 3, true));
        assert_eq!(
            (b.target(1), b.target_neuron_ids[1]),
            (Some(0), 1),
            "unit 0 is a target"
        );
        assert!(b.is_apical(1));
        assert!(!b.link(u32::MAX));
        assert!(b.is_end());
        assert!(b.link(0));
        assert_eq!(
            (b.next(), b.next_block_idx),
            (Some(0), 1),
            "block 0 can follow"
        );
        b.unlink();
        assert!(b.is_end());
        assert!(!b.clear_synapse(4));
        assert!(b.clear_synapse(1));
        assert_eq!(b, SynapseBlock::new());
        let mut unit = DendriticSuperNeuron::new(1);
        assert!(!unit.set_first_block(u32::MAX));
        assert_eq!(unit.first_block(), None);
    }

    #[test]
    fn tokens_and_messages_round_trip_and_the_oversize_is_refused_or_clamped() {
        let t = synapse_token(MAX_TOKEN_BLOCK, 3).unwrap();
        assert_eq!(t, MAX_TOKEN, "the largest token fits the wheel exactly");
        assert_eq!((token_block(t), token_slot(t)), (MAX_TOKEN_BLOCK, 3));
        let t = synapse_token(12_345, 2).unwrap();
        assert_eq!((token_block(t), token_slot(t)), (12_345, 2));
        assert_eq!(synapse_token(MAX_TOKEN_BLOCK + 1, 0), None, "finding F-23");
        assert_eq!(synapse_token(0, 4), None);
        assert_eq!(
            token_block(t | (0xF << 28)),
            12_345,
            "a coarse-ring residual in the top bits is ignored"
        );

        for (e, apical) in [
            (65_023, false),
            (-65_025, true),
            (0, true),
            (1, false),
            (-1, false),
        ] {
            let m = spike_message(e, apical);
            assert_eq!(message_efficacy_q16(m), e);
            assert_eq!(message_is_apical(m), apical);
            assert_eq!(m >> 19, 0, "bits 19 to 31 are zero");
        }
        assert_eq!(
            message_efficacy_q16(spike_message(i32::MAX, false)),
            (1 << 17) - 1
        );
        assert_eq!(
            message_efficacy_q16(spike_message(i32::MIN, false)),
            -(1 << 17)
        );
        assert!(
            spike_message(-1, false) < spike_message(1, false)
                || spike_message(-1, false) > spike_message(1, false),
            "messages have a total order"
        );
    }

    #[test]
    fn release_stores_the_efficacy_of_the_spike_for_delayed_delivery() {
        let mut b = SynapseBlock::new();
        assert!(b.set_synapse(0, 5, i16::MAX, 10, false));
        assert!(b.set_synapse(2, 6, i16::MIN, 0, true));
        assert_eq!(b.release(0, STP_U, STP_MAX), 13_004);
        assert_eq!(b.last_release_q16[0], 13_004);
        assert_eq!(
            b.release(1, STP_MAX, STP_MAX),
            0,
            "an empty slot releases nothing"
        );
        assert_eq!(b.release(4, STP_MAX, STP_MAX), 0);
        assert_eq!(b.release_all(STP_MAX, STP_MAX), [65_023, 0, -65_025, 0]);
        assert_eq!(b.last_release_q16, [65_023, 0, -65_025, 0]);
        let m = spike_message(b.last_release_q16[2], b.is_apical(2));
        assert_eq!(
            (message_efficacy_q16(m), message_is_apical(m)),
            (-65_025, true)
        );
    }

    fn one_synapse(weight: i16, prev_pre: u32) -> SynapseBlock {
        let mut b = SynapseBlock::new();
        assert!(b.set_synapse(0, 1, weight, 1, false));
        b.stamp_presynaptic(prev_pre);
        b
    }

    #[test]
    fn pre_before_post_potentiates_and_post_before_pre_depresses_by_the_window() {
        // Potentiation alone: the target fired `delta` after the previous presynaptic spike
        // and this presynaptic spike is at the same tick as that (no depression at zero).
        for (delta, amount) in [(1, 328), (500, 257), (2000, 123), (10_000, 2), (40_000, 0)] {
            let mut b = one_synapse(0, 1000);
            assert_eq!(
                b.step_stdp(0, 1000 + delta, 1000 + delta),
                amount,
                "delta {delta}"
            );
        }
        // Depression alone: no previous presynaptic spike; the target fired `delta` before.
        for (delta, amount) in [(1, 344), (500, 269), (2000, 129), (10_000, 3), (40_000, 0)] {
            let mut b = one_synapse(0, NO_SPIKE_ON_RECORD);
            assert_eq!(b.step_stdp(0, 5000 + delta, 5000), -amount, "delta {delta}");
        }
        // Both: previous pre at 1000, post at 1500, this pre at 3500.
        let mut b = one_synapse(0, 1000);
        assert_eq!(b.step_stdp(0, 3500, 1500), 257 - 129);
        // A post at the same tick as the previous pre is not after it; at the same tick as
        // this pre it is not before it.
        let mut b = one_synapse(100, 1000);
        assert_eq!(b.step_stdp(0, 1000, 1000), 100);
        assert_eq!(b.last_spike_tick, 1000, "step_stdp does not stamp");
        let mut b = one_synapse(100, 1000);
        assert_eq!(
            b.step_stdp(0, 2000, 2000),
            100 + 201,
            "the post fired 1000 ticks after the previous presynaptic spike"
        );
    }

    #[test]
    fn a_weight_saturates_at_both_ends_and_an_empty_slot_is_untouched() {
        let mut b = one_synapse(32_700, 1000);
        assert_eq!(b.step_stdp(0, 1001, 1001), i16::MAX);
        let mut b = one_synapse(-32_760, NO_SPIKE_ON_RECORD);
        assert_eq!(b.step_stdp(0, 1001, 1000), i16::MIN);
        let before = b;
        assert_eq!(b.step_stdp(1, 1001, 1000), 0);
        assert_eq!(b.step_stdp(4, 1001, 1000), 0);
        assert_eq!(b, before);
    }

    #[test]
    fn stamps_pair_correctly_across_the_tick_wrap_and_a_stale_stamp_pairs_with_nothing() {
        let mut wrapped = one_synapse(0, u32::MAX - 100);
        let mut plain = one_synapse(0, 100);
        assert_eq!(
            wrapped.step_stdp(0, 20, u32::MAX - 50),
            plain.step_stdp(0, 221, 150),
            "the same intervals across the wrap give the same weight"
        );
        assert_ne!(plain.weights_q1_15[0], 0);
        // A post stamp one tick in the future is older than 2^31 ticks: unpaired.
        let mut b = one_synapse(100, 1000);
        assert_eq!(b.step_stdp(0, 2000, 2001), 100);
        // A previous presynaptic stamp in the future pairs with nothing for potentiation, but
        // the depression against a real post still applies.
        let mut b = one_synapse(100, 3000);
        assert_eq!(b.step_stdp(0, 2000, 1999), 100 - 344);
        // No spike on record on either side changes nothing.
        let mut b = one_synapse(100, NO_SPIKE_ON_RECORD);
        assert_eq!(b.step_stdp(0, 2000, NO_SPIKE_ON_RECORD), 100);
    }

    #[test]
    fn step_stdp_all_updates_every_slot_against_its_target_then_stamps() {
        let mut b = SynapseBlock::new();
        for slot in [0, 1, 3] {
            assert!(b.set_synapse(slot, slot as u32, 1000, 1, false));
        }
        b.stamp_presynaptic(1000);
        let weights = b.step_stdp_all(3500, [1500, NO_SPIKE_ON_RECORD, 9, 3000]);
        assert_eq!(
            weights,
            [1000 + 257 - 129, 1000, 0, 1000 + 123 - 269],
            "slot 0 and slot 3 both ways at their own intervals, slot 1 nothing on record, slot 2 empty"
        );
        assert_eq!(b.last_spike_tick, 3500);
        assert_eq!(b.weights_q1_15[2], 0);
    }

    #[test]
    fn two_blocks_given_the_same_history_stay_identical() {
        let mut a = full_block(0, 1);
        let mut b = full_block(0, 1);
        let mut x = 0x2545_F491u32;
        let mut now = 1u32;
        for _ in 0..10_000 {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            now = now.wrapping_add(1 + (x >> 20));
            let posts: [u32; 4] =
                core::array::from_fn(|k| now.wrapping_sub((x >> (k * 7)) & 0x3FFF));
            assert_eq!(a.step_stdp_all(now, posts), b.step_stdp_all(now, posts));
            assert_eq!(a.release_all(STP_U, STP_MAX), b.release_all(STP_U, STP_MAX));
            assert_eq!(a, b);
        }
    }
}
