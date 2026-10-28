---
status: accepted
date: 2026-10-28
depends-on: ADR-0224
title: A unit served on arrival, built
kind: built
summary: 'quiet_cannot_fire and quiet_catch_up in menteion-core; the executor leaves a unit that cannot fire off the schedule and catches it up bit for bit at its next turn and before the arena is read; Executor::caught_up beside turns; Executor::per_tick the tests'' reference. Every test held every pin.'
decision-makers: Menteion maintainers
---

# ADR-0225: A unit served on arrival, built

**In brief**
- **Decision.** [ADR-0224](0224-a-unit-served-on-arrival.md)'s service is built. A unit that cannot fire with no input leaves the schedule, asleep, and its quiet ticks' exact catch-up brings it to the present at its next turn and before the unit arena is read. Per-tick service stays as the tests' reference, `Executor::per_tick`.
- **Why.** One dispatch of every test held every pin on the round's last code commit, `8aac5c8`, and its longest shard took 62 of its 240 minutes. On AArch64 the 16-block workload read 0.74 to 0.76 of per-tick service's seconds, and the sparse drive 0.81 to 0.90.
- **Left open.** A read of the arena on every tick catches every asleep unit up a tick, and two tests that read it so took longer. A catch-up in fewer steps than ticks, and where AArch64's time goes, are as ADR-0224 left them.

## Context and Problem Statement

ADR-0224 decided, in its words:
- *"A unit not at rest leaves the schedule at the end of its turn when `quiet_cannot_fire` admits it."*
- *"Before the arena is read between ticks, every asleep unit is brought up to date; `Executor::sweep` is among the reads. Before it is written, every asleep unit is brought up to date and put back on the schedule."*
- *"The round brings asleep units up to date under `&mut self`, `Executor::units` taking it, or under ADR-0023's invariant restated for the write, with its test, in its ADR."*

What was read first (principle 2), on 2026-10-28 at `main` `9e97c13`:
1. **Exploration 008's code**, on the tag `explore/008-a-unit-served-on-arrival` at `e547c45`, written for an exploration and not reviewed. Its property tests over `testkit/prop.rs`'s lattice took 18.7 s in the debug profile. `menteion-core`'s suite took 0.56 s, and its 573 mutants each run that suite in the weekly mutation sweep.
2. **A turn with no mail** (`Worker::turn`): `integrate(0, 0, t)` and the gate. Phase 2 reads another unit's `last_soma_spike_tick` alone, which no quiet tick moves.
3. **The arena between ticks**: `Executor::units` takes `&self`, and about 185 call sites read it, `Image::encode` and the harness among them. The executor was `Sync`.
4. **The critic's weights** are written through `units_mut` at every reward.

## Decision Drivers

- ADR-0224's service, with no rule, record, constant, image byte or test assertion moved, and the format 26.
- **Every pin held**, by one dispatch of every test on the round's last code commit.
- **No `unsafe` beyond ADR-0023's** unless its invariant is restated for the write with its test.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)), and no new test that slows the weekly mutation sweep by more than its suites can carry.

## Considered Options

1. **The catch-up**: (a) `quiet_catch_up` as exploration 008 wrote it; (b) the same with the threshold's loop rewritten with no early exit; (c) `integrate(0, 0, t)` once a tick.
2. **The arena brought up to date between ticks**: (a) under `&self`, ADR-0023's invariant restated and the executor not `Sync`; (b) `Executor::units` taking `&mut self`.
3. **A reward's write of the critic's weights**: (a) a write as any other, waking every asleep unit; (b) a write of `value_weight` alone that leaves them asleep.
4. **The reference**: (a) `Executor::per_tick(self) -> Self`, applied to an executor just built or loaded; (b) a constructor beside `Executor::new` and one beside each loader.
5. **The property tests**: (a) exploration 008's in the default suite; (b) tests cheap in the debug profile that catch every mutant, and exploration 008's full strength as whole-domain tests.

## Decision Outcome

