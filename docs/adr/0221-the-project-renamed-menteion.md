---
status: proposed
date: 2026-10-27
title: The project renamed Menteion
kind: decision
summary: The project is Menteion, in a GitHub organisation of its own since 2026-10-26. Brief 083 carries the name into the tree, crates and image format included. What a check freezes, and the history, keep the old name. The schedule's sweep of 2026-10-26 is recorded.
decision-makers: Menteion maintainers
---

# ADR-0221: The project renamed Menteion

**In brief**
- **Decision.** The project is Menteion, at `github.com/Menteion/Menteion` since 2026-10-26. Brief 083 carries the name into the tree: the crates `menteion-*`, the header `MenteionFileHeader`, the magic `MENTEION` at format 26 and the extension `.menteion`. What a check freezes, and the history, keep the old name.
- **Why.** "VirtualCortex" named a technique and one organ, where the engine has thirty-two subsystems. Its dispatches held most of the old organisation's twenty places on GitHub's Free plan, and kept that organisation's other repositories waiting.
- **Left open.** The old address: the frozen files and the history reach it only by GitHub's redirect, which lasts while `DescentVTT` holds no repository named `VirtualCortex`.

## Context and Problem Statement

**The name.**
- VirtualCortex named two things the engine is built from: virtual actors and the cortex. The engine has thirty-two subsystems, one crate each ([ADR-0016](0016-thirty-two-crate-architecture.md)), and most of them name other organs or faculties.
- **Menteion** is the Latin *mens*, *ment-*, the mind, with the Greek *-eion*, the place of a thing, as in *Mouseion*, the museum: the dwelling of the mind.
- Its eight letters fill the magic's eight bytes.
- On 2026-10-26, `menteion` was free as a GitHub organisation and as a crate on crates.io, and a web search found no product of that name. It was not a trademark search.

**The move.**
- GitHub's Free plan runs 20 jobs at once for an organisation, not for a repository.
- A whole calibration takes 18 jobs, and a round's arms one job each ([ADR-0191](0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)). While they ran, the `DescentVTT` organisation's other repositories waited for places.
- The repository is public, so an organisation of its own has its own 20 places at no cost.
- **Transferred on 2026-10-26 at 15:09Z**, from `DescentVTT/VirtualCortex` to `Menteion/Menteion`, by the maintainers' leave.
- The repository's id, its history, its runs and its pull requests moved with it. So did the ruleset "Main Protect", with its four required checks and its signatures. The old address redirects to the new one.

**What names the old name**, re-derived on 2026-10-27 at `cda0e52`:

| Name | Where | Count |
| :--- | :--- | ---: |
| `VirtualCortex` | 289 files, 220 of them ADRs and 10 archived briefs | 556 |
| A crate's directory, `cortex-*` | 32 under `crates/`, `runtime/cortex-runtime`, `benches/cortex-bench` | 34 |
| A crate's Rust path, `cortex_*` | 34 Rust and TOML files | 172 |
| `CortexFileHeader` | the Rust sources | 88 |
| The magic, `VCORTEX1` | `cortex-connectome`, `tests/image.rs`, the whitepaper, ADR-0007 | 11 |
| The address, `DescentVTT/VirtualCortex` | `Cargo.toml`, `README.md`'s badge, two links in `synapse.rs`, four research reports, the history | 279 |
| A directive naming a crate's path | the whitepaper 297, the research reports 266, `README.md` 1 | 564 |

**What a check freezes.**
- An accepted ADR keeps its body byte for byte (`spec:shape`), and an archived brief its hash (`spec:briefs`). An ADR's file keeps its name.
- None of them links into a crate's directory. Ten archived briefs carry directives that name crate paths, and `spec-guard` runs nothing under `briefs/archive/`.
- **Five archived briefs link to whitepaper headings that the rename moves**: §5.2.1, §5.2.4, §5.2.6 and §5.2.9 by their crates' names, §6.7 by `.cortex`. `spec-graph` checks those links, and reads an anchor only from a heading.

