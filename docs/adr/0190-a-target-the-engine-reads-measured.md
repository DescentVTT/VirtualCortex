---
status: proposed
date: 2026-10-18
depends-on: ADR-0189
decision-makers: VirtualCortex maintainers
---

# ADR-0190: A target the engine reads, measured — H-40's protocol, written before any lead-in is run with the read set: on seeds 22, 23 and 24 at 1 024 units, at a sixteenth, the gain held for 128 windows and the mean count of the last sixteen the target; after the read, H-39's settled network's rule over the windows the rule moved the gain in, and a network settled only where the read made a target; a derivation under the read held by its rules, by the gain held's lead-in through the read's windows and by its first run's tables; an arm before its one run held by its rules alone; the three predicted readings as rules over the tables; the readings follow the run

## Context and Problem Statement

[ADR-0188](0188-a-target-the-engine-reads.md) wrote H-40 before any run and predicts yes: with the engine holding its gain for 128 windows from its first and taking the mean count of the last sixteen as its target, then running the rate's rule at a sixteenth, on seeds 22, 23 and 24 at 1 024 units, clause 0 and H-25's four hold on every network. [ADR-0189](0189-a-target-the-engine-reads-built.md) built the read. Brief 074 runs H-40 once, its runs dispatched as [ADR-0184](0184-the-runs-off-the-developer-machine.md) has them, and asks that the span, H-40's clauses, its constants and its three predicted readings be committed before any lead-in is run with the read set.

**This ADR is written in two parts.** The protocol below is committed first. The readings are added after the run of each derivation and each arm, and nothing of the protocol moves then.

