---
status: proposed
date: 2026-09-26
---

# Brief 047: A schedule of reversals — H-19's configuration and critic through three reversals, 7 680 trials, the first 56 blocks H-19's bit for bit; does every mapping get learned, and does every coupling stay below 1.30 of its image's

## Mission

**This brief runs one hypothesis and builds no mechanism of the engine.** H-19 is no on clause 3 alone
([ADR-0108](../docs/adr/0108-the-reward-prediction-error-measured.md)): with the critic of
[ADR-0106](../docs/adr/0106-the-reward-prediction-error.md) the engine learned and revised, in both arms, and did not
settle. One answer pair of each mapping still moved by 1.22 to 1.87 per cent over its last 256 trials, where H-18's
moved by 3.6 to 9.5. [ADR-0109](../docs/adr/0109-a-schedule-of-reversals.md) read what keeps them moving:
- the critic's advantage while the selection still errs, which falls as it errs less;
- an error's carry-over into the next trial.

ADR-0109 rejected an expectation per stimulus and readout, because it would stop the learning with the rise. It kept
H-19's configuration, and took H-18's second named question under it: **a schedule of reversals**. It wrote H-20
before any run.

When the round is done, the tree holds:
- H-20 run once in both of H-19's arms over 7 680 trials, with the mapping flipped at trials 1 536, 3 584 and 5 632,
  and the first 56 blocks shown to be H-19's bit for bit;
- the run pinned per block and read against the criterion committed before the first rewarded run: **yes or no**,
  and if no, the clause, the mapping and the block;
- a new ADR that records the verdict and names the next decision H-20's stopping rule makes, **without taking it**.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing:
  - no rule of the engine changes: not the critic, the modulator, the signed gate, the baselines, the address, the
    delivery, the reward or the selection;
  - no dependency, and no version bump of a tool.

  The schedule of flips is the harness's.
- **H-20 is ADR-0109's, exactly:**
  - *The configuration:* H-19's, each arm from H-19's image with its critic (one expectation per stimulus, shift 5).
  - *The schedule:* 7 680 trials, the mapping flipped between trials 1 536 and 1 537, 3 584 and 3 585, and 5 632 and
    5 633.
  - *Asserted:* the first 56 blocks are H-19's arm's bit for bit; no excitatory synapse outside the four
    stimulus–readout pairs moves; the oracle agrees with the record at every trial.
  - *Clause 1:* at least 80 of the last 128 trials of each mapping correct, a tie not correct.
  - *Clause 2:* every stimulus–readout coupling at or below 1.30 of its image coupling at the end of every block.
  - ADR-0109's two predicted readings, its reads, and its stopping rule.
- **Pre-registration.** The flips, the run's length, the two clauses, the assertion, the predicted readings and the
  readings' rules are committed as constants and rules **before the first rewarded run**. None of them moves after a
  run, and there is no second attempt at H-20 in this configuration.
