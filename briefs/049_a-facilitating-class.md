---
status: proposed
date: 2026-09-27
---

# Brief 049: A facilitating class — the class of short-term plasticity ADR-0113 decided, built beside `step_stp` and unset bit for bit; then ADR-0112's assemblies measured again with their members marked, over spans that tell a refreshed hold from a fading one

## Mission

**This brief builds one mechanism and measures with it.** ADR-0112 read no assembly usable
([ADR-0112](../docs/adr/0112-an-assembly-that-holds.md)). A kick set off one population burst, the burst drained the
members' vesicle pool, and the assembly then fell silent. The strong cells burst again of their own whether kicked or
not.

[ADR-0113](../docs/adr/0113-a-facilitating-class-of-synapses.md) chose a mechanism of persistence: **a facilitating
class of synapses**. A unit may be marked facilitating, and its synapses then release under short-term plasticity
whose facilitation outlasts its depression. The class's constants are a parameter of the image. By ADR-0113's
arithmetic, under either of its two sets a kicked assembly is primed for a while after its burst, up to 1.38 or 1.60
times the unkicked assembly's release. Under ADR-0019's constants it never is.

When the round is done, the tree holds:
- the class in the rule, the executor and the image, format 17, with its own ADR and its tests, and every pinned
  number of the tree standing when no unit is marked;
- a grid of assemblies on ADR-0112's substrate with their members marked, sizes, weights and the two sets, measured
  over spans by rules written before any run;
- an ADR that records the usable region, if one exists, and names the next decision **without taking it**.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0113's: the
  Tsodyks–Markram rule at other constants (Tsodyks and Markram 1997; Mongillo, Barak and Tsodyks 2008), selected per
  unit. **Nothing else is adopted**:
  - no slower current;
  - no time constant beyond $2^{16}$ ticks: `stp_decay_factor_q16` and ADR-0028's bound are untouched;
  - no change to the membrane, STDP, the modulator, the executor's schedule, the drive, the task or the critic;
  - no dependency, and no version bump of a tool.
- **Unset, bit for bit.**
  - `step_stp` is not touched. The class's step is written beside it, and at ADR-0019's constants it is `step_stp`
    bit for bit over the lattice of `testkit/prop.rs`.
  - With no unit marked, every pinned number of the tree holds and the determinism pin does not move.
  - A pin of a whole image moves with the format number and nothing else, restated under a masked check as
    [ADR-0095](../docs/adr/0095-an-image-pin-moves-with-its-format.md) restated H-17's.
- **Every weight frozen in the measurement.** The excitatory baseline is zero, the inhibitory baseline unset, the
  signed gate unset, and no reward is delivered. Every weight at the end of each run equals its value at the start.
- **The grid, the spans, the measures, their thresholds and the release are written before any run**, and none moves
  after one. The grid may be widened before the first run, not narrowed.
- **The arithmetic is computed first, with the class's own step.** ADR-0113's numbers come from a Python replica of
  `step_stp`; the engine's step replaces them. If the class's step disagrees with ADR-0113's steady table or its
  post-burst course, that is a finding and no cell runs until it is explained.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0113's included. Every oracle and
  every reading that steps a member's short-term state (ADR-0112's `stp_sums`, `steady`, `stp_course` among them)
  steps a marked member with the class's step.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are
  beside it. The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The runs are dealt so that no shard of the weekly job passes 60 per cent of its bound** under the regenerated deal
  ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md)). By ADR-0113's spans a cell is about fifteen times
  ADR-0112's ticks.
- No pinned number of an earlier round moves, but for the whole-image pins restated above.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-27 against `main` at `3e61e50`, after ADR-0112 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The rule** (`crates/cortex-core/src/dynamics/plasticity.rs`).
   - `step_stp(&mut self, elapsed_ticks)` relaxes `stp_u_rel` toward `STP_U` with `STP_TAU_F_SHIFT` and `stp_r_ves`
     toward `STP_MAX` with `STP_TAU_D_SHIFT`, facilitates $u$ by $U(1-u)$, returns the pair the release uses, and
     depletes $R$ by $uR$.
   - `stp_decay_factor_q16` *"reads a time constant above $2^{16}$ ticks as $2^{16}$"*.
   - ADR-0019 chose one set of constants: *"Per-type constants are a record question under ADR-0016's test."*
