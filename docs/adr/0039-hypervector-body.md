---
status: accepted
date: 2026-09-13
decision-makers: VirtualCortex maintainers
depends-on: ADR-0016
---

# ADR-0039: The hypervector body — a second record of `cortex-symbolic` under ADR-0016's test, 160 words of 64 bits, with the vector-symbolic algebra as integer rules over the words: binding by XOR, permutation by rotation, bundling by majority with a fixed tie-breaker, the Hamming distance, the nearest codebook entry and a decode confidence; no intrinsic, no `unsafe`; the width 10 240

## Context and Problem Statement

`cortex-symbolic` held one record, `SymbolicHypervectorHeader`, whose rules wrote metadata: `bind` stored a role id and a filler id, `blend` a blend's header, `rebase` a composed shift. The vector itself did not exist: whitepaper §5.2.9 said "The vector body (1 250 bytes at 10 000 bits) lives in a separate arena addressed by `vector_id`" and its status row "Bundling, permutation, unbinding, the codebook and the vector arithmetic of a blend: Specified" since 3.0.0; §8.8's row "Vector-symbolic architecture (Plate, Kanerva) | Binding by XOR / circular convolution, bundling by majority, permutation by cyclic shift, clean-up by nearest codebook entry | Specified"; and R-9's Layer 1, $\text{Concept} \approx S \otimes \text{Role}^{-1}$, "Specified". Two fields of the header, `hamming_distance_cache` and `confidence_score`, were written by nothing. The proposal of brief 020 asked for "the 10,000-bit hypervector body arena (1,250 bytes = 160 × `u64`, or aligned to 1,280 bytes = 20 cache lines)", "bit-exact, zero-allocation SIMD-vectorized unbinding using AVX-512 (`_mm512_xor_si512`), ARM SVE2, and portable 64-bit SWAR bitwise fallbacks", "associative codebook search using hardware `popcnt`", and Gärdenfors quality spaces "in saturating Q16.16, enabling geometric concept distance checks in $O(1)$".

Three facts shaped the answer. 160 words of 64 bits are 1 280 bytes and 10 240 bits, not 1 250 bytes: 10 000 bits are 156.25 words, and a record is a whole number of words and, under [ADR-0001](0001-64-byte-pod-records.md), of cache lines. Every `core::arch` intrinsic is an `unsafe fn` and `unsafe_code` is forbidden in every state crate by `[workspace.lints]` ([ADR-0029](0029-structural-enforcement.md), TC-9), while `core::simd` is nightly-only (TC-1); a loop of `u64` XORs and `count_ones` over 160 words is what the compiler vectorises for the target it builds for, and its result is an integer that is the same on every target, which is what T-1 holds. And a concept has one distance: the Hamming distance over bodies is the metric the codebook is read by, so a second metric on the same concept would be two rules for one quantity.

## Decision Drivers

- ADR-0016's six-part test for a second record in a crate: a gap (no record holds a vector; the header addresses one that does not exist), no other owner (§5.2.9 owns "10 000-dimensional bipolar hypervectors that bind spiking activity to discrete symbols with a clean-up codebook"), the mechanism in §8.8 with the layout, widths that fit (64-bit words), the boundaries of §1.5 and §8.10 untouched.
- ADR-0001: a record is a whole number of cache lines, aligned to one, asserted at compile time; this one is twenty.
- TC-9, TC-1 and §2.1: no intrinsic, no nightly, no `unsafe`; the compiler's vectorisation is neither measured nor promised, and a benchmark under `docs/benchmarks/README.md` is where a figure would come from.
- §8.3: the same words in give the same words out on every target; every rule is integer.
- Rule L-3 and §1.5: a body holds bits and a codebook index, never a word; the lexicon is the runtime's boundary.
- "Latest ≠ Newest": binary spatter codes and hyperdimensional computing (Kanerva 1996, 2009) and holographic reduced representations (Plate 2003) are decades old with documented failure modes (the capacity of a bundle falls with its count; a rotation by zero is the identity; an even bundle has ties).

