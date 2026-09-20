---
status: archived
date: 2026-09-19
---

> **Executed 2026-09-20 in pull request #71.** Writes ADR-0062 (the triage rule that tells an
> inherent timeout from a directive-class one; the thirty-six timeouts of weekly run `35459078284`
> classified by it, twenty-five inherent and left alone, ten directive-class and now ranges bit for
> bit, one the instrument's; the twenty-one survivors dispositioned, ten by a test, ten by a shape
> that has no equivalent, one by a named exclusion; the decode timeout read as a slow pass starved
> by the hang the sweep ran beside it). Findings F-41 (§9's table lacked ADR-0061's row) resolved
> and F-42 (a timeout that is a slow pass under a concurrent hang) recorded and left open; image
> format 14 unchanged; the determinism pin untouched. Every deliverable is done; notes under the
> boxes say where the tree departs from the text: the first reading was overruled at survivors 1
> to 4 and 9 (a shape, not a test alone or an exclusion) and 18 to 21 (the loop moved into one
> form), `convergent` was rewritten by the rule though no timeout named it, and the twenty-second
> survivor of the latest run, `Task::coin_at`, was ADR-0061's. The report is in the pull request and
> in `CHANGELOG.md`. The body below describes the tree before execution and is not maintained; its
> relative links gained one `../`.

# Brief 028 — The first complete sweep's list: twenty-one survivors dispositioned, and the loops whose end one operator can undo

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

The weekly mutation level of [ADR-0030](../../docs/adr/0030-verification-governance.md) produced its
first complete sweep on 2026-09-18 ([ADR-0058](../../docs/adr/0058-the-weekly-sweep-and-its-timeouts.md)):
3 396 of 3 396 mutants, **21 missed** and **35 timeouts**. ADR-0030 makes the survivors the next
round's list and whitepaper §11.1 carries them as an open item; this is that round. Every survivor
leaves it **dispositioned** — killed by a test, removed by a restructuring that leaves no
equivalent to exclude, or excluded in `.cargo/mutants.toml` with the reason that makes it
equivalent — and the round does not end with a survivor nobody decided about. The 35 timeouts are
detections, not defects, and none of them needs disposing of; but they are also the only map this
repository has of the loops whose end a single operator can undo, which a standing directive
forbids. **One ADR** writes the rule that tells an *inherent* hang (the mutant deletes a bound the
code has, or stops a concurrency primitive's progress) from a *directive-class* one (the unmutated
loop ends by a comparison alone), classifies all 35 by it, and rewrites the directive-class loops as
ranges where the domain gives one, every result bit-identical. When the round is done a weekly sweep
dispatched on its branch lists no survivor that is not recorded as a finding, no directive-class
timeout that is not recorded with its reason, and every job green; §11.1's item is closed; the
determinism pin has not moved; and this brief is archived.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4): `cargo-mutants` stays at its pin
  (27.1.0), and the options this round uses (`--file`, `--shard`, `--timeout`, `exclude_re`) are
  years old. No new tool (a test runner, a mutation framework, the `mutants` crate), no version
  bump and no newer technique enters this round, however recent its results; a restructuring uses
  the language as the MSRV (1.85) has it, and `let` chains, which that floor refuses, are out.
- Every claim is Implemented, Specified, Target or Hypothesis. A mutation outcome is stated with the
  run that produced it (its id and head), never from memory; a timing is CI's wall clock and is never
  a claim about the engine ([ADR-0010](../../docs/adr/0010-measured-or-target.md)).
- The repository wins over the document; a disagreement is a numbered finding in whitepaper §11,
  never a silent edit. An accepted ADR is history: ADR-0058's figures stay in it.
- No `f32`/`f64`, in the crates and in the tests; every operation on a state field saturates or wraps
  by name (`clippy::arithmetic_side_effects` is denied everywhere,
  [ADR-0029](../../docs/adr/0029-structural-enforcement.md)); a shift amount is bounded a line above the
  shift.
- **Every loop ends by construction**: a countdown, a range, a scan by `get`, a slice's iterator, a
  recursion whose depth argument falls to a stated bound; never by a comparison alone that one
  operator flip turns into a walk without end. This round is the first to hold the tree to that
  sentence with evidence, and the evidence is Context item 4.
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads or
  `unsafe` in a state crate (`unsafe_code = "forbid"`, ADR-0029).
- Every quantity has one owner ([ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md)). No new
  crate; the crate count stays 32. No dependency in a state crate (`npm run spec:deps`).
