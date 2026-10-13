---
status: accepted
date: 2026-10-13
depends-on: ADR-0175
decision-makers: VirtualCortex maintainers
---

# ADR-0177: The controller's step, measured — H-37 is no, on clause 0 at every step, as ADR-0175 predicted: on seed 22 at 1 024 units with the criticality controller on from the lead-in's first window, at a sixteenth, one in 32 and one in 64 the gain alternates about the saturation rule's rate and the lead-in settles in its gain at no window of 320; at one in 128 and one in 256 it settles, at the seventy-first and the 114th window, and the frozen block fails both measures, the volley 48.5 and 48.7 units of 51 a presentation and the stimulus seen in 46 and 49 trials of 64; every step holds the network at 24.4 Hz a unit, a gain of 2.49 to 2.50 and an excitatory sum at 0.45 of the prior's, half its windows at the estimate's ceiling, and a smaller step narrows the gain's band, 2.295 to 2.654 down to 2.488 to 2.507, and does not lower it; the three predicted readings each held, the estimate reading 0 to 0.498 at every rate below the saturation rule's over 1 145 windows and near one in none; the derivation given how it settles its network (`derive_under`), every derivation of the tree the call it was; no arm run, no file under `src/` and no pinned number changed; its stopping rule reached step 5, the step is not the lever, and the next decision, an ADR on what the controller regulates, its estimate or a rate, is named and not taken

## Context and Problem Statement

[ADR-0175](0175-the-controllers-step.md) closed the line under an unreliable reward and asked the operating regime of the one lever that needs no build, the criticality controller's step. It wrote **H-37** before any run and predicted no, on clause 0, at every step of the scan. Brief 071 runs H-37 once and changes no file under `src/`. This ADR is the round's protocol, committed before any lead-in is run under a step of the scan, and then its readings.

