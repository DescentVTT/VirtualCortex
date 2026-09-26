---
status: archived
date: 2026-09-26
---

> **Executed 2026-09-26 in pull request #134.** Writes ADR-0107 (the critic built) and ADR-0108 (the reward-prediction
> error, measured); opens and resolves findings F-52 (ADR-0106 mis-rounded three of H-18's moves: the range is 3.6 to 9.5
> per cent, not 3.7) and F-53 (ADR-0106's arithmetic of the punishment's fading left out the dopamine signal's
> carry-over from trial to trial). The critic (`Task::critic`, off unless set, no record field, image format 16) and
> H-19's constants, clauses, assertion, readings' rules and gate were committed before the first rewarded run; every
> other whole-domain test then reproduced its pinned numbers with the critic unset, and each arm reproduced ADR-0077's
> settled candidate, H-18's image and H-18's first block before its own run.
>
> **H-19 is no, clause 3 the one that failed.** Clause 1 read 122 and 125 of 128, clause 2 read 119 and 121, and in
> both arms one answer pair of each mapping moved by 1.22 to 1.87 per cent of its image coupling over its span, the
> other within 0.34, where H-18's moved by 3.6 to 9.5. ADR-0106's two predicted readings held; the assertion held. The
> expectations settled about a tenth below the reward because the selection stayed wrong or tied in two to five trials
> of 64. **H-19's stopping rule is at step 4**, clause 3 alone: the next decision is an ADR on what else keeps the
> couplings rising, named and not taken.
>
> The weekly dispatch at `scope=both` (run 36236559958) was green: all 65 whole-domain tests passed, the sweep found no survivor, and the cost table is regenerated from it. Image format 16; whitepaper 4.59.0.
> Every deliverable is dispositioned below. Relative links gained one `../` so that they resolve from `archive/`; no
> other word, claim or figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 046: The reward-prediction error — a critic in the task, an expected reward per stimulus from which the outcome's reward is taken before the dopamine signal receives it; then H-19 run once on H-18's configuration, schedule and arms: does the engine learn, revise and settle

## Mission

**This brief builds one mechanism and runs one hypothesis.** H-18 is yes
([ADR-0096](../../docs/adr/0096-the-punished-pair-measured.md)): the engine learns the two-alternative mapping and, with
the signed gate, learns it again after a flip. But its reward is the outcome itself every trial, so a learned mapping
is reinforced for ever. Over the last 256 trials of each mapping, with every trial correct, the answer pairs rose by
3.7 to 9.5 per cent of the image's coupling, "the rise has no stop but the rail".

[ADR-0106](../../docs/adr/0106-the-reward-prediction-error.md) takes the reward-prediction error:
- The task keeps an expected reward for each stimulus, $V_s$.
- The dopamine signal receives $\delta = r - V_s$.
- $V_s$ then moves by $\delta \gg 5$.

ADR-0106 wrote H-19 before any run, with its criterion, its predicted readings, the tension the rules predict between
settling and revising, and its stopping rule.

When the round is done, the tree holds:
- the critic, built and tested, off unless set, with every pinned number of every round unchanged while it is unset;
- H-19 run once in both of H-18's arms, pinned per block and read against the criterion committed before the first
  rewarded run: **yes or no**, with the clause that failed if no;
- a new ADR that records the verdict and names the next decision H-19's stopping rule makes, **without taking it**.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts one rule, ADR-0106's critic, and
  nothing else:
  - no exploration in the selection, and no Q-value;
  - no change to the modulator's rules, the signed gate, the baselines, the address or the delivery;
  - no new dependency, and no version bump of a tool.
- **The critic is ADR-0106's, exactly:**
  - it is the task's state, not a record's: **no record field moves, and the image format stays 16**;
  - $V_s$ is Q16.16 per stimulus, zero at the run's start;
  - $r = \pm$`reward_q16` as `Feedback::Answer` gives it, a tie an error;
  - $\delta = r - V_s$, saturating; $\delta$ is what `Executor::reward` receives and what the trial records as its reward;
  - then $V_s \leftarrow V_s + (\delta \gg 5)$, an arithmetic shift, saturating;
  - **the shift is 5 and does not move after a rewarded run.**
- **Pre-registration.** H-19's criterion, its assertion, ADR-0106's predicted readings and the readings' rules are
  committed as constants and rules **before the first rewarded run**. The flip's trial (between 1 536 and 1 537) and
  the run's length (4 608) do not move, and there is no second attempt at H-19 in this configuration.
