---
status: accepted
date: 2026-09-20
depends-on: ADR-0058
decision-makers: VirtualCortex maintainers
---

# ADR-0062: The first complete sweep's list — a triage rule that tells an inherent timeout from a directive-class one, thirty-six timeouts classified and four loops made ranges, twenty-one survivors dispositioned by a test, a shape or a named exclusion, and one timeout that was a slow pass starved by the hang beside it

## Context and Problem Statement

[ADR-0030](0030-verification-governance.md) gives the weekly mutation level one purpose: its survivors are the next round's list. [ADR-0058](0058-the-weekly-sweep-and-its-timeouts.md) made the level complete for the first time on 2026-09-18 (run [35284722491](https://github.com/DescentVTT/VirtualCortex/actions/runs/35284722491)): 3 396 of 3 396 mutants, 21 missed, 35 timeouts. Whitepaper §11.1 carried the 21 as an open item; this is the round that reads the artifacts.

The lists were re-derived on 2026-09-20 from the latest weekly run, [35459078284](https://github.com/DescentVTT/VirtualCortex/actions/runs/35459078284) at `fa814e4` (the branch of [ADR-0061](0061-the-learning-runs-leave-the-gate.md)), whose engine code differs from `main` at `bbb831d` by one test in `task.rs` and nothing else: **22 missed and 36 timeouts**. Against `35284722491` the missed list gained `task.rs:301:26: replace ^ with | in Task::coin_at`, which ADR-0061 answered with the test `main` now carries, and the timeout list gained `barrier.rs:41:55: replace == with != in SpinBarrier::wait`, a second line of the same barrier. Run [35374662481](https://github.com/DescentVTT/VirtualCortex/actions/runs/35374662481) on `main` at `e29f3ab` repeated `35284722491`'s two lists exactly, and run [35444829805](https://github.com/DescentVTT/VirtualCortex/actions/runs/35444829805) at `07c6774`, with brief 027's five learning runs still in the gate, listed three more timeouts (`trace` in `branching.rs`, `Induction::search` in `store.rs`, the `clauses` field of `Image::decode`) that the latest run reports caught.

Two questions. The survivors: each one is a test gap, an equivalent, or a boundary nobody has established, and ADR-0030 says which disposition each class gets. The timeouts: Appendix B's V-6 row counts a hung test as caught, so none of them blocks anything and none needs disposing of, but they are the only map this repository has of the loops whose end a single flipped operator undoes, and a standing directive of every brief since 004 says a loop ends by construction, never by a comparison alone. Nothing in the tree had ever held the code to that sentence with evidence. Some of the thirty-six are the sentence's counter-examples; most are not, and the two kinds had to be told apart by a rule written before it was applied, or the classification would be a reading of each case on its own.

## Decision Drivers

- ADR-0030: a survivor becomes a test, a restructuring that leaves no equivalent, or an exclusion with the function's name and the reason in the file; a test first, an exclusion last. An exclusion that would also silence a mutant a CI runner compiles and tests is a wrong entry.
- The standing directive on loops, applied for the first time with evidence: a countdown, a range, a scan by `get`, a slice's iterator, a recursion whose depth argument falls to a bound. A timeout is not a defect, but a timeout in a loop that ends by a comparison alone is the directive's counter-example, and the rewrite must be bit-identical, held by a test against the previous form and by the determinism pin.
- Latest ≠ Newest: `cargo-mutants` stays at 27.1.0; `--file`, `--shard`, `--timeout` and `exclude_re` are what the tool has had for years; no `mutants` crate for `#[mutants::skip]` (a dependency in a state crate, TC-2); `let` chains are out (MSRV 1.85 refuses them).
- No change to a rule's result, to a record, to the image format or to the sweep of ADR-0058; no exclusion for a timeout, which would hide the loop it locates.
- Every pinned number an arithmetic oracle can produce is computed by that oracle before the test that asserts it: the soma's landing potential, the vocal period's quotients, the trust threshold.

## Considered Options

1. **Exclude the hanging mutants, or bound every loop that hangs.** Refused twice over: ADR-0058 already refused the exclusion (a hang is a detection), and a bound added to an iterator's `next` or to a compare-and-swap retry only to shorten a hang changes nothing a test reads and duplicates or damages the bound the loop has.
2. **Read each timeout on its own and rewrite the ones that look wrong.** Thirty-six readings with no rule between them; the next sweep's timeouts would be read differently. Refused.
3. **A triage rule written first, applied to all thirty-six, with only the directive class rewritten; every survivor dispositioned by ADR-0030's order; a dispatched sweep as the evidence.**
4. **Record the twenty-one as findings and move on.** ADR-0030 says a survivor is a test, an exclusion or a finding, and a finding is for a survivor that exposes a behaviour the documents do not state. None of the twenty-one does; every one is a boundary the code already has. Refused.

## Decision Outcome

Option 3.

### The triage rule, written before it was applied

For a mutant in `timeout.txt`, the per-mutant log under `mutants.out/log/` names the test binary and the test that was running when the bound cut it; from there the loop that did not end is read, and the question is asked of the **unmutated** loop: what ends it? In this order:

1. **Another thread.** The loop ends when another thread acts: a compare-and-swap another producer may win, a generation another party advances, a flag the coordinator sets. **Inherent.** The loop is the protocol, a bound would change the primitive, and the hang is what a broken protocol looks like. It lives in the runtime only; a state crate has no thread to wait on (TC-5).
2. **A test's own loop.** The loop is in a `#[cfg(test)]` module or an integration test and ends by the function under test. **Directive-class.** A test's loop never ends by the rule it tests: it takes a range, and the assertion after the range is what the mutant meets.
3. **An ordering comparison on moved state.** The loop's continuation is `<`, `>`, `<=` or `>=` on a value its own body changes, and the number of iterations is fixed before the loop begins: the bits of a word, a depth below a constant. **Directive-class.** The loop is rewritten as a `for` over that count, the comparison kept only where it is an early exit that cannot extend the loop, the result bit for bit what it was, held by a property test against the previous form kept in the test as the oracle.
4. **A bound by construction that the mutant removed.** The loop ends by a `for` over a range or a slice, a countdown by one tested for zero, an end sentinel, or a `get` that returns `None`, and the mutant deleted or inverted that test or replaced the body that carries it. **Inherent.** The hang is the detection of the deleted bound; a second bound would duplicate the first and hide its deletion. An iterator's `next` is always here when it is bounded at all: the consumer drives it, so it cannot be a range.
5. **None of these.** The log is read for the loop, and if no loop of the mutated code is found, the timeout is the instrument's, not the code's: a slow pass under the bound. It is recorded as a finding with its evidence, never as a class of loop.

### The thirty-six timeouts by the rule

From run `35459078284`; the sites at `fa814e4`, which `main` at `bbb831d` holds line for line outside `task.rs`.

| Site | Mutants | Class | Reason |
| :--- | ---: | :--- | :--- |
| `Injector::push`, `Injector::pop` (`runtime/cortex-runtime/src/injector.rs`) | 14 | Inherent, 1 | Vyukov's queue retries a compare-and-swap in a `loop` that ends when the position moves, which another producer may do first (ADR-0023). A slot mask or a sequence comparison inverted makes the position never match: every producer spins. |
| `SpinBarrier::wait` (`barrier.rs:35`, `:41`) | 2 | Inherent, 1 | The waiter spins until the generation moves, which the last arrival does. The arrival count's `==` inverted at line 35 releases no one; the generation's `==` inverted at line 41 makes every waiter pass at once, and the barrier's own test fails while the executor's phases, no longer separated, hang in the task tests (caught in `35284722491`, a timeout here: both are detections). |
| `Executor::stop_workers` (`executor.rs:1715`, twice) | 2 | Inherent, 1 | The workers are told to stop and then met at the barrier; with the swap inverted or the body deleted they are never released, and `shutdown` joins them forever. |
| `Worker::run` (`executor.rs:1763`) | 1 | Inherent, 1 | A worker that returns at once leaves the coordinator waiting at a barrier of parties who will never arrive. |
| `Chain::next` (`crates/cortex-core/src/dynamics/synapse.rs:358`) | 3 | Inherent, 4 | `if self.next == CHAIN_END \|\| self.remaining == 0 { return None; }`: a countdown by one tested for zero, the bound the `\|\|` to `&&` and the two `Some(_)` bodies remove. An iterator's `next`. |
| `MailboxDrain::next` (`neuron.rs:78`) | 2 | Inherent, 4 | The same shape, the same two mutants. |
| `DeltaChain::next` (`delta.rs:116`) | 1 | Inherent, 4 | The same shape, the `\|\|` to `&&`. |
| `stp_decay_factor_q16` (`plasticity.rs:35`) | 3 | Directive-class, 3 | `while exp > 0 && result > 0` with `exp >>= 1`: two ordering comparisons on values the body moves, and the count was always the exponent's significant bits. **Rewritten** as `for bit in 0..bits` with `bits = u32::BITS - leading_zeros(elapsed)`; the early exit on a zero result is gone, because a zero result stays zero through the bits that remain, so it was never a result. |
| `deepest` (`crates/cortex-arithmetic/src/fraction.rs:194`) | 1 | Directive-class, 3 | `while walk.depth < depth && walk.advance(..).is_ok() {}`: the depth is below `MAX_DEPTH` (64) before the loop begins. **Rewritten** as `for _ in 0..depth` breaking at the first flag. |
| `deepest_verdict` (`fraction.rs:339`, twice) | 2 | Directive-class, 3 | A `loop` broken by `walk.depth >= depth \|\| advance.is_err()`. **Rewritten** as `for remaining in (0..=depth).rev()`, the same evaluations at the same depths, the same advances, the same slot afterwards. |
| `Walk::advance` replaced by `Ok(())` (`fraction.rs:151`) | 1 | Directive-class, 3 | Not a loop: a step that does not move the walk, which hung the three callers whose loops asked the walk where it was. `convergent` (`fraction.rs:175`), whose own mutants were all caught, has the same shape and was **rewritten** by the rule, not by the sweep: `for _ in 0..depth`. With the three walks ranges, this mutant is caught by their exact tests; the oracles kept in the test step the walk with their own copy of `advance`, because the first dispatched sweep of this round found the oracle test hanging under this very mutant, the rule's second kind met by the round's own test. |
| `Episode::is_spent`, `Episode::depotentiate` (`crates/cortex-hippocampus/src/lib.rs:112`, `:128`) | 3 | Directive-class, 2 | The hanging loop was a test's: `while !e.is_spent() { e.depotentiate(); }` in `an_episode_is_bound_to_a_symbol_once_and_never_to_none`, which ended by the two functions under test. The crate's other tests failed the three mutants at once (the log shows four `FAILED` before the hang), and the binary never ended. **Rewritten** as `for _ in 0..e.tag` with `assert!(e.is_spent())` after it. |
| `Image::decode`, `delete !` at `image.rs:718` | 1 | None of these, 5 | See F-42 below: not a loop of the mutated code. |

Twenty-five inherent (nineteen of the first kind, six of the fourth), ten directive-class, one of the fifth kind. Every directive-class loop was rewritten, and no inherent one was touched.

### The rewrites, each bit-identical

- `stp_decay_factor_q16`: `the_decay_factor_over_a_range_of_bits_is_the_previous_form_bit_for_bit` in the crate's `mod prop` holds the range form to the previous `while` form, kept in the test as `decay_factor_before_the_range`, over every pair of the `u32` lattice and ten time constants and over 200 000 seeded pairs; the existing exact tests and monotonicity property stand unchanged.
- `convergent`, `deepest`, `deepest_verdict`: `the_walks_over_a_range_are_the_previous_forms_bit_for_bit_and_leave_the_same_slot` holds each to its previous form over 3 000 seeded coefficient tuples at depths 0 to 66, comparing the convergent, the verdict and the whole `ArithmeticScratchpadSlot` afterwards, so that the sequence of operations through the slot is the same and not only the result. The previous forms step the walk with a copy of `Walk::advance` of their own (`step_before_the_range`): an oracle's loop must not end by a function under mutation, or the mutant that stops the walk hangs the oracle instead of failing the test, which the first dispatched sweep found; the pinned convergents of $e$ and the silver ratio and the search's tests stand unchanged.
- The hippocampus test's loop, a range over the tag; the crate's tests stand.
- The determinism pin (`PINNED_ARENA_HASH`, `PINNED_SPIKE_COUNT` in `runtime/cortex-runtime/tests/differential.rs`) did not move, and the image format is 14.

### F-42: the timeout that was a slow pass under the hang beside it

`image.rs:718:24: delete ! in Image::decode` turns `if !exec.load_term(node)` into its opposite: every image with a well-formed term node is refused at its first node, which `tests/store.rs`'s loader test and every round trip of a term arena fail at once. It timed out in all four complete sweeps, always in the `runtime-0` shard. Its per-mutant log says why: the last binary running when the bound cut the run was `criticality`, whose `the_loop_is_bit_identical_on_one_and_four_workers` had been "running for over 60 seconds", and that binary decodes nothing; in the same shard's caught mutants it finishes in 8 to 95 seconds, once in 270. The shard runs mutants two at a time (`--jobs 2`, ADR-0058), and the tool's `debug.log` gives the neighbour: the decode mutant's test run began at 3 197 s into the job, and from 3 207 s to 4 348 s the other slot ran `injector.rs:60:40: replace & with | in Injector::push`, a hang that spins every producer thread for its whole bound; before it, the same slot had run `stop_workers`'s hang from 1 944 s to 3 084 s. A four-worker test of spin barriers that yield, sharing four vCPUs with a spinning neighbour, did not reach the binary that would have failed it before its own bound of about 1 140 s. Run alone on a developer machine (`cargo mutants -p cortex-runtime --file runtime/cortex-runtime/src/image.rs -F "delete ! in Image::decode" --jobs 1`, not an admissible figure, a classification only) the mutant is **caught**, by `tests/store.rs`'s `every_clause_of_the_loader_s_checks_refuses_on_its_own`, with every binary before it passing and the `criticality` binary among them finishing in 2.65 s.

In the sweep dispatched on this round's branch (`35485126396`, below) the same mutant was caught, and the `clauses` field's mutant of the same function took its place in `timeout.txt` with the same shape of evidence: the class recurs, the mutant does not. This is not a loop of the code and no rewrite answers it: it is the instrument, under which a hang costs its own bound and part of its neighbour's. It is recorded as finding F-42 in whitepaper §11 with the disposition that ADR-0058's sweep is unchanged by this round (its `--jobs`, bounds, shards and completeness check are not this brief's to move), that V-6's "counted as caught" already covers a slow pass, and that a next change to the sweep would run the runtime shards at `--jobs 1` or read `debug.log` for a neighbour before reading a timeout as a hang. The three timeouts of run `35444829805` that the latest run reports caught (`trace`, `Induction::search`, the `clauses` field) are, by the same evidence, the same kind: their logs name binaries that do not test them.

### The twenty-one survivors, and the one ADR-0061 met

In the order of brief 028's first reading, each with what settled it. "Overruled" marks a disposition the reading did not predict.

| # | Mutant (at the run's head) | First reading | Disposition |
| ---: | :--- | :--- | :--- |
| 1 | `cortex-cerebellum/src/lib.rs:49:22: replace > with == in CerebellarMicrozone::set_plant_delay` | Test gap | **Restructured and tested** (overruled: a shape, not a test alone). The clamp is `d.min(MAX_PLANT_DELAY)`, which has no comparison to mutate; `plant_delay_is_clamped_and_zero_disables_learning` now reads the stored byte of `delay_ctl` after setting 9, 7 and 6, because the getter's own clamp had hidden the setter's. |
| 2 | the same site, `> with >=` | Equivalent | **Removed** by the same `min`: nothing to exclude. |
| 3 | `cortex-cerebellum/src/lib.rs:76:19: replace > with >= in CerebellarMicrozone::filled` | Equivalent | **Restructured** (overruled: not excluded). A `const fn` cannot call `min`; the clamp is a range pattern, `match filled { 0..=RING => .., _ => RING }`, whose only mutant is the arm's deletion, caught by `delay_line_fills_to_seven_and_wraps`. The getter `plant_delay`, the same clamp beside it, took the same shape and its exclusion entry is removed. |
| 4 | `cortex-cerebellum/src/lib.rs:169:10: replace > with >= in mul_q16` | Equivalent | **Restructured** (overruled): `match p { LOW..=HIGH => p as i32, i64::MIN..LOW => i32::MIN, _ => i32::MAX }`; the `<` side's exclusion entry is removed with it, and the two arms' deletions are caught by the product's property test against its reference. |
| 5 | `cortex-core/src/dynamics/membrane.rs:134:63: replace < with <= in DendriticSuperNeuron::integrate` | Test gap | **Tested**: `a_soma_that_lands_exactly_on_a_positive_threshold_fires_and_one_lsb_short_does_not`. From every compartment at 1.0 with no input, one tick leaves the soma at `0xFFD4` (the leak $2^{-11}$ and the two couplings' shares of the compartments' leaks, $2^{-13}$ and $2^{-14}$, worked by hand from §8.8 before the number was pinned); a threshold there fires on that tick, one LSB above does not. The rule is the documented one, "at or above". |
| 6 | `cortex-core/src/dynamics/plasticity.rs:35:29: replace > with >= in stp_decay_factor_q16` | Equivalent as written | **Removed** by the loop's rewrite above: the comparison is gone. |
| 7 | `cortex-embodiment/src/lib.rs:139:9: replace EmbodimentRingBuffer::is_empty -> bool with true` | Test gap | **Tested**: `empty_ring_has_nothing_to_read_and_a_slot_to_write` asserts `is_empty` true on a new ring, false after a publish, true after the release. |
| 8 | `cortex-embodiment/src/vocal.rs:259:38: replace > with >= in VocalSynth::from_frame` | To establish | **Reached by a test**: `the_longest_period_the_bound_admits_renders_and_one_sample_longer_is_refused`. At a sample rate of 65 535 Hz and a fundamental of 1.0 Hz the period is exactly 65 535 samples, rendered; at 65 535/65 536 Hz it is 65 536, refused (the quotients asserted in the test before the frames are built). |
| 9 | `cortex-knowledge/src/lib.rs:137:25: replace > with >= in SemanticOntologyNode::consolidate` | Equivalent | **Restructured** (overruled): `self.safety_hazard_level.max(hazard_level)`; the test gains the same level and a worse one. |
| 10 | `cortex-linguistic/src/lib.rs:273:29: replace > with >= in LinguisticFrameSlot::mark_indirect` | Test gap | **Tested**: `an_indirect_frame_keeps_its_surface_act_and_says_what_it_means` marks `SPEECH_ACT_EXPRESSIVE`, the last act, and refuses 4. |
| 11 | `cortex-linguistic/src/lib.rs:307:36: replace \|= with &= in LinguisticFrameSlot::apply_face` | Test gap | **Tested**: `tact_softens_bad_news_in_a_courteous_or_formal_register_only` softens a frame whose particle slot was closed and whose metaphor bit was set, and asserts both bits afterwards. The earlier assertion had passed under `&=` because `advance_prosody` had opened the slot first. |
| 12 | `cortex-social/src/lib.rs:152:38: replace * with + in SocialPerspectiveNode::register` | Test gap | **Tested**: `the_register_follows_the_trust` reads courteous at 0.5 and at `0xBFFF`, and pins the familiar threshold at `0xC000`. |
| 13 | `cortex-thalamus/src/lib.rs:62:17: replace > with >= in ThalamicRelayNode::set_gating_mode` | Test gap | **Tested**: `set_gating_mode_refuses_unknown_modes_and_resets_the_counter` switches to `GATING_CLOSED`, the last mode, and relays nothing. |
| 14 | `runtime/cortex-runtime/src/executor.rs:1303:9: replace Executor<CAP>::workers -> usize with 1` | Test gap | **Tested**: the contention test asserts `workers()` on one, two and four. |
| 15 | `executor.rs:1814:23: replace == with != in Worker<CAP>::steal` | Equivalent | **Excluded** beside the existing `steal` entry, with the reason: inverted, the skip leaves a worker trying only its own deque, which its own `pop` has just found empty, so no worker steals, the case the entry already names (ADR-0030: the shares are reported, not asserted). |
| 16–17 | `image.rs:209:9: replace WriteAheadLog::holds -> bool with false`; `:211:33: replace != with ==` | Test gap | **Tested**: `the_log_refuses_a_unit_outside_it_before_writing` asserts `holds` false before an append, true for the logged unit and false for the other, then true for both. |
| 18–21 | `image.rs:253:11`, `:255:14` (`read_at`), `:270:11`, `:272:14` (`write_at`): `delete !`, `== with !=` | Platform-conditional | **The loop moved into one form.** `read_at` and `write_at` are now one platform-independent loop each over a positioned call the platform supplies: `pread`/`pwrite` on Unix (`FileExt::read_at`, `write_at`), `seek_read`/`seek_write` on Windows, under distinct names bound by a `cfg`'d `use … as`. The loops' four mutants meet the log's tests on every runner, and `a_log_entry_the_file_no_longer_holds_in_full_is_an_early_end_not_a_short_record` reaches the early-end branch on every platform. The Windows pair's `Ok(0)`/`Ok(1)` replacements are excluded by name, a pattern that can match nothing a CI runner compiles. The old entry, `(read_at\|write_at) -> io::Result<()> with Ok(())`, is removed: it had also silenced the Unix forms, which Linux compiles and tests, a defect of the entry that this round found and that the new names make impossible to repeat. The Unix loop no longer retries an interrupted call, which `read_exact_at` did: the one branch no test can reach is not written, and an `Interrupted` error reaches the caller like any other. |
| 22 | `runtime/cortex-runtime/src/task.rs:301:26: replace ^ with \| in Task::coin_at` (run `35459078284` only) | Not in the brief | **Answered by ADR-0061** before this round: `the_stimulus_and_the_coin_of_the_first_sixteen_trials_at_seed_27` on `main`, which the run's head did not have. This round changes nothing in `task.rs`. |

The exclusion list is one entry shorter: three removed (`plant_delay`'s `>`, `mul_q16`'s `<`, the I/O `Ok(())`), two added (`steal`'s `==`, the Windows pair), every remaining entry with its function's name and its reason.

### The evidence

**The in-diff gate** of pull request #71 (run [35485126980](https://github.com/DescentVTT/VirtualCortex/actions/runs/35485126980) at `fe476ac`): 43 mutants in the changed lines, 43 caught, in 8 minutes. On a Windows developer machine the same diff gave 20 caught, 2 missed (`pread` and `pwrite`'s `Ok(1)`, the Unix forms that machine does not compile) and 21 unviable on the linker's file lock; the runner is the truth.

**Two weekly sweeps were dispatched on the branch.** The first, [35484280368](https://github.com/DescentVTT/VirtualCortex/actions/runs/35484280368) at `abb2cff`, finished its `crates` job (2 391 of 2 391: 2 307 caught, 0 missed, 7 timeouts, 77 unviable) and found the round's own oracle test hanging under `Walk::advance -> Ok(())`, the rule's second kind met by the test written to hold the rule; the oracles were given their own step (`fe476ac`), the sweep was dispatched again on the same branch, and the second dispatch cancelled the first's runtime shards (they share the concurrency group). The second, [35485126396](https://github.com/DescentVTT/VirtualCortex/actions/runs/35485126396) at `fe476ac`, the head of this round's engine code, is the evidence: **3 456 of 3 456 mutants, every job green, 0 missed**.

| Job | Tested | Caught | Missed | Timeout | Unviable | Job |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| `crates` | 2 391 / 2 391 | 2 308 | 0 | 6 | 77 | 22 m |
| `runtime-0` | 178 / 178 | 167 | 0 | 4 | 7 | 1 h 34 m |
| `runtime-1` | 178 / 178 | 163 | 0 | 4 | 11 | 1 h 33 m |
| `runtime-2` | 178 / 178 | 167 | 0 | 2 | 9 | 1 h 36 m |
| `runtime-3` | 177 / 177 | 166 | 0 | 3 | 8 | 1 h 07 m |
| `runtime-4` | 177 / 177 | 163 | 0 | 3 | 11 | 1 h 55 m |
| `runtime-5` | 177 / 177 | 163 | 0 | 4 | 10 | 1 h 59 m |
| **Whole tree** | **3 456 / 3 456** | **3 297** | **0** | **26** | **133** | |
| `35284722491` (ADR-0058, for comparison) | 3 396 / 3 396 | 3 211 | 21 | 35 | 129 | |

The mutant count moved from 3 396 to 3 456: `crates/**` lost twenty-six, the comparisons the shapes and the ranges removed, less the match arms the range patterns added; `runtime/**` gained the positioned calls, `read_at`'s and `write_at`'s no longer excluded replacements, and brief 027's `task.rs` (979 at ADR-0058, 1 060 at `35459078284`, 1 065 here). No shard came near ADR-0058's three-hour rule.

**The missed list** is empty. Against `35459078284`'s twenty-two, every one is gone: the thirteen under `crates/**`, the eight under `runtime/**`, and `Task::coin_at`, which ADR-0061's test answered before this round.

**The timeout list** is twenty-six against `35459078284`'s thirty-six: the twenty-five the rule reads as inherent, all present (fourteen in the injector, `barrier.rs:35` and `:41`, `stop_workers` twice, `Worker::run`, and the six of the three iterators' `next`), and one of the fifth kind. Gone are the ten directive-class timeouts, every one now caught by the tests the ranges left in place (`stp_decay_factor_q16` three, `deepest` one, `deepest_verdict` two, `Walk::advance` one, the hippocampus three), and `Image::decode`'s `delete !`, caught this time (it is `image.rs:724` in this tree). In its place, and in the same function, `image.rs:564:13: delete field clauses from struct Config expression in Image::decode` timed out in `runtime-3` (1 319 s to 2 320 s into the job): its log ends inside `reference`, whose three waking-day tests had been running for over a minute and which does not test it, and `debug.log` shows the shard's other slot running `Injector::push`'s `< with >` hang from 1 729 s to the end of that window. The same mutant was caught in `35459078284` and timed out in `35444829805` (ADR-0061): the class recurs, not the mutant, which is F-42's finding exactly, and the reason the disposition there is a way of reading a timeout rather than a change to one.

Every job green, no survivor, and no timeout that is not recorded as inherent or as F-42's kind with its evidence. The determinism pin did not move on either architecture (the `arm64` job of run `35485126980`), and the format is 14.

### Consequences

- Good: every survivor of the first complete sweep is dispositioned, none by a reading alone: ten by a test, ten by a shape that has no equivalent (one of them with a test of the byte the shape stores, four of them the I/O loop moved into one form and tested everywhere), one by a named exclusion, and the twenty-second by the ADR before this one.
- Good: the loop directive has a rule, and the rule has been applied to every timeout the sweep has ever listed. Four loops of the engine and one of a test are ranges; the twenty-five inherent hangs are named and left alone, which is what a detection deserves.
- Good: the log's I/O loop is tested on every platform for the first time, and an exclusion that silenced tested code is gone.
- Bad: `stp_decay_factor_q16` no longer exits early on a zero result, so an interval whose factor reaches zero pays the remaining squarings of the base, at most a handful; a result, never, which the oracle test says over 200 000 pairs and the lattice.
- Bad: the Unix log I/O returns `Interrupted` instead of retrying; a caller that meets it retries the append or the read, and no caller in the tree has.
- Neutral: a Windows developer machine sees the mirror image of the exclusion: `pread` and `pwrite` are not compiled there, so a local in-diff run reports their `Ok(1)` replacements missed (this round's local run did, 2 of 43, with 21 unviable on the linker's file lock); CI's runner is the gate's truth (ADR-0030).
- Bad: a slow pass under a spinning neighbour still lands in `timeout.txt`, indistinguishable there from a hang; F-42 says how to tell them apart (the per-mutant log and `debug.log`) and what a next change to the sweep would do. That change is not this round's.

## Alternatives considered and why rejected

- **`#[mutants::skip]` on the inherent loops.** A dependency (`mutants`) in a state crate, TC-2; and the hangs are detections, which ADR-0058 keeps.
- **A percentage or a count threshold on timeouts.** ADR-0030 refused thresholds; a number would pass a directive-class loop as readily as an inherent one.
- **A bound on `Chain::next` and the other iterators' `next`.** The countdown is the bound; a second one would hide the first's deletion, which is the mutant the sweep exists to see.
- **Keeping the `while` forms and excluding their operator mutants.** An exclusion for a timeout hides the loop it locates, and the directive says the loop's shape is wrong, not the mutant.
- **A pattern on the Windows forms' mutant kinds (`delete !`, `== with !=`) under the old names.** It would have been exact on the day and wrong the day a `!` entered the Unix form; the distinct names make the pattern's scope a property of the code, not of the pattern.

## Confirmation

- `.cargo/mutants.toml`: the `steal` entry's second line and the `(seek_read|seek_write)` entry; no entry for `plant_delay`, `mul_q16` or `(read_at|write_at)`.
- The tests named in the dispositions table, in the crates that own the mutants; `mod prop`'s oracle tests in `plasticity.rs` and `fraction.rs`.
- `runtime/cortex-runtime/src/image.rs`: `read_at` and `write_at` without a `cfg`; `pread`, `pwrite`, `seek_read`, `seek_write` with one each.
- Whitepaper §11: F-41, F-42; §11.1's item closed; Appendix B's V-6 row; the executable directives under them.
- The dispatched weekly run named above.