## Considered Options

1. A body of 10 000 bits in 157 words, the last 240 bits reserved and MUST be zero, rotation masked at bit 9 999.
2. **A body of 160 words, 10 240 bits, every bit used; the header's `DIMENSIONS` following it; the algebra as `u64` loops with no intrinsic.**
3. Intrinsics under a new ADR admitting `unsafe` to `cortex-symbolic`, with `cfg(target_feature)` paths and a scalar fallback.
4. Bodies in the runtime alone (a `Vec<u64>` store), the state crate keeping the header only.

## Decision Outcome

Option 2.

- **`HypervectorBody`** (`cortex-symbolic`, `body.rs`; 1 280 bytes, `align(64)`, asserted at compile time as twenty lines): `words: [u64; BODY_WORDS]`, `BODY_WORDS` 160, `BODY_BITS` 10 240; word $w$ holds bits $64w$ to $64w + 63$, least significant first. `Default` is the zero body, `ZERO`, the identity of binding. Admitted under ADR-0016's test as above; the crate count stays thirty-two. No section of the image carries bodies yet: the arena takes section kind 46 when a runtime store composes it (Specified; each word little-endian, `BODY_WORDS` words per record, Appendix A row 9), and `FORMAT_VERSION` stays 13 (rule L-6 bumps it for a record a section carries).
- **The width.** `SymbolicHypervectorHeader::DIMENSIONS` is `BODY_BITS`, 10 240, where it was 10 000: Kanerva's figure is a nominal order of magnitude, not a property of the code, and a rotation over 10 000 of 10 240 bits would need a mask on every word and a well-formedness clause for no gain. `rebase` composes shifts modulo the header's `dimensionality`, whose value for these bodies is 10 240; the capacity model's row 9 is 1 280 bytes per body.
- **The algebra**, every rule a loop over the words with named operations, no allocation: `from_seed(seed)` (the 160 outputs of splitmix64 started at the seed: a dense pseudo-random body, the same on every target; two seeds give bodies about half the width apart); `bind(&other)` (XOR, its own inverse: $\text{Concept} \approx S \otimes \text{Role}^{-1} = S \otimes \text{Role}$, so unbinding is binding again); `permute(shift)` (rotation toward the high end by `shift` modulo `BODY_BITS`, the $\Pi^k$ the header's `permutation_shift` names; 0 and the width the identity; $k$ then $\text{BITS} - k$ the identity; distance-preserving); `bundle(items)` (the per-bit majority of one to `BUNDLE_MAX` = 15 bodies, counted in four bit-planes as carry-save adders and compared bit-sliced against the majority threshold; an even count adds `TIE_BREAKER`, `from_seed(TIE_SEED)`, as one more operand so that every bit has a majority and the rule is symmetric between the items; `None` for no item or sixteen; for independent items each is expected to agree with the bundle on more than half the bits, a property of the items and not of the rule: a body bundled with two copies of its complement is its complement); `hamming(&other)` (the population count of the XOR, 0 to `BODY_BITS`, a metric); `nearest(book)` (the index and distance of the nearest entry, the lowest index on a tie; `None` for an empty book); `confidence_q16(distance)` ($1 - 2d / \text{BODY\_BITS}$ floored at zero: 1.0 at a match, 0.5 at a quarter of the width, 0 at half and beyond). `SymbolicHypervectorHeader::record_readout(role, filler, distance)` binds the pair and writes `hamming_distance_cache` and `confidence_score`, the two fields nothing wrote.
- **What a bundle leaves** (the capacity case, `cortex-symbolic`'s tests; computed by an independent oracle before the test was written): a bundle of three role-bound fillers on a codebook of sixty-four seeded bodies, each role unbound, recovers its filler at distances 2 545, 2 556 and 2 501 (a quarter of the width, as three-way majority gives) with the nearest wrong entry above 4 900; a fourth role reads as noise at 4 979 from its nearest entry, a confidence of 0.028. The runtime's decode floor of 0.125 is a distance of 4 480 bits: twelve standard deviations of the noise (about 50 bits) below chance at 5 120, and about forty of a bound role's own spread (about 44 bits) above its readout near 2 560 (probed over thousands of seeded pairs, not Measured).
- **Intrinsics refused.** Whitepaper §8.10 names SIMD intrinsics as a future `unsafe`; this decision does not take it: the loops here are what the compiler vectorises, the result is bit-identical by construction, and no measurement shows an intrinsic beating the compiler on this width. The word "SIMD" is not a claim about the tree.
- **Gärdenfors coordinates not adopted.** A quality dimension in a vector-symbolic architecture is a level body bound to its dimension's role and read by the same distance (Specified in §5.2.9); a second coordinate metric on the same concept beside the Hamming distance would be two rules for one quantity.

### Consequences

- Good: the whitepaper's vector-symbolic row is Implemented in the crate that owns it, R-9's Layer 1 has its primitive, and the two header fields have a writer.
- Good: no dependency, no `unsafe`, no allocation, no intrinsic; the same integers on both CI targets, which the runtime's exit test pins.
- Good: the six-part test admitted a second record without a crate; the format did not move.
- Bad: a body is twenty lines, the first record in the workspace larger than one; the assertion is `64 × 20`, and the arena discipline (index-addressed, aligned) is the same.
- Bad: `bundle` is $O(15 \times 160)$ word operations plus a fixed body's generation each call, and `nearest` is $O(\lvert\text{book}\rvert \times 160)$: a codebook of a million entries (Appendix A row 9) is 160 M popcounts per readout, between ticks; an index over the codebook is the next decision when a measurement asks for it.
- Bad: the tie-breaker biases an even bundle toward one fixed body; for the runtime's frames (one to four roles) the even cases are two and four roles, and the exit test pins what they decode to.
- Bad: the width moved from a documented 10 000 to 10 240; a reader of Kanerva expects the former, and §5.2.9 says why.

## Alternatives considered and why rejected

- **Option 1** keeps a number that is not a property of the code at the cost of a mask on every rotation and a clause the loader would have to check.
- **Option 3** would be the workspace's second `unsafe` for a speed-up nobody has measured; §8.10's door stays open for the round that measures.
- **Option 4** puts the record the crate's responsibility names outside the crate, so no state-crate rule could be tested against a body; the runtime composes bodies, it does not own them.
- **A `[u8; 1280]` body** would make the popcount a byte loop; 64-bit words are what the target's population count and XOR take.
- **A random tie-breaker per call** would make a bundle depend on something outside `(image, seed, trace)`.

## Confirmation

`cortex-symbolic`: the record is twenty lines and the seeded words are splitmix64's (the first, second and last of seed 1 and the first of seed 0 pinned against an independent implementation); binding is an involution with the zero body its identity over the lattice's seeds and commutative; a rotation by 0 and by the width is the identity, by 1 moves every bit by one, by $k$ then $\text{BITS} - k$ is the identity, by 64 moves a whole word; the distance equals a per-bit count, is symmetric, zero on itself, the width on a complement, and counts the last bit; a bundle equals a per-bit oracle for one to fifteen items, one is itself, two agree-or-tie-breaker, three two-of-three, fifteen eight-of-fifteen, sixteen refused; the nearest entry is exact on the book, the lowest index on a tie, `None` on nothing; the capacity case above with every number pinned; the confidence at 0, 1, a quarter, half less one, half, half plus one, the width and `u32::MAX`; the readout's three fields; two property walks (bind round-trips, rotations compose modulo the width and preserve distance, the distance is a metric on sampled triples and seeded bodies sit near half the width; a bundle of any count equals the oracle and every item is closer than chance). `npx spec-guard` asserts `HypervectorBody`, `fn bundle` and `fn nearest`. The mutation gate on the changed lines ([ADR-0030](0030-verification-governance.md)) passes in CI.
