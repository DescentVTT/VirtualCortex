---
status: archived
date: 2026-09-29
---

> **Executed 2026-09-29 in pull request #161.** Writes ADR-0129 (a target the network fires at, measured), and opens and
> resolves F-59 (the settled image every learning run starts from holds 0.775 of the prior's inhibitory sum, 45 of its
> 6 528 inhibitory synapses at the rail, where ADR-0128 read the start as the rail). Nothing of the engine changed. The
> target's rule, H-21's clauses and constants, the readings' rules, the gate and the two arms were committed and pushed
> before any run, with the pull request opened as a draft; the target was pinned in a commit of its own before any rewarded
> run; and the eleven earlier whole-domain tests through the changed harness reproduced their pinned numbers before it.
>
> **H-21 is yes, and its stopping rule reached step 3.** The rule read ADR-0077's settled image, frozen, at 18 449 spikes over
> $2^{20}$ ticks, 1.718 Hz a unit, and gave a target period of 58 201 ticks, a depression of 23 per spike against the
> default's 67. In both arms the inhibitory sum stayed at or above 0.848 of the settled image's at every block's end (H-20's
> fell to 0.072 and 0.078), and every mapping was learned, 113 to 124 of each mapping's last 128, no coupling past 1.30. The
> sum rose to 1.01, then fell, and was still falling at the end by about 0.15 per cent of the image's a block: the drain
> slowed rather than stopped. The stimulus units' inhibition rose to the rail; the inhibitory units' and the readouts' fell
> to 0.65 and 0.82. The learning configuration is named with the target, and **the next decision** — an ADR choosing among
> the operating regime, another size and a critic of the engine's own — is named and not taken.
>
> The weekly dispatched at `scope=exhaustive` (run 36561924959) was green in every job: all 73 whole-domain tests passed,
> the 71 before this round reproducing their pinned numbers and H-21's two arms their tables, in 1 705 and 1 790 s. The
> shards' tests ran at 22 to 36 per cent of the bound under the old table's deal. The cost table is regenerated from the run
> (73 lines, 23 654 s) and plans each of the six shards at about 28 per cent, inside the brief's 60. The pull request's gate
> is green in every job, the mutation gate finding no mutant to make in a diff of tests and documents.
>
> The body below describes the tree before execution and is not maintained. Relative links gained one `../` when the brief
> was archived; no word, claim or figure changed.

# Brief 054: A target the network fires at — the inhibitory rule's target period set to the settled network's own rate by a rule written first, then H-20's schedule run once in both arms, and H-21 read: does the inhibition stay at half the image's or more while every mapping is still learned; nothing of the engine changed

## Mission

**This brief runs one hypothesis once and changes no rule.** Under the learning configuration the network's inhibitory
sum falls to 0.07 and 0.08 of the settled image's by the end of H-20's schedule
([ADR-0110](../../docs/adr/0110-a-schedule-of-reversals-measured.md)).

[ADR-0128](../../docs/adr/0128-a-target-the-network-fires-at.md) reads the cause in a parameter:
- the inhibitory rule's target rate is the default 5 Hz, while the settled network fires at about 1.7 Hz;
- from the rail, the rule can only weaken inhibition, so it weakens it onto nearly every unit.

ADR-0128 sets the target to the network's own rate and wrote **H-21** before any run.

When the round is done, the tree holds:
- the target's rule, the target it gives, and H-21's constants, committed before the first rewarded run;
- H-20's two arms run once under that target, as weekly `exhaustive` tests, their tables pinned;
- H-21's verdict and **the step of its stopping rule reached**, with the next decision it names and does not take;
- the cost table regenerated from this round's dispatch.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). This round adopts nothing:
  - no rule of the engine changes: the target period is a parameter of the image
    ([ADR-0053](../../docs/adr/0053-the-waking-day-and-the-target-period.md));
  - H-20's configuration, critic, schedule, arms, seeds and criterion code are the tree's;
  - no dependency, and no version bump of a tool.
- **One change from H-20**: the image's target period. Every other byte of the image each arm decodes is H-20's, shown
  by a masked check.
- **The target is read by ADR-0128's rule and not chosen.** The rule: the settled image's population rate, every unit,
  under ADR-0044's drive, over a lead-in of $2^{20}$ ticks with every weight frozen; the period 100 000 ticks over that
  rate in hertz, rounded to the nearest tick, within ADR-0053's bounds. It is committed before it is computed.
- **H-21's constants and clauses are ADR-0128's** and are committed in the tests before the first rewarded run:
  - clause 1: the inhibitory sum at or above 0.5 of the settled image's at every block's end, in both arms;
  - clause 2: every mapping learned (at least 80 of its last 128) and no coupling above 1.30 of its image's at any
    block's end, in both arms.
