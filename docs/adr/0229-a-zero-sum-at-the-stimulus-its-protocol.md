---
status: accepted
date: 2026-10-31
depends-on: ADR-0228
title: A zero sum at the stimulus, and a gentler depression, its protocol
kind: protocol
hypothesis: H-46
summary: H-46's protocol, committed before any arm runs. Six arms and six controls over sixteen schedules in two jobs, a row a schedule pinned from their logs, each control's first schedule held to H-42's arm in its own job, and a short form of each arm on the weekly job.
decision-makers: Menteion maintainers
---

# ADR-0229: A zero sum at the stimulus, and a gentler depression, its protocol

**In brief**
- **Decision.** H-46 runs as `zero_half` in `tests/inhibition.rs`: twelve runs of sixteen schedules, each two `…_paused` jobs of eight, the eighth image carried with its CRC-64. Its clauses, constants and four predicted readings are tests' constants and rules from this commit on.
- **Why.** [ADR-0226](0226-a-zero-sum-at-the-stimulus.md) and brief 085 require them committed before any arm runs, and fixed after it.
- **Left open.** The verdict, the readings and the next decision: [H-46's report](../research/H-46.md) and the measured ADR, after the one run.

## Context and Problem Statement

ADR-0226 wrote H-46 on 2026-10-28 before any run, and [ADR-0228](0228-a-zero-sum-at-the-stimulus-built.md) built its rules.
Brief 085 leaves the round to choose how the runs are dealt into tests, how each reading is computed and pinned, and how the calibration's check of the stopping rule's step 2 is a test.

Explorations 011 to 013 ran this path on seed 22 and are its base: `long_schedule`, H-43's twin on time, each schedule ending in a quiet run, its image written, and the next engine decoded from it. Their harness read far more than H-46 asks, printed it whole, and was not reviewed. Here it is written again, narrower.

## Decision Drivers

- **Nothing moves after a run**: clauses, constants and readings are code, held by the gate's test.
- **One run each**, at `scope=arms`, as the job's bound allows: an eight-schedule job took 5 517 to 6 081 s in exploration 013.
- **A schedule is held bit for bit**, or a sixteen-schedule run could drift unseen between its pinned numbers.
- **The tree takes what it must**: twelve images, and no more tables than the clauses and readings read.

## Considered Options

1. **The step 2 check**: (a) in each control's first half, its first schedule held to H-42's arm before its second runs, the job failing there; (b) six weekly tests more; (c) H-42's weekly arms alone.
2. **What is pinned**: (a) a row a schedule, with its trace, its values' hash and the CRC-64 of the image it wrote; (b) every block's table.
3. **The engine a schedule decodes**: (a) `kept_from`'s configuration, the explorations'; (b) H-42's `arm_on`.
4. **A trial of the harness**: (a) one test of two short schedules dispatched before this commit; (b) none.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**
Committed before any dispatch ran an arm, in brief 085's pull request.

- *Why not 1(b)*: ADR-0226 gives a control's first schedule no short form: it is H-42's arm, already weekly.
- *Why not 1(c)*: H-42's arms do not run this harness, and the step asks for the control's own first schedule.
- *Why not 2(b)*: 23 040 rows of a 12-field tuple to hold what a schedule's three hashes hold bit for bit. The blocks stay in each job's log.
- *Why not 3(b)*: `arm_on` has no schedule after its first, and `kept_from` is H-42's arm bit for bit under the default, by H-43's twins.
- *Why not 4(b)*: a harness that fails in its fourth hour costs a day of the organisation's places. The trial's two blocks a schedule read no clause, and decide nothing.

### The run

`zero_half(n, arm, form, half)`, one body for the arms and the controls:
1. **The calibration of the network**: `probe_made`, its derivation run again and held to H-42's pinned tables, in every job.
2. **The groups** (`zero_groups`): each stimulus set's excitatory units, the readout units of both sets, excitatory and inhibitory, as targets, and each held value read from H-42's image and held to H-42's pinned couplings.
3. **Each schedule** (`zero_schedule`): the image decoded under H-25's configuration with two addresses kept and the form's exponent; under an arm, the groups set, every fraction at a half; H-25's arm, the drawn delivery, H-20's 7 680 trials and three flips, every reward at age zero.
4. **Its end**: quiet, the image written, the row made and dumped before anything is held.
5. **The carry**: a first half prints its eighth image whole in hexadecimal with its CRC-64. It enters `tests/h46/`, and a second half reads it and holds it to `ZERO_IMAGES_CRC`.

| Network | Group A's held value | Group B's held value |
| :--- | ---: | ---: |
| Seed 22 | 12 470 305 | 12 922 530 |
| Seed 23 | 12 763 347 | 13 246 451 |
| Seed 24 | 12 698 180 | 13 107 374 |

The fractions are in no image, so each schedule starts them at a half, as the explorations' did.

### H-46's constants, restated from ADR-0226 and pinned in the tests

| Constant | Value | In the tests |
| :--- | :--- | :--- |
| The networks | seeds 22, 23 and 24 at 1 024 units, H-42's | `ZERO_SEEDS` |
| The arms | H-25's two on H-20's schedule | `ZERO_ARMS` |
| The schedules, a job's | 16, 8 | `ZERO_SCHEDULES`, `ZERO_HALF` |
| The exponent | a quarter; the controls one | `ZERO_EXPONENT` |
| The rounding | each synapse's fraction carried | `carried_weight`, the one form built |
| The prediction | yes, and each reading | `ZERO_PREDICTED`, `ZERO_READINGS_PREDICTED` |
| Readings 1 and 4's schedules | 9 to 16 | `ZERO_LATE` |
| Reading 1's bound | `arm × 4 ≤ control × 5` | `ZERO_SPEED_QUARTERS` |
| Reading 2's bound | at most 1.25 of the image's | `ZERO_HIGHEST_QUARTERS` |
| Reading 3's bound | 16 blocks, half or more in the bottom eighth | `ZERO_SPLIT_REVERSALS`, `ZERO_BOTTOM_EIGHTH` |
| Reading 4's band | 0.45 to 0.65 of the image's | `ZERO_INHIBITORY_HUNDREDTHS` |

### H-46's clauses and verdict

| Clause | Rule |
| :--- | :--- |
| 1, each arm | H-25's four (`kept_clauses`) in every one of its sixteen rows, every coupling against H-42's image on its network (`zero_clauses`) |
| 2, each control | H-25's second false in one of its sixteen rows (`zero_passed`) |

- **The verdict** (`zero_verdict`): yes when both hold in all twelve runs.
- **The stopping rule's step** (`zero_stop`), the first that applies: 3 for yes; 4 for a coupling in clause 1; 5 for a mapping or a reversal; 6 for the excitatory sum outside the couplings; 7 otherwise.

### The four predicted readings as rules (`zero_predicted`)

| Reading | Rule |
| :--- | :--- |
| 1. Each arm's speed | `reversals_summed` over schedules 9 to 16 within 1.25 of its control's (`zero_speed_held`) |
| 2. Each arm's highest coupling | every coupling's highest over all sixteen at most 1.25 of the image's (`zero_highest_held`) |
| 3. The reversals and the split | in every arm and control, each schedule whose starting image has half or more of the four couplings' synapses in the first two sixteenths of the width sums its reversals at 16 or fewer (`zero_split_held`); none where no schedule of any run does |
| 4. Each arm's inhibitory sum | its lowest and highest at the blocks' ends of schedules 9 to 16 within the band of the image's (`zero_inhibitory_held`) |

None is read until every run is pinned.

### A schedule's row

`ZeroRow`, in `tests/inhibition/pins/h46.rs`, one a schedule: the trial sequence's hash, the values' hash and the CRC-64 of the image written; H-25's four clauses; each mapping's last 128 correct and its crossing; each coupling's highest and lowest and the inhibitory sum's over the blocks' ends; each coupling's most synapses at the rail at a block's end; and the image by sixteenths of the width, each coupling's. `ZERO_IMAGE_BINS` holds H-42's image the same way, what the first schedule starts from.

### The tests and the dispatches

- **The gate's one test**, `the_zero_sum_of_h_46_its_clauses_and_its_predicted_readings_rules`: the constants above; both clauses, the stopping rule's steps and the readings' rules at their edges over tables written by hand; `zero_tables` over the pinned tables, empty until the runs.
- **The twelve first halves and twelve second halves**, `…_paused` from this commit ([ADR-0127](0127-the-paused-line-leaves-the-weekly.md)): `a_zero_sum_and_a_quarter_s_exponent_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_{first,second}_half_paused`, and `the_default_over_sixteen_schedules_on_…` the same.
- **The six short forms**: `a_zero_sum_and_a_quarter_s_exponent_s_first_schedule_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, each its arm's first schedule, held to its first row.
- **The step 2 check**: `zero_first_is_h42`, in each control's first half, on its first schedule: blocks, trace and every value. It fails the job, so that the job's conclusion reads it, and no arm's log is read before all six are green.
- **The trial** `the_harness_of_h_46_over_two_short_schedules_paused`, on the commit before this one, at `scope=arms`, run [41197365047](https://github.com/Menteion/Menteion/actions/runs/41197365047) on `7b3641f`: two schedules of two blocks on seed 22, the control's first H-42's arm as far as it runs, the groups' sums within a thousandth of their values. It took 193 s and held.
- **The order** ([briefs/README.md](../../briefs/README.md#evidencing-a-round), and ADR-0226's two departures):
  1. on the commit that carries this ADR, the calibration at `scope=exhaustive` with the six short forms excluded, and the twelve first halves at `scope=arms`, dispatched together;
  2. the calibration's shards read first, then the six controls' jobs; no arm's log is read before all are green;
  3. a commit that adds the twelve images, their CRC-64s and nothing else that runs; the twelve second halves on it;
  4. the rows pinned from both logs;
  5. as soon as those are pushed, the six short forms at `scope=exhaustive` with `only`, and the twenty-four halves again at `scope=arms`, every pinned row held.

### What `npm run round:additive` read on this commit

Exit 1: the change does not only add, so the calibration runs every test but the round's own ([ADR-0215](0215-the-calibration-a-change-needs.md)).

```text
crates/menteion-core/src/dynamics/mod.rs: a file a Rust build reads, outside a package's tests/
crates/menteion-core/src/dynamics/synapse.rs: a file a Rust build reads, outside a package's tests/
crates/menteion-core/src/lib.rs: a file a Rust build reads, outside a package's tests/
runtime/menteion-runtime/src/executor.rs: a file a Rust build reads, outside a package's tests/
runtime/menteion-runtime/src/lib.rs: a file a Rust build reads, outside a package's tests/
runtime/menteion-runtime/tests/contention.rs:57: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/differential.rs:374: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/inhibition.rs:237: 2 line(s) of the base removed or changed
runtime/menteion-runtime/tests/inhibition.rs:43468: an item named `Bins`, a name the base already uses
check-additive: e00b9bc...<this commit> does not only add, 9 reason(s): the calibration runs every test (ADR-0215)
```

### What does not move after a run

The exponent, the rounding, the groups and their held values, the arms, the controls, the networks, the schedules, the clauses, the constants and the four readings.
There is no second attempt.

## Consequences

- Good: a schedule's three hashes hold a run bit for bit in a row, and its tables stay small enough to read.
- Good: a control not H-42's stops its own job at once, and the job's conclusion is the reading.
- Bad: twenty-four jobs of about 100 minutes on AArch64, and the same again to reproduce them; every job runs its network's derivation again.
- Bad: reading 3's rule reads a schedule's start, so the sixteenth image is pinned and no schedule reads it.
- Neutral: the eighth images are 590 656 bytes each, about 7 MB, as ADR-0226 counted.

## Confirmation

- `runtime/menteion-runtime/tests/inhibition.rs`: the `ZERO_*` constants, the rules above, `zero_half`, `zero_short`, `zero_tables`, and the tests named above.
- `runtime/menteion-runtime/tests/inhibition/pins/h46.rs`: the tables, empty until the runs.
- [H-46's report](../research/H-46.md) holds the hypothesis as ADR-0226 wrote it.
