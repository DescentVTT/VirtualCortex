---
status: proposed
date: 2026-10-13
depends-on: ADR-0175
decision-makers: VirtualCortex maintainers
---

# ADR-0177: The controller's step, measured — H-37's protocol, written before any lead-in is run under a step of the scan: the derivation given how it settles its network (`derive_under`, a `Settling`), every derivation of the tree the call it was; the settled network's rule under a step, ADR-0055's criterion over the excitatory sums and over the gains at one window, within 320; the calibration at zero and at an eighth, the second reaching ADR-0077's tables under ADR-0077's own rule; clause 0, the verdict, the step of the stopping rule and the three predicted readings as integer rules; the estimate by the rate of its window by bands of an eighth of the saturation rule's count; the readings follow the run

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

## Consequences

- Good: a derivation is one body at every step, and seed 22's, H-33's and H-34's run through the form every week.
- Good: ADR-0077's second candidate is reproduced by the derivation, by ADR-0077's own rule, before any step of the scan is read.
- Good: ADR-0175's sentences about the two pinned lead-ins are held by rules in the gate, and the controller's record by an oracle of its rule.
- Good: each predicted reading and what would refute the prediction is a rule written before the tables exist.
- Bad: the settled network's rule has two forms in the tree, the sums alone and the sums with the gains, and which a derivation takes is a value it is given.
- Bad: the arms' body under a step is designed and not built before the scan; where a step holds it is built before the arms run and after the scan's tables are known.
- Bad: the train's capacity is held by an assertion and a pinned margin, not by construction.
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

`runtime/cortex-runtime/tests/inhibition.rs` holds the form, the rules, the six weekly tests and the gate's test; `tests/instrument/harness.rs` holds `settled_in` and `lead_in_until`. `npm run spec` holds the links, the rows and the version. The readings, the verdict, the evidence and the dispatch follow the run.
