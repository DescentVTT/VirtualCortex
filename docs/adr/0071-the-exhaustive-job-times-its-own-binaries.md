---
status: accepted
date: 2026-09-21
amends: ADR-0030
decision-makers: VirtualCortex maintainers
---

# ADR-0071: The whole-domain tests report what each binary took — one number for thirty-seven tests across four binaries could not say what a round had added, and two rounds left the question undecided on it; cargo already prints the seconds per binary, so the job reads them, and sharding stays a decision for a round that has the numbers

## Context and Problem Statement

The weekly `exhaustive` job runs `cargo test --workspace --release --locked -- --ignored exhaustive` and reports one duration. Every round that measures something adds `#[ignore]`d runs to it ([ADR-0061](0061-the-learning-runs-leave-the-gate.md) moved five there, [ADR-0066](0066-the-reward-path-measured-again.md) added ten, [ADR-0069](0069-the-addressed-reward-measured.md) six, [ADR-0070](0070-where-256-units-settle.md) three), nothing takes a run out, and the job's bound is 120 minutes. The obvious worry is that it grows until it does not fit.

The job's own history does not support that worry, and cannot refute it either:

| Run | Content | Duration |
| :--- | :--- | ---: |
| `35444829805` | brief 027's runs still in the pull request's gate | 32 m |
| `35459078284` | ADR-0061 moved five runs here | 55 m |
| `35485126396` | the same | 33 m |
| `35502753399` | brief 029 added ten | 40 m |
| `35509558631` | **the same content as the row above** | **71 m** |
| `35517026903` | brief 030 added six | 69 m |
| `35528304550` | brief 031 added three | 69 m |

About twenty-one heavy runs were added across those seven runs and the duration went from 32 to 69 minutes, but two runs of **identical content read 40 and 71 minutes**, so runner variance of about 1.8 to one is the larger term, the highest ever seen is 71 of the 120 available, and no trajectory is established. [ADR-0061](0061-the-learning-runs-leave-the-gate.md) measured the same variance directly on an unchanged `reference.rs` (1 238 s against 2 118 s).

So the question — is this job growing, and where — is open, and **it is open because the instrument cannot answer it**. One duration covers thirty-seven `exhaustive` tests in four binaries (`instrument.rs`, `reference.rs`, `learning.rs` and `cortex-core`'s `plasticity.rs`), and a round that adds runs to one of them learns nothing from a total that moves by twice that much for other reasons. ADR-0070 states the worry and leaves it: "the exhaustive job (it grows with every measuring round and nothing takes a run out of it) is now two runs heavier and still undecided." A reviewer of this repository asserted a trajectory to the bound from the same total and was wrong to; that is the shape of the defect, and it is recorded as **F-45**.

## Decision Drivers

- The tree's own rule: a number is Measured when a committed command produced it ([ADR-0010](0010-measured-or-target.md)). "The job grows" is currently neither Measured nor refuted.
- The cheapest instrument that answers a question beats the change that assumes its answer. [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md) made the same choice for F-42: collect the evidence the job already holds, decide later.
- A pinned number that stops being run stops being true. Whatever is done about the job's size, it should not be to stop checking a measurement a round paid for.
- Nothing may be added to the job's wall clock to measure the job's wall clock.

## Considered Options

1. **Shard the job by test binary**, as [ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md) sharded the sweep. It would cut the wall clock to the slowest binary and multiply the headroom — and it would be built on a premise the table above does not support, for a job at 59 per cent of its bound.
2. **Retire the runs of a superseded round** (ADR-0060's five, now that ADR-0066 and ADR-0069 measure the same question through a better instrument). It buys time by no longer checking numbers the tree still cites.
3. **Raise the bound.** It answers nothing and removes the signal that would say something is wrong.
4. **Read what cargo already prints**, and decide when there is something to decide.

## Decision Outcome

**Option 4.** `--report-time` is a nightly flag and the toolchain is pinned to stable ([ADR-0009](0009-rust-edition-and-msrv.md)), but cargo prints a `Running <source> (<binary>)` line before each test binary and a `test result: …; finished in <seconds>` line after it, and runs the binaries one after another. The job now tees its output and a second step pairs those lines, printing the binary, how many `exhaustive` tests it ran, and the seconds — as a table in the job summary, and as `exhaustive-times.txt` in an artifact kept ninety days beside the log, so a later round reads weeks rather than a screenshot. A binary that ran none of them is left out.

**It decides nothing else.** No sharding, no run retired, no bound moved, no test changed. When two or three weekly runs have produced the table, a round can say whether the growth is in `instrument.rs` (which briefs 029, 030 and 032 all feed), whether it is growth at all, and what to do — with numbers of the kind [ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md) had when it sharded the sweep: that decision was taken on a run that had timed out, not on a run that might.

### Consequences

- Good: the question becomes answerable, per binary, at no cost in wall clock — the lines are already printed and thrown away.
- Good: the artifact makes the series readable across weeks, which is the form the question needs; a round comparing two totals could not have separated content from runner.
- Good: a round that adds runs can now state what its own runs cost, which is what briefs 029 to 032 each ask for and none could give beyond the total.
- Neutral: the parser depends on cargo's output — the `Running` line, the `test result:` line and their order. That is the test harness's format on a pinned toolchain, not an interface; a version that changes it makes the table empty, not wrong, and the totals in the job summary would show it. A doc-test section carries no binary of its own, so it is named rather than left to inherit the previous binary's name; under `--ignored exhaustive` all thirty-three of them report none, and the naming is insurance rather than a correction.
- Bad: nothing is fixed. If the job is growing, this round has not slowed it; it has only made the growth visible, and the next round on it pays what this one did not.
- Bad: the step has not yet run in CI. It is written against the shape of the job's output and run end to end locally against a reconstruction of it (a total of 4 144.60 s against the 4 143 s run `35528304550` reported); its first execution in the job is the next weekly, and `if: always()` keeps it from failing one.

## Alternatives considered and why rejected

- **Options 1 and 2 above**, on the reasoning given: one acts on an unestablished premise, the other stops checking a number the documents cite.
- **`cargo nextest`**, which reports per-test times directly. A new tool on the hot path of verification, against Latest ≠ Newest, to read two lines cargo already prints.
- **Timing each binary from the outside**, by enumerating them with `--no-run --message-format=json` and running each. It works — it was written and discarded — but it reimplements what cargo does, and cargo's own result line already carries the seconds.
- **A bound per binary, checked**, in the shape of ADR-0058's per-area timing. That is a gate on a number nobody has yet; it belongs to the round that reads the table.

## Confirmation

- `.github/workflows/ci.yml`: the `weekly` job's tee, the "What each binary of the whole-domain tests took" step and its artifact.
- The step's own shell, extracted from the workflow and run end to end twice. Against **real** output (`cargo test --workspace --exclude cortex-runtime --release --locked -- --ignored exhaustive`, fifty binaries and thirty-three doc-test sections): one row, `cortex_core`, and nothing from the doc-tests. Against a reconstruction carrying the runtime's heavy binaries with a doc-test between two of them: `instrument` 12 in 2 480.55 s, `reference` 6 in 1 238.02 s, `cortex_core` 2 in 41.13 s and the doc-test named as its own row, totalling 21 tests in 3 759.71 s. The first pass of that second case put the doc-test's seconds in the wrong column, because its name held a space and the summary splits on fields; the name is now `doc-tests:<crate>`.
- Whitepaper §11's F-45 and Appendix B's row for the whole-domain tests.
- The first weekly run after this change, whose table is the first entry of the series this decision exists to build.
