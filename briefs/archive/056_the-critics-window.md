---
status: archived
date: 2026-09-30
---

> **Executed 2026-09-30 in pull request #168.** Writes ADR-0134 (the critic's window built) and ADR-0135 (the critic's
> window, measured), and opens and resolves F-62 (§8.7's layout of the modulator section stopped at the slow current's bytes
> after ADR-0131 placed the critic's). The window — a length in ticks at the modulator section's `[52..54)`, a spike
> counted only when its tick is fewer than the length after the previous reward the critic took, or after the engine's
> start before the first, the length read from the image by `shortest_delay` and held to it by the loader, format 20 — is
> unset bit for bit: with it unset every one of the weekly job's 75 whole-domain tests passed before any rewarded run, and
> the five whole-image pins moved with the format and nothing else. Its length, 100 ticks, ADR-0132's arithmetic restated
> for the window's features with H-22's shift 9 and scale 2 kept, H-23's constants and clauses, the gate and the two arms
> were committed and pushed before any run, with the pull request opened as a draft.
>
> **H-23 is yes, and its stopping rule reached step 3.** Every mapping was learned in both arms, 115 to 124 of each
> mapping's last 128, no coupling past 1.30; each reversal passed 40 of 64 in 14 to 20 blocks, within the task critic's 23
> and against H-22's 10 to 30, as ADR-0133 predicted; each stimulus's mean value stood within 0.115 of the reward of
> $2p - 1$. The window admitted the presented stimulus's volley and about 1.6 background spikes a trial, the value came to
> rest on each stimulus's own units, and after each flip it came back as the task critic's did. **The next decision** — an
> ADR choosing among the operating regime, another size, the rule held by the network reopened on this configuration and a
> critic carried by a population — is named and not taken.
>
> The weekly dispatched at `scope=both` (run 36664142217) ran every exhaustive shard green: all 77 whole-domain tests
> passed, the 75 before this round reproducing their pinned numbers and H-23's two arms their tables, in 1 362 and 1 845 s;
> the shards' tests ran at 20 to 44 per cent of the bound. The mutation sweep was green in its seven jobs: 3 595 mutants
> caught, none missed, every one in the window's code caught, 23 of its 24 timeouts the known protocol ones and the other
> in a loader check the round did not change. The cost table is regenerated from the run (77 lines, 29 058 s) and plans each
> of the six shards at about 34 per cent, inside the brief's 60. The pull request's gate is green in every job, the mutation
> gate on the changed lines catching every mutant it could make (31: 27 caught, 4 unviable).
>
> The body below describes the tree before execution and is not maintained. Relative links gained one `../` when the brief
> was archived; no word, claim or figure changed.

# Brief 056: The critic's window — ADR-0133's window built, the engine's critic counting a unit's spikes only within the shortest synaptic delay after each reward, unset bit for bit; then H-23 run once on H-22's configuration with the window set

## Mission

**This brief builds one parameter and runs one hypothesis once.** H-22 ([ADR-0132](../../docs/adr/0132-a-critic-of-the-engines-own-measured.md))
learned every mapping with the engine's own critic. Two of its three reversals, though, were 7 to 11 blocks slower than
H-21's with the task's critic, and the value fell deeper after those flips, with about half of it on units both
stimuli share.

[ADR-0133](../../docs/adr/0133-the-critics-window.md) gives the critic a window:
- a unit's spike counts only within $W$ ticks after the previous reward;
- $W$ is the shortest delay of any synapse the image carries, read by a rule written first;
- unset, the window is ADR-0131's critic bit for bit.

ADR-0133 wrote **H-23** before any run, with H-22's clause 3 replaced by a reversal clause and a prediction clause in
F-61's form.

When the round is done, the tree holds:
- the window in the executor and the image, format 20, with its own ADR and tests, and every pinned number standing
  when it is unset;
- H-23's two arms run once on H-22's configuration with the window set, as weekly `exhaustive` tests;
- H-23's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The one mechanism is ADR-0133's: an integer
  comparison of a spike's tick with the previous reward's, gating ADR-0131's count. **Nothing else is adopted**:
  - no kernel over time, no second window and no change to the critic's value, error or step;
  - no change to the modulator's rule, STDP, the membrane, the task's selection or the inhibitory rule;
  - no dependency, and no version bump of a tool.
- **The engine's own inputs only.** The window opens at the engine's own reward. Its length is a rule of the image's own
  synapses and is never read from the task's timing, sets or geometry.
- **Unset, bit for bit.**
  - With the window unset the critic is ADR-0131's, and H-22's arms reproduce their pinned tables.
  - With the critic unset nothing of the engine changes. Every pinned number of the tree holds, and the determinism pin
    does not move.
  - A pin of a whole image moves with the format number and nothing else, restated under a masked check
    ([ADR-0095](../../docs/adr/0095-an-image-pin-moves-with-its-format.md)).
- **H-22's shift 9 and scale 2 are kept.** ADR-0132's arithmetic is restated for the window's features before any run
  and held by the gate. If it shows the constants no longer resolve the rule, the round stops before any rewarded run.
- **H-23's clauses, constants and the window's rule and length are written before the first rewarded run** and do not
  move after it.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0133's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. For the measurement, the runtime's gate grows by
  at most one test ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)); the build's own tests are beside
  it. The mutation gate on the changed lines must pass ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- No pinned number of an earlier round moves, but for the whole-image pins restated above.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-30 against `main` after ADR-0132 and ADR-0133 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The critic** ([ADR-0131](../../docs/adr/0131-the-critic-built.md)):
   - `runtime/cortex-runtime/src/executor.rs`: `Config::critic`, `merge_spikes`, which counts each merged spike against
     its unit before it pushes it into the ring, `Executor::{reward, features, prediction}` and the sweep's rule for a
     unit with a count pending;
   - `crates/cortex-neuromod/src/lib.rs`: `ValueCritic`;
   - `runtime/cortex-runtime/src/image.rs`: the modulator section's `[48]` flag, `[49]` shift and `[50]` scale, and
     `MODULATOR_RESERVED`, `[26..28)`, `[39]` and `[51..64)`. The format is 19.