- No record changes and the image format stays 14. **No rule's result changes**: a restructured loop
  returns bit for bit what it returned before, which its existing tests and the determinism pin of
  [ADR-0030](../../docs/adr/0030-verification-governance.md) (`PINNED_ARENA_HASH` in
  `runtime/cortex-runtime/tests/differential.rs`) hold. If a change moves the pin, it is not this
  round's change.
- A survivor's disposition follows ADR-0030 and the routine the repository already uses: **a test
  first**; **a restructuring** when the survivor is an equivalent that the code's shape creates and
  a different shape would not (a guard whose out-of-range case is naturally false becomes a range
  pattern; a sign branch becomes `signum`); **an exclusion last**, only for an equivalent, with the
  reason in the file beside the pattern, the function name in the pattern, and no pattern broader
  than the mutants it names. Every pinned number an arithmetic oracle can produce is computed by that
  oracle before the test that asserts it is written.
- The engine never amends its own code ([ADR-0031](../../docs/adr/0031-policy-amendment.md)).
- Conventional Commits with a real body; never commit on `main`; the required checks keep their
  names; no product name enters a crate.

## Context

Re-derived on 2026-09-19 against `main` at `e29f3ab`. The mutation outcomes come from run
`35284722491`, whose head `cf1b626` holds engine code identical to `e29f3ab`'s and a workflow that
differs from it in comments only (`git diff cf1b626 e29f3ab -- crates runtime testkit benches
Cargo.toml Cargo.lock .cargo` is empty); run `35374662481`, dispatched on `main` at `e29f3ab`,
repeats it. **Brief 027 executes before this one** and touches `runtime/cortex-runtime/src/task.rs`,
the runtime's `lib.rs` and its manifest, none of which holds a survivor below; line numbers still
move, so re-derive the two lists from the latest weekly run's `mutants-out-*` artifacts
(`missed.txt`, `timeout.txt`) before starting, and treat any difference as the tree's word.

1. **The sweep and where the lists live.** `.github/workflows/ci.yml`, job `mutants-weekly`: one job
   for `crates/**` and six round-robin shards for `runtime/**`, each deriving its per-mutant bound
   from its own suite timed on its runner (three times plus thirty seconds) and failing if its
   outcome counts differ from `cargo mutants --list` under the same arguments (ADR-0058). Run
   `35284722491`: 3 396 tested, 3 211 caught, **21 missed**, **35 timeouts**, 129 unviable; the
   slowest shard 1 h 58 m. `gh workflow run ci.yml --ref <branch>` dispatches it on a branch; the
   artifacts are `mutants-out-crates` and `mutants-out-runtime-0` to `-5`, ninety days.
2. **What the configuration already says.** `.cargo/mutants.toml`'s `exclude_re` names equivalents by
   description with a reason each; classes this list meets again: "A clamp compared at its own
   bound" (`plant_delay`'s getter, `mul_q16`'s `<` side, `membrane.rs`'s `add`), "Work stealing
   balances load and changes no result" (`Worker<CAP>::steal -> Option<u32> with None`), and "The
   platform-conditional log I/O: `read_at` and `write_at` each have a Unix and a Windows form, and
   the form a platform does not compile has no test on that platform" (only the `-> Ok(())`
   replacements). Whitepaper Appendix B's V-6 row: "a mutant that hangs a test is counted as caught".
