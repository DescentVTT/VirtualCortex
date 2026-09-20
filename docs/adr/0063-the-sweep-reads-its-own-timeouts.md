---
status: accepted
date: 2026-09-20
amends: ADR-0062
decision-makers: VirtualCortex maintainers
---

# ADR-0063: The sweep reads its own timeouts — the weekly job writes, beside `timeout.txt`, when each timeout ran, what ran in the shard's other slot and whether its test run was still finishing tests, because F-42's reading was an afternoon of hand forensics per sweep

## Context and Problem Statement

[ADR-0062](0062-the-first-complete-sweeps-list.md)'s triage rule reads a timeout by five ordered questions, and its fifth kind — a mutant that was not hung at all but passing slowly, starved by the spinning mutant in the shard's other slot (`--jobs 2`, [ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md)) — is whitepaper §11's **F-42**. `timeout.txt` is a list of names and nothing else, so separating that kind from a hang needs three facts the list does not carry: when the mutant ran, what ran beside it, and whether its own test run was still finishing tests when the bound arrived.

All three are already in the artifact the job keeps for ninety days. `debug.log` brackets every mutant with an `Apply` and a `Revert` line carrying a timestamp and the slot's build directory; `outcomes.json` carries every mutant's verdict and the path of its log; the per-mutant log carries the test binary it was inside, cargo's warnings for tests that had not returned, and the results of the tests that did. Reading them is mechanical and it is not quick: F-42's two paragraphs in the whitepaper are the product of downloading an artifact, grepping `debug.log` for a name, converting two timestamps into an overlap and reading a log's tail — per timeout, and there were twenty-six in the sweep that produced ADR-0062.

F-42's own disposition names the two changes that would answer it: run the runtime shards at `--jobs 1`, or read `debug.log` for the neighbour before reading a timeout as a hang. The first doubles a shard: the six ran 56 minutes to 2 h 00 m at `--jobs 2` (run `35485126396`), so at one job each they approach the four hours ADR-0058 sharded them to avoid, and the job's limit is 330 minutes. The second costs nothing and is what this decision does.

## Decision Drivers

- A timeout that hides a survivor is the risk V-6 names; a reading that is expensive is a reading a round will skip.
- Evidence the job already holds should not have to be re-derived by hand from an artifact download.
- A boundary that is mechanical beats one that is a sentence in a finding (CLAUDE.md principle 5), but a verdict that is mechanical and wrong is worse than none: the last two ADRs turned on a distinction no single signal makes.
- The sweep's cost is its wall clock. Nothing here may add to it.

## Considered Options

1. **Run the runtime shards at `--jobs 1`.** Decisive: with one slot there is no neighbour to starve anything. It roughly doubles every runtime shard and would need twelve shards to stay inside ADR-0058's bound, for a fact needed once a week about one or two mutants.
2. **Re-run each timeout alone at the end of its shard.** Also decisive, and priced at the bound per timeout: about twenty hang for the full bound, which is some fifty-five minutes added to the six shards, paid mostly to re-learn that a hang hangs.
3. **Have the job read what it already wrote, and report it.** No wall clock, no decisiveness. The round still applies ADR-0062's rule; it applies it to evidence instead of to a list of names.
4. **Have the job classify each timeout and fail on an unexplained one.** Cheap and wrong: see below, where two facts that each look sufficient disagree on a real mutant.

## Decision Outcome

**Option 3.** `scripts/read-timeouts.mjs` reads a `mutants.out` directory and prints, for every name in `timeout.txt`:

- the window it ran in and the slot it ran in, from `debug.log`'s `Apply`/`Revert` pair, and the bound the lab logged;
- every mutant whose window overlapped it, how many of those timed out, and the longest overlap with its verdict;
- from its own log: the binary it ended inside, whether any test reported **after** the last "has been running for over" warning, the tests that warned and never reported, and whether one of those belongs to the mutant's own module.

The weekly job runs it after the completeness check, writes `timeout-evidence.txt` into `mutants.out` before the artifact is uploaded, and repeats it in the step summary. `npm run mutants:timeouts <dir>` is the same reading on a downloaded artifact; `npm run mutants:timeouts:test` is its tests, inside `npm run spec`.

**It reports and does not classify.** The verdict stays ADR-0062's rule, applied by the round that reads the list. The reason is in the evidence: over the sweep of run `35485126396`, "the run was still finishing tests" alone marks two of the twenty-six, and one of them is a hang. `injector.rs:91:9: replace Injector::pop -> Option<(u32, u32)> with Some((0, 1))` finished `deque::tests::every_pushed_index_is_taken_exactly_once_under_thieves` after its warning — the process was making progress — while `injector::tests::four_producers_and_one_consumer_lose_and_duplicate_nothing`, the test that would have caught it, never returned. Progress on another thread is not progress toward a verdict, so the report carries the second fact beside the first and leaves the two together to a reader.