What was read before anything was written (principle 2, and brief 071's directive that the engine and the harness are read before a description of them is trusted, the brief's and ADR-0175's included):

1. **The controller** (`HomeostaticDrivePool::regulate`, `crates/cortex-homeostasis/src/lib.rs`). Once a window it takes an estimate, the window's lag-one slope or `SIGMA_MAX_Q16` for a window `is_saturated`, stores it in the record, and moves the gain by $g \leftarrow g\,(1 - \kappa\,\operatorname{clamp}(\hat\sigma - 1, -1, 1))$, each product rounded to nearest and the gain clamped to `GAIN_MIN_Q16` and `GAIN_MAX_Q16`. **It stores the estimate at a step of zero too**, which is why the gain held's lead-in holds one a window. The executor names the ceiling, `saturation_ceiling(units)` in `runtime/cortex-runtime/src/executor.rs`: one spike a unit a bin, 32 768 spikes a window at 1 024 units.
2. **Where the derivation writes its step** (`derive_at`, `runtime/cortex-runtime/tests/inhibition.rs`). In two places: the candidate's record, `candidate_from(&p, units, gain, BACKGROUNDS[SETTLED])`, and the assertion that the frozen image carries it. `frozen_image` patches the modulator's baseline and nothing else, so the homeostasis record goes into the image as the lead-in left it, its gain and its step with it; `frozen_from` decodes under a configuration whose step is zero and the image's outranks it. **So a step given the candidate is carried through the frozen block, the target's lead-in and every image written from the settled one, with no further change.**
3. **The gain the image carries.** `derive_at` asserts the image's gain is the gain's rule's, which holds with the gain held and at no step above zero. Under a step the image carries the gain the lead-in's regulation left.
4. **The cancel does not read the step.** Its candidates are probed and run on the prior's network at the gain's rule's gain, each on an engine made by `at_gain(&p, config(units, 2, 0), gain)`, whose step is zero. That is *"the cancel, derived as it is"* of ADR-0175.
5. **ADR-0077's second candidate was held to the sums alone.** `background_candidate(1, …)` runs `lead_in_until_settled` within `LEAD_IN_EXTENDED`, 160 windows, and its lead-in ends at the thirty-fifth, where ADR-0055's criterion holds over the excitatory sums (`SETTLED_AT_1024[1]`). Over its gains the criterion holds at no window of the thirty-five (`CONTROLLER_READ`'s second clause). **Under ADR-0175's rule for the scan, sums and gains within 320, a derivation at an eighth would not stop at the thirty-fifth window**, and the frozen block ADR-0175's calibration names, *"from that candidate's thirty-fifth window"*, would not be reached.
6. **The first window under any step is the gain held's.** The controller moves the gain at a window's end, so the first window of every lead-in runs at 1.75 and reads what `BACKGROUND_LEAD_IN_1024[SETTLED]`'s first row reads, the gain after it apart.
7. **ADR-0175's sentences about the two pinned lead-ins, read by rule** (`step_read`, `estimate_by_rate`, below) over `BACKGROUND_LEAD_IN_1024`. Each holds:

   | | The gain held | An eighth |
   | :--- | ---: | ---: |
   | Windows | 96 | 35 |
   | The criterion first holds over the sums, the gains, both at one window | 96, 16, 96 | 35, none, none |
   | The gain first falls at window | never | 6 |
   | The gain's band from it | — | 2.210 to 2.805 |
   | Spikes a window from it | — | 16 385 to 54 398, 12.2 to 40.5 Hz |
   | Windows at the estimate's ceiling | 0 | 14, the sixth to the thirty-fourth |
   | Windows at the saturation rule's count | 0 | the same 14 |
   | The estimate below the ceiling | 0 to 0.451 | 0 to 0.409 |
   | Windows with an estimate between a half and two | 0 | 0 |
   | The most spikes of a window | 2 462 | 54 398 |
   | The last window | 2 308 spikes | 19 757 spikes, gain 2.472 |
   | The excitatory sum, of the prior's | 0.925 | 0.453 |

   **In all 131 windows the gain after the window is the controller's rule's from the gain before it and the window's estimate**, by an oracle written from the rule (`regulated`): the table's gains and estimates are one record.
8. **The capacities of `config`** (`tests/instrument/harness.rs`), and what a population firing ten to twenty times faster can do to each:
   - **the train**, a ring of 335 872 spikes at 1 024 units, a trial's most by the refractory window, which lets go of its oldest. A lead-in's window is eight trials long: by the same bound it can hold 2 684 928 spikes, eight times the ring. **This is the one capacity a run could pass without the process failing by itself**, and `lead_in_window` asserts at every window that the ring let go of nothing younger than the window, which is that the window's spikes are at most the capacity. A trial's reading cannot pass it: the task refuses a train below a trial's bound;
   - **the injector ring**, 4 096 entries, emptied once a tick: what a run injects before one tick is the drive's eight messages and the stimulus's, and does not read the rate. A refused injection fails the run (`run_driven(…).expect`, `Task::trial(…).expect`);
   - **the mailbox pool**, 65 536 nodes a worker: a pool exhausted aborts the process (`Worker::deliver`), as a full slot of the wheel does;
   - **a worker's trace**: `config` keeps none, so no entry of one can be dropped, and a worker's report says zero.

**No sweep is owed.** ADR-0174 records the schedule's weekly one (run 38234325824); brief 070's dispatch (38307712843) was `scope=exhaustive` and ran none; and no weekly run was dispatched or scheduled between it and this protocol, looked for at 01:52Z on 2026-10-13.

## Decision Drivers

- Brief 071's standing directives: no file under `src/` and no file of a state crate changes; one network, seed 22 at 1 024 units; the five steps in their order and no other; the controller on from the lead-in's first window; with the step at zero the derivation returns seed 22's constants and at an eighth its lead-in is ADR-0077's second candidate's; a step at which a rule gives no value is a reading and is not repaired; H-37's clauses, constants, bound, criterion and predicted readings committed before any lead-in under a step of the scan and not moved after it; no second attempt; no capacity reached silently; no float; every loop ends by construction; the gate grows by at most one test; no pinned number of an earlier round moves.
- **Every derivation of the tree must be the derivation it was**: seed 22's, H-33's two, H-34's two, and the twelve arms that derive their network first are in the calibration and in the weekly job.
- **One body for a derivation**, so that what a step changes is the step and not a second copy.
- **ADR-0077's tables are reached by ADR-0077's rule.** A calibration that reaches a pinned table by another rule has not reproduced it.
- **A reading is a rule over a table**, written before the table exists.
- Latest ≠ Newest: no dependency, no tool, no rule of the engine; the controller is ADR-0036's as built.

## Considered Options

1. **How the derivation takes its step**: (a) a form given how it settles its network, `derive_under(units, seed, &Settling, name)`, which `derive_at` calls with the gain held; (b) a step added to `derive_at`'s arguments and to every call of it; (c) a second derivation for a step.
2. **How the settled network's rule reads the gains beside the sums**: (a) the first window at which the criterion holds over both, at that one window; (b) the later of the two windows at which each first holds; (c) the gains alone.
3. **How the calibration at an eighth reaches ADR-0077's tables**: (a) the rule and the bound are the settling's, and at an eighth they are ADR-0077's, the sums alone within 160 windows; (b) under the scan's rule, its first thirty-five windows held to ADR-0077's; (c) `background_candidate` alone, as the weekly job already runs it.
4. **The capacities under the controller**: (a) each as it is, the train asserted window by window and its greatest window pinned, with what follows a loud failure written here; (b) the train sized to a window's bound for the scan's lead-ins.
5. **How the scan is dealt into tests**: (a) one weekly test a step and one for the calibration at an eighth; (b) one test for the five.
6. **The estimate by the rate of its window**: (a) by bands of an eighth of the saturation rule's count, every window of the lead-ins read; (b) a list of every window; (c) by the gain.
7. **The arms**: (a) built when a step holds, on `arm_on` with a `Site` given its settling, and not built where none holds; (b) built before the scan whether or not they can run.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).** Everything from here to the order of the work was committed before any lead-in was run under a step of the scan.

### The form (`derive_under`, option 1(a))

- **How a derivation settles its network is a value** (`Settling`): the controller's step its lead-in runs under and its image carries; whether the settled network's rule reads the gains beside the sums; and the lead-in's bound in windows.
  - `GAIN_HELD`: the step zero, the sums alone, `LEAD_IN_EXTENDED`. It is every derivation to brief 070.
  - `AT_AN_EIGHTH`: ADR-0077's second candidate, the step an eighth, the sums alone, `LEAD_IN_EXTENDED`.
  - `under_step(step)`: a step of the scan, the sums and the gains, `SCAN_BOUND`.
- **`derive_under` is `derive_at`'s body** with the settling's step where it wrote `BACKGROUNDS[SETTLED]`, in the candidate's record and in the assertion on the image, and the settling's rule and bound where it wrote `settled_within` and `LEAD_IN_EXTENDED`. `derive_at(units, seed, name)` is `derive_under(units, seed, &GAIN_HELD, name)`, and no call of the tree changes.
- **The lead-in** is `lead_in_until`, `lead_in_until_settled_over`'s loop with the rule given, a `for` over the bound; `lead_in_until_settled_over` calls it with the criterion over the sums.
- **The gain the image carries** is read from the engine after the quiet run, asserted the gain's rule's where the step is zero, asserted carried by the image at every step, and is what a derivation's configuration holds (`Made::gain`), which an arm asserts of the engine it decodes. With the gain held it is the value it was.
- **Two readings are added to what a derivation holds** and to no pinned table of an earlier round: the gain and the estimate the image carries, and the two after the frozen block.
- **Every derivation's settling engine is shut down** where the derivation leaves it and every worker's report held to no entry dropped (`undropped`), as an arm's is.

### The settled network's rule under a step (option 2(a))

- **ADR-0055's criterion at one window** is `settled_in(prior, series, window)`: each of the sixteen windows up to and including it within 75 per ten thousand of the value after the window sixteen before, the prior's for the sixteenth; strict; false below the sixteenth window and past the series. `settled_within` is the first window at which it holds, and is the rule it was: the criterion's body moved under a name, held by ADR-0077's gate at every edge it was held at.
- **Under a step** the rule is `settled_in_both`: the first window at which `settled_in` holds over the excitatory sums and over the gains, **both at that window**, within the bound; nothing when no window is. The gain before the first window is the gain's rule's.
- **With the gain held it is the rule over the sums**: every gain is the prior's and the criterion over the gains holds at every window from the sixteenth. Over ADR-0077's gain-held lead-in both rules return the ninety-sixth window.
- **What every later rule returns when nothing passes.** A derivation stops at the first rule that returns nothing, as it always has: every rule after it holds nothing, every table after it is empty, and clause 0 does not hold for that step. A network that does not fall quiet inside `QUIET_BOUND` gives the settled network's rule nothing to stand on, as on every network since brief 067.
- **A step holds** when `Rules::hold` is true of its derivation: every rule gave a value and the frozen block passed both measures, the stimulus firing once and the stimulus seen in `SEEN_MIN` trials of 64.

### The calibration, before any lead-in under a step of the scan (H-37's stopping rule, step 2)

1. **Every pinned number of the tree holds**: every whole-domain test of the tree, 114 before this round, from the committed build.
2. **With the step at zero the derivation returns seed 22's constants**: `the_rules_return_the_tree_s_constants_on_seed_22_at_1024_units_exhaustive`, which is `derive_under` with `GAIN_HELD` and holds each rule's return, every table and the CRC of every image on the way to H-25's. H-33's and H-34's four derivations and the twelve arms that derive first run through the same form.
3. **At an eighth** (`the_derivation_at_an_eighth_is_the_controller_s_candidate_at_1024_units_exhaustive`): `derive_under` with `AT_AN_EIGHTH` runs ADR-0077's second candidate's lead-in window for window, settles at its thirty-fifth, runs its quiet run, and the frozen block from that window is ADR-0077's frozen run, block, trace, composition, every trial's counts and the census, reading the two measures ADR-0077 pinned, both failed. The derivation stops there and writes no configuration. What it makes before the settled network and its cancel are seed 22's.

A pinned number that moves stops the round there, as a finding, and no lead-in is run under a step of the scan.

### H-37, restated as integer rules (ADR-0175's clauses and constants)

- **The network**: ADR-0044's prior at 1 024 units from seed 22 (`SCAN_UNITS`, `PRIOR_SEED`).
- **The steps** (`SCAN_STEPS`), Q0.16, in this order: `0x1000`, `0x0800`, `0x0400`, `0x0200`, `0x0100`. The largest is first, so the first that holds is the largest that holds.
- **The bound** (`SCAN_BOUND`): 320 windows. ADR-0175's reason for the smallest step is held by the controller's rule: from 1.75, at an estimate of zero, a step of one in 256 reaches the gain's ceiling inside 320 windows and its half does not.
- **Clause 0** by step: `Rules::hold` of `derive_under(1024, 22, &under_step(step), …)`.
- **Clauses 1 to 4**, at the step taken, in both arms: H-25's by H-25's rules (`drawn_of`) against the four couplings and the outside sum of the image that step's derivation wrote — at least `REWARDED_MIN` (80) of each mapping's last 128 correct; no coupling past `BOUND_PER_CENT` (130) of the image's; each reversal passing `CROSSING_MARK` (40) of 64 within `REVERSAL_BLOCKS_MAX` (23) blocks; the outside sum within three and five quarters of the image's.
- **The verdict** (`controlled`): clause 0 by step; the step taken; clauses 1 to 4 where one is taken and its arms ran; yes when a step is taken and all four hold in both arms.
- **The step of the stopping rule** (`controlled_step`): 3, yes; 5, no step holds; otherwise 4.
- **The prediction** (`CONTROLLED_PREDICTED`): no, on clause 0, at every step.

### The three predicted readings, as rules (`scan_predicted`)

Over the five lead-ins, each predicted to hold (`SCAN_READINGS_PREDICTED`):

1. **every lead-in holds a window whose estimate is at its ceiling**, `SIGMA_MAX_Q16`, which the saturation rule gives it. The windows holding the saturation rule's count are read beside it;
2. **no window of any lead-in has an estimate between a half and two**, `0x8000` to `0x2_0000` (`NEAR_ONE`). **Both ends are counted**, the reading least kind to the prediction: ADR-0175 names such a window as what would refute it;
3. **every lead-in's last window holds more than five times 2 308 spikes**, 11 540; exactly that many is not more. The 2 308 is read from the gain held's ninety-sixth window.

Over ADR-0077's two lead-ins the rules read true, true, true at an eighth, which is where ADR-0175 predicts them from, and false, true, false with the gain held.

### The readings, no clause

- **Per step** (`StepRead`), beside the gain held's and an eighth's, which the table above holds:
  - the lead-in's windows, and the window the criterion first holds at over the sums alone, over the gains alone and over both;
  - the first window whose regulation lowered the gain, and from it the gain's band and the least and the most spikes of a window;
  - the windows at the estimate's ceiling — how many, the first and the last — and those at the saturation rule's count;
  - the windows with an estimate between a half and two;
  - the windows whose gain is not the controller's rule's from the gain before and the window's estimate, which is none while the record and the oracle agree;
  - the most spikes of a window and the highest gain;
  - the last window's spikes, its readouts' spikes, its gain and its estimate;
  - the inhibitory and the excitatory sum after the last window, of the prior's.
- **The estimate by the rate of its window** (`estimate_by_rate`, option 6(a)): every window of the lead-ins read, by bands of 4 096 spikes a window, an eighth of the saturation rule's count and 3.05 Hz a unit; per band its windows, those at the ceiling, those between a half and two, and the least and the greatest estimate below the ceiling. It is read per step and over the five.
- **The frozen block, where a step reaches it**: the block as every derivation holds it — the volley's units a presentation, the readouts' spikes before the injection and after the volley, the trials seen — its composition with ADR-0072's sign, every trial's counts and the census, and the gain and the estimate after it.

### The capacities (option 4(a))

- **No capacity of `config` is changed**, so a lead-in at an eighth is ADR-0077's by construction and every earlier run is as it was.
- **The train**: asserted at every window read from it; each step pins the most spikes of a window of its lead-in, and its test holds that number to the capacity. At an eighth the greatest window is 54 398 of 335 872, a sixth.
- **An injection refused** fails the test; **a worker's report** is held to no entry dropped.
- **What follows a loud failure**, written before any run: a lead-in under a step whose test fails the train's assertion, or whose process aborts on the mailbox pool, is run again with that one capacity sized by what the run can put in it — the train to a window's bound, 2 684 928 spikes — and the ADR says so. The windows before the failure are the same by the engine's determinism, and no constant of H-37 moves.

### The arms, where a step holds (option 7(a))

- **The step taken** is the first of `SCAN_STEPS` whose pinned `Rules` hold.
- **An arm** is `arm_on(Ground::Site(…), None, arm)`: H-25's arm under the true reward on a `Site` whose derivation is `derive_under` with that step's settling, held to the step's pinned derivation, then the network's own first block, then 7 680 trials from the image the derivation wrote, the controller on through every trial. The gain and the estimate are read at every trial's end beside the critic's step, and pinned per block.
- **Where no step holds, no arm is built and none is run.**

### The gate

One test, `the_settled_network_s_rule_under_a_step_clause_0_of_h_37_and_the_predicted_readings_rules`. It runs no lead-in and holds:

- the steps, their order, their bound, the network and the three settlings;
- the controller's rule as an oracle at its edges, and ADR-0175's reason for the smallest step by it;
- ADR-0055's criterion at one window at its edges, and `settled_within` as the first window at which it holds, over tables written by hand and over ADR-0077's two lead-ins;
- the settled network's rule over sums and gains at its edges: the gain held; a gain that settles late; sums that settle late; each alone at some window and never both at one; exactly 0.75 per cent; a series shorter than sixteen;
- over ADR-0077's two lead-ins, the gain held settling where it did by either rule and an eighth settling in its sums and not in its gains;
- clause 0, the verdict, the step taken and the stopping rule's three steps over tables written by hand;
- the three predicted readings' rules at their edges;
- the estimate by rate and the lead-in's reading at their edges, and both over ADR-0077's two lead-ins, pinned;
- the train's capacity, the greatest window at an eighth inside it and a window's bound past it.

### The weekly tests and their cost

Six tests, each `#[ignore]`d and named `exhaustive`: the calibration at an eighth and one a step, `the_controller_s_step_of_{a_sixteenth, one_in_32, one_in_64, one_in_128, one_in_256}_at_1024_units_exhaustive`. A lead-in under the controller reads about two seconds a window on a hosted runner (`the_controller_at_1024_units_exhaustive`, 80 s over forty-three windows of lead-in and frozen block), so a step that runs to its bound is about 650 s and the six about 3 400 s, on a table of 81 785 s: under 34 per cent of a shard's 180 minutes as dealt.

### The order of the work, as the history holds it

1. The form, the rules, the gate's test and this protocol, committed and pushed. Before that commit, from the release build of this tree: the gate's test; the derivation at zero on seed 22 through the form; and the derivation at an eighth.
2. The calibration: every whole-domain test of the tree from the committed build.
3. The five steps, each run once.
4. The arms, where a step holds.
5. The tables pinned, the gate's checks over them, the dispatch.

### The readings

Everything below this heading was written after the scan. Nothing above it moved after the first lead-in under a step of the scan was run: the steps, the bound, the criterion, H-37's constants and the three predicted readings are the protocol's, at `93f7b5b`.

**The calibration held.** From the build of `93f7b5b`, each test in a process of its own, eight at a time, from 02:03:53Z to 05:22:38Z on 2026-10-13: the 114 whole-domain tests of the tree before this round and the calibration at an eighth, 115 in all, each exit 0 with one test reported.
- **Every pinned number of the tree holds through the form.** H-33's and H-34's four derivations and the twelve arms that derive their network first reproduced their tables through `derive_under` with the gain held.
- **With the step at zero the derivation returns seed 22's constants**: each rule's return, every table and the CRC of every image on the way to H-25's.
- **At an eighth it is ADR-0077's second candidate**: the lead-in window for window, settled in its sums at the thirty-fifth; the quiet run, 2 541 ticks; the frozen block, block, trace, composition, sixty-four trials' counts and the census; both measures failed. The image carries a gain of 2.472 and the frozen block leaves it at 2.306, the two gains ADR-0077's comment names.

**The scan, each step run once**, side by side from the same build, from 05:23:19Z to 05:31:15Z. Each test ended at its first pin, its tables being empty, after everything was dumped; the tables are pinned from those dumps (`SCAN_*`) and the dispatch reproduces them.

#### H-37's verdict: no, on clause 0, at every step

| Step | The settled network's rule | The quiet run | The cancel | The frozen block: fires once, seen | Clause 0 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| A sixteenth | nothing in 320 windows | — | — | — | no |
| One in 32 | nothing in 320 windows | — | — | — | no |
| One in 64 | nothing in 320 windows | — | — | — | no |
| One in 128 | the 71st window | 2 542 ticks | six messages, seed 22's | no, no | no |
| One in 256 | the 114th window | 2 552 ticks | six messages, seed 22's | no, no | no |

- **At the three larger steps the lead-in does not settle in its gain.** The criterion holds over the excitatory sums from the thirty-seventh, the forty-first and the fifty-first window, and at 283, 279 and 270 windows of the 320; over the gains it holds at none. The derivation stops there, as the rule written first says, and nothing after it is run.
- **At the two smaller steps it settles**, at the first window at which the criterion holds over the gains, the sums having settled at the sixty-eighth and the 107th. The network falls quiet, the cancel's rules return seed 22's, and **the frozen block from each image fails both measures**: the volley short and the stimulus not seen. The derivation stops there and writes no configuration.
- **No step holds, so none is taken and no arm is run** (`CONTROLLED_1024`, `SCAN_STOPPED_1024`). Clauses 1 to 4 are not read.

**The prediction held** (`CONTROLLED_PREDICTED`): no, on clause 0, at every step. Its reason held too, as far as these tables read it: *"a smaller step narrows the swing about that rate and does not lower it."*

**What ADR-0175 did not foresee**: that a step small enough would pass the settled network's rule. At one in 128 and one in 256 the gain's swing, two steps wide, fits inside the criterion's 0.75 per cent either side of its reference, so the rule reads a gain that alternates as settled. The frozen block then answers what the rule could not.

#### The three predicted readings: each held

| | Predicted | Read (`SCAN_READINGS_READ_1024`) |
| :--- | :--- | :--- |
| 1 | in every lead-in the estimate at its ceiling in one window at least | **held**: 149, 149, 144, 11 and 8 windows, in each lead-in exactly the windows that hold the saturation rule's count, 32 768 spikes or more |
| 2 | no window of any lead-in with an estimate between a half and two | **held**: none of 1 145 windows |
| 3 | every lead-in's last window above five times 2 308 spikes | **held**: 42 021, 33 267, 33 397, 33 002 and 33 204 |

**The second held by 114 parts in 32 768.** The greatest estimate below the ceiling in the five lead-ins is 0.498 (32 654 in Q16.16, where a half is 32 768), in the third window at a sixteenth, at 5.7 Hz, while the gain is still rising. The next are 0.382, the third window at one in 32, and 0.343. The rule counts a half, and 0.498 is not a half; the reading is recorded as held and as near.

#### Per step, beside the gain held and an eighth

| | The gain held | An eighth | A sixteenth | One in 32 | One in 64 | One in 128 | One in 256 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Windows | 96 | 35 | 320 | 320 | 320 | 71 | 114 |
| The criterion first holds over the sums | 96 | 35 | 37 | 41 | 51 | 68 | 107 |
| over the gains | 16 | none | none | none | none | 71 | 114 |
| The gain first falls at window | never | 6 | 8 | 15 | 26 | 51 | 100 |
| Windows from it | — | 30 | 313 | 306 | 295 | 21 | 15 |
| The gain's band from it | 1.750 | 2.210 to 2.805 | 2.295 to 2.654 | 2.419 to 2.581 | 2.458 to 2.540 | 2.483 to 2.515 | 2.488 to 2.507 |
| Its width, in steps of the mean gain | — | 1.9 | 2.3 | 2.1 | 2.1 | 1.6 | 2.0 |
| The mean gain from it | 1.750 | 2.485 | 2.492 | 2.497 | 2.498 | 2.498 | 2.499 |
| A unit's rate from it, Hz | 1.6 to 1.8 | 12.2 to 40.5 | 17.1 to 32.5 | 20.5 to 28.6 | 22.4 to 26.4 | 23.7 to 25.3 | 23.8 to 24.9 |
| The mean rate from it, Hz | 1.7 | 24.8 | 24.2 | 24.4 | 24.4 | 24.5 | 24.5 |
| Windows at the ceiling | 0 | 14 | 149 | 149 | 144 | 11 | 8 |
| of the windows from the first fall | — | 47 % | 48 % | 49 % | 49 % | 52 % | 53 % |
| The estimate below the ceiling, from the first fall: zero in | — | 11 of 16 | 110 of 164 | 104 of 157 | 96 of 151 | 5 of 10 | 5 of 7 |
| its mean | — | 0.038 | 0.033 | 0.031 | 0.039 | 0.056 | 0.005 |
| The greatest estimate below the ceiling | 0.451 | 0.409 | 0.498 | 0.382 | 0.322 | 0.287 | 0.317 |
| The most spikes of a window | 2 462 | 54 398 | 43 631 | 38 376 | 35 439 | 33 960 | 33 427 |
| The last window's spikes | 2 308 | 19 757 | 42 021 | 33 267 | 33 397 | 33 002 | 33 204 |
| Excitatory sum, of the prior's | 0.925 | 0.453 | 0.452 | 0.451 | 0.450 | 0.450 | 0.450 |
| Inhibitory sum, of the prior's | 0.775 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |

The rates are a window's spikes over 1 024 units and the window's 1.31072 s. The rows to the last window's spikes are `step_read`'s, pinned (`HELD_READ`, `EIGHTH_READ`, `SCAN_READ`), with the windows from the first fall, the means and the zeros read from the pinned lead-ins.

- **Every step takes the network to one place**: the saturation rule's rate, 24.4 Hz a unit, at a gain of 2.49 to 2.50, with the excitatory sum at 0.450 to 0.452 of the prior's and the inhibitory sum at the prior's. It is where an eighth takes it, and the excitatory sum is the one [ADR-0055](0055-a-weight-that-settles.md) read under the controller after eighty windows, 0.452.
- **The step sets how wide the gain swings there.** The band is about two steps of the mean gain wide at every step, 1.6 to 2.3: 0.360, 0.162, 0.083, 0.032 and 0.020 of gain.
- **From its first fall the gain goes up a step and down a step in turn.** Half the windows are at the ceiling, 47 to 53 per cent, and the runs of windows on either side are one window long, 1.0 to 1.4 on average and never more than two. In a window below the saturation rule's count the estimate reads zero in two windows of three at the three larger steps and 0.06 at most on average at every step, so the gain rises by nearly the whole step; in a window at it the gain falls by the whole step.
- **So the controller holds the network where its saturation rule fires, and that rule is what holds it.** Below that rate the estimate asks for more gain at every rate read, and nothing but the saturation rule ever asks for less.
- **The step sets how long the rise takes**: the gain first falls at the 8th, 15th, 26th, 51st and 100th window, twice as late for half the step, from a gain of 2.45 to 2.54.
- **The gain's course and the rate's**, after the window named:

  | Window | A sixteenth | One in 32 | One in 64 | One in 128 | One in 256 |
  | ---: | ---: | ---: | ---: | ---: | ---: |
  | 1 | 1.828, 1.8 Hz | 1.789, 1.8 Hz | 1.770, 1.8 Hz | 1.760, 1.8 Hz | 1.755, 1.8 Hz |
  | 4 | 2.097, 7.5 Hz | 1.928, 4.7 Hz | 1.839, 3.0 Hz | 1.796, 2.4 Hz | 1.771, 2.0 Hz |
  | 8 | 2.295, 25.4 Hz | 2.158, 11.4 Hz | 1.943, 5.6 Hz | 1.845, 3.4 Hz | 1.798, 2.5 Hz |
  | 16 | 2.538, 19.4 Hz | 2.536, 23.1 Hz | 2.187, 12.6 Hz | 1.954, 5.9 Hz | 1.851, 3.5 Hz |
  | 32 | 2.438, 29.3 Hz | 2.479, 27.2 Hz | 2.540, 24.4 Hz | 2.197, 12.2 Hz | 1.960, 5.9 Hz |
  | 64 | 2.525, 18.6 Hz | 2.433, 25.1 Hz | 2.515, 23.4 Hz | 2.505, 23.9 Hz | 2.202, 12.2 Hz |
  | 96 | 2.579, 21.1 Hz | 2.499, 21.0 Hz | 2.462, 24.5 Hz | | 2.479, 23.1 Hz |
  | 160 | 2.391, 26.9 Hz | 2.552, 23.2 Hz | 2.479, 25.3 Hz | | |
  | 320 | 2.468, 31.3 Hz | 2.424, 24.8 Hz | 2.465, 24.9 Hz | | |

- **The excitatory sum falls as the rate rises and is at 0.45 of the prior's once the gain has reached its band**: by the thirty-second window at a sixteenth, one in 32 and one in 64, the sixty-fourth at one in 128 and the ninety-sixth at one in 256. The inhibitory sum falls a little on the way, to 0.90 at one in 256, and is back at the prior's by then.
- **In every window of the five lead-ins the gain is the controller's rule's** from the gain before and the window's estimate (`regulated`; `unruled` is zero in each).

#### The estimate by the rate of its window, over the five lead-ins

1 145 windows (`SCAN_BY_RATE_1024`), by bands of 4 096 spikes a window:

| A unit's rate, Hz | Windows | At the ceiling | Between a half and two | The estimate below the ceiling |
| ---: | ---: | ---: | ---: | ---: |
| 0 to 3.1 | 25 | 0 | 0 | 0 to 0.287 |
| 3.1 to 6.1 | 39 | 0 | 0 | 0 to 0.498 |
| 6.1 to 9.2 | 31 | 0 | 0 | 0 to 0.317 |
| 9.2 to 12.2 | 28 | 0 | 0 | 0 to 0.222 |
| 12.2 to 15.3 | 21 | 0 | 0 | 0 to 0.272 |
| 15.3 to 18.3 | 48 | 0 | 0 | 0 to 0.261 |
| 18.3 to 21.4 | 115 | 0 | 0 | 0 to 0.236 |
| 21.4 to 24.4 | 377 | 0 | 0 | 0 to 0.343 |
| 24.4 to 27.5 | 346 | 346 | 0 | — |
| 27.5 to 30.5 | 84 | 84 | 0 | — |
| 30.5 to 33.6 | 31 | 31 | 0 | — |

- **It reads near one nowhere.** In the 684 windows below the saturation rule's count, in every band from 1.8 to 24.4 Hz, the estimate is under a half; in the 461 at or above it, it is the ceiling the rule gives. The gate holds both over the pinned tables.
- **Its greatest values are in the rise**, where the gain and the rate climb inside the window: 0.498, 0.382 and 0.317 are each in a lead-in's first forty windows. From the first fall it reads at most 0.343.
- **With the gain held it reads 0 to 0.451**, all ninety-six windows in the lowest band, and at an eighth 0 to 0.409 below the count and the ceiling in fourteen windows. Over these seven lead-ins on seed 22, 1 276 windows, no window reads between a half and two.

#### The frozen block, where a step reached it

| Of sixty-four trials | The gain held | An eighth | One in 128 | One in 256 |
| :--- | ---: | ---: | ---: | ---: |
| The gain the image carries | 1.750 | 2.472 | 2.483 | 2.497 |
| The gain after the block | 1.750 | 2.306 | 2.483 | 2.497 |
| The volley, units a presentation of 51, A and B | 50.8, 50.8 | 48.4, 48.6 | 49.0, 48.0 | 48.7, 48.7 |
| The stimulus's spikes after the opening | 0 | 14 | 7 | 13 |
| The readouts' spikes before the injection | 258, 256 | 4 034, 3 942 | 3 430, 3 489 | 3 541, 3 343 |
| after the volley | 645, 637 | 4 312, 4 325 | 3 823, 3 704 | 3 823, 3 786 |
| Trials in which the stimulus is seen, of 64 | 62 | 51 | 46 | 49 |
| ADR-0072's sign, of 64 | 53 | 47 | 42 | 46 |
| Fires once, seen | yes, yes | no, no | no, no | no, no |

- **The instrument is drowned at the two smaller steps as at an eighth.** The readouts fire 13 times what they fire with the gain held before the stimulus arrives, and the volley adds a tenth to that where it adds one and a half times with the gain held.
- **The volley is short by the same two or three units**: those the background had inside their refractory window at the injection, as ADR-0077 read it.
- **Through the block the gain barely moves**, 2.4832 to 2.4826 and 2.4972 to 2.4970, where at an eighth it moves from 2.472 to 2.306.

#### The capacities

- **The train held every window read from it.** The most spikes of a window in the five lead-ins are 43 631, at a sixteenth, of the 335 872 the ring holds: 13 per cent. Each step's test holds its greatest window to the capacity, and `lead_in_window` asserted each of the 1 145.
- **No injection was refused**, the drive's or the frozen blocks', and **no worker's report carries an entry dropped**.
- **No capacity was sized**, and what the protocol wrote for a loud failure did not arise.

#### What these readings do not say

- **One network, one size, one generator**: seed 22 at 1 024 units.
- **The two smaller steps' regime is read over 21 and 15 windows**, the lead-in ending at the rule's window. Their bands are narrower than a larger step's by the step's own arithmetic and by fewer windows read; the frozen block's eight windows moved the gain inside the same band.
- **Whether the configuration learns with the controller on is not read.** No step holds, so no arm ran, and clauses 1 to 4 have no reading.
- **Why the estimate reads zero in two windows of three at the saturation rule's rate** is not read. The rule returns zero for a slope at or below zero, and which it was is in no table.
- **Nothing here says what the controller should regulate.** The readings are of the controller as built.

### The evidence

- **The dispatch's scope.** The diff changes no file under `src/` — the runtime's tests, the documents and the cost table — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**, as brief 071 asks: run [38352602613](https://github.com/DescentVTT/VirtualCortex/actions/runs/38352602613) at `6970bee`, the pins' commit. The round's commits, in pull request #206: `93f7b5b` the protocol, before any lead-in under a step of the scan; `86d5105` the index's row given its status; `6970bee` the five steps' tables and the gate's checks over them; `5bd4cd4` the readings and the documents; and the commit that carries this evidence, which changes no test.
- **Every whole-domain shard is green.** All 120 `exhaustive` tests passed on the hosted runners, each exit 0 with one test reported, 93 020 s summed. The 114 before this round reproduced their pinned numbers through the form: seed 22's derivation in 233 s, H-33's two in 146 and 236 s, H-34's two in 458 and 474 s, and the twelve arms that derive their network first. **This round's six reproduced their tables there**: the calibration at an eighth in 125 s, and the steps of a sixteenth, one in 32, one in 64, one in 128 and one in 256 in 631, 704, 624, 116 and 267 s.
- **The shards**, dealt by the table before this round, which did not know the six and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 86.5 min | H-34, seed 22 from the assignment 4 412; H-23 from the assignment 2 192 | 9 | 10 179 | 5 102 s, 47 % |
  | 1 | 45.0 min | H-34, seed 22 from the mirrored 2 163; H-24 from the mirrored 1 082 | 9 | 5 157 | 2 639 s, 24 % |
  | 2 | 80.5 min | H-34, seed 23 from the assignment 3 751; H-36, seed 23 from the assignment 1 919 | 10 | 9 190 | 4 747 s, 44 % |
  | 3 | 83.5 min | H-34, seed 23 from the mirrored 3 242; H-31 from the assignment 2 041 | 11 | 9 829 | 4 935 s, 46 % |
  | 4 | 81.2 min | H-31 from the mirrored 1 887; H-25 from the assignment 1 858 | 10 | 8 903 | 4 792 s, 44 % |
  | 5 | 41.7 min | H-36, seed 23 from the mirrored 1 128; H-28 from the assignment 1 097 | 11 | 4 339 | 2 435 s, 23 % |
  | 6 | 73.8 min | H-33, seed 23 from the assignment 1 924; H-22 from the mirrored 1 852 | 10 | 8 213 | 4 349 s, 40 % |
  | 7 | 69.4 min | H-33, seed 23 from the mirrored 1 926; H-33, seed 24 from the assignment 1 913 | 9 | 8 165 | 4 086 s, 38 % |
  | 8 | 75.1 min | H-33, seed 24 from the mirrored 1 890; H-30 from the mirrored 1 855 | 10 | 8 815 | 4 423 s, 41 % |
  | 9 | 62.8 min | H-36, seed 24 from the assignment 1 906; H-32 from the mirrored 1 850 | 10 | 7 126 | 3 694 s, 34 % |
  | 10 | 74.0 min | H-29 from the mirrored 1 872; H-35 from the assignment 1 845 | 10 | 8 409 | 4 351 s, 40 % |
  | 11 | 43.4 min | H-36, seed 24 from the mirrored 1 113; H-30 from the assignment 1 071 | 11 | 4 695 | 2 540 s, 24 % |

  The percentages are of the job's bound, 180 minutes ([ADR-0176](0176-a-shards-bound-by-its-longest-test.md)); each shard ran its tests two at a time, at 1.78 to 2.00 of their summed seconds over the wall. The twelve started within a second of each other, four seconds after the dispatch, no sweep and no other run holding a place, and the last ended at 07:07:01Z, 1 h 27 min after it. **No shard passed 60 per cent of its bound**: the most is 47, the shard that holds H-34's longest arm. This round's six stand in the second, third, eighth, ninth, tenth and eleventh shards, none among a shard's two heaviest.
- **The longest test read past what ADR-0176's rule holds, and that is recorded as F-68.** ADR-0176 sets the bound as *"the least multiple of thirty minutes that holds the longest whole-domain test two and a half times over"*, from a longest reading of 3 730 s. Here H-34's arm on seed 22 from the assignment read 4 412 s. Two and a half times that is 184 minutes, so by the rule as written the bound would be 210. No test of that length was added: an arm the tree has held since brief 068 read slower on this run's runner. Brief 071 changes no workflow, and the bound is not moved here. It is the workflow's next decision, with this reading.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 38352602613`): 120 lines, 93 020 s, where it held 114 and 81 785. ADR-0092's deal plans each of the twelve shards at 7 750 to 7 755 s summed, 3 880 to 4 360 s of wall time at this run's ratios, and the shard that holds the longest arm at no less than that arm's 4 412: **36 to 41 per cent of the bound**. No shard is planned past 60.

  **The table is one run's, and this run's runners were slow.** The 114 tests before this round summed to 90 553 s here where the last run read them at 81 785, a tenth more; the six of this round are 2 467 s of the rest.
- **No sweep is owed and none was dispatched.** The scope is `exhaustive`; ADR-0174 records the last sweep that ended, the schedule's of run 38234325824, and no weekly run was scheduled or dispatched between it and this one.
- **The pull request's gate** on `93f7b5b`, on `86d5105` and on `5bd4cd4` is green in every job, the determinism pin on AArch64 and the MSRV among them. The mutation gate on the changed lines found no mutant to make, in each: no line under `src/` changed. The gate on `6970bee` was cancelled by the push of `5bd4cd4`, which changes documents and whose tests are its.
- **On the developer machine**, at `6970bee`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 739 passed and 160 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 120 whole-domain tests are listed; and `cargo mutants --in-diff` over the round's diff found no mutant to make. The five steps were not run a second time there: each ran once, and the dispatch is their reproduction.

### The step of the stopping rule reached

**Step 5** (`CONTROLLED_STEP_1024`): no step holds, as predicted. Steps 1 and 2 were met: one round, no file under `src/` changed, the steps, the bound, the criterion and H-37's constants committed before any lead-in under a step of the scan, and a calibration in which no pinned number moved. Steps 3 and 4 did not arise. Step 6 was kept: no constant moved after a lead-in was run under a step of the scan, and there was no second attempt, no other step and no other network.

ADR-0175's step 5: *"the step is not the lever. The next decision is an ADR on what the controller regulates, its estimate or a rate, with this round's readings of the estimate by rate and of the regime each step keeps. It is named and not taken."* **The next decision is that ADR. It is named here and not taken.**

**What this round hands it**, by kind:

- **The estimate by rate.** Over 1 276 windows on seed 22 — ninety-six with the gain held, thirty-five at an eighth and 1 145 at the five steps — the estimate reads 0 to 0.498 at every rate from 1.6 to 24.4 Hz and its ceiling at every rate from there up. So a controller that rests where its estimate reads one finds no such rate below the saturation rule's on this network. Above it the estimate is the rule's ceiling, and what the slope would read there is in no table.
- **What does hold the network.** The saturation rule, at 24.4 Hz a unit: half the windows at the ceiling and half below, the gain a step up and a step down in turn, at every step from an eighth to one in 256.
- **What the step is a lever on.** The width of the gain's swing there, about two steps, and the windows the rise takes. Not the rate.
- **The regime that rate is.** A gain of 2.49 to 2.50, an excitatory sum at 0.45 of the prior's and an inhibitory sum at the prior's, where the gain held leaves 0.925 and 0.775. The readouts fire thirteen times what they fire with the gain held before a stimulus arrives, the volley is two or three units short of fifty-one, and the stimulus is seen in 46 to 51 trials of 64 where 56 are asked.
- **Where the instrument reads.** At a gain held at 1.75 and 1.7 Hz a unit, the working point of every learning result, where the estimate reads 0 to 0.451.
- **What the settled network's rule cannot tell.** At a step of one in 128 or less the gain's swing fits inside ADR-0055's tolerance, and the rule reads the network as settled. A rule for a network under a controller would need to tell a still gain from one that alternates.

## Consequences

- Good: a derivation is one body at every step, and seed 22's, H-33's and H-34's run through the form every week.
- Good: ADR-0077's second candidate is reproduced by the derivation, by ADR-0077's own rule, before any step of the scan is read.
- Good: ADR-0175's sentences about the two pinned lead-ins are held by rules in the gate, and the controller's record by an oracle of its rule.
- Good: each predicted reading and what would refute the prediction is a rule written before the tables exist.
- Good: the prediction was written with its reason and with what would refute it, and each is read: the verdict as predicted, the three readings held, the second by 114 parts in 32 768.
- Good: the decision on what the controller regulates has the estimate by rate over 1 276 windows and the regime the controller keeps at six steps.
- Bad: the round's yield is its readings. Whether the configuration learns with the controller on is not read.
- Bad: the settled network's rule passed two networks whose gain alternates inside its tolerance. The frozen block failed them, and the rule is as it was.
- Bad: the settled network's rule has two forms in the tree, the sums alone and the sums with the gains, and which a derivation takes is a value it is given.
- Bad: the train's capacity is held by an assertion and a pinned margin, not by construction.
- Bad: six more weekly tests, 2 467 s in the dispatched run; and in that run the longest test of the job read 4 412 s, past what ADR-0176's rule holds at 180 minutes (F-68).
- Neutral: no step held, so the arms' body under a step is designed here and not built.
- Neutral: `settled_within`'s body moved under `settled_in`; the rule is the one it was.

## Alternatives considered and why rejected

- **Option 1(b), a step in `derive_at`'s arguments**: every call of the tree changes to say zero, and the rule and the bound still need a second argument. **Option 1(c), a second derivation**: what a step changes could not be told from what the copy changed.
- **Option 2(b), the later of the two first windows**: a series that held early and moved since would pass at a window where it does not hold. **Option 2(c), the gains alone**: ADR-0175's rule names both, and the image is written from the weights.
- **Option 3(b), the scan's rule at an eighth**: its gains do not settle, so the lead-in would run to 320 windows and the frozen block ADR-0175 names would not be reached. **Option 3(c), `background_candidate` alone**: it is not the derivation, and what the scan runs is the derivation.
- **Option 4(b), a train sized for the scan**: a configuration that differs between the calibration and the scan, for a margin of six at an eighth that a smaller step is predicted to widen.
- **Option 5(b), one test for five steps**: a step's failure would hide the steps after it, and the longest test of the job would grow by five lead-ins.
- **Option 6(b), every window listed**: it is the pinned lead-ins themselves. **Option 6(c), by the gain**: the question ADR-0175 asks of the estimate is at what rate it reads near one.
- **Option 7(b), the arms built before the scan**: a body no run could exercise where the prediction holds.

## Confirmation

`runtime/cortex-runtime/tests/inhibition.rs` holds the form, the rules, the six weekly tests and the gate's test; `tests/instrument/harness.rs` holds `settled_in` and `lead_in_until`. `npm run spec` holds the links, the rows and the version. `SCAN_*` hold the five steps' tables, `CONTROLLED_1024` the verdict, `CONTROLLED_STEP_1024` the step reached, `SCAN_READINGS_READ_1024` the three readings and `SCAN_BY_RATE_1024` the estimate by rate; the gate's test holds each by its rule. Whitepaper §11.1 carries H-37's verdict and the step reached.