**Options 1(b), 2(a), 3(a), 4(a) and 5(b).**
Built in `07c3457`, `e75d84a` and `8aac5c8`, in brief 084's pull request #253.
`affectedFiles` was not widened.

- *Why not 1(a)*: its threshold loop broke on `left == 0 || e == 0`, and a step after either moves nothing, so `||` to `&&` is a mutant no test can catch. 1(b) bounds the loop by the first step and by the ticks instead.
- *Why not 1(c)*: ADR-0224's table, 0.907 and 0.910 of the arm's time, against 0.581 and 0.771.
- *Why not 2(b)*: about 185 call sites, and every function that reads the arena through `&Executor` would take `&mut`. 2(a) holds the same exclusion by the type.
- *Why not 3(b)*: a reward wakes the units once a trial, 7 680 ticks in the arm's 125 829 620 (exploration 008), and 3(a) needs no rule about which fields a write may touch.
- *Why not 4(b)*: the reference is the tests', and a consuming method keeps it off `Config`, the image and every loader.
- *Why not 5(a)*: 18.7 s for each of `menteion-core`'s 573 mutants in the weekly sweep. Under 5(b) the suite takes 0.68 s.

### The service (Implemented)

| Item | What it is |
| :--- | :--- |
| `DendriticSuperNeuron::quiet_cannot_fire` | true when $\max(s, 0, \lceil (B + A)/2 \rceil) < \min(\theta, \theta_0)$, with $\lceil (B + 4A)/5 \rceil$ in the maximum while the burst bit is set, or $\theta \le 0$; a quiet tick keeps it true |
| `DendriticSuperNeuron::quiet_catch_up(Δ)` | the fields Δ calls of `integrate(0, 0, t)` leave, and the turns per-tick service serves in them; the potentials stepped, the windows and the plateau counted down, the threshold by segments |
| The asleep map | one bit a unit in the schedule's layout, written by the unit's owner in phase 1 and by the coordinator between ticks, read by a marker in phases 2 and 3 |
| The turn | an asleep unit due is caught up from the tick it was left at; at its end, a unit not at rest that the condition admits is asleep from the next tick, its gate recorded scheduled |
| A mark | an asleep unit is marked as an idle one is, though its gate reads scheduled |
| A read (`units`, `sweep`) | after a tick, every asleep unit caught up to the next tick; one at rest leaves the map, its gate idle unless marked |
| A write (`units_mut`, `arenas_mut`) | every asleep unit caught up and put back on the schedule |
| `FLAG_SLOW` under the image's slow current | never sleeps |
| `Executor::turns`, `Executor::caught_up` | the turns served, as each worker's count; the steps caught up at a turn and at a read |
| `Executor::per_tick(self)` | per-tick service, the tests' reference: no unit sleeps |

A gap past `u32::MAX` ticks is caught up at `u32::MAX`, every unit being at rest by then.
No rule, record, constant or image byte changes, and the format stays 26.

### The arena between ticks: ADR-0023's invariant, restated

`Executor::units` writes asleep units' records under `&self`, once after each tick, in `settle`.
A `&mut` to a record never overlaps another reference to it there, because:
- no worker references the arena between ticks, every one parked at the barrier;
- no other thread holds `&self`: `unsettled` is a `Cell`, so the executor is not `Sync`, and a compile-time assertion beside `Executor` fails the build if it ever were;
- no reference the executor handed out is alive: only `tick` sets `unsettled`, `tick` takes `&mut self`, and the first read after it clears the flag before it makes a reference.

Its tests: the assertion, and `tests/arrival.rs`, which reads every arena through `units` after every tick and holds it to per-tick service's.

### Its tests

