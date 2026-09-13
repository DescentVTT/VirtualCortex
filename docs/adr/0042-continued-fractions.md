---
status: accepted
date: 2026-09-13
decision-makers: VirtualCortex maintainers
depends-on: ADR-0016
---

# ADR-0042: Exact continued fractions on the scratchpad — the convergents of a polynomial continued fraction as the first sequencing of slots into an expression, overflow as the slot's flag; an exact tolerance test against a rational target; a bounded coefficient search whose pins come from an oracle outside the tree; the meet-in-the-middle search Specified

## Context and Problem Statement

`cortex-arithmetic` held one record, `ArithmeticScratchpadSlot`, with eight opcodes over 128-bit operands and explicit error flags, and its crate doc said "the sequencing of slots into an expression is Specified". The proposal of brief 021 asked for "meet-in-the-middle integer lattice searches for polynomial continued fractions and fundamental mathematical invariants over preallocated scratchpad slots, with strict iteration and error bounds (Ramanujan machine pattern)" and stated that these "remain Specified". No document in the tree mentioned a continued fraction, a lattice or a meet-in-the-middle search (finding F-31): the proposal asserted a status nothing carried.

Three facts shaped the answer. The convergents of a continued fraction are an integer recurrence, $p_n = a_n p_{n-1} + b_n p_{n-2}$ and $q_n = a_n q_{n-1} + b_n q_{n-2}$ (Wall 1948; Lorentzen and Waadeland 1992), so a slot that multiplies and adds exactly computes them exactly until they no longer fit, which the slot flags: this is the expression the crate's Specified sentence was waiting for, and the smallest one. An engine that holds no real number can still hold a target as two integers and decide exactly whether a convergent lies within a rational tolerance of it by cross-multiplication, which the slot also does. And the Ramanujan machine's contribution (Raayoni et al. 2021) is the search, not the arithmetic: enumerating polynomial coefficient tuples and matching their limits against a target is old, and what a match means is a conjecture for the prover, which is where §6.10 already sends what the native track cannot close.

## Decision Drivers

- ADR-0016 and the §5.2.31 table: the slot is the record; an expression over it is a rule, not a record.
- §8.1 and [ADR-0029](0029-structural-enforcement.md): exact arithmetic that must not saturate goes through the slot or a checked operation whose failure is a result.
- TC-4: no float anywhere, including the tests; a target near $e$ is a fifteen-digit rational and the oracle that produced it runs outside the tree.
- [ADR-0010](0010-measured-or-target.md): what a search over 2 401 tuples reports is stated as what it is; that a match is an identity is a conjecture until certified.
- "Latest ≠ Newest": continued fractions and their recurrence are two centuries old with known failure modes (convergents grow exponentially or factorially; equivalent fractions have different coefficients; a slowly converging fraction never reaches a tight tolerance at any affordable depth).

## Considered Options

1. **The recurrence through the slot; `within` by reduced cross-multiplication; `search` as a bounded enumeration comparing at the deepest decidable depth; the meet-in-the-middle Specified.**
2. The recurrence in plain `i128` with `checked_*`, the slot unused.
3. A general expression tree over slots (opcode sequences), with the continued fraction as one program.
4. The meet-in-the-middle search of the Ramanujan machine: rational functions of the target enumerated on one side, fractions on the other, matched by value.

## Decision Outcome

Option 1, in `crates/cortex-arithmetic/src/fraction.rs`.

