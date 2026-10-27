---
status: exploratory
date: 2026-10-27
---

# 008: A unit served on arrival

**Branch**: `explore/008-a-unit-served-on-arrival`
**Commit**: `e547c45` for build B, the scripts and the reader; build A is `b455b14`, this branch's timed workload on
`main`'s engine. B adds the change under `src/`, `tests/arrival.rs`, the `COUNT` lines of its counts and the scripts.
**Runs**: [40512122156](https://github.com/Menteion/Menteion/actions/runs/40512122156) (one job on x86-64 and one on
AArch64, at `scope=arms`)

[ADR-0097](../adr/0097-the-active-set-measured.md) counted the executor's turns: one message of 0.125 at the gain 1.75
keeps a unit awake 2 503 ticks, 1 023 of them a one-LSB tail, so under ADR-0044's drive the executor serves 99.99 % of
the units every tick. [ADR-0098](../adr/0098-the-integration-model.md) kept per-tick service and wrote that service on
arrival with a catch-up for the membrane as written (its option 3) had no known candidate.
[Exploration 006](006-where-an-arm-spends-its-time.md) found the units' turns at 84 % of a heavy arm's test thread on
x86-64 and 65 % on AArch64, `integrate` alone at 45 % and 38 %. This note asks:

1. a condition under which a unit not at rest provably cannot fire before its next message;
2. whether a unit left off the schedule under it, and brought up to date by `integrate(0, 0, t)` once a skipped tick
   when a message arrives or its state is read, is the per-tick service bit for bit, every pin of an arm included;
3. a faster exact catch-up, if one is found;
4. how much faster the arm runs either way.

It writes no prediction and decides nothing. **Every figure here is exploratory**: the times from one alternation on
each architecture on GitHub's hosted runners, four CPUs each, the x86-64 one an EPYC 7763 of two cores of two threads,
the AArch64 one a Neoverse-N2 of four cores; nothing here is Measured ([ADR-0010](../adr/0010-measured-or-target.md)).

**In brief**
- **A condition was found, and held.** A unit cannot fire with no input while its soma, and a weighted mean of its two
  compartments' positive parts, stay under the lower of its threshold and the base. On the lattice, none of the 11 628
  states it admits fired run to rest, and the soma stayed under the bound; of the 8 145 it refuses, 4 862 do fire.
- **Every pin held bit for bit, on both architectures, under both catch-ups.** The whole arm's derivation, its 120
  blocks, its trace, every reward's value and H-45's control; every unit's and block's bytes at each of the 120 blocks'
  ends, the mailbox head read as holding mail or not; and the turns served and caught up together are build A's turns
  exactly, 128 845 414 112. On the developer machine 819 of the workspace's 820 tests passed under each catch-up, the
  one left being ADR-0097's test of the turn count itself.
- **The executor served 1 % of the units a tick, and did the same number of steps.** About 10 of the 1 024 a tick, where
  A served 1 023.97. A woken unit was caught up 117.8 ticks on average, 116.0 a message: 98.3 % of A's turns became
  catch-up steps at a turn and 0.75 % at a read. Under ADR-0044's drive a message comes long before rest, so the
  catch-up's steps are the per-tick turns, moved, not removed.
- **The faster catch-up is the potentials stepped alone.** The threshold, the refractory window and the plateau have
  closed forms over a gap, the threshold by segments of a constant step. The soma has none that was found: it floors two
  differences a tick.
- **The whole arm took 0.581 of A's time on x86-64 and 0.771 on AArch64 under the faster catch-up**, and 0.907 and 0.910
  under `integrate` itself, each against A on the same runner. The 16-block alternation, three rounds, read 0.583 and
  0.773, and 0.906 and 0.917.

## 1. When a unit may sleep

**The condition** (`DendriticSuperNeuron::quiet_cannot_fire`, on the branch in `membrane.rs`). A unit not at rest may
leave the schedule when, given no input from now on, it never fires again:

- its threshold is at or below zero, which never fires and which a quiet tick does not move; or
- $M < \theta_\min$, where $\theta_\min = \min(\theta, \theta_0)$, $\theta_0$ the base, and
  $M = \max(s, 0, \lceil (B + A)/2 \rceil)$, with $\lceil (B + 4A)/5 \rceil$ beside it while the burst bit is set;
  $B$ and $A$ are the basal and apical potentials' parts above zero.

**Why it holds.** With no input:
- the threshold never falls below $\theta_\min$: above the base it moves down by $\max(e \gg 12, 1)$ of its excess
  $e$, which is at most $e$, and at or below the base a quiet tick leaves it;