2. **The trial** (`runtime/cortex-runtime/src/task.rs`, the module's documentation): the reward is delivered before the
   next trial's first tick; the next stimulus is injected before that same tick; the set fires together about twelve
   ticks on, each of its 51 units once under ADR-0076's shape and cancel.
3. **The reference prior** (`runtime/cortex-runtime/tests/instrument/harness.rs`): the local delay band is 100 to 300
   ticks and the far band starts at 1 400. `WINDOW` opens 100 ticks after the injection, *"before which no local synapse
   of the volley can have landed"*.
4. **H-22** ([ADR-0132](../../docs/adr/0132-a-critic-of-the-engines-own-measured.md), `tests/inhibition.rs`):
   - `VALUED_CRITIC`, `valued_image`, `valued_arm` and `earned_run_valued` on the shared harness;
   - its pinned tables, among them `CROSSINGS_VALUED_1024`, `STRONG_BY_FLIP_VALUED_1024` and `VALUED_WEIGHTS_1024`;
   - its readings: the value per stimulus at about $2p - 1$, the troughs after flips 1 and 3 at −0.92 to −1.03, the
     reversals at 22 to 30 blocks, and half the value on shared units.
5. **H-21 and H-20**: their reversal speeds (`CROSSINGS_TARGET_1024`, `CROSSINGS_1024`), 13 to 23 blocks; the slowest
   is 23.
6. **F-61** (whitepaper §11): for a critic of a stimulus's expected reward, H-22's clause 3 is an accuracy bound of
   three quarters.
7. **The weekly job**: six shards. A round that changes `src/` dispatches `scope=both`
   ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch; after ADR-0132 it plans about 33 per cent of the bound.

## Deliverables

- [x] **The window built, in a new ADR at the next free number (`ls docs/adr`).**
  - *The parameter*: $W$ in the modulator section's reserved bytes, zero meaning unset; format 20; whitepaper §5.2's
    table and the version row.
  - *The rule*: with the window set, a spike counts only when its tick is within $W$ ticks after the previous reward
    (after the engine's start before the first reward). The critic's value, error and step are unchanged.
  - *The loader*: refuses a window the rule does not resolve, and a window set without the critic.
  - *The tests*:
    - with the window unset, the counts and every reward are ADR-0131's, bit for bit;
    - with it set, the counts are exactly the train's spikes within $W$ ticks after the previous reward, at the
      window's edges on both sides;
    - the image: the window written and read set and unset, each refusal, and a version-19 header refused;
    - `crates/cortex-connectome`: the version's assertions at 20;
    - the whole-image pins restated under a masked check.
- [x] **H-23's protocol, in an ADR, before the first rewarded run.**
  - The window's rule, *"the shortest delay of any synapse the image carries"*, as a function over the image, pinned
    with the length it reads from H-21's image.
  - ADR-0132's arithmetic restated for the window's features: the volley's step, the background spikes the window
    admits, and the dead band, at shift 9 and scale 2.
  - H-23's clauses and constants, restated from ADR-0133 and pinned in the tests:
    - clause 3's bound of 23 blocks;
    - clause 4's quarter of the reward about $2p - 1$, per stimulus and mapping over each mapping's last 128 trials.
  - The readings, no clause:
    - the spikes per trial the window admits, by group;
    - the value per stimulus and block beside $2p - 1$, H-21's task critic and H-22's engine critic;
    - the trough after each flip and the strong punishments per flip beside H-21's and H-22's;
    - the weights by group at every block's end;
    - the reversal speeds beside H-20's, H-21's and H-22's;
    - the inhibitory sum's course.
- [x] **The calibration**, before any rewarded run: every pinned number holds and H-22's arms reproduce, with the
  window unset.
- [x] **The runs**: H-23's two arms, each a weekly `exhaustive` test, their tables pinned per block as H-22's.
- [x] **The ADR's reading.**
  - H-23's verdict per clause and arm.
  - The account's prediction (clause 3 holds) against the reading.
  - The readings.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** The build's tests, and at most one runtime test for the measurement: the window's rule over images
  written by hand, the arithmetic, and H-23's clauses at their edges.
- [x] **The evidence.** A weekly dispatched on this round's branch at `scope=both`. It must be green in every job, with
  no survivor of the sweep on the window and every pinned number reproduced. The cost table is regenerated from that
  run's artifacts.
- [x] **The documents, in the same pull request.**
  - Whitepaper §5.2's image bytes and the format's version row, §8.7's format number, §8.8's modulator row, §11.1's
    H-23 with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes but the window.** The critic's value, error and step, the modulator's rule, STDP,
  the membrane, the selection, the inhibitory rule and the task's reward are untouched.
- The window's length is never read from the task's timing, sets or geometry.
- H-22's shift and scale are not moved.
- H-23's clauses, constants and the window's length do not move after a rewarded run. There is no second attempt.
- No pinned number of an earlier round moves, but for the whole-image pins restated under a masked check.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, and no `unsafe` beyond ADR-0023's invariant.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the parameter's width and bytes, and how the loader refuses;
- how the executor keeps the previous reward's tick, and how the window gates the count;
- how the rule reads the shortest delay from the image, within *"the shortest delay of any synapse the image carries"*;
- where the tests live, and how the two arms are dealt into tests;
- whether the round writes one ADR or two (the build and the measurement).

It may not:
- read the window from the task;
- move H-22's constants;
- move a clause or a constant after a rewarded run;
- change a rule of the engine beyond the window;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

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

Every command exits 0. The dispatched run is green and reproduces every pinned number. In the history, the window, its
tests, its rule and length, the arithmetic and H-23's constants all precede the first rewarded run.

## Report

The closing message states:
- the window as built: the parameter, the image's bytes, the rule, the refusals, and that unset nothing moved;
- the window's length as its rule read it, and the arithmetic restated, and when they were committed;
- the calibration;
- H-23's verdict per clause and arm, the account's prediction against the reading, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
