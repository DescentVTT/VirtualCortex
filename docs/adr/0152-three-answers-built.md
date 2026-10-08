---
status: accepted
date: 2026-10-08
depends-on: ADR-0151
decision-makers: VirtualCortex maintainers
---

# ADR-0152: Three answers, built — ADR-0151's selection and mapping in the task: `Readout`, `Task` and `Outcome` generic over the number of channels, two unless the type names another; each channel's indirect drive the largest of the other channels' counts, so the largest count alone is selected and a shared largest selects nothing; an answer for each stimulus in the flag's place, moved to the next readout by `Task::flip`; an answer that names no readout refused; a readout of more than 64 channels refused by the compiler; no rule of the engine, no record and nothing of the image changed, and under two readouts a trial is the trial it was and a flip the flip it was, held against the task as ADR-0148 left it written out as an oracle

## Context and Problem Statement

[ADR-0151](0151-three-answers.md) decided a choice among three answers and left its placement to brief 062. Its decision:
- *"Each channel's direct drive is its own set's count. Its indirect drive is the largest of the other channels' counts, and its hyperdirect drive is zero. `compute_gating` decides each channel, unchanged."*
- *"A channel is selected when its count is above every other's. When the largest count is shared, no channel is selected, as at a tie between two."*
- *"An answer for each stimulus, a readout's index, in place of the flag."*
- *"At a flip every stimulus's answer moves to the next readout, the last to the first. With two readouts that is the flip in place."*

Brief 062's empowerment names the choices: a readout and a task generic over the number of channels or a three-channel form beside the two-channel one, how the answers and the flip are held, and how the harness's schedule drives them. This ADR is the build. The readouts' deal, H-29's protocol and its run are the measurement's, in the ADR that follows this one.