- the compartments never rise above $B$ and $A$: a leak moves a potential toward zero and never past it;
- the soma never rises above $M$. With $s \le M$ and $d = M - s$, the two couplings add at most
  $\lfloor d/8 \rfloor$ under the resting shift ($2M \ge B + A$) and at most $\lfloor 5d/16 \rfloor$ under the burst's
  ($5M \ge B + 4A$). The soma's leak takes $s \ge 0$ no higher and $s < 0$ up by $\max(|s| \gg 11, 1)$, where
  $d \ge |s| \ge 1$. In either case $s' \le M$, and the clamp to `i32` keeps it there. The burst's shift holds while the
  bit is set and the resting one after the plateau clears it, so both bounds are taken while it is set.

So the soma never reaches the threshold. The refractory window needs no term of its own, since it only forbids a spike
the bound already forbids. Nor does the plateau's length, which decides only which coupling holds. A unit marked
`FLAG_SLOW` under an image's slow current is never put to sleep.

**The property tests** (`testkit/prop.rs`'s generator and lattice):
- **The lattice.** 13 values of each potential (the lattice without its ends' neighbours) beside nine combinations of
  threshold, plateau, burst bit and refractory window: 19 773 states. The condition admits 11 628 of them, 7 234 with a
  threshold above zero. Each admitted state was run with `integrate(0, 0, t)` until its potentials were all zero, within
  $2^{16}$ ticks: **none fired, and the soma never rose above $M$.** Every case the derivation names was reached by an
  admitted state: a running window, the burst's coupling with apical depolarisation, a threshold above its base, a soma
  below zero with a compartment above it, a soma under its basal compartment, a threshold at or below zero and one
  between zero and the base. Of the 8 145 states the condition refuses, 4 862 fire with no input and 3 283 do not.
- **A driven walk.** One unit for $2^{18}$ ticks under messages of the reference network's working range: a basal
  0.21875 about a tick in 85, now and then an apical or an inhibitory one, and an edge-biased input a tick in 4 096. It
  fired 1 244 times. Every 37th tick its state was taken, 7 085 states, and the 3 980 the condition admitted were run to
  rest with no input: none fired.

## 2. The catch-up, and every pin

**The executor** (on the branch in `executor.rs`), under a mode read from `MENTEION_EXPLORE_008` when an executor is
built, off by default:
- **At the end of a turn**, a unit not at rest that the condition admits leaves the schedule, asleep. A bit is set in a
  map laid out as the schedule is, the next tick is kept as the first it skipped, and its gate byte is recorded
  scheduled, as the per-tick service records it.
- **A message or an activation** marks an asleep unit on the schedule as it marks an idle one.
- **At the start of its next turn**, the unit is brought from the tick it fell asleep to this one. Under `integrate`
  that is `integrate(0, 0, t)` once a skipped tick, stopping where the unit is at rest, as the per-tick service stops
  serving it; under `quiet` it is `quiet_catch_up` (§3). Then the turn is the turn it was. Under `integrate` a spike in
  a catch-up would have stopped the process; none came.
- **Before the arena is read between ticks** (`Executor::units`, and through it the image and the harness's dumps and
  checks), every asleep unit is brought up to date and sleeps on from the next tick. One that reached rest asleep wakes,
  its gate recorded idle unless a message has marked it. **Before the arena is written** (`units_mut`, `arenas_mut`),
  every asleep unit is also put back on the schedule, since a write may take it out of the condition.

A quiet tick of the per-tick service is `integrate(0, 0, t)` on a unit with an empty mailbox, so under `integrate` the
catch-up is the same function on the same state. Each step is a turn the per-tick service would have served, and is
counted as one.

**The mailbox head is not compared.** It is a node index from a worker's pool, whose order the two workers' frees in
one turns phase set, whichever lands first: [ADR-0099](../adr/0099-the-engines-speed.md)'s scheduler bytes. Two runs
of `main`'s own engine on two workers need not share it. Every comparison here reads it as whether the mailbox holds a
message, and every other byte of every unit and every block as it is.

**On the developer machine** (correctness, not time):
- **The workspace**, `cargo test --workspace --locked`: 820 passed with the mode off. Under each mode 819 passed and
  one failed: ADR-0097's gate test of the turn count, which holds each tick's turns to the units whose gate is
  scheduled. An asleep unit's gate is scheduled and it takes no turn: that count is what this exploration changes, not
  a reading of behaviour. The determinism pin and every other pin of the gate's tests held under both modes.
- **`tests/arrival.rs`**: the reference network at 256 units under each mode beside the per-tick service, two networks
  built alike. Every unit's and block's bytes, the spike train and the messages delivered were held to each other after
  every tick for 2 048 ticks, and every 4 999 ticks for 32 768 under ADR-0044's drive and under ADR-0097's drive 256
  times sparser. All held. In each run the turns served and caught up were the per-tick service's exactly: 8 353 378
  under ADR-0044's drive and 2 133 157 under the sparser one. One armed unit given one message of 0.125 at the gain 1.75
  was served once and caught up 2 502 ticks, ADR-0097's 2 503 together.

**On the runners**, every one of the 24 runs passed, each held as exploration 006 held its workload:
- **the whole arm** (`the_control_on_time_on_seed_22_from_the_assignment_timed_explore`) in A, B under `integrate` and
  B under `quiet` on each architecture: its derivation held to H-42's pinned derivation, and its 120 blocks, its trace
  and every reward's value to H-42's arm, `UntoldControl { moved: None, h42: true }`;
- **the 16-block workload** three times in each build on each architecture: its derivation whole and its 16 blocks to
  the first 16 pinned;
- **every `DUMP` line of every B run is A's**, and **every `ARENA` line**, the hash of every unit and block at each
  block's end, is A's: 120 of 120 in the whole arm and 16 of 16 in each 16-block run;
- **the turns**: A served 128 845 414 112 turns in the whole arm's 125 829 620 ticks, 1 023.97 a tick. B served
  1 256 967 871 and caught up 126 624 358 804 steps at a turn and 964 087 437 at a read: A's, to the turn. The
  16-block runs read the same, 17 179 759 009 in each build.

## 3. A faster exact catch-up

**`quiet_catch_up`** (on the branch in `membrane.rs`) gives the fields $\Delta$ quiet ticks of `integrate` would leave,
and the turns they stand for:
- **the threshold** by segments of a constant step. With $2^{13}$ or more of excess the step $q = e \gg 12$ holds for
  $\lfloor (e - 2^{12} q)/q \rfloor + 1$ ticks, one division a segment; below it the step is one LSB and the excess
  falls by the ticks;
- **the refractory window and the plateau** count down by $\Delta$, the burst bit cleared where the plateau reaches
  zero, as `integrate` clears it;
- **the three potentials** stepped as `integrate` steps them with no input: the two leaks and the soma's coupling, the
  burst's coupling for the plateau's ticks. The window, the plateau and the threshold are not stepped, nor the apical
  compartment while it is zero, and the loop stops once all three are zero.

**No closed form of the soma was found.** Each compartment's leak alone is a map of one variable, which jumps by
segments of $v \gg k$ as the threshold does. The soma floors $(b - s)/16$ and $(a - s)/2^k$ every tick, and no segment of
either compartment fixes those floors. A jump straight to rest, for a gap longer than the ticks to rest, was not built:
it would need a proven bound on the soma's last ticks. It would do nothing for this arm either, where a woken unit had
slept 117.8 ticks on average and a message keeps one awake 2 503.

**The property tests**: from each of the 19 773 lattice states, admitted or not, until it would fire, `quiet_catch_up`
of $\Delta$ was held to $\Delta$ calls of `integrate(0, 0, t)`, field by field and the turns with them, at 23 values of
$\Delta$ from 0 to 4 096, around the plateau's and windows' lengths, the leaks' shifts and tails, and 2 503. It was held
at `u32::MAX` too for the states at rest by 4 096. From a driven walk's states, every 13th tick of $2^{17}$, it was held
at a random gap of up to 4 096, and the threshold alone at every segment's edges and the lattice's values for up to
200 000 ticks. All held.

## 4. The time

Run as exploration 006 ran its profiles: both builds' `inhibition` binaries in the release profile in target
directories of their own, then the 16-block workload in three rounds of A, B under `integrate` (B1) and B under
`quiet` (B2), each run in a process of its own on one runner, then the whole arm once in each. The whole arm is
estimated from the 16-block runs as exploration 006 estimated it: the derivation and 120 blocks at the mean of blocks 2
to 16.

| x86-64, EPYC 7763 | The runs, rounds 1 to 3 | Derivation | A block | The whole arm (est.) | Beside A |
| :--- | :--- | ---: | ---: | ---: | ---: |
| A | 313.6, 312.8, 312.7 s | 197.0 s | 7.25 s | 1 067.2 s | |
| B1, `integrate` | 279.8, 282.7, 286.5 s | 177.8 s | 6.57 s | 966.5 s | 0.906 |
| B2, `quiet` | 181.1, 180.6, 181.0 s | 113.0 s | 4.24 s | 621.7 s | 0.583 |

| AArch64, Neoverse-N2 | The runs, rounds 1 to 3 | Derivation | A block | The whole arm (est.) | Beside A |
| :--- | :--- | ---: | ---: | ---: | ---: |
| A | 308.8, 311.2, 310.6 s | 193.2 s | 7.31 s | 1 070.5 s | |
| B1, `integrate` | 283.1, 282.8, 279.8 s | 174.2 s | 6.73 s | 981.5 s | 0.917 |
| B2, `quiet` | 241.5, 239.4, 245.9 s | 152.2 s | 5.63 s | 827.4 s | 0.773 |

| The whole arm, once | x86-64 | Beside A | Derivation, blocks | AArch64 | Beside A | Derivation, blocks |
| :--- | ---: | ---: | :--- | ---: | ---: | :--- |
| A | 1 073.4 s | | 197.7, 875.6 s | 1 074.6 s | | 191.9, 882.7 s |
| B1, `integrate` | 973.7 s | 0.907 | 176.9, 796.8 s | 977.8 s | 0.910 | 171.4, 806.5 s |
| B2, `quiet` | 623.5 s | 0.581 | 113.4, 510.1 s | 828.1 s | 0.771 | 149.0, 679.1 s |

- **Every B run was faster than the A run of its round**, on both architectures. B2's three 16-block runs spread by
  0.3 % on x86-64 and 2.7 % on AArch64, A's by 0.3 % and 0.8 %.
- **The derivation gained as the blocks did**: 0.574 and 0.585 under B2 on x86-64, 0.788 and 0.770 on AArch64. Its
  engines served units on arrival too: in the derivation's 28 320 107 ticks they served 270 176 805 turns and caught up
  28 666 855 361 steps at a turn and 56 537 460 at a read. A prints no turns of its derivation's engines, so these are
  not held to A's.
- **Per turn of the per-tick service**, over the whole arm's blocks and both workers: A took 13.6 ns on x86-64 and 13.7
  on AArch64, B1 12.4 and 12.5, and B2 7.9 and 10.5. That is the blocks' seconds twice over A's turns, barriers and
  phase 3 included, not a cost of `integrate` alone.

**What the executor counted** in the whole arm, the same under both catch-ups; the 16-block runs counted alike in every
round and mode:

| | The whole arm |
| :--- | ---: |
| Ticks | 125 829 620 |
| Turns served, a tick | 9.99 (5 to 1 024) |
| Ticks serving under 32 units, 32 to 63, 64 to 95 | 124 208 194; 911 388; 702 358 |
| Ticks serving 992 or more | 7 680, one a trial |
| Units put to sleep, and woken | 1 082 863 446; 1 075 004 278 |
| The messages the woken units drained | 1 091 595 819 |
| Catch-up steps at a turn: a woken unit, a message | 117.79; 116.00 |
| Woken units by steps: 0, 1, 2–3, 4–7, … | 12.2 M, 9.3 M, 18.3 M, 35.6 M, 67.3 M, 120.4 M, 193.8 M, 256.7 M, 235.6 M, 110.2 M, 15.3 M, 238 875, 66 |
| Reads that brought asleep units up to date, units, steps | 7 682; 7 861 205; 964 087 437 |

The 7 680 ticks that served nearly every unit are one a trial, as the rewards are. The critic writes its weights
through `units_mut`, which puts every asleep unit back on the schedule, though `value_weight` is a field no quiet tick
reads or writes; the tick after a reward then serves them all. That reading is from the count, not traced tick by tick.

## What it suggests

- **ADR-0098's option 3 has a candidate on this workload.** The units left the schedule under a proven condition and
  came back bit for bit, every pin of a heavy arm held on both architectures. That is a suggestion for a decision ADR and
  a measured round, not a verdict: no rule changed, and per-tick service stays the model.
- **What it bought here is cheaper steps, not fewer.** Under ADR-0044's drive a unit is woken about every 118 ticks,
  long before the 2 503 a message keeps it awake, so the catch-up runs the per-tick service's turns as steps. Run by
  `integrate` itself they cost about 9 % less than turns: the mail check, the batch, the gate and the schedule's bits go.
  Run by the potentials alone they cost 42 % less on x86-64 and 23 % less on AArch64. Fewer steps would need what §3 did
  not find: a closed form of the soma, or a proven jump to rest, which would pay only under a sparse drive, where a gap
  outlasts the 2 503 ticks.
- **The two architectures differ by more than any profile of exploration 006 did.** B2 took 0.581 of A on x86-64 and
  0.771 on AArch64, where A took about the same on both runners. A question for a later exploration: where B2 spends its
  time on AArch64, read as exploration 006 read A, with the barrier's wait and the atomics' out-of-line helpers beside
  the catch-up loop.
- **The count ADR-0097 reads would change meaning.** Under service on arrival `Executor::turns` counts the units
  served, about 1 % of the per-tick count here, and only the turns, the catch-up's steps and the reads' steps together
  are the active set ADR-0097 measured. A round that took this would restate ADR-0097's gate test, and every budget
  written from `turns`.
- **What a round would have to settle.** Units come up to date through `Executor::units` under `&self`, by a write
  through the arena that holds only because nothing else references the arena between ticks; a round would restate
  ADR-0023's invariant for it, or settle the arena under `&mut self`. And a reward's write of the critic's weights
  need not wake every asleep unit.
