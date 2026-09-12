---
status: proposed
date: 2026-09-12
---

# Brief 019 — Sleep as a state machine and the episodic ledger: two-process regulation on the window cadence, slow-wave replay through the executor, REM depotentiation

> A self-contained prompt for one round of work. Read `briefs/README.md`, then this file, then
> the files it names. Frozen when executed; the outcome lives in the ADRs that accept it, the
> changelog and the code.

## Mission

Answer an architectural proposal ("Biological Memory Continuity, Episodic Event Ledger & Dual-Phase
SWS/REM Ultradian Sleep Substrate", received 2026-09-12) with decisions, code and tests instead
of a document that drifts. The proposal names four frontiers: an append-only episodic event
ledger in `cortex-hippocampus` with a zero-copy path into `cortex-imagination`; slow-wave sleep
as sharp-wave-ripple replay with synaptic downscaling and a sleep-phase clock sweep; REM as
counterfactual recombination with affective recalibration in `cortex-affect`; and an ultradian
state machine in place of the binary sleep flag, scheduled on a cadence, with an emergency wake.
Three of its stated baseline findings are not in the tree (the Context below says which), so
the round begins by re-deriving them. When the round is done: **one ADR** makes sleep a state
machine in `cortex-homeostasis` stepped once per window on the cadence of
[ADR-0035](../docs/adr/0035-cadence-and-the-population-tally.md): a sleep pressure that rises
while awake and falls while asleep (the two-process model, Borbély 1982), a circadian phase
that advances once per window so that its sixteen bits are a day, onset and wake thresholds by
day and night, three stages (awake, slow-wave, REM) alternating under a bounded budget of
windows, and a wake as an input between ticks; with the regulation off by default so that the
reference dynamics are unchanged. **One ADR** admits a second record to `cortex-hippocampus`
under [ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md)'s test: an *episode*, a
tagged pattern of units appended to an index-addressed ledger the executor holds and the image
carries, never overwritten; replayed during slow-wave sleep on a ripple cadence by delivering
the pattern's units a drive that fires them together, so that the three-factor rule of
[ADR-0032](../docs/adr/0032-three-factor-plasticity.md) consolidates the synapses among them
(the transfer the whitepaper's complementary-learning row called Specified); depotentiated
during REM by lowering the episode's tag, which is how many REM ripples it survives; with the
canvas hydration of `cortex-imagination` written down as Specified. Synaptic downscaling as a
weight sweep, affect recalibration in the executor, sensory gating and a sleep-gated clock sweep
are not adopted, with the reasons. Image format 13. The whitepaper, README, `CLAUDE.md`, the
reader's guide, the ADR index and the changelog say all of this, and this brief is archived
with every check green.

## Standing directives

- Every claim is Implemented, Specified, Target or Hypothesis. What replay does to a
  four-unit ring in a test is stated as what it is; what it does at the reference scale is a
  hypothesis with a protocol ([ADR-0010](../docs/adr/0010-measured-or-target.md)).
- The repository wins over the document; a disagreement is a numbered finding in whitepaper
  §11, never a silent edit.
- No `f32`/`f64`; Q16.16 in `i32`/`u32`, widened to `i64` to multiply; every operation on a
  state field saturates or wraps by name (`clippy::arithmetic_side_effects` is denied
  everywhere, [ADR-0029](../docs/adr/0029-structural-enforcement.md)); a relaxation toward a
  target moves by at least one LSB so that the target is reached exactly (§8.1).
- 64-byte `#[repr(C, align(64))]` records with compile-time assertions; no heap types, threads
  or `unsafe` in a state crate; the runtime's arena access is the one `unsafe`
  ([ADR-0023](../docs/adr/0023-executor.md)), every site naming its phase.
- Every quantity has one owner ([ADR-0016](../docs/adr/0016-thirty-two-crate-architecture.md));
  a second record in a crate passes ADR-0016's six-part test in its ADR; the crate count stays
  32.
- A change to a record bumps `CortexFileHeader::FORMAT_VERSION`, updates its §5.2 table and
  gets a changelog entry (rule L-6).
- A run is `(image, seed, input trace)` (§8.3): anything that changes what a run does is in
  the image, always written and required, or is an input between ticks like an injection or
  a reward; never a configuration or a constant a caller can vary without the image knowing.
- A rule slower than the tick runs on a cadence, between ticks on the coordinator or inside a
  phase by the record's holder; no new barrier, no new phase
  ([ADR-0035](../docs/adr/0035-cadence-and-the-population-tally.md)).
- The determinism pin of [ADR-0030](../docs/adr/0030-verification-governance.md) moves only
  with a stated reason; the mutation gate on the changed lines must pass.
- Conventional Commits with a real body; never commit on `main`; the required checks keep
  their names.
- Latest ≠ Newest (§2.1): the two-process model of sleep regulation (Borbély 1982; Daan,
  Beersma and Borbély 1984), the two-stage model of memory consolidation (Buzsáki 1989;
  Diekelmann and Born 2010), REM's affective depotentiation (Walker and van der Helm 2009) and
  the synaptic homeostasis hypothesis (Tononi and Cirelli 2014) are decades old with
  documented failure modes and pass; they are admissible as mechanisms, and adopted only where
  the tree has the input they need.

## Context

Re-derived on 2026-09-12 against `main` at `cb43350`.

- `crates/cortex-hippocampus/src/lib.rs`: `HippocampalAttractorState` is `dg_sparsity_bits`
  `[0..4)`, `ca3_recurrent_energy` `[4..8)`, `ca1_comparator_error` `[8..12)`,
  `swr_replay_ticks` `[12..16)` ("Sharp-wave ripple replay cycle countdown"), `grid_theta_phase`
  `[16..20)`, `place_field_id` `[20..24)`, `_reserved: [u8; 40]`. The crate has one layout test
  and no rule; whitepaper §5.2.15 says "Attractor dynamics and replay: Specified". A countdown is
  what a cadence expresses as a mask on the tick since ADR-0035, so the field is a rule's
  placeholder that the tree already has another form for.
- `crates/cortex-homeostasis/src/lib.rs`: `sleep_mode_active: u8` `[18)` is 0 or 1;
  `circadian_phase: u16` `[16..18)`; `sensory_fatigue: u32` `[4..8)`; `_reserved: u16`
  `[22..24)`. `update_circadian_tick(dt_ticks)` advances the phase by the low sixteen bits of
  its argument and sets the flag when `sensory_fatigue > 0x8000_0000` (32 768.0 in Q16.16, a
  value no rule writes, since nothing writes the field) or the phase exceeds `0xC000`.
  **Nothing in `runtime/cortex-runtime` calls it**: the executor composes the tally, the bin,
  the window and the gain (`Executor::tally`) and no circadian or sleep rule. The proposal's
  "currently a binary flag" is right; its "evolve the flag" understates it: the flag is set by
  a rule nothing runs, on an input nothing produces.
- `crates/cortex-imagination/src/lib.rs`: `MentalCanvasFrame` with `simulation_id` `[0..8)`,
  `hypothetical_action_hash` `[8..12)`, `wander_at(temperature)`, `_reserved: [u8; 23]`
  `[41..64)`; `is_sandboxed` is `motor_release_flag == 0`. A frame is built by its fields; a
  "hydration" from an episode is a choice of `simulation_id` and action hash, and needs no
  code that reads another crate's type (state crates declare no dependencies, TC-2).
- `crates/cortex-affect/src/lib.rs`: `InteroceptiveState::integrate(pain, strain, recovery)`
  takes recovery as its caller's argument; the record is composed by nothing in the runtime.
  `crates/cortex-salience/src/lib.rs`: `evaluate_threat` sets `emotional_tag_priority` to 255
  ("Priority boost for hippocampal SWR replay") and the freeze reflex; composed by nothing.
  `crates/cortex-sensory/src/lib.rs` holds an 8-byte event and a driver trait; no sensory path
  reaches the loop. **The injector ring is the only input to the loop** (§8.5), so the
  proposal's "cortex-sensory salience burst immediately forces AWAKE" can only be a caller's
  call between ticks, as `Executor::reward` is.
- `runtime/cortex-runtime/src/executor.rs`: `Executor::sweep(quiet_ticks, budget)` and
  `sweep_by_policy` run between ticks when the caller calls them; nothing ties them to a sleep
  phase, and compaction does not exist (the slot is zeroed, not reclaimed;
  [ADR-0024](../docs/adr/0024-cortex-image-and-clock-sweep.md)). The proposal's "glymphatic
  clearance compacts memory and evicts cold units to the WAL" describes the sweep's eviction
  and a compaction that is Specified in `cortex-immune`.
- `runtime/cortex-runtime/src/executor.rs`: worker 0 drains the injector in phase 3 and
  delivers each pair with `Worker::deliver` (a pool node, a mailbox push, a schedule); a
  message delivered in phase 3 of tick $t$ is integrated in phase 1 of $t + 1$. The turn holder
  applies the gain to the sums and `integrate` decides the spike; a unit at rest with the
  threshold at its base fires about ten ticks after three messages of about 1.0 (the kicks of
  `tests/criticality.rs`); the message efficacy is 18 bits, at most 2.0 (`spike_message`).
  Phase 2 pairs each fired unit's blocks against the targets' last spikes, settled: two units
  that fire in the same tick pair as $q = t$, potentiation only, no depression
  ([ADR-0022](../docs/adr/0022-synapse-fan-out-and-stdp.md)'s rule as `step_stdp` reads it).
- `runtime/cortex-runtime/src/executor.rs`: `Shared` carries `modulation`, `gain` and `spikes`
  as atomics stored by the coordinator before the tick's first barrier and read by the workers
  after it: the pattern a per-tick replay decision follows. `Config` has eleven fields;
  `tests/differential.rs` and `tests/contention.rs` spell every one out in a literal.
- `runtime/cortex-runtime/src/image.rs`: sections 42 and 43 are always written and required,
  the loader refuses a count other than one and a record the rules would not leave, and the
  image's values outrank the configuration's; section 41 is written when non-empty and its
  arena is sized as the image's count plus `Config::amendments`. The pattern for a control
  record and for an appended arena both exist.
- `crates/cortex-connectome/src/lib.rs`: `FORMAT_VERSION` is 12; section kinds are Appendix A
  row numbers, 43 the last; Appendix A's next rows are 44 and 45.
- `docs/WHITEPAPER.md` §6.6 (R-6, Specified): "`update_circadian_tick` flips
  `sleep_mode_active`. While asleep: the hippocampus replays tagged episodes at compressed
  speed and drives slow neocortical plasticity; the homeostasis controller rescales weights
  toward $\sigma = 1$; the immune scrubber walks `SynapseBlock` arenas …". The second clause is
  ADR-0036's loop, which runs awake or asleep; the third is `cortex-immune`'s and stays
  Specified. §8.8: "Complementary learning systems | `cortex-hippocampus` | Fast one-shot CA3
  attractor; replay during slow-wave sleep into slow neocortical weights | Specified".
- `docs/WHITEPAPER.md` §8.4: the window cadence is $2^{17}$ ticks (1.31 s); sixteen bits of
  windows are $2^{33}$ ticks, 23.86 h at the fine tick: a phase advanced once per window is a
  day to within one per cent, which no other cadence in the tree gives.
- `crates/cortex-core/src/dynamics/synapse.rs`: `STDP_A_PLUS_Q1_15` 328, `STDP_TAU_SHIFT` 11;
  $A_+ (1 - 2^{-11})^{512}$ is 255 in Q1.15, so a pattern replayed every 512 ticks with
  consolidation at 1.0 gains 255 per synapse per ripple after the first (the amount the exit
  test pins). `crates/cortex-core/src/dynamics/membrane.rs`: `REFRACTORY_TICKS` 200,
  `THRESHOLD_STEP` 0.02, `THRESHOLD_DECAY_SHIFT` 12: a unit fired every 512 ticks keeps a
  threshold below 1.2 and is out of its window before the next ripple.
- Preconditions, checked by `spec-guard` until this brief is archived:

<!-- @assert-absence target="crates/cortex-hippocampus" symbol="Episode" word="true" reason="brief 019 precondition: no episode record exists yet" -->

<!-- @assert-count target="crates/cortex-homeostasis" symbol="update_circadian_tick" min="1" reason="brief 019 precondition: the sleep gate is still the rule nothing composes" -->

<!-- @assert-count target="crates/cortex-hippocampus" symbol="swr_replay_ticks" min="1" reason="brief 019 precondition: the replay countdown the cadence supersedes is still a field" -->

## Deliverables

1. [ ] **Sleep regulation** (`cortex-homeostasis`, the first ADR; format 13). `HomeostaticDrivePool`
   `[4..8)` becomes `sleep_pressure_q16: u32` (Process S, in $[0, 1]$; `sensory_fatigue`
   renamed, since the rule now defines it), `[18)` `sleep_stage: u8` (`STAGE_AWAKE` 0,
   `STAGE_SWS` 1, `STAGE_REM` 2; `sleep_mode_active` renamed), `[22)` `sleep_shift: u8` (the
   time constant of the pressure, $2^k$ windows; 0 leaves the stage and the pressure where the
   image put them, the default), `[23)` `stage_windows: u8` (windows in the current stage,
   saturating). Rule `step_sleep() -> u8`, once per window: the circadian phase advances by
   one, wrapping (sixteen bits of windows are 23.86 h); with the shift $k > 0$: awake, the
   pressure rises by $(1 - S) \gg k$, at least one LSB, to at most 1.0; asleep, it falls by
   $S \gg \max(k - 2, 0)$, at least one LSB, to at least 0 (four times as fast, Borbély's
   ratio); then the transitions: awake to SWS when $S \ge$ the onset threshold (0.875 by day,
   0.5 in the night quarter, phase $\ge$ `0xC000`); asleep to awake when $S \le$ the wake
   threshold (0.375 by day, 0.125 at night); SWS to REM after `SWS_WINDOWS` (4) windows in the
   stage and REM to SWS after `REM_WINDOWS` (2), with the wake test first; a stage the constants
   do not name wakes. `wake() -> bool` (an input: the stage to awake, the stage's windows to
   zero, the pressure kept; false when already awake). `is_asleep`, `is_night`. `is_well_formed`
   gains the stage at most 2, the pressure at most 1.0, the shift at most `SLEEP_SHIFT_MAX`
   (15), an SWS or REM record's windows below its budget; the reserved clause goes with the
   bytes. `update_circadian_tick` is removed (its gate was composed by nothing and its fatigue
   threshold unreachable: a finding). Tests at every threshold on both sides, both
   day-and-night pairs, the wrap of the phase, the pressure reaching 1.0 and 0 exactly, the
   budgets, the wake, $k = 0$ moving nothing, the bytes, each well-formedness clause alone,
   and the property walk (every step keeps the record well formed and the pressure in its
   range; the stage is one of three).
2. [ ] **The ledger** (`cortex-hippocampus`, the second ADR; format 13). A second record,
   `Episode`, 64 bytes: `tagged_tick: u32` `[0..4)`, `tag: u8` `[4)` (the REM ripples it
   survives; 0 is spent), `replays: u8` `[5)` (saturating), `len: u8` `[6)`, `_pad: u8` `[7)`,
   `pattern: [u32; 12]` `[8..56)` (unit indices, the first `len` of them; the rest MUST be
   zero), `_reserved: [u8; 8]` `[56..64)`; `PATTERN_MAX` 12; `RIPPLE_SHIFT` 9 (512 ticks, 5.12
   ms, 195 Hz: inside the 150 to 250 Hz band of sharp-wave ripples). Rules: `tag(tick, units,
   priority) -> Option<Episode>` (refused for no unit, more than twelve, a duplicate, a priority
   of zero), `pattern() -> &[u32]`, `replay() -> Option<&[u32]>` (`None` when spent; counts),
   `depotentiate() -> u8` (the tag down by one, saturating), `is_spent`, `is_well_formed`
   (one to twelve units, the unused slots zero, no duplicate, the pad and the reserved bytes
   zero), `encode`, `decode`; `Default` is the empty slot, which is not well formed. The
   pattern and the tick are never written after `tag`; the tag and the count are the ledger's
   annotations. `HippocampalAttractorState` `[12..16)` becomes `episodes: u32` (the ledger's
   length; `swr_replay_ticks` removed, a finding: a countdown the cadence expresses), `[24..28)`
   `replay_hand: u32` (the episode the next ripple considers), `[28..64)` reserved; the six
   Specified fields stay. Rules: `new` (= `Default`), `append() -> Option<u32>` (the next
   index; refused at the width), `next_hand() -> Option<u32>` (the hand, then the hand
   advanced modulo the length; `None` for an empty ledger), `is_well_formed` (the hand below
   the length, or both zero; the reserved bytes zero), `encode`, `decode`. The ADR carries
   ADR-0016's six-part test for the second record. Tests at each refusal, the order kept, the
   spent episode, the tag reaching zero and staying, the hand's wrap, each clause alone, the
   bytes, and a property walk.
3. [ ] **The composition** (`cortex-runtime`, both ADRs). `Config::sleep_shift` (default 0;
   refused above 15, `ConfigError::SleepShiftOutOfRange`) and `Config::episodes` (room for
   tagged episodes beyond what an image holds; default 0; refused at the width). The executor
   holds one `HippocampalAttractorState` and an arena of `Episode`s (in `Shared`, read by
   worker 0 in phase 3, written between ticks), exposed as `hippocampus()`, `episodes()`,
   `sleep_stage()`, `replays()`. Inputs between ticks: `tag_episode(units, priority) ->
   Result<u32, TagError>` (the index; refused for a full ledger, a unit outside the arena, a
   pattern `Episode::tag` refuses) and `wake() -> bool`. On the window cadence, after the
   gain's regulation, `step_sleep`. Before every tick's first barrier the coordinator decides
   the ripple: on `Cadence::new(RIPPLE_SHIFT, 0)`, in SWS the hand walks up to `RIPPLE_SCAN`
   (16) episodes to the first that is not spent, counts its replay and publishes its index in
   `Shared` (index + 1, zero for none); in REM the same walk depotentiates that episode and
   publishes nothing; awake nothing. In phase 3, after the injector, worker 0 delivers each
   unit of the published episode `REPLAY_MESSAGES` (2) messages of `REPLAY_DRIVE_Q16` (1.5),
   the drive that fires a unit at rest within about ten ticks, in the pattern's order: the
   pattern fires together at $t + 1$ and phase 2 of that tick pairs every synapse among its
   units as potentiation, consolidated under the tick's modulation. Sections 44
   (`SECTION_HIPPOCAMPUS`, one record, always written and required) and 45 (`SECTION_EPISODE`,
   the appended episodes, written when the ledger is not empty, required when the record says
   it is not); the loader sizes the episode arena as the image's count plus `Config::episodes`,
   refuses a control record that is not well formed or whose length is not the section's
   (`MalformedHippocampus`), an episode that is not well formed or names a unit outside the
   arena (`MalformedEpisode(index)`), a shift above its bound (`Config(SleepShiftOutOfRange)`),
   a stage, pressure or stage window the rules would not leave (`MalformedHomeostasis`); the
   image's shift, stage and pressure outrank the configuration's. `FORMAT_VERSION` 13 with the
   reason in its list. The pin's `Config` literal gains both fields and the pin holds: with the
   shift at 0 and no episode nothing runs.
4. [ ] **The exit test** (`runtime/cortex-runtime/tests/sleep.rs`, and `tests/image.rs` for
   the loader). The stage machine on the executor equals an oracle record stepped once per
   window, exactly, over a full cycle (awake until onset, SWS, REM, SWS, awake), and one tick
   short of a window boundary nothing has moved. In SWS with the regulation off (the stage as
   the image put it), a tagged four-unit ring is fired together on every ripple, its four
   synapses gain exactly the pair rule's amount per ripple after the first (an oracle block fed
   the same ticks), an untagged ring's synapses do not move, the episode's count equals the
   ripples, and `replays()` equals them. In REM the tag falls by one per ripple, a spent
   episode is skipped, nothing is delivered and no unit fires; `wake` between ticks returns
   the engine to awake with the pressure kept and the next ripple delivers nothing. The whole
   loop (stages, ripples, consolidation) is bit-identical on one and four workers. The ledger
   and the stage round-trip through the image mid-sleep and a loaded engine continues alike.
   Tagging is refused for a full ledger, a missing unit and a bad pattern; the defaults never
   sleep; every loader refusal on its own; the image's sleep state read back.
5. [ ] **Specified and not adopted.** The canvas hydration: a `MentalCanvasFrame` with
   `simulation_id` the episode's index and tagged tick and `hypothetical_action_hash` a hash
   of its pattern, wandered at a temperature, reads the ledger and never writes it; Specified
   in §5.2.32 and §8.8 with the rule, not built, since nothing reads a frame's outcome yet.
   The second ADR says why synaptic downscaling as a weight sweep is not adopted (the
   multiplicative actuator exists as the gain of
   [ADR-0036](../docs/adr/0036-criticality-control.md), whose loop runs asleep as awake; a
   uniform downscale differs from it only in the pair rule's relative step; nothing in the tree
   shows weight saturation under a depression-biased rule; an $O(S)$ sweep is what ADR-0036
   removed), why affect is not recalibrated by the executor (`InteroceptiveState` is composed
   by nothing; its recovery is its caller's argument; the round that composes affect takes
   it), why the injector is not gated during sleep (the trace defines the run; the wake needs
   an input) and why the clock sweep is not tied to a stage (it is the caller's call between
   ticks; `sleep_stage()` is what a caller gates on; compaction is `cortex-immune`'s and
   Specified). The first ADR says why the ultradian alternation is a budget of windows and not
   a completion signal, and why the pressure is time-based and not activity-based (Borbély's
   form; the bin activity is the input a use-dependent variant would take).
6. [ ] **Documents.** Whitepaper §1.6 (the two Logic cells; thirty-eight records), the
   executive summary, §5.2.1 (the executor's status), §5.2.2 (format 13, sections 44 and 45),
   §5.2.15 (both tables, the API, the status, the rules with executable assertions), §5.2.16
   (the table, the API, the status, the rule), §5.2.32 (the hydration), §6.6 (R-6 with what is
   Implemented), §8.4 (the ripple and the circadian cadences), §8.5 (the slow rules), §8.7
   (the sections), §8.8 (the complementary-learning row, a two-process row, a REM row, the
   equations), §9, §11 (the finding), §11.1 (the hypothesis on replay at scale; the budgets and
   thresholds as Targets), Appendix A (rows 44 and 45), Appendix C (M5), Appendix D (Borbély
   1982; Daan, Beersma and Borbély 1984; Buzsáki 1989; Diekelmann and Born 2010; Walker and
   van der Helm 2009; Tononi and Cirelli 2014), the glossary; the ADR index; `README.md`,
   `CLAUDE.md`, the reader's guide (its §11 row is stale at F-27 and H-6); `CHANGELOG.md`;
   this brief archived with every box dispositioned.

## Not empowered

- To add a state crate, a dependency, `#[allow]`, a feature flag or a nightly attribute; to
  add `unsafe` outside `arena.rs`; to add a barrier or a phase.
- To let a sleep rule touch weights (a sweep over blocks in the loop), or to put the shift,
  the thresholds, the budgets, the ripple or the drive in the amendment registry of
  [ADR-0031](../docs/adr/0031-policy-amendment.md): they change what the engine does.
- To keep the ledger or the stage out of the image, or to write the control section only
  sometimes: they change results (the review finding of PR #44).
- To drop or gate injections by the stage: the trace is the run's definition.
- To change the tick, the wheel, the bin or the window: the ripple and the circadian step are
  cadences on the existing clock.
- To move a pin without the ADR's reason and the test that shows the boundary.
- To decide the `mmap` path, core pinning, the broker, the lexicon, affect's composition or
  per-column records.

## Architectural empowerment

- The executing round may choose the thresholds, the budgets, the shift's ratio, the pattern's
  width, the ripple's shift, the scan's bound and the drive, with the reason in the ADR and a
  test at each boundary; the ripple MUST stay above the refractory window and the pressure's
  arithmetic MUST reach its bounds exactly.
- It may make the pressure use-dependent (the bin's activity as its input) if it shows a run
  on which the time-based form fails a stated property; Borbély's form is the default.
- It may replay by a priority order instead of the hand's round robin if it shows an $O(1)$
  form; a scan over the whole ledger per ripple is not one.
- It may put the replay's constants in `cortex-hippocampus` instead of the runtime if it
  names why a state crate should carry a message drive that `cortex-core` encodes.
- It may leave the one-and-four-worker test out if the ripple's delivery cannot be made
  worker-independent, and must then say why and what was measured instead.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo bench -p cortex-bench --bench hot_path --locked -- --test
cargo +1.85 check --workspace --all-targets --locked
cargo +1.85 test --workspace --locked
npm run spec
git diff main...HEAD > target/pr.diff && cargo mutants --workspace --in-diff target/pr.diff
```

Every command exits 0; the last one reports no survivor in CI (a local Windows run marks
mutants whose build failed as unviable and is not the gate's truth). `npm run spec:deps` still
reports no dependency in a state crate. The determinism pin holds at `0x1724f3486c1d674e`
with 95 spikes, or the ADR says why it moved. The three precondition directives above are gone
with the archived brief.

## Report

The closing message states: which of the proposal's premises the tree contradicted and where
each correction went; the record layouts and the byte accounting of both crates; the stage
machine's thresholds, budgets and cadence with the reason for each number; the ledger's rules
and the ripple's schedule; what the exit test holds with the numbers it reached (the pressure
at onset, the windows of each stage, the weight gained per ripple); the format version;
whether the pin moved and why; what the mutation gate found on the changed lines and how each
survivor was answered; what was not done (the downscaling sweep, affect, sensory gating, the
sleep-gated sweep, the canvas hydration, per-column records) and why.