- **The condition, over the lattice** (`membrane.rs`, `prop`): 13 values of each potential beside nine of the threshold, the plateau with the burst bit and the window, 19 773 states.
  - `the_condition_admits_what_it_admits_of_the_lattice_and_a_quiet_tick_keeps_it_admitted`: 11 628 admitted, 7 234 of them with a threshold above zero, and 8 145 refused, pinned; every admitted state held 256 quiet ticks to the induction, no spike, the soma at or under its bound, still admitted, the bound not rising; every case of the derivation reached.
  - `the_burst_s_coupling_has_a_bound_of_its_own_and_fires_a_unit_the_resting_bound_admits`: no lattice state falls between the two bounds, and this one fires under the burst's coupling.
  - `a_driven_unit_whenever_the_condition_admits_it_cannot_fire_and_stays_admitted`: a walk of $2^{16}$ ticks in the reference network's working range.
- **The catch-up, against the stepped rule** (`prop`), field by field and in turns:
  - `quiet_catch_up_is_integrate_with_no_input_bit_for_bit_and_counts_the_turns_served`: every lattice state at 8 gaps up to 17, every 7th at 15 up to 256, every 61st at 24 up to 4 096, and past rest at `u32::MAX`;
  - `quiet_catch_up_over_a_driven_unit_s_states_is_integrate_bit_for_bit`, a gap of up to 4 096 every 13th tick of a walk;
  - `the_threshold_after_quiet_ticks_is_the_rule_s_by_segments`, every segment's edges.
- **Exploration 008's full strength, whole-domain**: `the_condition_s_units_run_to_rest_with_no_input_never_fire_exhaustive`, every admitted state to rest, 4 862 of the refused firing, and a walk of $2^{18}$ ticks; `quiet_catch_up_at_every_gap_from_every_state_is_integrate_exhaustive`.
- **The two services, held to each other** (`tests/arrival.rs`), at 256 units under ADR-0044's drive and under ADR-0097's drive 256 times sparser: every unit's 64 bytes and every block's, the mailbox head read as holding mail or not, the spikes and the messages delivered, after every tick for 2 048 ticks and every 4 999 over 32 768; at the end, the turns served and the steps caught up together are the reference's turns.
- **One unit beside its per-tick twin** (`tests/arrival.rs`): one message, served once and caught up 2 502 steps at a read, 2 503 together; a unit at rest asleep, read idle unmarked and scheduled marked; a write that puts it back on the schedule; the sweep, which evicts it as the twin's does; and a unit under the slow current, which never sleeps.
- **ADR-0097's tests** run on `Executor::per_tick`, their assertions unchanged; `tests/contention.rs` holds the workers' summed turns to `Executor::turns` on the engine as built.

**The mutation gate.**
- Before the pull request, 51 mutants in `membrane.rs`'s changed lines, 51 caught.
- On `e75d84a` the gate left one survivor of 115: `Shared::is_asleep` replaced with `true`. Marking a unit already scheduled and awake stores what it holds, so no test can see the mutant.
- `8aac5c8` reads the check inside `mark`, and no function returns it whole. On it the gate made 110 mutants: 108 caught, 2 unviable, no survivor.

### The dispatch