What was read first (principle 2, and the brief's directive that the engine is read before a description of it is trusted), on 2026-10-08:

1. **The readout** (`runtime/cortex-runtime/src/task.rs`): `Readout` held `sets: [Set; 2]` and `channels: [BasalGangliaChannelState; 2]`. `select` wrote each channel's own count as `striatal_d1_drive` and the other's as `striatal_d2_drive`, called `compute_gating` on both and matched the two results: one channel, *"or none when neither is or both are"*.
2. **The gate** (`crates/cortex-basal-ganglia/src/lib.rs`): `compute_gating` releases a channel when `striatal_d2_drive + stn_hyperdirect_drive − striatal_d1_drive` is below zero, saturating. Between two counts as drives, at most one channel is released, so the match's *"both are"* was never reached.
3. **The mapping**: `Task::mirrored`, a `bool`, read by `Task::answer` as `stimulus ^ 1`. Outside `task.rs` the field was written in five places: the harness's `task` and `probe_task`, `run_on_scheduled`'s flip (`task.mirrored = !task.mirrored`), `tests/learning.rs`'s task and `tests/inhibition.rs`'s `hold_probe`.
4. **The hold** ([ADR-0144](0144-the-gates-output-built.md)): `Readout::hold` sends its messages into every channel whose `gpi_snr_inhibition` the selection left above zero. Between two that is the channel not selected, and neither at a tie.
5. **What names a count's shape**: `Outcome::counts: [u32; 2]`, and `Readout::count` and `count_window`, which return it. The critic's expectations are per stimulus, and there are still two stimuli.
6. **The channel's record**: `channel_id` is documented as an index `0..63`.

## Decision Drivers

- ADR-0151's decision and brief 062's standing directives: one mechanism, a third channel in the task's readout and an answer for each stimulus; no change to `cortex-basal-ganglia`'s gate, the critic, its window, the address, the modulator, the signed gate, STDP, the membrane or the inhibitory rule; under two readouts, bit for bit.
- **One composition, not two.** A three-channel form beside the two-channel one would be a second writing of the trial, and the second step of ADR-0151 would need a third.
- **A structural boundary beats a reviewed one** (principle 5): what the type can hold is the compiler's to refuse.
- Latest ≠ Newest (§2.1): const generics with a default are stable since Rust 1.59 and an inline `const` block since 1.79, both below the workspace's floor of 1.85 ([ADR-0009](0009-rust-edition-and-msrv.md)). Nothing is adopted.
- The mutation gate ([ADR-0030](0030-verification-governance.md)): every mutant in the changed lines is caught by a test.

## Considered Options

1. **The shape**: (a) `Readout<const N: usize = 2>`, `Task<const N: usize = 2>` and `Outcome<const N: usize = 2>`; (b) `Readout3` and `Task3` beside the two-channel types; (c) a readout over a slice, its length read at run time.
2. **The selection's result**: (a) the channel the gate released, at most one by the rule; (b) the two-channel match generalised, none where more than one is released.
3. **The mapping**: (a) `answers: [u8; 2]`, a readout's index for each stimulus, in the flag's place; (b) the flag kept for two readouts and the answers added beside it.
4. **The flip**: (a) `Task::flip`, a method of the task; (b) the harness's arithmetic on the field.
5. **A readout with too many channels**: (a) refused by the compiler, at 64; (b) refused by `Task::check`; (c) not bounded.
6. **The hold among more than two**: (a) the gate's reading as it stands, every channel whose net output is above zero; (b) no hold where nothing was selected; (c) a hold refused on a task of more than two readouts.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

- **The shape** (`runtime/cortex-runtime/src/task.rs`, option 1(a)): the number of channels is a parameter of the three types, two unless the type names another. `Readout::new` takes one set a channel and gives each channel its index as its id; `count` and `count_window` return one count a set; `Outcome::counts` holds them. Every caller that names `Readout`, `Task` or `Outcome` names the two-channel type, and no caller outside `task.rs` changed for the shape. Two types (option 1(b)) would write the trial twice. A slice (option 1(c)) would move a mismatch between the sets, the counts and the answers from the compiler to a refusal.
- **The selection** (`Readout::select`, option 2(a)): each channel's direct drive is its own count, its indirect drive the largest of the other channels' counts — zero for a channel with no other — and its hyperdirect drive zero; `compute_gating` on each, unchanged. The gate releases a channel whose drive is above every other's, so it releases at most one: the selection is that channel, or none.
  - **With two channels** the largest of the others is the other's, so every channel holds the drives, the output and the flag it held, and the selection is the one it was.
  - **Why not the match generalised** (option 2(b)): a branch for more than one released channel cannot be reached, as the two-channel match's could not, and a mutant of it could not be caught.
  - **At the width.** A count is a drive of one a spike, saturating at the `i32`. Two counts beyond it tie, as before; `Task::check` refuses a readout set whose count could reach it.
- **The mapping** (option 3(a)): `Task::answers`, a readout's index for each stimulus. `[0, 1]` and `[1, 0]` are the flag's two values. `Task::answer(stimulus)` reads the field. The same answer for both stimuli is a mapping, not refused: what a task asks is the caller's. A flag kept beside the answers (option 3(b)) would be two fields that can disagree.
- **The flip** (`Task::flip`, option 4(a)): every answer moves to the next readout, the last to the first. With two readouts each answer moves to the other, the flag negated. The harness's `run_on_scheduled` calls it where it negated the flag.
- **The refusals.**
  - `TaskError::AnswerOutsideReadout`, from `Task::check`: an answer at or beyond the number of channels, either stimulus's. With the flag no such task could be written. It is read after the sets and the stimuli and before the trial's length.
  - **`MAX_CHANNELS`, 64**, the width of a channel's index. `Readout::new` asserts it in an inline `const` block, so a readout of 65 channels does not compile (option 5(a)), and a selection always fits the `u8` an outcome records. A run-time refusal (option 5(b)) could not cover `Readout::select`, which is public and takes no task.
  - Every other refusal reads the readout's sets as it read two: empty, malformed, outside the arena, sharing a unit with any other set, a count beyond the width.
- **The hold among more than two** (option 6(a)): `Readout::hold` is unchanged, the gate's own reading. Among three it holds both channels below a selected one. Where two share the largest count it holds the third, though nothing was selected, since that channel's output is above zero; where all three share it, none. H-29 carries no hold, and [ADR-0146](0146-the-tag-the-address-already-is.md) left the hold unset. A rule of its own for a tie (option 6(b)) or a refusal (option 6(c)) would each be a rule added for a case no run asks for.
- **The deliveries** read the selection as they did: the selected readout's units as the targets under the addressed and the drawn delivery, none where nothing was selected, every unit under the released and the global one.
- **Nothing else moves.** No rule of the engine or of a state crate, no record, no byte of the image, no format. The critic stays per stimulus.

### The harness

- `run_on_scheduled` (`tests/instrument/harness.rs`) flips by `Task::flip`. `answers_of(mirrored)` gives the harness's `task` the flag's two values as answers; the two probes and `tests/learning.rs`'s task write theirs out.
- Nothing else of the harness changed for the build. Its blocks, its oracles and its tallies are the two-readout ones; what reads three is the measurement's.

### The tests

- `a_trial_under_two_readouts_is_the_trial_it_was_and_a_flip_the_flip_it_was`: `Task::trial` and `Task::flip` against the task as [ADR-0148](0148-a-reward-right-seven-times-in-eight-built.md) left it, written out in the test as the oracle — two channels composed by hand, each one's own count its direct drive and the other's its indirect, the mapping a flag negated at a flip, the counts a filter over the train, the hold a walk over the two channels. Twin engines, twelve trials with a flip before the fifth and the ninth, from either mapping, under a drive, a cancel and a sub-window: under each of the four feedbacks, with the task's critic and the addressed delivery, the engine's critic and the drawn one, the engine's critic and the released one under a hold, and no critic under the global one. The same outcome, channels, answers, units' fields, train, messages drained, addressed set, signal, counts, expectations and weights after every trial.
- `the_selection_is_the_largest_count_alone_over_the_lattice` (over `testkit/prop.rs`): among three, for every triple of a lattice of nine counts — within the width, at it and beyond it — and 4 096 seeded triples of small counts, the selection is a hand rule and every channel's drives, output and flag are the rule's; each channel is selected in 189 triples of 729 and none in 162. Among two, for every pair of the `u32` lattice, the selection and both channels are those of the composition before this ADR, written out.
- `a_selection_among_three_is_the_largest_count_alone_and_none_where_it_is_shared`: the cases by hand, with the channels' fields; counts of 10, 6 and 6 select the first, which the sum of the others would not ([ADR-0151](0151-three-answers.md)'s option 2(b)); one channel, no channel, and 64.
- `a_trial_among_three_selects_the_cued_readout_and_judges_it_by_the_answer`: a trial's three counts, its selection and its outcome under five mappings.
- `a_flip_moves_every_answer_to_the_next_readout_and_the_last_to_the_first` and `a_flip_is_the_next_readout_by_the_remainder_over_every_mapping`: H-29's two arms' four mappings each, in three flips; among one, two, three, four and 64 readouts, every pair of answers against the remainder of one more over the number, and as many flips as readouts bringing the mapping back.
- `every_refusal_is_named_with_three_readouts`: thirty-three refusals of `Task::check` met by a task of three, each also refused by `trial` before any tick — the third readout's set empty, malformed, outside the arena, sharing a unit with each of the four other sets and beyond the width, and an answer that names no readout, either stimulus's; among two, the first index past two.
- `each_delivery_addresses_the_third_readout_when_it_is_selected_and_none_at_a_tie`: under each of the four deliveries, the addressed sources and targets when each readout is selected, when two share the largest count and when all three do.
- `the_gate_holds_every_channel_below_the_largest_count_among_three`: the hold's messages into both channels not selected, into the third where two share the largest, and into none where all share it.

