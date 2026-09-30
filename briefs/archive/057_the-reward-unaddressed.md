---
status: archived
date: 2026-09-30
---

> **Executed 2026-09-30 in pull request #171.** Writes ADR-0137 (the reward unaddressed, measured); opens and closes no
> finding, and changes no file under `src/`. The harness's run under a delivery — the addressed delivery's run bit for bit —
> a network's oracle replaying every excitatory synapse of the arena, 26 240 at 1 024 units, from the train and holding
> it to the record at every trial, H-24's clauses and constants, the gate and the two arms were committed and pushed before
> any run, with the pull request opened as a draft. Before the rewarded run every one of the weekly job's 77 whole-domain
> tests passed from the release build, H-23's arms among them, and in each arm H-23's first block reproduced under the
> addressed delivery, table by table.
>
> **H-24 is no, on clause 1, and its stopping rule reached step 5.** Each mapping's last 128 trials read 79, 78, 64 and
> 87 correct from the assignment and 89, 67, 64 and 83 from the mirrored, against the 80 asked; every coupling stayed
> bounded, the highest 1.173 of its image's; the excitatory sum outside the four couplings stayed within 0.987 and 1.043 of
> the image's, though every synapse outside them moved. The reward's net in the answer's pairs was 0.8 to 2.9 per cent of
> the weight it moved there, the other pairs moved as much, and from the second mapping both stimuli selected readout 1 more
> often than readout 0; the critic held each stimulus's expected reward. **The next decision** — an ADR on the
> eligibility's specificity under a global reward, with the readings of where the consolidation went as its need — is
> named and not taken.
>
> The weekly dispatched at `scope=exhaustive` (run 36700851135) ran every exhaustive shard green: all 79 whole-domain tests
> passed, the 77 before this round reproducing their pinned numbers and H-24's two arms their tables, in 1 809 and 1 907 s;
> the shards' tests ran at 34 to 49 per cent of the bound. No source changed, so the mutation sweep did not run. The cost
> table is regenerated from the run (79 lines, 34 700 s) and plans each of the six shards at about 41 per cent, inside the
> brief's 60. The pull request's gate is green in every job.
>
> The body below describes the tree before execution and is not maintained. Its relative links gained one `../` when it
> moved to `archive/`; no word, claim or figure changed.

# Brief 057: The reward unaddressed — H-24 run once on H-23's configuration with the dopamine term delivered to every synapse, nothing of the engine changed

## Mission

**This brief runs one hypothesis once and changes nothing of the engine.** Since H-14 every learning run has addressed
the reward: the task names the synapses from the stimulus it presented onto the readout the engine selected, and only
those consolidate. H-23 ([ADR-0135](../../docs/adr/0135-the-critics-window-measured.md)) removed the host's label from
the critic. The address is the one place the learning loop still takes it.

[ADR-0136](../../docs/adr/0136-the-reward-unaddressed.md) delivers the reward to every synapse again
(`Delivery::Global`), on H-23's configuration with the engine's critic and its window. It wrote **H-24** before any
run: every mapping learned, every coupling bounded, and the rest of the network held, in both arms.

When the round is done, the tree holds:
- H-24's two arms run once on H-23's image with the global delivery, as weekly `exhaustive` tests, their tables pinned;
- H-24's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round measures a delivery the tree already has.
  **Nothing is adopted**:
  - no rule, no parameter and no field of the engine changes;
  - no change to the modulator, STDP, the membrane, the critic, its window, the selection or the inhibitory rule;
  - no dependency, and no version bump of a tool.
- **One change from H-23**: the task's delivery, `Delivery::Global` in place of `Delivery::Addressed`. The image each
  arm decodes is H-23's bit for bit, since the delivery is the task's and not the image's.
- **H-24's clauses and constants are written before the first rewarded run** and do not move after it. There is no
  second attempt.