**What the shape check reads.**
- A block the base holds passes whatever its length, and a block whose text changed is new text ([ADR-0197](0197-one-place-for-each-kind-of-fact.md)).
- Renamed, 48 blocks of the living documents are over their limits: 46 in the whitepaper, one in H-17's report and one in H-22's.
- [ADR-0201](0201-math-that-github-renders.md) read math moved into GitHub's forms as the text it was.

**The image** (`crates/cortex-connectome/src/lib.rs`).
- `CortexFileHeader::MAGIC` is `*b"VCORTEX1"`, and `validate` refuses any other magic and any other version than `FORMAT_VERSION`, 25.
- No image is committed: the tests write the images they read.
- `CLAUDE.md`: changing any field of any record, including reserved bytes, bumps the version.

**The schedule's sweep of 2026-10-26, recorded here** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).
The schedule's run on `main`, [40292157254](https://github.com/Menteion/Menteion/actions/runs/40292157254) at `df405be`, ended green at 15:06Z. No ADR merged since has recorded it.
- Its eighteen weekly shards took 66.8 to 143.5 minutes against their bound of 240. Shard 12's 143.5 is 59.8 %, half a minute under [ADR-0179](0179-the-bounds-rule-restated.md)'s line.
- Its seventeen sweep jobs started at 10:49Z: the crates' took 29.1 minutes, and the runtime's sixteen 119.6 to 257.7, under their bound of 330.

| Mutants | Count |
| :--- | ---: |
| Listed | 4 106 |
| Tested | 4 106 |
| Caught | 3 918: 2 418 in the state crates, 1 500 in the runtime |
| Missed | 0 |
| Unviable | 161 |
| Timeouts | 27 |

- 22 timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent: `cortex-core`'s three chain iterators, six; the injector, twelve; the barrier, one; the workers' stop and run, three.
- 4 ran beside an injector timeout: two in `Image::decode`, `image.rs:1229:63` and `image.rs:1252:24`, and two in the store, `store.rs:148:9` and `store.rs:259:12`. `store.rs:148:9` was still finishing tests, F-42's kind by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence; the other three had stopped beside the hang.
- 1, `executor.rs:2664:9`, `Executor::compact` replaced by `Ok(Default::default())`, ran beside no timeout. It was still finishing tests, and its one test never reported is not its module's: F-42's kind.
- There is no survivor.

## Decision Drivers

- **The maintainers' choices** (2026-10-26): the name Menteion; the full scope, the crates and the image's format included; an organisation of its own.
- **A rename moves no number.** It changes names, and an image's header. One whole calibration shows every pin held.
- **What a check freezes stays frozen**, and the history says what was true when it was written. Both are read through this record's table.
- **Latest ≠ Newest**: no dependency and no tool changes. The tools under `@descent-vtt` stay at their pins.

## Considered Options

1. **The scope**: (a) the name, the address, the crates, the type, the magic and the extension; (b) the name and the address alone.
2. **The magic**: (a) `MENTEION` at format 26; (b) `VCORTEX1` kept; (c) `MENTEION`, with `VCORTEX1` still read at format 25.
3. **The frozen files and the history**: (a) as they are; (b) the history renamed too.
4. **The five archived links**: (a) each old anchor kept in the whitepaper as an HTML anchor, and named in `.spec-graph.json`'s `ignoreReferences`; (b) the five headings keep the old names.
5. **The shape check**: (a) a block whose only change is this record's table is the base's text; (b) the 48 blocks rewritten under their limits in the rename.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** Brief 083 carries them into the tree.

### The names (Specified; brief 083 writes them)

| Old | New |
| :--- | :--- |
| VirtualCortex, the project | Menteion |
| `DescentVTT/VirtualCortex`, the address | `Menteion/Menteion` |
| `cortex-<name>`, a crate and its directory | `menteion-<name>` |
| `cortex_<name>`, its Rust path | `menteion_<name>` |
| `CortexFileHeader` | `MenteionFileHeader` |
| `VCORTEX1`, the magic | `MENTEION` |
| `.cortex`, the image's extension | `.menteion` |
| `virtualcortex-specs`, the npm package | `menteion-specs` |
| VirtualCortex maintainers | Menteion maintainers |