| | |
| :--- | :--- |
| Run | [40575462006](https://github.com/Menteion/Menteion/actions/runs/40575462006), `scope=exhaustive`, every test, on `8aac5c8` |
| `npm run round:additive -- origin/main 8aac5c8` | exit 1, *"does not only add, 19 reason(s): the calibration runs every test (ADR-0215)"* |
| The shards | 18 of 18 green; the 221 tests the listing names, each passed |
| The longest shard | 62 minutes, 25.8 % of its bound of 240; the shortest 34 |
| The pins | every pinned number held, every test's own |

The cost table is regenerated from its artifacts with `--keep`.
Its shards run on x86-64, two tests at a time on processors that differ, so the seconds are an indication only.

| Binary | Tests | Before | After | After / before |
| :--- | ---: | ---: | ---: | ---: |
| `inhibition` | 157 | 219 435 s | 89 480 s | 0.408 |
| `instrument` | 28 | 4 485 s | 2 234 s | 0.498 |
| `learning` | 11 | 1 846 s | 892 s | 0.483 |
| `assembly` | 4 | 1 585 s | 1 975 s | 1.246 |
| `reference` | 7 | 1 263 s | 439 s | 0.348 |
| `everywhere` | 1 | 676 s | 414 s | 0.612 |
| `active`, on the reference | 10 | 152 s | 164 s | 1.079 |
| **219 tests** | | **229 444 s** | **95 601 s** | **0.417** |

The two new whole-domain tests in `menteion-core` took 1 s and 2 s.
Five tests took more than a tenth longer:
- `assembly`'s two core grids under the gate raised to 0.9, 425 to 651 s and 528 to 785 s. They read the arena on every tick, and each read catches every asleep unit up a tick.
- `instrument`'s recalibrated rewarded run at 256 units on four workers, 225 to 308 s, not read further.
- two of `active`'s timed runs on the reference, 12 to 18 s and 24 to 31 s.

On the same shards the tests of `inhibition` read 0.26 to 0.55, so the five are the tests' own, not their runners'.

### The alternation on AArch64

[40575467414](https://github.com/Menteion/Menteion/actions/runs/40575467414), `scope=arms`, on `8aac5c8`: `on_arrival_and_per_tick_service_alternate_on_one_runner_timed` in one job of 2 714 s.
In one process, exploration 008's 16-block workload and the reference network at 1 024 units under ADR-0097's sparse drive ran under both services, per tick first in rounds 1 and 3 and second in round 2.
Every engine of a per-tick run, its derivation's among them, was served per tick through the harness's `served`.

| Seconds | Round 1 | Round 2 | Round 3 |
| :--- | ---: | ---: | ---: |
| 16 blocks, per tick | 337.3 | 330.4 | 335.7 |
| 16 blocks, on arrival | 256.7 | 244.7 | 247.9 |
| On arrival / per tick | 0.761 | 0.741 | 0.738 |
| Sparse, $2^{27}$ ticks, per tick | 186.8 | 177.5 | 156.3 |
| Sparse, on arrival | 151.6 | 148.8 | 140.3 |
| On arrival / per tick | 0.811 | 0.838 | 0.898 |

- **Every on-arrival run held to its round's per-tick run**: the arena's hash, the messages delivered and sent, the spikes, and its turns and steps together the per-tick run's turns. The 16-block runs also held H-42's derivation and their 16 blocks.
- **The counts**, the same in every round: the 16-block arm's engine served 167 800 187 turns and caught up 17 011 958 822 steps, per-tick service's 17 179 759 009, which exploration 008 read too. The sparse network served 4 194 304, one a message, and caught up 10 133 042 606, per-tick service's 10 137 236 910. No unit of it fired.
- **ADR-0224's reading is met.** On arrival read below per tick in every round of the 16-block workload. Under the sparse drive every on-arrival run read below the fastest per-tick run, 156.3 s, against a spread of 156.3 to 186.8 s.
- The reference carries the service's checks, and read 330.4 to 337.3 s, where exploration 008 read `main`'s engine at 308.8 to 311.2 s on another runner.

The figures are hosted runners' and exploratory, not Measured ([ADR-0010](0010-measured-or-target.md)).

### The sweep

The schedule's next run, on 2026-10-28 at 20:17 UTC, had not begun when this was written. The first ADR that merges after it ends records it.

## Consequences

- Good: the weekly job's tests took 0.417 of the table before on x86-64, its longest shard 62 minutes of 240.
- Bad: a read of the arena catches every asleep unit up first. A test that reads it on every tick pays a catch-up a tick for each, and `assembly`'s two core grids took longer.
- Bad: every change to the turn is tested on both services, and ADR-0097's and ADR-0101's tests read the reference, not the engine as built.
- Bad: `Executor` is not `Sync`; nothing in the tree shared one between threads.
- Neutral: a reward wakes every asleep unit, once a trial.

## Confirmation

- `npm run spec` runs the directives under §6.1's paragraph on the service, and brief 084 is archived.
- The pull request's gate on its last commit, the mutation gate on the changed lines among it.
