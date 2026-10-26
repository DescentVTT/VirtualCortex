---
status: archived
date: 2026-10-27
affectedFiles: [crates/**, runtime/**, benches/**, testkit/**, Cargo.toml, Cargo.lock, .cargo/**, .github/workflows/**, scripts/check-deps.mjs, scripts/check-doc-shape.mjs, scripts/*.test.mjs, scripts/exhaustive-costs.tsv, package.json, package-lock.json, .spec-graph.json, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, CONTRIBUTING.md, README.md, SECURITY.md]
protectedFiles: [docs/explorations/0*.md, docs/benchmarks/results/**, rust-toolchain.toml, clippy.toml, .spec-brief.json, LICENSE-APACHE, LICENSE-MIT, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/adr-index.mjs, scripts/read-timeouts.mjs]
integrity: sha256-67f8054746c7c60c19fc46ca3bcdd1a16944bfeed84ed8f65960e3094ed16a6e
---

<!-- spec-brief:banner -->
> **Executed 2026-10-26 (UTC) in pull request [#246](https://github.com/Menteion/Menteion/pull/246).** ADR-0222 built ADR-0221's rename: the 34 crates moved to menteion-*, MenteionFileHeader, the magic MENTEION at format 26 with no reader of 25, and every living document, the five old anchors kept and the shape check reading the rename back. The tree's whole-image pins are read with their headers written back to format 25, so no pinned number moved; F-83 is resolved and F-84, a stale cell of §5.2.2, opened. One dispatch at scope=exhaustive on 0216043, run 40371343923, ran all 219 whole-domain tests green, the longest shard at 52 % of its bound. The body below describes the tree before execution and is not maintained.
<!-- /spec-brief:banner -->

# Brief 083: The project renamed Menteion

<!-- @assert-count target="Cargo.toml" symbol="runtime/cortex-runtime" min="1" reason="brief 083 precondition: the crates still carry the old prefix; the round that renames them archives this brief in the same pull request" -->

## Mission

**This brief renames the tree and runs every test once.**
[ADR-0221](../../docs/adr/0221-the-project-renamed-menteion.md) renamed the project Menteion. The repository moved to `Menteion/Menteion` on 2026-10-26; the tree still says VirtualCortex.

When the round is done, the tree holds:
- the names of ADR-0221's table in the code, the workspace, the workflow, the scripts and every living document;
- the image's magic `MENTEION` at format 26, and no other byte of an image changed;
- the old names only in the files a check freezes, in the history, in ADR-0221 and in five HTML anchors;
- one dispatch at `scope=exhaustive` on the round's last code commit, every shard green, every pinned number held.

It writes a built ADR and rewrites `docs/STATUS.md` ([briefs/README.md](../README.md#a-rounds-documents)).

The scope is the front matter's ([ADR-0191](../../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): `spec-brief archive --strict` refuses the round outside it.

**Do not run this round beside another round's**: every path in the tree moves.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version of either. The tools under `@descent-vtt` stay at their pins, under that name.
- **The names are ADR-0221's table**, and nothing else is renamed.
  - The word *cortex* where it names the anatomy stays: cortical, subcortical, the cerebral cortex.
  - The ADRs' files keep their names; so do the tests, the required checks and the workflow's jobs.
- **What a check freezes is not touched**: an accepted ADR's body and front matter, and an archived brief.
- **The history keeps what it said**: `CHANGELOG.md`'s entries before this round's, the exploration notes and `docs/benchmarks/results/`. The last two are protected.
- **The magic is `MENTEION` and the format 26**, ADR-0221's. A reader of format 25 is not kept.
- **No number moves.** No record's layout, no rule, no constant and no test's assertion changes beyond the names, the magic and the version.
- **The run is one dispatch** ([ADR-0215](../../docs/adr/0215-the-calibration-a-change-needs.md), [ADR-0211](../../docs/adr/0211-the-round-shortened.md)).
  - The round runs `npm run round:additive -- origin/main <its last code commit>` and quotes it in its ADR.
  - It dispatches `scope=exhaustive` with every test, on its last commit that changes a file under `crates/`, `runtime/`, `benches/`, `testkit/` or `.cargo/`, or a Cargo file. After that commit, none of them changes.
  - No arm, no reproduction and no sweep. No whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-083-…`**, never rebased or force-pushed once a dispatch has run on it. Every commit is signed.
- **Directories move by `git mv`**, so the history follows each file.
- **The engine and the documents are read before a description of them is trusted**, this brief's and ADR-0221's included.
- No `f32`/`f64`; every loop ends by construction ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)); no `unsafe` outside the runtime's one, under ADR-0023.
- The mutation gate catches every mutant in the lines the change touches ([ADR-0030](../../docs/adr/0030-verification-governance.md)).
- **The documents are ADR-0197's and ADR-0201's**: the new ADR in its shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits.
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADR to `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 240 minutes ([ADR-0179](../../docs/adr/0179-the-bounds-rule-restated.md)). One that does is reported with its job's minutes.
- **The maintainer is notified** as `briefs/README.md` says: when a question only they can answer stops the round, and when the pull request is ready for review.
- Conventional Commits with a real body; never commit on `main`.

## Context

Re-derived on 2026-10-27 against `main` at `cda0e52`. Line numbers move; the symbols and the quoted sentences are what to re-derive, by `git grep` for each old name.

1. **The names**, ADR-0221's table and counts: `VirtualCortex` 556 times in 289 files, 34 crate directories, 172 Rust paths `cortex_*`, `CortexFileHeader` 88 times, `VCORTEX1` 11, the old address 279, and 564 directives naming a crate's path.
2. **The workspace** (`Cargo.toml`): the members, `[workspace.package]`'s `authors` and `repository`, each crate's `description`, the runtime's and the bench's path dependencies, `Cargo.lock`.
3. **What names a crate outside Rust**:
   - `.github/workflows/ci.yml`: its first comment, `cargo bench -p cortex-bench`, the sweep's `--exclude cortex-runtime --exclude cortex-bench` and sixteen `-p cortex-runtime`.
   - `.cargo/mutants.toml`: three exclusions by a crate's path. Left as they are, they match nothing, and the sweep tests mutants the tree reads as equivalent.
   - `scripts/check-deps.mjs`: the runtime's and the bench's manifests, and the pattern `^cortex-`.
   - `scripts/exhaustive-costs.tsv`: one line, the binary `cortex_core`. `npm run spec:costs` holds each line to a test in the tree.
   - Five script tests hold fixtures with the old names; `package.json` and `package-lock.json` name `virtualcortex-specs`.
4. **The image** (`crates/cortex-connectome/src/lib.rs`): `CortexFileHeader::MAGIC`, `FORMAT_VERSION` 25, the tests of the magic's big-endian value and its CRC; `runtime/cortex-runtime/tests/image.rs`'s first eight bytes; the whitepaper's §5.2.2 table and its list of the format's versions; `.cortex` in the documents and `SECURITY.md`.
5. **The documents' checks**:
   - `spec-guard` runs 297 directives in the whitepaper, 266 in the research reports and one in `README.md` that name a crate's path.
   - `spec:shape` reads 48 renamed blocks as over their limits: 46 in the whitepaper, one each in H-17's and H-22's reports. Its comparison is `normalise` in `scripts/check-doc-shape.mjs`.
   - `spec-graph` checks five archived briefs' links to headings the rename moves: `#521-cortex-core--neural-state-and-dispatch`, `#524-cortex-embodiment--sensorimotor-loop`, `#526-cortex-cerebellum--forward-models`, `#529-cortex-symbolic--vector-symbolic-bridge` and `#67-scenario-r-7-cold-boot-from-a-cortex-image`. It reads anchors from headings alone, and `.spec-graph.json`'s `ignoreReferences` silences a target that never returns.
   - The whitepaper links three of its own §5.2 anchors from its table of crates.
6. **What keeps the old name**: every accepted ADR's body and every archived brief, held by `spec:shape` and `spec:briefs`; the history, `spec-graph`'s `historyPatterns` among it. None of the history holds a directive or a link into a crate.
7. **Brief 065**, live and paused, names `cortex-runtime` four times. It has no scope of its own.
8. **The mutation gate** reads `git diff origin/main...HEAD`. Git reads a moved file whose lines mostly stay as a rename, and the 129 files under `crates/`, `runtime/`, `benches/` and `testkit/` are within its limit for that. `Image::encode` and `Image::decode` name the header's type. The round lists the gate's mutants with `cargo mutants --list --in-diff` on that diff before it pushes.
9. **The process** ([briefs/README.md](../README.md#evidencing-a-round)): one dispatch at `scope=exhaustive`; the cost table regenerated from its artifacts with `--keep`; `spec-brief archive <NNN> --strict` measured from `origin/main`.
10. **The sweep of 2026-10-26** is recorded by ADR-0221. The schedule runs every Monday at 03:00Z and holds most of the organisation's twenty places for hours; a dispatch made then waits.

## Deliverables

- [x] **The rename, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - The crates, their directories, the workspace, `Cargo.lock`, the workflow, `.cargo/mutants.toml`, the scripts and their tests, `package.json` and its lock.
  - `MenteionFileHeader`, the magic `MENTEION`, the format 26, its tests and the whitepaper's §5.2.2 with the format's new version in its list.
  - What `git grep` finds of each old name after the round, file by file, each one frozen, history, ADR-0221 or an anchor.
- [x] **The documents' checks.**
  - Every directive naming a crate's path renamed, and every one of them run by `npm run spec`.
  - `scripts/check-doc-shape.mjs` reading a block whose only change is ADR-0221's table as the base's text, with cases in `spec:shape:test`.
  - The five old anchors kept as HTML anchors before their headings, and named in `.spec-graph.json`'s `ignoreReferences`.
- [x] **The dispatch at `scope=exhaustive`**, every test, on the round's last code commit, every shard green: every pinned number held. The cost table regenerated from its artifacts.
- [x] **The documents, in the same pull request.**
  - The living documents renamed, as ADR-0221 lists them; brief 065's paths among them.
  - `docs/STATUS.md` rewritten whole.
  - `CHANGELOG.md`, one entry of at most 400 characters; the earlier entries as they are.
  - The whitepaper's version in both declarations, with its date; the tables written by `node scripts/adr-index.mjs`.
- [x] **The brief archived** with `spec-brief archive 083 --strict`, every deliverable dispositioned. The round's ADR is set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No accepted ADR and no archived brief changes**, not even a link.
- **The history is not renamed**, and its links to the old address stay.
- **No name outside ADR-0221's table changes**: not a test, a check, a job, a type other than the header's, or a file of an ADR.
- **No number moves.** A pinned number that moves stops the round as a finding: the rename is not a change to the engine.
- No dependency, no tool version, no toolchain, no MSRV and no edition changes.
- No whole-domain test runs on the developer machine, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- how `normalise` reads ADR-0221's table back, so long as a block with any other change stays new text;
- the form of the five HTML anchors and how `ignoreReferences` names them, so long as each archived link lands on its section on GitHub;
- the order of the commits, so long as each builds and the dispatch runs on the last code commit;
- how the whitepaper's prose names the project where a sentence reads wrongly renamed word for word;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a frozen file, the history, a test's assertion other than a name, the magic or the version;
- keep a reader of format 25;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the dispatch moves a pinned number, the round stops there: the finding is written, and nothing is pinned anew.

## Verification

Before each push, on the developer machine:

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
npm ci
npm run spec
```

The pull request's gate runs those and the rest: the release tests, the benchmarks once, the MSRV, AArch64 and the mutation gate on the changed lines. Every check passes on the last commit.

By dispatch, never on the developer machine:

```bash
npm run round:additive -- origin/main <the last code commit>
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive
node scripts/exhaustive-costs.mjs from <its downloaded artifacts> --run '<the dispatch>' --keep scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @descent-vtt/spec-brief archive 083 --strict --pr <n> --summary "<one paragraph>"
```

- `round:additive` exits 1, and the ADR quotes it.
- The dispatch's eighteen shards are green, every test run, on the last code commit.
- `git grep -n -e VirtualCortex -e 'cortex-[a-z]' -e 'cortex_[a-z]' -e CortexFileHeader -e VCORTEX1 -e '\.cortex\b'` finds each old name only where ADR-0221 keeps it.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the rename as built, and what `git grep` still finds of each old name, by kind;
- the magic and the format, and that no other byte of an image changed;
- how the shape check reads the rename, and how the five anchors are kept;
- the dispatch with its run id and commit, each shard's job time against its bound, and that every pinned number held;
- what `round:additive` read;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
