---
status: accepted
date: 2026-10-09
depends-on: ADR-0030
decision-makers: VirtualCortex maintainers
---

# ADR-0159: The gate's jobs bounded — a job of the pull request's gate sat in its test step for half an hour on a change that touched no Rust source, was cancelled by hand and left no log; each test step of the three Rust jobs is bounded at three times the longest it took in twelve green runs, and each job at an hour, so that a hang fails in bounded time and keeps its log; the cause not known, F-67

## Context and Problem Statement

**What happened**, on 2026-10-09, on pull request 191, which changed documents and the workflow and no Rust source:
- Attempt 1 of run [37881932940](https://github.com/DescentVTT/VirtualCortex/actions/runs/37881932940): the job *Rust (check, test, fmt, clippy)* entered its step `Test`, `cargo test --workspace --locked`, at 04:01:43Z. It was still in it at 04:34Z, when the run was cancelled by hand.
- In the same attempt the jobs on the MSRV and on AArch64 ran the same tests and passed.
- Attempt 2 of the same job passed. Its `Test` step took 7 m 46 s.
- **The cancelled job's log was not kept.** The API answers `BlobNotFound` for it, and the attempt's log archive holds the four other jobs' logs and not this one. Hours later it is still so.

**What is not known**: whether a test hung, or the runner stopped answering. A missing log fits a runner that stopped; it does not exclude a test.

**What made it cost more than a re-run**: the job had no bound of its own. The platform's is six hours. A watcher that merges on green would have waited, and a hang nobody was watching would have held its pull request for the day.

**How long the steps take**, over the last twelve green runs of the gate:

| Job | Step | Shortest | Longest | The job, longest |
| :--- | :--- | ---: | ---: | ---: |
| Rust (check, test, fmt, clippy) | `Test` | 302 s | 651 s | 849 s |
| | `Test in the release profile` | 86 s | 192 s | |
| Rust (minimum supported version) | `Test on the MSRV` | 434 s | 925 s | 947 s |
| Rust (AArch64 determinism, T-1) | `Test on AArch64` | 589 s | 681 s | 704 s |

## Decision Drivers

- **A structural boundary beats a reviewed one** (principle 5): a hang should fail by itself, not when someone looks.
- **Evidence**: the next occurrence has to leave its log, which this one did not.
- **The commands stay the commands**: the gate runs what `CLAUDE.md` lists, unchanged.
- Latest ≠ Newest: a step's and a job's `timeout-minutes` are the platform's own keys. Nothing is adopted.

## Considered Options

1. (a) a bound on each test step, and a wider one on each job; (b) a bound on the jobs alone; (c) the tests run under `timeout` in the step's command; (d) nothing, and a re-run when it happens.
2. The bound: (a) three times the longest in twelve green runs, rounded up to five minutes; (b) a round number for all.

## Decision Outcome

**Options 1(a) and 2(a).**

- **Each test step is bounded** at three times the longest it took, rounded up to five minutes:

  | Step | Longest | Bound |
  | :--- | ---: | ---: |
  | `Test` | 651 s | 35 min |
  | `Test in the release profile` | 192 s | 10 min |
  | `Test on the MSRV` | 925 s | 50 min |
  | `Test on AArch64` | 681 s | 35 min |

- **Each of the three jobs is bounded at 60 minutes**, past every step's bound. It is the outer bound, for a runner that stops answering, where a step's bound cannot fire.
- **A step that passes its bound fails**, and its log up to there is the step's log like any other failure's. That is what the next occurrence leaves, if the runner is alive.
- **The step's command is unchanged.**
- **F-67 is opened and narrowed, not resolved**: the cause is not known.

### What does not change

- What the gate runs and what it requires before a merge.
- The mutation gate on the changed lines, the documentation gate and the weekly jobs, which keep the bounds they have.

## Consequences

- Good: a hang fails in 35 or 50 minutes where it would have run for six hours.
- Good: a hung test leaves the names of the tests that were running, from the harness's own warnings in the log.
- Bad: a bound is a number that the suite can outgrow. Three times the longest is the room; when a step's longest passes a third of its bound, the bound moves, by an ADR's row or a commit that says so.
- Bad: if the runner stopped, the job's bound cuts it at an hour and there is still no log.
- Neutral: one occurrence. Nothing here says how often it happens.

## Alternatives considered and why rejected

- **Option 1(b), the jobs alone**: a job that passes its bound is cancelled as this one was, and this one kept no log.
- **Option 1(c), `timeout` in the command**: the same effect, and the command would no longer be the one `CLAUDE.md` lists.
- **Option 1(d), nothing**: it costs six hours the next time nobody is watching.
- **Option 2(b), one round number**: the MSRV's step takes half again as long as the others.

## Confirmation

- `.github/workflows/ci.yml`: `timeout-minutes` on the jobs `rust`, `msrv` and `arm64` and on their four test steps.
- Whitepaper §11's F-67 and §9's row.