### What it reads on the sweep that produced ADR-0062

Run [35485126396](https://github.com/DescentVTT/VirtualCortex/actions/runs/35485126396), all seven areas, twenty-six timeouts:

| | Timeouts | Beside another timeout | Stopped finishing tests, or hung a test of their own module | Still finishing tests with no test of their own module hung | Without the evidence to say |
| :--- | ---: | ---: | ---: | ---: | ---: |
| `crates` | 6 | 5 | 0 | 0 | **6** |
| `runtime-0` | 4 | 4 | 4 | 0 | 0 |
| `runtime-1` | 4 | 3 | 4 | 0 | 0 |
| `runtime-2` | 2 | 0 | 2 | 0 | 0 |
| `runtime-3` | 3 | 3 | 2 | **1** | 0 |
| `runtime-4` | 3 | 3 | 3 | 0 | 0 |
| `runtime-5` | 4 | 4 | 4 | 0 | 0 |

The one in the fourth column is `image.rs:564:13: delete field clauses from struct Config expression in Image::decode`, which is the single timeout ADR-0062 read as F-42's kind, by hand, from the same artifact: it ran 1 319 s to 2 320 s beside `Injector::push`'s hang for 591 of those seconds, ended inside `reference`, and the test it left unfinished there is `a_slow_wave_onset_compacts_the_arena_bit_identically_on_one_and_four_workers`, which is not `image`'s. The reading reproduces the round's, and no other timeout of the sweep joins it.

The six in the last column are the `crates` area, and they are why that column exists. That area's suite is about a second, so its bound is thirty-three seconds — under the sixty at which cargo first warns about a test that has not returned — and the log therefore carries no warning to read. The report says so rather than reading the silence as progress. ADR-0062 classified those six as inherent (the three iterators' `next` with the countdown the mutant deleted), which is the first question of the triage rule and needs none of this.

### Consequences

- Good: the three facts F-42 needs arrive with the sweep, in the artifact and in the job summary, for every timeout of every area, at no wall-clock cost. A round reads `timeout-evidence.txt`; ADR-0062's afternoon of forensics was for one timeout.
- Good: a timeout that no longer fits any of the rule's four inherent or directive-class answers is now visible as such in the report — a line with no timeout beside it and no test of its own module hung — rather than waiting for a reader to notice.
- Neutral: the reading depends on cargo's "has been running for over 60 seconds" line and on `debug.log`'s `Apply`/`Revert` pair, which are cargo-mutants 27.1.0's and the test harness's output, not an interface. The pin is exact ([ADR-0030](0030-verification-governance.md)); a version that changes either makes the report say less, not something false, and the tests fix the shapes.
- Bad: it is still not decisive. Only a lone re-run separates a hang from a starved neighbour with certainty, and that is options 1 and 2, whose price is stated above. F-42 is **narrowed**, not closed: the evidence is now free, the inference is not.
- Bad: the step has not yet run in CI. It is written against the artifact of run `35485126396`, all seven areas, downloaded and read; its first execution inside the job is the next weekly run, and `if: always()` keeps it from failing a sweep.

## Alternatives considered and why rejected

- **A verdict per timeout, and a failed job on an unexplained one.** Rejected on the evidence above: the one signal that looks sufficient calls a hang a starved neighbour. A job that fails on a wrong verdict teaches a round to dismiss the check.
- **Reading the neighbour only, without the log.** It is the fact F-42's disposition names, and on this sweep it separates nothing: twenty-two of the twenty-six ran beside another timeout, because a hang starves whatever shares its runner and the starved mutant times out too. Cause and effect look alike from the window alone.
- **Parsing `outcomes.json`'s phase durations instead of the log.** Every timeout's test phase is the bound, by definition; the durations say nothing about what happened inside it.
- **Keeping the reading in a document.** F-42 is exactly that — a disposition that tells a round how to read a timeout — and the round that wrote it still had to do the reading by hand.

## Confirmation

- `scripts/read-timeouts.mjs` and `scripts/read-timeouts.test.mjs` (eleven cases, `npm run mutants:timeouts:test`, inside `npm run spec`), whose fixtures are the three readings of one bound expiring: a hang, a starved neighbour, and a run finishing another thread's tests while its own module's test hangs.
- `.github/workflows/ci.yml`: the "What the timeouts were doing when the bound cut them" step of `mutants-weekly`, before the artifact upload.
- Whitepaper §11: F-42's disposition; Appendix B's V-6 row.
- The table above, reproducible on the artifact of run `35485126396` with `npm run mutants:timeouts <dir>` per area.
