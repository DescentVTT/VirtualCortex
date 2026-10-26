---
status: accepted
date: 2026-10-27
depends-on: ADR-0221
title: The project renamed Menteion, built
kind: built
summary: ADR-0221's names are in the code, the workspace, the workflow, the scripts and the living documents, and the magic is `MENTEION` at format 26. The whole-image pins are read at format 25, so none moves. One dispatch at scope=exhaustive held every pin. F-83 is resolved and F-84 opened.
decision-makers: Menteion maintainers
---

# ADR-0222: The project renamed Menteion, built

**In brief**
- **Decision.** Brief 083 carries [ADR-0221](0221-the-project-renamed-menteion.md)'s table into the tree. The 34 crate directories move by `git mv`. The header is `MenteionFileHeader`, its magic `MENTEION` at format 26, and every living document takes the names. The tree's whole-image pins are read with their headers written back to format 25, so none of them moves.
- **Why.** ADR-0221 predicted that no pinned number would move, since an image changes only in its header. But the tree pins whole images by their CRC-64, the header included, and the version alone moves every one of them (F-83).
- **Left open.** §5.2.2's Public API cell still says format 21 (F-84). The old address in the frozen files and the history, as ADR-0221 left it.

## Context and Problem Statement

What was read first (principle 2), on 2026-10-27 at `main` `d29760e`:
1. **The names**, ADR-0221's table and counts. Brief 083 carries them into the tree and names what keeps the old ones.
2. **The image** (`crates/cortex-connectome/src/lib.rs`). `validate` checks the magic before the version, so an image whose magic is not this build's is refused before its version is read. No image is committed: the tests write the images they read.
3. **The whole-image pins.** `tests/inhibition.rs` pins five images by their CRC-64 at the current format: `REVERSAL_`, `PUNISHED_`, `TARGET_`, `VALUED_` and `WINDOWED_IMAGE_CRC_1024`. `tests/assembly.rs` pins two more, `H20_IMAGE_CRC` and the drained image's CRC in `DRAINED_050`. The derivations' `crcs` are held at the current format to `PROBE_CRCS` and to the five.
4. **How a format moved them.** A CRC over a whole image covers its header. [ADR-0095](0095-an-image-pin-moves-with-its-format.md) decided that such a pin *"is re-pinned in the round that moves the format, the rest of the image held to the pin before it"*, and each format since did so, the old value kept in the comment. `with_version` writes a header's version back and reseals it, which holds each round's own read at its format (`_FORMAT_N`). [ADR-0202](0202-a-step-the-engine-makes-from-its-own-loop-built.md) held H-41's derivations at 24 that way (`BOTH_TARGETS_FORMAT`) instead of re-pinning them.
5. **So the version alone, 25 to 26, moves every one of them**, as every bump did. And `with_version` keeps the magic, so an image written back to a format before 26 would carry `MENTEION`, which no such format had: every `_FORMAT_N` pin would move too.
6. **Brief 083**: no number moves, and nothing is pinned anew. ADR-0221's prediction and the tree disagree; the tree wins, and the disagreement is F-83 (principle 1).

## Decision Drivers

- ADR-0221's decision and brief 083's directives: the table's names alone, `MENTEION` at format 26, no reader of 25, no number moved, one dispatch.
- **What a check freezes, and the history, untouched.**
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)).

## Considered Options

1. **The whole-image pins**: (a) each read at format 25, through `with_version`, which writes `VCORTEX1` back below 26, as ADR-0202 read H-41's at 24; (b) each re-pinned at 26 by ADR-0095's rule, from the CRC's linearity as ADR-0202 re-pinned them by a mask, its value at 25 kept in its comment.
2. **The shape check**: (a) `normalise` reads ADR-0221's table back, the new names to the old; (b) the 44 blocks the rename puts over their limits named in a list.
3. **The five archived links**: (a) an HTML anchor of each old id before its heading, a blank line between, and the five links named in `ignoreReferences`; (b) the old ids as `name` attributes.
4. **Two names beside the table**: the worker thread's `cortex-worker-N` and the image tests' `vcortex-*-tests` scratch directories: (a) renamed, since each prefix is the project's name; (b) kept.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**
Built in `0216043`, `09552da` and `2d9f10e`, in brief 083's pull request.
Brief 083's empowerment covers 2 and 3. Option 1(a) changes no assertion but by the magic and the version, and option 4(a) is ADR-0221's first row read where the project's name is a prefix.