**No sweep has ended unrecorded**: the last dispatched, run [38417864748](https://github.com/DescentVTT/VirtualCortex/actions/runs/38417864748)'s, [ADR-0183](0183-a-sixteenth-on-three-networks-measured.md) recorded, and no dispatch since ran one.

What was read first (principle 2), on 2026-10-18, against the build's commit `356d10d`:

1. **The read as built** (`runtime/cortex-runtime/src/executor.rs`). `Config::rate_read_windows` at 128 starts the read with 128 windows left. Each window's end holds the gain (`HomeostaticDrivePool::hold`), counts one window down and sums the windows with sixteen or fewer left, 113 to 128. At the 128th the engine holds the period of the mean and that period's count, the read none, the rate's rule from the 129th. A mean the rule refuses ends the read refused, the gain held from there on.
2. **The settled network's rule under the rule** (`tests/inhibition.rs`, `settled_in_mean`). It finds the first window at which ADR-0055's criterion holds over the excitatory sums and the gain's mean of sixteen windows is within 0.75 per cent of the sixteen before. **Read over a lead-in under the read whole, it would settle every network inside the read**, before the rule had moved the gain once: through the read the gain is one number, so its means agree from the 32nd window, and the sums settle where the gain held's do, at windows 96, 106 and 99.
3. **The gain held's lead-ins** (`BACKGROUND_LEAD_IN_1024[SETTLED]`, `LEAD_IN_SEEDED`): 96, 106 and 99 windows on seeds 22, 23 and 24, all inside the read's 128. Under the read the gain is held as there, and the estimate is stored by the slope as there, the saturation rule never reached (the greatest window of the three holds 2 487 spikes, the rule's count 32 768). So the read's windows are those lead-ins, window for window, as far as each ran, and nothing of the read's last sixteen is pinned anywhere.
4. **H-39 on the three networks** (`RATE_RULES[1]`, `SEEDED_RATE_RULES`): under the rule from the first window the lead-ins settled at windows 96, 105 and 93.
5. **The arms' tables** (`arm_on`). An arm under the true reward is held to its pinned tables after it has dumped them. Under ADR-0184 a run is a job that must exit 0 with one test passed, so an arm whose tables are not yet pinned would fail its own run at its first pin.

## Decision Drivers

- ADR-0188's protocol and brief 074's standing directives: three networks, one step, one span; the read as ADR-0189 built it; no rule that derives a constant changed so that a read holds; no constant moved after such a run, and no second attempt.
- **The settled network's rule reads the controller it settles**: windows in which no rule moved the gain cannot show that the rule holds it.
- **A derivation on a network with no pin is held by its rule and its first run's tables**, and by what of it the read does not reach, which is pinned already (ADR-0183's form).
- **A check of the harness is tried first by a dispatch**, before the run it guards (brief 074; ADR-0181's arms ran twice for a check that was not).
- **Every derivation and every arm of the tree is the one it was**, H-39's three derivations and six arms among them.
- **A run runs once and on the repository's host**, and its job exits 0 with its log kept (ADR-0184).
- A heavy run is a weekly test; the gate grows by one test ([ADR-0061](0061-the-learning-runs-leave-the-gate.md)).

## Considered Options

1. **The settled network's rule under the read**: (a) H-39's, over the windows after the read, from the sums and the gain the read's last window left, the window counted from the lead-in's first; (b) H-39's over the whole lead-in, at the first window past the read; (c) H-39's over the whole lead-in.
2. **A read that makes no target**: (a) no settled network, folded into the rule's window, so clause 0 is a no and the derivation stops; (b) a rule of its own beside the others.
3. **What holds a derivation under the read**: (a) its rules — what it makes before the settled network and its cancel the gain held's, the read's windows the gain held's lead-in as far as it ran, the target the hand rule's of the read's sixteen windows, every window after the read the rate's rule's — and its first run's tables where pinned; (b) its first run's tables alone.
4. **An arm before its one run**: (a) held by its rules alone, every table of it unpinned, so that its run exits 0; (b) held to empty tables, its run failing at its first pin with its dump kept.
5. **The dispatches**: (a) the calibration with the round's tests excluded, beside a trial of the harness; the three derivations; then the six arms, once their networks' derivations are pinned; (b) the derivations and the arms in one dispatch.
6. **Predicted reading 1's target**: (a) a window's count, the engine's beside the derivation's; (b) a period.
7. **What an arm is read beside**: (a) the same network's two arms under H-39; (b) its two with the gain held.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).**

### The protocol (written before any lead-in with the read set)

- **The networks** (`READ_SEEDS`): ADR-0044's generator at 1 024 units from seeds 22, 23 and 24, H-39's.
- **The step** (`READ_STEP`): a sixteenth, `0x1000`, on every network.
- **The read** (`READ_SPAN`, `READ_LAST`): 128 windows from the lead-in's first, the last sixteen summed; no target in the configuration (ADR-0189 refuses one beside a read).
- **The settling** (`UNDER_THE_READ`): H-39's at a sixteenth with the read in the target's place — the step and the read in the candidate's configuration and in the image it reloads from; the lead-in's 128 windows of held gain first; then H-39's settled network's rule, the excitatory sums by ADR-0055's criterion and the gain's mean of sixteen windows, **read over the windows after the read**, from the excitatory sum and the gain the read's last window left, the window counted from the lead-in's first (option 1(a), `Settling::settled`); within 320 windows in all. The earliest window it can settle at is the 160th, both blocks of sixteen gains windows the rule moved the gain in. *Why not the whole lead-in* (options 1(b), 1(c)): read whole, the rule compares the gain the rule moved with the gain the read held, and settles a network at the 129th window, one window after the rule began, or inside the read before it began.
- **A network settles only where the read made a target the rule accepts** (option 2(a), `derive_under`): a read still under way or refused at the lead-in's end leaves no settled network, which is clause 0's first part, and the derivation stops. The derivation records what the read left — under way, refused, or none — and the period and the count the engine then held (`Derivation::read`). The image it writes carries that period at `[56..60)` and no read; the inhibitory rule's target stays the derivation's at `[20..24)`.
- **Then H-25's rules**, as `derive_under` runs them; a rule that returns nothing stops the derivation there.
- **What holds a derivation under the read** (option 3(a), `read_derivation_held`):
  - what it makes before the settled network, and its cancel where it reached it, are the network's derivation's with the gain held: on seed 22 `before_the_settling_held` and `cancel_held`, on seeds 23 and 24 `settling_apart` against H-33's pins;
  - **the read's windows are the gain held's lead-in, window for window** — every spike, both readouts, both sums, the units at the target, the gain and the estimate — as far as that lead-in ran, 96, 106 and 99 windows (`gain_held_lead_in`), and no window of the read moved the gain;
  - **what the engine made of the read is the hand rule's of its sixteen windows** (`read_target_by_hand`, ADR-0188's text in `i128` by doubling): the period and its count with the read ended, or, where the hand rule refuses, the read refused;
  - every window after the read is the rate's rule's against that count (`rate_read` over them, its `unruled`), and no window passed the train's capacity; no worker dropped an entry;
  - and where its one run is pinned, every table of it (`read_pins_held`), its images at format 23.
- **The arms** (`read_arm`, `read_site`): H-25's two on H-20's schedule on each network whose clause 0 holds, from the image its derivation wrote, under the true reward, the step and the read target carried through every trial; each its own weekly test. An arm's calibration is its network's derivation run again and held as the derivation's own test holds it, to its pinned tables (`read_made`); on a network whose clause 0 does not hold there is no arm.
- **What an arm is read beside** (option 7(a), `read_stand`): the same network's two arms under H-39 (`sixteenth_site`), so each network stands beside its own arms under H-39, as ADR-0188's readings ask.
- **H-40's clauses** are ADR-0188's, by the rules the tree holds, in H-39's form (`read_verdict`, which is `sixteenth`): clause 0, `Rules::hold` with the read folded into the settled window; clauses 1 to 4, `drawn_of` against the image that network's derivation wrote — at least 80 of each mapping's last 128 trials correct; no coupling past 1.30 of its image's; each reversal passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's. **Yes** when all five hold in both arms on all three networks; a network whose arms did not run is no yes.
- **The stopping rule's steps** (`read_stop`): 3, yes; 4, a no on clause 0 on a network; otherwise 5.
- **The prediction**: yes (`READ_PREDICTED`).
- **The three predicted readings as rules** (`read_predicted`), each predicted to hold (`READ_READINGS_PREDICTED`):
  1. each network's read target within five per cent of its derived one, seed 22's below it: the read's count, the engine's, against `SIXTEENTH_COUNTS` — 2 306, 2 255 and 2 268 — twenty times their distance at most the derived count, and seed 22's strictly below (`read_target_within`, option 6(a)). A count below is a rate below, as ADR-0188 wrote it of seed 22's live count. A network whose read made no target does not hold it;
  2. in each of the six arms the mean count a window within one per cent of its read target: the windows' spikes summed against as many times the target, a hundred times their distance at most it (`count_within`, from the arm's `GainRead`);
  3. through each of the six arms the gain after every window within five per cent of 1.75, 108 954 to 120 422 both counted (`gain_within`, by the arm's `gain_readings`).

  The second and the third are read over the arms that ran, and are none where none did.
- **Readings, no clause**, as ADR-0188 lists them: each network beside its own arms under H-39 — each mapping's last 128, each reversal's blocks, the highest coupling, a window's count and its scatter, the gain's band and mean, the inhibitory sum's course, and the value and its troughs; per network the read's sixteen windows, the read target beside the derived one, the lead-in's window of settling beside H-39's, and the inhibitory target period beside H-39's; over the three networks the mappings learned of twenty-four, and the reversals' blocks beside H-39's.

### The harness (built before any such run)

- **`Settling` carries the read** (`Settling::read`, zero in every settling the tree held before), and `Settling::settled` reads the windows after it (`settled_over`, the rule as it was). `candidate_under` sets it in the candidate's configuration and holds the engine to it.
- **`RuledSite` says whether it is a read's network** (`RuledSite::read`): `ruled_pins`, `ruled_stand` and `ruled_made` take H-40's (`read_rate_pins`, `read_stand`, `read_made`) where it is set; every site the tree held before has it unset.
- **An arm before its one run** (option 4(a), `Ground::unpinned`): an arm of a read's network whose every table is empty and whose every reading is none is held by its rules alone — the calibration, the critic's step, the train, the window's count against the record's, every window's gain the rule's, no worker's entry dropped — and its dump is the run's record; its run exits 0. An arm of a read's network pinned in part is refused, and every other arm of the tree is held to its tables as it was.
- **The derivations** (`read_derivation`, `read_derivation_held`, `read_derived`), each a weekly test — `the_target_read_at_a_sixteenth_on_seed_{22,23,24}_at_1024_units_exhaustive` — dumped whole before anything of it is held, their tables empty until their one run (`READ_*`).
- **The six arms**, `the_address_drawn_under_the_target_read_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, their tables empty until their one run (`READ_ARM_*`).
- **The trial of the harness**, `the_read_s_first_eight_windows_on_seed_22_are_the_gain_held_s_at_1024_units_exhaustive`: the candidate under the read on the instrument's prior run for eight windows, each held row for row to ADR-0077's lead-in with the gain held, the read under way with 120 windows left and nothing summed, no window moving the gain. It reads nothing of H-40 — no window of the read's last sixteen — and stays a weekly guard of the read's first windows.
- **The gate** (one test, `the_networks_of_h_40_the_read_s_settling_and_its_predicted_readings_rules`): the networks, the step, the span, the bound, the settling against H-39's, the arms and the prediction; the settled network's rule under the read over tables written by hand — a read whose sums drift past the tolerance settles nothing and the first settled window is the 160th, one short is none, a read whose windows would settle the whole-lead-in rule at its 32nd still settles at the 160th, a step of one per cent after the read settles at the 166th, a gain's mean apart by one per cent at the 165th; `read_target_by_hand` at the three derived counts, a half either side, no spike and a short read; `read_read` over a table by hand; the three predicted readings' rules at their edges; the verdict and the stopping rule's steps over tables written by hand; and, over H-40's pinned tables, each network's derivation by its rules, each arm's gain by block against its reading, and the verdict, its step and the three readings as written, none before every derivation and every arm of each network whose clause 0 holds is pinned (`read_tables`). It runs no lead-in.

### The dispatches (option 5(a))

1. **The calibration**: `scope=exhaustive` on the protocol's commit with this round's ten tests excluded. Every pinned number of the tree must hold at format 23 with the read unset, H-39's three derivations and six arms among them; if one does not, the round stops there as a finding (ADR-0188's stopping rule, step 2).
2. **The trial**, beside the calibration: `scope=arms` with the trial alone. It is a check of the harness, not a run of H-40.
3. **The derivations**: `scope=arms` with the three, once the calibration's shards are green and the trial passed. Their tables are pinned from their logs and committed.
4. **The arms**: `scope=arms` with the six, on the commit that pins their networks' derivations, which each arm's calibration holds. *Why not one dispatch* (option 5(b)): an arm's stand is its network's pinned image, and an arm whose calibration held nothing pinned would be a second derivation's run with no table to reproduce.
5. **The reproduction**: `scope=both` once the arms' tables are pinned (a file under `src/` changed, [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)).

### How a check added to an arm was tried before the runs

- **`arm_on` gains one branch and no check**: `Ground::unpinned` is false for every ground the tree held before — the instrument's, every `Site`, and every `RuledSite` with the read unset — so the calibration runs every pinned arm of the tree, H-38's two and H-39's six among them, through the branch it took, which holds every table as it did.
- **The ground of a read's network** reads its arms beside from `sixteenth_site`, the tables H-39's arms are held to every week, and its derivation through `read_made`, which holds the derivation as its own test does.
- **The checks a derivation under the read is held by are tried before its run**: the settling's rule, `read_target_by_hand` and `read_read` in the gate over tables by hand; the read's windows against the gain held's lead-in on seed 22 by the trial's dispatch, eight windows; `settling_apart`, `before_the_settling_held` and `cancel_held` every week on H-39's derivations.
- **What cannot be tried before** is a derivation under the read itself, past its eighth window: its one run is its trial.

### The readings

Added after the one run of each derivation and each arm.

### Consequences

- Good: the settled network's rule under the read says that the rule holds the network, not that the read held it.
- Good: a derivation under the read is held through its read by tables earlier rounds pinned: every window of the gain held's lead-in on each network.
- Good: an arm's first run is a run the host passes, its log the record, so a round's arms need no second run to be pinned.
- Bad: `arm_on` gains a branch that holds nothing pinned; it is refused for an arm half pinned, and taken by no arm but a read's before its run.
- Neutral: three derivations, six arms and one trial of eight windows join the weekly job, about 11 600 s.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `READ_SEEDS`, `READ_STEP`, `READ_SPAN`, `READ_LAST`, `READ_BOUND`, `READ_NAMES`, `UNDER_THE_READ`, `READ_ARMS`, `READ_PREDICTED`, `READ_READINGS_PREDICTED`, `READ_TARGET_DIVISOR`, `READ_COUNT_DIVISOR`; `Settling::read`, `Settling::settled`, `settled_over`, `candidate_under`, `derive_under`, `Derivation::read`; `ReadMade`, `read_target_by_hand`, `ReadRead`, `read_read`, `read_target_within`, `count_within`, `ReadArm`, `read_arm_of`, `read_predicted`; `read_verdict`, `read_stop`; `gain_held_lead_in`, `read_rate_pins`, `read_derivation`, `read_derivation_held`, `read_pins_held`, `read_derived`; `RuledSite::read`, `read_site`, `read_stand`, `read_made`, `read_arm`, `Ground::unpinned`; the trial, the gate's test and `read_tables`; the three derivations' and the six arms' weekly tests.
- Whitepaper §11.1 (H-40) and §9; `CHANGELOG.md`; `docs/adr/README.md`.
