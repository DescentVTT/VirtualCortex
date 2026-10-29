---
status: proposed
date: 2026-10-29
title: The pinned tables in a file a round, a row a line
kind: process
summary: A round's pinned tables move out of the tests that read them into a file of the round's own, such as tests/inhibition/pins/h45.rs, a row a line and rustfmt kept off. They stay typed consts in the test sources. spec:pins holds the layout. The schedule's sweep of 2026-10-28 is recorded.
decision-makers: Menteion maintainers
---

# ADR-0227: The pinned tables in a file a round, a row a line

**In brief**
- **Decision.** The tables a round pins from its runs live in `tests/<binary>/pins/<round>.rs`, one file a round, and the tests that read them stay in `tests/<binary>.rs`.
  - Each row is one line, in Debug's form as the round's DUMP prints it.
  - `#[rustfmt::skip]` on the `mod pins` declaration keeps rustfmt off every round's file.
  - The tables stay typed `const` and `static` items in the test sources ([F-81](../WHITEPAPER.md#11-risks-and-technical-debt)).
  - `npm run spec:pins` holds the layout.

  This pull request moves 554 tables out of `tests/inhibition.rs` and 317 out of `tests/assembly.rs`, every token as it was.
- **Why.** `tests/inhibition.rs` was 471 723 lines and 15.1 MB, about 91 % of them tables, and each measured round added about 22 000 lines at its end. Its diffs could not be reviewed, its tests were buried among the tables, and an agent could not read it.
- **Left open.** The tables the other binaries pin, in the harness, `everywhere.rs`, `learning.rs` and `active.rs`, stay where they are. So do the one-line pins.

## Context and Problem Statement

**The two files on `main` at `48783c9`:**

| File | Lines | Bytes | `const` and `static` items | Of them, over one line | Their lines |
| :--- | ---: | ---: | ---: | ---: | ---: |
| `tests/inhibition.rs` | 471 723 | 15 067 299 | 1 120 | 598 | 427 958 |
| `tests/assembly.rs` | 70 427 | 2 664 047 | 497 | 325 | 60 905 |

- **The growth.** The last three measured rounds added 21 732 (H-43), 23 220 (H-44) and 21 801 (H-45) lines to `tests/inhibition.rs`, each at its end.
- **The cause is rustfmt's layout.** It lays a row longer than its widths out one field a line. A `Block` row is 152 columns on one line and 14 lines under rustfmt.
- **The cost to build is settled.** [Exploration 002](../explorations/002-what-the-pinned-tables-cost.md) read, exploratory, the release build no slower with the tables in the source than in a data file. [ADR-0212](0212-three-in-four-on-the-default.md) resolved F-81 so: the tables stay in the test sources, as typed `const` items, with no parser and no data file.
- **The burden is the reader's.** A round's pull request adds 20 000 lines to one file. The file's 43 000 lines of tests, rules and constants sit among 428 000 lines of tables, and a tool that reads a file whole cannot read it.

**The schedule's sweep of 2026-10-28, recorded here** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).
The schedule's run on `main`, [40791077285](https://github.com/Menteion/Menteion/actions/runs/40791077285) at `48783c9`, ended green at 05:29Z on 2026-10-29. No ADR merged since has recorded it.
- It was created at 23:59:40Z on 2026-10-28, 3 h 42 min after its cron of 20:17Z.
- Its eighteen weekly shards took 33.8 to 62.6 minutes against their bound of 240.
- Its seventeen sweep jobs started at 01:02Z. The crates' took 33.7 minutes, and the runtime's sixteen 134.2 to 266.9, under their bound of 330.

| Mutants | Count |
| :--- | ---: |
| Listed | 4 189 |
| Tested | 4 189 |
| Caught | 3 987: 2 469 in the state crates, 1 518 in the runtime |
| Missed | 0 |
| Unviable | 163 |
| Timeouts | 39 |

- **22 timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent**, as on 2026-10-26:
  - `menteion-core`'s chain iterators, six: `DeltaChain`, `MailboxDrain` twice and `Chain` three times;
  - the injector, twelve;
  - the barrier, one;
  - the workers' stop and run, three.
- **17 others, where 2026-10-26 had 5**, read by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence:

| Where | The mutant | Beside a timeout | Its run |
| :--- | :--- | :--- | :--- |
| `image.rs:1202:40`, `Image::decode` | `>` replaced by `>=` | yes | had stopped finishing tests |
| `image.rs:1205:33`, `Image::decode` | `>` replaced by `>=` | yes | had stopped |
| `image.rs:1229:24`, `Image::decode` | `!` deleted | yes | still finishing tests |
| `image.rs:1229:44`, `Image::decode` | `\|\|` replaced by `&&` | yes | had stopped |
| `image.rs:1229:47`, `Image::decode` | `!` deleted | yes | still finishing |
| `image.rs:1236:31`, `Image::decode` | `>` replaced by `<` | yes | still finishing |
| `image.rs:1252:24`, `Image::decode` | `!` deleted | yes | had stopped |
| `executor.rs:2697:9`, `clauses` | replaced by `Vec::leak(Vec::new())` | yes | had stopped |
| `executor.rs:2725:9`, `untagged` | replaced by `0` | yes | still finishing |
| `executor.rs:2784:9`, `term` | replaced by `Ok(1)` | yes | still finishing |
| `executor.rs:2915:9`, `set_induction` | replaced by `true` | yes | had stopped |
| `executor.rs:2925:9`, `set_affect` | replaced by `true` | yes | had stopped |
| `executor.rs:3040:12`, `ripple` | `!` deleted | yes | still finishing |
| `branching.rs:100:16`, `trace` | `!=` replaced by `==` | yes | still finishing |
| `branching.rs:98:5`, `trace` | replaced by `Ok(vec![])` | no | still finishing, F-42's kind |
| `executor.rs:2891:41`, `search_on_cadence` | `!=` replaced by `==` | no | still finishing, F-42's kind |
| `executor.rs:901:45` | `*` replaced by `+` | no | still finishing, F-42's kind |

- Of the 17, 10 were still finishing tests and 7 had stopped.
- None of the three beside no timeout hung a test of its own module, which makes each F-42's kind.
- **A reading, not a finding.** cargo-mutants sets each shard's bound from its baseline's tests.
  - The runtime's sixteen bounds were 1 674 to 3 318 s, median 2 723, on 2026-10-26, and 1 617 to 2 856 s, median 2 246, here.
  - The runtime's timeouts rose from 21 to 33.
  - The engine served on arrival ([ADR-0225](0225-a-unit-served-on-arrival-built.md)) merged between the two sweeps and may run the baselines faster. A mutant that slows the engine would then meet a lower bound sooner.
  - Whether the 12 more would have ended under the old bounds is not read.
- **There is no survivor**, so the sweep leaves no finding.

## Decision Drivers

- **The repository wins over the document** (principle 1), and a reader must be able to read the repository.
- **Every pinned number stays as it is.** The move changes where a table is and how it is laid out, never a type, a value or a test's name, which the weekly job, `scripts/exhaustive-costs.tsv` and the ADRs cite.
- **F-81 stands**: the tables stay typed `const` items in the test sources.
- **A structural boundary beats a reviewed one** (principle 5): a check holds what a check can.
- **Latest ≠ Newest**: no dependency, no tool and no configuration of rustfmt changes.

## Considered Options

1. **Where the tables live**: (a) a file a round, `tests/<binary>/pins/<round>.rs`; (b) one file a binary, `tests/<binary>/pins.rs`; (c) where they are.
2. **Their layout**: (a) a row a line, with rustfmt kept off the round's files; (b) a wider `max_width` in a `rustfmt.toml` for the workspace; (c) `#[rustfmt::skip]` on each table where it stands.
3. **What holds it**: (a) a check in `npm run spec`, with its tests; (b) review alone.

## Decision Outcome

**Options 1(a), 2(a) and 3(a)**, built in this pull request.

### Where

- **A round's file is named after its round**: `h<N>.rs` for a round that read hypothesis H-*N*, and `brief<NNN>.rs` for one that read none, as briefs 048 to 053 in `tests/assembly.rs`.
- **The binary declares the module once**: `#[rustfmt::skip]`, `#[path = "<binary>/pins/mod.rs"]` and `mod pins;`, then `use pins::*;`.
- **`pins/mod.rs` declares each round's file**, `mod h45;` and `pub(crate) use h45::*;`. A path given to `mod pins` makes its directory, `pins/`, the home of its modules.
- **A round's file** opens with `use crate::*;`, for the types and constants its tables name, and declares each table `pub(crate)`.
- **The tests stay in `tests/<binary>.rs`** under their names, and read the tables as before.
- A name the harness also holds is imported by name, so the binary's own table is the one its tests read: `BACKGROUND_ROWS_1024` in `tests/assembly.rs`.

### The layout

| What | Its line |
| :--- | :--- |
| A row, a tuple or a struct in a list | one line, in Debug's form, up to 1 000 columns |
| Anything that fits in 200 columns | one line |
| A longer list | a row a line; a list of numbers filled to 200 columns |
| A struct wider than 1 000 columns, a reading of tables | a field a line |

- Debug's form is what a DUMP prints: no trailing comma inside a row, `Name { a: 1 }` for a struct.
- Below a table's first line no tuple is broken.
- **rustfmt lays none of it out**: `#[rustfmt::skip]` on `mod pins` keeps `cargo fmt` off `mod.rs` and every round's file. Without it, `cargo fmt --check` reports every round's file, which this pull request read on `tests/assembly.rs`.

### What moved

- **The tables a run printed, over more than one line**: an arm's, a calibration's, an oracle's, or a reading of pinned tables. A verdict over the pinned tables is one.
- **What stays**: the protocol's constants, such as the arms, the schedules, the spans, the settlings and the predictions; and every one-line `const`, 162 of them pins in `tests/inhibition.rs` and 7 in `tests/assembly.rs`.
- **Each table went to the round whose commit wrote most of its lines**, by `git blame` on `main` at `48783c9`. A table read in a protocol's commit, such as `H18_SETTLE_PER_MYRIAD` in H-19's, is that round's.

| Binary | Tables | Rounds' files | Its lines, before and after | The rounds' files' lines |
| :--- | ---: | ---: | ---: | ---: |
| `inhibition` | 554 | 30, `h16.rs` to `h45.rs` | 471 723 to 43 468 | 90 551 |
| `assembly` | 317 | 6, `brief048.rs` to `brief053.rs` | 70 427 to 9 032 | 49 659 |

- **Every token is as it was.** A script out of the tree compared each table's tokens on `main` with its tokens in its round's file. It set aside `pub(crate)` and the trailing commas rustfmt's vertical layout adds. All 871 were equal and none was missing or twice. Each binary's file was `main`'s without the tables and the blank lines beside them, with the module's lines added.
- **14 directives that named a moved table now name its round's file**: one in the whitepaper, F-52's, and 13 in the research reports of H-19 to H-39.

### What holds it

`npm run spec:pins` (`scripts/check-pins.mjs`, with its cases in `spec:pins:test`), for every `tests/<binary>/pins/`:
1. the binary declares the module, `#[rustfmt::skip]` first, and imports it;
2. `mod.rs` declares and re-exports every round's file, `h<N>.rs` or `brief<NNN>.rs`, and no module without one;
3. a round's file holds no function and no test;
4. a round's file breaks no tuple: no line but a table's first ends in `(`;
5. the binary's own file holds no `const` or `static` longer than 24 lines.

That a shorter table is in its round's file, and that each row is one line, is held by review.

### The rounds

- **A round pins its tables in its own file**: H-46's in `tests/inhibition/pins/h46.rs`, declared in `mod.rs`.
- A round's tests and rules stay in the binary's file.
- A DUMP prints a table on one line. The round writes one row a line, by the table above.

## Consequences

- Good: `tests/inhibition.rs` is 43 468 lines and 1.8 MB, its tests and rules beside one another.
- Good: a round's pull request adds a file of its own, one row a line, which a review can read.
- Good: no type, value, test or name changes, and the build is what F-81 read.
- Bad: rustfmt checks none of a round's file, so its layout is the round's and the review's, beyond what `spec:pins` holds.
- Bad: 4 481 lines of the rounds' files are over 200 columns, up to 698.
- Bad: the default `git diff` shows this pull request's two files with lines the move did not add, since many lines repeat. `git diff --diff-algorithm=histogram` shows the 14 lines it added.
- Neutral: F-81 is unchanged.

## Alternatives considered and why rejected

- **Option 1(b), one file a binary**: it is 90 551 lines today and grows by a round's 3 000 to 4 500 at its end, the same burden later.
- **Option 1(c)**: the burden above.
- **Option 2(b), a wider `max_width`**: it moves every line of the workspace's code, and still lays a row out one field a line past its widths.
- **Option 2(c), `#[rustfmt::skip]` in place**: the tables keep two thirds of the file's lines, between the tests.
- **Option 3(b), review alone**: a round's file is pasted from a log, and a check catches a missing declaration or a table left behind.

## Confirmation

- `cargo check`, `cargo test`, `cargo fmt --check` and `cargo clippy` compile and run both binaries over the moved tables.
- `npm run spec` runs `spec:pins` and `spec:pins:test`, and spec-guard runs the 14 retargeted directives.
- The maintainers' dispatch at `scope=exhaustive` on this pull request runs every weekly test that reads a moved table.
