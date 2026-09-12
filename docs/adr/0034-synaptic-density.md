---
status: accepted
date: 2026-09-12
decision-makers: VirtualCortex maintainers
depends-on: ADR-0012
---

# ADR-0034: Synaptic density — the four-synapse block stands; eight-bit logarithmic weights, eight synapses per block and an in-engine far-memory synapse tier rejected; the density levers in order, each gated by a measurement

## Context and Problem Statement

An architectural proposal received on 2026-09-12 (brief 018) named a density frontier for the synapse arena: "eight-bit log-scale weight packing" to put eight synapses in a 64-byte block and "halve the memory footprint to ~2.15 GB per 268 M synapses", and "multi-tiered compaction (LLC hot cache vs. CXL Tier-2 cold synaptic arenas via clock-sweep WAL)". It rejected static compressed sparse row and column formats for a network with in-place plasticity and structural growth. Whitepaper quality goal 2 is memory density, and Appendix A row 3 sizes the block arena at 67 108 864 blocks, 4.29 GB, the most a delivery token can name (finding F-23). Should the block change?

The proposal's premise does not survive the block's byte accounting. `SynapseBlock` (ADR-0022, ADR-0032) holds four synapses in 64 bytes: per synapse 4 bytes of target index, 2 of weight (Q1.15), 2 of delay, 4 of stored release (Q16.16), 2 of eligibility trace (Q1.15), and a quarter of the block's chain word and presynaptic stamp (2 bytes). **The weight is 2 of a synapse's 16 bytes.** An eight-bit weight saves one sixteenth of the arena, not half. Eight synapses in a block need 32 bytes of targets alone, and the remaining 32 would have to hold eight weights, eight delays, eight traces, the chain and the stamp, with no stored release: the delay at 8 bits (a horizon of 2.55 ms, below every coarse-ring delay of ADR-0013), no release (the worker whose wheel delivers a token has neither the source's short-term-plasticity factors nor its weight at the spike, which is why ADR-0022 stores the release), and a trace of 8 bits, which ADR-0032 rejected because a pairing of 0.0100 is 1.3 LSB of Q0.7.

## Decision Drivers

- Quality goal 2 (density) against quality goals 1 and 3 (determinism, latency): a change to the weight's format touches the numeric model of §8.1 and every plasticity rule.
- ADR-0010: no figure is written as a finding without a measurement; nothing in the tree measures the block arena as the binding constraint (Appendix A's Tier 1 is 15.8 GB against T-2's 20 GB, and the plan depends on hypothesis H-1).
- ADR-0012's reason for Q1.15: in-place STDP moves a weight by a fraction of a percent, which a coarser format loses below one LSB; deterministic arithmetic admits no stochastic rounding.
- "Latest ≠ Newest" (§2.1): a memory tier the operating system pages transparently is admissible where the engine has nothing to build for it; one the engine must manage is not, before a measurement says the arena needs it.

## Considered Options

1. Eight-bit logarithmic weights, eight synapses per block (the proposal).
2. An in-engine far-memory synapse tier: cold blocks moved by the clock sweep into a second arena.
3. **The four-synapse block stands. The density levers are named in order with the measurement that admits each; none is taken now.**

## Decision Outcome

Option 3.

- **Eight-bit logarithmic weights are rejected.** A logarithmic range of 256 levels over 2^10 has a level ratio of about 1.027; a pairing amount of 1 % of full scale is below one level for most of the range, so in-place STDP (ADR-0022) and consolidation (ADR-0032) would lose their steps unless rounding were made stochastic, which the deterministic numeric model of §8.1 and §8.3 does not allow; ADR-0012 rejected Q8.8 for the same resolution reason, with 13 representable steps in a typical weight against 1 600 in Q1.15. The instruction the format was chosen for, a Q15 multiply-high with rounding (`pmulhrsw`, `sqrdmulh`), has no logarithmic form; a log-domain weight needs an exponential per synapse on the hot path. And the saving is one sixteenth of the arena, not half.
- **Eight synapses per block are rejected** for the three fields above (the delay's horizon, the stored release, the trace's precision); the block's other 14 bytes per synapse are what the delivery and plasticity rules read, not padding.
- **An in-engine far-memory synapse tier is not built.** A block is touched only when its unit fires (the fan-out phase, ADR-0023) or when a token names it at delivery; a block whose unit is quiet is a page nothing touches, which is what a transparently tiered memory (Linux memory tiering over a CXL expander, or NUMA balancing) demotes on its own. The engine's contribution is a placement hint at load, which is the `mmap` round's (FR-4, Specified); the clock sweep (ADR-0024) evicts units, not blocks, and stays so. A memory tier the engine would have to manage itself fails §2.1 until a measurement on the reference platform shows the arena is what binds.
- **Compressed sparse row and column formats stay rejected**, as ADR-0001 and ADR-0022 already have it: an insertion is a chain link in $O(1)$, and structural plasticity retires blocks under ADR-0011.
- **The density levers**, in the order they would be taken, each admitted only by the measurement named:
  1. *The stored release as the block's factors.* `last_release_q16` is 16 bytes per block, the largest field; the four releases of one spike are `synaptic_efficacy_q16(weight, u, r)` with the source's `(u, r)` at the spike, 2 bytes. Storing the pair and recomputing the efficacy at delivery frees 14 bytes per block, at the cost of widening ADR-0022's overwrite defect (a second presynaptic spike within the delay would deliver the second spike's weight as well as its factors). Admitted when T-2 on a reference image shows the block arena above its row and a delivery benchmark shows the recomputation within T-3's budget.
  2. *A column-relative target.* A 24-bit target index within a macro-column (row 1 of Appendix A; Specified) saves one byte per synapse and needs a second block type for inter-column synapses. Admitted when the column directory exists and the same T-2 measurement holds.
  3. *The token's width* (finding F-23): 26 bits of block index bound the arena at $2^{26}$ blocks; widening the token changes ADR-0013's residual and costs 512 KB per worker, admitted when a reference image needs more blocks than the token names.
- **What the whitepaper records.** §11.1 carries the hypothesis that the block arena is the binding constraint at the reference scale, with T-2 on a reference image as the test; §8.6 and Appendix A say the block layout stands.

### Consequences

- Good: nothing changes in the tree for this decision; the block's byte accounting is written where a proposal will meet it next time.
- Good: the levers are ordered by what they cost the rules (a wider overwrite window, a second block type, a wider token) and each names its measurement, so the next density round starts from a number, not a premise.
- Bad: a reference image that exceeds Appendix A's row 3 has no lever ready; the first lever is the round that measures it.

## Alternatives considered and why rejected

- **Q1.15 plus a shared per-block exponent** (ADR-0012's option 4): range on demand, which nobody has asked for; density unchanged.
- **Widening the block to 128 bytes for eight synapses**: two cache lines per fan-out step for the same bytes per synapse; the line is the transfer quantum (ADR-0001).
- **Dropping the trace** (ADR-0032's option 4 revisited): the three-factor rule has no memory without it.

## Confirmation

No code changes. `npx spec-guard` still asserts `weights_q1_15` and `SynapseBlock` in `cortex-core` (§5.2.1); the layout assertion holds the block at 64 bytes; brief 018's report states the accounting.
