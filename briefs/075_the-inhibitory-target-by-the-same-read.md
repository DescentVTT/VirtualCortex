---
status: proposed
date: 2026-10-19
affectedFiles: [runtime/cortex-runtime/src/**, runtime/cortex-runtime/tests/**, crates/cortex-connectome/src/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/cortex-core/src/**, crates/cortex-homeostasis/src/**, .github/workflows/**, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, .spec-brief.json, rust-toolchain.toml, Cargo.toml, Cargo.lock]
---

# Brief 075: The inhibitory target by the same read — ADR-0192's question: with a flag set, the engine's read sets the inhibitory rule's period beside the rate's at its end, and the rest of the lead-in settles under the network's own targets, on seeds 22, 23 and 24 at 1 024 units at a sixteenth, H-25's two arms on each; the flag built in the engine at format 24; H-41 run once, its runs dispatched

## Mission

**This brief builds one thing in the engine and runs one hypothesis once.** H-40 read that the engine can read its
own rate target ([ADR-0190](../docs/adr/0190-a-target-the-engine-reads-measured.md)). The inhibitory rule's
target is still the derivation's. The derivation settles every network under the rule's default, 20 000 ticks
(5 Hz), and writes the network's own period only into the arms' images.
[ADR-0192](../docs/adr/0192-the-inhibitory-target-by-the-same-read.md) gives the read a flag: set, the read sets the
inhibitory rule's period beside the rate's at its end, so the rest of the lead-in settles under the network's own
targets.

ADR-0192 wrote **H-41** before any run and **predicts yes**, with what would refute it and three predicted readings.

When the round is done, the tree holds:
- the flag in the engine, unset bit for bit, at the modulator section's `[26]`, the image at format 24;
- on each of the three networks, the derivation under the read with the flag set, and H-25's two arms under it, as
  weekly `exhaustive` tests;
- H-41's verdict per network, arm and clause, and **the step of its stopping rule reached**, with the next decision
  it names and does not take.

**This is the first brief that declares its scope** ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).
`affectedFiles` and `protectedFiles` above are what the round may and may not write.
`spec-brief archive --strict` refuses the round otherwise. `briefs/README.md` says what to do with a file the scope
does not hold.

**Do not run this round beside brief 065's**: both dispatch the weekly workflow.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no
  tool and no version bump. The inhibitory rule and the read are the engine's own, and the flag joins them.
- **The flag is ADR-0192's.**
  - Unset, every run is the run it was, bit for bit, at format 24; an image of format 23 reads as unset.
  - Set beside the read: at the read's last window the inhibitory rule's period becomes the period the read gives
    the rate's, through `set_istdp_target_period`.
  - Set without the read: refused.
  - Set, a read's mean whose period the inhibitory rule would refuse is refused.
  - Its byte is `[26]`; `[27]` stays reserved.
- **The networks are seeds 22, 23 and 24 at 1 024 units, the step a sixteenth, the read H-40's 128 windows.** No
  other network, size, step or span is run.
- **A read refused, a lead-in that does not settle, a rule that gives no value, or a frozen block that fails a
  measure, on a network, is a reading of H-41** (its clause 0). It stops that network, not the round, and it is not
  repaired.
- **H-41's clauses, its constants and its three predicted readings are written before any lead-in is run with the
  flag set**, and do not move after it. There is no second attempt.
- **The runs are dispatches, as `briefs/README.md`'s "Evidencing a round" says.**
  - The calibration excludes the round's own tests.
  - The derivations, the arms and any trial run at `scope=arms`, once.
  - The reproduction is dispatched as soon as the tables are pinned, at `scope=both` since a file under `src/`
    changes, and the readings, documents and archive are written while it runs.
  - A change after the calibration is calibrated again, or shown in the ADR to change no run.
  - No whole-domain test runs on the developer machine.
- **The branch is named `claude/brief-075-…`**, never rebased or force-pushed once a dispatch has run on it. Every
  commit is signed.
- **The prediction is ADR-0192's**: yes. The round reports it against what was read, and each predicted reading as
  held or missed.
- **The engine and the harness are read before a description of them is trusted**, this brief's and ADR-0192's
  included.
- **No capacity of a run's configuration is reached silently.**
- No `f32`/`f64`, oracles included; every operation on a state field saturates or wraps by name
  ([ADR-0029](../docs/adr/0029-structural-enforcement.md)). Every loop ends by construction
  ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)). No `unsafe`.
- A change to a record's or a section's bytes bumps the image's version, updates its table in whitepaper §5.2, and
  gets a changelog entry, in the same pull request.
- A heavy run is an `#[ignore]`d test whose name contains `exhaustive`. The runtime's gate grows by at most one test
  ([ADR-0061](../docs/adr/0061-the-learning-runs-leave-the-gate.md)). A new rule carries a test over the lattice of
  `testkit/prop.rs`, and the pull request's mutation gate catches every mutant in the lines it changes
  ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The pull request leaves draft with the commit that asks for the merge**, which sets the round's ADRs to
  `accepted`.
- **No shard of the weekly job passes 60 per cent of its bound**, 180 minutes, at eighteen shards
  ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md), ADR-0191). One that does is reported with its job's
  minutes.
- No pinned number of an earlier round moves, but the whole-image pins by the format's mask.
- Conventional Commits with a real body; never commit on `main`; the required checks keep their names.

## Context

Re-derived on 2026-10-19 against `main` at `583e56d`, after ADR-0191 merged. Line numbers move; the symbols and the
quoted sentences are what to re-derive.

1. **The inhibitory rule** (`crates/cortex-core/src/dynamics/synapse.rs`):
   - `ISTDP_TARGET_PERIOD_TICKS` is 20 000.
   - `istdp_alpha_q1_15(period)` is the depression a presynaptic spike, 1 343 488 over the period rounded down, the
     period clamped to 100 to 1 000 000.
   - `SynapseBlock::step_stdp` loses it from an inhibitory slot's trace at a presynaptic spike, and gains for each
     pairing with the target's last spike.
2. **The engine** (`runtime/cortex-runtime/src/executor.rs`):
   - `Config::istdp_target_period_ticks`, default 20 000, refused outside 100 to 1 000 000 at `Executor::new`.
   - `set_istdp_target_period` stores the period and publishes `shared.istdp_alpha`.
   - The read (ADR-0189): `tally` calls `read_close` at each window's cadence, which calls
     `HomeostaticDrivePool::hold` then `read_step`. At the last window `ReadStep::Target` sets
     `rate_target_period_ticks`, `rate_target` and ends the read. Nothing touches the inhibitory period.
   - `read_target_of` makes the mean `(sum + 8) >> 4` and the period the units times $2^{17}$ over the mean, to
     nearest.
3. **The image** (`runtime/cortex-runtime/src/image.rs`):
   - The modulator section carries the inhibitory period at `[20..24)`, the rate's at `[56..60)`, and the read's
     state at `[39]`, `[54..56)` and `[60..64)`.
   - `MODULATOR_RESERVED` is `[26..28)`, two bytes. `FORMAT_VERSION` is 23.
   - The loader refuses an inhibitory period out of range with `IstdpPeriodOutOfRange`.
   - `rate_target_of` accepts periods the inhibitory rule refuses: at 1 024 units a mean below about 134 spikes a
     window.
4. **The derivation** (`runtime/cortex-runtime/tests/inhibition.rs`):
   - `derive_under` runs the settling lead-in, the read, the quiet run and the frozen block under the default 20 000.
   - After the frozen block, `target_lead_in_at` counts a frozen copy over `TARGET_LEAD_IN_TICKS` ($2^{20}$) and
     `targeted_image` writes `target_period` into the images the arms decode.
5. **H-40's runs, pinned**:
   - the derivations `the_target_read_at_a_sixteenth_on_seed_{22,23,24}_at_1024_units_exhaustive`, through
     `read_derived`, `read_derivation_held` and `read_pins_held` (`READ_RULES`, `READ_LEAD_IN`, `READ_READ`,
     `READ_MADE`);
   - the arms `the_address_drawn_under_the_target_read_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`,
     each re-deriving its network first;
   - the gate test `the_networks_of_h_40_the_read_s_settling_and_its_predicted_readings_rules`.
6. **H-40's numbers** (ADR-0190):
   - Read periods 57 604, 58 002 and 57 579; inhibitory periods derived under the read 57 512, 58 046 and 57 890.
     All six give a depression of 23.
   - The lead-ins settle at window 160. The inhibitory sum stands at 148.3, 148.8 and 148.2 M at the read's end and
     129.9, 130.9 and 130.2 M at the settling.
   - The arms: mappings 116 to 128 of 128, reversals in 10 to 15 blocks, each arm's count 0.03 per cent above its
     target, the gain within 1.70 and 1.73.
   - The cost table reads the arms at 1 031 to 1 692 s on x86-64 (F-69).
7. **The process** (ADR-0184, ADR-0191, `briefs/README.md`):
   - eighteen weekly shards;
   - `scope=arms` jobs on AArch64, each test's log kept as `arm-<n>`, numbered in the names' sorted order;
   - `spec-brief archive <NNN> --strict` measured from `origin/main`;
   - `npm run spec:scope` prints each widening of `affectedFiles` and fails a shrunken `protectedFiles`.
8. **No sweep is unrecorded**: the last, run 39131009704's, ADR-0191 recorded. This round's own sweep is recorded by
   the first ADR that merges after it ends (ADR-0158).

## Deliverables

- [ ] **The flag, built in the engine, in a new ADR at the next free number (`ls docs/adr`).**
  - ADR-0192's flag at `[26]`.
  - The read sets the inhibitory period beside the rate's at its end, through `set_istdp_target_period`.
  - Refused: the flag without the read, and a mean whose period the inhibitory rule would refuse.
  - The image at format 24, its table in §5.2.
  - Unset, every run the run it was, bit for bit; a format-23 image read as unset.
  - Its tests:
    - by hand at the edges;
    - over the lattice against an oracle;
    - an image written during the read with the flag set reading back to the same run;
    - the inhibitory period set exactly at the read's last window and not before.
  - The whole-image pins moved by the format's mask alone.
- [ ] **H-41's protocol, in a new ADR at the next free number, before any lead-in is run with the flag set.**
  - The networks, the read, the step and the flag.
  - What holds a derivation with the flag set, where none has a pin.
  - H-41's clauses and constants, restated from ADR-0192 and pinned in the tests.
  - The three predicted readings as rules over the tables:
    - each network's inhibitory sum at its lead-in's settling above H-40's 129.9, 130.9 and 130.2 M;
    - each arm's mean count a window within one per cent of its read target;
    - through each arm the gain after every window within 1.6625 and 1.8375.
  - The readings, no clause, as ADR-0192 lists them.
- [ ] **The calibration**, before any such run: a dispatch at `scope=exhaustive` excluding this round's own tests.
  Every pinned number of the tree holds at format 24 with the flag unset, H-40's derivations and six arms among them.
- [ ] **The derivations with the flag set on the three networks**, each a weekly `exhaustive` test, run once by a
  dispatch at `scope=arms`. Pinned from that run:
  - the lead-in by window, the inhibitory sum among it;
  - the read and the period both rules took;
  - what each rule returned, or that it returned nothing;
  - the frozen block's readings, and the frozen copy's period beside the read's.
- [ ] **The arms**: on each network whose clause 0 holds, H-25's two, each a weekly `exhaustive` test, run once by a
  dispatch at `scope=arms`, their tables pinned per block.
- [ ] **The reproduction**, dispatched at `scope=both` as soon as the arms' tables are pinned. Its eighteen
  whole-domain shards are green and reproduce every pinned number, and the cost table is regenerated from them.
- [ ] **The ADR's reading.**
  - H-41's verdict per network, arm and clause.
  - The prediction and each of the three predicted readings against what was read.
  - Each network beside its own arms under H-40, the inhibitory sum's course in the lead-in and the arms among it.
  - Over the three networks: the mappings learned of twenty-four, and the reversals' blocks beside H-40's.
  - **The step of the stopping rule reached, and the next decision it names, not taken.**
  - Each dispatch with its run id and commit, each shard's job time against its bound, and whether the sweep had
    ended.
- [ ] **The gate.** At most one runtime test: H-41's clauses and its predicted readings' rules at their edges, over
  tables written by hand.
- [ ] **The documents, in the same pull request.**
  - Whitepaper: §5.2's modulator table at format 24, §11.1's H-41 with its verdict and step, and §9.
  - The ADR index and `CHANGELOG.md`.
  - `CLAUDE.md` where the standing changes: the image's format, and the result.
  - `README.md` and `docs/zh-TW`'s reader's guide as the result requires.
  - The whitepaper's version in both declarations, with its date.
- [ ] **The brief archived** with `spec-brief archive 075 --strict`, every deliverable dispositioned. The round's
  ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review
  with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into
  `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says: widened in a
  commit of its own when within this brief's empowerment, stopped at when protected or past it.
  - The inhibitory rule's form, the rate's rule, the read's span and the estimate's rule do not change.
  - The whole punishment of ADR-0155 and the exploration of ADR-0163 stay unset, and the feedback is the true one.
- **No change under `src/` but the flag** and what carries it, in the executor and the image. No state crate's
  record changes; `Config`'s default leaves the flag unset.
- The read, the step and the networks are H-40's. None is fitted to the result, and none other is run.
- No rule that derives a constant is changed so that a network's read or settling holds.
- H-41's clauses, its constants and its predicted readings do not move after such a run. There is no second attempt,
  and a run is not repeated for a check of the harness.
- No whole-domain test runs on the developer machine.
- No pinned number of an earlier round moves, but the whole-image pins by the format's mask.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the flag's name in `Config`, its encoding in `[26]`, and where the executor sets the inhibitory period at the
  read's end;
- the error a refused flag or a refused period raises, and where;
- how the harness sets the flag, and how a derivation and an arm carry it, every call of the tree the call it was;
- how a derivation with the flag set on a network with no pin is held: what is asserted by rule, and what is pinned
  from the one run;
- how the three derivations and six arms are dealt into tests and dispatches;
- how each reading is computed and pinned;
- a file the round must change and its scope does not hold, within this brief's mission, widened as
  `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change the read, its span, the step, a network, the task or the schedule;
- move a clause, a constant or a predicted reading after such a run;
- run a whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the calibration fails, the round stops there as H-41's stopping rule says: the finding is written, and no such run
is made.

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

The pull request's gate runs those and the rest: the release tests, the benchmarks once, the MSRV, AArch64 and the
mutation gate on the changed lines. Every check passes on the last commit.

By dispatch, never on the developer machine:

```bash
gh workflow run ci.yml --ref <this round's branch> -f scope=exhaustive -f exclude='<this round's own tests>'
gh workflow run ci.yml --ref <this round's branch> -f scope=arms -f tests='<the derivations, then the arms>'
gh workflow run ci.yml --ref <this round's branch> -f scope=both
node scripts/exhaustive-costs.mjs from <the reproduction's downloaded artifacts> --run <its id> > scripts/exhaustive-costs.tsv
npx --no-install @descent-vtt/spec-brief archive 075 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration's shards are green with the round's tests excluded.
- Each run's job exits 0 with one test passed and its log kept.
- The reproduction's eighteen whole-domain shards are green and reproduce every pinned number.
- In the history, H-41's constants and its predicted readings precede every dispatch that runs a lead-in with the flag
  set, and every commit a dispatch ran on is an ancestor of the pull request's head.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the flag as built, the format, and that every run with it unset is the run it was;
- the protocol, and when it was committed;
- per network, the lead-in's inhibitory sum and its settling beside H-40's, what each rule returned, and the frozen
  copy's period beside the read's;
- H-41's verdict per network, arm and clause, and the prediction and each predicted reading against what was read;
- each network beside its own arms under H-40;
- the step of the stopping rule reached and the next decision it names, not taken;
- each dispatch with its run id and commit: the calibration, the runs and the reproduction;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine, that each run ran once, and that no pinned number moved
  but by the format's mask;
- each shard's job time against its bound and the regenerated cost table;
- that the pull request is ready for review;
- what was not done and why.
