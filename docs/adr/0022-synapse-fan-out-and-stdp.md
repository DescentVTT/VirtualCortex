---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0013
---

# ADR-0022: Synaptic fan-out and STDP — index + 1 chains, synapse tokens, stored releases, and the nearest-neighbour pair rule at the presynaptic spike

## Context and Problem Statement

`SynapseBlock` held four targets, weights, delays, a chain index and a presynaptic stamp; nothing walked a chain, nothing updated a weight, and the whitepaper's R-1 step 6 ("fan-out: walk the chain from `synapse_slab_idx`; for each target → step 1") was Specified. Brief 013 asked for the chain's sentinel, the walk's bound, the encoding of a delayed delivery in the wheel's 28-bit token, what a zero delay does, and the form of spike-timing-dependent plasticity; and for milestone M3's exit test, a three-neuron delayed oscillator whose period is exact to the tick.

Four facts shaped the answer. A token is 28 bits ([ADR-0013](0013-timing-wheel-geometry.md)) and a block has four slots. The membrane targets a compartment (basal or apical, [ADR-0018](0018-membrane-integration.md)) and the block did not say which. The short-term-plasticity factors that scale a release are the presynaptic unit's ([ADR-0019](0019-short-term-plasticity.md)) and are not available to whichever worker's wheel delivers the spike later. And STDP needs the postsynaptic unit's last spike, which lives in another record.

## Decision Drivers