- **A run whose first 56 blocks are not H-19's stops the round** before any reading is taken, and is a finding. So does
  a calibration that does not reproduce, or a pinned number that moves.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The engine is read before a description of it is trusted**, this brief's and ADR-0109's included. ADR-0109's bound
  of 32 full rewards for an expectation per stimulus and readout is arithmetic from the rule, and its 50 and 100 are
  an estimate from ADR-0108's tables.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-26 against `main` at `2c1ab21`, after ADR-0108 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The flip** (`runtime/cortex-runtime/tests/instrument/harness.rs`, `run_on_flipped(exec, task, units, trials,
   flip, observe)`): `flip: Option<usize>`, and `if flip == Some(trial) { task.mirrored = !task.mirrored; }`. One flip.
   A schedule is a list of trials at which the same toggle happens.
2. **H-19's arms** (`runtime/cortex-runtime/tests/inhibition.rs`). The critic's arms and helpers
   (`critic_step`, `earned_run_predicted`), `CRITIC_BLOCKS_1024` and the other `CRITIC_*` tables for 72 blocks, and
   the calibration each arm runs first. The two arms are `the_critic_from_the_assignment_at_1024_units_exhaustive` and
   `the_critic_from_the_mirrored_assignment_at_1024_units_exhaustive`.
3. **What H-19 read** (ADR-0108 and its tables):

   | Reading | Arm 1 | Arm 2 |
   | :--- | ---: | ---: |
   | Highest coupling at any block's end, as a fraction of its image coupling | 1.145 | 1.165 |
   | Selection first at 40 of 64 after the flip, in blocks | 19 | 23 |
   | Correct trials 32 blocks after the flip, of 64 | 59 | 57 |

   Other readings:
   - H-18's couplings passed 1.30 in the 15th and 18th blocks.
   - H-19's first new selection came within 3 to 68 trials of the flip.
   - The inhibitory sum fell in every block, to a tenth of the image's.
4. **The run's cost.** H-19's arms took 1 209 and 1 104 s on the hosted runners and about 450 to 510 s on the
   developer machine. H-20's 7 680 trials are 1.67 times as many.
5. **The weekly job** ([ADR-0092](../docs/adr/0092-the-shards-dealt-by-cost.md),
   [ADR-0075](../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The scope follows the round's diff: a round
   that changes only tests dispatches `scope=exhaustive`. The cost table is regenerated from its own dispatch.

## Deliverables

- [ ] **The schedule in the harness.** `run_on_flipped` generalized to a list of flips, or a sibling beside it. With a
  single flip, or none, every earlier arm's pinned tables must still reproduce.
- [ ] **The calibration, before any rewarded run.** ADR-0077's settled candidate and H-19's image reproduced, as H-19's
  arms reproduce them. A mismatch stops the round, and is a finding.
- [ ] **The constants commit**, preceding the first commit that holds a rewarded outcome. It holds:
  - H-20's two clauses as integer rules over the pinned tables' shape (clause 2 as
    `coupling × 100 ≤ image × 130` for every pair at every block's end);
  - the flips, the run's length and the arms;
  - the assertion that the first 56 blocks are H-19's, held to `CRITIC_BLOCKS_1024` and H-19's other tables;
  - ADR-0109's two predicted readings, written as constants, and the readings' rules;
  - every constant of ADR-0065, ADR-0076, ADR-0077, ADR-0080, ADR-0085, ADR-0093, ADR-0106 and ADR-0109, restated
    unchanged.
- [ ] **The two arms**, from the assignment and from the mirrored assignment, **each its own `exhaustive` test**:
  7 680 trials, the three flips, nothing else changed at a flip. Tables pinned per block as H-19's are.
- [ ] **The assertion.** The first 56 blocks are H-19's; no excitatory synapse outside the four pairs moves; the oracle
  agrees at every trial. A divergence in the first 56 blocks stops the round, as the Standing directives say. Any
  other failure is a numbered finding, reported beside the verdict.
- [ ] **The verdict.** The rule committed first, over the pinned tables: **H-20 is yes or no**, and if no, the clause,
  the mapping and the block. §11.1's H-20 is checked with the result and its scope, and its stopping rule's item with
  the step reached. The ADR states the next decision H-20's rule makes **and does not take it**.
- [ ] **The readings, in every branch**:
  - ADR-0109's two predicted readings beside what the run read;
  - for each reversal: the blocks from the flip to the first block with 40 of 64 correct, each stimulus's first new
    selection and crossing block, beside H-19's first reversal;
  - the errors and ties per mapping, and H-19's settle measure over each mapping's last 256 trials;
  - each expectation and the dopamine signal by block;
  - the highest coupling of each mapping;
  - the inhibitory sum by block.
- [ ] **The gate.** At most one runtime test for H-20, running no whole run:
  - the clauses at their edges over tables written by hand (80 and 79; 1.30 and one LSB above);
  - the schedule's flips on a few trials;
  - the first-56-blocks assertion on a hand-written prefix.
- [ ] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives the diff (`exhaustive`
  if only tests change), the clause stated in the ADR. It must be green in every job, and every pinned number
  reproduced. The run id and the shards' times go in the ADR, and the cost table is regenerated from that run's
  artifacts.
- [ ] **The documents, in the same pull request.** Whitepaper §11.1's H-20 and its stopping rule (the step reached),
  and §9; the ADR index; `CHANGELOG.md`; `README.md`; `CLAUDE.md`'s opening paragraph; `docs/zh-TW`'s reader's guide as
  the result requires. The whitepaper's version in both declarations, with its date
  ([ADR-0064](../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [ ] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, and the frozen banner naming
  the pull request, the ADRs, **H-20's answer and the step of its stopping rule reached**.

## Not empowered

- **No rule of the engine changes**: not the critic, its shift, the modulator, the signed gate, the baselines, the
  address, the delivery, the reward, the selection. No record field and no format.
- **The flips, the run's length and the clauses do not move** after a rewarded run, and there is no second attempt at
  H-20 in this configuration.
- H-19's clause 3 is not reread, and its verdict is not restated.
- No pinned number of an earlier round is restated.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the schedule is written in the harness;
- whether the first 56 blocks are held to H-19's tables table by table or by a hash of the readings;
- where the arms live;
- which further readings it takes beside the criterion's;
- whether the round writes one ADR or two.

It may not reach ADR-0109's configuration, its flips, the run's length, H-20's clauses, the standing directives, the
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
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=<what ADR-0075 gives this diff>
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. The constants commit precedes
the first rewarded outcome in the history.

## Report

The closing message states:
- the schedule, and that every earlier arm still reproduced;
- the calibration, and the first 56 blocks against H-19's;
- H-20's answer, clause by clause, in both arms;
- ADR-0109's predicted readings against what the run read;
- each reversal's speed beside H-19's first;
- the highest coupling of each mapping, the expectations, the dopamine signal and the inhibitory sum;
- the assertion;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why;
- the next decision H-20's stopping rule names, not taken.
