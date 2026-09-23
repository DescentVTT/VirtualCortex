---
status: archived
date: 2026-09-23
---

> **Executed 2026-09-23 in pull request #113.** Writes ADR-0090 (the old answer held from the trial after the
> flip: ADR-0089's assertion amended before any run, because a trial's delivery is consolidated in the trial
> after it, so the first trial under the second mapping still consolidates the last reward of the first; the
> old answer's pairs held from the end of the 1 537th trial, the pair the 1 536th did not carry from the end of
> the 1 536th, no excitatory synapse outside the four stimulus–readout pairs; nothing else of H-17 changed) and
> ADR-0091 (the assignment reversed, measured: the calibration reproduced ADR-0077's settled candidate before
> each rewarded run and the first half of each arm reproduced H-16's arm bit for bit; the two arms run once in
> `tests/inhibition.rs`, two weekly tests, the flip `run_on_flipped` on the shared harness). **H-17's answer:
> no**, as ADR-0089 predicted — clause 1 read 128 and 128 and clause 2 read 0 and 0, against 80; after the flip
> the engine selected the other readout 2 and 1 times in 1 536 trials, earned 2 and 1 rewards and tied 0 and 1
> times, the old answer's readout counting 2.2 to 3.2 times the new's, 16 to 24 spikes a trial, and the old
> answer's pairs did not move through 1 534 punishments each; the assertion as ADR-0090 amended it held in both
> arms. **The step of H-17's stopping rule reached: step 4 — H-17 recorded no with its scope, and the next
> decision an ADR choosing between an exploration in the selection and a depression under the gate, with this
> round's readings as the need each must meet; this round did not take it.** No finding. Image format 15; the
> determinism pin untouched; no constant of the instrument or of H-17 moved. Every deliverable is done — the
> assertion's box notes the change ADR-0090 made to it before any run — and the notes under the boxes say
> what each read. Relative links gained one `../` so that they resolve from `archive/`, and one `@spec-ignore`
> directive was placed above the context item whose word "blocks" spec-graph reads as a relation once the brief
> is retired; no other word, claim or figure changed.
> *The body below describes the tree before execution and is not maintained.*

# Brief 040: The assignment reversed — H-17, run once under the criterion, the prediction and the stopping rule ADR-0089 wrote before it

## Mission

**This brief runs H-17, and it runs it once.** [ADR-0087](../../docs/adr/0087-inhibition-off-the-reward-gate-measured.md)
read H-16 yes and named the configuration the engine learns in: every excitatory synapse under the reward's
gate, every inhibitory synapse under a baseline of its own. [ADR-0089](../../docs/adr/0089-the-assignment-reversed.md)
chose, among the candidates H-16's stopping rule named, **the assignment reversed within a run**, and wrote it
as **H-17** before this brief existed: H-16's configuration over **3 072 trials with the answer's mapping
flipped once at the half**, in both directions.

The question is the one a learning engine is judged by after "does it learn": **can it change its mind?** The
prediction is written as a **no**, from two rules of the tree — the selection is a deterministic comparison with
no exploration in it, and under the reward's gate a pair consolidates only when it is selected and correct while
nothing depresses what the reward raised — and the round is run because **the size of the no is the reading**:
how often a learned engine selects the readout it did not learn, and how many rewards it earns after the flip.
That number is the measured need the mechanism after this round must meet.

When the round is done, the tree holds: the calibration reproduced (or the round stopped there); the two arms as
weekly `exhaustive` tests with their tables pinned per block; the assertion that the old answer's pair neither
rises nor falls after the flip, held; the criterion's verdict; H-17 checked in §11.1 with its scope; and **the
step of H-17's stopping rule reached**, with the next decision it names and does not take.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adds **no mechanism**: the task, its
  delivery, its feedback and its selection are as they stand, and the flip is the task's own `mirrored` flag.
  **No** exploration term, noise, temperature or sampling in the gate; no depression under the gate; no reward
  predictor, critic or reward-prediction error; no new learning rule, gradient, surrogate gradient, e-prop,
  eligibility variant, structural plasticity or 2025–2026 method; no dependency, no new tool.
- Every claim is Implemented, Specified, Target or Hypothesis. ADR-0089's prediction — **no**, with the new
  answer pair's coupling at or near the image's — is a **Hypothesis** and is reported as one
  ([ADR-0010](../../docs/adr/0010-measured-or-target.md)). Nothing here says anything about 256 units or Appendix
  A's scale. No timing figure from a developer machine.
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11. Every number of
  ADR-0053 to ADR-0088 stays in them and their tests keep pinning them.
- No `f32`/`f64` anywhere, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)); every loop ends by construction.
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)). **No new crate**;
  no record changes; the image format stays 15.