- *Why not 1(b)*: it would move seven pinned numbers, where brief 083 says none moves and none is pinned anew. Under 1(a) each pin holds as it is, and holds what ADR-0221 predicted: an image differs in its magic, its version and its seal alone.
- **ADR-0095's rule is not followed for these seven from here**: they hold their images at 25, and a later format need not re-pin them. ADR-0095 stays as it is, as brief 083 asks of every accepted ADR. The header's own bytes are held by the image tests instead.
- *Why not 2(b)*: a list is the rename restated, and a block changed beyond the table would pass with it.
- *Why not 3(b)*: `name` on an anchor is obsolete in HTML, and GitHub keeps `id` as it keeps `name`.
- *Why not 4(b)*: both names say only the old project's name. They change no number: no test reads the thread's name, and a scratch directory's name is a path.

### The names (Implemented)

| What | As built |
| :--- | :--- |
| The crates | `crates/menteion-*`, `runtime/menteion-runtime`, `benches/menteion-bench`, 128 files moved by `git mv`; every manifest, Rust path and description |
| The workspace | `authors` the Menteion maintainers, `repository` `https://github.com/Menteion/Menteion`; `Cargo.lock` the same packages at the same versions and checksums, sorted again by cargo |
| The workflow | its first comment, `cargo bench -p menteion-bench`, the sweep's `--exclude menteion-runtime --exclude menteion-bench` and sixteen `-p menteion-runtime` |
| The scripts | `.cargo/mutants.toml`'s three exclusions, `scripts/check-deps.mjs`'s manifests and pattern, the cost table's crate binary, five tests' fixtures; the npm package `menteion-specs` |
| Beside the table | the worker thread `menteion-worker-N`; the image tests' `menteion-image-tests` and `menteion-amendment-tests` |
| The documents | every living document ADR-0221 lists, brief 065's paths among them, and every directive that names a crate's path |

Kept, as the table leaves them: the anatomy's *cortex*, the ADRs' files, every test's name, the required checks and the jobs. The fabric packet's magic `VCFB` is not in the table and stays.

### The image (Implemented)

- **The magic is `MENTEION`**, big-endian `0x4D45_4E54_4549_4F4E`, and `FORMAT_VERSION` is 26, its line in the format's versions in the code and in whitepaper §5.2.2.
- **A version-25 image is refused by its magic**, before its version is read. No reader of 25 is kept.
- **No record moved**, and no byte of an image but the header's magic, version and seal. A probe not kept wrote two images with `d29760e`'s writer and with this one's, written back to 25: `tests/image.rs`'s wired network, 13 312 bytes, CRC `0x583727662ca214af` both, and one of two units, 1 024 bytes, `0x4d19c6cab1e83299` both.
- Tests: the magic's value and format 26, and a format-25 header refused by its magic, in the connectome; in `tests/image.rs`, an image of format 26 that is one of 25 but for the magic, the version and the seal, refused by its magic.

### The whole-image pins (Implemented)

- **`with_version(image, v)`** writes the magic back to `VCORTEX1` below 26, in `tests/inhibition.rs` and in `tests/assembly.rs`'s copy. Every `_FORMAT_N` pin then holds as it did.
- **`PINNED_FORMAT`, 25, and `pinned_crc`** read an image's CRC-64 with its header written back to 25. Each comparison with a current-format pin goes through it: 33 in `tests/inhibition.rs`, the derivations' `crcs` among them, and 2 in `tests/assembly.rs`.
- Each of the seven pins' comments says it holds the image at 25 since this ADR. `with_version`'s own test on a small image reads the magic, the version and the seal apart, and nothing else.

### The documents' checks (Implemented)

- **Every directive that names a crate's path names the new one**, and `npm run spec` runs every one. One more holds the magic in §5.2.2.
- **`renamedBack`** in `scripts/check-doc-shape.mjs` reads the table back inside `normalise`, the longer names first. A block whose only change is the rename is the base's text; with one more word changed it is new. Without it 44 blocks are over their limits: 42 in the whitepaper, one in H-17's report and one in H-22's, where ADR-0221 counted 48. Two cases in `spec:shape:test`.
- **The five anchors**: `<a id="521-cortex-core--neural-state-and-dispatch"></a>` and its four siblings, each before its heading. The five archived links are in `.spec-graph.json`'s `ignoreReferences`, each by its exact text.
- The whitepaper's table of crates links §5.2.1, §5.2.16 and §5.2.30 by their new anchors. Its two diagrams that name the project keep their columns.

### What `git grep` finds of each old name

| Kind | Files | What |
| :--- | ---: | :--- |
| Frozen by a check | 302 | 220 accepted ADRs, their bodies and front matter, and 82 archived briefs, brief 083 among them |
| ADR-0221 and this record | 2 | the table, and what this record says of the tree before it |
| Written from frozen front matter | 2 | the ADR index, and §9's rows for ADR-0007, ADR-0024 and ADR-0025 in the whitepaper |
| The ADRs' files | 1 | the whitepaper's links to `0007-cortex-image-format.md` and `0024-cortex-image-and-clock-sweep.md` |
| The history | 12 | `CHANGELOG.md`'s entries before brief 083's, five exploration notes, six benchmark results |
| The five anchors | 2 | the whitepaper and `.spec-graph.json` |
| The magic before 26 | 5 | `VCORTEX1` in the connectome's test, three runtime tests and the whitepaper's §5.2.2 |
| The read-back | 2 | `scripts/check-doc-shape.mjs` and its test |
| Protected | 1 | `LICENSE-MIT`'s copyright line, which brief 083 protects |