3. **The twenty-one survivors, with a first reading.** The reading is this brief's, from the code at
   the line; it is a hypothesis to verify, not a disposition.

   | # | Site (at `e29f3ab`) | Mutant | First reading |
   | ---: | :--- | :--- | :--- |
   | 1 | `crates/cortex-cerebellum/src/lib.rs:49:22` `set_plant_delay` | `>` → `==` | Test gap: with `==` a delay above `MAX_PLANT_DELAY` passes unclamped, so no test gives the *setter* a delay above the bound. |
   | 2 | same | `>` → `>=` | Equivalent: a clamp compared at its own bound. The getter `plant_delay` (line 63) carries the same clamp and is already excluded; the setter is not. |
   | 3 | `cortex-cerebellum/src/lib.rs:76:19` `filled` | `>` → `>=` | Equivalent: the same clamp class. |
   | 4 | `cortex-cerebellum/src/lib.rs:169:10` `mul_q16` | `>` → `>=` | Equivalent: the same class; the `<` side of this clamp is already excluded, this side is not. |
   | 5 | `crates/cortex-core/src/dynamics/membrane.rs:134:63` `integrate` | `<` → `<=` | Test gap: `self.v_soma < self.v_thresh` decides firing; the two differ only at `v_soma == v_thresh`, which no test sets. Whether the unit fires *at* the threshold is §8.8's rule; the test pins what the code does. |
   | 6 | `crates/cortex-core/src/dynamics/plasticity.rs:35:29` `stp_decay_factor_q16` | `>` → `>=` | Equivalent as written: `result` is `u32`, so `result >= 0` is always true and the mutant only removes an early exit whose result is the same zero. Item 4's rewrite of this loop removes the mutant with the comparison. |
   | 7 | `crates/cortex-embodiment/src/lib.rs:139:9` `EmbodimentRingBuffer::is_empty` | `-> bool` with `true` | Test gap: no test asserts `is_empty()` is false on a ring holding a frame. |
   | 8 | `crates/cortex-embodiment/src/vocal.rs:259:38` `VocalSynth::from_frame` | `>` → `>=` | To establish: the two differ only at `period == u16::MAX`, with `period = (sample_rate_hz << 16) / f0_hz_q16`. A test if some frame reaches it; an exclusion carrying the arithmetic if none can. |
   | 9 | `crates/cortex-knowledge/src/lib.rs:137:25` `SemanticOntologyNode::consolidate` | `>` → `>=` | Equivalent: keeping the maximum, `>=` at equality writes the value already there. |
   | 10 | `crates/cortex-linguistic/src/lib.rs:273:29` `LinguisticFrameSlot::mark_indirect` | `>` → `>=` | Test gap: the two differ at `intended_act == SPEECH_ACT_EXPRESSIVE`, the last valid act, which no test marks. |
   | 11 | `cortex-linguistic/src/lib.rs:307:36` `LinguisticFrameSlot::apply_face` | `\|=` → `&=` | Test gap: no test asserts `GATE_PARTICLE_OPEN` in `syntax_gate_flags` after a softened turn (§8.14's tact). |
   | 12 | `crates/cortex-social/src/lib.rs:152:38` `SocialPerspectiveNode::register` | `*` → `+` | Test gap: `3 * (Q16_ONE / 4)` becomes 16 387, so no test asserts the register for a trust between 0.25 and 0.75, where `REGISTER_COURTEOUS` is the answer. |
   | 13 | `crates/cortex-thalamus/src/lib.rs:62:17` `ThalamicRelayNode::set_gating_mode` | `>` → `>=` | Test gap: the two differ at `mode == GATING_CLOSED`, a valid mode no test sets through the setter. |
   | 14 | `runtime/cortex-runtime/src/executor.rs:1303:9` `Executor::workers` | `-> usize` with `1` | Test gap: no test reads `workers()` on an executor of more than one. |
   | 15 | `executor.rs:1814:23` `Worker::steal` | `==` → `!=` | Equivalent by the configuration's own reason: with `!=` a worker steals only from itself, and stealing changes no result (the contention test runs every unit exactly once). The existing exclusion names only the `-> None` replacement. |
   | 16–17 | `runtime/cortex-runtime/src/image.rs:209:9`, `211:33` `WriteAheadLog::holds` | `-> bool` with `false`; `!=` → `==` | Test gap: no test asserts `holds(unit)` true for a logged unit and false for one that is not. |
   | 18–21 | `image.rs:253:11`, `255:14` `read_at`; `270:11`, `272:14` `write_at` | `delete !`; `==` → `!=` | Platform-conditional: these lines are the `#[cfg(windows)]` forms, compiled by no CI runner. The existing exclusion covers only the `-> Ok(())` replacements, and the Unix forms share the function names, so a pattern on the name alone would also silence mutants that Linux does test. |

   Tally of the first reading: ten test gaps, six equivalents, one boundary to establish, four
   platform-conditional.
4. **The thirty-five timeouts, and the evidence they are.** A timeout is caught (V-6), costs its
   job three times its suite, and blocks nothing. What each one locates is a loop the mutant made
   endless; the rule this round writes asks whether the *unmutated* loop ended by construction.
   - **Inherent — the mutant deletes a bound the code has (6).** `Chain::next`
     (`crates/cortex-core/src/dynamics/synapse.rs:358`, three), `MailboxDrain::next`
     (`dynamics/neuron.rs:78`, two) and `DeltaChain::next` (`dynamics/delta.rs:116`, one) each open
     with `if self.next == END || self.remaining == 0 { return None; }`: the walk is bounded by a
     countdown, and the mutants (`||` → `&&`, or the whole body replaced by `Some(_)`) remove it.
     An iterator's `next` cannot be a range; the hang is the detection.
   - **Inherent — a concurrency primitive stops making progress (18).** `Injector::push` and
     `Injector::pop` (`runtime/cortex-runtime/src/injector.rs`, fourteen): Vyukov's bounded queue
     retries a compare-and-swap in a `loop`, which ends by another thread's progress, not by
     construction (ADR-0023); `SpinBarrier::wait` (`barrier.rs:35`, one), `Executor::stop_workers`
     (`executor.rs:1715`, two) and `Worker::run` (`executor.rs:1763`, one): a barrier that never
     releases or a worker never stopped hangs by nature.
   - **Directive-class — the unmutated loop ends by a comparison alone (10).**
     `stp_decay_factor_q16` (`plasticity.rs:35`, three): `while exp > 0 && result > 0` with
     `exp >>= 1`, where the exponent's bits are a range (`for _ in 0..u32::BITS`, breaking early on a
     zero result). `fraction.rs:194` `deepest` and `fraction.rs:339` `deepest_verdict`
     (`crates/cortex-arithmetic/src/fraction.rs`, three): `while walk.depth < depth &&
     walk.advance(..).is_ok() {}` and a `loop` broken by `walk.depth >= depth ||
     walk.advance(..).is_err()`, where the depth is a range bounded by `MAX_DEPTH` above it; and
     `Walk::advance` replaced by `Ok(())` (`fraction.rs:151`, one), which hangs only because the
     callers' loops are not ranges. And one **test**: `crates/cortex-hippocampus/src/lib.rs:442`,
     inside the crate's `#[cfg(test)]` module, `while !e.is_spent() { e.depotentiate(); }` — the
     three mutants of `is_spent` and `depotentiate` (lines 112, 128) make that loop endless, where a
     range over the tag's width ends it and the assertion after it catches them at once.
   - **Unexplained (1).** `runtime/cortex-runtime/src/image.rs:718:24`, `Image::decode`,
     `delete !` in `if !exec.load_term(node) { return Err(ImageError::MalformedTerm(..)) }`. That is
     not a loop; a decode that now fails on every valid term should fail a test fast. Why some test
     hangs on it instead is not known, and is the one timeout to read before classifying.