- **Nothing is chosen after a rewarded run.** H-17's constants, arms, length, the flip's trial, criterion,
  assertion and stopping rule are ADR-0089's; the round's integer form of the criterion is committed **before the
  first rewarded run**, and no clause is dropped or added because of what a run read.
- The determinism pin of [ADR-0030](../../docs/adr/0030-verification-governance.md) does not move; the mutation gate
  on the changed lines must pass; every number an arithmetic oracle can produce is computed by it before the test
  that asserts it — and **the engine is read before a description of it is trusted**, this brief's included.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive` and runs in the weekly job; the gate grows
  by at most one test ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)).
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)); no registry entry
  (F-37). Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-23 against `main` with ADR-0088 and ADR-0089 beside it. Line numbers move; the symbols and
the quoted sentences are what to re-derive.

1. **What H-16 read** ([ADR-0087](../../docs/adr/0087-inhibition-off-the-reward-gate-measured.md)): 128 of the last
   128 correct in both assignments with the excitatory gate at zero and the inhibitory baseline at 0.5, the
   selection crossing 40 of 64 at trials 512 and 256; the assertion held (every excitatory synapse outside the
   answer pairs the image's bit for bit); the inhibitory sum fell to about 0.41 of the image's in twenty-four
   blocks, and the learning held anyway.
2. **What a learned engine looks like** ([ADR-0081](../../docs/adr/0081-the-reinforced-form-measured.md)): at the end
   of the run the answer readout counted **28.5 and 27.8** spikes in the task's window against the other's **12.8
   and 11.6**, every presentation of the last 128 going to its answer.
3. **The selection** (`runtime/cortex-runtime/src/task.rs`, `crates/cortex-basal-ganglia/src/lib.rs` — **read
   them**): `Readout::select` puts each readout's count in as its channel's direct drive and the other's as its
   indirect drive; `compute_gating` selects a channel when `indirect + hyperdirect − direct < 0`, which is the
   larger count, and a tie selects neither. **No noise, no temperature, no sampling.**
4. **The gate's arithmetic** ([ADR-0080](../../docs/adr/0080-the-reinforced-form.md)'s derivation, held on every arm
   of ADR-0081 and ADR-0087): the dopamine signal at a trial's end lies between −0.712 and 0.712, so a punishment
   leaves it negative and the wrong selection's pair consolidates nothing in the trial after it; with the
   excitatory baseline at zero, a pair consolidates **only when it is selected and correct**, and nothing lowers
   what a reward raised.
5. **H-17, as ADR-0089 wrote it** (whitepaper §11.1 and the ADR, which is the authority where this summary is
   shorter): H-16's configuration; **3 072 trials, forty-eight blocks, the mapping flipped once between trial
   1 536 and 1 537** by `Task::mirrored` and nothing else; two arms, the assignment first and the mirrored first;
   the criterion's two clauses (it learned, it revised) in both arms; the assertion; the readings; the prediction
   of no; the stopping rule.
