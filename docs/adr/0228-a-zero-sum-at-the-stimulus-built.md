---
status: proposed
date: 2026-10-31
depends-on: ADR-0226
title: A zero sum at the stimulus, and a gentler depression, built
kind: built
summary: '`Executor::set_zero_sum` holds each group''s couplings at a value the host gives, each synapse''s fraction carried; `Config::depression_exponent` shapes the excitatory depression at one, a half or a quarter. Unset, every run is the run it was, and neither rule is in the image.'
decision-makers: Menteion maintainers
---

# ADR-0228: A zero sum at the stimulus, and a gentler depression, built

**In brief**
- **Decision.** [ADR-0226](0226-a-zero-sum-at-the-stimulus.md)'s two rules are built: `Executor::set_zero_sum` in the runtime, and `DepressionExponent` in `menteion-core`, a field of `Config`. The explorations' other forms are not taken: no ceiling, no other rounding, no bound at consolidation, no soft bound and no readout's sum held.
- **Why.** H-46 asks both rules together. It needs them with every run unchanged where they are unset, each held by tests over the lattice.
- **Left open.** H-46's protocol is the next ADR, and its verdict the measured ADR's.

## Context and Problem Statement

ADR-0226 decided the rules, in its words: *"each of its synapses' weights times the held value over the sum, plus the fraction that synapse's last scaling dropped, rounded down, no weight past the rail"*, and *"The excitatory depression scales with $(M / M_{ref})^{\mu}$, the fractional powers by integer square roots."*

What was read first (principle 2), on 2026-10-31 at `main` `e00b9bc`:
1. **The explorations' code**, written for explorations and not reviewed: `Executor::set_zero_sum`, `set_zero_sum_rounding`, `zero_sum_round` and `rounded_weight` on the tags `explore/011-zero-sum-at-the-stimulus` and `explore/012-the-zero-sums-rounding`; `DepressionExponent` and `depression_magnitude` on `explore/013-the-carried-sum-and-the-exponent`.
2. **ADR-0055's depression** (`synapse.rs`): `depression_at`, read by `step_stdp` at the slot's magnitude before the pairing. `step_stdp_all`'s signature is the benchmark's, under a protected path.
3. **Where a weight moves in a tick**: `consolidate_each` in `phase_fan_out` alone.

## Decision Drivers

- ADR-0226's rules, and brief 085's standing directives: no record, no section and no format move, and nothing enters the image.
- **Unset, every run the run it was**, bit for bit.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)), and a new rule over the lattice.
- **A group's sum can pass what an `i64` scales**: the wheel's token bounds the arena at $2^{26}$ blocks, so $S$ reaches past $2^{42}$.

## Considered Options

1. **The carried quotient**: (a) in `i128`, from $w V 2^{32} / S$; (b) in `i64`, from the quotient and the remainder apart, as exploration 012 wrote it.
2. **Unset**: (a) no groups; (b) an `Option` beside them, as the explorations' form was.
3. **The exponent**: (a) a field of `Config`, plain in `Shared`; (b) a setter and an atomic, as exploration 013's.
4. **The depression's signature**: (a) `step_stdp_shaped` and `step_stdp_all_shaped`, the rule's two calling them at `One`; (b) a parameter on `step_stdp_all`.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**
Brief 085's empowerment names each choice: the names, how a group's sum is read and where the fractions live, and the integer form of each power.

- *Why not 1(b)*: above $S = 2^{31}$ the remainder times $2^{32}$ saturates, and the fraction with it. The two are the same integer below that: $\lfloor w V 2^{32} / S \rfloor$ is $q 2^{32} + \lfloor r 2^{32} / S \rfloor$.
- *Why not 2(b)*: with one form left an `Option` names nothing that no groups does not.
- *Why not 3(b)*: an exponent that moves between ticks has no use here; `Config` is how a rule of the engine's is set, and an engine decoded from an image takes the configuration's.
- *Why not 4(b)*: `benches/**` is protected, and the rule's own signature stays the rule's.

### The zero sum (Implemented)

| Item | What it is |
| :--- | :--- |
| `Executor::set_zero_sum(groups, targets)` | each group its source units and its value, between ticks, from the next tick; every fraction set to a half; no groups unsets it |
| Refused whole, nothing changed | more than `ZERO_SUM_GROUPS_MAX` (64) groups, a value below zero, a unit outside the arena, a source not excitatory, a source named twice |
| A touch | a worker whose consolidation moved a group's synapse onto a held target sets the group's bit, one `fetch_or` |
| The round (`hold_zero_sum`, `zero_sum_round`) | after the tick's fourth barrier, on the coordinator, before the spikes merge: each touched group's chains walked, its synapses onto the targets summed, each scaled unless the sum is the value |
| The weight (`carried_weight`) | $\lfloor w V / S + f \rfloor$ within `[0, i16::MAX]`, $f$ in $2^{-32}$ LSB, the fraction left over kept; at a sum of zero nothing moves |
| `Executor::zero_sum_groups` | how many groups are held |