**Every run under two readouts is the run it was, bit for bit.** The whole-domain tests reproduce their pinned numbers in the measurement's calibration and in the weekly dispatched on its branch; the measurement's ADR records both.

### Consequences

- Good: one composition of the trial for any number of channels up to 64, and the second step of ADR-0151 adds no selection.
- Good: a selection that does not fit its record, and a readout whose counts and sets disagree in number, do not compile.
- Good: a mapping is data, and a flip is one rule the task and the harness share.
- Bad: `Task` grew a refusal the flag could not need.
- Bad: among more than two the hold reaches a channel at a tie of the two above it. No run carries a hold there; a round that does decides whether that is what it wants.
- Neutral: the three types carry a parameter. A type written without it is the two-channel one, so a three-channel task is always written as one.
- Neutral: `Task::answer` indexes the answers by the stimulus, 0 or 1, where it took any `u8`. The trial's stimulus is one bit of its draw.

## Alternatives considered and why rejected

- **Two types** (option 1(b)) **or a slice** (option 1(c)): see above.
- **The match generalised** (option 2(b)): see above.
- **The flag kept beside the answers** (option 3(b)): see above.
- **The harness's arithmetic on the field** (option 4(b)): the rule would be written where the task cannot be held to it.
- **A run-time bound or none** (options 5(b) and 5(c)): see above; with no bound a selection past 255 would be recorded as another channel's.
- **A rule or a refusal for the hold** (options 6(b) and 6(c)): see above.

## Confirmation

- `runtime/cortex-runtime/src/task.rs`: `Readout<N>`, `Task<N>`, `Outcome<N>`, `MAX_CHANNELS`, `Readout::select`, `Task::answers`, `Task::flip`, `TaskError::AnswerOutsideReadout`, and the tests named above; `runtime/cortex-runtime/src/lib.rs` exports the bound.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `answers_of`, the flip in `run_on_scheduled`, and `probe_task`'s answers. `runtime/cortex-runtime/tests/learning.rs` and `tests/inhibition.rs`'s `hold_probe`: the task's answers.
- Whitepaper §6.5 and §9; `CHANGELOG.md`.