<!-- @spec-ignore -->
6. **The harness** (`runtime/cortex-runtime/tests/instrument/harness.rs`, shared since brief 038;
   `tests/inhibition.rs`, H-16's): `settled_image`, `calibration_holds`, `frozen_from`, `run_on` — which takes a
   `Task` by value and an `observe` callback that does **not** receive the task, so a run that flips the mapping
   at a trial needs a variant of it or a loop of its own — `last_correct` over a slice of blocks, and ADR-0079's
   oracle as ADR-0087 extended it.
7. **The budget** ([ADR-0084](../../docs/adr/0084-the-shards-take-tests.md),
   [ADR-0088](../../docs/adr/0088-a-fourth-shard.md)): the weekly runs four shards by test since ADR-0088, and **the
   floor is the longest single test**, so this round's two arms are **two tests**, not one. H-16's three arms of
   1 536 trials took 1 238 s on ADR-0087's runner; two arms of 3 072 are about the same again. **This brief names
   no dispatch scope**: the round takes it from its own diff under
   [ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md).

## Deliverables

- [x] **The calibration, before any rewarded run (the next free ADR number; `ls docs/adr`)**
  (`depends-on: ADR-0089`; ADR-0087 and H-17's stopping rule named). The settled engine built and held to
  ADR-0077's tables; a frozen first block from the image with both baselines unset, held to ADR-0077's frozen
  run. **A mismatch stops the round**: no rewarded run, a numbered finding in §11.
  **Done** (ADR-0091, the next free number after ADR-0090, which the round wrote first to amend the assertion): in each arm's test before its rewarded run, the lead-in's 96 windows, the quiet run and the two images held to ADR-0077's, and a frozen block from the zero image with the inhibitory baseline unset ADR-0077's frozen run bit for bit; both arms decoding one image, its CRC-64 pinned; no mismatch, no finding.
- [x] **The constants commit**, preceding the first commit that holds a rewarded outcome: the criterion as an
  integer rule over the pinned-table shape (clause 1 over the last two blocks of the first half, clause 2 over
  the last two blocks of the run, each at least `REWARDED_MIN`, both arms), the flip's trial, the arms, ADR-0089's
  prediction written as a constant, the assertion's shape, the readings' rules, and every constant of ADR-0065,
  ADR-0076, ADR-0077, ADR-0080, ADR-0085 and ADR-0089 restated unchanged.
  **Done**: `f2ba2e7` on the branch, after ADR-0090's amendment (`1d06422`) and before the first rewarded run — `tests/inhibition.rs` with the constants, the rules, the gate and every table of the second half empty, the flip on the shared harness (`run_on_flipped`, `earned_run_flipped`).
- [x] **The two arms** — the assignment first, the mirrored first — **each its own `exhaustive` test**, each 3 072
  trials from the one image with the excitatory baseline zero and the inhibitory baseline 0.5, the mapping flipped
  once between trials 1 536 and 1 537 and nothing else changed at the flip; their tables pinned per block: the
  correct selections under the mapping in force, the selections per stimulus and readout, the ties, the four
  stimulus–readout couplings, the rewards earned, the two readouts' counts per stimulus, the arena's sums, the
  sight, the stimulus's volley and the dopamine signal.
  **Done**: `the_assignment_reversed_from_the_assignment_at_1024_units_exhaustive` and `the_assignment_reversed_from_the_mirrored_assignment_at_1024_units_exhaustive`; the first half of each held to H-16's pinned arm, which it equals bit for bit, and the second half's tables (`REVERSAL_*_1024`) pinned per block from one run and reproduced by a second on the same machine (`c44ce0c`).
- [x] **The assertion.** In both arms the pair that was the answer before the flip has, at the 3 072nd trial,
  exactly the coupling it had at the 1 536th — bit for bit, every synapse — and no synapse outside the four
  stimulus–readout pairs moved. If it fails, a numbered finding against ADR-0080's derivation as ADR-0089 extends
  it, reported beside the verdict and not in place of it.
  **Amended before any run** (ADR-0090): as written here it would fail by the rule it rests on, the 1 537th trial consolidating the 1 536th trial's reward; so the old answer's pairs are held from the end of the 1 537th trial, the one the carry-over did not reach from the end of the 1 536th, and no *excitatory* synapse outside the four pairs from the image's — the inhibitory synapses move under their own baseline by H-16's rule, so the sentence above is read as the excitatory clause. **Held** in both arms: the carry-over moved 69 and 68 synapses of B's old pair in the 1 537th trial and nothing else; no finding.
- [x] **The verdict.** The rule committed first, over the pinned tables: **H-17 is yes or no**, and §11.1's H-17 is
  checked with the result and its scope, and its stopping rule's item with the step reached. If clause 1 fails,
  the round reports a failure to replicate H-16 and **not** an answer to H-17. The ADR states the next decision
  H-17's rule makes **and does not take it**.
  **Done** (ADR-0091): no — clause 1 128 and 128, clause 2 0 and 0; §11.1's H-17 checked with its scope, its stopping rule at step 4, the next decision named and not taken.
- [x] **The readings — the measured need.** Per block after the flip, and the same before it: the trials in which
  the engine selected the readout that is the answer under the mapping in force, the rewards earned, the ties, and
  the two readouts' counts per stimulus with their gap; whether the new answer pair's coupling moved at all, and
  by how much; the inhibitory sum's course; the sight; whether the stimulus still fires once. If no reward is
  earned after the flip in either arm, the ADR says so as the number it is.
  **Done** (ADR-0091): after the flip the new answer was selected 2 and 1 times of 1 536, with 2 and 1 rewards (the first at trials 1 763 and 2 374) and 0 and 1 ties; the old answer's readout 2.2 to 3.2 times the new's, 16 to 24 spikes a trial, the gap before the flip's; the new answer's pairs +3 415 and +625 in the assignment first and 0 and +162 in the mirrored first; the old answer's trace still pairing +5 to +10 per synapse per trial; the inhibitory sum to 0.173 and 0.170 of the image's, falling in every block; the sight 64 and the stimulus firing once in every block; before the flip, H-16's readings bit for bit.
- [x] **The gate.** At most one test, running no whole run: the criterion's two clauses at their edges over tables
  written by hand (including a clause-1 failure), and a few trials over the flip on the instrument's network —
  the mapping in force changing the outcome's correctness and nothing else.
  **Done**: one test, `a_few_trials_over_the_flip_at_1024_units_and_the_rules_of_the_assignment_reversed` — the clauses at their edges with a clause-1 failure, eight trials on the instrument's network flipped before the fifth beside five with no flip (the carry-over showing there too), and the checks over the pinned tables.
- [x] **The evidence.** A weekly dispatched on this round's branch at the scope ADR-0075 gives **this round's own
  diff**, the clause that applied stated in the ADR; green in every job it runs, every pinned number reproduced;
  its run id and the **four** shards' times, with the shard holding this round's tests against its bound, in the
  ADR.
  **Done** (ADR-0091's evidence): no file under `src/` changed and no test was removed or moved out of the swept suite, so ADR-0075's last clause applied and the weekly was dispatched at `scope=exhaustive`; the run id and the four shards' times are in the ADR.
- [x] **The documents, in the same pull request.** Whitepaper §11.1's H-17 and its stopping rule (the step
  reached), §6.5's loop, §9; the ADR index; `CHANGELOG.md`; `README.md`; `CLAUDE.md`'s opening paragraph;
  `docs/zh-TW`'s reader's guide as the result requires. The whitepaper's version moves in **both** declarations
  with its date ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)), in the same commit as
  any edit to it.
  **Done**: whitepaper 4.42.0 (ADR-0090: §11.1's assertion, §9) and 4.43.0 (§6.5 and its directives, §8.8, §9, §11.1's H-17, its stopping rule and the operating regime's item); the ADR index; `CHANGELOG.md`; `README.md`; `CLAUDE.md`; the zh-TW guide.
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned, the frozen banner naming
  the pull request, the ADRs, **H-17's answer and the step of its stopping rule reached**.
  **Done**: this file.

## Not empowered

- **No constant of H-17 moves**: not the 3 072 trials, the flip's trial, the two baselines, the reward of 1.0, the
  feedback, the delivery, the window, the trial, the seed, the last 128 trials, the mark of 80, or the tie counted
  as not correct. They are ADR-0089's.
- **No exploration is added to the selection** — no noise, no temperature, no sampling, no tie-breaking rule — and
  **no depression under the gate**. Those are the mechanisms a no would name, and naming them is the next
  decision's, not this round's.
- **No constant of the engine moves**; the gain stays 1.75, the controller off; the stimulus is ADR-0076's,
  pinned; the network is ADR-0077's settled engine.
- **No second flip, no schedule of reversals, no flip before the first half has learned.**
- **No rewarded run unless the calibration reproduces.** No constant chosen or moved after a rewarded run; no
  criterion changed after a run; **no second attempt** at H-17 in this configuration.
- **The round does not take the next decision** its answer names, and writes nothing beside H-12 or H-16.
- No record change, no format bump; no new crate; no dependency in a state crate; no `unsafe` outside the
  runtime's arena access; no float anywhere.
- **The two arms are two tests**, so that no shard's floor is one test of both (ADR-0088). No change to
  `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or the sweep's configuration.
- "Learns" and "revises" only with their scope: this task, these arms, this flip, 3 072 trials, this network at
  1 024 units, the excitatory gate at zero and the inhibitory baseline at 0.5.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR that owns it:
how the flip is made — a variant of `run_on` that hands the task to a callback between trials, a loop of its own,
or two runs on one executor — so long as the two halves are one run on one engine, nothing but `mirrored` changes
at the flip, and every pinned run of an earlier round is unchanged; whether the arms live in `inhibition.rs`, a
binary of their own or elsewhere (ADR-0088 removed the naming rule); whether the calibration is its own test;
how often the composition is read, if it is read at all, so long as the blocks around the flip are; whether the
round writes one ADR or two; and which numbers the ADR restates. **A defect found in H-17's criterion, assertion
or stopping rule before the first rewarded run** is written as an ADR amending ADR-0089, with the reason, and
committed before that run; never after it. The round may not reach the standing directives, the whitepaper's
invariants, the constraints in `CLAUDE.md`, or H-17's criterion and stopping rule by any other route.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo test -p cortex-runtime --release --locked -- --ignored exhaustive
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
```

Every command exits 0; the in-diff gate reports no survivor in CI. The dispatched run is green in every job it
runs and reproduces every pinned number. The constants commit precedes any commit that holds a rewarded outcome.
The determinism pin, the image format and every number a prior round pinned are unchanged.

## Report

The closing message states: **whether the calibration reproduced**; **H-17's answer, yes or no**, with clause 1's
and clause 2's counts in both arms against 80, and **the step of H-17's stopping rule reached** and the next
decision it names; whether the assertion held — the old answer's pair unchanged from the flip to the end, bit for
bit; **the measured need**: after the flip, the trials in which the engine selected the other readout, the rewards
it earned, the ties, and the counts' gap, per block and over the last 128; whether the new answer pair's coupling
moved at all; the inhibitory sum, the sight and the stimulus's volley; the scope ADR-0075 gave this diff and the
clause that applied; the four shards' times against their bound; what the mutation gate and any sweep found; what
was not done and why; and what the re-examination after the round recommends next.