5. **What already enforces what.** The in-diff gate (`mutants` job) fails a pull request on any
   survivor in its changed lines, so every test and restructuring this round adds is itself held.
   The weekly level blocks nothing, so this round's evidence is a dispatched weekly run on its
   branch, compared with `35284722491`.

## Deliverables

- [x] **The triage ADR (the next free number; `ls docs/adr`)** (`depends-on: ADR-0058`; ADR-0030 and
  ADR-0029 named). The rule that separates an inherent timeout from a directive-class one, written
  before the classification is applied, and the classification of all 35 by it in a table (site,
  mutant, class, reason); for every directive-class loop the rewrite or the reason it keeps its
  shape; the image decode timeout explained and classified. The per-survivor dispositions, one row
  each, with the test, restructuring or exclusion that settled it and the run that shows it.
  **Done:** [ADR-0062](../../../docs/adr/0062-the-first-complete-sweeps-list.md). The rule has five
  kinds, in order (another thread; a test's own loop; an ordering comparison on moved state; a
  bound the mutant removed; none of these), and was applied to the thirty-six timeouts of the
  latest run rather than the thirty-five named here: `barrier.rs:41` had joined the list.
- [x] **Each of the 21 survivors dispositioned**, in the order the first reading suggests and
  overruling it wherever the code says otherwise:
  - the test gaps get tests in their own crate (a runtime test never kills a state-crate mutant:
    `.cargo/mutants.toml`), each naming the boundary or behaviour it holds, with its numbers from an
    oracle first;
  - the equivalents are removed by restructuring where a shape exists that has no equivalent, and
    otherwise excluded beside the existing entry of their class, with the function name in the
    pattern and the reason in a comment;
  - `VocalSynth::from_frame`'s boundary is reached by a test or proved unreachable in the exclusion's
    comment;
  - the four `#[cfg(windows)]` survivors are answered without silencing the Unix forms: either a
    pattern that names them and nothing Linux compiles, or the loop moved into one
    platform-independent function tested on every platform, with only the positioned read and write
    gated.
  **Done**, with the first reading overruled where the code said so: survivors 1 to 4 and 9 by a
  shape that has no equivalent (`min`, range patterns, `max`) rather than by a test or an
  exclusion; 8 reached by a test (65 535 samples); 15 excluded beside its entry; 18 to 21 by the
  second answer, the loop moved into one form, with the Windows calls under distinct names so that
  the exclusion can name nothing Linux compiles, and the old `Ok(())` entry, which silenced the
  Unix forms, removed. The twenty-second survivor of the latest run, `Task::coin_at`, was
  [ADR-0061](../../../docs/adr/0061-the-learning-runs-leave-the-gate.md)'s and is not this round's.