- **The engine is read before a description of it is trusted**, this brief's and ADR-0136's included.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal.
- **No pinned number of an earlier round moves.**
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-30 against `main` after ADR-0135 and ADR-0136 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The delivery** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`): between a trial's last tick and its reward,
   `Delivery::Global` calls `exec.address_all()`, and `Delivery::Addressed` calls `exec.address(sources, targets)`.
   The sources are the presented stimulus's set and the targets are the selected readout's set, none at a tie.
   `Executor::address_all` (`runtime/cortex-runtime/src/executor.rs`): *"every unit a source and a target, the rule
   before ADR-0068 and the state a new executor starts in."*
2. **The gate under the delivery** ([ADR-0094](../../docs/adr/0094-the-signed-gate-built.md),
   `crates/cortex-core/src/dynamics/synapse.rs`): with the signed gate set, an addressed excitatory synapse consolidates
   under `clamp(baseline + dopamine, −1, 1)` through `consolidate_signed`; any other synapse consolidates under the
   baseline alone. The baseline is zero in the learning configuration, and the inhibitory synapses consolidate under
   their own baseline, 0.5 ([ADR-0086](../../docs/adr/0086-the-inhibitory-baseline-built.md)).
3. **H-23** ([ADR-0135](../../docs/adr/0135-the-critics-window-measured.md), `tests/inhibition.rs`):
   - `windowed_arm`, `windowed_run` and `earned_run_valued` on the shared harness, under `Delivery::Addressed`;
   - the image `WINDOWED_IMAGE_CRC_1024`;
   - its pinned tables (`WINDOWED_BLOCKS_1024` to `SUMS_AFTER_WINDOWED_1024`), its verdict `WINDOWED_1024`, and its
     readings: the reversals, the value per stimulus, the troughs and the strong punishments.
4. **The assertion and the oracle.** H-20 to H-23 assert that no excitatory synapse outside the four stimulus–readout
   pairs moves (`punished_held`, over `Reach`). The harness's oracle replays the consolidation over the synapses of the
   pair the last trial's delivery addressed (`tests/instrument/harness.rs`, the addressed delivery's section). Both are
   written for the addressed delivery. Under the global one every excitatory synapse consolidates, so the assertion
   does not apply, and what the oracle replays is this round's to decide.
5. **The global delivery's last reading** ([ADR-0066](../../docs/adr/0066-the-reward-path-measured-again.md)): under a
   configuration that no longer exists (unsettled prior, baseline 0.5, raw reward, the stimulus as two messages of
   1.25), 53 and 49 of the last 128 correct at 256 and 1 024 units, every coupling falling alike.
6. **The weekly job**: six shards. This round changes no file under `src/`, so it dispatches `scope=exhaustive`
   ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch; after ADR-0135 it plans about 34 per cent of the bound.

## Deliverables

- [x] **H-24's protocol, in a new ADR at the next free number (`ls docs/adr`), before the first rewarded run.**
  - The arms: H-23's, from H-23's image, with `Delivery::Global`. The image is asserted H-23's by its CRC.
  - H-24's clauses and constants, restated from ADR-0136 and pinned in the tests:
    - clause 1: at least 80 of each mapping's last 128;
    - clause 2: no coupling above 1.30 of its image's at any block's end;
    - clause 3: the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's at every block's
      end.
  - The readings, no clause:
    - the four couplings per block beside H-23's;
    - outside the couplings, by source and target class: the excitatory sum, the fraction moved, the largest move and
      the synapses at either rail;
    - where the reward's consolidation went per block: the answer's pairs, the other pairs and the rest;
    - the population's rate by class;
    - the reversal speeds, the value beside $2p - 1$ and the troughs, beside H-23's;
    - the inhibitory sum's course.
  - What the oracle replays under the global delivery, and why that is enough to trust the readings.
- [x] **The calibration**, before any rewarded run: every pinned number holds; in each arm, H-23's first block from
  H-23's image under the addressed delivery reproduces H-23's tables.
- [x] **The runs**: H-24's two arms, each a weekly `exhaustive` test, their tables pinned per block.
- [x] **The ADR's reading.**
  - H-24's verdict per clause and arm.
  - The literature's account (yes) and ADR-0066's reading (no), against the result.
  - The readings.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** At most one runtime test: H-24's clauses at their edges over tables written by hand, and the
  readings' rules.
- [x] **The evidence.** A weekly dispatched on this round's branch at `scope=exhaustive`, green in every job and
  reproducing every pinned number. The cost table is regenerated from that run's artifacts.
- [x] **The documents, in the same pull request.**
  - Whitepaper §11.1's H-24 with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule, parameter or field of the engine changes.** No file under `src/` changes.
- The delivery is the one change from H-23. The baseline, the signed gate, the critic, its window and the inhibitory
  rule are H-23's.
- H-24's clauses and constants do not move after a rewarded run. There is no second attempt.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, and no dependency.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the arms are built on H-23's harness, and how they are dealt into tests;
- what the oracle replays under the global delivery;
- how each reading is computed and pinned;
- whether the readings are read from the record or from the oracle.

It may not:
- change a rule of the engine;
- move a clause or a constant after a rewarded run;
- change anything of H-23's configuration but the delivery;
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
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. In the history, H-24's constants
and clauses precede the first rewarded run.

## Report

The closing message states:
- the protocol, and when it was committed;
- the calibration;
- H-24's verdict per clause and arm, the account's and ADR-0066's readings against it, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved and no file under `src/` changed;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