The fractions are a `Vec<u32>` of the executor's, one for each slot of the arena, empty while unset, so an engine that sets no group allocates none.
Every tick the workers read a unit's group once a spike, and copy a block's weights once a block.
Neither the groups nor the fractions are in the image, and the format stays 26: a reloaded image, a fork among them, needs its groups set again.

### The exponent (Implemented)

| Item | What it is |
| :--- | :--- |
| `DepressionExponent` | `One`, the default and ADR-0055's rule; `Half`; `Quarter` |
| `Config::depression_exponent` | read by every worker in the fan-out phase, and `Executor::depression_exponent`; not in the image, so a decoded engine's is its configuration's |
| `depression_magnitude` | $M$; $\lfloor \sqrt{2^{13} M} \rfloor$; $\lfloor \sqrt{\lfloor \sqrt{2^{39} M} \rfloor} \rfloor$, which is $\lfloor (2^{39} M)^{1/4} \rfloor$ exactly |
| `step_stdp_shaped`, `step_stdp_all_shaped` | `step_stdp` and `step_stdp_all` under an exponent; those two are these at `One` |

Neither rule enters `Config::default()` or `Config::reads_its_targets()`, and a test holds both at `One`.
The potentiation, the inhibitory rule and the consolidation do not read the exponent.

### Its tests

- **By hand at the edges**:
  - `each_exponent_is_the_floor_of_its_power_at_every_magnitude_and_one_is_the_magnitude`, in `menteion-core`, at zero, the reference, the rail and one LSB;
  - `a_carried_weight_by_hand_at_its_edges`, at a quotient just under one, a carry from just under one, the rail, zero, a sum of zero and a product past an `i64`;
  - `a_round_scales_a_group_s_synapses_onto_the_targets_and_no_other`, either way, at its value and at a sum of zero;
  - `a_zero_sum_is_set_whole_or_refused_and_its_fractions_start_at_a_half`;
  - `a_group_is_held_after_a_tick_that_moved_it_and_no_other_is` and `the_last_group_has_its_own_bit`, on an engine.
- **Over the lattice against an oracle**:
  - the same `each_exponent_…` test holds every magnitude of the width against its power's integer oracle, and monotone;
  - `every_carried_weight_is_the_exact_rational_s`, the weight and the fraction from the rational over $S 2^{32}$, over the lattices and 200 000 steps;
  - `an_exponent_moves_the_excitatory_depression_alone_and_one_is_the_rule`, a pairing's trace from the depression written out, and `One` and the inhibitory rule `step_stdp`'s bit for bit.
- **Tick by tick**: `over_a_walk_the_engine_s_zero_sum_is_the_host_s_exact_rule_tick_by_tick`, twelve walks of 4 000 ticks on one worker and two and under both services, every block of the arena and every fraction after every tick equal to a twin on which the host applies the exact rule; and an engine whose groups were set and unset equal to one that never had any.
- **The configuration and the image**: `the_exponent_is_the_configuration_s_and_neither_rule_is_in_the_image`.

## Consequences

- Good: unset, the zero sum costs a byte load a spike and a four-weight copy a block, and the exponent one match a depression.
- Good: a group's sum may pass $2^{31}$ with no saturated fraction.
- Bad: a round's `i128` division per synapse is slower than exploration 012's `i64`; not timed apart, and a round ran between ticks at 19.8 to 20.8 µs there.
- Bad: a host that reloads an image, or forks one, must set its groups again.
- Neutral: the arena of a group's chains is walked each round, not listed at the set: a chain never moves, and the explorations' walk read about 1 600 synapses a group.

## Confirmation

- `crates/menteion-core/src/dynamics/synapse.rs`: `DepressionExponent`, `depression_magnitude`, `step_stdp_shaped`, `step_stdp_all_shaped` and the tests above.
- `runtime/menteion-runtime/src/executor.rs`: `Config::depression_exponent`, `set_zero_sum`, `hold_zero_sum`, `zero_sum_round`, `carried_weight`, `ZERO_SUM_GROUPS_MAX` and `mod zero_sum_prop`.