- **`Poly`** = `[i64; 3]`, $c_0 + c_1 n + c_2 n^2$; `poly_at(c, n)` is exact for every 64-bit tuple and 32-bit $n$ (the magnitude is below $2^{127}$, shown in the code), so a polynomial never fails. **`MAX_DEPTH`** = 64, **`MAX_DEGREE`** = 2.
- **`convergent(a, b, depth, slot)`** walks the recurrence from $(p_{-1}, p_0, q_{-1}, q_0) = (1, a_0, 0, 1)$, each step two `OP_MUL` and one `OP_ADD` for $p$ and the same for $q$; a flag ends the walk as `FractionError::Arithmetic(flags)` with the slot left showing the operation, and a depth above the bound is `DepthExceeded`. **`deepest`** returns the last convergent before the first flag.
- **`within(c, target, tolerance, slot)`** reduces $p / q$ by their greatest common divisor and decides $\lvert p\,t_d - t_n\,q \rvert \cdot \lvert o_d \rvert \le \lvert o_n \rvert \cdot \lvert q\,t_d \rvert$ through the slot; a zero denominator is `ERR_DIVIDE_BY_ZERO`, a product that does not fit is `ERR_OVERFLOW`, both results.
- **`search(target, tolerance, degree, bound, depth, slot, out)`** enumerates every tuple with $\lvert c_i \rvert \le$ bound for $i \le$ degree (the lowest coefficient fastest, $a$ before $b$), skips a polynomial that is identically zero, walks each candidate to the depth or the first flag, and keeps the verdict of the deepest depth at which `within` could be decided; a candidate whose verdict is within is written to `out` with that convergent. `SearchError::Bound` for a degree above two, a negative bound, a depth above the bound, a zero denominator or a non-positive tolerance; `OutFull` when one more matched than `out` holds. **`Candidate::statement_hash`** is FNV-1a over the six coefficients, the `param_hash` a prover frame carries for it.
- **What the pins say** (computed by an oracle outside the tree before the tests were written; TC-4 keeps the oracle outside). For the target $2718281828459045 / 10^{15}$ within $10^{-12}$, degree 1, bound 3, depth 20 (2 401 tuples less the zero polynomials), exactly one candidate: $a = [3, 1, 0]$, $b = [0, -1, 0]$, the fraction $e = 3 - 1/(4 - 2/(5 - 3/(6 - \dots)))$, at depth 20 with $p = 2916471173788403280463$ and $q = 1072909785605898240000$; its walk fits to depth 31 and the step to 32 overflows. For $2414213562373095 / 10^{15}$, degree 0, bound 2, depth 40 (twenty tuples), exactly one: $a = [2, 0, 0]$, $b = [1, 0, 0]$, the silver ratio $1 + \sqrt 2$. A tolerance of one half over the same twenty tuples at depth 8 matches four, in enumeration order. The same target near $e$ at degree 2 and bound 1 matches nothing: $a_0 = 3$ is outside the bound.
- **What is not claimed.** That a match is an identity: the fraction near $e$ is one because the mathematics says so, not because the search found it; a caller that wants more than a conjecture sends the candidate's hash to the broker ([ADR-0043](0043-discovery-path.md), §6.10). That the search finds a fraction that converges slowly: $4/\pi$ as Brouncker's fraction is within $10^{-2}$ at depth 64 and no tighter, and the search reports it only under a tolerance that admits many others.
- **The meet-in-the-middle search** (option 4) is Specified: at this bound the direct comparison enumerates every tuple in well under a second, and no measurement asks for the larger space where the two-sided match pays.

### Consequences

- Good: §5.2.31's Specified sentence has its first instance, and every step of it is the slot's own opcode, so a wrong figure is a flag the caller sees.
- Good: a conjecture generator for the native track of R-10 exists that needs no float, no product and no search the caller cannot bound.
- Bad: the search is $O(\text{bound}^{2(\text{degree} + 1)} \times \text{depth})$ slot operations, 117 649 tuples at degree 2 and bound 3, and a caller that raises either pays for it between ticks.
- Bad: the target is an input; nothing in the tree produces one, and which constants an organism would ask about is the executive's Specified search.
- Bad: two tuples that are equivalent fractions with different coefficients are two candidates.

## Alternatives considered and why rejected

- **Option 2** computes the same integers and reports the same failures but leaves the record the crate exists for unused, so nothing would test the slot as an expression's element.
- **Option 3** is the general expression tree §8.8 names; a program interpreter over slots is a larger decision than one recurrence and no caller asks for it yet.
- **Option 4** needs the target as an exact rational on both sides and a sort or a hash over the enumerated values, a search structure the caller would have to hold; the direct comparison at this bound is the same answer sooner.
- **A float oracle inside the tests** fails TC-4 by the letter and by the intent: the tree would then hold a number it cannot recompute.

## Confirmation

`cortex-arithmetic`: the polynomial at its extremes, exactly; the convergents of the fraction near $e$ at depth 0, 1 and 20 and of the silver ratio at depth 40 and 64, pinned; the deepest convergent of the fraction near $e$ at depth 31 with the flag set, the step to 32 refused, the depth bound refused at 65, the largest constant term's second step refused; `within` at, just inside and just outside the tolerance, with signs ignored, every zero denominator and every overflow named; the search's two pins, the hash pinned and asymmetric in $a$ and $b$, the depth bound itself accepted (the candidate near $e$ then carries its depth-21 convergent, the deepest the comparison fits), nothing at degree 2 and bound 1, `OutFull` with an empty slice and with a slice one short of the four loose matches, every `Bound` case, twelve matches of a trivial target at depth zero; a property walk comparing `convergent` and `deepest` against a checked `i128` reference over 2 000 random polynomials with more than a hundred flagged. `npx spec-guard` asserts `fn convergent` and `fn search`. The mutation gate on the changed lines ([ADR-0030](0030-verification-governance.md)) passes in CI.