- **Every pinned number of every round stands with the critic unset**, H-18's arms among them, and the determinism pin
  with them. A pinned number that moves with the critic unset stops the round before any rewarded run and is a finding.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)). Nothing allocates after start-up.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). A new rule carries a test over the lattice of
  `testkit/prop.rs`, and the mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **The engine is read before a description of it is trusted**, this brief's and ADR-0106's included. In particular,
  ADR-0106's arithmetic of the punishment's fading ($\delta_n = -2(31/32)^n$, about 54 presentations' worth) takes no
  account of the dopamine signal's carry-over from one trial into the next. It is a statement about the rules, not a
  reading.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-26 against `main` after ADR-0104 merged. Line numbers move; the symbols and the quoted sentences
are what to re-derive.

1. **The reward** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`). After the selection and the address,
   `Feedback::Answer` gives `positive = correct`, then
   `reward_q16 = if positive { self.reward_q16 } else { self.reward_q16.saturating_neg() }`, then
   `let signal_q16 = exec.reward(reward_q16)`. `Outcome` records `reward_q16` and `signal_q16`. `Task` is `Copy` and
   holds every field of the task; `check` refuses a task that cannot run.
2. **The modulator** (`crates/cortex-neuromod/src/lib.rs`). `reward(reward_prediction_error_q16)` adds its argument to
   `dopamine_rpe`, saturating. `decay_dopamine(DOPAMINE_TAU_SHIFT)` runs every tick ($2^{14}$). `signed_modulation`
   is `baseline + dopamine` clamped to $[-1, 1]$.
3. **H-18's configuration** (`runtime/cortex-runtime/tests/inhibition.rs`, `tests/instrument/harness.rs`).
   - `PUNISHED_TRIALS` (4 608), the flip at `FLIP` (1 536), `PUNISHED_ARMS`.
   - `run_on_flipped(exec, task, units, trials, flip, observe)`.
   - `REWARD_Q16` (1.0) and `TRIAL_TICKS` ($2^{14}$).
   - `REWARDED_MIN` (80) over `LAST_BLOCKS` (2) of `BLOCK` (64).
   - The `Block` tuple, whose eleventh field is the four couplings `[stimulus][readout]`, and `IMAGE_COUPLINGS_1024`.
   - `PUNISHED_BLOCKS_1024` and the other `PUNISHED_*` tables.
   - The oracle's signed branch, `earned_run_signed`.
   - The calibration that reproduces ADR-0077's settled candidate before any rewarded run.
4. **What H-18 read** (ADR-0096 and `PUNISHED_BLOCKS_1024`, read as fractions of the image's coupling):

   | Answer pairs | Arm | Movement | H-19 bound (clause 3) |
   | :--- | :--- | :--- | :--- |
   | First mapping, trials 1 281–1 536 | assignment first | +3.7 %, +7.6 % | < 1 % |
   | | mirrored first | +4.2 %, +9.5 % | < 1 % |
   | Second mapping, trials 4 353–4 608 | assignment first | +7.1 %, +8.8 % | < 1 % |
   | | mirrored first | +4.9 %, +6.5 % | < 1 % |

   Other readings from H-18:
   - The dopamine signal was 1.712 at the end of every correct block, and −1.712 after the flip.
   - Each stimulus crossed 14 to 20 blocks after the flip.
   - The first mapping's pairs stood at 1.372 to 1.441 at the flip.
   - The inhibitory sum ended at 0.11 of the image's.
5. **ADR-0106's H-19** (whitepaper §11.1): the configuration, the three clauses, the assertion, the two predicted
   readings, the four reads, the tension and the stopping rule.
6. **The weekly job** ([ADR-0092](../../docs/adr/0092-the-shards-dealt-by-cost.md),
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The critic changes `src/`, so the scope is
   `both`. The cost table is regenerated from the round's own dispatch. Each of H-18's arms takes about 1 000 to
   2 000 s on a runner.

## Deliverables

- [x] **The critic (a new ADR at the next free number; `ls docs/adr`)**, in `Task`, off unless set, as ADR-0106 wrote
  it. `Outcome` carries the prediction error delivered, and the expected reward before and after the trial, in the
  form the round chooses. Tests:
  - $\delta$ and the update at their edges: $V_s$ at $\pm r$; $\delta$ of $\pm 2r$; the arithmetic shift's floor for a
    negative $\delta$; a tie;
  - $V_s$ staying within $[-r, r]$, over the lattice of `testkit/prop.rs`;
  - `check`'s refusals for a critic that cannot run, if any;
  - with the critic unset, `Task::trial` returns every field it returned before.

  **Every pinned number of the tree and the determinism pin unchanged with the critic unset.**
- [x] **The calibration, before any rewarded run.** The settled engine held to ADR-0077's tables, and H-18's image and
  first block held to what H-18 read, as H-18's arms do. **A mismatch, or any pinned number that moved, stops the
  round**: no rewarded run, and a numbered finding in §11.
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome. It holds:
  - H-19's three clauses as integer rules over the pinned tables' shape (clause 3 as $|\Delta| \times 100 <$ the image
    coupling, for each answer pair over its span);
  - the critic's shift (5), the 4 608 trials and the flip, and the arms;
  - ADR-0106's two predicted readings, written as constants;
  - the assertion's shape and the readings' rules;
  - every constant of ADR-0065, ADR-0076, ADR-0077, ADR-0080, ADR-0085, ADR-0093 and ADR-0106, restated unchanged.
- [x] **The two arms**, the assignment first and the mirrored first, **each its own `exhaustive` test**: 4 608 trials
  from H-18's image with the critic set, the mapping flipped once between trials 1 536 and 1 537, nothing else changed
  at the flip. Their tables are pinned per block as H-18's are, plus each stimulus's expected reward at the block's
  end.
- [x] **The assertion.** No excitatory synapse outside the four stimulus–readout pairs moves in either arm, and the
  oracle, fed the prediction error each trial delivered, agrees with the record's weights, traces and signal at every
  trial. If it fails, that is a numbered finding, reported beside the verdict and not in place of it.
- [x] **The verdict.** The rule committed first, over the pinned tables: **H-19 is yes or no**, and if no, which
  clause failed. §11.1's H-19 is checked with the result and its scope, and its stopping rule's item with the step
  reached. The ADR states the next decision H-19's rule makes **and does not take it**.
- [x] **The readings, in every branch**:
  - ADR-0106's two predicted readings beside what the runs read;
  - the trials after the flip whose modulation of the old answer's pair was at −0.5 or below;
  - each stimulus's expected reward by block;
  - the dopamine signal by block;
  - the answer pairs' course by block, beside H-18's;
  - each stimulus's first new selection and its crossing block, if any;
  - the inhibitory sum, the sight, and whether the stimulus still fires once.
- [x] **The gate.** The critic's unit and property tests; and at most one runtime test for H-19, running no whole run:
  - the criterion's clauses at their edges over tables written by hand, clause 3 at 1 per cent and one LSB either side;
  - a few trials on the instrument's network with the critic set, where the delivered reward is $\delta$, $V_s$ moves
    by $\delta \gg 5$, every unaddressed excitatory synapse is unmoved, and the oracle is held at every trial.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives the diff (`both`, since
  `src/` changes), the clause stated in the ADR. It must be green in every job, every pinned number reproduced, and the
  sweep's survivors dispositioned. The run id and the shards' times go in the ADR, and the cost table is regenerated
  from that run's artifacts.
- [x] **The documents, in the same pull request.** Whitepaper §11.1's H-19 and its stopping rule (the step reached),
  the task's paragraph where §6 describes the reward, and §9; the ADR index; `CHANGELOG.md`; `README.md`;
  `CLAUDE.md`'s opening paragraph; `docs/zh-TW`'s reader's guide as the result requires. The whitepaper's version in
  both declarations, with its date ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the frozen banner naming
  the pull request, the ADRs, **H-19's answer and the step of its stopping rule reached**.

## Not empowered

- **No second mechanism**: no exploration, no Q-value, no change to the selection, the modulator, the signed gate, the
  baselines, the address, the delivery or the reward's magnitude.
- **No critic in a record or the image**; no record field and no format change.
- **The critic's shift does not move** after a rewarded run, and neither do the clauses, the flip or the run's length.
  There is no second attempt at H-19 in this configuration.
- No pinned number of an earlier round is restated. With the critic unset every one stands.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, no `unsafe` outside the runtime's arena.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- where in `Task` the critic lives and how it is set (a field, a variant of `Feedback`, or a struct beside it), so long
  as it is off unless set and `Task::trial` is unchanged while it is;
- what `Outcome` carries of the critic;
- how the arms reuse H-18's harness;
- which further readings it takes beside the criterion's;
- whether the round writes one ADR or two.

It may not reach ADR-0106's rule, its shift, H-19's clauses, the flip, the run's length, the standing directives, the
whitepaper's invariants, or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo test -p cortex-runtime --release --locked --test inhibition -- --ignored exhaustive
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=<what ADR-0075 gives this diff>
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The in-diff gate reports no survivor in CI. The dispatched run is green and reproduces every
pinned number. The image format is 16, and the constants commit precedes the first rewarded outcome in the history.

## Report

The closing message states:
- the critic and its tests, and that every pinned number stood with it unset;
- the calibration;
- H-19's answer, clause by clause, in both arms;
- ADR-0106's predicted readings against what the runs read;
- the punishment's course after the flip, each stimulus's expected reward and the dopamine signal by block, and the
  answer pairs beside H-18's;
- the assertion;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what the mutation gate and the sweep found;
- what was not done and why;
- the next decision H-19's stopping rule names, not taken.