2. **The record** (`crates/cortex-core/src/dynamics/neuron.rs`, `membrane.rs`).
   - `flags` at `[57]` carries `FLAG_BURST_MODE` (0x01), which `integrate` sets and clears by mask, and
     `FLAG_INHIBITORY` (0x02).
   - `stp_r_ves` and `stp_u_rel` are at `[58]` and `[59]`.
3. **The executor** (`runtime/cortex-runtime/src/executor.rs`, the turn). After `integrate` returns a spike, the turn
   calls `u.step_stp(elapsed)` and pushes `(unit, release_u, release_r)` to the tick's spikes.
4. **The image** (`runtime/cortex-runtime/src/image.rs`, `crates/cortex-connectome`).
   - The neuron section holds the records' bytes, and the loader does not read `flags`.
   - The modulator section's record is laid out in ADR-0094: the signed gate at `[25]`, and `[26..28)` and `[32..64)`
     reserved and refused unless zero.
   - `CortexFileHeader::FORMAT_VERSION` is 16.
5. **The substrate** (`runtime/cortex-runtime/tests/assembly.rs`, ADR-0112): `grown`, `wire`, `PLACES`, the kick
   (`KICK_MESSAGE_Q16`, a ramp, and `KICK_RESET_Q16`), `release`, `protocol`, `background_of`, `cell`, `bursts` and
   `stp_sums`. ADR-0077's settled image comes from the shared harness (`tests/instrument/harness.rs`).
6. **ADR-0112's readings.**
   - A burst drains the pool to 9 to 27 of 255.
   - Between bursts $u$ is 0.6 to 0.7 and the pool about a fifth of its rest.
   - The strong cells re-burst every two to twelve windows of 2 048 ticks.
   - The release reaches the members but does not drain their pool.
7. **ADR-0113's arithmetic** (the replica):
   - steady pairs for sets (i) $(U, \tau_f, \tau_d) = (51, 2^{16}, 2^{13})$ and (ii) $(26, 2^{16}, 2^{13})$;
   - after a burst of four spikes 220 ticks apart, the primed window: about 7 400 to 106 000 ticks at up to 1.38 times
     the unkicked product for (i), and about 4 100 to 117 500 ticks at up to 1.60 for (ii);
   - faded below the unkicked product by 131 072 ticks under both.
8. **The weekly job** ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md),
   [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). A round that changes `src/` dispatches
   `scope=both`. The cost table is regenerated from its own dispatch.

## Deliverables

- [ ] **The class built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The mark*: a bit of `flags`, named, that `integrate` leaves as it finds it.
  - *The constants*: the class's $U$ (Q0.8, 1 to 255) and its two shifts (1 to 16), a parameter of the image. The
    loader refuses a value out of range, refuses a marked unit while the class is unset, and refuses the reserved bytes
    as today.
  - *The rule*: the class's step beside `step_stp`.
  - *The executor*: the class's step for a marked unit where the turn calls `step_stp`, with no word, no allocation
    and no syscall added to the tick.
  - *The image*: format 17.
  - *The tests*:
    - `cortex-core`: over the lattice, the class's step at ADR-0019's constants is `step_stp` bit for bit, and at any
      constants it is an `i64` oracle of Tsodyks–Markram; at the edges, shifts 1 and 16, $U$ 1 and 255, a first spike
      (`u32::MAX`), and $R$ at zero;
    - the executor: in one run, a marked unit steps under the class and an unmarked one under `step_stp`;
    - the image: the class written and read set and unset, each refusal, and a version-16 header refused;
    - `crates/cortex-connectome`: the version's assertions at 17;
    - the whole-image pins restated under a masked check.