- **No constant moves after a rewarded run, and there is no second attempt** (ADR-0128's stopping rule, step 6).
- **The engine is read before a description of it is trusted**, this brief's and ADR-0128's included: the target the
  settled image carries, where the loader takes it from, and how the inhibitory rule reads it.
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`; the runtime's gate grows by at most one test
  ([ADR-0061](../../docs/adr/0061-the-learning-runs-leave-the-gate.md)). The mutation gate on the changed lines must pass
  ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **No shard of the weekly job passes 60 per cent of its bound** under the regenerated deal
  ([ADR-0127](../../docs/adr/0127-the-paused-line-leaves-the-weekly.md) left about a quarter).
- No pinned number of an earlier round moves.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-09-29 against `main` after ADR-0127 and ADR-0128 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The inhibitory rule** (`crates/cortex-core/src/dynamics/synapse.rs`).
   - `ISTDP_TARGET_PERIOD_TICKS` (20 000, 5 Hz) and `istdp_alpha_q1_15(target_period_ticks)`, the period clamped to
     `[ISTDP_PERIOD_MIN_TICKS, ISTDP_PERIOD_MAX_TICKS]`.
   - The rule balances where the target fires at the period's rate: above it the inhibition grows, below it weakens
     ([ADR-0057](../../docs/adr/0057-the-inhibitory-rule-from-below-the-rail.md)).
2. **The target in the executor and the image** (`runtime/cortex-runtime/src/executor.rs`, `image.rs`):
   `Config::istdp_target_period_ticks`, default `ISTDP_TARGET_PERIOD_TICKS`; the modulator section's `[20..24)`; the
   loader takes the image's over the configuration's.
3. **H-20** (`runtime/cortex-runtime/tests/inhibition.rs`, the shared harness):
   - its two arms, `run_on_scheduled`, the critic of shift 5, the flips before trials 1 536, 3 584 and 5 632, 7 680
     trials;
   - its image `PUNISHED_IMAGE_CRC_1024` (restated at format 18, ADR-0123) and its tables;
   - the inhibitory sums it pins by block.
4. **The inhibitory baseline** ([ADR-0086](../../docs/adr/0086-the-inhibitory-baseline-built.md)): 0.5, under which every
   inhibitory synapse consolidates every trial.
5. **The rail**: every inhibitory weight of the prior at `-i16::MAX`, from which the rule can only weaken
   ([ADR-0053](../../docs/adr/0053-the-waking-day-and-the-target-period.md), [ADR-0120](../../docs/adr/0120-the-contexts-own-inhibition-measured.md)).
6. **ADR-0057's reading**: the fraction of units at the target (`spikes_and_at_target` in the harness).
7. **The weekly job**: six shards; a round that changes only tests dispatches `scope=exhaustive`
   ([ADR-0075](../../docs/adr/0075-the-dispatch-scope-follows-the-diff.md)). The cost table is regenerated from its own
   dispatch.

## Deliverables

- [x] **The protocol, in a new ADR at the next free number (`ls docs/adr`), before any rewarded run.**
  - The target's rule (ADR-0128's), the lead-in's run, and the target it gives, pinned.
  - H-21's clauses and constants, restated from ADR-0128 and pinned in the tests.
  - The readings, no clause:
    - the inhibitory sum and the network's rate at every block's end;
    - the fraction of units at the target (ADR-0057's rule) at the start and at every flip;
    - the reversal speeds beside H-20's;
    - the settled network's rate distribution before the first trial, and the share of units at or above the target.
- [x] **The calibration**: ADR-0077's settled image reproduced, and each arm's image shown to be H-20's in every byte
  but the target period, before any rewarded run.
- [x] **The runs**: H-20's two arms under the target, each a weekly `exhaustive` test, their tables pinned per block as
  H-20's.
- [x] **The ADR's reading.**
  - H-21's verdict per clause and arm.
  - The inhibitory sum's course beside H-20's.
  - The readings.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
- [x] **The gate.** At most one runtime test: the target's rule over a table written by hand, H-21's clauses at their
  edges, and the image patch shown to touch only the target.
- [x] **The evidence.** A weekly dispatched on this round's branch at `scope=exhaustive`. It must be green in every job
  and reproduce every pinned number. The cost table is regenerated from that run's artifacts.
- [x] **The documents, in the same pull request.**
  - Whitepaper §11.1: H-21 checked with its verdict and step, and §9.
  - The ADR index, `CHANGELOG.md`, `CLAUDE.md`, and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date
    ([ADR-0064](../../docs/adr/0064-the-documentation-gate-and-the-version.md)).
- [x] **The brief archived** as `briefs/README.md` says, every deliverable dispositioned.

## Not empowered

- **No rule of the engine changes.** The inhibitory rule, its rail, the inhibitory baseline, the signed gate, the
  critic, STDP and the task are the tree's.
- No other target, no inhibition started below the rail, no second attempt. Those are the stopping rule's next steps.
- H-21's clauses, constants and the target's rule do not move after a rewarded run.
- No pinned number of an earlier round moves.
- No change to `.github/workflows/ci.yml`, `scripts/exhaustive-shard.sh` or `scripts/exhaustive-costs.mjs`; the cost
  table changes only by regeneration from this round's dispatch.
- No float anywhere, no dependency, no `unsafe`.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how the target is written into the image each arm decodes, and how the masked check is made;
- the lead-in's run for the target's rule, within its definition;
- where the tests live, and how the two arms are dealt into tests;
- which readings are pinned in full and which by hash;
- whether the round writes one ADR or two.

It may not move H-21's clauses or constants, choose the target by any rule but ADR-0128's, change a rule of the engine,
or reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo test --workspace --release --locked -- --ignored exhaustive --list
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm ci
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <the dispatch's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
```

Every command exits 0. The dispatched run is green and reproduces every pinned number. The target's rule and H-21's
constants precede the first rewarded run in the history.

## Report

The closing message states:
- the target's rule, the target it gave, H-21's constants, and when they were committed;
- the calibration;
- H-21's verdict per clause and arm, the inhibitory sum's course beside H-20's, and the readings;
- the step of the stopping rule reached and the next decision it names, not taken;
- that no pinned number moved;
- the scope ADR-0075 gave the diff, the shards' times and the regenerated cost table;
- what was not done and why.