- **The word *cortex* where it names the anatomy stays**, and so do the names of the ADRs' files.
- **A crate's name moves everywhere at once**: its `Cargo.toml`, its directory, the workspace's members, `Cargo.lock`, the workflow's `-p` and `--exclude`, `.cargo/mutants.toml`'s exclusions and `scripts/check-deps.mjs`.
- **The required checks keep their names**, and so does every test.
- **The tools under `@descent-vtt` stay at their pins**: they are the old organisation's, admitted by [ADR-0185](0185-the-spec-tools-upgraded-and-spec-brief.md).

### The image

- **The magic is `MENTEION`**, big-endian `0x4D45_4E54_4549_4F4E`, and the format is 26.
- An image of format 25 is refused, as one of format 24 is today.
- **Predicted: no pinned number moves.** In an image, only the magic, the version and the header's CRC change. The whole calibration shows it.

### What keeps the old name

- **Frozen by a check**: every accepted ADR's body and every archived brief.
- **The history**: `CHANGELOG.md`'s entries before brief 083's, the exploration notes, which describe their kept branches, and the benchmark results under `docs/benchmarks/results/`. None of them holds a directive or a link into a crate.
- In both, a link to the old address resolves by GitHub's redirect.
- **Everything else takes the new names**: the code, the workspace, the workflow, the scripts and their tests, the whitepaper, the research reports, the live briefs, `docs/STATUS.md`, the reader's guide, `docs/benchmarks/README.md`, `README.md`, `CLAUDE.md`, `CONTRIBUTING.md` and `SECURITY.md`.

### The five archived links

- Each of the five headings takes its new name, with an HTML anchor of its old one before it, so a link from an archived brief lands on its section.
- `.spec-graph.json`'s `ignoreReferences` names the five old anchors. `spec-graph` reads anchors from headings alone, and these are never headings again.

### The shape check

- `scripts/check-doc-shape.mjs` reads a block whose only change is this record's table, read back from the new names to the old, as the base's text, as ADR-0201 reads moved math.
- Its cases are in `spec:shape:test`.

### The round

- **Brief 083 renames the tree in one pull request.** Its change does not only add, and it has no test of its own.
- **One dispatch at `scope=exhaustive`, every test**, on its last commit under `crates/`, `runtime/`, `benches/` or `testkit/`, or to a Cargo file. After it, none of them changes.
- No arm and no reproduction: there is no table to pin.
- **Brief 065 stays live and paused**, its paths renamed.

## Consequences

- Good: the name says what the engine is for, and the project's runs take places no other repository needs.
- Good: a reader of a frozen file finds each old name in this record's table.
- Bad: every path in the tree moves at once. A branch cut before it, such as a kept exploration's, merges only with conflicts.
- Bad: the frozen files and the history name paths that no longer exist.
- Bad: an image of format 25 is refused.
- Neutral: no record's layout and no rule changes; the format moves for the magic alone.

## Alternatives considered and why rejected

- **Option 1(b), the name alone**: every path a reader opens, and every image, would name a project that is no longer there.
- **Option 2(b), `VCORTEX1` kept**: every image would carry the old name in its first eight bytes.
- **Option 2(c), both magics read**: no image of format 25 exists outside the tests that write it, and a second magic is a branch in `validate` that no image needs.
- **Option 3(b), the history renamed**: an entry would name a crate before it had that name, and an exploration note would no longer match its kept branch.
- **Option 4(b), the headings kept**: five of the whitepaper's headings would name crates that no longer exist.
- **Option 5(b), the 48 blocks rewritten**: the rename would become an edit of the whitepaper's text, and the review could not tell the two apart.

## Confirmation

- `cargo test --workspace --locked` and the pull request's gate on brief 083's last commit.
- `npm run spec`, with the five old anchors ignored and the shape check's new cases.
- The dispatch at `scope=exhaustive` on the round's last code commit, every shard green: every pinned number held.
- `git grep` for each old name finds it only in the frozen files, the history, this record and the five anchors.