### The run

- **What the change needs.** `npm run round:additive -- origin/main 0216043` exits 1: *"d29760e...0216043 does not only add, 132 reason(s): the calibration runs every test (ADR-0215)"*. So the run is every test, with no `exclude` and no `only`.
- **The dispatch.** Run [40371343923](https://github.com/Menteion/Menteion/actions/runs/40371343923) at `scope=exhaustive` on `0216043`, at 17:43:52Z on 2026-10-26. That is the round's last commit under `crates/`, `runtime/`, `benches/`, `testkit/` or `.cargo/`, or to a Cargo file. None of them changes after it.
- **Every shard is green, and every pinned number held** on x86-64. All 219 whole-domain tests passed, each exit 0 with one test reported, none excluded, 229 444 s summed.

| Shard | Job | Of its bound | Tests | Their seconds summed | Tests' wall time |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 121.8 min | 51 % | 10 | 14 173 | 7 201 s |
| 1 | 122.5 min | 51 % | 12 | 14 101 | 7 251 s |
| 2 | 120.1 min | 50 % | 10 | 13 635 | 7 098 s |
| 3 | 75.7 min | 32 % | 10 | 8 902 | 4 466 s |
| 4 | 89.2 min | 37 % | 13 | 10 055 | 5 268 s |
| 5 | 125.7 min | 52 % | 13 | 13 642 | 7 447 s |
| 6 | 94.5 min | 39 % | 12 | 10 978 | 5 596 s |
| 7 | 119.1 min | 50 % | 13 | 13 589 | 7 041 s |
| 8 | 125.3 min | 52 % | 13 | 13 845 | 7 425 s |
| 9 | 119.4 min | 50 % | 12 | 13 675 | 7 061 s |
| 10 | 112.7 min | 47 % | 13 | 13 298 | 6 670 s |
| 11 | 117.8 min | 49 % | 12 | 13 441 | 6 972 s |
| 12 | 90.4 min | 38 % | 13 | 10 657 | 5 330 s |
| 13 | 121.8 min | 51 % | 11 | 14 096 | 7 221 s |
| 14 | 122.7 min | 51 % | 13 | 14 155 | 7 273 s |
| 15 | 112.3 min | 47 % | 13 | 12 875 | 6 644 s |
| 16 | 114.0 min | 48 % | 13 | 13 343 | 6 741 s |
| 17 | 95.3 min | 40 % | 13 | 10 984 | 5 642 s |

- The bound is 240 minutes a job ([ADR-0220](0220-the-cost-of-a-test-run-alone.md)). **No job passed 60 % of it**: the longest, shard 5's, took 125.7 minutes, 52 %.
- **The organisation ran six of the shards at once at first and never more than ten**, where GitHub's Free plan gives 20 jobs at once. The others waited for a place, so the last shard ended at 23:05:56Z, 5 h 22 min after the dispatch. What holds the organisation's places is not read here.
- **The cost table** is regenerated from this run's artifacts with `--keep`: 219 lines, 229 444 s, where it held 244 578 s.
- **The pull request's gate** on `9195f31`, run 40373092433: the Rust checks, the minimum version, AArch64 and the mutation gate green. The gate found 7 mutants, 3 caught and 4 unviable, no survivor. The documentation check failed on brief 083's precondition alone, until the archive.
- **The sweep.** The last one, the schedule's of 2026-10-26, is recorded by ADR-0221, and none has run since. This round dispatches none ([ADR-0211](0211-the-round-shortened.md)).

## Consequences

- Good: the tree says Menteion, and an image carries the name in its first eight bytes.
- Good: every pinned number held as it was, and the whole-image pins now read an image's records, not its header. A later format moves none of them.
- Bad: a reader of the pins must know that 25 is the format they were last taken at.
- Bad: brief 065's branch, and every kept exploration branch, merges only with conflicts, as ADR-0221 said.
- Neutral: `VCORTEX1` stays in the tests that write a format-25 header back.

## Confirmation

- `cargo test --workspace --locked` and the pull request's gate on the round's last commit.
- `npm run spec`, with the five anchors ignored and the shape check's new cases.
- The dispatch at `scope=exhaustive` on `0216043`, every shard green.
- `git grep` for each old name, read by kind above.