- [x] **The directive-class loops rewritten** as ranges or countdowns where the domain gives one
  (`stp_decay_factor_q16`, `deepest`, `deepest_verdict`, the hippocampus test), every result
  bit-identical: the existing exact-step and property tests of each function hold unchanged, a
  property test over the lattice of `testkit/prop.rs` compares each rewritten function with its
  previous form over seeded inputs (the previous form kept in the test as the oracle), and the
  determinism pin does not move.
  **Done**, and `convergent` with them: the rule named it, the sweep had not, because its own
  mutants were all caught. The pin did not move.
- [x] **The evidence.** A weekly dispatch on the round's branch after the last change: every job
  green; the survivors only those the ADR records as findings; the timeouts only those it classifies
  as inherent or keeps with a reason. Its run id, the counts beside `35284722491`'s, and each list's
  difference are in the ADR.
  **Done:** the dispatched run's id, its counts and the two lists' differences are in ADR-0062's
  section "The evidence". The decode timeout stays in `timeout.txt` by the instrument's nature
  (F-42), recorded with its reason as this box allows.
- [x] **The documents, in the same pull request.** Whitepaper §11.1's item "The first complete
  mutation sweep's survivors" closed with its disposition and the ADR; Appendix B's V-6 row, if the
  triage rule changes what it says; `CHANGELOG.md`; the ADR index; `CLAUDE.md` if its sentence about
  loops or the mutation gate changes. Executable directives under every sentence that claims a test
  or an exclusion exists.
  **Done:** whitepaper 4.16.0 (§9's rows for ADR-0061, F-41, and ADR-0062, every ADR's row now
  held by `scripts/check-decisions.mjs` in `npm run spec`; §11 F-41 and F-42; §11.1's item closed with eight directives under it;
  Appendix B's V-6 row), `CHANGELOG.md`, the ADR index, `CLAUDE.md` (the loop invariant, which it
  had not carried) and `CONTRIBUTING.md`'s code rules.
- [x] **The brief archived** as `briefs/README.md` says, with every deliverable dispositioned.
  **Done:** this file.

## Not empowered

- No change to any rule's result, no record change, no format bump, no move of the determinism pin.
- No new crate, no dependency in a state crate, and no `mutants` crate for `#[mutants::skip]`: an
  attribute that needs a dependency is a dependency.
- No exclusion without its reason in the file; no pattern without the function's name; no pattern that
  would also match a mutant a runner compiles and tests (the Unix `read_at` and `write_at` above all).
- No exclusion for a timeout: a timeout is caught, and hiding it would also hide the loop it locates.
- No change to `.github/workflows/ci.yml`'s sweep, its bounds, its shards or its completeness check
  (ADR-0058); no percentage threshold; no lowering of the in-diff gate.
- No rewrite of an iterator's `next` or of the injector, the barrier or the worker loop to make a
  timeout go away: those are inherent by Context item 4, and a bound added only to shorten a hang
  changes nothing a test reads.
- No change to brief 027's code (`task.rs` and its tests), which its own round owns.
- No claim that a survivor is equivalent without the argument, in the file, that makes it so.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in the ADR:
the disposition of any survivor, including overruling the first reading in either direction (a
reading of "equivalent" that a test can distinguish is a test gap, and one of "test gap" that no
input reaches is an equivalent with its proof); the shape of every rewrite, so long as the result is
bit-identical and the loop ends by construction; the wording of the triage rule and its classes; the
answer for the `#[cfg(windows)]` forms; whether the image decode timeout is inherent, directive-class
or a finding of its own; and whether a survivor that exposes a behaviour the documents do not state
becomes a finding rather than a test. It may not reach the standing directives, the whitepaper's
invariants or the constraints in `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
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
gh workflow run ci.yml --ref <this round's branch>
```

Every command exits 0; the in-diff gate reports no survivor in CI (a local Windows run marks mutants
whose build failed as unviable and is not the gate's truth). The dispatched weekly run is green in
every job, and its `missed.txt` and `timeout.txt` hold only what the ADR records. The determinism pin
and the format are unchanged.

## Report

The closing message states: the triage rule as written before it was applied; the table of the 21
survivors with each disposition and what settled it, and every place the first reading was overruled
and why; the 35 timeouts by class, the loops rewritten and the ones kept with their reasons, and what
the image decode timeout turned out to be; the dispatched weekly run's id and its counts beside
`35284722491`'s; what the in-diff gate found on the round's own lines; what was not done and why
(anything left as a finding, anything the empowerment was used for); and what the re-examination
after the round recommends next.
