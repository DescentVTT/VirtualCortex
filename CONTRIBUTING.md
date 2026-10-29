# Contributing to Menteion

Thank you for considering a contribution. This file is the map; the reasoning behind each rule lives in the [whitepaper](docs/WHITEPAPER.md) and in the [ADRs](docs/adr/README.md).

## Before you start

1. Read the whitepaper's [How to read this document](docs/WHITEPAPER.md#how-to-read-this-document) section. The four status labels (Implemented, Specified, Target, Hypothesis) govern everything written here.
2. Check [§11 Risks and technical debt](docs/WHITEPAPER.md#11-risks-and-technical-debt). Many good first contributions are already numbered there.
3. Anything that changes an architectural rule needs an ADR first (below).

## Workflow

1. Fork and branch from `main`. Branch names are free-form; commit messages are not.
2. Make the change, including its documentation, in the same pull request. A layout change without its whitepaper table and changelog entry is incomplete.
3. Run the first block of the verification locally (below). The pull request's gate runs it and the rest ([ADR-0184](docs/adr/0184-the-runs-off-the-developer-machine.md)).
4. Open a pull request. Describe what changed, which finding or milestone it addresses, and how you verified it.
5. `main` takes it by a merge commit, the only method the ruleset allows. To bring the branch up to date, merge `main` into it; do not rebase or force-push it once its pull request is open, so that the commits it cites stay in the history ([ADR-0186](docs/adr/0186-merge-commits.md)).

### Commit messages

[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/): `type(scope): summary`, where `type` is one of `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci` and `scope` is a crate name without the `menteion-` prefix (`core`, `hippocampus`), or `whitepaper`, `adr`, `briefs`, `workspace`, `specs`, `ci`.

```text
docs(whitepaper): transcribe SynapseBlock layout from source
feat(core): add saturating Q16.16 helpers
fix(cerebellum): compare prediction with delayed observation (F-8)
```

## Code rules

These are the technical constraints of whitepaper §2.2. They are checked where a tool can check them.

| Rule | How it is checked |
| :--- | :--- |
| Stable Rust, edition 2024, minimum supported version 1.85 ([ADR-0009](docs/adr/0009-rust-edition-and-msrv.md)); no nightly features. | `rust-toolchain.toml` pins the toolchain CI and contributors build with; a second CI job builds and tests on the `rust-version` in `Cargo.toml`; `spec-guard` holds the edition, the floor and the thirty-two inheriting manifests. |
| State crates declare no dependencies. The benchmark crate may carry the harness as a dev-dependency and nothing else ([ADR-0014](docs/adr/0014-benchmark-harness.md)). | `scripts/check-deps.mjs` (`npm run spec:deps`, CI, blocking; [ADR-0029](docs/adr/0029-structural-enforcement.md)); `cargo tree -e normal -p <crate>` to look. |
| A new state crate passes the admission test of [ADR-0016](docs/adr/0016-thirty-two-crate-architecture.md): an ADR names the gap, no existing record owns the quantity, the mechanism is in whitepaper §8.8 with the layout, every Q-format fits its width, and §1.5 and §8.10 are intact or moved by that ADR first. | `spec-guard` holds the member count at 32 in the whitepaper and the README; the pull request that adds a crate moves them and carries the ADR that admits it. |
| Performance figures are Measured only from an admissible run recorded under `docs/benchmarks/results/` ([ADR-0010](docs/adr/0010-measured-or-target.md)). | Review; the results file's `admissible:` line. |
| Every public function and associated constant has at least one unit test, and every state crate carries a `#[cfg(test)]` module. | The module: `spec-guard`, one directive per crate in whitepaper §1.6. The per-item rule: review; no tool checks it yet, so a PR that adds a public item without a test is rejected on review. |
| Every primary record is `#[repr(C)]`; arena records are `align(64)` and exactly 64 bytes; size and alignment are asserted in a `const _: () = { ... }` block. | `cargo check` fails otherwise. |
| No `f32` or `f64` anywhere in the workspace. Use Q16.16 (whitepaper §8.1). | `clippy::disallowed_types` under `[workspace.lints]`, a build error (ADR-0029); `spec-guard` directives in the whitepaper. |
| Every state crate is `#![no_std]`; no `Box`, `Vec`, `String` or thread spawning in state crates. | `spec-guard` directives (the `no_std` count is asserted at 32). |
| A record without atomics derives `Clone, Copy, Debug, PartialEq, Eq`; a control record derives `Debug` only. | Review; whitepaper §8.2 rule L-5. |
| No `unsafe` without an ADR naming the invariant and the test. | `unsafe_code = "forbid"` under `[workspace.lints]` in every state crate and the benchmark crate (ADR-0029); `spec-guard` directives for the runtime's one `unsafe`; review. |
| Arithmetic on Q16.16 state fields is saturating, or explicitly wrapping for phase counters; no plain `+`, `-`, `*`, `/` or `%` on any integer anywhere in the workspace. | `#![deny(clippy::arithmetic_side_effects)]` in every crate and in every integration-test crate root ([ADR-0029](docs/adr/0029-structural-enforcement.md), brief 016; finding F-4 resolved): a build error. A site that cannot be written by name is a finding, not an exemption; `#[allow]` is not used. |
| A parameter the engine may amend by itself is an entry of `menteion-executive`'s `REGISTRY` with an owner and bounds, added by an ADR; the veto gate's parameters never are; the amendment passes the four gates of `PolicyAmendment` and its trial ([ADR-0031](docs/adr/0031-policy-amendment.md)). | The registry test in `menteion-executive`; `spec-guard` holds `OWNER_VETO_GATE` and the record; the loader's `is_well_formed`; review of the ADR that adds an entry. |
| Every loop ends by construction (a `for` over a range or a slice, a countdown by one tested for zero, a scan by `get`, a recursion depth that falls to a bound), never by an ordering comparison alone on a value the body moves; a test's loop never ends by the function it tests ([ADR-0062](docs/adr/0062-the-first-complete-sweeps-list.md)). | The weekly sweep's `timeout.txt`, read by ADR-0062's triage rule: a directive-class timeout is rewritten as a range and held to its previous form by a property test in `mod prop`; an inherent one (a wait on another thread, a bound the mutant removed) is a detection and stays; review. |
| Trailing padding is an explicit `_reserved` or `padding` byte array. | Review. |
| Changing any field of any record, including reserved bytes, bumps `MenteionFileHeader::version` and gets a changelog entry. | Review. |

Formatting (`cargo fmt`) and lints (`cargo clippy -D warnings`) run in CI as blocking checks. Run them before pushing.

## Documentation rules

The whitepaper is the canonical architecture document and is governed by [ADR-0008](docs/adr/0008-documentation-governance.md).

- **Label every claim.** A sentence about the system carries Implemented, Specified, Target or Hypothesis. If you cannot decide, it is Specified at best.
- **Requirement keywords** (MUST, SHOULD, MAY) follow BCP 14 and are written in capitals only when meant that way.
- **Layout tables are transcribed from source**, never from memory. If you change a record, change its table in the same commit.
- **Numbers are Measured or Target** ([ADR-0010](docs/adr/0010-measured-or-target.md)). A Measured figure links to a benchmark in this repository and the command that produced it. Do not write "tested on".
- **Make claims executable.** When you write that something exists in the tree, add a `<!-- @assert-count ... min="1" -->` directive under the sentence; when you write that something is gone, add `<!-- @assert-absence ... -->`. See the [spec-guard README](https://www.npmjs.com/package/@descent-vtt/spec-guard) for the directive syntax.
- **English is canonical.** The Traditional Chinese file is a reader's guide; do not add layouts, figures or new claims to it.
- **Findings, not silent fixes.** If the document and the tree disagree, record a numbered finding in §11 and fix it in a separate, visible step.
- **The whitepaper's version moves with it** ([ADR-0064](docs/adr/0064-the-documentation-gate-and-the-version.md)).
  A pull request that changes `docs/WHITEPAPER.md` bumps its version: **patch** for a correction
  that changes no claim (a typo, a link, formatting), **minor** for new or changed content (a
  finding, a row, a section, a status, a number), **major** for a restructuring that supersedes.
  The document declares its version and its date twice — in the front matter and in the Document
  control table — and both move together (F-43). `npm run spec:version` checks the agreement; CI
  checks the bump against the base of the pull request.

### Where each fact goes, and the shape of new text

Each kind of fact has one home, and every other document links to it ([ADR-0197](docs/adr/0197-one-place-for-each-kind-of-fact.md)).

| Fact | Its home |
| :--- | :--- |
| Where the work stands | [`docs/STATUS.md`](docs/STATUS.md), rewritten whole by every round, no history |
| Why a thing was decided | an ADR |
| What a hypothesis read | its report in [`docs/research/`](docs/research/README.md) |
| What changed | `CHANGELOG.md`, one entry a change, linking its ADR |
| The architecture | the whitepaper; a status cell gives its label and links |
| How to work | this file, `CLAUDE.md` and `briefs/README.md` |

**Checked by `npm run spec:shape`** (`scripts/check-doc-shape.mjs`), on the text a change adds or modifies against `origin/main`:
- a paragraph or a list item of at most 1 200 characters, a table cell of at most 400, and in the ADR index at most 300;
- `docs/STATUS.md` of at most 60 lines, and a new `CHANGELOG.md` entry of at most 400 characters;
- a new ADR's `title:` of at most 80 characters, its `summary:` of at most 300 and its `kind:` one of `decision`, `protocol`, `built`, `measured` and `process`;
- a new ADR's H1 as `# ADR-NNNN: ` and its title, and its body opening with **In brief**;
- an accepted ADR's body byte for byte its base copy, its front matter changed only in `status:`, `superseded-by:` and `amended-by:`;
- math as GitHub renders it, in every block and every ADR, not only new text ([ADR-0201](docs/adr/0201-math-that-github-renders.md)): `\mathrm{...}` for an operator's name, never `\operatorname`, which GitHub refuses; and math holding a backslash before punctuation, such as `\,` `\;` `\{` or `\%`, written `` $`...`$ `` inline and as a ```` ```math ```` fence for a block, since Markdown takes `\,` in `$...$` as an escape.

Text the base already holds passes whatever its length, so a sentence moved verbatim from one document to another is not new text, nor is a block whose only change is its math's form.
Archived briefs, code blocks and the directive comments are not read.

**Checked by `npm run spec:index`** (`scripts/adr-index.mjs`): the ADR index, the whitepaper's §9 and the hypotheses' tables in §11.1 and `docs/research/README.md` are what the script writes from front matter. Run `node scripts/adr-index.mjs` to write them.

**Held by review**, because no check can read them: an ADR's **In brief** as three bullets (the decision, why, and what is left open), and these six:
1. the conclusion first;
2. one thing a sentence, at most 35 words, and no clauses chained by semicolons;
3. numbers and symbols, `step = 1/16` rather than *a sixteenth*;
4. a mechanism's one short name, the one in the whitepaper's §12 Glossary;
5. results in tables, reasoning in paragraphs;
6. one sentence a line in new text.

### Briefs

Rounds of work are written as numbered, self-contained prompts in [`briefs/`](briefs/README.md). A brief says what done looks like, re-derives its facts with paths and a date, lists deliverables as checkboxes, states what the round is not empowered to do and what it may override (as an ADR), and says how it is verified and reported. When executed it is archived by `spec-brief archive`, which refuses a deliverable left open, moves it to `briefs/archive/` with a frozen banner and writes an integrity hash; the gate fails an archived brief changed since, or one with no hash ([ADR-0185](docs/adr/0185-the-spec-tools-upgraded-and-spec-brief.md)). See that README for the sections every brief carries; `npm run spec:briefs` enforces them from `.spec-brief.json`.

### Architecture decision records

`docs/adr/` holds one [MADR](https://adr.github.io/madr/) file per decision. To propose one: copy the newest file, take the next number, set `status: proposed`, fill in context, drivers, options, outcome and confirmation, and open a pull request. It becomes `accepted` on merge: the pull request's last commit sets the status, so that no second pull request is needed to say so ([ADR-0150](docs/adr/0150-a-round-waits-for-what-it-checks.md)). Superseding a decision means adding `supersedes: ADR-NNNN` to the new record and `superseded-by:` to the old one; `spec-graph` reports the omission. Never renumber, never delete.

## Verification

All of these must pass before a change is called done. They run in three places ([ADR-0184](docs/adr/0184-the-runs-off-the-developer-machine.md)): the first block on your machine before a push and again in the pull request's gate, the second in the gate alone, and the third, the whole-domain tests, only by dispatch. A round's calibration, arms and reproduction are dispatches ([briefs/README.md](briefs/README.md#evidencing-a-round)). A pull request whose every file is under `docs/` or `briefs/`, or is a Markdown file at the root, skips the gate's four Rust jobs, which report success ([ADR-0211](docs/adr/0211-the-round-shortened.md)).
So does one whose two trees, its base's and its merge's as those jobs read them, a green gate of this repository already tested (`scripts/gate-fingerprint.mjs`, [ADR-0223](docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)).

```bash
# 1. Before a push, on the developer machine; the pull request's gate runs these too
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
npm ci
npm run spec
```

```bash
# 2. The pull request's gate alone, with the tests again on AArch64
cargo test --workspace --release --locked
cargo bench -p menteion-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked   # the MSRV floor
cargo +1.85 test --workspace --locked
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff   # cargo-mutants 27.1.0
```

```bash
# 3. By dispatch alone, never on the developer machine
gh workflow run ci.yml --ref <branch> -f scope=exhaustive   # every whole-domain test, eighteen shards; a calibration adds -f exclude='<the round's own tests>'
gh workflow run ci.yml --ref <branch> -f scope=exhaustive -f only='<the round's own tests>'   # a round's reproduction, when nothing its calibration ran has changed
gh workflow run ci.yml --ref <branch> -f scope=arms -f tests='<test> ...'   # a round's arms, lead-ins and trials: one job a test on AArch64, its output kept
gh workflow run ci.yml --ref explore/NNN-<slug> -f scope=arms -f tests='<test>_explore ...'   # an exploration's runs, as often as it needs (docs/explorations/)
```

`npm run spec` is the whole documentation gate and is the single step CI runs, so a check added to it is enforced and a check outside it is enforced nowhere (F-44). The parts run alone: `npm run spec:guard` the executable assertions; `npm run spec:prove` that each of them can fail; `npm run spec:graph` the cross-document checks; `npm run spec:briefs` the briefs' contract and the archived briefs' hashes; `npm run spec:archive` that every archived brief carries one; `npm run spec:explorations` that every exploration note is marked exploratory with its branch, commit and runs and that no ADR cites one outside its Context ([ADR-0193](docs/adr/0193-an-exploration-lane.md)); `npm run spec:scope` that every brief from 075 declares its scope and protects no less than on `origin/main` ([ADR-0191](docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)); `npm run spec:decisions` that every ADR has its row in whitepaper §9 and in `docs/adr/README.md` (F-41); `npm run spec:version` the whitepaper's two version declarations (F-43); `npm run spec:deps` the manifests (TC-2); `npm run spec:costs` that every line of the whole-domain tests' cost table names a test in the tree ([ADR-0092](docs/adr/0092-the-shards-dealt-by-cost.md)). All exit non-zero with a file and line number when something is wrong. `npm run mutants:timeouts <mutants.out>` is not part of the gate: it reads a weekly sweep's own output and says what each of its timeouts was doing when the bound cut it ([ADR-0063](docs/adr/0063-the-sweep-reads-its-own-timeouts.md)).

It runs `npm run spec:index` and `npm run spec:shape` too, the two checks of the documents' shape above, each with its tests.
And it runs `npm run spec:pins` with its tests: a round's pinned tables are in its own file under `tests/<binary>/pins/`, declared, with no test among them ([ADR-0227](docs/adr/0227-the-pinned-tables-in-a-file-a-round.md)).

## Definition of done

A change is done when all of the following hold:

- code compiles on the pinned toolchain and on the MSRV ([ADR-0009](docs/adr/0009-rust-edition-and-msrv.md)) with no new warnings;
- layout assertions and tests pass, in the debug and the release profile;
- a new or changed rule carries a test over the lattice ([ADR-0030](docs/adr/0030-verification-governance.md): `testkit/prop.rs`, `include!`d into the crate's `mod prop`), and the mutation gate on the lines the change touches passes (`cargo mutants --in-diff`; a survivor is a test that does not constrain the rule);
- whitepaper tables and status labels reflect the change;
- an ADR is added or amended if a rule changed;
- there is a changelog entry under `Unreleased`;
- `npm run spec` is green.
