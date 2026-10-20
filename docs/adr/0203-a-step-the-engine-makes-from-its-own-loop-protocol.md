---
status: proposed
date: 2026-10-20
depends-on: ADR-0202
title: A step the engine makes from its own loop, its protocol
kind: protocol
hypothesis: H-42
summary: H-42's protocol, committed before any lead-in with the probe set. The networks, the read, the probe and λ = 1/4 are ADR-0200's. A run with no pin is held by rule to H-41's twin and an oracle of the probe. The four predicted readings are rules over the tables, and a trial on seed 22 runs first.
decision-makers: VirtualCortex maintainers
---

# ADR-0203: A step the engine makes from its own loop, its protocol

**In brief**
- **Decision.** H-42 runs under `UNDER_THE_PROBE` in `tests/inhibition.rs`: H-41's settling with the probe set. Its clauses, constants and four predicted readings are tests' constants and rules from this commit on.
- **Why.** [ADR-0200](0200-a-step-the-engine-makes-from-its-own-loop.md) and brief 078 require them committed before any lead-in with the probe set, and fixed after it.
- **Left open.** The verdict, the readings and the next decision: [H-42's report](../research/H-42.md) and the measured ADR, after the one run.

## Context and Problem Statement

ADR-0200 wrote H-42 on 2026-10-20 before any run, and [ADR-0202](0202-a-step-the-engine-makes-from-its-own-loop-built.md) built the probe.
Brief 078 leaves the round five choices for the protocol:
- how a derivation with the probe set is held where none has a pin;
- how each reading is computed and pinned;
- how the harness sets the flag, every call of the tree the call it was;
- how the three derivations and six arms are dealt into tests and dispatches;
- whether a trial of the harness runs first.

H-41's harness is the base ([ADR-0195](0195-the-inhibitory-target-by-the-same-read-measured.md)): `derive_under`, `Settling`, `RuledSite` and `arm_on`.
Each network's twin is its H-41 run, pinned in `BOTH_*`.

## Decision Drivers

- **Nothing moves after a run**: clauses, constants and readings are code, held by the gate's test.
- **A rule where no pin exists**: a run with no table of its own is held to its twin and to an oracle, so a harness defect fails it, not a reading.
- **Every call of the tree the call it was**: the flag is unset in every settling and site but H-42's.
- **One run each**: the derivations and the arms run once, at `scope=arms`, after the calibration.

## Considered Options

1. **A derivation with no pin**: (a) held by rule — the read H-41's window for window, the probe by an oracle of ADR-0200's text, the rule after it at the oracle's step; (b) dumped and held by nothing until pinned.
2. **The oracle**: (a) written from ADR-0200's text in `i128`, $S = 64D / (2 \cdot 8 c^{*})$ and $\kappa = \lambda / S$, by doubling; (b) the engine's shift, $c^{*} \ll 12$ over $D$.
3. **A trial**: (a) one, on seed 22, the read's and the probe's 144 windows and no rule window, before the derivations; (b) none.
4. **The deal**: (a) each derivation, each arm and the trial its own `exhaustive` test, the derivations in one dispatch and the arms in one; (b) one test per network.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### The harness

- **`Settling::probe`** and **`RuledSite::probe`**, false in every settling and site but H-42's.
  - `candidate_under` writes `Config::step_probe` from it.
  - `Settling::settled` reads the windows after the read and the probe's sixteen, the probe's last window's sum and gain in the prior's place.
  - `derive_under` records the step the engine held after the lead-in (`Derivation::probe`), and the image carries it in the settling's step's place.
  - With `probe` false every call is the one it was.
- **`UNDER_THE_PROBE`** is `UNDER_BOTH_TARGETS` with `probe` set: the configuration's step 0x1000, no target, the read of 128 windows with both targets by it, H-39's settled network's rule within 320 windows.

### H-42's constants, restated from ADR-0200 and pinned in the tests

| Constant | Value | In the tests |
| :--- | :--- | :--- |
| The networks | seeds 22, 23 and 24 at 1 024 units | `PROBE_SEEDS` |
| The read | 128 windows, the last 16 summed | `PROBE_SPAN` |
| The probe | 16 windows, the 5th to the 12th higher | `PROBE_WINDOWS`, `PROBE_HIGHER` |
| The move | 1/64 | `PROBE_MOVE` |
| λ | 1/4 | `PROBE_LAMBDA` |
| The bound | 320 windows | `PROBE_BOUND` |
| The arms | H-25's two | `PROBE_ARMS` |
| The prediction | yes, and each reading | `PROBE_PREDICTED`, `PROBE_READINGS_PREDICTED` |

### The rules

- **The oracle** (option 2(a)): `probe_sum_by_hand`, `probe_gain_by_hand`, `steepness_by_hand` in hundredths, and `probe_step_by_hand`, refused at D ≤ 0 or past 0x8000. *Why not the engine's shift* (option 2(b)): a test that restates the code checks nothing.
- **`probe_read`** reads a lead-in's table: the read's sixteen windows and target, the windows of the read and the probe whose gain moved, the probe's sixteen windows, D, S, the step, and the rate's rule after the probe at that step (`rate_read`).
- **A derivation with no pin** (option 1(a), `probe_derivation_held`):
  - what it makes before the settled network, and its cancel, are the gain held's, as H-41's are;
  - its first 128 windows are H-41's lead-in on the network, in every field;
  - no window of the read or the probe moved the record's gain;
  - the read's sixteen windows, its target, period and count are H-41's;
  - the engine ended the probe in a step where the oracle makes one, and refused where it does not;
  - the inhibitory period is the read's, and the step the oracle's, or 0x1000 where the probe was refused;
  - every window after the probe is the rate's rule's at that step, and no window passed the train.
- *Why not option 1(b)*: a derivation held by nothing until pinned would pin a harness defect as a reading.

### H-42's clauses and verdict

- **Clause 0**, per network, is `Rules::hold` with the read and the probe folded into `settled`: the read and the probe each gave a value the rules accept, the lead-in settled after the probe, every later rule gave a value, and the frozen block passed both measures.
- **Clauses 1 to 4** are H-25's four in each arm, against the image that network's derivation wrote (`seeded`).
- **The verdict** is H-41's form (`probe_verdict`): yes when all five hold in both arms on all three networks.
- **The stopping rule's step** (`probe_stop`): 3 for yes, 4 for a no on clause 0 on a network, 5 otherwise.

### The four predicted readings as rules (`probe_predicted`)

| Reading | Rule | Where none |
| :--- | :--- | :--- |
| 1. Each network's S | $12 \le 64D / (16 c^{*}) \le 24$, both counted (`steepness_within`) | a network not pinned; false where its probe made no sum |
| 2. Each arm's opposite pairs | fewer than 540 of 959 (`opposite_pairs`) | no arm ran |
| 3. Each arm's mean count | within 1 % of its read target (`count_within`) | no arm ran |
| 4. Each arm's gain | within 5 % of 1.75 after every window (`gain_within`) | no arm ran |

### The tests and the dispatches

- **The gate's one test**, `the_networks_of_h_42_the_probe_s_step_and_its_predicted_readings_rules`:
  - the constants above;
  - the oracle at its edges;
  - `probe_read` and the settled network's rule over tables written by hand;
  - the four readings' rules at their edges;
  - the verdict and its step over tables written by hand;
  - `probe_tables` over the pinned tables, empty until the runs.
- **The trial** (option 3(a)), `the_probe_reads_its_step_after_the_read_on_seed_22_at_1024_units_exhaustive`:
  - the read's 128 windows held to H-41's row for row;
  - the probe's state, the record's gain and the gain the workers were given held to the oracle in each window;
  - the step held to the oracle at the 144th window;
  - no window of the rule runs.
  - *Why one*: H-41's first trial caught a harness defect before its one run ([ADR-0195](0195-the-inhibitory-target-by-the-same-read-measured.md)).
- **The derivations**: `a_step_from_the_engine_s_own_loop_on_seed_{22,23,24}_at_1024_units_exhaustive`.
- **The arms**: `the_address_drawn_under_the_probe_s_step_on_seed_{22,23,24}_from_the_assignment_at_1024_units_exhaustive` and `…_from_the_mirrored_assignment_…`.
- **The order** ([briefs/README.md](../../briefs/README.md#evidencing-a-round)):
  1. the calibration at `scope=exhaustive`, the ten tests above excluded, on the commit that carries this ADR;
  2. the trial at `scope=arms`, after the calibration is green;
  3. the three derivations at `scope=arms`, once, their tables pinned;
  4. the six arms at `scope=arms`, once, their tables, the verdict and the readings pinned;
  5. the reproduction at `scope=both`, as soon as those are pushed.

### What does not move after a run

λ, the probe's size and windows, the read, the step's bound, the networks, the task, the schedule, the clauses, the constants and the four readings.
There is no second attempt.

## Consequences

- Good: each run is held by rule before it has a pin, so a harness defect fails as a defect.
- Good: every call of the tree is the call it was: the flag is false in every settling and site but H-42's.
- Bad: the trial runs a lead-in with the probe set before the derivations. It reads seed 22's D and step, which its derivation then pins.
- Neutral: the arms' calibration runs each network's derivation again, as H-41's did.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `Settling::probe`, `RuledSite::probe`, `Derivation::probe`, `UNDER_THE_PROBE` and the `PROBE_*` constants, the rules above, `probe_derivation_held`, `probe_tables`, and the ten `exhaustive` tests and the gate's test named above.
- [H-42's report](../research/H-42.md) holds the hypothesis as ADR-0200 wrote it.