- An image at rest is zero (§8.7): a zeroed arena must read as empty, not as a chain of synapses to unit 0 ([ADR-0017](0017-mailbox-and-gate-protocol.md)'s encoding).
- The walk must terminate on a corrupt or cyclic chain without a visited set (no allocation, TC-5).
- A delayed delivery must carry, in 28 bits, enough to deliver the right efficacy to the right compartment of the right unit; the delivering worker has the token and the arena, nothing else.
- Determinism (§8.3): the same input trace must give the same arenas; a batch's order must not depend on arrival order.
- Every comparison of tick stamps is a wrapping difference read as signed (§8.4).
- No reverse index exists (which synapses land on a unit); STDP must run where the block is, at the presynaptic spike.

## Considered Options

1. Sentinel `u32::MAX`; token = target unit index; efficacy recomputed at delivery from the block and the presynaptic unit; STDP at the postsynaptic spike.
2. **Every index `index + 1` with zero as nothing; token = `block × 4 + slot`; the efficacy of the spike stored in the block per slot at the spike and read back at delivery; a compartment bit per slot; STDP as the nearest-neighbour pair rule at the presynaptic spike against the target's last somatic spike; format version 6.**
3. A per-synapse record (one per synapse, 16 bytes) with its own stamp and eligibility trace.

## Decision Outcome

Option 2.

- **Encoding.** `target_neuron_ids[k]`, `next_block_idx` and `DendriticSuperNeuron::synapse_slab_idx` store `index + 1`; zero is an empty slot, the end of a chain, and a unit without fan-out (`SLOT_EMPTY`, `CHAIN_END`). `set_synapse`, `link` and `set_first_block` refuse `u32::MAX`, which the encoding cannot hold; `target`, `next` and `first_block` decode. A zeroed block is `SynapseBlock::new()` and `Default`.
- **Walk.** `FanOut` yields every non-empty slot of every block of a chain, in block and slot order, as a `Synapse` (block, slot, compartment, target, weight, delay); `Chain` yields the block indices for a caller that mutates. Both stop at the chain's end, at an index outside the arena, and after `arena.len()` blocks, so a cycle terminates. A delay beyond the wheel's horizon is yielded unchanged: the loader rejects it (§6.2).
- **Token.** A delayed delivery is scheduled as `synapse_token(block, slot)` = `block << 2 | slot`, 28 bits, so a token names a block up to $2^{26} - 1$ (`MAX_TOKEN_BLOCK`; finding F-23). `token_block` and `token_slot` decode, ignoring a coarse-ring residual in the top bits.
- **Release.** At the presynaptic spike, `release(slot, u, r)` computes `synaptic_efficacy_q16(weight, u, r)` under the unit's current short-term-plasticity factors and stores it in `last_release_q16[slot]`; a delayed delivery reads it back, so what arrives is the efficacy of the spike that was sent, and the delivering worker needs only the block. A zero delay is delivered now, into the target's mailbox.
- **Message.** A delivery reaches a mailbox as `spike_message(efficacy, apical)`: the efficacy in 18-bit two's complement (bits 0–17; it is below 1.0 in magnitude, and a wider value is clamped) and the compartment in bit 18; `apical_mask` bit $k$ says which compartment slot $k$ lands in. Messages have a total order, and the executor orders a batch by it (§8.3).
- **STDP.** At a presynaptic spike at tick $t$, for each slot with the block's previous stamp $p$ and the target's last somatic spike $q$: if $p < q \le t$, the target fired after the previous presynaptic spike and the weight gains $A_+ (1 - 2^{-11})^{q - p}$; then, if $q < t$, the target fired before this spike and the weight loses $A_- (1 - 2^{-11})^{t - q}$. Every difference is `wrapping_sub` read as signed; a stamp of zero is no spike on record and pairs with nothing. $A_+ = 328/32768$, $A_- = 344/32768$ ($A_-/A_+ = 1.05$), $\tau = 2^{11}$ ticks (20.48 ms), the window by `stp_decay_factor_q16`, the product rounded to nearest, the weight saturating in $[-1, 1)$. `step_stdp` updates one slot and does not stamp; `step_stdp_all` updates every slot and then stamps; `stamp_presynaptic` stamps. The rule pairs each presynaptic spike with the target's *last* postsynaptic spike (nearest neighbour on both sides), so a target that fired twice between two presynaptic spikes is paired once.
- **Sequence at a spike** (R-1 step 6, the executor's, mirrored by the oscillator test): `step_stp` for the unit; for each block of the chain, `step_stdp_all(now, the four targets' last spikes)`, `release_all(u, r)`, then each slot delivered now (delay 0) or scheduled; at a token's delivery, `spike_message(last_release_q16[slot], is_apical(slot))` into the target's mailbox, `try_schedule`.
- **Layout.** `SynapseBlock` `[40..56)` `last_release_q16: [i32; 4]`, `[56]` `apical_mask: u8`, `[57..64)` reserved. Image format version 6: the meaning of every stored index moved by one and the reserved bytes became fields; a version-5 image MUST NOT be read as version 6.

### Consequences

- Good: a zeroed arena, a zeroed unit and a zeroed image are consistent without a fix-up pass; every "nothing" is zero.
- Good: delivery needs no cross-record read: the token, the block and the mailbox suffice; the efficacy delivered is the one released.
- Good: the oscillator's period is exact to the tick and reproducible: 1 527 ticks for delays (300, 500, 700), 3 793 for (2 559, 1, 1 200), 2 433 for (0, 1 500, 900), each the delays plus nine or eleven ticks of integration per hop, over a hundred steady cycles after a hundred of settling.
- Bad: the token addresses $2^{26}$ blocks (67 M, 4.3 GB), half of what Appendix A sizes the arena at; finding F-23 records it, with the amendment of ADR-0013 (the residual out of the token) as the way to widen it.
- Bad: a second presynaptic spike within the delay of the first overwrites the stored release; the second spike's efficacy is delivered for both. Bounded by the horizon (25.6 ms) and one interspike interval; a per-token efficacy would need a wider token.
- Bad: STDP reads the target's stamp from another record. In a single thread this is a read; under workers it needs the executor's tick barrier so that no turn writes the stamp while another reads it (brief 012's ADR names the invariant).
- Bad: a spike at a tick that is zero modulo $2^{32}$ leaves no STDP record; the next one does.

## Alternatives considered and why rejected

- **Option 1**: `u32::MAX` sentinels make a zeroed arena a chain of synapses to unit 0, so every loader and every test must fill sentinels first; a target-index token cannot say which synapse arrived, so the delivering worker would recompute the efficacy from the presynaptic unit's factors, which it does not have; STDP at the postsynaptic spike needs the reverse index that does not exist.
- **Option 3** quadruples the synapse arena's record count and is the three-factor rule's question (an eligibility trace per synapse), not this round's; the block's reserved bytes were enough.
- **Folding the axonal delay into the pairing** (the dendritic-delay convention) needs the arrival time of the previous spike per slot, which the block does not keep; the rule pairs somatic times and says so.

## Confirmation

Eleven unit tests in `cortex-core` (`dynamics/synapse.rs`): a zeroed block is empty and ends its chain; the walk yields every synapse of a three-block chain once, in order, with an over-horizon delay unchanged; a cyclic or out-of-arena chain terminates; the encodings refuse `u32::MAX`; tokens and messages round-trip, the oversize block is refused and the oversize efficacy clamped; a release is stored for delivery; potentiation and depression by the window's amounts at 1, 500, 2 000, 10 000 and 40 000 ticks; saturation at both ends; the same intervals across the tick wrap give the same weight and a stale stamp pairs with nothing; `step_stdp_all` updates every slot then stamps; two blocks with the same history stay identical over 10⁴ spikes. The M3 exit test `tests/oscillator.rs` runs three delay triples for two hundred cycles and asserts the period of the last hundred on every unit, the constancy of each hop's latency, the period as delays plus latencies, and a second ring's identical spike train. `npx spec-guard` asserts `fan_out`, `step_stdp` and `spike_message` exist and `FORMAT_VERSION` is 6.