- [ ] **The measurement's protocol, in an ADR, before any run.** At least:
  - *The grid*:
    - sizes 16, 32 and 64 at ADR-0112's placement, wiring and delays;
    - weights at least 0.25, 0.375, 0.5, 0.625, 0.75 and 1.0 of Q1.15's range;
    - ADR-0113's sets (i) and (ii);
    - every member marked.
  - *The spans*, in epochs of $2^{14}$ ticks, repeated for eight rounds after a lead-in:
    - an unkicked span, by default sixteen epochs;
    - a hold span with ADR-0112's kick at its start, by default sixteen epochs;
    - a release span;
    - a tail.
  - *The stretch*: two consecutive epochs.
  - *The release*, derived from the class's arithmetic before any run: the members kept from firing for as long as the
    priming takes to fade below the unkicked product, about $2^{17}$ ticks by ADR-0113's replica. It may be
    ADR-0076's cancel repeated, or a volley into nearby inhibitory units.
  - *The measures and their thresholds*, committed as rules:
    - the background: for each set, the members marked and unwired under the drive, with no kick, over the same ticks,
      and the rest of the network's rate beside it;
    - **a held stretch**: the members' spikes over it at least five times the background;
    - **holds**: in at least 7 of 8 rounds, every stretch of the hold span's second half is held. By ADR-0113's
      arithmetic, a kick's priming has faded there unless a burst refreshed it;
    - **ignites**: an unkicked span with any held stretch;
    - **lets go**: in at least 7 of 8 rounds, every stretch of the tail is at most twice the background;
    - **spills**: the rest of the network's rate over the held stretches more than twice its background;
    - a cell is **usable** when it holds, lets go, ignites in at most 1 of 8 rounds, and does not spill.
  - *Readings, no clause*: the intervals between bursts in the hold spans (ADR-0112's burst window); the members' sums
    of $(u, R)$ at each window's end; the product a member's next spike would release with, in held and in unkicked
    spans.
- [ ] **The arithmetic, before any run, with the class's step**:
  - both sets' steady pairs at ADR-0112's eight rates;
  - the post-burst course through $2^{18}$ ticks, held to ADR-0113's tables or recorded as a finding;
  - what one spike delivers to a target at each weight, at the unkicked product and at the primed peak, against the
    threshold;
  - the release's length.
- [ ] **The kick read on the engine with the members marked**, by ADR-0112's measure, before any cell. It fires every
  member once, or it is derived again before any cell.
- [ ] **The runs.** Every cell and each set's background and control. The weights shown unchanged at each run's end.
  Tables pinned per window, as ADR-0112's rows, and per span.
- [ ] **The ADR's reading.**
  - The grid table: holds, ignites, lets go, spills and usable, per cell.
  - The usable region, if one exists, and the arithmetic beside what the runs read.
  - If no cell is usable, what failed in each: never holding, running away, or not letting go.
  - **The next decision named and not taken**:
    - if a region is usable, ADR-0111's second round, a readout gated by the context, with F-54's geometry to solve;
    - otherwise a longer $\tau_f$ (the factor's bound) or the slower current, with this round's readings as its need.
- [ ] **The gate.** The build's tests, and at most one runtime test for the measurement: the rules at their edges over
  tables written by hand, and one marked assembly wired and kicked for a few hundred ticks.
- [ ] **The evidence.** A weekly dispatched on this round's branch at `scope=both`, since `src/` changes
  ([ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). It must be green in every job, with no survivor
  of the sweep on the class and every pinned number reproduced. The cost table is regenerated from that run's
  artifacts.
- [ ] **The documents, in the same pull request.**
  - Whitepaper: §5.2's record tables (the mark's bit and the image's bytes), the format's version row, §8.8's
    short-term plasticity row, §11.1's question on a rule held by the network with this round's reading, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes but the class.** `step_stp`, `stp_decay_factor_q16`, the membrane, STDP, the
  modulator, the executor's schedule, the drive, the task and the critic are untouched.
- No weight moves during a measurement run. The assemblies and the marks are the test's own wiring on the image.
- No context readout, no switch on errors, no reward. Those are ADR-0111's later rounds.
- The grid, the spans, the thresholds and the release do not move after a run, and the grid is not narrowed.
- No pinned number of an earlier round moves, but for the whole-image pins restated under a masked check.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- which bit marks a unit, where the class's constants sit in the image, and how the loader refuses;
- the class's step's name and signature, and how the executor selects it;
- the spans' lengths, the stretch and the number of rounds, within the measures' definitions;
- the release;
- whether the grid is widened: a weight between two of the six, a size of 128, or ADR-0113's option 2(iii) as a
  reading;
- where the tests live;
- whether the round writes one ADR or two (the build and the measurement).

It may not narrow the grid, move a threshold after a run, change a rule beyond the class, or reach the standing
directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. The class, its tests and the
measurement's protocol precede the first measurement run in the history.

## Report

The closing message states:
- the class as built: the bit, the image's bytes, the step, the refusals, and that unset nothing moved;
- the grid, the spans, the release and the thresholds, and when they were committed;
- the class's arithmetic beside ADR-0113's replica, and any disagreement;
- the kick read on the engine with the members marked;
- the grid table: holds, ignites, lets go, spills, usable;
- the usable region, or what failed in each cell;
- that no weight moved in a measurement run and no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why;
- the next decision named, not taken.
