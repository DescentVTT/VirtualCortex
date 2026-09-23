pub(crate) use cortex_connectome::{
    CortexFileHeader, Prior, SECTION_HOMEOSTASIS, SECTION_MODULATOR, SectionEntry, crc64,
    ring_distance,
};

pub(crate) use cortex_core::{
    DendriticSuperNeuron, ELIGIBILITY_TAU_SHIFT, FLAG_INHIBITORY, MODULATION_ONE_Q16,
    NO_SPIKE_ON_RECORD, REFRACTORY_TICKS, STDP_A_MINUS_Q1_15, STDP_A_PLUS_Q1_15, STDP_TAU_SHIFT,
    THRESHOLD_BASE, message_efficacy_q16, spike_message, stp_decay_factor_q16,
};

pub(crate) use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool,
};

pub(crate) use cortex_neuromod::DOPAMINE_TAU_SHIFT;

pub(crate) use cortex_runtime::{
    Cancel, Config, Delivery, Drive, Executor, Feedback, Image, Outcome, Readout, Set, Stimulus,
    Task, TaskError, Window, blocks_for, run_driven, spikes_per_unit, synthesize,
};

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testkit/prop.rs"
));

pub(crate) type Engine = Executor<2048>;

pub(crate) const ONE: i32 = MODULATION_ONE_Q16;

// ------------------------------------------- written before the run (ADR-0065, ADR-0066)

/// A trial: $2^{14}$ ticks, 164 ms simulated, one time constant of the dopamine signal, so
/// that the signal one outcome left has decayed to $e^{-1}$ of itself at the next trial's
/// end and one outcome gates one trial's traces; a quarter of the eligibility trace's window
/// of $2^{16}$ ticks, so every pairing of the trial is pending when its reward comes.
pub(crate) const TRIAL_TICKS: u32 = 1 << DOPAMINE_TAU_SHIFT;

const _: () = assert!(TRIAL_TICKS == 1 << 14);

/// A block: sixty-four trials.
pub(crate) const BLOCK: usize = 64;

/// A run: 512 trials, eight blocks, $2^{23}$ ticks at either size.
pub(crate) const TRIALS: usize = 8 * BLOCK;

/// The criterion counts the last two blocks, 128 trials.
pub(crate) const LAST_BLOCKS: usize = 2;

/// The modulation with the dopamine signal at rest in the rewarded runs and under the
/// shuffled reward, brief 027's: half of every pending trace consolidates at a presynaptic
/// spike with no reward; a reward carries the next trial's consolidation toward the ceiling
/// and a punishment toward the floor.
pub(crate) const BASELINE_Q16: i32 = 0x8000;

/// The reward's magnitude, 1.0, brief 027's: the width of the modulation, signed by the
/// outcome.
pub(crate) const REWARD_Q16: i32 = ONE;

/// The stimulus: two messages of 1.25 into every unit of the set, the replay drive's
/// (ADR-0038), which fires a unit at its base threshold once.
pub(crate) const STIMULUS_Q16: i32 = 0x0001_4000;

pub(crate) const STIMULUS_MESSAGES: u32 = 2;

/// The seed the trials' stimuli and the shuffled coin are drawn from, brief 027's.
pub(crate) const SEED: u64 = 27;

/// The prior's local window: a local synapse reaches a unit within this many places on the
/// ring (ADR-0044's prior, below).
pub(crate) const PRIOR_WINDOW: u32 = 8;

/// The geometry, a rule of the prior's window and of its inhibitory rule. The ring is read
/// in periods of twenty places from a rotation: stimulus A is the first place of every
/// period and stimulus B the twelfth, so that every stimulus unit is at least nine places
/// from every other (beyond the window: no unit fires twice through a local synapse, and
/// neither stimulus drives the other through one) and, the period being a multiple of five
/// and the rotation not three or four more than one, no stimulus unit is one the prior makes
/// inhibitory (every fifth unit, from the fifth); readout 0 is the odd places but the twelfth
/// and readout 1 the even places but the first, nine each, so that each stimulus unit has
/// four units of either readout on either side of it, at mirrored distances, and every unit
/// of a period is in exactly one set. The places from the last whole period to the ring's
/// end, and from the ring's start to the rotation, are in no set.
pub(crate) const PERIOD: u32 = 20;

pub(crate) const A_OFFSET: u32 = 0;

pub(crate) const B_OFFSET: u32 = 11;

pub(crate) const R0_MASK: u32 = 0xAA2AA;

pub(crate) const R1_MASK: u32 = 0x55554;

const _: () =
    assert!(B_OFFSET - A_OFFSET > PRIOR_WINDOW && PERIOD + A_OFFSET - B_OFFSET > PRIOR_WINDOW);

const _: () = assert!(PERIOD % 5 == 0 && (A_OFFSET + 1) % 5 != 0 && (B_OFFSET + 1) % 5 != 0);

const _: () = assert!(((1 << A_OFFSET) | (1 << B_OFFSET) | R0_MASK | R1_MASK) == (1 << PERIOD) - 1);

const _: () = assert!(((1 << A_OFFSET) & R0_MASK) == 0 && ((1 << B_OFFSET) & R0_MASK) == 0);

const _: () = assert!(((1 << A_OFFSET) & R1_MASK) == 0 && ((1 << B_OFFSET) & R1_MASK) == 0);

const _: () = assert!((R0_MASK & R1_MASK) == 0 && R0_MASK.count_ones() == R1_MASK.count_ones());

/// The rotation at each size: the first place from which the pattern's four excitatory
/// couplings, read from the prior's census before any run, are equal within ten per cent
/// and none zero (`the_geometry_holds_against_the_census_at_both_sizes` holds it, and that
/// every smaller rotation fails). At 256 units it is 17, eleven whole periods; at 1 024 it
/// is 0, fifty-one.
pub(crate) const ROTATION_256: u32 = 17;

pub(crate) const ROTATION_1024: u32 = 0;

/// The readout window, a rule of the prior's local delay band (1 to 3 ms, 100 to 300 ticks):
/// it opens `delay_min` ticks after the stimulus is injected, before which no local synapse
/// of the volley can have landed, and closes `2 × delay_max` after it, leaving the volley's
/// own latency and the readout unit's as much again as the band's longest delay; the far
/// band (1 400 ticks on) lies outside it. The same window for both readouts and every trial.
pub(crate) const WINDOW: Window = Window {
    from: 100,
    ticks: 500,
};

/// The lead-in before the first trial: one window under the drive with no stimulus, so that
/// the first trial has a window before its injection.
pub(crate) const LEAD_IN: u32 = WINDOW.ticks;

/// The calibration's candidate gains, in the order they are tried: 1.75, then 2.0
/// (ADR-0044's table: five to six times quieter at 1.75).
pub(crate) const GAINS: [u32; 2] = [0x0001_C000, 0x0002_0000];

/// The calibration's pass mark: trials in which the window after the volley held more
/// readout spikes than the window before the injection, of sixty-four.
pub(crate) const SEEN_MIN: u32 = 56;

/// The gain the calibration picked at each size, the first candidate that passed
/// (`CALIBRATION_256`, `CALIBRATION_1024` below): 2.0 at 256 units, where 1.75 saw the
/// stimulus in 50 trials of 64; 1.75 at 1 024, where it saw it in 62.
pub(crate) const GAIN_256: u32 = 0x0002_0000;

pub(crate) const GAIN_1024: u32 = 0x0001_C000;

/// The criterion's counts over the last 128 trials (ADR-0066): the rewarded run's correct
/// trials at least 80, in both assignments, at both sizes (by noise about once in 337 per
/// run); a control's at most 76 (exceeded by noise about once in 74 per control);
/// `the_criterion_reads_as_written` computes both from the binomial's tail in integers.
pub(crate) const REWARDED_MIN: u32 = 80;

pub(crate) const CONTROL_MAX: u32 = 76;

// ------------------------------------------------- written before the run (brief 031)

/// A window of the population tally: $2^{12 + 5}$ ticks, the cadence at which ADR-0055
/// read the day and at which the settling is read here.
pub(crate) const WINDOW_TICKS: u64 = 1 << (ACTIVITY_BIN_SHIFT + ACTIVITY_WINDOW_SHIFT);

const _: () = assert!(WINDOW_TICKS == 1 << 17);

/// The settling measurement's length: eighty windows, the length ADR-0055 gave 1 024 units;
/// a run of the task is sixty-four.
pub(crate) const SETTLING_WINDOWS: u64 = 80;

/// Brief 026's clause, unchanged (ADR-0055): a window satisfies it when each of the last
/// four windows up to and including it moved the excitatory sum by less than two per cent
/// of the sum before those four, the prior's sum standing before the first window. The
/// lead-in is the smallest window that satisfies it, or `SETTLING_WINDOWS` when none does
/// up to the bound; derived, never chosen.
pub(crate) const SETTLING_CLAUSE_WINDOWS: usize = 4;

pub(crate) const SETTLING_CLAUSE_PER_CENT: u64 = 2;

/// The lead-in in windows, derived from `SETTLING_256` by the rule above and not chosen:
/// the clause first holds at the ninth window (the last four moving the sum by 1.82, 1.79,
/// 1.58 and 1.44 per cent of the fifth's; the sum 0.818 of the prior's), so the run behind
/// the lead-in starts its first trial nine windows in. The gate holds the constant to the
/// rule over the pinned table; it was committed before the first run behind it.
pub(crate) const LEAD_IN_WINDOWS: u64 = 9;

// ------------------------------------------------------------------------- the network

/// The prior of ADR-0044 at `units`: a fifth inhibitory at the rail, 32 synapses per unit, a
/// window of eight, a quarter rewired, local delays of 1 to 3 ms and far ones of 14 to
/// 25.6 ms, excitatory weights in [6 000, 12 000], seed 22.
pub(crate) fn prior(units: u32) -> Prior {
    Prior {
        units,
        inhibitory_every: 5,
        synapses_per_unit: 32,
        window: PRIOR_WINDOW,
        rewire_q0_8: 64,
        delay_min: 100,
        delay_max: 300,
        far_delay_min: 1400,
        far_delay_max: 2559,
        weight_min: 6000,
        weight_max: 12000,
        inhibitory_gain_q4_4: 255,
        apical_q0_8: 0,
        seed: 22,
    }
}

/// The drive of ADR-0044: every tick, one message of 0.125 per 128 units into units drawn
/// by the tick.
pub(crate) fn drive(units: u32) -> Drive {
    Drive {
        every: 1,
        messages: units / 128,
        efficacy_q16: 0x2000,
        units,
        seed: 3,
    }
}

/// The executor for a run: the gain held (`control_step_q0_16` 0), no sleep, a train that
/// holds the most spikes a trial can produce, no arena and no store.
pub(crate) fn config(units: u32, workers: usize, baseline_q16: i32) -> Config {
    Config {
        workers,
        units: units as usize,
        blocks: blocks_for(&prior(units)) as usize,
        nodes_per_worker: 1 << 16,
        injector_capacity: 1 << 12,
        train_capacity: (units as usize).saturating_mul(spikes_per_unit(TRIAL_TICKS) as usize),
        modulation_baseline_q16: baseline_q16,
        control_step_q0_16: 0,
        sleep_shift: 0,
        ..Config::default()
    }
}

/// The image of `exec` with its homeostasis record patched, decoded under `config`.
pub(crate) fn reload_with(
    exec: &Engine,
    config: Config,
    patch: impl Fn(&mut HomeostaticDrivePool),
) -> Engine {
    let mut img = Image::encode(exec).expect("quiescent");
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_HOMEOSTASIS {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            let mut pool = HomeostaticDrivePool::decode((&img[offset..][..64]).try_into().unwrap());
            patch(&mut pool);
            img[offset..][..length].copy_from_slice(&pool.encode());
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            break;
        }
    }
    Image::decode::<2048>(&img, config).expect("a well-formed record")
}

/// The network at `gain`, synthesized from `p` and reloaded with the gain in its record.
pub(crate) fn at_gain(p: &Prior, config: Config, gain: u32) -> Engine {
    let mut exec = Engine::new(config.clone()).unwrap();
    let (u, b) = exec.arenas_mut();
    synthesize(u, b, p).unwrap();
    reload_with(&exec, config, |h| h.synaptic_gain_q16 = gain)
}

// ---------------------------------------------------------------------------- the task

/// The four sets of the geometry at `units` from `rotation`: stimulus A, stimulus B,
/// readout 0, readout 1, over as many whole periods as fit from the rotation.
pub(crate) fn geometry(units: u32, rotation: u32) -> [Set; 4] {
    let count = units.saturating_sub(rotation) / PERIOD;
    let set = |mask: u32| Set {
        first: rotation,
        period: PERIOD,
        mask,
        count,
    };
    [
        set(1 << A_OFFSET),
        set(1 << B_OFFSET),
        set(R0_MASK),
        set(R1_MASK),
    ]
}

/// The rotation at `units`, as written above.
pub(crate) fn rotation(units: u32) -> u32 {
    match units {
        256 => ROTATION_256,
        1024 => ROTATION_1024,
        _ => panic!("no rotation is written for {units} units"),
    }
}

/// The gain at `units`, as the calibration picked it.
pub(crate) fn gain(units: u32) -> u32 {
    match units {
        256 => GAIN_256,
        1024 => GAIN_1024,
        _ => panic!("no gain is written for {units} units"),
    }
}

/// A stimulus's shape: the messages into every unit of the set and each message's efficacy
/// (brief 033; `SHAPE_F46` is the shape every run before it used).
pub(crate) type Shape = (u32, i32);

/// The shape of ADR-0065's stimulus, F-46's: two messages of 1.25.
pub(crate) const SHAPE_F46: Shape = (STIMULUS_MESSAGES, STIMULUS_Q16);

/// The task at `units` with the stimulus of `shape` and, when one is given, its cancel
/// (brief 034; every run before it passes `None`, so its task is the task it was).
pub(crate) fn task(
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    feedback: Feedback,
    mirrored: bool,
    delivery: Delivery,
) -> Task {
    let [a, b, r0, r1] = geometry(units, rotation(units));
    let stimulus = |set| Stimulus {
        set,
        messages: shape.0,
        efficacy_q16: shape.1,
        cancel,
    };
    Task {
        stimuli: [stimulus(a), stimulus(b)],
        readout: Readout::new([r0, r1]),
        drive: drive(units),
        ticks: TRIAL_TICKS,
        window: WINDOW,
        seed: SEED,
        reward_q16: REWARD_Q16,
        mirrored,
        feedback,
        delivery,
    }
}

// -------------------------------------------------------------------------- the readings

/// The weights over the arena by polarity: the sum of the inhibitory magnitudes and the sum
/// of the excitatory weights.
pub(crate) fn weights_by_polarity(exec: &Engine) -> (i64, i64) {
    let mut inhibitory = 0i64;
    let mut excitatory = 0i64;
    for unit in exec.units() {
        let inhibitory_unit = unit.flags & FLAG_INHIBITORY != 0;
        for s in unit.fan_out(exec.blocks()) {
            let w = s.weight_q1_15 as i64;
            if inhibitory_unit {
                inhibitory = inhibitory.wrapping_add(w.wrapping_neg());
            } else {
                excitatory = excitatory.wrapping_add(w);
            }
        }
    }
    (inhibitory, excitatory)
}

/// The excitatory coupling from `from` into `into`: the sum of the weights of the synapses an
/// excitatory unit of `from` sends to a unit of `into`.
pub(crate) fn coupling(exec: &Engine, from: Set, into: Set) -> i64 {
    let mut sum = 0i64;
    for unit in exec.units() {
        if unit.flags & FLAG_INHIBITORY != 0 || !from.contains(unit.id as u32) {
            continue;
        }
        for s in unit.fan_out(exec.blocks()) {
            if into.contains(s.target) {
                sum = sum.wrapping_add(s.weight_q1_15 as i64);
            }
        }
    }
    sum
}

/// The four couplings of a geometry on `exec`: A→R0, A→R1, B→R0, B→R1.
pub(crate) fn couplings(exec: &Engine, sets: &[Set; 4]) -> [i64; 4] {
    let [a, b, r0, r1] = *sets;
    [
        coupling(exec, a, r0),
        coupling(exec, a, r1),
        coupling(exec, b, r0),
        coupling(exec, b, r1),
    ]
}

/// (c): the four couplings equal within ten per cent (the largest at most eleven tenths of
/// the smallest) and none zero.
pub(crate) fn balanced(c: &[i64; 4]) -> bool {
    let min = c.iter().copied().min().unwrap_or(0);
    let max = c.iter().copied().max().unwrap_or(0);
    min > 0 && max.saturating_mul(10) <= min.saturating_mul(11)
}

/// One block: the correct trials of sixty-four; the trials that presented stimulus A; the
/// readouts' spikes in the window after the volley summed over the block, by the stimulus
/// presented (`[stimulus][readout]`); the presented stimulus set's own spikes in the ticks
/// before the window opens (the volley), summed over the block by stimulus; the stimulus
/// sets' own spikes over the whole trial, summed over the block; the readouts' spikes in the
/// window before the injection, summed over the block; the trials in which the window after
/// the volley held more readout spikes than the window before it (the calibration's
/// measure); the inhibitory and the excitatory sum over the arena after the block; the
/// modulator's signal after the block's last reward; the excitatory coupling from each
/// stimulus set into each readout set after the block (`[stimulus][readout]`); and the
/// trials whose two counts tied, which select nothing and are errors (ADR-0059).
pub(crate) type Block = (
    u32,
    u32,
    [[u64; 2]; 2],
    [u64; 2],
    [u64; 2],
    [u64; 2],
    u32,
    i64,
    i64,
    i32,
    [[i64; 2]; 2],
    u32,
);

/// A run of `trials` trials on the prior at `units` at `gain` on `workers` workers under a
/// delivery of the dopamine term (ADR-0068; `Delivery::Global` is ADR-0066's form): the
/// blocks' readings, and the FNV-1a hash of every trial's `(stimulus, selection, correct)`,
/// the accuracy sequence in one number. The first trial is preceded by a lead-in of one
/// window under the drive: `run_behind` with no whole windows before it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run(
    units: u32,
    workers: usize,
    gain: u32,
    baseline_q16: i32,
    feedback: Feedback,
    mirrored: bool,
    delivery: Delivery,
    trials: usize,
) -> (Vec<Block>, u64) {
    run_behind(
        0,
        SHAPE_F46,
        None,
        units,
        workers,
        gain,
        baseline_q16,
        feedback,
        mirrored,
        delivery,
        trials,
        &mut |_, _, _, _| {},
    )
}

/// `run` behind a lead-in of `lead_in_windows` whole windows under the drive alone
/// (brief 031), before the instrument's own lead-in of one readout window, with the
/// stimulus of `shape` (brief 033; `run` passes `SHAPE_F46`) and its cancel (brief 034;
/// `run` passes `None`); the executor, the task and every constant are `run`'s, and the
/// windows, the shape and the cancel are the only differences. `observe` is called after
/// every trial with the executor, the trial's index, its first tick and its outcome
/// (brief 032's composition reads the arena and the train through it); it reads and never
/// writes, so a run that observes nothing is the run before it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_behind(
    lead_in_windows: u64,
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    workers: usize,
    gain: u32,
    baseline_q16: i32,
    feedback: Feedback,
    mirrored: bool,
    delivery: Delivery,
    trials: usize,
    observe: &mut dyn FnMut(&mut Engine, usize, u32, &Outcome),
) -> (Vec<Block>, u64) {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, workers, baseline_q16), gain);
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain);
    assert_eq!(exec.modulation_baseline_q16(), baseline_q16);
    let task = task(shape, cancel, units, feedback, mirrored, delivery);
    lead_in(&mut exec, &task.drive, lead_in_windows);
    run_on(&mut exec, task, units, trials, observe)
}

/// `run_behind` from the executor's present state (brief 035): the instrument's own lead-in
/// of one readout window under the task's drive, then `trials` trials of `task`, read as
/// `run_behind` reads them. The executor is the caller's, so a lead-in, a settling or an
/// image the caller made stands before the first trial; `run_behind` builds the network and
/// runs its whole windows, then calls this.
pub(crate) fn run_on(
    exec: &mut Engine,
    task: Task,
    units: u32,
    trials: usize,
    observe: &mut dyn FnMut(&mut Engine, usize, u32, &Outcome),
) -> (Vec<Block>, u64) {
    run_on_flipped(exec, task, units, trials, None, observe)
}

/// `run_on` with the task's mapping flipped once (brief 040): before the trial of index
/// `flip`, if one is given, `task.mirrored` is negated and nothing else is touched, so the
/// trials before it are `run_on`'s and a trial after it is judged, rewarded and counted
/// correct under the other mapping; with none it is `run_on`, which calls it so.
pub(crate) fn run_on_flipped(
    exec: &mut Engine,
    mut task: Task,
    units: u32,
    trials: usize,
    flip: Option<usize>,
    observe: &mut dyn FnMut(&mut Engine, usize, u32, &Outcome),
) -> (Vec<Block>, u64) {
    assert_eq!(
        exec.addressed_counts(),
        (units as usize, units as usize),
        "every unit a source and a target before the first trial, whatever the delivery"
    );
    task.check(exec).expect("the task fits the executor");
    let [a, b, r0, r1] = geometry(units, rotation(units));
    // The stimulus sets counted as a readout would count them: the same rule, the other
    // two sets.
    let stimuli = Readout::new([a, b]);
    let inject = exec.injector();
    for _ in 0..LEAD_IN {
        task.drive
            .step(&inject, exec.ticks())
            .expect("the drive runs");
        exec.tick();
    }
    let mut blocks = Vec::new();
    let mut sequence: Vec<i32> = Vec::with_capacity(trials);
    let mut correct = 0u32;
    let mut a_trials = 0u32;
    let mut spikes = [[0u64; 2]; 2];
    let mut volley = [0u64; 2];
    let mut stimulus_spikes = [0u64; 2];
    let mut before_spikes = [0u64; 2];
    let mut seen = 0u32;
    let mut ties = 0u32;
    for trial in 0..trials {
        if flip == Some(trial) {
            task.mirrored = !task.mirrored;
        }
        let overwritten = exec.train_overwritten();
        let start = exec.ticks() as u32;
        let before =
            task.readout
                .count_window(exec.train(), start.wrapping_sub(WINDOW.ticks), WINDOW.ticks);
        let outcome = task.trial(exec, trial as u64).expect("a trial runs");
        assert!(
            exec.train_overwritten().saturating_sub(overwritten)
                <= u64::from(spikes_per_unit(TRIAL_TICKS)).saturating_mul(u64::from(units)),
            "the ring let go no more than one trial's bound"
        );
        let in_stimuli = stimuli.count(exec.train(), start, TRIAL_TICKS);
        let in_volley = stimuli.count_window(exec.train(), start, WINDOW.from);
        correct = correct.saturating_add(u32::from(outcome.correct));
        a_trials = a_trials.saturating_add(u32::from(outcome.stimulus == 0));
        let s = usize::from(outcome.stimulus);
        spikes[s][0] = spikes[s][0].saturating_add(u64::from(outcome.counts[0]));
        spikes[s][1] = spikes[s][1].saturating_add(u64::from(outcome.counts[1]));
        volley[s] = volley[s].saturating_add(u64::from(in_volley[s]));
        stimulus_spikes[0] = stimulus_spikes[0].saturating_add(u64::from(in_stimuli[0]));
        stimulus_spikes[1] = stimulus_spikes[1].saturating_add(u64::from(in_stimuli[1]));
        before_spikes[0] = before_spikes[0].saturating_add(u64::from(before[0]));
        before_spikes[1] = before_spikes[1].saturating_add(u64::from(before[1]));
        let after_total = outcome.counts[0].saturating_add(outcome.counts[1]);
        let before_total = before[0].saturating_add(before[1]);
        seen = seen.saturating_add(u32::from(after_total > before_total));
        ties = ties.saturating_add(u32::from(outcome.selection.is_none()));
        observe(exec, trial, start, &outcome);
        sequence.push(
            i32::from(outcome.stimulus)
                | i32::from(outcome.selection.map_or(3, |r| r)) << 1
                | i32::from(outcome.correct) << 3,
        );
        if trial.wrapping_add(1) % BLOCK == 0 {
            let (inhibitory, excitatory) = weights_by_polarity(exec);
            blocks.push((
                correct,
                a_trials,
                spikes,
                volley,
                stimulus_spikes,
                before_spikes,
                seen,
                inhibitory,
                excitatory,
                exec.modulator().dopamine_rpe,
                [
                    [coupling(exec, a, r0), coupling(exec, a, r1)],
                    [coupling(exec, b, r0), coupling(exec, b, r1)],
                ],
                ties,
            ));
            correct = 0;
            a_trials = 0;
            spikes = [[0; 2]; 2];
            volley = [0; 2];
            stimulus_spikes = [0; 2];
            before_spikes = [0; 2];
            seen = 0;
            ties = 0;
        }
    }
    (blocks, fnv1a_64(&sequence))
}

/// Dumps a run, then holds it to its pinned table and, where one is pinned, its trace.
pub(crate) fn pinned(name: &str, blocks: &[Block], trace: u64, table: &[Block], pin: u64) {
    eprintln!(
        "DUMP {name} {blocks:?} trace {trace:#018x} curve {:?}",
        curve(blocks)
    );
    assert_eq!(blocks, table, "{name}");
    if pin != 0 {
        assert_eq!(trace, pin, "{name}: the accuracy sequence");
    }
}

/// The correct trials per block of a run, for the dump.
pub(crate) fn curve(blocks: &[Block]) -> Vec<u32> {
    blocks.iter().map(|b| b.0).collect()
}

// ------------------------------------------------------------- the calibration (ADR-0065)

/// The calibration's measure over one block: the readouts' spikes after the volley against
/// the spikes before the injection, by readout, and the trials in which the window after
/// held more.
pub(crate) fn measure(block: &Block) -> ([u64; 2], [u64; 2], u32) {
    let after = [
        block.2[0][0].saturating_add(block.2[1][0]),
        block.2[0][1].saturating_add(block.2[1][1]),
    ];
    (after, block.5, block.6)
}

/// A gain passes when the window after the volley held more readout spikes than the window
/// before it in at least `SEEN_MIN` of the block's trials, and each readout's total after
/// exceeds its total before.
pub(crate) fn calibrated(block: &Block) -> bool {
    let (after, before, seen) = measure(block);
    seen >= SEEN_MIN && after[0] > before[0] && after[1] > before[1]
}

/// The gain a size's calibration picks: the first candidate, in the candidates' order, that
/// passed; none when neither did.
pub(crate) fn picked(calibration: &[(Block, u64); 2]) -> Option<u32> {
    GAINS
        .iter()
        .zip(calibration.iter())
        .find(|(_, (block, _))| calibrated(block))
        .map(|(&gain, _)| gain)
}

/// One calibration run: sixty-four trials at `gain` with the modulation baseline at zero
/// and no reward, so that no weight of either polarity moves (asserted against the sums
/// before the run); the block's readings and the run's trace.
pub(crate) fn calibration(units: u32, gain: u32) -> (Block, u64) {
    let p = prior(units);
    let frozen = at_gain(&p, config(units, 2, 0), gain);
    let sums = weights_by_polarity(&frozen);
    let (blocks, trace) = run(
        units,
        2,
        gain,
        0,
        Feedback::Withheld,
        false,
        Delivery::Global,
        BLOCK,
    );
    let block = blocks[0];
    assert_eq!(
        (block.7, block.8),
        sums,
        "no weight moves at a modulation of zero"
    );
    (block, trace)
}

/// The calibration at 256 units, both candidates in the order tried: at 1.75 the window
/// after the volley held more readout spikes than the window before in 50 trials of 64
/// (148 and 181 against 54 and 51 over the block) and at 2.0 in 58 (481 and 516 against 231
/// and 249), so the gain at 256 units is 2.0. The presented set fires 10.9 spikes per
/// presentation in the volley for its eleven units.
pub(crate) const CALIBRATION_256: [(Block, u64); 2] = [
    (
        (
            20,
            34,
            [[80, 98], [68, 83]],
            [374, 329],
            [1300, 1174],
            [54, 51],
            50,
            53_475_744,
            58_968_247,
            0,
            [[1_487_772, 1_409_916], [1_458_815, 1_477_095]],
            21,
        ),
        0xa2cf2b82436c5cc3,
    ),
    (
        (
            25,
            34,
            [[250, 284], [231, 232]],
            [370, 325],
            [1995, 2017],
            [231, 249],
            58,
            53_475_744,
            58_968_247,
            0,
            [[1_487_772, 1_409_916], [1_458_815, 1_477_095]],
            11,
        ),
        0xee01046fee1077e9,
    ),
];

/// The calibration at 1 024 units: at 1.75 the window after held more in 62 trials of 64
/// (775 and 741 against 255 and 241), so the gain at 1 024 units is 1.75; 2.0, tried second,
/// would have passed too (63; 2 119 and 2 094 against 1 293 and 1 215). The presented set
/// fires 50.8 spikes per presentation in the volley for its fifty-one units.
pub(crate) const CALIBRATION_1024: [(Block, u64); 2] = [
    (
        (
            27,
            34,
            [[396, 378], [379, 363]],
            [1727, 1526],
            [6171, 5504],
            [255, 241],
            62,
            213_902_976,
            235_822_619,
            0,
            [[6_986_739, 7_212_710], [7_146_878, 7_257_384]],
            8,
        ),
        0xa84553d901278f4f,
    ),
    (
        (
            25,
            34,
            [[1063, 1050], [1056, 1044]],
            [1697, 1503],
            [10_291, 8904],
            [1293, 1215],
            63,
            213_902_976,
            235_822_619,
            0,
            [[6_986_739, 7_212_710], [7_146_878, 7_257_384]],
            9,
        ),
        0x92d20337e23f16cb,
    ),
];

// --------------------------------------------------------------- the criterion (ADR-0066)

/// The correct trials over the last `LAST_BLOCKS` blocks.
pub(crate) fn last_correct(blocks: &[Block]) -> u32 {
    blocks
        .iter()
        .rev()
        .take(LAST_BLOCKS)
        .fold(0u32, |sum, b| sum.saturating_add(b.0))
}

/// The criterion of ADR-0066, clause by clause: the rewarded run's correct trials over the
/// last 128 at least `REWARDED_MIN`, in both assignments; the shuffled reward's and the
/// fixed modulation's at most `CONTROL_MAX`; the accuracy sequence the same on one worker
/// and on four. `learned` is all five.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Verdict {
    pub(crate) rewarded: bool,
    pub(crate) mirrored: bool,
    pub(crate) shuffled: bool,
    pub(crate) fixed: bool,
    pub(crate) workers: bool,
    pub(crate) learned: bool,
}

pub(crate) fn verdict(
    rewarded: &[Block],
    mirrored: &[Block],
    shuffled: &[Block],
    fixed: &[Block],
    workers: bool,
) -> Verdict {
    let v = Verdict {
        rewarded: last_correct(rewarded) >= REWARDED_MIN,
        mirrored: last_correct(mirrored) >= REWARDED_MIN,
        shuffled: last_correct(shuffled) <= CONTROL_MAX,
        fixed: last_correct(fixed) <= CONTROL_MAX,
        workers,
        learned: false,
    };
    Verdict {
        learned: v.rewarded && v.mirrored && v.shuffled && v.fixed && v.workers,
        ..v
    }
}

/// Row `n` of Pascal's triangle, $\binom{n}{0} \ldots \binom{n}{n}$, in `u128` by the
/// additive rule, which never overflows for `n` at most 128 (the largest coefficient is
/// below $2^{127}$; the multiplicative form's product before its division would).
pub(crate) fn row(n: usize) -> Vec<u128> {
    let mut row = vec![0u128; n.saturating_add(1)];
    row[0] = 1;
    for r in 1..=n {
        for j in (1..=r).rev() {
            row[j] = row[j].saturating_add(row[j.wrapping_sub(1)]);
        }
    }
    row
}

/// $\sum_{j \ge k} \binom{n}{j}$ from the row: the numerator of the binomial's upper tail
/// at $p = \tfrac12$ over $2^n$.
pub(crate) fn tail(row: &[u128], k: usize) -> u128 {
    row.iter()
        .skip(k)
        .fold(0u128, |sum, &c| sum.saturating_add(c))
}

// ------------------------------------------------------------ the measurement (ADR-0066)

/// The 256-unit form at the gain the calibration picked (2.0): 512 trials, eight blocks.
/// Each run is its own weekly `exhaustive` test; the pinned tables below are the runs' as
/// the engine produced them, the criterion's test reads the tables, and the gate runs the
/// rewarded run's first block (ADR-0061).
pub(crate) const REWARDED_256: &[Block] = &[
    (
        28,
        34,
        [[202, 222], [200, 202]],
        [371, 326],
        [1885, 1873],
        [229, 233],
        57,
        52_361_380,
        48_281_155,
        -56_439,
        [[983_329, 938_855], [950_214, 997_274]],
        9,
    ),
    (
        27,
        31,
        [[139, 162], [148, 150]],
        [338, 355],
        [1745, 1851],
        [231, 205],
        46,
        51_528_899,
        43_234_480,
        -112_031,
        [[787_567, 740_290], [779_456, 794_820]],
        8,
    ),
    (
        26,
        31,
        [[140, 157], [135, 148]],
        [335, 361],
        [1682, 1745],
        [191, 217],
        44,
        50_566_801,
        40_014_833,
        87_150,
        [[683_704, 662_362], [670_201, 662_578]],
        11,
    ),
    (
        33,
        28,
        [[130, 112], [120, 137]],
        [304, 392],
        [1560, 1846],
        [196, 175],
        42,
        49_332_645,
        37_323_666,
        -96_335,
        [[604_442, 612_136], [575_761, 589_365]],
        9,
    ),
    (
        37,
        30,
        [[108, 120], [93, 139]],
        [327, 370],
        [1600, 1785],
        [185, 215],
        38,
        48_000_750,
        35_402_883,
        -61_863,
        [[562_351, 568_462], [538_010, 548_929]],
        7,
    ),
    (
        28,
        36,
        [[130, 141], [97, 112]],
        [395, 308],
        [1810, 1606],
        [188, 163],
        42,
        46_895_024,
        34_136_212,
        59_399,
        [[530_695, 539_297], [541_956, 542_158]],
        8,
    ),
    (
        22,
        30,
        [[107, 144], [104, 126]],
        [327, 369],
        [1591, 1787],
        [187, 211],
        36,
        45_746_830,
        33_229_310,
        -106_113,
        [[512_519, 532_004], [522_511, 519_005]],
        13,
    ),
    (
        31,
        32,
        [[130, 121], [101, 123]],
        [349, 350],
        [1589, 1711],
        [189, 222],
        31,
        44_541_544,
        32_394_170,
        18_921,
        [[484_090, 484_645], [510_708, 512_616]],
        6,
    ),
];

pub(crate) const TRACE_256: u64 = 0x4b0afbb563e291c3;

pub(crate) const MIRRORED_256: &[Block] = &[
    (
        32,
        34,
        [[188, 217], [192, 202]],
        [371, 324],
        [1896, 1869],
        [232, 238],
        56,
        52_376_380,
        47_896_122,
        88_856,
        [[943_706, 909_075], [949_340, 984_919]],
        4,
    ),
    (
        28,
        31,
        [[136, 153], [152, 149]],
        [338, 355],
        [1735, 1841],
        [228, 201],
        42,
        51_472_579,
        42_875_947,
        111_625,
        [[767_221, 723_455], [774_070, 774_193]],
        10,
    ),
    (
        28,
        31,
        [[133, 152], [123, 154]],
        [336, 362],
        [1688, 1737],
        [202, 223],
        41,
        50_366_424,
        39_555_619,
        -45_728,
        [[664_222, 642_121], [645_177, 656_215]],
        11,
    ),
    (
        23,
        28,
        [[118, 110], [121, 129]],
        [304, 393],
        [1561, 1844],
        [198, 171],
        42,
        49_400_426,
        37_397_711,
        96_213,
        [[598_989, 577_919], [568_964, 591_399]],
        10,
    ),
    (
        22,
        30,
        [[106, 125], [95, 135]],
        [327, 371],
        [1609, 1804],
        [182, 218],
        39,
        48_294_322,
        35_912_003,
        29_766,
        [[562_735, 541_973], [543_314, 571_182]],
        11,
    ),
    (
        27,
        36,
        [[131, 143], [96, 115]],
        [395, 308],
        [1809, 1604],
        [192, 170],
        40,
        47_141_791,
        34_590_493,
        -34_870,
        [[533_267, 529_352], [549_299, 560_169]],
        9,
    ),
    (
        29,
        30,
        [[110, 141], [109, 127]],
        [327, 370],
        [1594, 1792],
        [182, 219],
        41,
        45_844_271,
        33_506_486,
        -110_453,
        [[515_210, 519_713], [533_129, 544_933]],
        16,
    ),
    (
        24,
        32,
        [[130, 124], [104, 131]],
        [349, 350],
        [1601, 1708],
        [195, 214],
        31,
        44_667_876,
        32_681_564,
        -68_369,
        [[497_659, 496_302], [526_086, 510_303]],
        12,
    ),
];

pub(crate) const SHUFFLED_256: &[Block] = &[
    (
        28,
        34,
        [[207, 219], [190, 193]],
        [371, 325],
        [1910, 1894],
        [236, 231],
        55,
        52_441_684,
        48_874_413,
        105_959,
        [[973_866, 951_500], [963_381, 1_010_598]],
        6,
    ),
    (
        30,
        31,
        [[137, 156], [154, 145]],
        [338, 354],
        [1748, 1860],
        [238, 205],
        44,
        51_609_561,
        43_747_881,
        -47_354,
        [[773_547, 744_582], [797_319, 813_200]],
        5,
    ),
    (
        27,
        31,
        [[134, 156], [130, 157]],
        [335, 362],
        [1689, 1750],
        [198, 219],
        46,
        50_604_594,
        40_110_815,
        49_150,
        [[674_526, 657_540], [667_126, 675_087]],
        10,
    ),
    (
        29,
        28,
        [[124, 115], [123, 128]],
        [304, 392],
        [1558, 1848],
        [200, 173],
        39,
        49_384_153,
        37_445_419,
        111_552,
        [[585_396, 587_847], [563_554, 592_656]],
        11,
    ),
    (
        35,
        30,
        [[106, 117], [100, 134]],
        [327, 372],
        [1614, 1799],
        [178, 210],
        33,
        47_910_985,
        35_396_856,
        -25_029,
        [[552_938, 547_701], [539_215, 554_443]],
        10,
    ),
    (
        28,
        36,
        [[131, 141], [99, 114]],
        [395, 307],
        [1814, 1602],
        [189, 167],
        43,
        46_925_247,
        34_313_813,
        19_035,
        [[534_125, 533_462], [548_288, 541_510]],
        9,
    ),
    (
        22,
        30,
        [[110, 140], [103, 131]],
        [327, 370],
        [1593, 1789],
        [186, 213],
        36,
        45_431_383,
        33_170_249,
        94_524,
        [[512_328, 524_382], [522_604, 525_876]],
        12,
    ),
    (
        29,
        32,
        [[133, 124], [103, 126]],
        [349, 350],
        [1593, 1705],
        [191, 221],
        32,
        44_085_735,
        32_302_095,
        -47_100,
        [[486_908, 490_839], [516_995, 502_421]],
        6,
    ),
];

pub(crate) const FIXED_256: &[Block] = &[
    (
        26,
        34,
        [[195, 215], [185, 185]],
        [370, 326],
        [1874, 1853],
        [215, 224],
        55,
        52_040_170,
        46_218_375,
        0,
        [[909_382, 872_278], [880_088, 923_029]],
        8,
    ),
    (
        25,
        31,
        [[130, 144], [146, 142]],
        [339, 353],
        [1723, 1796],
        [232, 194],
        44,
        50_726_380,
        40_594_139,
        0,
        [[701_740, 676_871], [720_576, 715_831]],
        7,
    ),
    (
        23,
        31,
        [[127, 141], [121, 145]],
        [335, 362],
        [1653, 1721],
        [197, 212],
        42,
        49_220_999,
        37_260_901,
        0,
        [[613_603, 599_581], [604_417, 601_551]],
        15,
    ),
    (
        29,
        28,
        [[117, 100], [118, 126]],
        [305, 392],
        [1537, 1826],
        [200, 173],
        40,
        47_596_723,
        34_891_795,
        0,
        [[548_693, 551_456], [525_831, 541_873]],
        13,
    ),
    (
        35,
        30,
        [[98, 122], [93, 130]],
        [327, 370],
        [1579, 1773],
        [180, 211],
        35,
        45_893_073,
        33_420_974,
        0,
        [[521_736, 527_691], [516_554, 528_002]],
        9,
    ),
    (
        31,
        36,
        [[130, 135], [96, 111]],
        [395, 308],
        [1791, 1594],
        [184, 164],
        40,
        44_332_392,
        32_325_074,
        0,
        [[509_015, 504_647], [525_571, 534_969]],
        7,
    ),
    (
        23,
        30,
        [[104, 138], [101, 125]],
        [327, 370],
        [1586, 1779],
        [179, 211],
        39,
        42_588_464,
        31_447_208,
        0,
        [[489_771, 508_936], [505_183, 508_030]],
        11,
    ),
    (
        28,
        32,
        [[127, 124], [104, 122]],
        [349, 350],
        [1578, 1702],
        [191, 220],
        31,
        40_908_307,
        30_846_947,
        0,
        [[478_579, 479_598], [500_762, 491_982]],
        8,
    ),
];

pub(crate) const VERDICT_256: Verdict = Verdict {
    rewarded: false,
    mirrored: false,
    shuffled: true,
    fixed: true,
    workers: true,
    learned: false,
};

/// The 1 024-unit form at the gain the calibration picked (1.75): 512 trials, eight blocks.
pub(crate) const REWARDED_1024: &[Block] = &[
    (
        27,
        34,
        [[376, 361], [361, 349]],
        [1727, 1525],
        [6142, 5474],
        [249, 229],
        61,
        210_427_900,
        230_916_261,
        -84_933,
        [[6_493_095, 6_799_725], [6_705_098, 6_817_017]],
        8,
    ),
    (
        21,
        31,
        [[304, 329], [359, 344]],
        [1572, 1676],
        [5654, 5889],
        [241, 255],
        64,
        208_008_451,
        227_482_898,
        -112_151,
        [[6_255_219, 6_502_025], [6_421_477, 6_496_745]],
        4,
    ),
    (
        27,
        31,
        [[327, 343], [299, 302]],
        [1577, 1679],
        [5619, 5851],
        [248, 237],
        62,
        204_824_565,
        223_771_696,
        -109_231,
        [[5_913_232, 6_181_071], [6_093_045, 6_244_583]],
        5,
    ),
    (
        31,
        28,
        [[282, 255], [320, 326]],
        [1423, 1827],
        [5115, 6243],
        [265, 220],
        63,
        201_295_600,
        220_527_358,
        109_063,
        [[5_665_304, 5_991_844], [5_802_861, 5_971_959]],
        6,
    ),
    (
        30,
        30,
        [[263, 247], [274, 286]],
        [1530, 1732],
        [5471, 5951],
        [224, 225],
        60,
        197_825_311,
        217_575_437,
        80_911,
        [[5_416_069, 5_799_363], [5_507_726, 5_662_839]],
        11,
    ),
    (
        26,
        36,
        [[272, 248], [248, 218]],
        [1832, 1425],
        [6301, 5097],
        [234, 250],
        58,
        194_611_651,
        215_165_041,
        -88_442,
        [[5_125_532, 5_489_094], [5_363_502, 5_494_672]],
        9,
    ),
    (
        25,
        30,
        [[251, 243], [298, 274]],
        [1525, 1729],
        [5346, 5926],
        [252, 232],
        58,
        191_506_629,
        212_911_869,
        92_059,
        [[4_919_506, 5_340_292], [5_119_722, 5_273_076]],
        7,
    ),
    (
        24,
        32,
        [[234, 278], [258, 256]],
        [1629, 1629],
        [5689, 5619],
        [248, 253],
        57,
        188_702_707,
        211_017_969,
        29_991,
        [[4_784_180, 5_152_084], [4_924_049, 5_108_595]],
        5,
    ),
];

pub(crate) const TRACE_1024: u64 = 0xd35f65f1e45140ab;

pub(crate) const MIRRORED_1024: &[Block] = &[
    (
        34,
        34,
        [[364, 359], [361, 340]],
        [1727, 1525],
        [6129, 5475],
        [253, 233],
        60,
        209_792_108,
        230_207_898,
        69_101,
        [[6_420_795, 6_725_625], [6_705_954, 6_784_935]],
        5,
    ),
    (
        39,
        31,
        [[299, 316], [351, 332]],
        [1572, 1676],
        [5644, 5867],
        [243, 260],
        64,
        205_654_739,
        225_315_860,
        112_093,
        [[6_116_317, 6_331_028], [6_270_822, 6_303_448]],
        5,
    ),
    (
        30,
        31,
        [[321, 344], [296, 296]],
        [1578, 1679],
        [5608, 5842],
        [253, 243],
        62,
        202_241_102,
        221_698_313,
        27_863,
        [[5_747_309, 6_012_548], [5_971_815, 6_065_383]],
        7,
    ),
    (
        22,
        28,
        [[277, 244], [306, 312]],
        [1423, 1829],
        [5111, 6237],
        [272, 214],
        63,
        199_439_026,
        219_139_194,
        -109_934,
        [[5_538_155, 5_837_081], [5_715_329, 5_843_072]],
        10,
    ),
    (
        22,
        30,
        [[262, 238], [285, 281]],
        [1530, 1732],
        [5469, 5949],
        [217, 228],
        60,
        196_872_179,
        216_905_494,
        -80_948,
        [[5_283_881, 5_631_537], [5_538_397, 5_659_971]],
        11,
    ),
    (
        32,
        36,
        [[261, 244], [239, 217]],
        [1833, 1425],
        [6299, 5093],
        [235, 259],
        58,
        193_408_520,
        214_290_049,
        88_442,
        [[5_030_693, 5_362_796], [5_350_038, 5_410_356]],
        5,
    ),
    (
        32,
        30,
        [[243, 242], [290, 276]],
        [1525, 1730],
        [5341, 5917],
        [247, 235],
        58,
        189_512_145,
        211_851_286,
        -92_093,
        [[4_795_672, 5_195_167], [5_114_786, 5_172_675]],
        7,
    ),
    (
        33,
        32,
        [[238, 277], [250, 251]],
        [1628, 1629],
        [5658, 5611],
        [231, 260],
        59,
        185_500_296,
        209_518_496,
        -91_027,
        [[4_615_099, 4_989_680], [4_897_248, 4_997_023]],
        8,
    ),
];

pub(crate) const SHUFFLED_1024: &[Block] = &[
    (
        27,
        34,
        [[368, 361], [367, 345]],
        [1727, 1524],
        [6142, 5473],
        [256, 231],
        61,
        210_639_650,
        231_216_688,
        105_959,
        [[6_490_716, 6_773_268], [6_749_565, 6_837_479]],
        5,
    ),
    (
        22,
        31,
        [[314, 328], [355, 337]],
        [1572, 1676],
        [5660, 5887],
        [237, 259],
        64,
        207_699_748,
        227_567_247,
        -47_354,
        [[6_216_739, 6_416_894], [6_417_147, 6_491_039]],
        5,
    ),
    (
        26,
        31,
        [[324, 334], [296, 306]],
        [1578, 1679],
        [5633, 5862],
        [254, 241],
        61,
        204_324_583,
        223_701_566,
        49_150,
        [[5_852_301, 6_108_536], [6_078_916, 6_213_713]],
        10,
    ),
    (
        32,
        28,
        [[285, 248], [315, 320]],
        [1423, 1828],
        [5115, 6243],
        [272, 223],
        62,
        200_484_804,
        220_236_635,
        111_552,
        [[5_607_337, 5_887_031], [5_729_530, 5_913_131]],
        6,
    ),
    (
        33,
        30,
        [[270, 248], [271, 282]],
        [1530, 1732],
        [5461, 5947],
        [225, 230],
        60,
        196_108_503,
        216_619_182,
        -25_029,
        [[5_313_039, 5_642_233], [5_412_583, 5_585_270]],
        11,
    ),
    (
        28,
        36,
        [[268, 240], [238, 217]],
        [1833, 1425],
        [6303, 5095],
        [234, 257],
        58,
        193_029_377,
        214_275_489,
        19_035,
        [[5_056_613, 5_375_196], [5_260_653, 5_391_522]],
        9,
    ),
    (
        27,
        30,
        [[244, 238], [283, 275]],
        [1525, 1730],
        [5342, 5922],
        [246, 231],
        58,
        188_598_008,
        211_483_797,
        94_524,
        [[4_807_940, 5_187_464], [5_001_768, 5_123_320]],
        3,
    ),
    (
        28,
        32,
        [[247, 274], [248, 255]],
        [1629, 1629],
        [5671, 5609],
        [235, 254],
        59,
        184_707_688,
        209_162_019,
        -47_100,
        [[4_643_423, 4_974_975], [4_780_585, 4_952_133]],
        7,
    ),
];

pub(crate) const FIXED_1024: &[Block] = &[
    (
        26,
        34,
        [[363, 357], [362, 335]],
        [1727, 1525],
        [6119, 5467],
        [252, 232],
        61,
        208_089_383,
        228_225_119,
        0,
        [[6_289_430, 6_627_747], [6_574_171, 6_675_503]],
        4,
    ),
    (
        21,
        31,
        [[291, 312], [339, 332]],
        [1572, 1677],
        [5611, 5844],
        [244, 260],
        64,
        202_727_661,
        222_337_377,
        0,
        [[5_940_067, 6_166_206], [6_097_651, 6_173_941]],
        7,
    ),
    (
        29,
        31,
        [[307, 333], [278, 285]],
        [1578, 1679],
        [5569, 5819],
        [250, 242],
        62,
        197_291_101,
        217_293_045,
        0,
        [[5_522_296, 5_796_889], [5_684_375, 5_847_779]],
        4,
    ),
    (
        36,
        28,
        [[265, 232], [282, 300]],
        [1424, 1829],
        [5062, 6201],
        [269, 221],
        61,
        191_556_498,
        213_234_844,
        0,
        [[5_234_146, 5_571_742], [5_315_193, 5_520_576]],
        5,
    ),
    (
        30,
        30,
        [[245, 234], [263, 263]],
        [1530, 1732],
        [5415, 5901],
        [212, 221],
        60,
        185_885_652,
        209_732_113,
        0,
        [[4_937_632, 5_351_981], [4_999_732, 5_205_343]],
        10,
    ),
    (
        28,
        36,
        [[251, 231], [207, 196]],
        [1833, 1427],
        [6246, 5055],
        [245, 255],
        56,
        180_206_849,
        206_642_589,
        0,
        [[4_662_813, 5_017_097], [4_804_684, 4_946_144]],
        8,
    ),
    (
        29,
        30,
        [[227, 231], [277, 274]],
        [1527, 1730],
        [5291, 5871],
        [250, 239],
        57,
        174_305_265,
        203_905_215,
        0,
        [[4_425_440, 4_821_311], [4_547_152, 4_695_690]],
        5,
    ),
    (
        31,
        32,
        [[231, 265], [223, 238]],
        [1629, 1629],
        [5630, 5579],
        [242, 258],
        56,
        168_442_172,
        201_552_838,
        0,
        [[4_243_523, 4_607_215], [4_305_115, 4_514_524]],
        3,
    ),
];

pub(crate) const VERDICT_1024: Verdict = Verdict {
    rewarded: false,
    mirrored: false,
    shuffled: true,
    fixed: true,
    workers: true,
    learned: false,
};

// ------------------------------------------- the addressed delivery (ADR-0068, ADR-0069)
//
// Written before the run. The same instrument, the same controls, the same seeds and the
// same counts as ADR-0066; the one variable is where the dopamine term reaches: under
// `Delivery::Addressed` the synapses onto the readout the engine selected, none at a tie
// (the addressed set is the outcome's, written by the task between the trial's last tick and
// the reward; the presynaptic side does not narrow it, ADR-0068), under `Delivery::Global`
// every synapse alike, ADR-0066's form, which reruns under this round's code as the
// comparison the claim rests on. The criterion is at 1 024 units; 256 units is one reading
// under the rule below.

/// The rule for a size that stops seeing, written before the run: a size whose calibration
/// measure (the trials of a block in which the window after the volley held more readout
/// spikes than the window before the injection) falls below `SEEN_MIN` in any block before
/// the criterion's window is recorded as not measured, never as a pass or a fail. ADR-0066
/// read 256 units below the mark from its second block and 1 024 units above it throughout.
pub(crate) fn sees_through(blocks: &[Block]) -> bool {
    let before_window = blocks.len().saturating_sub(LAST_BLOCKS);
    blocks
        .iter()
        .take(before_window)
        .all(|block| block.6 >= SEEN_MIN)
}

/// The criterion under the addressed delivery (ADR-0069), ADR-0066's clause by clause with
/// the global form beside the controls: the addressed rewarded run's correct trials over the
/// last 128 at least `REWARDED_MIN`, in both assignments; the addressed shuffled reward's,
/// the fixed modulation's and the global form's at most `CONTROL_MAX`; the addressed
/// rewarded run's sequence the same on one worker and on four. `learned` is all six.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AddressedVerdict {
    pub(crate) rewarded: bool,
    pub(crate) mirrored: bool,
    pub(crate) shuffled: bool,
    pub(crate) fixed: bool,
    pub(crate) global: bool,
    pub(crate) workers: bool,
    pub(crate) learned: bool,
}

pub(crate) fn addressed_verdict(
    rewarded: &[Block],
    mirrored: &[Block],
    shuffled: &[Block],
    fixed: &[Block],
    global: &[Block],
    workers: bool,
) -> AddressedVerdict {
    let v = AddressedVerdict {
        rewarded: last_correct(rewarded) >= REWARDED_MIN,
        mirrored: last_correct(mirrored) >= REWARDED_MIN,
        shuffled: last_correct(shuffled) <= CONTROL_MAX,
        fixed: last_correct(fixed) <= CONTROL_MAX,
        global: last_correct(global) <= CONTROL_MAX,
        workers,
        learned: false,
    };
    AddressedVerdict {
        learned: v.rewarded && v.mirrored && v.shuffled && v.fixed && v.global && v.workers,
        ..v
    }
}

// ------------------------------------------------- the measurement, addressed (ADR-0069)

/// The addressed rewarded run at 1 024 units, the gain 1.75: 512 trials, eight blocks, on
/// four workers; the same run on one worker. The other runs at 1 024 units and the one
/// reading at 256 follow. Every table is the engine's own, pinned from one run.
pub(crate) const ADDRESSED_1024: &[Block] = &[
    (
        24,
        34,
        [[363, 359], [363, 339]],
        [1727, 1525],
        [6138, 5469],
        [251, 232],
        61,
        209840321,
        230036999,
        -91922,
        [[6352193, 6767883], [6695954, 6741722]],
        8,
    ),
    (
        20,
        31,
        [[299, 320], [350, 328]],
        [1572, 1676],
        [5636, 5869],
        [240, 263],
        64,
        206116890,
        225361098,
        -112151,
        [[6034948, 6364842], [6315559, 6274589]],
        8,
    ),
    (
        29,
        31,
        [[324, 345], [291, 300]],
        [1577, 1679],
        [5601, 5840],
        [251, 238],
        62,
        202335976,
        221198844,
        -87406,
        [[5638657, 6057918], [5949954, 5968162]],
        5,
    ),
    (
        30,
        28,
        [[270, 248], [301, 317]],
        [1423, 1828],
        [5098, 6224],
        [273, 220],
        62,
        198336847,
        217793631,
        87077,
        [[5374260, 5860103], [5611568, 5676560]],
        9,
    ),
    (
        32,
        30,
        [[251, 243], [275, 276]],
        [1530, 1732],
        [5453, 5935],
        [219, 225],
        59,
        194479928,
        214719302,
        80936,
        [[5091101, 5671035], [5322333, 5360730]],
        9,
    ),
    (
        25,
        36,
        [[253, 247], [230, 206]],
        [1833, 1425],
        [6291, 5083],
        [239, 260],
        58,
        190556391,
        212037088,
        -88451,
        [[4808504, 5382082], [5139875, 5115799]],
        6,
    ),
    (
        29,
        30,
        [[245, 243], [278, 272]],
        [1525, 1730],
        [5328, 5911],
        [250, 233],
        57,
        186505360,
        209547846,
        92165,
        [[4577601, 5220432], [4896016, 4867378]],
        3,
    ),
    (
        27,
        32,
        [[226, 274], [246, 239]],
        [1628, 1629],
        [5657, 5602],
        [228, 244],
        58,
        182495646,
        207400983,
        31411,
        [[4416757, 5031874], [4698487, 4700314]],
        5,
    ),
];

pub(crate) const ADDRESSED_TRACE_1024: u64 = 0x35238a0892c87363;

pub(crate) const ADDRESSED_MIRRORED_1024: &[Block] = &[
    (
        31,
        34,
        [[365, 358], [361, 342]],
        [1727, 1525],
        [6137, 5469],
        [251, 232],
        60,
        209843510,
        229951437,
        84733,
        [[6387363, 6666469], [6639144, 6784676]],
        7,
    ),
    (
        35,
        31,
        [[298, 312], [343, 332]],
        [1572, 1677],
        [5632, 5873],
        [239, 263],
        64,
        206124863,
        225217502,
        112093,
        [[6070946, 6240702], [6199346, 6327555]],
        9,
    ),
    (
        30,
        31,
        [[322, 345], [286, 296]],
        [1577, 1679],
        [5597, 5839],
        [250, 240],
        62,
        202353296,
        221015892,
        27863,
        [[5691518, 5897219], [5810546, 6047872]],
        4,
    ),
    (
        22,
        28,
        [[272, 237], [295, 316]],
        [1423, 1828],
        [5095, 6229],
        [272, 219],
        63,
        198360989,
        217641444,
        -109934,
        [[5439822, 5687188], [5466490, 5769290]],
        9,
    ),
    (
        22,
        30,
        [[259, 233], [273, 271]],
        [1530, 1732],
        [5451, 5935],
        [218, 227],
        59,
        194505947,
        214630001,
        -80939,
        [[5191866, 5472155], [5180974, 5503674]],
        11,
    ),
    (
        31,
        36,
        [[261, 237], [225, 213]],
        [1833, 1425],
        [6293, 5083],
        [236, 255],
        58,
        190597661,
        211950344,
        86813,
        [[4947506, 5168378], [4997364, 5266694]],
        7,
    ),
    (
        33,
        30,
        [[239, 235], [285, 274]],
        [1525, 1730],
        [5323, 5910],
        [245, 236],
        56,
        186553543,
        209439531,
        -92165,
        [[4711272, 4994859], [4732740, 5016482]],
        6,
    ),
    (
        32,
        32,
        [[236, 269], [240, 240]],
        [1628, 1629],
        [5652, 5592],
        [230, 248],
        58,
        182553563,
        207266033,
        -91027,
        [[4543951, 4792801], [4519381, 4854166]],
        6,
    ),
];

pub(crate) const ADDRESSED_SHUFFLED_1024: &[Block] = &[
    (
        25,
        34,
        [[365, 358], [363, 341]],
        [1727, 1525],
        [6138, 5469],
        [251, 231],
        61,
        209836061,
        230015138,
        105959,
        [[6367139, 6743062], [6659622, 6760931]],
        7,
    ),
    (
        21,
        31,
        [[299, 319], [344, 328]],
        [1572, 1677],
        [5633, 5872],
        [240, 263],
        64,
        206117930,
        225293328,
        -47354,
        [[6048682, 6332423], [6244718, 6296918]],
        8,
    ),
    (
        31,
        31,
        [[325, 347], [289, 299]],
        [1577, 1679],
        [5601, 5841],
        [251, 240],
        62,
        202332944,
        221115510,
        49150,
        [[5651943, 6023395], [5860626, 6001721]],
        3,
    ),
    (
        33,
        28,
        [[270, 246], [296, 317]],
        [1423, 1829],
        [5096, 6226],
        [271, 220],
        62,
        198344261,
        217709708,
        111552,
        [[5396395, 5818960], [5517275, 5713211]],
        7,
    ),
    (
        33,
        30,
        [[257, 244], [272, 276]],
        [1530, 1732],
        [5454, 5940],
        [219, 225],
        60,
        194494267,
        214590843,
        -25029,
        [[5131871, 5597278], [5224073, 5413314]],
        9,
    ),
    (
        25,
        36,
        [[256, 247], [228, 212]],
        [1833, 1425],
        [6290, 5079],
        [238, 254],
        58,
        190568856,
        211952745,
        19035,
        [[4865707, 5317038], [5048221, 5173250]],
        6,
    ),
    (
        26,
        30,
        [[244, 238], [285, 272]],
        [1525, 1730],
        [5325, 5911],
        [251, 235],
        57,
        186507189,
        209458063,
        94524,
        [[4625870, 5132276], [4795081, 4939273]],
        5,
    ),
    (
        27,
        32,
        [[228, 274], [240, 240]],
        [1629, 1629],
        [5658, 5600],
        [231, 251],
        57,
        182504612,
        207261475,
        -47100,
        [[4458442, 4929297], [4581524, 4760296]],
        5,
    ),
];

pub(crate) const ADDRESSED_256: &[Block] = &[
    (
        29,
        34,
        [[194, 214], [193, 196]],
        [371, 325],
        [1889, 1855],
        [230, 239],
        55,
        52318657,
        47323437,
        -110011,
        [[913694, 936764], [945767, 950517]],
        6,
    ),
    (
        21,
        31,
        [[132, 159], [145, 135]],
        [337, 354],
        [1722, 1819],
        [233, 198],
        44,
        51222726,
        41858984,
        -112167,
        [[711414, 712805], [753918, 729555]],
        7,
    ),
    (
        22,
        31,
        [[133, 151], [121, 150]],
        [335, 362],
        [1661, 1745],
        [196, 223],
        43,
        49988841,
        38571473,
        19763,
        [[619518, 639091], [630845, 612368]],
        14,
    ),
    (
        28,
        28,
        [[119, 114], [116, 127]],
        [305, 392],
        [1551, 1834],
        [192, 163],
        40,
        48645459,
        36201873,
        -98136,
        [[554662, 587762], [547639, 555299]],
        13,
    ),
    (
        38,
        30,
        [[105, 117], [92, 131]],
        [327, 371],
        [1593, 1787],
        [176, 214],
        34,
        47196507,
        34606708,
        69209,
        [[529727, 551881], [532705, 533423]],
        7,
    ),
    (
        29,
        36,
        [[124, 137], [95, 115]],
        [395, 308],
        [1797, 1595],
        [192, 170],
        38,
        45894396,
        33421695,
        31485,
        [[508055, 527113], [531193, 537457]],
        8,
    ),
    (
        22,
        30,
        [[107, 138], [105, 129]],
        [327, 370],
        [1590, 1786],
        [186, 216],
        38,
        44392493,
        32468169,
        24959,
        [[495266, 522135], [509156, 518023]],
        12,
    ),
    (
        33,
        32,
        [[134, 126], [100, 127]],
        [349, 350],
        [1592, 1702],
        [190, 215],
        31,
        42961889,
        31729687,
        68263,
        [[479084, 488171], [505446, 502122]],
        6,
    ),
];

/// The criterion's outcome at 1 024 units under the addressed delivery, as the engine
/// produced it: the addressed rewarded run reads 56 correct of the last 128 and the mirrored
/// assignment 65, against the 80 the clause asks for; the addressed shuffled reward 53, the
/// fixed modulation 60 and the global form 49, at or below 76; the run the same on one worker
/// and on four. The rewarded clause fails at 1 024 units with the global form reproduced.
pub(crate) const ADDRESSED_VERDICT_1024: AddressedVerdict = AddressedVerdict {
    rewarded: false,
    mirrored: false,
    shuffled: true,
    fixed: true,
    global: true,
    workers: true,
    learned: false,
};

/// The reading at 256 units: the calibration's measure is 55 of 64 in the first block, below
/// the mark, so the size is not measured under the rule written before the run, as
/// ADR-0066's run was not.
pub(crate) const ADDRESSED_256_SEES: bool = false;

// ------------------------------------------------ where 256 units settle (brief 031)

/// One window of the settling measurement: the population's spikes over the window, read
/// from the executor's train; the sum of the inhibitory magnitudes and the sum of the
/// excitatory weights over the arena after it; and the fraction of units at the inhibitory
/// rule's target over it (ADR-0057's rule at the image's period, Q16.16).
pub(crate) type SettlingWindow = (u64, i64, i64, u32);

/// Runs `windows` whole windows under `drive` from the executor's clock: brief 031's
/// lead-in, in windows, which precedes the instrument's own lead-in of one readout window
/// (`LEAD_IN`); a countdown, so it ends by construction. Not `settle`, which in
/// `reference.rs` runs quiet until the network is quiescent.
pub(crate) fn lead_in(exec: &mut Engine, drive: &Drive, windows: u64) {
    for _ in 0..windows {
        let until = exec.ticks().wrapping_add(WINDOW_TICKS);
        run_driven(exec, drive, until).expect("the drive runs");
    }
}

/// The population's spikes over `[from, to)` and the fraction of units at the target over
/// it, read from the train, which must hold the whole window: a unit is at the target when
/// its spikes in the window are within a factor of two of the target's count per window,
/// `WINDOW_TICKS / period` (six at the default period of 20 000 ticks), inclusive both
/// ways (ADR-0057). The run's ticks stay below the stamp's width, so the stamp is the tick.
pub(crate) fn spikes_and_at_target(exec: &mut Engine, from: u64, to: u64) -> (u64, u32) {
    let units = exec.units().len();
    let period = exec.istdp_target_period_ticks();
    // The period is at least `ISTDP_PERIOD_MIN_TICKS`, so the quotient exists.
    let target = WINDOW_TICKS.checked_div(u64::from(period)).unwrap_or(0) as u32;
    let mut counts = vec![0u32; units];
    let mut spikes = 0u64;
    for &(tick, unit) in exec.train() {
        let tick = u64::from(tick);
        if tick >= from && tick < to {
            spikes = spikes.saturating_add(1);
            if let Some(count) = counts.get_mut(unit as usize) {
                *count = count.saturating_add(1);
            }
        }
    }
    let at_target = counts
        .iter()
        .filter(|&&count| count.saturating_mul(2) >= target && count <= target.saturating_mul(2))
        .count() as u64;
    // At most the unit count, which is below 2^16: the shift cannot wrap.
    let fraction = (at_target << 16).checked_div(units as u64).unwrap_or(0) as u32;
    (spikes, fraction)
}

/// The excitatory sums of a settling table, window by window.
pub(crate) fn excitatory_sums(table: &[SettlingWindow]) -> Vec<i64> {
    table.iter().map(|w| w.2).collect()
}

/// Brief 026's clause over the excitatory sums of a settling table, `prior` the sum before
/// the first window: the smallest window (from one) at which each of the last
/// `SETTLING_CLAUSE_WINDOWS` windows up to and including it moved the sum by less than
/// `SETTLING_CLAUSE_PER_CENT` per cent of the sum before those windows; none when no window
/// of the table does. Integers throughout: a move of `m` against a reference of `r` is under
/// `p` per cent when `100 m < p r`.
pub(crate) fn settled_at(prior: i64, sums: &[i64]) -> Option<usize> {
    let sum_after = |window: usize| -> Option<i64> {
        window
            .checked_sub(1)
            .map_or(Some(prior), |k| sums.get(k).copied())
    };
    (SETTLING_CLAUSE_WINDOWS..=sums.len()).find(|&window| {
        let first = window.wrapping_sub(SETTLING_CLAUSE_WINDOWS);
        let Some(reference) = sum_after(first) else {
            return false;
        };
        let bound = u64::try_from(reference)
            .unwrap_or(0)
            .saturating_mul(SETTLING_CLAUSE_PER_CENT);
        (first.wrapping_add(1)..=window).all(|k| {
            match (sum_after(k), sum_after(k.wrapping_sub(1))) {
                (Some(now), Some(before)) => now.abs_diff(before).saturating_mul(100) < bound,
                _ => false,
            }
        })
    })
}

/// The lead-in the rule derives from a settling table of `bound` windows: the window at
/// which the clause first holds, or the bound.
pub(crate) fn derived_lead_in(prior: i64, table: &[SettlingWindow], bound: u64) -> u64 {
    settled_at(prior, &excitatory_sums(table)).map_or(bound, |window| window as u64)
}

/// The settling measurement (brief 031): the executor of the task at `units`, as `run`
/// builds it for the rewarded runs (the prior of ADR-0044 at seed 22, the gain the
/// calibration picked held through the image, the modulation baseline 0.5, no controller,
/// no sleep, the inhibitory period at its default), run under the drive alone, no task and
/// no stimulus, for `windows` whole windows from the first tick; per window the reading
/// above. The sums before the first window are the prior's, as the calibration pinned them.
pub(crate) fn settling(units: u32, windows: u64) -> Vec<SettlingWindow> {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, 2, BASELINE_Q16), gain(units));
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain(units));
    assert_eq!(exec.modulation_baseline_q16(), BASELINE_Q16);
    let drive = drive(units);
    let mut out = Vec::new();
    for _ in 0..windows {
        let from = exec.ticks();
        let held = exec.train().len() as u64;
        let overwritten = exec.train_overwritten();
        lead_in(&mut exec, &drive, 1);
        let to = exec.ticks();
        // The ring lets go of its oldest entries once full (about 27 windows in at this
        // size): the window is held whole when nothing let go was younger than it, that
        // is, no more than the ring held before it.
        assert!(
            exec.train_overwritten().wrapping_sub(overwritten) <= held,
            "the train held the window"
        );
        let (spikes, at_target) = spikes_and_at_target(&mut exec, from, to);
        let (inhibitory, excitatory) = weights_by_polarity(&exec);
        out.push((spikes, inhibitory, excitatory, at_target));
    }
    out
}

/// The prior's sums at 256 units before any window, as the calibration pinned them with
/// the weights frozen: (inhibitory, excitatory).
pub(crate) const PRIOR_SUMS_256: (i64, i64) = (CALIBRATION_256[1].0.7, CALIBRATION_256[1].0.8);

/// The settling at 256 units over eighty windows, pinned from one run.
pub(crate) const SETTLING_256: &[SettlingWindow] = &[
    (3_090, 53_264_642, 56_693_908, 38_656),
    (2_506, 53_098_805, 55_376_488, 50_944),
    (2_656, 52_941_813, 53_987_339, 47_104),
    (2_595, 52_826_485, 52_729_916, 48_128),
    (2_563, 52_694_975, 51_628_294, 51_200),
    (2_487, 52_522_141, 50_691_256, 50_944),
    (2_481, 52_382_720, 49_769_277, 50_688),
    (2_453, 52_253_156, 48_953_681, 51_968),
    (2_352, 52_143_204, 48_210_448, 52_224),
    (2_480, 52_044_866, 47_404_654, 52_480),
    (2_371, 51_896_212, 46_688_151, 53_248),
    (2_421, 51_755_304, 46_025_327, 50_688),
    (2_294, 51_580_011, 45_439_030, 55_808),
    (2_338, 51_412_291, 44_905_736, 54_016),
    (2_225, 51_213_362, 44_393_555, 55_040),
    (2_352, 51_077_296, 43_862_956, 54_784),
    (2_232, 50_936_006, 43_438_627, 55_296),
    (2_241, 50_793_923, 43_008_702, 56_832),
    (2_182, 50_609_680, 42_622_827, 58_112),
    (2_270, 50_460_955, 42_216_613, 56_320),
    (2_231, 50_287_690, 41_762_576, 55_296),
    (2_183, 50_113_080, 41_407_926, 56_064),
    (2_228, 49_954_210, 41_091_863, 56_320),
    (2_199, 49_811_562, 40_715_016, 57_856),
    (2_197, 49_657_873, 40_371_087, 54_784),
    (2_104, 49_464_144, 40_011_682, 58_112),
    (2_236, 49_319_257, 39_669_924, 57_856),
    (2_182, 49_077_710, 39_389_616, 56_064),
    (2_122, 48_881_959, 39_101_242, 57_856),
    (2_187, 48_710_409, 38_820_609, 56_320),
    (2_102, 48_542_690, 38_573_708, 57_344),
    (2_139, 48_373_885, 38_321_850, 58_368),
    (2_180, 48_226_961, 38_092_697, 58_112),
    (2_193, 48_052_772, 37_819_065, 57_344),
    (2_033, 47_864_074, 37_591_169, 57_600),
    (2_066, 47_670_482, 37_413_730, 58_880),
    (2_148, 47_480_584, 37_211_645, 56_064),
    (2_033, 47_255_921, 37_019_781, 57_600),
    (2_180, 47_074_219, 36_819_756, 58_112),
    (2_067, 46_845_222, 36_630_250, 59_136),
    (2_092, 46_615_556, 36_462_159, 57_856),
    (2_070, 46_462_487, 36_282_041, 58_880),
    (2_110, 46_322_331, 36_112_512, 59_648),
    (2_098, 46_128_940, 35_962_763, 59_904),
    (2_132, 45_967_418, 35_836_423, 57_344),
    (2_069, 45_774_994, 35_700_717, 59_392),
    (2_122, 45_591_778, 35_540_372, 58_368),
    (2_127, 45_405_859, 35_376_910, 57_856),
    (2_053, 45_176_056, 35_242_937, 57_856),
    (2_049, 45_012_321, 35_103_513, 59_904),
    (2_105, 44_829_653, 34_983_990, 58_880),
    (2_079, 44_638_991, 34_866_758, 59_136),
    (1_986, 44_419_882, 34_779_473, 58_368),
    (2_108, 44_222_708, 34_629_361, 58_368),
    (2_025, 44_027_980, 34_503_692, 59_904),
    (2_024, 43_793_594, 34_409_536, 58_368),
    (2_114, 43_617_556, 34_285_231, 59_136),
    (2_028, 43_423_068, 34_171_957, 58_624),
    (2_034, 43_202_652, 34_074_752, 58_368),
    (2_065, 43_029_689, 34_010_524, 59_136),
    (2_111, 42_835_647, 33_899_485, 59_648),
    (1_965, 42_623_357, 33_826_291, 59_392),
    (2_090, 42_474_661, 33_741_523, 57_088),
    (2_037, 42_257_106, 33_632_752, 60_416),
    (2_058, 42_044_259, 33_529_204, 57_856),
    (1_982, 41_827_754, 33_470_054, 58_112),
    (1_952, 41_606_594, 33_381_882, 59_136),
    (2_000, 41_402_480, 33_285_922, 58_112),
    (2_027, 41_193_201, 33_148_561, 58_624),
    (2_054, 40_997_922, 33_082_612, 59_648),
    (2_055, 40_824_662, 33_027_666, 60_160),
    (2_039, 40_611_438, 32_950_616, 57_088),
    (2_078, 40_403_489, 32_834_325, 57_856),
    (2_118, 40_201_031, 32_787_524, 59_136),
    (2_145, 40_021_536, 32_740_288, 58_880),
    (2_056, 39_824_386, 32_725_565, 59_136),
    (2_058, 39_613_863, 32_669_906, 58_112),
    (2_087, 39_401_641, 32_618_796, 56_832),
    (2_035, 39_243_885, 32_595_157, 59_392),
    (2_024, 39_053_140, 32_515_504, 56_832),
];

/// The criterion behind the lead-in (brief 031), written before the run: the calibration's
/// measure (the trials of a block in which the window after the volley held more readout
/// spikes than the window before the injection) at least `SEEN_MIN` of 64 in every block of
/// the run; false for no blocks. Holding, 256 units is usable for this task behind the
/// lead-in and a later round may carry a criterion there; failing, it is not, and the block
/// at which the measure crosses is the reading. The correct trials are a reading, never a
/// clause: the run measures the instrument, not learning.
pub(crate) fn holds_through(blocks: &[Block]) -> bool {
    !blocks.is_empty() && blocks.iter().all(|block| block.6 >= SEEN_MIN)
}

/// The rewarded run at 256 units behind the lead-in (brief 031): ADR-0066's rewarded run in
/// the task's order — the same prior, gain, geometry, window, trial, block, baseline,
/// reward, seeds and four workers — with `LEAD_IN_WINDOWS` whole windows under the drive
/// alone before the instrument's own lead-in. The run without them is
/// `the_recalibrated_rewarded_run_at_256_units_on_four_workers_exhaustive`, held to
/// `REWARDED_256`, ADR-0066's table, so the windows are the only difference between the
/// two. Pinned from one run, with the trace of every trial's stimulus, selection and
/// outcome.
pub(crate) const LEAD_IN_256: &[Block] = &[
    (
        36,
        34,
        [[189, 184], [125, 172]],
        [370, 324],
        [1860, 1778],
        [219, 213],
        50,
        51_087_300,
        42_302_966,
        86_187,
        [[914_350, 828_681], [866_868, 893_382]],
        7,
    ),
    (
        29,
        31,
        [[151, 144], [138, 158]],
        [338, 363],
        [1709, 1783],
        [197, 216],
        44,
        50_089_181,
        39_078_798,
        103_842,
        [[760_717, 707_823], [706_612, 734_776]],
        12,
    ),
    (
        30,
        31,
        [[128, 124], [131, 133]],
        [337, 359],
        [1696, 1746],
        [189, 180],
        43,
        48_848_769,
        36_580_685,
        -43_878,
        [[635_297, 610_438], [618_391, 626_872]],
        14,
    ),
    (
        32,
        28,
        [[94, 112], [99, 140]],
        [306, 392],
        [1542, 1833],
        [181, 205],
        35,
        47_558_929,
        34_948_120,
        39_078,
        [[574_253, 570_099], [567_596, 572_557]],
        6,
    ),
    (
        26,
        30,
        [[106, 118], [125, 132]],
        [328, 371],
        [1604, 1819],
        [189, 172],
        41,
        46_405_804,
        33_764_237,
        -39_813,
        [[538_332, 547_718], [551_144, 564_469]],
        12,
    ),
    (
        30,
        36,
        [[122, 147], [80, 115]],
        [394, 305],
        [1798, 1599],
        [191, 219],
        34,
        45_123_980,
        32_828_499,
        43_056,
        [[501_695, 500_964], [538_489, 547_462]],
        11,
    ),
    (
        21,
        30,
        [[103, 120], [131, 126]],
        [329, 371],
        [1537, 1796],
        [176, 218],
        36,
        44_206_626,
        32_233_480,
        -112_225,
        [[496_818, 477_874], [530_668, 535_936]],
        13,
    ),
    (
        29,
        32,
        [[111, 126], [101, 116]],
        [350, 349],
        [1648, 1684],
        [194, 197],
        36,
        42_882_344,
        31_666_316,
        27_149,
        [[489_599, 486_545], [527_159, 509_825]],
        7,
    ),
];

pub(crate) const LEAD_IN_TRACE_256: u64 = 0x379636313802f88d;

/// The criterion's outcome behind the lead-in, as the engine produced it: the calibration's
/// measure is 50 of 64 in the first block, below the mark from the start, and 34 to 44
/// after it, so it holds in no block of the run; 256 units is not usable for this task
/// behind the lead-in the rule derived. The gate reads the rule over the pinned table.
pub(crate) const LEAD_IN_HOLDS: bool = false;

// ------------------------------------------ what the trace is made of (brief 032)
//
// Written before the run. Three decisions (ADR-0066, ADR-0068, ADR-0069) reasoned from one
// sentence about the eligibility trace on a stimulus's synapses into a readout — "a readout
// unit's background spike within the pair window before the volley is depression, and the
// background's are the more" — and nothing in the tree held a number for it. The
// composition reads it at 1 024 units, in the calibration's configuration (the weights
// frozen: the modulation baseline at zero and no reward, so the traces fill and decay and
// are never written into a weight), over sixty-four trials, per trial: (a) the record's
// summed `eligibility_q1_15` over the synapses from each stimulus set onto each readout
// set; (b) the terms that entered those traces since the previous reading, split by what
// paired them and by their sign; (c) the readouts' spikes in the pair window before the
// volley's arrival and in the pair window after it. The terms are an oracle's: the pair
// rule of `cortex-core` (ADR-0022, ADR-0032, ADR-0049, ADR-0055) replayed over the
// executor's own train, synapse by synapse, whose trace must equal the record's at every
// reading, so that the split is of the numbers the engine computed and of nothing else.
//
// The split, a rule of `STDP_TAU_SHIFT` and the volley's arrival: the rule enters a term at
// a presynaptic spike for the target's last somatic spike $q$, a potentiation when $q$
// followed the block's previous presynaptic spike and a depression when $q$ preceded this
// one; a term is **the volley's** when $q$ lies within one pair window ($2^{11}$ ticks) from
// the arrival of that synapse's volley message at the readout — the presented stimulus
// unit's volley spike plus the synapse's own delay — and **the background's** otherwise.
// The census (c) reads the readouts around the readout window's opening, the trial's first
// tick plus the prior's `delay_min`, before which no local message of the volley can have
// landed (ADR-0065's rule).
//
// Then the ladder: the gain, the one handle the calibration turns (ADR-0065), tried
// downward from the present in a fixed order, each rung a frozen run of sixty-four trials
// read by two measures that see no selection, reward or outcome — *sight*, ADR-0065's,
// unchanged, and *sign*, this round's: the presented stimulus's summed eligibility onto
// both readouts positive in at least fifty-six trials of sixty-four. A rung passes only
// when both pass; the first that does is the gain, and if none does there is no rewarded
// run. The expectation, written first: the background's terms are the product of two
// rates, the stimulus units' and the readouts', and the volley's the product of one and a
// response, so a quieter gain shrinks the background's faster than the volley's; whether a
// rung reaches net potentiation before the sight fails is the measurement.

/// The pair window, one STDP time constant: $2^{11}$ ticks (20.48 ms).
pub(crate) const PAIR_WINDOW: u32 = 1 << STDP_TAU_SHIFT;

const _: () = assert!(PAIR_WINDOW == 2048);

/// The pair window after the readout window's opening lies inside the trial.
const _: () = assert!(WINDOW.from + PAIR_WINDOW < TRIAL_TICKS);

/// The ladder of gains, in the order tried and no other: 1.75 (the present), 1.5, 1.25
/// and 1.0.
pub(crate) const LADDER: [u32; 4] = [0x0001_C000, 0x0001_8000, 0x0001_4000, 0x0001_0000];

const _: () = assert!(LADDER[0] == GAIN_1024 && LADDER[0] == GAINS[0]);

const _: () = assert!(LADDER[0] > LADDER[1] && LADDER[1] > LADDER[2] && LADDER[2] > LADDER[3]);

/// The sign calibration's pass mark: trials of sixty-four in which the presented stimulus's
/// summed eligibility onto both readouts, read from the record after the trial, is
/// positive; as strict as the sight's.
pub(crate) const SIGN_MIN: u32 = 56;

const _: () = assert!(SIGN_MIN == SEEN_MIN);

/// The excitatory depression's reference magnitude, $2^{13}$ (ADR-0055), as the shift that
/// divides by it in the oracle below; the gate holds the oracle to the amounts the rule
/// documents at the reference, at the rail and at the magnitude below which it rounds to
/// nothing, and every reading holds the oracle to the record.
pub(crate) const DEPRESSION_REFERENCE_SHIFT: u32 = 13;

const _: () = assert!(1 << DEPRESSION_REFERENCE_SHIFT == 0x2000);

/// The trials the gate runs of the composition at the present gain, held to the first rows
/// of the pinned table: eight, $2^{17}$ ticks at 1 024 units.
pub(crate) const GATE_TRIALS: usize = 8;

// ------------------------------------------------------------------------- the oracle

/// The pair rule's window at a distance, $A (1 - 2^{-11})^{\Delta t}$ rounded to nearest
/// (`cortex-core`'s `window`, written a second time as the oracle).
pub(crate) fn pair_window(amplitude_q1_15: i16, delta_ticks: u32) -> i16 {
    let factor = i64::from(stp_decay_factor_q16(delta_ticks, STDP_TAU_SHIFT));
    (i64::from(amplitude_q1_15)
        .saturating_mul(factor)
        .saturating_add(0x8000)
        >> 16) as i16
}

/// An excitatory depression at a magnitude (ADR-0055): `amount × magnitude / 2^13`, rounded
/// to nearest (`cortex-core`'s `depression_at`).
pub(crate) fn depression_at(amount_q1_15: i16, magnitude: i32) -> i16 {
    (i64::from(amount_q1_15)
        .saturating_mul(i64::from(magnitude))
        .saturating_add(1 << (DEPRESSION_REFERENCE_SHIFT - 1))
        >> DEPRESSION_REFERENCE_SHIFT) as i16
}

/// A trace decayed by a Q16.16 factor below 1.0, rounded to nearest and by at least one LSB
/// toward zero for a trace that is not zero (`cortex-core`'s `decayed`).
pub(crate) fn decayed(trace: i16, factor_q16: i64) -> i16 {
    if trace == 0 {
        return 0;
    }
    let magnitude = i64::from(trace).abs();
    let kept = (magnitude.saturating_mul(factor_q16).saturating_add(0x8000) >> 16)
        .min(magnitude.saturating_sub(1));
    kept.saturating_mul(i64::from(trace).signum()) as i16
}

/// Whether a readout unit's spike at `q` is the volley's: within one pair window from the
/// arrival of a volley message over a synapse of delay `delay`, `volleys` the presynaptic
/// unit's volley spikes in tick order. Ticks compare as numbers: the run is far below the
/// stamp's width.
pub(crate) fn volleyed(volleys: &[u32], delay: u32, q: u32) -> bool {
    let at = volleys.partition_point(|&v| v.saturating_add(delay) <= q);
    at.checked_sub(1)
        .and_then(|k| volleys.get(k))
        .is_some_and(|&v| q < v.saturating_add(delay).saturating_add(PAIR_WINDOW))
}

/// One synapse from a stimulus unit onto a readout unit as the oracle replays it: where it
/// is in the arena, its ends and the sets they are in, its delay, its magnitude (frozen),
/// and the block's presynaptic stamp and the slot's trace as the rule would hold them.
pub(crate) struct Replayed {
    pub(crate) block_idx: usize,
    pub(crate) slot: usize,
    pub(crate) source: u32,
    pub(crate) target: u32,
    pub(crate) stimulus: usize,
    pub(crate) readout: usize,
    pub(crate) delay: u32,
    pub(crate) magnitude: i32,
    pub(crate) stamp: u32,
    pub(crate) trace: i16,
}

/// One trial's reading of the trace's composition: the stimulus presented; (a) the record's
/// summed eligibility after the trial over the synapses from each stimulus set onto each
/// readout set, `[stimulus][readout]`; (b) the terms that entered those traces since the
/// previous reading (for the first trial, since the run's first tick), by stimulus and
/// readout, as the volley's potentiation, the volley's depression, the background's
/// potentiation and the background's depression, signed as they entered; (c) the spikes in
/// the pair window before the readout window's opening and in the pair window after it, of
/// readout 0, of readout 1 and of the presented stimulus set (whose before is the volley
/// and whose after is what its units fire beyond it), `[set][before, after]`.
pub(crate) type Composed = (u8, [[i64; 2]; 2], [[[i64; 4]; 2]; 2], [[u32; 2]; 3]);

/// The pinned row of a trial: the stimulus presented, the presented stimulus's summed
/// eligibility onto both readouts after the trial (the sign measure reads its sign), and its
/// four classes of terms since the previous reading over both readouts.
pub(crate) type Row = (u8, i64, [i64; 4]);

/// A block of the composition: the sums after the block's last trial; the block's terms by
/// stimulus, readout and class; the census summed over the block; the trials in which the
/// presented stimulus's sum onto both readouts was positive (the sign measure); and the
/// FNV-1a hash of every trial's whole reading.
pub(crate) type Composition = ([[i64; 2]; 2], [[[i64; 4]; 2]; 2], [[u64; 2]; 3], u32, u64);

/// The reader of the composition: the executor's train collected per unit since the run's
/// first tick, the volley spikes of the stimulus units, the synapses from the stimulus sets
/// onto the readout sets with their replayed traces, and the readings.
pub(crate) struct Composer {
    pub(crate) sets: [Set; 4],
    pub(crate) readout: Readout,
    pub(crate) stimuli: Readout,
    pub(crate) spikes: Vec<Vec<u32>>,
    pub(crate) volleys: Vec<Vec<u32>>,
    /// The presented set's volley spikes by their tick after the trial's first, over the run
    /// (brief 034): index `k` counts the units whose volley spike fell on trial tick `k`, for
    /// every `k` before the readout window opens.
    pub(crate) volley_ticks: Vec<u64>,
    pub(crate) synapses: Vec<Replayed>,
    pub(crate) cursor: u32,
    pub(crate) out: Vec<Composed>,
    /// The delivery as the oracle replays it (brief 036; the task's own since brief 037): the
    /// pair the last trial's delivery addressed and the signal its reward left, so that the
    /// oracle consolidates where and by as much as the engine does; none in a frozen run,
    /// where nothing consolidates and the weights are held to the record as they were.
    pub(crate) taught: Option<Taught>,
    /// What each trial consolidated into the weights, `[stimulus][readout]`, by the oracle
    /// (brief 036); zero in a frozen run.
    pub(crate) transferred: Vec<[[i64; 2]; 2]>,
    /// The reward the task itself delivered at the trial's end, before the reading
    /// (brief 037): the record's signal at the reading is the course's end plus it. Zero
    /// where the harness rewards after the reading (brief 036) or nothing rewards.
    pub(crate) rewarded: i32,
    /// The modulation baseline the engine consolidates under where a synapse is not
    /// addressed, and beneath the signal where it is (brief 038): zero in every run before
    /// it, so that the oracle consolidates nothing but the addressed pair under the signal,
    /// and 0.5 under H-15, where every synapse consolidates half its trace at each
    /// presynaptic spike.
    pub(crate) baseline: i32,
}

impl Composer {
    pub(crate) fn new(units: u32) -> Self {
        let sets = geometry(units, rotation(units));
        let [a, b, r0, r1] = sets;
        Self {
            sets,
            readout: Readout::new([r0, r1]),
            stimuli: Readout::new([a, b]),
            spikes: vec![Vec::new(); units as usize],
            volleys: vec![Vec::new(); units as usize],
            volley_ticks: vec![0; WINDOW.from as usize],
            synapses: Vec::new(),
            cursor: 0,
            out: Vec::new(),
            taught: None,
            transferred: Vec::new(),
            rewarded: 0,
            baseline: 0,
        }
    }

    /// The synapses from a stimulus unit onto a readout unit, from the arena, in the walk's
    /// order, each with its block's presynaptic stamp and its slot's trace as the record holds
    /// them, and every unit's last spike on record seeded as the first entry of its list
    /// (brief 035): on a fresh network the stamps and the traces are none and zero and no unit
    /// has spiked, so the oracle starts as it started; on a network a lead-in left, it starts
    /// where the engine is. A stimulus unit is excitatory, as the geometry holds.
    pub(crate) fn enumerate(&mut self, exec: &Engine) {
        let [a, b, r0, r1] = self.sets;
        let blocks = exec.blocks();
        for unit in exec.units() {
            let id = unit.id as u32;
            if unit.last_soma_spike_tick != NO_SPIKE_ON_RECORD {
                if let Some(list) = self.spikes.get_mut(id as usize) {
                    list.push(unit.last_soma_spike_tick);
                }
            }
            let stimulus = if a.contains(id) {
                0
            } else if b.contains(id) {
                1
            } else {
                continue;
            };
            assert_eq!(unit.flags & FLAG_INHIBITORY, 0, "stimulus unit {id}");
            for s in unit.fan_out(exec.blocks()) {
                let readout = if r0.contains(s.target) {
                    0
                } else if r1.contains(s.target) {
                    1
                } else {
                    continue;
                };
                let block = blocks
                    .get(s.block_idx as usize)
                    .expect("a block of the arena");
                self.synapses.push(Replayed {
                    block_idx: s.block_idx as usize,
                    slot: usize::from(s.slot),
                    source: id,
                    target: s.target,
                    stimulus,
                    readout,
                    delay: u32::from(s.delay_ticks),
                    magnitude: i32::from(s.weight_q1_15).max(0),
                    stamp: block.last_spike_tick,
                    trace: block.eligibility_q1_15[usize::from(s.slot)],
                });
            }
        }
        assert!(!self.synapses.is_empty());
    }

    /// The synapses from each stimulus set onto each readout set, `[stimulus][readout]`.
    pub(crate) fn counts(&self) -> [[u32; 2]; 2] {
        let mut counts = [[0u32; 2]; 2];
        for syn in &self.synapses {
            let into = &mut counts[syn.stimulus][syn.readout];
            *into = into.saturating_add(1);
        }
        counts
    }

    /// After a trial: the train since the last reading collected, the presented set's
    /// volleys noted, the oracle advanced over the stimulus units' spikes since the last
    /// reading with every term classed, the record read and held to the oracle, the census
    /// counted.
    pub(crate) fn observe(
        &mut self,
        exec: &mut Engine,
        trial: usize,
        start: u32,
        outcome: &Outcome,
    ) {
        if self.synapses.is_empty() {
            self.enumerate(exec);
        }
        let end = start.wrapping_add(TRIAL_TICKS);
        let cursor = self.cursor;
        // The train since the last reading. The ring holds a trial's most spikes, so what
        // it let go is older than the reading before this one; asserted.
        let overwritten = exec.train_overwritten();
        let train = exec.train();
        assert!(
            overwritten == 0 || train.first().is_some_and(|&(t, _)| t <= cursor),
            "trial {trial}: the train held every spike since the last reading"
        );
        for &(tick, unit) in train {
            if tick < cursor {
                continue;
            }
            if tick >= end {
                break;
            }
            if let Some(list) = self.spikes.get_mut(unit as usize) {
                list.push(tick);
            }
        }
        // The presented set's volleys: each unit's first spike before the readout window
        // opens; a unit the background fired within its refractory window before the
        // injection has none this trial.
        let presented = self.sets[usize::from(outcome.stimulus)];
        for s in presented.units() {
            let Some(list) = self.spikes.get(s as usize) else {
                continue;
            };
            let at = list.partition_point(|&t| t < start);
            if let Some(&v) = list.get(at) {
                if v < start.wrapping_add(WINDOW.from) {
                    if let Some(volleys) = self.volleys.get_mut(s as usize) {
                        volleys.push(v);
                    }
                    if let Some(count) = self.volley_ticks.get_mut(v.wrapping_sub(start) as usize) {
                        *count = count.saturating_add(1);
                    }
                }
            }
        }
        // The oracle over the stimulus units' spikes since the last reading, synapse by
        // synapse: the block's decay since its stamp, then the rule's two terms for the
        // target's last spike, each classed by that spike, then the stamp; then, under the
        // taught delivery (brief 036), the consolidation at that spike under the signal the
        // executor published at its tick where the synapse is addressed — its source in the
        // stimulus the last trial presented and its target in that stimulus's assigned
        // readout — and under the baseline alone otherwise (brief 038): zero in every run
        // before it, 0.5 under H-15.
        let mut terms = [[[0i64; 4]; 2]; 2];
        let mut transferred = [[0i64; 2]; 2];
        let course = self.taught.as_ref().map(|t| signal_course(t.signal));
        let addressed = self.taught.as_ref().and_then(|t| t.addressed);
        let baseline = self.baseline;
        let Self {
            synapses,
            spikes,
            volleys,
            ..
        } = self;
        for syn in synapses.iter_mut() {
            let (Some(pre), Some(post), Some(volleys)) = (
                spikes.get(syn.source as usize),
                spikes.get(syn.target as usize),
                volleys.get(syn.source as usize),
            ) else {
                panic!("a synapse's end is outside the arena");
            };
            let from = pre.partition_point(|&t| t < cursor);
            for &t in pre.iter().skip(from) {
                let elapsed = t.wrapping_sub(syn.stamp);
                if elapsed != 0 {
                    let factor = i64::from(stp_decay_factor_q16(elapsed, ELIGIBILITY_TAU_SHIFT));
                    syn.trace = decayed(syn.trace, factor);
                }
                let at = post.partition_point(|&q| q <= t);
                if let Some(&q) = at.checked_sub(1).and_then(|k| post.get(k)) {
                    let since_post = t.wrapping_sub(q) as i32;
                    let class = if volleyed(volleys, syn.delay, q) {
                        0
                    } else {
                        2
                    };
                    let into = &mut terms[syn.stimulus][syn.readout];
                    if syn.stamp != NO_SPIKE_ON_RECORD {
                        let post_after_prev = q.wrapping_sub(syn.stamp) as i32;
                        if post_after_prev > 0 && since_post >= 0 {
                            let pot = pair_window(STDP_A_PLUS_Q1_15, post_after_prev as u32);
                            syn.trace = syn.trace.saturating_add(pot);
                            into[class] = into[class].saturating_add(i64::from(pot));
                        }
                    }
                    if since_post > 0 {
                        let dep = depression_at(
                            pair_window(STDP_A_MINUS_Q1_15, since_post as u32),
                            syn.magnitude,
                        );
                        syn.trace = syn.trace.saturating_sub(dep);
                        let k = class.wrapping_add(1);
                        into[k] = into[k].saturating_sub(i64::from(dep));
                    }
                }
                syn.stamp = t;
                if let Some(course) = &course {
                    // An addressed synapse's spike is inside the trial: the addressed pair is
                    // written at a trial's end, so the lead-in's spikes before the first
                    // trial, the only ones before `start` the oracle replays, are never
                    // addressed.
                    let signal = if addressed == Some((syn.stimulus, syn.readout)) {
                        course
                            .get(t.wrapping_sub(start) as usize)
                            .copied()
                            .expect("an addressed synapse's spike is inside the trial")
                    } else {
                        0
                    };
                    // The engine's rule (`Modulations::of`, `NeuromodulatorState::modulation`):
                    // the baseline plus the signal where the synapse is addressed, the
                    // baseline alone elsewhere, saturating; `consolidated` clamps it to
                    // [0, 1] as the engine does.
                    let modulation = baseline.saturating_add(signal);
                    let (trace, magnitude, absorbed) =
                        consolidated(syn.trace, syn.magnitude, modulation);
                    syn.trace = trace;
                    syn.magnitude = magnitude;
                    let into = &mut transferred[syn.stimulus][syn.readout];
                    *into = into.saturating_add(i64::from(absorbed));
                }
            }
        }
        // The record: (a), each slot's trace held to the oracle and each weight to the
        // oracle's magnitude — the prior's in a frozen run, the consolidated one under the
        // taught delivery.
        let blocks = exec.blocks();
        let mut sums = [[0i64; 2]; 2];
        for syn in synapses.iter() {
            let block = blocks.get(syn.block_idx).expect("a block of the arena");
            let e = block.eligibility_q1_15[syn.slot];
            assert_eq!(
                e, syn.trace,
                "trial {trial}: the record's trace of {}→{} is the oracle's",
                syn.source, syn.target
            );
            assert_eq!(
                i32::from(block.weights_q1_15[syn.slot]),
                syn.magnitude,
                "trial {trial}: the record's weight of {}→{} is the oracle's",
                syn.source,
                syn.target
            );
            let into = &mut sums[syn.stimulus][syn.readout];
            *into = into.saturating_add(i64::from(e));
        }
        // The signal at the trial's end, held to its course (brief 036), plus the reward the
        // task itself delivered before this reading (brief 037; zero otherwise); at rest in
        // a frozen run.
        let end_signal = course.as_ref().map_or(0, |c| signal_end(c));
        assert_eq!(
            exec.modulator().dopamine_rpe,
            end_signal.saturating_add(self.rewarded),
            "trial {trial}: the record's signal at the trial's end is the course's"
        );
        self.transferred.push(transferred);
        // The census around the readout window's opening: the readouts and the presented
        // set, counted as a readout counts.
        let arrival = start.wrapping_add(WINDOW.from);
        let opening = arrival.wrapping_sub(PAIR_WINDOW);
        let before = self
            .readout
            .count_window(exec.train(), opening, PAIR_WINDOW);
        let after = self
            .readout
            .count_window(exec.train(), arrival, PAIR_WINDOW);
        let s = usize::from(outcome.stimulus);
        let own_before = self
            .stimuli
            .count_window(exec.train(), opening, PAIR_WINDOW)[s];
        let own_after = self
            .stimuli
            .count_window(exec.train(), arrival, PAIR_WINDOW)[s];
        self.out.push((
            outcome.stimulus,
            sums,
            terms,
            [
                [before[0], after[0]],
                [before[1], after[1]],
                [own_before, own_after],
            ],
        ));
        self.cursor = end;
    }
}

// ----------------------------------------------------------------------- the readings

/// A trial's pinned row.
pub(crate) fn trial_row(t: &Composed) -> Row {
    let s = usize::from(t.0);
    let sum = t.1[s][0].saturating_add(t.1[s][1]);
    let mut terms = [0i64; 4];
    for readout in &t.2[s] {
        for (k, term) in readout.iter().enumerate() {
            terms[k] = terms[k].saturating_add(*term);
        }
    }
    (t.0, sum, terms)
}

/// The sign measure of a trial: the presented stimulus's summed eligibility onto both
/// readouts, read from the record after the trial, is positive.
pub(crate) fn positive(t: &Composed) -> bool {
    trial_row(t).1 > 0
}

/// The FNV-1a hash of every trial's whole reading, each number as its `i32` words.
pub(crate) fn hash_of(trials: &[Composed]) -> u64 {
    let mut words: Vec<i32> = Vec::new();
    let mut wide = |x: i64| {
        words.push(x as i32);
        words.push((x >> 32) as i32);
    };
    for t in trials {
        wide(i64::from(t.0));
        for readouts in &t.1 {
            for &sum in readouts {
                wide(sum);
            }
        }
        for readouts in &t.2 {
            for classes in readouts {
                for &term in classes {
                    wide(term);
                }
            }
        }
        for readout in &t.3 {
            for &count in readout {
                wide(i64::from(count));
            }
        }
    }
    fnv1a_64(&words)
}

/// A block's composition from its trials' readings.
pub(crate) fn composition(trials: &[Composed]) -> Composition {
    let sums = trials.last().map_or([[0; 2]; 2], |t| t.1);
    let mut terms = [[[0i64; 4]; 2]; 2];
    let mut census = [[0u64; 2]; 3];
    let mut signs = 0u32;
    for t in trials {
        for (s, readouts) in t.2.iter().enumerate() {
            for (r, classes) in readouts.iter().enumerate() {
                for (k, term) in classes.iter().enumerate() {
                    terms[s][r][k] = terms[s][r][k].saturating_add(*term);
                }
            }
        }
        for (r, counts) in t.3.iter().enumerate() {
            for (k, count) in counts.iter().enumerate() {
                census[r][k] = census[r][k].saturating_add(u64::from(*count));
            }
        }
        signs = signs.saturating_add(u32::from(positive(t)));
    }
    (sums, terms, census, signs, hash_of(trials))
}

/// A rung passes when both measures pass: the sight (`calibrated`, ADR-0065's) and the
/// sign, at least `SIGN_MIN` trials of the block positive.
pub(crate) fn passes(block: &Block, composition: &Composition) -> bool {
    calibrated(block) && composition.3 >= SIGN_MIN
}

/// The gain the ladder picks: the first rung, in the ladder's order, that passes both
/// measures; none when no rung does.
pub(crate) fn ladder_pick(rungs: &[(Block, u64, Composition)]) -> Option<u32> {
    LADDER
        .iter()
        .zip(rungs.iter())
        .find(|(_, (block, _, composition))| passes(block, composition))
        .map(|(&gain, _)| gain)
}

/// One frozen run of `trials` trials at `gain`, read through the composer: the calibration's
/// run (the modulation baseline at zero, no reward, two workers), so that at the present
/// gain over a block it is `CALIBRATION_1024[0]`'s run bit for bit, with the composition
/// read after every trial. The sums by polarity are asserted unchanged after a whole block,
/// as the calibration asserts them.
pub(crate) fn compose(units: u32, gain: u32, trials: usize) -> (Vec<Block>, u64, Vec<Composed>) {
    let (blocks, trace, trials, _) = compose_counting(units, gain, trials);
    (blocks, trace, trials)
}

/// `compose`, with the count of synapses from each stimulus set onto each readout set.
pub(crate) fn compose_counting(
    units: u32,
    gain: u32,
    trials: usize,
) -> (Vec<Block>, u64, Vec<Composed>, [[u32; 2]; 2]) {
    let (blocks, trace, trials, counts, _, _) =
        compose_shaped(SHAPE_F46, None, units, gain, trials);
    (blocks, trace, trials, counts)
}

/// A trial's readout counts as the task read them (brief 033): the stimulus presented and
/// the spikes of each readout set in the readout window.
pub(crate) type Counted = (u8, [u32; 2]);

/// A frozen run read under a shape: the sight's blocks and trace, the composed trials, the
/// synapses from each stimulus set onto each readout set, every trial's counts, and the
/// presented set's volley spikes by their tick after the trial's first (brief 034).
pub(crate) type Shaped = (
    Vec<Block>,
    u64,
    Vec<Composed>,
    [[u32; 2]; 2],
    Vec<Counted>,
    Vec<u64>,
);

/// `compose_counting` with the stimulus of `shape` (brief 033) and its cancel (brief 034),
/// and every trial's readout counts beside the composition; under `SHAPE_F46` with no cancel
/// it is `compose_counting`.
pub(crate) fn compose_shaped(
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    gain: u32,
    trials: usize,
) -> Shaped {
    let p = prior(units);
    let mut exec = at_gain(&p, config(units, 2, 0), gain);
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain);
    compose_on(&mut exec, shape, cancel, units, trials)
}

/// `compose_shaped` on an executor the caller made (brief 035): the modulation baseline
/// asserted zero, the sums by polarity read before the run and asserted unchanged after every
/// block, the composer's oracle seeded from the record before the first trial (a settled
/// network carries a stamp and a trace on every block and a last spike on every unit; a fresh
/// one none, so `compose_shaped` reads as it read), and the calibration's task run by
/// `run_on`.
pub(crate) fn compose_on(
    exec: &mut Engine,
    shape: Shape,
    cancel: Option<Cancel>,
    units: u32,
    trials: usize,
) -> Shaped {
    assert_eq!(exec.modulation_baseline_q16(), 0, "the weights are frozen");
    let sums = weights_by_polarity(exec);
    let mut composer = Composer::new(units);
    composer.enumerate(exec);
    // The oracle replays the train from the executor's clock: what the record already
    // holds — the seeded stamps, traces and last spikes — is where it starts, not what it
    // replays (a seeded last spike of a stimulus unit replayed as a presynaptic spike would
    // enter the record's last pairing twice). On a fresh engine the clock is at zero.
    composer.cursor = exec.ticks() as u32;
    let mut counted: Vec<Counted> = Vec::with_capacity(trials);
    let task = task(
        shape,
        cancel,
        units,
        Feedback::Withheld,
        false,
        Delivery::Global,
    );
    let (blocks, trace) = run_on(
        exec,
        task,
        units,
        trials,
        &mut |exec, trial, start, outcome| {
            composer.observe(exec, trial, start, outcome);
            assert_eq!(
                outcome.selection,
                selected(outcome.counts),
                "trial {trial}: the selection is the sign of the count difference"
            );
            counted.push((outcome.stimulus, outcome.counts));
        },
    );
    for block in &blocks {
        assert_eq!(
            (block.7, block.8),
            sums,
            "no weight moves at a modulation of zero"
        );
    }
    assert_eq!(composer.out.len(), trials);
    let counts = composer.counts();
    (
        blocks,
        trace,
        composer.out,
        counts,
        counted,
        composer.volley_ticks,
    )
}

/// Dumps a composed run: the sight's blocks and trace, the rows and the composition.
pub(crate) fn dump_composition(name: &str, blocks: &[Block], trace: u64, trials: &[Composed]) {
    let rows: Vec<Row> = trials.iter().map(trial_row).collect();
    eprintln!("DUMP {name} sight {blocks:?} trace {trace:#018x}");
    eprintln!("DUMP {name} rows {rows:?}");
    eprintln!("DUMP {name} composition {:?}", composition(trials));
}

/// Holds a composition's rows and, where a whole block was run, its block to their pinned
/// tables.
pub(crate) fn pinned_composition(
    name: &str,
    trials: &[Composed],
    rows: &[Row],
    block: Option<&Composition>,
) {
    let read: Vec<Row> = trials.iter().map(trial_row).collect();
    assert_eq!(read, rows, "{name}: the rows");
    if let Some(block) = block {
        assert_eq!(&composition(trials), block, "{name}: the block");
    }
}

// ------------------------------------------------------------ the measurement (ADR-0072)

/// The composition and the ladder at 1 024 units, rung by rung in the ladder's order, each
/// a frozen run of sixty-four trials: the sight's block and trace (at the present gain,
/// `CALIBRATION_1024[0]`'s), and the composition. Pinned from one run each.
pub(crate) const LADDER_1024: [(Block, u64, Composition); 4] = [
    (
        (
            27,
            34,
            [[396, 378], [379, 363]],
            [1727, 1526],
            [6171, 5504],
            [255, 241],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            8,
        ),
        0xa84553d901278f4f,
        (
            [[-51652, -55823], [-42894, -46658]],
            [
                [
                    [453094, -273496, 372055, -1330229],
                    [461454, -231820, 401769, -1314171],
                ],
                [
                    [433249, -239333, 364647, -1196198],
                    [415580, -234738, 381002, -1226100],
                ],
            ],
            [[1038, 2265], [1035, 2270], [3358, 6737]],
            1,
            11256068980148820643,
        ),
    ),
    (
        (
            25,
            34,
            [[63, 56], [48, 52]],
            [1734, 1529],
            [4463, 3966],
            [15, 22],
            49,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            17,
        ),
        0xd1a3a314d585fdcb,
        (
            [[7354, 5844], [991, 1416]],
            [
                [
                    [72257, -21448, 18293, -57382],
                    [82430, -17452, 21756, -62247],
                ],
                [
                    [65958, -16157, 21609, -47132],
                    [69522, -25426, 26529, -66552],
                ],
            ],
            [[57, 218], [73, 223], [3273, 5045]],
            45,
            2876531214492687737,
        ),
    ),
    (
        (
            4,
            34,
            [[2, 4], [2, 3]],
            [1734, 1530],
            [3487, 3078],
            [1, 1],
            10,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            54,
        ),
        0xf4bd9f85758e39e5,
        (
            [[88, 1709], [-173, 0]],
            [
                [[1770, 0, 352, -240], [5303, 0, 1574, -3447]],
                [[1854, 0, 387, -1426], [3449, 0, 1468, -1509]],
            ],
            [[1, 4], [4, 10], [3264, 3300]],
            32,
            13527546449650777006,
        ),
    ),
    (
        (
            0,
            34,
            [[0, 0], [0, 0]],
            [1734, 1530],
            [2527, 2251],
            [0, 0],
            0,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            64,
        ),
        0x420a6b796a3a6725,
        (
            [[0, 0], [0, 0]],
            [[[0, 0, 0, 0], [0, 0, 0, 0]], [[0, 0, 0, 0], [0, 0, 0, 0]]],
            [[0, 0], [0, 0], [3264, 1514]],
            0,
            1431228405596711687,
        ),
    ),
];

/// The rows of every rung's sixty-four trials, in the ladder's order.
pub(crate) const LADDER_ROWS_1024: [[Row; BLOCK]; 4] = [
    [
        (0, 4351, [39500, -27437, 11767, -19732]),
        (0, -15516, [38260, -7400, 19473, -71119]),
        (1, -88961, [33439, -41843, 27108, -92702]),
        (0, -52656, [23321, -14896, 23194, -74132]),
        (0, -54695, [25052, -3478, 20577, -56279]),
        (0, -85952, [19395, -7961, 13421, -69410]),
        (0, -99666, [14580, -5212, 13206, -55800]),
        (0, -96614, [18256, -6071, 14928, -48693]),
        (0, -110919, [17971, -11849, 18427, -58165]),
        (0, -140667, [15004, -9144, 24106, -87411]),
        (1, -44149, [49532, -38665, 27111, -58618]),
        (1, -32438, [34244, -9136, 22037, -47435]),
        (0, -130937, [30841, -28673, 29700, -86403]),
        (0, -118955, [40124, -13949, 25172, -67092]),
        (1, -71524, [24891, -22383, 23803, -74969]),
        (0, -108754, [23135, -21497, 20767, -58341]),
        (1, -72153, [22889, -14786, 20027, -59292]),
        (0, -124145, [20755, -7924, 21387, -91940]),
        (0, -145336, [29612, -6560, 16774, -87912]),
        (0, -175808, [17222, -7982, 14883, -86923]),
        (1, -68593, [22659, -24894, 27624, -59189]),
        (0, -153711, [22209, -13532, 16795, -72094]),
        (1, -75206, [41016, -20391, 21560, -83711]),
        (1, -85629, [25851, -10566, 21384, -63575]),
        (1, -130549, [19720, -15426, 20149, -89478]),
        (0, -104593, [21604, -19623, 18608, -61928]),
        (1, -133222, [17934, -16143, 18076, -76288]),
        (1, -140469, [26468, -7051, 13245, -69780]),
        (1, -157654, [8799, -6243, 15362, -66741]),
        (0, -99453, [33339, -22427, 20578, -85729]),
        (0, -109293, [24563, -7415, 20110, -70334]),
        (0, -128654, [27249, -20342, 16562, -67343]),
        (0, -136254, [21014, -13555, 12936, -59530]),
        (0, -151754, [16014, -5581, 13485, -73383]),
        (1, -73945, [36302, -35583, 18756, -54837]),
        (1, -85516, [39430, -10148, 19286, -79086]),
        (1, -98720, [18841, -3718, 13412, -60167]),
        (0, -113908, [30167, -34057, 23889, -67695]),
        (1, -107873, [25423, -17176, 29961, -84463]),
        (1, -120074, [24040, -6977, 20537, -73873]),
        (0, -88213, [23899, -21466, 20667, -54839]),
        (1, -113369, [18896, -12367, 15187, -54332]),
        (0, -79959, [28922, -15956, 26612, -68976]),
        (1, -108306, [26201, -20299, 20537, -65219]),
        (0, -80853, [21452, -12803, 18927, -59854]),
        (1, -99837, [30574, -21208, 16234, -63947]),
        (0, -104501, [20796, -12145, 20667, -70120]),
        (0, -109353, [27229, -16497, 19386, -56995]),
        (0, -113572, [18445, -7885, 14379, -53999]),
        (1, -86353, [36676, -29568, 30257, -83035]),
        (1, -105370, [23500, -11405, 21915, -72470]),
        (0, -112699, [26420, -24359, 24130, -72766]),
        (1, -110437, [20577, -9843, 25166, -84533]),
        (1, -115894, [17875, -5522, 14712, -55546]),
        (0, -104534, [29291, -27705, 21471, -77413]),
        (1, -133071, [14379, -11138, 13785, -77839]),
        (1, -152763, [30120, -10947, 16608, -85417]),
        (1, -167995, [17365, -4765, 14076, -78093]),
        (0, -89200, [30425, -22783, 22368, -75092]),
        (1, -139386, [17483, -11394, 18783, -65170]),
        (0, -101184, [30678, -17096, 21999, -81083]),
        (0, -117893, [19497, -10056, 16616, -67623]),
        (1, -91000, [25961, -14107, 21364, -58421]),
        (1, -89552, [21619, -10378, 16549, -47788]),
    ],
    [
        (0, 1042, [2581, -1557, 295, -277]),
        (0, 7859, [8098, -482, 2177, -2930]),
        (1, -3563, [1869, -1685, 937, -4718]),
        (0, -4571, [2566, -743, 443, -11660]),
        (0, -1523, [2151, -3, 742, -736]),
        (0, -3000, [0, 0, 107, -1874]),
        (0, -1594, [3872, -1163, 1584, -3467]),
        (0, -1550, [0, 0, 303, -554]),
        (0, -740, [3979, -1029, 702, -3120]),
        (0, -3095, [2037, -2, 1967, -6425]),
        (1, 5803, [12132, -4415, 2076, -3761]),
        (1, 9089, [4614, 0, 1296, -1410]),
        (0, -4841, [3971, -4975, 1527, -3894]),
        (0, 1471, [5782, 0, 1467, -1984]),
        (1, 1277, [5187, -7036, 1480, -2605]),
        (0, 110, [1820, -996, 1142, -2798]),
        (1, 8505, [7395, -1741, 4123, -2007]),
        (0, -7592, [1097, 0, 371, -9188]),
        (0, 1979, [7395, -3, 2322, -1793]),
        (0, 1800, [3632, -386, 1500, -4484]),
        (1, 8636, [4996, -30, 1976, -1193]),
        (0, 3815, [2860, 0, 768, -776]),
        (1, 10504, [9782, -4443, 2952, -3672]),
        (1, 11948, [5172, -781, 2630, -3156]),
        (1, 9364, [1230, -1197, 324, -195]),
        (0, 659, [3298, -1057, 1203, -4071]),
        (1, -2064, [2205, -1836, 2574, -10598]),
        (1, 1790, [2648, 0, 1281, -347]),
        (1, 496, [1773, -1301, 1583, -2845]),
        (0, 6274, [10485, -2527, 1266, -3230]),
        (0, 3599, [9263, -402, 1558, -11482]),
        (0, 1818, [5167, -3782, 1083, -3441]),
        (0, 920, [4521, 0, 501, -5456]),
        (0, 3753, [2909, 0, 1650, -1411]),
        (1, 1715, [2754, -816, 1271, -1535]),
        (1, 3848, [7990, -1430, 2154, -6128]),
        (1, 4964, [3528, 0, 1482, -2906]),
        (0, 2552, [3957, -768, 1098, -2711]),
        (1, -2710, [2428, 0, 1118, -9212]),
        (1, -6988, [1840, -1304, 1955, -7158]),
        (0, 3645, [5460, -1057, 1205, -3545]),
        (1, -6168, [2937, 0, 934, -5727]),
        (0, 7845, [7759, -1941, 930, -1000]),
        (1, -3718, [2185, -1750, 1686, -2094]),
        (0, 2381, [4317, -3933, 1326, -3878]),
        (1, 3299, [8438, -1323, 1890, -3363]),
        (0, 613, [3222, -1839, 1661, -3680]),
        (0, 3985, [6792, -1291, 1314, -3183]),
        (0, 3121, [3786, -552, 877, -4009]),
        (1, 2632, [9649, -3956, 2007, -6111]),
        (1, 5313, [3642, -739, 1736, -1334]),
        (0, -2165, [4089, -3318, 818, -5173]),
        (1, 4233, [1638, -14, 1182, -1668]),
        (1, 5435, [3515, -75, 751, -2126]),
        (0, 2381, [4092, 0, 714, -1502]),
        (1, -1400, [1308, 0, 1297, -7221]),
        (1, 1687, [6271, -613, 920, -3794]),
        (1, -893, [2503, -737, 187, -4047]),
        (0, 5603, [3656, 0, 1809, -552]),
        (1, -2616, [2889, -2836, 1350, -3426]),
        (0, 13544, [12577, -2858, 1211, -652]),
        (0, 13104, [6573, -2236, 1929, -3371]),
        (1, 1372, [6757, -1525, 1887, -4582]),
        (1, 2407, [4955, 0, 1034, -4435]),
    ],
    [
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 1122, [812, 0, 314, 0]),
        (0, 872, [0, 0, 0, 0]),
        (0, 678, [0, 0, 0, 0]),
        (0, -897, [0, 0, 0, -1427]),
        (0, -698, [0, 0, 0, 0]),
        (0, -600, [0, 0, 1, -57]),
        (0, -468, [0, 0, 0, 0]),
        (1, -1, [0, 0, 0, -2]),
        (1, 0, [0, 0, 0, 0]),
        (0, -203, [0, 0, 70, -51]),
        (0, 707, [867, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 428, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, -801, [590, 0, 296, -1945]),
        (0, -629, [0, 0, 0, 0]),
        (0, -492, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, -302, [0, 0, 0, 0]),
        (1, 2446, [2284, 0, 571, -402]),
        (1, 1902, [0, 0, 0, 0]),
        (1, 1478, [0, 0, 0, 0]),
        (0, -110, [0, 0, 3, 0]),
        (1, 844, [0, 0, 299, -344]),
        (1, 1817, [897, 0, 309, -40]),
        (1, 1411, [0, 0, 0, 0]),
        (0, -42, [0, 0, 0, 0]),
        (0, -37, [0, 0, 0, 0]),
        (0, -198, [0, 0, 2, -169]),
        (0, -99, [0, 0, 56, -2]),
        (0, 967, [786, 0, 262, 0]),
        (1, 846, [268, 0, 268, 0]),
        (1, 1417, [873, 0, 2, -106]),
        (1, 1090, [0, 0, 0, 0]),
        (0, 354, [0, 0, 0, 0]),
        (1, 655, [0, 0, 0, 0]),
        (1, -302, [0, 0, 0, -794]),
        (0, 148, [0, 0, 0, -22]),
        (1, -176, [0, 0, 21, 0]),
        (0, 90, [0, 0, 0, 0]),
        (1, -114, [0, 0, 0, 0]),
        (0, 55, [0, 0, 0, 0]),
        (1, -79, [0, 0, 0, -3]),
        (0, 2096, [1734, 0, 339, 0]),
        (0, 1624, [0, 0, 0, 0]),
        (0, 1256, [0, 0, 0, 0]),
        (1, -32, [0, 0, 0, 0]),
        (1, 1274, [981, 0, 327, 0]),
        (0, 1741, [1180, 0, 0, -14]),
        (1, 807, [0, 0, 44, 0]),
        (1, 624, [0, 0, 0, 0]),
        (0, 812, [0, 0, 0, 0]),
        (1, 375, [0, 0, 0, 0]),
        (1, -941, [0, 0, 14, -1244]),
        (1, -734, [0, 0, 0, 0]),
        (0, 291, [0, 0, 0, 0]),
        (1, -447, [0, 0, 0, 0]),
        (0, 165, [0, 0, 0, 0]),
        (0, 1797, [1104, 0, 583, 0]),
        (1, -214, [0, 0, 0, 0]),
        (1, -173, [0, 0, 0, 0]),
    ],
    [
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (0, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
        (1, 0, [0, 0, 0, 0]),
    ],
];

/// The gain the ladder picked, as `ladder_pick` reads the pinned tables; none when no rung
/// passed both measures.
pub(crate) const GAIN_PICKED_1024: Option<u32> = None;

/// The synapses from each stimulus set onto each readout set at 1 024 units,
/// `[stimulus][readout]`, as the composer walks them: the census behind the couplings.
pub(crate) const SYNAPSES_1024: [[u32; 2]; 2] = [[775, 806], [798, 809]];

// ------------------------------------------ a stimulus that fires once (brief 033)
//
// Written before the run. F-46 (ADR-0072): the stimulus of ADR-0065 fires each of its
// units about three times within one pair window of the volley, once before the readout
// window opens and 2.06 more inside the pair window after it, and the pair rule enters a
// depression at every presynaptic spike. The executor scales every input of a unit by the
// tick's synaptic gain (ADR-0036; `scaled` in `executor.rs`), the injected stimulus
// included, so at the present gain of 1.75 the two messages of 1.25 arrive as 4.375 of
// basal drive, above the 3.0 at which ADR-0038 read a second spike. This round changes the
// stimulus and nothing else. The candidates below, in this order and no other, are each
// read by the measure below over one frozen run of sixty-four trials at the present gain,
// the calibration's run with the shape the one difference; the measure reads no selection,
// no reward and no outcome. The first candidate that passes is the stimulus; if none passes
// there is no rewarded run and the candidates are recorded with their censuses. Under the
// chosen stimulus the composition is read again by ADR-0072's oracle at no extra cost, and
// both calibrations, the sight (ADR-0065's) and the sign (ADR-0072's), are read from the
// same run at the gain 1.75, which does not move.
//
// What the membrane rule says of one message, computed apart from the tree before any run
// (an oracle over `integrate` in `membrane.rs` on a unit at rest at the base threshold with
// no other input; a Hypothesis about the instrument, whose units carry the drive's standing
// potential besides): a basal drive of $D$ after the gain fires the unit once when $D$ is
// between about 1.13 and 1.53, twice from 1.53 (the second spike about 200 ticks after the
// first, at the end of the refractory window, from what the basal compartment, whose time
// constant is 512 ticks, still holds), three times from 2.27 and four from 3.38; so 4.375
// is four spikes at rest, and F-46's three are that under the instrument's inhibition.
// Candidate (a), one message at the unit's threshold, is 1.75 after the gain: two spikes at
// rest, at 13 and 214 ticks. Candidate (b), one message of 1.25, is 2.19: two spikes at
// rest, at 9 and 210. The gate holds the engine's own unit at rest to these on the
// instrument's network at the gain (`the_candidates_rules_over_their_tables_...`).

/// Candidate (a): one message at the unit's threshold, `THRESHOLD_BASE` (1.0), the value
/// the record's threshold gives and no other; 1.75 of basal drive after the gain.
pub(crate) const CANDIDATE_A: Shape = (1, THRESHOLD_BASE);

/// Candidate (b): one message of 1.25, half of F-46's stimulus; 2.19 after the gain.
pub(crate) const CANDIDATE_B: Shape = (1, STIMULUS_Q16);

/// Candidate (c), the shape this round argues for, written after (a) and (b) were read and
/// before it was measured (ADR-0074): one message of 1.125, the midpoint of (a) and (b),
/// 1.97 after the gain. (a) read the volley short by 2.4 units of 51 and 0.28 spikes per
/// unit after the opening; (b) read the volley whole and 0.70 after; so the after clause's
/// tenth lies below the drive at which the volley clause first holds, if the after-count
/// is monotone in the drive between them, and (c) reads whether it is. The expectation,
/// written first: the volley at about 0.97 per unit and about 0.5 spikes per unit after,
/// so (c) fails the after clause and no one-message shape passes both.
pub(crate) const CANDIDATE_C: Shape = (1, 0x0001_2000);

/// The candidates in the order tried and no other.
pub(crate) const CANDIDATES: [Shape; 3] = [CANDIDATE_A, CANDIDATE_B, CANDIDATE_C];

const _: () = assert!(CANDIDATES[0].0 == 1 && CANDIDATES[1].0 == 1 && CANDIDATES[2].0 == 1);

const _: () = assert!(CANDIDATES[1].1 == SHAPE_F46.1 && SHAPE_F46.0 == 2);

const _: () = assert!(CANDIDATE_C.1 * 2 == CANDIDATE_A.1 + CANDIDATE_B.1);

/// The measure's second clause: the presented set's spikes in the pair window after the
/// readout window's opening, per unit per presentation, at most this many tenths of a
/// spike (a tenth), against the 2.06 F-46 records.
pub(crate) const AFTER_MAX_TENTHS: u64 = 1;

/// The volley clause's tolerance, ADR-0065's: one spike per unit within two spikes of the
/// set, in tenths of a spike per presentation.
pub(crate) const VOLLEY_TOLERANCE_TENTHS: u64 = 20;

/// The criterion's mark over the calibration's sixty-four trials, for the offset of
/// Deliverable D read there: the rewarded clause's 80 of 128 at the same proportion, 40.
pub(crate) const OFFSET_MARK_64: u32 = REWARDED_MIN * BLOCK as u32 / (LAST_BLOCKS * BLOCK) as u32;

const _: () = assert!(OFFSET_MARK_64 == 40);

/// The ticks the probe of a unit at rest runs after one message: one pair window.
pub(crate) const PROBE_TICKS: u32 = PAIR_WINDOW;

/// The selection as the task's readout makes it (ADR-0059; the property in `task.rs`): the
/// sign of the count difference, none at equal counts.
pub(crate) fn selected(counts: [u32; 2]) -> Option<u8> {
    match counts[0].cmp(&counts[1]) {
        core::cmp::Ordering::Greater => Some(0),
        core::cmp::Ordering::Less => Some(1),
        core::cmp::Ordering::Equal => None,
    }
}

/// The volley clause: over the block, the presented set's spikes before the readout window
/// opens are one per unit within `VOLLEY_TOLERANCE_TENTHS` tenths per presentation, for
/// each stimulus (ADR-0065's reading of the calibration, kept as it was read there).
pub(crate) fn volley_once(units: u32, block: &Block) -> bool {
    let [a, _, _, _] = geometry(units, rotation(units));
    let once = a.len().saturating_mul(10);
    let a_trials = u64::from(block.1);
    let b_trials = (BLOCK as u64).saturating_sub(a_trials);
    let per = |spikes: u64, trials: u64| spikes.saturating_mul(10).checked_div(trials);
    [per(block.3[0], a_trials), per(block.3[1], b_trials)]
        .iter()
        .all(|per| {
            per.is_some_and(|per| {
                per <= once && per >= once.saturating_sub(VOLLEY_TOLERANCE_TENTHS)
            })
        })
}

/// The after clause: over the block, the presented set's spikes in the pair window after
/// the readout window's opening are at most `AFTER_MAX_TENTHS` tenths per unit per
/// presentation.
pub(crate) fn after_quiet(units: u32, composition: &Composition) -> bool {
    let [a, _, _, _] = geometry(units, rotation(units));
    let after = composition.2[2][1];
    after.saturating_mul(10)
        <= (BLOCK as u64)
            .saturating_mul(a.len())
            .saturating_mul(AFTER_MAX_TENTHS)
}

/// A candidate passes when it fires once: the volley clause and the after clause both.
pub(crate) fn fires_once(units: u32, block: &Block, composition: &Composition) -> bool {
    volley_once(units, block) && after_quiet(units, composition)
}

/// The stimulus the round picks: the first candidate, in the candidates' order, whose
/// frozen run fires once; none when none does.
pub(crate) fn candidate_pick(runs: &[(Block, u64, Composition)]) -> Option<Shape> {
    CANDIDATES
        .iter()
        .zip(runs.iter())
        .find(|(_, (block, _, composition))| fires_once(1024, block, composition))
        .map(|(&shape, _)| shape)
}

/// One message of `shape` into unit 0 of the instrument's network at 1 024 units, at rest
/// at the gain, with no drive: the ticks at which unit 0 fires within `PROBE_TICKS`, read
/// from the train. The engine's own reading of what the oracle above computed.
pub(crate) fn probe(shape: Shape) -> Vec<u32> {
    let p = prior(1024);
    let mut exec = at_gain(&p, config(1024, 1, 0), GAIN_1024);
    assert!(exec.is_quiescent());
    let inject = exec.injector();
    for _ in 0..shape.0 {
        inject
            .inject(0, spike_message(shape.1, false))
            .expect("the ring has room");
    }
    let start = exec.ticks() as u32;
    for _ in 0..PROBE_TICKS {
        exec.tick();
    }
    exec.train()
        .iter()
        .filter(|&&(_, unit)| unit == 0)
        .map(|&(tick, _)| tick.wrapping_sub(start))
        .collect()
}

// --------------------------------------------- what the criterion requires (deliverable D)

/// The answer readout of a stimulus, as `Task::answer` has it.
pub(crate) fn answer_of(stimulus: u8, mirrored: bool) -> usize {
    usize::from(if mirrored { stimulus ^ 1 } else { stimulus })
}

/// (a) The instrument's bias, per stimulus: the sum over the trials that presented it of
/// the answer readout's count less the other readout's, and those trials, as an integer
/// ratio `(difference, trials)`; a negative difference favours the readout that is not the
/// answer.
pub(crate) fn bias(counted: &[Counted], mirrored: bool) -> [(i64, u32); 2] {
    let mut out = [(0i64, 0u32); 2];
    for &(stimulus, counts) in counted {
        let answer = answer_of(stimulus, mirrored);
        let other = answer ^ 1;
        let difference = i64::from(counts[answer]).saturating_sub(i64::from(counts[other]));
        let into = &mut out[usize::from(stimulus)];
        into.0 = into.0.saturating_add(difference);
        into.1 = into.1.saturating_add(1);
    }
    out
}

/// The correct trials of `counted` when `delta` is added to the answer readout's count on
/// every trial, the selection as `Task` makes it and a tie an error.
pub(crate) fn correct_with(counted: &[Counted], mirrored: bool, delta: u32) -> u32 {
    counted.iter().fold(0u32, |sum, &(stimulus, counts)| {
        let answer = answer_of(stimulus, mirrored);
        let mut shifted = counts;
        shifted[answer] = shifted[answer].saturating_add(delta);
        sum.saturating_add(u32::from(selected(shifted) == Some(answer as u8)))
    })
}

/// (b) The offset the criterion needs: the smallest whole `delta` such that adding it to
/// the answer readout's count on every trial carries the correct trials to `mark`, found by
/// a scan from zero to one past the largest count difference, at which every trial is
/// correct; none when `mark` exceeds the trials, which no scan reaches.
pub(crate) fn offset(counted: &[Counted], mirrored: bool, mark: u32) -> Option<u32> {
    let widest = counted
        .iter()
        .fold(0u32, |w, &(_, counts)| w.max(counts[0].abs_diff(counts[1])));
    (0..=widest.saturating_add(1)).find(|&delta| correct_with(counted, mirrored, delta) >= mark)
}

/// (c) The difference a block holds, per stimulus, from its summed counts: the answer
/// readout's spikes over the block's trials that presented the stimulus less the other
/// readout's, and those trials, as `(difference, trials)`; what the delivery moved is this
/// in a run's last block against its first.
pub(crate) fn block_bias(block: &Block, mirrored: bool) -> [(i64, u32); 2] {
    let a_trials = block.1;
    let b_trials = (BLOCK as u32).saturating_sub(a_trials);
    let of = |stimulus: u8, trials: u32| {
        let answer = answer_of(stimulus, mirrored);
        let s = usize::from(stimulus);
        (
            (block.2[s][answer] as i64).saturating_sub(block.2[s][answer ^ 1] as i64),
            trials,
        )
    };
    [of(0, a_trials), of(1, b_trials)]
}

/// The dump of Deliverable D's readings over a run's per-trial counts: the bias and the
/// offset at `mark`.
pub(crate) fn dump_requires(name: &str, counted: &[Counted], mirrored: bool, mark: u32) {
    eprintln!(
        "DUMP {name} requires bias {:?} offset {:?} at {mark} of {} counted {counted:?}",
        bias(counted, mirrored),
        offset(counted, mirrored, mark),
        counted.len()
    );
}

// ------------------------------------------------------------ the measurement (brief 033)

/// The candidates at 1 024 units, in the candidates' order, each a frozen run of
/// sixty-four trials at the present gain read by the sight, the sign and the measure that
/// picks: the sight's block and trace, and the composition. Pinned from one run each.
pub(crate) const ONCE_1024: [(Block, u64, Composition); 3] = [
    (
        (
            32,
            34,
            [[352, 327], [330, 338]],
            [1651, 1461],
            [3012, 2709],
            [257, 239],
            63,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0x42c488fa8370cd67,
        (
            [[-11202, -17464], [-9421, -6182]],
            [
                [
                    [280098, -80914, 354009, -736327],
                    [265688, -60844, 389347, -713873],
                ],
                [
                    [244715, -57031, 313006, -632692],
                    [254062, -67362, 367226, -660112],
                ],
            ],
            [[1059, 1807], [1049, 1832], [3217, 911]],
            2,
            3075011394120061531,
        ),
    ),
    (
        (
            31,
            34,
            [[390, 374], [374, 378]],
            [1715, 1514],
            [3822, 3392],
            [257, 232],
            63,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            9,
        ),
        0xd39ec55203c68a8b,
        (
            [[-15256, -25370], [-14720, -15719]],
            [
                [
                    [355325, -102799, 350608, -873999],
                    [332868, -88384, 387923, -876730],
                ],
                [
                    [313081, -97315, 336729, -781233],
                    [299550, -96603, 380003, -781616],
                ],
            ],
            [[1049, 2037], [1042, 2052], [3332, 2277]],
            1,
            1732380210409406099,
        ),
    ),
    (
        (
            30,
            34,
            [[365, 350], [353, 350]],
            [1702, 1497],
            [3381, 3026],
            [253, 237],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0xef44f6eee8436967,
        (
            [[-13221, -22593], [-6238, -10911]],
            [
                [
                    [311063, -93437, 357001, -806596],
                    [295797, -75992, 392519, -792305],
                ],
                [
                    [277828, -68645, 321778, -688156],
                    [270162, -83202, 373574, -710051],
                ],
            ],
            [[1045, 1905], [1038, 1936], [3304, 1502]],
            0,
            11588833636741486556,
        ),
    ),
];

/// The rows of every candidate's sixty-four trials, in the candidates' order.
pub(crate) const ONCE_ROWS_1024: [[Row; BLOCK]; 3] = [
    [
        (0, -5859, [13299, -8565, 10063, -20768]),
        (0, -80, [17533, -1124, 20316, -32032]),
        (1, -37841, [12659, -11371, 23933, -42364]),
        (0, -7194, [13770, -1206, 19032, -32056]),
        (0, 5495, [18915, -865, 19854, -27580]),
        (0, -2870, [15274, -3024, 12988, -31779]),
        (0, -8467, [10889, -1780, 11702, -26942]),
        (0, -11109, [12195, -1475, 16840, -33377]),
        (0, -9986, [18469, -7843, 17086, -27104]),
        (0, -20111, [13679, -2925, 26231, -49717]),
        (1, -18567, [11696, -7698, 23306, -26119]),
        (1, -3867, [24621, -5200, 13953, -23043]),
        (0, -26768, [12385, -6371, 24000, -40404]),
        (0, -13380, [26698, -8359, 27509, -38315]),
        (1, -14258, [9657, -4454, 17550, -30166]),
        (0, -13904, [15442, -10663, 13673, -24744]),
        (1, -12039, [10475, -3108, 18660, -29928]),
        (0, -31957, [10135, -2260, 24264, -51216]),
        (0, -41530, [15806, -5346, 12121, -38996]),
        (0, -45919, [13301, -3126, 14978, -37270]),
        (1, -16644, [3978, -1038, 18557, -25170]),
        (0, -46986, [13865, -7049, 15776, -41262]),
        (1, -15411, [15285, -2907, 18179, -40940]),
        (1, -6363, [18775, -2029, 17928, -29074]),
        (1, -23266, [21199, -10212, 19998, -46791]),
        (0, -34435, [4875, -5982, 16903, -26332]),
        (1, -26599, [17408, -6938, 12301, -36943]),
        (1, -24583, [16458, -1785, 12804, -31574]),
        (1, -23486, [11234, -2878, 13851, -26373]),
        (0, -28708, [18012, -8327, 20519, -36370]),
        (0, -25385, [12513, -1297, 16065, -30688]),
        (0, -25396, [15439, -6154, 14513, -30272]),
        (0, -23607, [20560, -3115, 13602, -35392]),
        (0, -28015, [17339, -2355, 13707, -38599]),
        (1, -24247, [4190, -3331, 16384, -30790]),
        (1, -19042, [20990, -2674, 12950, -31869]),
        (1, -5713, [22413, -978, 13212, -24795]),
        (0, -30833, [5722, -2025, 23931, -39541]),
        (1, -5285, [18290, -5486, 30517, -36747]),
        (1, 1873, [21808, -2670, 15869, -30160]),
        (0, -17157, [8605, -4161, 24569, -29835]),
        (1, -13901, [14028, -4255, 15376, -31263]),
        (0, -16165, [20894, -7886, 20443, -35867]),
        (1, -18952, [7962, -2174, 15959, -30174]),
        (0, -10768, [11568, -2968, 24702, -36736]),
        (1, -17621, [16807, -6727, 16148, -32566]),
        (0, -31069, [5071, -933, 12161, -35531]),
        (0, -13281, [23135, -4722, 21150, -26667]),
        (0, -6737, [20073, -5132, 13784, -25006]),
        (1, -13504, [16431, -10506, 28357, -33677]),
        (1, -15278, [15851, -4285, 16392, -32640]),
        (0, -19168, [5222, -2217, 21403, -33989]),
        (1, -9027, [15696, -386, 20498, -35827]),
        (1, -8105, [14958, -3392, 16168, -30489]),
        (0, -21872, [9452, -2685, 21860, -37361]),
        (1, -22163, [10569, -1959, 14265, -34054]),
        (1, -21159, [14012, -1906, 14918, -30457]),
        (1, -24423, [20034, -4903, 16179, -39597]),
        (0, -29715, [7794, -2789, 24615, -42715]),
        (1, -10399, [12497, -2618, 15564, -23628]),
        (0, -26717, [16760, -3760, 19729, -36944]),
        (0, -24133, [18095, -3269, 15365, -33388]),
        (1, -15497, [5055, -2579, 22602, -32091]),
        (1, -15603, [12947, -3945, 13088, -25706]),
    ],
    [
        (0, -1761, [21400, -11325, 8348, -20237]),
        (0, -3205, [19862, -483, 25230, -46990]),
        (1, -48589, [14342, -12429, 31620, -58757]),
        (0, -7949, [17296, -2604, 22164, -42677]),
        (0, 8274, [27187, -2966, 21497, -31863]),
        (0, -5297, [17301, -2806, 13492, -39957]),
        (0, -15065, [12593, -2364, 11182, -32304]),
        (0, -21320, [14792, -4377, 14656, -36080]),
        (0, -27850, [14361, -4670, 18457, -37287]),
        (0, -45317, [16566, -2350, 23670, -62636]),
        (1, -23043, [19357, -10400, 24329, -34788]),
        (1, -14251, [31210, -15476, 15324, -27971]),
        (0, -41153, [17098, -5552, 23176, -48436]),
        (0, -22087, [36761, -9501, 30468, -46620]),
        (1, -24460, [11915, -5428, 19456, -38388]),
        (0, -26712, [17030, -11679, 16127, -33615]),
        (1, -15212, [20328, -6889, 20355, -36154]),
        (0, -43953, [14462, -2195, 21532, -57715]),
        (0, -58370, [21420, -4168, 13475, -55044]),
        (0, -69675, [15785, -3783, 13723, -49432]),
        (1, -22296, [13043, -13322, 23137, -30726]),
        (0, -67218, [15405, -4960, 18442, -52716]),
        (1, -19768, [19568, -4520, 20889, -46929]),
        (1, -14267, [22524, -4946, 17872, -34581]),
        (1, -38852, [21220, -9483, 22298, -60272]),
        (0, -48158, [13031, -8629, 14333, -36635]),
        (1, -42282, [17300, -12050, 18173, -44727]),
        (1, -46657, [18908, -3374, 13730, -42444]),
        (1, -55012, [10243, -1544, 13554, -41051]),
        (0, -47100, [22329, -14914, 18289, -49802]),
        (0, -40766, [20917, -2680, 17986, -40123]),
        (0, -51419, [17556, -8616, 14468, -43760]),
        (0, -46379, [24110, -5898, 11403, -36884]),
        (0, -57402, [17999, -4978, 13208, -48714]),
        (1, -27358, [10412, -6483, 18235, -32285]),
        (1, -24030, [29387, -4533, 15039, -44161]),
        (1, -21067, [18303, -989, 14164, -33851]),
        (0, -51411, [13405, -11527, 21977, -49536]),
        (1, -26993, [17516, -4887, 26931, -45437]),
        (1, -22758, [23997, -3597, 19427, -41895]),
        (0, -30494, [11936, -6470, 21278, -32379]),
        (1, -37763, [18769, -8813, 13628, -38200]),
        (0, -22329, [26016, -11823, 23340, -41438]),
        (1, -40296, [13420, -4477, 19599, -44430]),
        (0, -21352, [16497, -5742, 24742, -43554]),
        (1, -32234, [24946, -13729, 17431, -37791]),
        (0, -41067, [10317, -1022, 12869, -44886]),
        (0, -34055, [21130, -3979, 22414, -40963]),
        (0, -34689, [20667, -6512, 12586, -34442]),
        (1, -28431, [17865, -11750, 30342, -43289]),
        (1, -33006, [20005, -4976, 19857, -45699]),
        (0, -45935, [9591, -5238, 20496, -45990]),
        (1, -21329, [18947, -1894, 19991, -40644]),
        (1, -24428, [16899, -5206, 12851, -33406]),
        (0, -44646, [12858, -4364, 20125, -48230]),
        (1, -38084, [12325, -2747, 12414, -42495]),
        (1, -34346, [21793, -1950, 17114, -42080]),
        (1, -41986, [20659, -4286, 19220, -51439]),
        (0, -42278, [11249, -5160, 23658, -48872]),
        (1, -29033, [16360, -4062, 16411, -34266]),
        (0, -42542, [22343, -4700, 20311, -49086]),
        (0, -40677, [17512, -3148, 15877, -39500]),
        (1, -30158, [9220, -4085, 19686, -36523]),
        (1, -30439, [15417, -5592, 13296, -29833]),
    ],
    [
        (0, -6113, [15928, -9026, 8047, -21194]),
        (0, -11945, [20835, -3424, 19311, -44218]),
        (1, -42295, [12599, -11634, 25107, -48030]),
        (0, -16307, [15284, -2536, 22021, -36884]),
        (0, -3619, [20318, -874, 18226, -30130]),
        (0, -13950, [15829, -3301, 13833, -37274]),
        (0, -19089, [11025, -1751, 11727, -29174]),
        (0, -19067, [13191, -1523, 14775, -32188]),
        (0, -18309, [17900, -7211, 17653, -29188]),
        (0, -33332, [14154, -2391, 25759, -57727]),
        (1, -22590, [12415, -9235, 23084, -29753]),
        (1, -7734, [25800, -5990, 13965, -24389]),
        (0, -29152, [19170, -7859, 22648, -41497]),
        (0, -17295, [30408, -8383, 29153, -44779]),
        (1, -18419, [8943, -3461, 17956, -34494]),
        (0, -18640, [15895, -9689, 14070, -29915]),
        (1, -14774, [12097, -3142, 21672, -34851]),
        (0, -35011, [11454, -766, 21724, -51737]),
        (0, -42262, [23945, -5201, 15059, -48476]),
        (0, -51165, [15334, -3758, 14280, -42983]),
        (1, -22627, [10502, -10705, 21378, -28195]),
        (0, -51484, [11942, -2781, 16837, -45089]),
        (1, -19506, [17439, -4481, 20172, -43468]),
        (1, -16215, [19512, -6439, 16998, -31045]),
        (1, -34567, [19939, -9175, 20620, -51101]),
        (0, -37507, [10515, -8618, 14074, -28331]),
        (1, -33248, [16878, -5852, 12327, -39212]),
        (1, -36858, [16768, -3280, 12923, -37787]),
        (1, -33269, [15750, -3146, 14108, -31168]),
        (0, -33428, [20580, -12935, 21428, -41746]),
        (0, -28403, [17266, -2262, 21083, -38259]),
        (0, -35821, [18136, -9714, 12137, -34621]),
        (0, -31243, [23143, -7517, 13152, -32376]),
        (0, -38559, [17588, -3815, 13992, -43129]),
        (1, -24170, [5423, -5602, 18683, -30249]),
        (1, -20431, [24452, -4681, 14878, -36846]),
        (1, -14899, [18277, -978, 13038, -29076]),
        (0, -39889, [11796, -11406, 26932, -43043]),
        (1, -19311, [16290, -6875, 26905, -38089]),
        (1, -14318, [23135, -3658, 17806, -37291]),
        (0, -25667, [9204, -5291, 22132, -30292]),
        (1, -25201, [13420, -3687, 15538, -33049]),
        (0, -20407, [16317, -3528, 20675, -38364]),
        (1, -24194, [12085, -2546, 19232, -36375]),
        (0, -20817, [13719, -4315, 22548, -40000]),
        (1, -20776, [18693, -6517, 16258, -34527]),
        (0, -39642, [9496, -2161, 13475, -42009]),
        (0, -29222, [19088, -3970, 21251, -33802]),
        (0, -25329, [20714, -6071, 12140, -28287]),
        (1, -16932, [16161, -9982, 29865, -38025]),
        (1, -22798, [17220, -5524, 20112, -41425]),
        (0, -31911, [6088, -1882, 21521, -38950]),
        (1, -13437, [17966, -907, 18675, -37303]),
        (1, -12973, [15826, -3653, 15503, -31546]),
        (0, -34414, [11576, -5422, 21443, -43426]),
        (1, -32019, [11378, -2653, 13216, -43047]),
        (1, -24573, [19438, -1105, 16226, -33595]),
        (1, -29734, [18864, -3660, 17749, -43812]),
        (0, -32527, [7750, -2252, 25047, -44905]),
        (1, -13483, [14537, -4002, 17136, -28140]),
        (0, -28575, [21894, -5374, 20424, -39690]),
        (0, -32072, [13984, -2422, 15730, -39414]),
        (1, -16857, [11749, -4340, 21095, -34411]),
        (1, -17149, [15365, -4936, 12978, -27479]),
    ],
];

/// Every trial's readout counts under every candidate, in the candidates' order.
pub(crate) const ONCE_COUNTED_1024: [[Counted; BLOCK]; 3] = [
    [
        (0, [6, 10]),
        (0, [14, 12]),
        (1, [15, 23]),
        (0, [13, 9]),
        (0, [5, 5]),
        (0, [5, 9]),
        (0, [7, 4]),
        (0, [10, 5]),
        (0, [12, 13]),
        (0, [6, 4]),
        (1, [12, 15]),
        (1, [10, 13]),
        (0, [15, 11]),
        (0, [15, 12]),
        (1, [14, 25]),
        (0, [10, 10]),
        (1, [10, 10]),
        (0, [5, 9]),
        (0, [12, 13]),
        (0, [15, 12]),
        (1, [16, 14]),
        (0, [8, 4]),
        (1, [11, 14]),
        (1, [5, 4]),
        (1, [8, 14]),
        (0, [7, 7]),
        (1, [9, 12]),
        (1, [10, 4]),
        (1, [11, 7]),
        (0, [13, 12]),
        (0, [9, 13]),
        (0, [10, 12]),
        (0, [16, 12]),
        (0, [6, 5]),
        (1, [14, 12]),
        (1, [18, 12]),
        (1, [12, 6]),
        (0, [14, 10]),
        (1, [12, 15]),
        (1, [5, 10]),
        (0, [13, 8]),
        (1, [13, 6]),
        (0, [16, 12]),
        (1, [12, 12]),
        (0, [9, 5]),
        (1, [14, 10]),
        (0, [8, 10]),
        (0, [8, 15]),
        (0, [8, 5]),
        (1, [11, 10]),
        (1, [7, 10]),
        (0, [7, 12]),
        (1, [10, 8]),
        (1, [6, 9]),
        (0, [13, 16]),
        (1, [9, 13]),
        (1, [14, 12]),
        (1, [10, 7]),
        (0, [9, 11]),
        (1, [10, 12]),
        (0, [13, 11]),
        (0, [15, 9]),
        (1, [11, 8]),
        (1, [11, 11]),
    ],
    [
        (0, [8, 12]),
        (0, [12, 11]),
        (1, [24, 33]),
        (0, [15, 13]),
        (0, [6, 6]),
        (0, [5, 9]),
        (0, [8, 4]),
        (0, [11, 6]),
        (0, [12, 12]),
        (0, [5, 6]),
        (1, [15, 16]),
        (1, [10, 15]),
        (0, [16, 14]),
        (0, [14, 12]),
        (1, [17, 28]),
        (0, [14, 10]),
        (1, [12, 9]),
        (0, [5, 13]),
        (0, [10, 17]),
        (0, [15, 13]),
        (1, [19, 17]),
        (0, [8, 4]),
        (1, [10, 16]),
        (1, [7, 5]),
        (1, [8, 15]),
        (0, [11, 15]),
        (1, [8, 11]),
        (1, [10, 4]),
        (1, [10, 6]),
        (0, [18, 15]),
        (0, [10, 14]),
        (0, [9, 12]),
        (0, [16, 10]),
        (0, [8, 3]),
        (1, [18, 18]),
        (1, [17, 9]),
        (1, [13, 6]),
        (0, [19, 13]),
        (1, [13, 19]),
        (1, [6, 10]),
        (0, [15, 9]),
        (1, [15, 7]),
        (0, [19, 13]),
        (1, [15, 15]),
        (0, [11, 4]),
        (1, [13, 12]),
        (0, [6, 10]),
        (0, [7, 14]),
        (0, [10, 7]),
        (1, [13, 13]),
        (1, [10, 11]),
        (0, [11, 18]),
        (1, [13, 8]),
        (1, [8, 8]),
        (0, [13, 17]),
        (1, [12, 12]),
        (1, [13, 13]),
        (1, [10, 6]),
        (0, [13, 13]),
        (1, [11, 13]),
        (0, [14, 11]),
        (0, [16, 14]),
        (1, [12, 9]),
        (1, [12, 14]),
    ],
    [
        (0, [7, 11]),
        (0, [15, 13]),
        (1, [18, 28]),
        (0, [13, 10]),
        (0, [5, 6]),
        (0, [5, 9]),
        (0, [7, 4]),
        (0, [10, 7]),
        (0, [12, 13]),
        (0, [5, 5]),
        (1, [14, 15]),
        (1, [10, 13]),
        (0, [16, 14]),
        (0, [15, 14]),
        (1, [15, 26]),
        (0, [12, 10]),
        (1, [10, 10]),
        (0, [6, 13]),
        (0, [12, 16]),
        (0, [15, 12]),
        (1, [17, 15]),
        (0, [8, 4]),
        (1, [9, 14]),
        (1, [7, 4]),
        (1, [8, 15]),
        (0, [9, 10]),
        (1, [8, 11]),
        (1, [10, 5]),
        (1, [12, 6]),
        (0, [14, 14]),
        (0, [9, 13]),
        (0, [9, 13]),
        (0, [16, 10]),
        (0, [6, 3]),
        (1, [14, 12]),
        (1, [19, 10]),
        (1, [12, 6]),
        (0, [15, 11]),
        (1, [12, 18]),
        (1, [5, 10]),
        (0, [13, 8]),
        (1, [15, 6]),
        (0, [16, 11]),
        (1, [15, 13]),
        (0, [9, 4]),
        (1, [13, 11]),
        (0, [7, 10]),
        (0, [8, 15]),
        (0, [9, 6]),
        (1, [12, 10]),
        (1, [7, 11]),
        (0, [9, 13]),
        (1, [11, 8]),
        (1, [9, 9]),
        (0, [13, 16]),
        (1, [12, 13]),
        (1, [13, 11]),
        (1, [11, 6]),
        (0, [13, 11]),
        (1, [11, 11]),
        (0, [11, 11]),
        (0, [16, 10]),
        (1, [12, 10]),
        (1, [12, 13]),
    ],
];

/// The stimulus the round picked, as `candidate_pick` reads the pinned tables: none, no
/// candidate having fired every unit once and added under a tenth of a spike after.
pub(crate) const SHAPE_PICKED_1024: Option<Shape> = None;

/// The ticks at which unit 0 of the instrument's network at 1 024 units, at rest at the
/// gain 1.75 with no drive, fires within one pair window after one injection of each
/// shape (`probe`): F-46's two messages of 1.25 fire it twice, at the end of the
/// refractory window from what the basal compartment still holds; (a) not at all; (b)
/// once; (c) not at all. The engine's own reading of what one message does to a unit at
/// rest, pinned; the executor scales an injected message by the synaptic gain (F-47).
pub(crate) const PROBED_1024: [(Shape, &[u32]); 4] = [
    (SHAPE_F46, &[5, 206]),
    (CANDIDATE_A, &[]),
    (CANDIDATE_B, &[22]),
    (CANDIDATE_C, &[]),
];

/// Every trial's readout counts of ADR-0072's composition run, F-46's stimulus at 1 024
/// units and the gain 1.75 with the weights frozen (`the_composition_at_1024_units_exhaustive`,
/// `CALIBRATION_1024[0]`'s run): Deliverable D's readings of the instrument as it stands.
pub(crate) const COUNTED_1024: &[Counted] = &[
    (0, [16, 19]),
    (0, [11, 12]),
    (1, [29, 31]),
    (0, [14, 9]),
    (0, [2, 3]),
    (0, [5, 9]),
    (0, [5, 5]),
    (0, [9, 5]),
    (0, [9, 12]),
    (0, [6, 6]),
    (1, [25, 24]),
    (1, [6, 8]),
    (0, [18, 18]),
    (0, [13, 8]),
    (1, [15, 31]),
    (0, [15, 8]),
    (1, [11, 9]),
    (0, [7, 13]),
    (0, [10, 12]),
    (0, [12, 11]),
    (1, [24, 20]),
    (0, [12, 7]),
    (1, [14, 15]),
    (1, [5, 4]),
    (1, [8, 12]),
    (0, [10, 12]),
    (1, [9, 12]),
    (1, [8, 4]),
    (1, [11, 5]),
    (0, [19, 18]),
    (0, [8, 14]),
    (0, [9, 11]),
    (0, [14, 10]),
    (0, [8, 3]),
    (1, [26, 26]),
    (1, [15, 8]),
    (1, [10, 4]),
    (0, [20, 18]),
    (1, [15, 15]),
    (1, [6, 7]),
    (0, [17, 8]),
    (1, [16, 7]),
    (0, [15, 15]),
    (1, [13, 12]),
    (0, [12, 3]),
    (1, [13, 15]),
    (0, [9, 11]),
    (0, [8, 13]),
    (0, [8, 6]),
    (1, [11, 15]),
    (1, [7, 7]),
    (0, [10, 19]),
    (1, [13, 8]),
    (1, [5, 6]),
    (0, [16, 18]),
    (1, [12, 12]),
    (1, [9, 12]),
    (1, [7, 5]),
    (0, [20, 21]),
    (1, [12, 13]),
    (0, [14, 12]),
    (0, [15, 9]),
    (1, [14, 8]),
    (1, [10, 8]),
];

/// A candidate's frozen run: the sight's blocks and trace, the composed trials and every
/// trial's counts.
pub(crate) type CandidateRun = (Vec<Block>, u64, Vec<Composed>, Vec<Counted>);

// ------------------------------------------------ two injections (brief 034)
//
// Written before the run. ADR-0074 named the shape that neither weakens the response nor
// leaves the stimulus units their spikes at the end of the refractory window: F-46's drive,
// then a message that cancels what the basal compartment holds when the window ends. Brief
// 034 asks for it and nothing else: a `Cancel` on the task's stimulus (`task.rs`), a
// negative basal message into the presented set injected inside the trial; the offset
// `REFRACTORY_TICKS` from the trial's first tick, "not searched"; the cancel's size from
// three candidates in a fixed order — the residual the drive leaves in the basal
// compartment of a unit at rest, twice it, and the bound one message carries — each
// derived after the gain (F-47) by an integer oracle from `BASAL_LEAK_SHIFT` and
// `REFRACTORY_TICKS` and probed on a unit at rest before it is used; the first candidate
// passing the probe and ADR-0074's two clauses over a frozen run is the stimulus.
//
// The tree says two things the brief's arithmetic did not (principle 1: the repository
// wins). *First*, the membrane rule drops an input that lands inside the refractory window
// (`integrate` in `membrane.rs`: `basal_in` is zero while `refractory_ticks` is above
// zero; ADR-0018 chose that so that a burst of input cannot fire the unit the tick the
// window ends). A unit that fires its volley spike on trial tick `k` drops every input on
// ticks `k + 1 ..= k + 200` and integrates again on `k + 201`; F-46's drive lands on tick
// one, the earliest volley spike is on tick one, so the earliest tick any volley unit can
// integrate again is 202, and the brief's cancel — injected at offset 200, landing on tick
// 201 — lands inside the window of every unit that fired in the volley and is dropped.
// *Second*, what fires the unit again is not the basal residual but the soma, which the
// coupling has pulled toward half of the residual through the whole window: on the first
// tick the unit integrates again the soma stands above the threshold already, and a cancel
// that lands then must pull it back below the threshold within that one tick through the
// coupling's sixteenth, so it must take the basal compartment well below rest rather than
// to it. The oracle below (`alone`: `cortex-core`'s `integrate` stepped alone on one unit,
// with the executor's scaling by the gain and its timing, a message injected before trial
// tick `k` landing on tick `k + 1`) reads for a unit at rest: F-46's drive fires it on tick
// 5 and again on 206; the basal residual on tick 205 is 2.94 after the gain; a cancel on
// tick 206 must be at least 7.41 after the gain to leave the unit its one spike, 2.5 times
// the residual; and a cancel on any tick from 201 to 205 is dropped whatever its size.
// So the brief's three candidates at the brief's offset are all dropped, and at the tick
// the unit integrates again the residual and twice the residual and one message at the
// bound (−3.5 after the gain) are all too small. They are probed below as written, and
// the probe is the record.
//
// What the engine's rule gives instead, derived and not searched. *The offset* is
// `REFRACTORY_TICKS + 1`, 201: the cancel lands on tick 202, the first tick a unit that
// fired on tick one integrates again. *The span*: the volley's spikes fall on several
// ticks, because every unit carries the drive's standing potential and the drive adds the
// same 4.375 to each, so one tick of cancel serves the units freed on that tick and no
// other; the cancel is one message per tick over as many consecutive ticks as the volley
// spreads, read from the census of ADR-0072's composition run (the calibration's, F-46's
// stimulus, no cancel; `VOLLEY_TICKS_1024`, the presented set's volley spikes by their
// tick after the trial's first): ticks 1 to 9, so the span is 9 and the cancel lands on
// ticks 202 to 210, every unit freed on one of them receiving it on that tick and on every
// later tick of the span. A cancel that lands after a unit's first free tick is dropped in
// its next window if it fired, and takes the basal compartment further below rest if it
// did not; ADR-0074's Context said what erring large costs — the stimulus units' own later
// firing, which is F-46's excess — and nothing else within the trial, the basal
// compartment returning to rest within a few thousand ticks of a trial of 16 384. *The
// size*, in messages at the bound per tick (`spike_message` clamps one message at −2.0,
// −3.5 after the gain, so a larger cancel is more messages): (i) the least count that
// leaves a unit at rest firing once, by the oracle, 3 (−10.5 after the gain; 2 leaves it
// its second spike); (ii) the least count that leaves a unit at the most standing
// potential it can carry without firing — the basal compartment one LSB below twice the
// threshold and the soma one LSB below it — firing once, by the oracle, 6, which is twice
// (i); (iii) twice (ii), 12, for the standing states the oracle does not model (a
// synapse's message landing on the same tick), the candidate that cannot under-cancel and
// so tests whether the mechanism works at all, apart from its size. In this order and no
// other; each probed on a unit at rest through the task before any run (`probe_task`,
// `PROBED_CANCEL_1024`, in the gate, and held to the oracle); then the pick, ADR-0074's
// measure unchanged (`fires_once`: the volley one spike per unit within two before the
// readout window opens, at most a tenth of a spike per unit in the pair window after it),
// over one frozen run of sixty-four trials at 1 024 units and the gain 1.75, the
// calibration's run with the cancel the one difference, the modulation baseline at zero and
// no reward, so no weight moves; the first candidate passing the probe and both clauses is
// the stimulus; none, and there is no rewarded run. The sight (ADR-0065's) and the sign
// (ADR-0072's) are read from the same run; the criterion runs only when both pass.
//
// The expectations, written before the run. *The brief's*: the volley's potentiation stays
// near ADR-0072's 17.2 per synapse per presentation, because the first injection is F-46's
// and the response it evokes is F-46's; the volley's depression falls toward zero, because
// the stimulus units do not fire again to pair the response as depression; and the terms
// reach ADR-0072's +5 per synapse per trial, named as the Hypothesis it is. *This round's,
// from the pair rule*: the rule enters a potentiation at the presynaptic spike for the
// target's last spike only, and the spike at which the response was entered under F-46's
// stimulus was the stimulus unit's own spike at the end of its window, 201 ticks after the
// volley; with that spike gone the response is entered at the unit's next spike, its next
// presentation two trials later on average or a background spike before it, and only where
// no background spike of the readout unit (0.28 per trial) displaced the response as its
// last spike. So the volley's potentiation falls below 17.2 by about the displaced share
// (a Hypothesis: to about 10), and part of it moves into later trials' rows; the volley's
// depression falls toward zero; the background's depression falls to about a third of 26.8,
// one presynaptic spike per presentation where there were three; the background's
// potentiation stays near 7.5; the terms sum net positive, between +5 and +10; the sign
// turns positive once the standing trace has integrated a few trials of that flow; and the
// sight stays at 62 or 63, the readouts' response being F-46's.

/// The cancel's message: the most negative efficacy one message carries, −2.0
/// (`spike_message`'s clamp, held below), −3.5 after the gain of 1.75.
pub(crate) const CANCEL_MESSAGE_Q16: i32 = -0x0002_0000;

/// The brief's offset: `REFRACTORY_TICKS` from the trial's first tick, landing on tick
/// 201, inside the window of every unit that fired in the volley.
pub(crate) const BRIEF_OFFSET: u32 = REFRACTORY_TICKS as u32;

/// The cancel's offset as the engine's rule gives it: `REFRACTORY_TICKS + 1`, landing on
/// tick 202, the first tick a unit that fired on tick one integrates again.
pub(crate) const CANCEL_OFFSET: u32 = REFRACTORY_TICKS as u32 + 1;

const _: () = assert!(BRIEF_OFFSET == 200 && CANCEL_OFFSET == 201);

/// The presented set's volley spikes by their tick after the trial's first, over the
/// sixty-four trials of ADR-0072's composition run (F-46's stimulus, no cancel; 3 253 of
/// 64 × 51), as `(tick, units)`; every other tick before the readout window opens holds
/// none. Pinned from that run, which `the_composition_at_1024_units_exhaustive` holds.
pub(crate) const VOLLEY_TICKS_1024: [(u32, u64); 9] = [
    (1, 142),
    (2, 944),
    (3, 1613),
    (4, 484),
    (5, 49),
    (6, 12),
    (7, 4),
    (8, 4),
    (9, 1),
];

/// The cancel's span in ticks, derived from `VOLLEY_TICKS_1024` by `span_of` and not
/// chosen: the last tick a volley spike fell on, so that the cancel lands on the first free
/// tick of every unit of the volley. The gate holds the constant to the rule.
pub(crate) const CANCEL_TICKS: u32 = 9;

/// The most messages the oracle scans to for a least count.
pub(crate) const LEAST_SCAN: u32 = 32;

/// The most standing potential a unit carries without firing, for the oracle: the basal
/// compartment one LSB below twice the threshold (the soma's fixed point is half of it)
/// and the soma one LSB below the threshold.
pub(crate) const EXTREME_STANDING: (i32, i32) = (2 * THRESHOLD_BASE - 1, THRESHOLD_BASE - 1);

/// Candidate (i): the least count of messages at the bound per tick that leaves a unit at
/// rest firing once, by the oracle.
pub(crate) const CANCEL_ONCE_AT_REST: u32 = 3;

/// Candidate (ii): the least count that leaves a unit at `EXTREME_STANDING` firing once, by
/// the oracle.
pub(crate) const CANCEL_AT_THE_EXTREME: u32 = 6;

/// The candidates, as counts of messages at the bound per tick, in the order tried and no
/// other: (i), (ii) and twice (ii).
pub(crate) const CANCELS: [u32; 3] = [
    CANCEL_ONCE_AT_REST,
    CANCEL_AT_THE_EXTREME,
    2 * CANCEL_AT_THE_EXTREME,
];

const _: () = assert!(CANCELS[0] < CANCELS[1] && CANCELS[1] < CANCELS[2]);

/// The cancel of `messages` messages at the bound per tick over the derived span from the
/// derived offset.
pub(crate) const fn cancel_of(messages: u32) -> Cancel {
    Cancel {
        offset: CANCEL_OFFSET,
        ticks: CANCEL_TICKS,
        messages,
        efficacy_q16: CANCEL_MESSAGE_Q16,
    }
}

/// The span the census gives: the last tick a volley spike fell on; zero for no spike.
pub(crate) fn span_of(volley_ticks: &[(u32, u64)]) -> u32 {
    volley_ticks
        .iter()
        .filter(|&&(_, units)| units > 0)
        .map(|&(tick, _)| tick)
        .max()
        .unwrap_or(0)
}

/// The nonzero entries of a run's volley-tick census, as the constant holds them.
pub(crate) fn census_of(volley_ticks: &[u64]) -> Vec<(u32, u64)> {
    volley_ticks
        .iter()
        .enumerate()
        .filter(|&(_, &units)| units > 0)
        .map(|(tick, &units)| (tick as u32, units))
        .collect()
}

// ------------------------------------------------------------------------- the oracle

/// A turn's sum under the tick's gain, as the executor scales it (`scaled` in
/// `executor.rs`, ADR-0036; the injected messages among the sum, F-47): `sum × gain`,
/// Q16.16, rounded to nearest and clamped to the width; written a second time as the
/// oracle's.
pub(crate) fn scaled_q16(sum: i32, gain_q16: u32) -> i32 {
    (i64::from(sum)
        .saturating_mul(i64::from(gain_q16))
        .saturating_add(0x8000)
        >> 16)
        .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// What `messages` messages of `efficacy_q16` sum to in a unit's batch: each clamped as
/// `spike_message` clamps it, then summed, clamped to the width.
pub(crate) fn batch_q16(messages: u32, efficacy_q16: i32) -> i32 {
    i64::from(message_efficacy_q16(spike_message(efficacy_q16, false)))
        .saturating_mul(i64::from(messages))
        .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// The membrane rule stepped alone: `cortex-core`'s `integrate` on one unit at the base
/// threshold with no synapse and no drive, from `standing` basal and somatic potentials,
/// under `shape`'s messages landing on tick one and `cancel`'s on the tick after each
/// offset it is due at (the executor's timing: a message injected before trial tick `k`
/// lands on tick `k + 1`), each tick's batch scaled by `gain` as the executor scales it.
/// Returns the ticks the unit fires on within `ticks`, and its basal potential after every
/// tick.
pub(crate) fn alone(
    standing: (i32, i32),
    shape: Shape,
    cancel: Option<Cancel>,
    gain: u32,
    ticks: u32,
) -> (Vec<u32>, Vec<i32>) {
    let mut unit = DendriticSuperNeuron::new(0);
    unit.v_thresh = THRESHOLD_BASE;
    unit.v_basal = standing.0;
    unit.v_soma = standing.1;
    let mut fires = Vec::new();
    let mut basal = Vec::with_capacity(ticks as usize);
    for k in 0..ticks {
        let mut sum = if k == 0 {
            batch_q16(shape.0, shape.1)
        } else {
            0
        };
        if let Some(c) = cancel {
            if c.is_due(k) {
                sum = sum.saturating_add(batch_q16(c.messages, c.efficacy_q16));
            }
        }
        let now = k.saturating_add(1);
        if unit.integrate(scaled_q16(sum, gain), 0, now) {
            fires.push(now);
        }
        basal.push(unit.v_basal);
    }
    (fires, basal)
}

/// The residual: the basal potential, after the gain, that `shape` alone leaves in a unit
/// at rest after trial tick `tick`, by the oracle; zero for a tick before the first.
pub(crate) fn residual(shape: Shape, gain: u32, tick: u32) -> i32 {
    let (_, basal) = alone((0, 0), shape, None, gain, tick);
    basal.last().copied().unwrap_or(0)
}

/// The least count of messages at the bound per tick, as one cancel over the derived span
/// from the derived offset, that leaves a unit with `standing` potentials firing exactly
/// once within one pair window of F-46's drive, by the oracle; none up to `LEAST_SCAN`.
pub(crate) fn least_cancel(standing: (i32, i32), gain: u32) -> Option<u32> {
    (1..=LEAST_SCAN).find(|&n| {
        alone(standing, SHAPE_F46, Some(cancel_of(n)), gain, PROBE_TICKS)
            .0
            .len()
            == 1
    })
}

/// The brief's three candidates as it wrote them, one injection each at `offset` over
/// `ticks`: (i) the residual, one message carrying what F-46's drive leaves in the basal
/// compartment of a unit at rest after tick `residual_tick` divided by the gain; (ii) twice
/// it, two such messages; (iii) the bound, one message at −2.0.
pub(crate) fn brief_cancels(offset: u32, ticks: u32, residual_tick: u32) -> [Cancel; 3] {
    let after_gain = i64::from(residual(SHAPE_F46, GAIN_1024, residual_tick));
    let before_gain = (after_gain << 16)
        .checked_div(i64::from(GAIN_1024))
        .unwrap_or(0);
    let of = |messages: u32, efficacy_q16: i64| Cancel {
        offset,
        ticks,
        messages,
        efficacy_q16: efficacy_q16.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
    };
    [
        of(1, before_gain.saturating_neg()),
        of(2, before_gain.saturating_neg()),
        of(1, i64::from(CANCEL_MESSAGE_Q16)),
    ]
}

/// `probe` through the task (the composition the runs use, on one unit): the instrument's
/// network at 1 024 units at rest at the gain 1.75 with no drive, a task whose stimulus A
/// is unit 0 alone with `shape` and `cancel`, one trial of `PROBE_TICKS` ticks, and the
/// ticks after the trial's first on which unit 0 fires, read from the train.
pub(crate) fn probe_task(shape: Shape, cancel: Option<Cancel>) -> Vec<u32> {
    let p = prior(1024);
    let mut exec = at_gain(&p, config(1024, 1, 0), GAIN_1024);
    assert!(exec.is_quiescent());
    let stimulus = |first| Stimulus {
        set: Set::contiguous(first, 1),
        messages: shape.0,
        efficacy_q16: shape.1,
        cancel,
    };
    let mut t = Task {
        stimuli: [stimulus(0), stimulus(1)],
        readout: Readout::new([Set::contiguous(2, 1), Set::contiguous(3, 1)]),
        drive: Drive {
            every: 0,
            messages: 0,
            efficacy_q16: 0,
            units: 1024,
            seed: 0,
        },
        ticks: PROBE_TICKS,
        window: Window::whole(PROBE_TICKS),
        seed: SEED,
        reward_q16: 0,
        mirrored: false,
        feedback: Feedback::Withheld,
        delivery: Delivery::Global,
    };
    let trial = (0..8u64)
        .find(|&k| t.stimulus_at(k) == 0)
        .expect("a trial presents A");
    let start = exec.ticks() as u32;
    t.trial(&mut exec, trial).expect("the probe runs");
    exec.train()
        .iter()
        .filter(|&&(_, unit)| unit == 0)
        .map(|&(tick, _)| tick.wrapping_sub(start))
        .collect()
}

/// The cancel the round picks: the first candidate, in the candidates' order, whose probe
/// passed (`probed[k]`: a unit at rest fires once) and whose frozen run fires once by
/// ADR-0074's measure; none when none does.
pub(crate) fn cancel_pick(probed: &[bool], runs: &[(Block, u64, Composition)]) -> Option<u32> {
    CANCELS
        .iter()
        .zip(probed.iter())
        .zip(runs.iter())
        .find(|&((_, &probed), (block, _, composition))| {
            probed && fires_once(1024, block, composition)
        })
        .map(|((&messages, _), _)| messages)
}

// ------------------------------------------------------------- the probes (brief 034)

/// The probes, each F-46's drive into unit 0 of the instrument's network at rest at the
/// gain 1.75 with a cancel, through the task: the ticks unit 0 fires on within one pair
/// window, pinned from the engine and held to the oracle. The brief's three at the brief's
/// offset land inside the window and are dropped; at the derived span they land on tick 206
/// for a unit at rest and are too small; two messages at the bound are one below the least;
/// the three candidates leave the unit its one spike.
pub(crate) const PROBED_CANCEL_1024: [(&str, Option<Cancel>, &[u32]); 11] = [
    ("F-46's drive alone", None, &[5, 206]),
    (
        "the brief's (i), the residual, at the brief's offset",
        Some(Cancel {
            offset: BRIEF_OFFSET,
            ticks: 1,
            messages: 1,
            efficacy_q16: -111_082,
        }),
        &[5, 206],
    ),
    (
        "the brief's (ii), twice the residual, at the brief's offset",
        Some(Cancel {
            offset: BRIEF_OFFSET,
            ticks: 1,
            messages: 2,
            efficacy_q16: -111_082,
        }),
        &[5, 206],
    ),
    (
        "the brief's (iii), the bound, at the brief's offset",
        Some(Cancel {
            offset: BRIEF_OFFSET,
            ticks: 1,
            messages: 1,
            efficacy_q16: CANCEL_MESSAGE_Q16,
        }),
        &[5, 206],
    ),
    (
        "the brief's (i) at the derived span",
        Some(Cancel {
            offset: CANCEL_OFFSET,
            ticks: CANCEL_TICKS,
            messages: 1,
            efficacy_q16: -110_004,
        }),
        &[5, 206],
    ),
    (
        "the brief's (ii) at the derived span",
        Some(Cancel {
            offset: CANCEL_OFFSET,
            ticks: CANCEL_TICKS,
            messages: 2,
            efficacy_q16: -110_004,
        }),
        &[5, 206],
    ),
    (
        "the brief's (iii) at the derived span",
        Some(Cancel {
            offset: CANCEL_OFFSET,
            ticks: CANCEL_TICKS,
            messages: 1,
            efficacy_q16: CANCEL_MESSAGE_Q16,
        }),
        &[5, 206],
    ),
    (
        "two at the bound, one below the least",
        Some(cancel_of(2)),
        &[5, 206],
    ),
    ("(i) three at the bound", Some(cancel_of(CANCELS[0])), &[5]),
    ("(ii) six at the bound", Some(cancel_of(CANCELS[1])), &[5]),
    (
        "(iii) twelve at the bound",
        Some(cancel_of(CANCELS[2])),
        &[5],
    ),
];

/// The residual after the gain on tick 200 and on tick 205, by the oracle: what stands
/// when the brief's cancel would land on tick 201, and when the derived span's message
/// lands on tick 206 for a unit at rest.
pub(crate) const RESIDUAL_200_Q16: i32 = 194_395;

pub(crate) const RESIDUAL_205_Q16: i32 = 192_507;

// ---------------------------------------------------------- the measurement (brief 034)

/// The candidates at 1 024 units, in the candidates' order, each a frozen run of sixty-four
/// trials at the present gain with F-46's stimulus and its cancel, read by the sight, the
/// sign and the measure that picks: the sight's block and trace, and the composition. Pinned
/// from one run each.
pub(crate) const CANCELLED_1024: [(Block, u64, Composition); 3] = [
    (
        (
            35,
            34,
            [[414, 400], [413, 422]],
            [1727, 1526],
            [4047, 3601],
            [258, 234],
            63,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            3,
        ),
        0xa64f571232e8df09,
        (
            [[-11702, -23587], [-15667, -12495]],
            [
                [
                    [338616, -36496, 356813, -936028],
                    [344026, -40337, 398293, -930256],
                ],
                [
                    [306535, -30943, 329496, -817392],
                    [301502, -27026, 372513, -817929],
                ],
            ],
            [[1046, 2146], [1044, 2190], [3356, 2717]],
            2,
            14119637557841610480,
        ),
    ),
    (
        (
            34,
            34,
            [[321, 315], [307, 310]],
            [1726, 1525],
            [2549, 2293],
            [262, 239],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0xeeb4acda1476b4c5,
        (
            [[-6937, -10356], [-2789, 1625]],
            [
                [
                    [211499, -6013, 326747, -631916],
                    [223967, -6680, 362378, -608124],
                ],
                [
                    [192145, -3500, 295516, -553067],
                    [204182, -5113, 332692, -560717],
                ],
            ],
            [[1071, 1638], [1069, 1679], [3359, 0]],
            13,
            7428265973369681826,
        ),
    ),
    (
        (
            34,
            34,
            [[320, 315], [307, 311]],
            [1726, 1525],
            [2532, 2285],
            [262, 239],
            62,
            213902976,
            235822619,
            0,
            [[6986739, 7212710], [7146878, 7257384]],
            6,
        ),
        0xeeb4acda1476b4c5,
        (
            [[-6551, -10787], [-2591, 1724]],
            [
                [
                    [210722, -4730, 320551, -624435],
                    [223647, -4574, 357606, -602920],
                ],
                [
                    [192084, -2943, 292620, -552306],
                    [203781, -4060, 337358, -565371],
                ],
            ],
            [[1072, 1636], [1071, 1679], [3359, 0]],
            13,
            268213348253458448,
        ),
    ),
];

/// The rows of every candidate's sixty-four trials, in the candidates' order.
pub(crate) const CANCELLED_ROWS_1024: [[Row; BLOCK]; 3] = [
    [
        (0, -568, [12182, -768, 7944, -19895]),
        (0, -15067, [17670, -2543, 28054, -58764]),
        (1, -48593, [8113, -2519, 25385, -60310]),
        (0, -18340, [18891, -3411, 27630, -48722]),
        (0, -9508, [24035, -1568, 21122, -39006]),
        (0, -21635, [19043, -1404, 12973, -44990]),
        (0, -29976, [14379, -1543, 11148, -37162]),
        (0, -36213, [9733, -724, 15943, -39361]),
        (0, -38963, [13515, -1795, 15829, -36533]),
        (0, -53322, [18573, -2340, 23590, -63897]),
        (1, -14748, [18476, -3437, 24670, -34373]),
        (1, 1056, [34403, -9810, 17788, -30895]),
        (0, -50552, [15388, -3554, 22683, -52031]),
        (0, -23802, [37429, -2787, 27509, -45369]),
        (1, -21220, [10167, -2249, 16799, -40166]),
        (0, -21781, [15995, -4978, 15752, -35781]),
        (1, -12202, [17572, -787, 20953, -37924]),
        (0, -43949, [17667, -541, 23102, -66758]),
        (0, -57467, [20805, -1185, 13740, -56140]),
        (0, -71554, [16007, -2248, 13720, -53750]),
        (1, -10066, [7503, -193, 22354, -30743]),
        (0, -62813, [11754, -124, 16002, -46342]),
        (1, -8163, [21612, -1501, 20505, -53527]),
        (1, 325, [25571, -186, 20160, -39134]),
        (1, -27536, [17354, -2959, 20592, -61492]),
        (0, -44165, [7105, -1076, 16246, -38400]),
        (1, -36062, [10823, -1603, 15642, -46270]),
        (1, -42478, [18475, -781, 11812, -44162]),
        (1, -42379, [13030, -1609, 15164, -36422]),
        (0, -46672, [18906, -4313, 18490, -56356]),
        (0, -34985, [24682, -1002, 18099, -40836]),
        (0, -38668, [19237, -4259, 15233, -41678]),
        (0, -36378, [22972, -2877, 13760, -41916]),
        (0, -47866, [16916, -1305, 12825, -47905]),
        (1, -24830, [8693, -1693, 14041, -31805]),
        (1, -27229, [28136, -2348, 14398, -49692]),
        (1, -17911, [27556, -426, 16728, -39476]),
        (0, -49106, [7480, -2661, 20727, -48578]),
        (1, -27877, [15735, -1880, 22396, -44509]),
        (1, -20207, [29297, -2677, 18780, -43701]),
        (0, -27239, [12245, -3892, 21700, -35111]),
        (1, -24149, [16027, -1465, 14910, -35721]),
        (0, -20808, [21772, -4125, 20111, -42346]),
        (1, -31278, [15691, -5980, 19678, -46325]),
        (0, -12231, [19303, -1764, 25713, -42994]),
        (1, -27709, [14670, -1063, 16366, -40048]),
        (0, -34163, [12423, -729, 17824, -47131]),
        (0, -27530, [20591, -3030, 21149, -39041]),
        (0, -29790, [19367, -3565, 13285, -37107]),
        (1, -33608, [10567, -3656, 29739, -53168]),
        (1, -35523, [23235, -3681, 19913, -48837]),
        (0, -42649, [7714, -457, 20781, -49921]),
        (1, -23678, [20601, -911, 18925, -41351]),
        (1, -26450, [16639, -874, 14494, -39191]),
        (0, -39424, [14336, -2045, 19456, -51934]),
        (1, -43802, [11065, -775, 14541, -50931]),
        (1, -42310, [24929, -234, 17709, -50282]),
        (1, -49195, [19417, -156, 15327, -51847]),
        (0, -40543, [13428, -2549, 18062, -45991]),
        (1, -40486, [13835, -1501, 17136, -43892]),
        (0, -31669, [25201, -4058, 22714, -49677]),
        (0, -34393, [17242, -1613, 15433, -42833]),
        (1, -31656, [9029, -886, 20035, -37121]),
        (1, -28162, [14939, -128, 14064, -32500]),
    ],
    [
        (0, -6963, [6892, -371, 6150, -19649]),
        (0, -3987, [11614, -468, 28988, -38786]),
        (1, -33007, [3582, -79, 19021, -33911]),
        (0, -3502, [11502, -360, 16615, -28525]),
        (0, 3265, [13804, -220, 14552, -22387]),
        (0, -2586, [13284, -518, 12147, -29407]),
        (0, -1546, [11967, -470, 10398, -21270]),
        (0, -3000, [11297, -450, 16869, -30257]),
        (0, -2593, [10414, -1370, 14433, -22384]),
        (0, -10731, [11644, -210, 21163, -41122]),
        (1, -9577, [7588, -275, 21225, -20671]),
        (1, 3425, [16442, -684, 16203, -20957]),
        (0, -12080, [7007, -338, 23176, -30568]),
        (0, 10764, [22015, -278, 23524, -25103]),
        (1, -10023, [5559, -352, 13888, -25911]),
        (0, 2781, [7749, -288, 15551, -27979]),
        (1, 571, [13000, -763, 20231, -29571]),
        (0, -12008, [13424, -445, 19413, -41303]),
        (0, -16552, [11927, -105, 11260, -30157]),
        (0, -18621, [14079, -71, 12829, -31933]),
        (1, -4987, [3643, -60, 17461, -20098]),
        (0, -14620, [10973, -77, 14442, -28376]),
        (1, -2373, [10231, -502, 19851, -33537]),
        (1, 8639, [15586, -37, 16235, -21811]),
        (1, 966, [13440, -72, 17564, -35318]),
        (0, -16383, [3752, -391, 17011, -23938]),
        (1, -6027, [12644, -228, 9103, -27845]),
        (1, -6734, [12541, -141, 11041, -25653]),
        (1, -10176, [7097, -48, 11674, -24011]),
        (0, -24464, [9604, -1311, 17512, -32589]),
        (0, -18648, [13128, -82, 16846, -29934]),
        (0, -15854, [10487, -126, 12812, -23812]),
        (0, -3860, [20893, -361, 13867, -26827]),
        (0, -301, [15429, -144, 13843, -25853]),
        (1, -12236, [3475, -565, 13812, -24105]),
        (1, -16367, [14637, -222, 9489, -31093]),
        (1, -1150, [23946, -13, 10430, -22049]),
        (0, -20913, [3037, -1022, 18739, -34024]),
        (1, -2476, [9764, -390, 26627, -28099]),
        (1, 5129, [19679, -671, 15146, -26857]),
        (0, -8226, [4253, -24, 20340, -22052]),
        (1, -3066, [10658, -496, 13102, -20436]),
        (0, -4401, [12909, -812, 16116, -25859]),
        (1, -1368, [7774, -258, 14156, -20628]),
        (0, -3169, [6680, -86, 20125, -29361]),
        (1, 723, [12373, -411, 15729, -25399]),
        (0, -14467, [7145, -162, 12994, -27216]),
        (0, 2299, [18344, -423, 19839, -23641]),
        (0, 6442, [14917, -10, 11962, -21811]),
        (1, -522, [8723, -431, 28367, -27680]),
        (1, -1468, [11603, -321, 15689, -28452]),
        (0, -8908, [5089, -179, 20427, -31225]),
        (1, 5190, [15363, -359, 17525, -26824]),
        (1, 3041, [13266, -458, 13980, -28909]),
        (0, -11583, [6309, -110, 20558, -29569]),
        (1, -15503, [9078, -14, 12544, -34100]),
        (1, -18260, [12935, -84, 13025, -31660]),
        (1, -16460, [15610, -58, 14680, -32215]),
        (0, -9627, [8651, -738, 20035, -26265]),
        (1, -8530, [10257, -244, 12699, -25761]),
        (0, -13671, [12389, -561, 17925, -30486]),
        (0, -12158, [12737, -112, 13148, -28440]),
        (1, -4647, [5586, -260, 21347, -24995]),
        (1, -1164, [10524, -116, 10675, -18246]),
    ],
    [
        (0, -6963, [6892, -371, 6150, -19649]),
        (0, -2217, [11545, -160, 28610, -36833]),
        (1, -33006, [3582, -79, 19021, -33910]),
        (0, -2948, [11350, -360, 16222, -28525]),
        (0, 3848, [13842, -217, 14626, -22358]),
        (0, -1801, [13284, -518, 12105, -29035]),
        (0, -1157, [11967, -470, 10121, -21270]),
        (0, -2723, [11297, -450, 16869, -30256]),
        (0, -1430, [8673, -145, 13906, -20368]),
        (0, -8373, [13139, -151, 19935, -39896]),
        (1, -9606, [7588, -275, 21226, -20677]),
        (1, 2497, [15530, -205, 18413, -23607]),
        (0, -12267, [7007, -338, 23250, -31898]),
        (0, 10393, [21592, -278, 23830, -25100]),
        (1, -9711, [5559, -352, 13948, -25907]),
        (0, 2636, [7749, -288, 15551, -27977]),
        (1, 743, [12999, -763, 20232, -29552]),
        (0, -12062, [13424, -445, 19413, -41301]),
        (0, -16625, [11927, -105, 11260, -30157]),
        (0, -18657, [14079, -71, 12829, -31933]),
        (1, -4902, [3643, -60, 17461, -20095]),
        (0, -14648, [10973, -77, 14444, -28376]),
        (1, -2318, [10231, -502, 19851, -33537]),
        (1, 8679, [15586, -37, 16235, -21811]),
        (1, 1001, [13440, -72, 17564, -35318]),
        (0, -15735, [3767, -344, 16169, -22433]),
        (1, -6006, [12644, -228, 9103, -27842]),
        (1, -6709, [12541, -141, 11044, -25653]),
        (1, -10160, [7097, -48, 11674, -24011]),
        (0, -24149, [9604, -1311, 17514, -32589]),
        (0, -18236, [13000, -36, 16817, -29700]),
        (0, -15517, [10615, -126, 12749, -23810]),
        (0, -1056, [20604, -124, 14253, -24293]),
        (0, -716, [15678, -143, 10376, -25766]),
        (1, -13647, [3475, -565, 13018, -24105]),
        (1, -17426, [14637, -219, 9489, -31091]),
        (1, -1915, [23948, -13, 10434, -22049]),
        (0, -18812, [2558, -291, 18499, -31818]),
        (1, -2232, [9764, -390, 27894, -28263]),
        (1, 5121, [19221, -490, 15145, -26905]),
        (0, -7602, [4254, -24, 20284, -22052]),
        (1, -3111, [10801, -198, 12534, -20245]),
        (0, -4239, [12381, -320, 16116, -25852]),
        (1, -1254, [7685, -50, 13564, -20005]),
        (0, -2994, [7208, -86, 19721, -29361]),
        (1, 1257, [12175, -107, 15592, -25040]),
        (0, -14331, [7145, -162, 12994, -27216]),
        (0, 2401, [18344, -423, 19839, -23641]),
        (0, 6520, [14917, -10, 11965, -21809]),
        (1, -173, [8859, -431, 28408, -27680]),
        (1, -1223, [11603, -321, 15689, -28450]),
        (0, -8861, [5089, -179, 20427, -31225]),
        (1, 5455, [15363, -359, 17525, -26822]),
        (1, 1795, [13266, -458, 12576, -28892]),
        (0, -12593, [6309, -110, 19522, -29569]),
        (1, -16251, [9078, -14, 12544, -34100]),
        (1, -18844, [12937, -84, 13025, -31660]),
        (1, -16918, [15610, -58, 14680, -32215]),
        (0, -9563, [8651, -737, 19807, -25554]),
        (1, -8690, [9741, -107, 12371, -24802]),
        (0, -13895, [12389, -322, 16302, -29495]),
        (0, -12390, [12737, -112, 13142, -28440]),
        (1, -4316, [5586, -260, 21347, -24994]),
        (1, -867, [10524, -116, 10676, -18245]),
    ],
];

/// Every trial's readout counts under each candidate, in the candidates' order.
pub(crate) const CANCELLED_COUNTED_1024: [[Counted; BLOCK]; 3] = [
    [
        (0, [10, 15]),
        (0, [14, 13]),
        (1, [28, 37]),
        (0, [15, 13]),
        (0, [6, 6]),
        (0, [5, 9]),
        (0, [6, 5]),
        (0, [10, 6]),
        (0, [12, 13]),
        (0, [4, 6]),
        (1, [21, 21]),
        (1, [11, 14]),
        (0, [18, 16]),
        (0, [15, 9]),
        (1, [18, 31]),
        (0, [14, 12]),
        (1, [16, 10]),
        (0, [5, 13]),
        (0, [10, 15]),
        (0, [13, 13]),
        (1, [22, 21]),
        (0, [11, 5]),
        (1, [15, 21]),
        (1, [6, 4]),
        (1, [8, 15]),
        (0, [11, 15]),
        (1, [9, 12]),
        (1, [12, 5]),
        (1, [11, 6]),
        (0, [21, 19]),
        (0, [10, 16]),
        (0, [10, 13]),
        (0, [16, 10]),
        (0, [9, 3]),
        (1, [22, 23]),
        (1, [21, 8]),
        (1, [14, 7]),
        (0, [22, 15]),
        (1, [15, 21]),
        (1, [7, 12]),
        (0, [17, 10]),
        (1, [17, 7]),
        (0, [17, 15]),
        (1, [16, 15]),
        (0, [13, 4]),
        (1, [13, 16]),
        (0, [8, 11]),
        (0, [8, 14]),
        (0, [10, 8]),
        (1, [11, 14]),
        (1, [9, 10]),
        (0, [12, 20]),
        (1, [12, 9]),
        (1, [8, 10]),
        (0, [16, 19]),
        (1, [15, 12]),
        (1, [10, 14]),
        (1, [9, 7]),
        (0, [17, 16]),
        (1, [12, 14]),
        (0, [14, 13]),
        (0, [15, 10]),
        (1, [13, 12]),
        (1, [12, 14]),
    ],
    [
        (0, [6, 9]),
        (0, [15, 12]),
        (1, [17, 22]),
        (0, [10, 7]),
        (0, [4, 5]),
        (0, [5, 9]),
        (0, [6, 4]),
        (0, [9, 4]),
        (0, [11, 13]),
        (0, [8, 5]),
        (1, [13, 15]),
        (1, [10, 12]),
        (0, [14, 11]),
        (0, [13, 11]),
        (1, [12, 25]),
        (0, [9, 9]),
        (1, [8, 9]),
        (0, [5, 13]),
        (0, [10, 14]),
        (0, [12, 11]),
        (1, [13, 9]),
        (0, [8, 5]),
        (1, [9, 14]),
        (1, [5, 5]),
        (1, [9, 12]),
        (0, [5, 6]),
        (1, [7, 9]),
        (1, [9, 4]),
        (1, [11, 7]),
        (0, [13, 10]),
        (0, [9, 13]),
        (0, [8, 13]),
        (0, [16, 9]),
        (0, [6, 2]),
        (1, [10, 12]),
        (1, [20, 11]),
        (1, [9, 6]),
        (0, [10, 9]),
        (1, [13, 13]),
        (1, [4, 12]),
        (0, [12, 7]),
        (1, [12, 5]),
        (0, [13, 12]),
        (1, [13, 11]),
        (0, [9, 4]),
        (1, [12, 7]),
        (0, [8, 8]),
        (0, [8, 17]),
        (0, [8, 6]),
        (1, [8, 9]),
        (1, [7, 9]),
        (0, [6, 12]),
        (1, [11, 7]),
        (1, [6, 9]),
        (0, [11, 15]),
        (1, [9, 13]),
        (1, [10, 10]),
        (1, [9, 5]),
        (0, [10, 11]),
        (1, [13, 13]),
        (0, [10, 9]),
        (0, [14, 10]),
        (1, [10, 6]),
        (1, [8, 9]),
    ],
    [
        (0, [6, 9]),
        (0, [15, 12]),
        (1, [17, 22]),
        (0, [10, 7]),
        (0, [3, 5]),
        (0, [5, 9]),
        (0, [6, 4]),
        (0, [9, 4]),
        (0, [11, 13]),
        (0, [8, 5]),
        (1, [13, 15]),
        (1, [10, 12]),
        (0, [14, 11]),
        (0, [13, 11]),
        (1, [12, 25]),
        (0, [9, 9]),
        (1, [8, 9]),
        (0, [5, 13]),
        (0, [10, 14]),
        (0, [12, 11]),
        (1, [13, 9]),
        (0, [8, 5]),
        (1, [9, 14]),
        (1, [5, 5]),
        (1, [9, 12]),
        (0, [5, 6]),
        (1, [7, 9]),
        (1, [9, 4]),
        (1, [11, 7]),
        (0, [13, 10]),
        (0, [9, 13]),
        (0, [8, 13]),
        (0, [16, 9]),
        (0, [6, 2]),
        (1, [10, 12]),
        (1, [20, 11]),
        (1, [9, 6]),
        (0, [10, 9]),
        (1, [13, 13]),
        (1, [4, 12]),
        (0, [12, 7]),
        (1, [12, 5]),
        (0, [13, 12]),
        (1, [13, 11]),
        (0, [9, 4]),
        (1, [12, 8]),
        (0, [8, 8]),
        (0, [8, 17]),
        (0, [8, 6]),
        (1, [8, 9]),
        (1, [7, 9]),
        (0, [6, 12]),
        (1, [11, 7]),
        (1, [6, 9]),
        (0, [11, 15]),
        (1, [9, 13]),
        (1, [10, 10]),
        (1, [9, 5]),
        (0, [10, 11]),
        (1, [13, 13]),
        (0, [10, 9]),
        (0, [14, 10]),
        (1, [10, 6]),
        (1, [8, 9]),
    ],
];

/// The presented set's volley spikes by tick under each candidate, in the candidates'
/// order: the volley is F-46's, the cancel landing after it, and the census differs from
/// `VOLLEY_TICKS_1024` by the few units the previous trial's cancel left elsewhere.
pub(crate) const CANCELLED_CENSUS_1024: [&[(u32, u64)]; 3] = [
    &[
        (1, 142),
        (2, 943),
        (3, 1613),
        (4, 485),
        (5, 48),
        (6, 13),
        (7, 4),
        (8, 3),
        (9, 2),
    ],
    &[
        (1, 142),
        (2, 942),
        (3, 1611),
        (4, 486),
        (5, 50),
        (6, 11),
        (7, 4),
        (8, 4),
        (9, 1),
    ],
    &[
        (1, 142),
        (2, 944),
        (3, 1609),
        (4, 486),
        (5, 50),
        (6, 11),
        (7, 4),
        (8, 4),
        (9, 1),
    ],
];

/// The cancel the rules pick over the pinned tables: candidate (ii), six messages at the
/// bound per tick, the first whose probe passed and whose run fires once — the volley whole
/// (1 726 and 1 525 of 34 × 51 and 30 × 51) and no spike of the presented set in the pair
/// window after the readout window opens, against F-46's 2.06 per unit; (i) leaves 0.83 per
/// unit, the units whose standing potential the least count at rest does not cover.
pub(crate) const CANCEL_PICKED_1024: Option<u32> = Some(CANCEL_AT_THE_EXTREME);

/// Whether the picked cancel's run passes both calibrations at 1.75: the sight passes (62 of
/// 64) and the sign does not (13 of 64), so there is no rewarded run.
pub(crate) const CANCEL_CALIBRATED_1024: bool = false;

/// A candidate's frozen run with its cancel: the sight's blocks and trace, the composed
/// trials, every trial's counts and the volley-tick census.
pub(crate) type CancelledRun = (Vec<Block>, u64, Vec<Composed>, Vec<Counted>, Vec<u64>);

// ------------------------------------------------ the background side (brief 035)
//
// Step 2 of H-12's stopping rule (whitepaper §11.1): the background, in its two named
// settings, each tried alone and in a fixed order — the network settled before the task with
// the gain held at 1.75, then the criticality controller on at ADR-0055's step of an eighth —
// and picked by three measures that read no selection, reward or outcome: ADR-0074's measure
// that the stimulus still fires once, ADR-0065's sight and ADR-0072's sign, all unchanged. The
// stimulus is ADR-0076's pick, its efficacies pinned under both candidates (under the
// controller the drive it delivers follows the gain, F-47, which is part of what that
// candidate is); the rule, the geometry, the readout window, the trial and the seeds are
// unchanged. The candidates are never combined, and the order is fixed so that no preference
// chooses.
//
// The lead-in, per candidate: the instrument's executor at 1 024 units (the prior of ADR-0044
// at seed 22, the gain 1.75 in the image, the modulation baseline 0.5 as the rewarded runs
// have it, no sleep, the inhibitory period at its default) under the drive of ADR-0044 alone,
// no task, whole windows of $2^{17}$ ticks read per window — the population's spikes, the two
// readout sets' spikes, the sums by polarity, the fraction at the target, the gain and the
// estimate — until ADR-0055's criterion holds or the bound is reached. The criterion, as an
// integer rule written before the run (`settled_within`): a window $W$ is settled when each of
// the sixteen windows up to and including $W$ is within 0.75 per cent of the sum after the
// window sixteen before $W$ (the prior's sum for $W = 16$), $10\,000\,|E_k - E_{W-16}| <
// 75\,E_{W-16}$, strict, which is at least as strict as ADR-0055's reading ("every one of the
// last sixteen windows is within 0.75 per cent of the sixty-fourth", this rule at $W = 80$).
// Not brief 026's clause: ADR-0070 read that one holding while the sum still fell by a third.
// The lead-in is the first such $W$; the bound is eighty windows (ADR-0055's day), and a
// lead-in unsettled at the eightieth runs on to 160, the cost stated in the ADR; unsettled at
// 160, the lead-in is 160 and the ADR says the sum was still moving and at what rate. The rule
// is evaluated after every window, so the lead-in stops at $W$ and the frozen run starts from
// the state after it: a `for` over the larger bound, left when the rule holds.
//
// The settled engine is then run quiet without the drive until it is quiescent (`quiet`, a
// countdown to one window of ticks; the ticks it took and what it changed in the sums are
// read, not assumed) — encoding requires it — and saved as an image whose modulator record
// carries a baseline of zero (`frozen_image`, `frozen_from`), the one patch, so that the
// frozen run's weights do not move (asserted after every block); the homeostasis record is
// left as the lead-in left it, so under the controller the gain it reached and its step are
// carried and the controller still regulates through the frozen run. The clock resumes where
// the image was written (ADR-0033), so every stamp keeps its meaning; the train is not in the
// image, so the composer's oracle is seeded from the record before the first trial (each
// block's stamp and trace, each unit's last spike; `Composer::enumerate`) and is held to the
// record at every trial, as ADR-0072 holds it.
//
// The three measures, each over the frozen run of sixty-four trials (`compose_on` on the
// decoded engine, ADR-0076's stimulus): `fires_once` (the volley one per unit within two
// before the readout window opens, at most a tenth of a spike per unit in the pair window
// after it), `calibrated` (the sight, 56 of 64) and the sign (56 of 64). A candidate passes
// when all three do; the first that passes is the configuration (`background_pick`); none,
// and there is no rewarded run and the rule's step 3 applies. Deliverable D's readings and the
// composition are read from the same run under each candidate.
//
// The prediction for the controller, written before it ran, a Hypothesis from ADR-0053 and
// ADR-0055 on the lattice prior from a gain of 2.0 (`CONTROLLER_PREDICTED`, read by
// `controller_read`): (i) the gain after the lead-in's last window is above 1.75; (ii) the
// gain's course is not settled by the rule above; (iii) the two readout sets fire more in the
// lead-in's last window than under the settled network in its last; (iv) the candidate fails
// at least one of the three measures. The settled network carries no prediction: whether
// settling the weights escapes the squeeze ADR-0070 and ADR-0072's ladder found — a quieter
// background and a weaker propagation of the stimulus — is what its run measures.

/// The prior's sums at 1 024 units before any window, as the calibration pinned them with the
/// weights frozen: (inhibitory, excitatory).
pub(crate) const PRIOR_SUMS_1024: (i64, i64) = (CALIBRATION_1024[0].0.7, CALIBRATION_1024[0].0.8);

const _: () = assert!(PRIOR_SUMS_1024.0 == 213_902_976 && PRIOR_SUMS_1024.1 == 235_822_619);

/// ADR-0055's criterion: sixteen windows, each within 75 per ten thousand (0.75 per cent) of
/// the sum after the window sixteen before.
pub(crate) const SETTLED_WINDOWS: usize = 16;

pub(crate) const SETTLED_PER_MYRIAD: u64 = 75;

/// The lead-in's bound, eighty windows (the length ADR-0055 gave 1 024 units), and the bound
/// a lead-in unsettled at the eightieth runs on to.
pub(crate) const LEAD_IN_BOUND: u64 = 80;

pub(crate) const LEAD_IN_EXTENDED: u64 = 160;

const _: () =
    assert!(LEAD_IN_EXTENDED == 2 * LEAD_IN_BOUND && LEAD_IN_BOUND > SETTLED_WINDOWS as u64);

/// The controller's step, ADR-0055's eighth (Q0.16), the step already measured on this network
/// size and not searched.
pub(crate) const CONTROL_STEP_EIGHTH: u16 = 0x2000;

const _: () = assert!(CONTROL_STEP_EIGHTH as u32 * 8 == 1 << 16);

/// The candidates, as the controller's step each runs under, in the order tried and no other:
/// the settled network (the gain held, step 0, as every run of the instrument), then the
/// controller.
pub(crate) const BACKGROUNDS: [u16; 2] = [0, CONTROL_STEP_EIGHTH];

/// The quiet run's bound: one window of ticks.
pub(crate) const QUIET_BOUND: u64 = WINDOW_TICKS;

/// The windows the gate runs of the settled lead-in, held to the first rows of its table:
/// two, $2^{18}$ ticks at 1 024 units.
pub(crate) const GATE_WINDOWS: u64 = 2;

const _: () = assert!(GATE_WINDOWS < SETTLED_WINDOWS as u64);

/// One window of a lead-in (brief 035): the population's spikes; the two readout sets' spikes;
/// the inhibitory and the excitatory sum over the arena after the window; the fraction of
/// units at the target (Q16.16, ADR-0057); and the gain and the estimate as the window's
/// regulation left them.
pub(crate) type LeadInWindow = (u64, [u64; 2], i64, i64, u32, u32, u32);

/// ADR-0055's criterion as a rule over the excitatory sums of a lead-in, `prior` the sum
/// before the first window: the smallest window (from one) at which each of the last
/// `SETTLED_WINDOWS` windows up to and including it is within `SETTLED_PER_MYRIAD` per ten
/// thousand of the sum after the window `SETTLED_WINDOWS` before it (the prior's for the
/// sixteenth); none when no window of the table is. Integers throughout: a sum `s` is within
/// `p` per myriad of a reference `r` when `10 000 |s − r| < p r`, strict either way.
pub(crate) fn settled_within(prior: i64, sums: &[i64]) -> Option<usize> {
    let sum_after = |window: usize| -> Option<i64> {
        window
            .checked_sub(1)
            .map_or(Some(prior), |k| sums.get(k).copied())
    };
    (SETTLED_WINDOWS..=sums.len()).find(|&window| {
        let first = window.wrapping_sub(SETTLED_WINDOWS);
        let Some(reference) = sum_after(first) else {
            return false;
        };
        let bound = u64::try_from(reference)
            .unwrap_or(0)
            .saturating_mul(SETTLED_PER_MYRIAD);
        (first.wrapping_add(1)..=window).all(|k| match sum_after(k) {
            Some(sum) => sum.abs_diff(reference).saturating_mul(10_000) < bound,
            None => false,
        })
    })
}

/// The excitatory sums of a lead-in table, window by window.
pub(crate) fn lead_in_sums(table: &[LeadInWindow]) -> Vec<i64> {
    table.iter().map(|w| w.3).collect()
}

/// The candidate's executor at 1 024 units (brief 035): the instrument's network at the gain
/// 1.75 with the modulation baseline 0.5 (the rewarded runs') and the controller's step `step`
/// in its homeostasis record; at 0 the gain is held, as in every run of the instrument.
pub(crate) fn candidate(units: u32, step: u16) -> Engine {
    let p = prior(units);
    let cfg = Config {
        control_step_q0_16: step,
        ..config(units, 2, BASELINE_Q16)
    };
    let exec = at_gain(&p, cfg, gain(units));
    assert_eq!(exec.homeostasis().synaptic_gain_q16, gain(units));
    assert_eq!(exec.homeostasis().control_step_q0_16, step);
    assert_eq!(exec.modulation_baseline_q16(), BASELINE_Q16);
    exec
}

/// The two readout sets' spikes over `[from, to)`, read from the train, which must hold the
/// whole window (the caller asserts it). The lead-in's ticks stay below the stamp's width, so
/// the stamp is the tick.
pub(crate) fn readout_spikes(
    exec: &mut Engine,
    readouts: [Set; 2],
    from: u64,
    to: u64,
) -> [u64; 2] {
    let mut counts = [0u64; 2];
    for &(tick, unit) in exec.train() {
        let tick = u64::from(tick);
        if tick >= from && tick < to {
            for (count, set) in counts.iter_mut().zip(readouts.iter()) {
                if set.contains(unit) {
                    *count = count.saturating_add(1);
                }
            }
        }
    }
    counts
}

/// One window of a lead-in under `drive`, run and read: the population's spikes and the
/// fraction at the target over it, the readout sets' spikes, the sums after it, and the gain
/// and the estimate as the window's regulation left them; the train asserted to have held
/// the window whole, as `settling` asserts it.
pub(crate) fn lead_in_window(exec: &mut Engine, drive: &Drive, readouts: [Set; 2]) -> LeadInWindow {
    let from = exec.ticks();
    let held = exec.train().len() as u64;
    let overwritten = exec.train_overwritten();
    lead_in(exec, drive, 1);
    let to = exec.ticks();
    assert!(
        exec.train_overwritten().wrapping_sub(overwritten) <= held,
        "the train held the window"
    );
    let (spikes, at_target) = spikes_and_at_target(exec, from, to);
    let readout = readout_spikes(exec, readouts, from, to);
    let (inhibitory, excitatory) = weights_by_polarity(exec);
    let h = exec.homeostasis();
    (
        spikes,
        readout,
        inhibitory,
        excitatory,
        at_target,
        h.synaptic_gain_q16,
        h.branching_ratio_q16,
    )
}

/// A candidate's lead-in (brief 035): whole windows under the drive alone from the executor's
/// clock, read per window, until ADR-0055's criterion holds over the table so far or `bound`
/// windows have run; the table's length is the lead-in. A `for` over the bound, left when the
/// rule holds: it ends by construction. `prior` is the excitatory sum before the first window.
pub(crate) fn lead_in_until_settled(
    exec: &mut Engine,
    units: u32,
    prior: i64,
    bound: u64,
) -> Vec<LeadInWindow> {
    let drive = drive(units);
    let [_, _, r0, r1] = geometry(units, rotation(units));
    let mut table = Vec::new();
    for _ in 0..bound {
        table.push(lead_in_window(exec, &drive, [r0, r1]));
        if settled_within(prior, &lead_in_sums(&table)).is_some() {
            break;
        }
    }
    table
}

/// Runs `exec` quiet, without the drive, until it is quiescent — no unit holds a message, no
/// token is in flight, the injector is empty: the state an image is written from — and
/// returns the ticks it took. A countdown from `QUIET_BOUND`, so it ends by construction; a
/// network still active at the bound fails the test, which is a reading of its own.
pub(crate) fn quiet(exec: &mut Engine) -> u64 {
    let mut ticks = 0u64;
    for _ in 0..QUIET_BOUND {
        if exec.is_quiescent() {
            return ticks;
        }
        exec.tick();
        ticks = ticks.wrapping_add(1);
    }
    assert!(
        exec.is_quiescent(),
        "the network fell quiet within {QUIET_BOUND} ticks"
    );
    ticks
}

/// The image of `exec`, quiescent, with its modulator record's baseline patched to zero, the
/// one change, so that an engine decoded from it consolidates nothing: the frozen form of the
/// settled network. Every other section is as the lead-in left it, the homeostasis record's
/// gain and step among them.
pub(crate) fn frozen_image(exec: &Engine) -> Vec<u8> {
    let mut img = Image::encode(exec).expect("quiescent");
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_MODULATOR {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            img[offset..][16..20].copy_from_slice(&0i32.to_le_bytes());
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            return img;
        }
    }
    panic!("the image holds a modulator section");
}

/// The frozen engine from a settled candidate's image, decoded under the calibration's
/// configuration (two workers, a train that holds a trial); the image's baseline, gain and
/// step outrank the configuration's (§8.3), and the baseline is asserted zero.
pub(crate) fn frozen_from(image: &[u8], units: u32) -> Engine {
    let exec = Image::decode::<2048>(image, config(units, 2, 0)).expect("a well-formed record");
    assert_eq!(exec.modulation_baseline_q16(), 0, "the weights are frozen");
    exec
}

/// A candidate's frozen run (brief 035): the composition's run of `trials` trials with
/// ADR-0076's stimulus — F-46's drive with the cancel it picked — on the frozen engine, the
/// composer seeded from the record; the sight's blocks and trace, the composed trials, every
/// trial's counts and the volley-tick census.
pub(crate) fn background_run(exec: &mut Engine, units: u32, trials: usize) -> CancelledRun {
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    let (blocks, trace, trials, counts, counted, volley_ticks) =
        compose_on(exec, SHAPE_F46, Some(cancel_of(picked)), units, trials);
    assert_eq!(counts, SYNAPSES_1024);
    (blocks, trace, trials, counted, volley_ticks)
}

/// A candidate passes when all three measures do: the stimulus still fires once (ADR-0074's
/// `fires_once`), the sight (ADR-0065's `calibrated`) and the sign (ADR-0072's, `SIGN_MIN`),
/// the last two `passes`.
pub(crate) fn background_passes(block: &Block, composition: &Composition) -> bool {
    fires_once(1024, block, composition) && passes(block, composition)
}

/// The configuration the round picks: the first candidate, in the candidates' order, whose
/// frozen run passes all three measures, as its step; none when none does.
pub(crate) fn background_pick(runs: &[(Block, u64, Composition)]) -> Option<u16> {
    BACKGROUNDS
        .iter()
        .zip(runs.iter())
        .find(|(_, (block, _, composition))| background_passes(block, composition))
        .map(|(&step, _)| step)
}

/// The prediction for the controller, read over its lead-in, the settled network's and its
/// frozen run (the four clauses above, in order): true where the clause holds.
pub(crate) fn controller_read(
    controller: &[LeadInWindow],
    settled: &[LeadInWindow],
    run: &(Block, u64, Composition),
) -> [bool; 4] {
    let last = controller
        .last()
        .expect("a window of the controller's lead-in");
    let settled_last = settled.last().expect("a window of the settled lead-in");
    let gains: Vec<i64> = controller.iter().map(|w| i64::from(w.5)).collect();
    [
        last.5 > GAIN_1024,
        settled_within(i64::from(GAIN_1024), &gains).is_none(),
        last.1[0].saturating_add(last.1[1]) > settled_last.1[0].saturating_add(settled_last.1[1]),
        !background_passes(&run.0, &run.2),
    ]
}

/// The prediction, written before the controller ran: every clause holds.
pub(crate) const CONTROLLER_PREDICTED: [bool; 4] = [true; 4];

/// A candidate's readings: the lead-in (dumped and held to its table), the quiet run's ticks
/// and the sums after it, the frozen image and its engine (the sums, the gain and the step
/// carried across, asserted), and the frozen run read by the three measures, dumped before
/// anything is held to its table so that a failure still shows the readings.
pub(crate) fn background_candidate(k: usize, name: &str) {
    let step = BACKGROUNDS[k];
    let mut exec = candidate(1024, step);
    assert_eq!(weights_by_polarity(&exec), PRIOR_SUMS_1024);
    let table = lead_in_until_settled(&mut exec, 1024, PRIOR_SUMS_1024.1, LEAD_IN_EXTENDED);
    let settled = settled_within(PRIOR_SUMS_1024.1, &lead_in_sums(&table));
    eprintln!(
        "DUMP {name} lead-in {table:?} settled {settled:?} windows {}",
        table.len()
    );
    let ticks = quiet(&mut exec);
    let quieted = weights_by_polarity(&exec);
    let (gain_after, estimate_after) = (
        exec.homeostasis().synaptic_gain_q16,
        exec.homeostasis().branching_ratio_q16,
    );
    eprintln!(
        "DUMP {name} quiet {ticks} sums {quieted:?} gain {gain_after:#x} estimate {estimate_after:#x}"
    );
    let image = frozen_image(&exec);
    let mut frozen = frozen_from(&image, 1024);
    assert_eq!(
        weights_by_polarity(&frozen),
        quieted,
        "{name}: the image carries the weights"
    );
    assert_eq!(
        frozen.homeostasis().synaptic_gain_q16,
        gain_after,
        "{name}: and the gain"
    );
    assert_eq!(
        frozen.homeostasis().control_step_q0_16,
        step,
        "{name}: and the step"
    );
    let (blocks, trace, trials, counted, volley_ticks) = background_run(&mut frozen, 1024, BLOCK);
    dump_composition(name, &blocks, trace, &trials);
    dump_requires(name, &counted, false, OFFSET_MARK_64);
    let composed = composition(&trials);
    eprintln!(
        "DUMP {name} volley {} after {} fires_once {} sight {} sign {} passes {} census {:?} gain after the run {:#x}",
        volley_once(1024, &blocks[0]),
        after_quiet(1024, &composed),
        fires_once(1024, &blocks[0], &composed),
        calibrated(&blocks[0]),
        composed.3,
        background_passes(&blocks[0], &composed),
        census_of(&volley_ticks),
        frozen.homeostasis().synaptic_gain_q16
    );
    assert_eq!(
        table.as_slice(),
        BACKGROUND_LEAD_IN_1024[k],
        "{name}: the lead-in"
    );
    assert_eq!((ticks, quieted), QUIET_1024[k], "{name}: the quiet run");
    let (block, pin, pinned_composed) = &BACKGROUND_1024[k];
    pinned(&format!("{name} sight"), &blocks, trace, &[*block], *pin);
    pinned_composition(
        name,
        &trials,
        &BACKGROUND_ROWS_1024[k],
        Some(pinned_composed),
    );
    assert_eq!(
        counted.as_slice(),
        &BACKGROUND_COUNTED_1024[k],
        "{name}: the counts"
    );
    assert_eq!(
        census_of(&volley_ticks),
        BACKGROUND_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

// ---------------------------------------------------------- the measurement (brief 035)

/// The candidates' lead-ins at 1 024 units, in the candidates' order, each pinned from one
/// run: the settled network's under the gain held at 1.75, the controller's under its step.
pub(crate) const BACKGROUND_LEAD_IN_1024: [&[LeadInWindow]; 2] = [
    &[
        (
            2446,
            [1085, 1121],
            213359105,
            235275227,
            27968,
            114688,
            18782,
        ),
        (
            2329,
            [1001, 1079],
            212880571,
            234948592,
            25984,
            114688,
            14600,
        ),
        (
            2306,
            [1043, 1031],
            212399982,
            234652372,
            26176,
            114688,
            14377,
        ),
        (2338, [1012, 1092], 211856927, 234369738, 25472, 114688, 0),
        (
            2388,
            [1059, 1077],
            211341639,
            234055848,
            27456,
            114688,
            7401,
        ),
        (2198, [1003, 986], 210858800, 233802827, 24512, 114688, 0),
        (2418, [1153, 1029], 210394382, 233441558, 27072, 114688, 0),
        (2319, [1073, 1009], 209931362, 233165658, 25344, 114688, 105),
        (2282, [986, 1071], 209440186, 232893632, 25536, 114688, 0),
        (
            2462,
            [1091, 1133],
            208973256,
            232558620,
            29184,
            114688,
            5648,
        ),
        (
            2247,
            [1001, 1019],
            208467654,
            232328675,
            24960,
            114688,
            7433,
        ),
        (2258, [1037, 1001], 208020347, 232055777, 25664, 114688, 0),
        (2331, [1055, 1031], 207572956, 231757255, 26368, 114688, 0),
        (2228, [982, 998], 207115120, 231511854, 24448, 114688, 5733),
        (2257, [1030, 995], 206623333, 231305112, 24384, 114688, 0),
        (2218, [986, 1012], 206197397, 231084619, 24320, 114688, 0),
        (
            2314,
            [1097, 1005],
            205748364,
            230786158,
            25792,
            114688,
            24506,
        ),
        (2217, [1045, 953], 205245258, 230560266, 23872, 114688, 0),
        (2292, [1050, 995], 204814608, 230327218, 25152, 114688, 0),
        (2223, [1023, 981], 204324017, 230129233, 23936, 114688, 0),
        (2378, [1087, 1040], 203855587, 229866069, 27072, 114688, 0),
        (
            2338,
            [1075, 1046],
            203411889,
            229586949,
            25600,
            114688,
            2043,
        ),
        (2341, [1069, 1061], 202912718, 229297053, 26816, 114688, 0),
        (
            2434,
            [1105, 1074],
            202384172,
            229013299,
            27328,
            114688,
            2184,
        ),
        (2258, [1066, 994], 201871721, 228798127, 24384, 114688, 3666),
        (
            2425,
            [1051, 1124],
            201387646,
            228534605,
            27712,
            114688,
            4483,
        ),
        (2179, [1011, 953], 200914490, 228364797, 24000, 114688, 0),
        (
            2318,
            [1036, 1037],
            200449709,
            228083637,
            24960,
            114688,
            8966,
        ),
        (
            2302,
            [1066, 1003],
            199963385,
            227894845,
            25664,
            114688,
            3284,
        ),
        (2212, [1044, 946], 199429909, 227689854, 23424, 114688, 7665),
        (2170, [1002, 959], 198906485, 227509818, 22272, 114688, 1878),
        (2182, [1003, 958], 198411019, 227340644, 24256, 114688, 0),
        (2146, [979, 970], 197909595, 227186565, 22976, 114688, 7880),
        (
            2308,
            [1060, 1011],
            197434974,
            226971255,
            24640,
            114688,
            2754,
        ),
        (
            2278,
            [1071, 984],
            196951252,
            226725904,
            23680,
            114688,
            16963,
        ),
        (2287, [1004, 1029], 196471084, 226521063, 25472, 114688, 0),
        (
            2296,
            [1002, 1043],
            195996948,
            226276661,
            24768,
            114688,
            10937,
        ),
        (
            2272,
            [1047, 991],
            195520803,
            226139285,
            24768,
            114688,
            10562,
        ),
        (2170, [1005, 964], 195038724, 225961883, 23104, 114688, 0),
        (
            2235,
            [1015, 1004],
            194538450,
            225783990,
            23104,
            114688,
            3315,
        ),
        (
            2177,
            [1011, 952],
            194062819,
            225632134,
            23616,
            114688,
            21198,
        ),
        (2337, [1058, 1067], 193508354, 225425960, 25408, 114688, 0),
        (
            2184,
            [1003, 971],
            193007964,
            225306353,
            23552,
            114688,
            26251,
        ),
        (
            2322,
            [1061, 1009],
            192501639,
            225116804,
            25024,
            114688,
            13699,
        ),
        (2160, [959, 977], 192036815, 224981683, 23296, 114688, 0),
        (2322, [1036, 996], 191524728, 224803168, 25600, 114688, 0),
        (2226, [1005, 999], 190996372, 224638213, 24192, 114688, 6352),
        (
            2298,
            [1009, 1044],
            190524522,
            224460111,
            25152,
            114688,
            29567,
        ),
        (2156, [972, 996], 190026675, 224347762, 23040, 114688, 0),
        (
            2387,
            [1115, 1017],
            189485926,
            224144929,
            26624,
            114688,
            14870,
        ),
        (
            2301,
            [1070, 1041],
            188968391,
            223943378,
            25664,
            114688,
            6017,
        ),
        (
            2266,
            [1079, 950],
            188473664,
            223785560,
            25152,
            114688,
            16091,
        ),
        (2245, [1008, 1003], 187989918, 223654378, 24960, 114688, 0),
        (2331, [1092, 1030], 187455344, 223498782, 25344, 114688, 0),
        (2143, [934, 985], 186924546, 223358080, 23360, 114688, 12832),
        (2213, [1048, 949], 186382434, 223204338, 23552, 114688, 0),
        (2221, [1014, 988], 185874694, 223065266, 24256, 114688, 0),
        (2132, [942, 979], 185383311, 222930288, 21312, 114688, 3869),
        (
            2295,
            [1040, 1023],
            184847230,
            222768360,
            24576,
            114688,
            13702,
        ),
        (2269, [1066, 983], 184318047, 222600675, 25088, 114688, 0),
        (
            2274,
            [1019, 1020],
            183803826,
            222441039,
            25088,
            114688,
            8503,
        ),
        (2239, [982, 1060], 183265633, 222297784, 24064, 114688, 6517),
        (2268, [1072, 983], 182749471, 222175583, 23936, 114688, 0),
        (2175, [970, 962], 182295585, 222063480, 23936, 114688, 18948),
        (2265, [1038, 1001], 181805389, 221900759, 24064, 114688, 0),
        (2252, [981, 1074], 181274039, 221752910, 23872, 114688, 9820),
        (2341, [1086, 1028], 180720644, 221623015, 25792, 114688, 0),
        (
            2299,
            [997, 1039],
            180214418,
            221440596,
            24512,
            114688,
            12643,
        ),
        (
            2273,
            [1045, 1005],
            179699585,
            221307497,
            24512,
            114688,
            13074,
        ),
        (2146, [944, 990], 179221133, 221198135, 23488, 114688, 0),
        (
            2254,
            [1039, 1004],
            178765721,
            221061424,
            24000,
            114688,
            5897,
        ),
        (
            2353,
            [1114, 1023],
            178258430,
            220881300,
            25408,
            114688,
            10902,
        ),
        (2245, [1024, 994], 177714283, 220767515, 24896, 114688, 0),
        (2324, [1043, 1051], 177171735, 220621883, 25280, 114688, 0),
        (2171, [971, 983], 176650812, 220507957, 22336, 114688, 14958),
        (
            2236,
            [1012, 1008],
            176161823,
            220394857,
            23488,
            114688,
            24187,
        ),
        (2340, [1082, 1023], 175687942, 220244766, 24960, 114688, 0),
        (
            2268,
            [1024, 1019],
            175149296,
            220147269,
            24384,
            114688,
            2861,
        ),
        (2256, [987, 1033], 174652364, 220019391, 24640, 114688, 6780),
        (2350, [1031, 1092], 174132530, 219887364, 26752, 114688, 0),
        (2142, [972, 958], 173657844, 219801483, 22720, 114688, 0),
        (2316, [1090, 988], 173132774, 219687064, 25728, 114688, 2299),
        (2246, [995, 1034], 172568625, 219579664, 24512, 114688, 2931),
        (2191, [1003, 954], 172043426, 219442039, 22592, 114688, 1070),
        (2182, [995, 984], 171499818, 219362408, 23232, 114688, 0),
        (
            2327,
            [1049, 1046],
            171027986,
            219230487,
            26304,
            114688,
            1971,
        ),
        (2143, [989, 955], 170528888, 219152474, 23360, 114688, 17859),
        (
            2154,
            [1001, 963],
            170018373,
            219058704,
            23424,
            114688,
            14469,
        ),
        (2322, [1102, 982], 169495824, 218925984, 24896, 114688, 0),
        (2203, [1003, 976], 168963177, 218845890, 23872, 114688, 0),
        (2222, [1018, 973], 168441628, 218734468, 25216, 114688, 0),
        (
            2200,
            [1015, 981],
            167898617,
            218650268,
            22400,
            114688,
            11728,
        ),
        (2314, [1068, 1036], 167365910, 218578023, 25152, 114688, 0),
        (
            2282,
            [992, 1060],
            166885026,
            218451597,
            23872,
            114688,
            18953,
        ),
        (2345, [1047, 1071], 166364950, 218357692, 25856, 114688, 0),
        (
            2308,
            [1013, 1063],
            165876154,
            218243354,
            25792,
            114688,
            19823,
        ),
    ],
    &[
        (
            2446,
            [1085, 1121],
            213359105,
            235275227,
            27968,
            124915,
            18782,
        ),
        (
            7918,
            [3537, 3585],
            212541010,
            231406417,
            59328,
            134144,
            26802,
        ),
        (
            13799,
            [6186, 6233],
            212528054,
            221515784,
            29056,
            145230,
            22209,
        ),
        (
            22710,
            [10131, 10283],
            213492369,
            200140518,
            3136,
            156301,
            25570,
        ),
        (
            31858,
            [14333, 14267],
            213860116,
            170962350,
            256,
            170792,
            16929,
        ),
        (
            44560,
            [20173, 19960],
            213901887,
            140387729,
            0,
            149443,
            1048576,
        ),
        (21427, [9694, 9608], 213883155, 134890027, 3264, 168123, 0),
        (
            39315,
            [17836, 17507],
            213901376,
            123374728,
            0,
            147108,
            1048576,
        ),
        (18653, [8304, 8462], 213851779, 121423030, 6912, 165497, 0),
        (
            35730,
            [16010, 16125],
            213893808,
            116130444,
            0,
            144810,
            1048576,
        ),
        (
            16385,
            [7399, 7409],
            213815082,
            115480001,
            15552,
            162684,
            822,
        ),
        (32277, [14544, 14484], 213887766, 112918474, 64, 183020, 0),
        (
            53795,
            [24254, 24082],
            213902976,
            109806549,
            0,
            160143,
            1048576,
        ),
        (29537, [13384, 13160], 213902485, 109065895, 128, 180161, 0),
        (
            50278,
            [22542, 22637],
            213902826,
            107997709,
            0,
            157641,
            1048576,
        ),
        (26891, [12164, 12030], 213900308, 107724148, 704, 177346, 0),
        (
            47253,
            [21447, 21111],
            213902800,
            107179569,
            0,
            155178,
            1048576,
        ),
        (24621, [11101, 11027], 213896026, 107237752, 1536, 174575, 0),
        (
            44229,
            [19956, 19844],
            213902151,
            106894660,
            0,
            152753,
            1048576,
        ),
        (22438, [10035, 10096], 213884837, 107042522, 1920, 171847, 0),
        (
            41132,
            [18546, 18363],
            213899497,
            106446783,
            0,
            150366,
            1048576,
        ),
        (20553, [9288, 9238], 213863194, 106843122, 5120, 169162, 0),
        (
            38212,
            [17342, 17062],
            213896154,
            106317651,
            64,
            148017,
            1048576,
        ),
        (
            18703,
            [8384, 8388],
            213848562,
            106610509,
            7488,
            165982,
            1907,
        ),
        (
            35176,
            [15817, 15869],
            213892917,
            106373443,
            0,
            145234,
            1048576,
        ),
        (16501, [7513, 7326], 213791779, 106698857, 15104, 163388, 0),
        (32224, [14521, 14484], 213879964, 106417189, 192, 183812, 0),
        (
            54398,
            [24474, 24402],
            213902976,
            107134111,
            0,
            160836,
            1048576,
        ),
        (30113, [13617, 13510], 213901363, 106880282, 256, 180941, 0),
        (
            51196,
            [23101, 23003],
            213902976,
            106750721,
            0,
            158323,
            1048576,
        ),
        (
            27393,
            [12471, 12243],
            213895007,
            106847917,
            512,
            174939,
            10510,
        ),
        (
            44269,
            [19933, 19985],
            213902110,
            106686159,
            0,
            153072,
            1048576,
        ),
        (
            22722,
            [10233, 10226],
            213893833,
            106831478,
            1472,
            170634,
            5387,
        ),
        (
            39885,
            [18003, 17862],
            213902676,
            106628922,
            0,
            149305,
            1048576,
        ),
        (
            19757,
            [8951, 8831],
            213866726,
            106938202,
            4672,
            161976,
            21036,
        ),
    ],
];

/// The quiet runs: the ticks each took and the sums by polarity after it.
pub(crate) const QUIET_1024: [(u64, (i64, i64)); 2] = [
    (2500, (165876268, 218243354)),
    (2541, (213866726, 106938185)),
];

/// The candidates' frozen runs, each of sixty-four trials with ADR-0076's stimulus on the
/// frozen engine: the sight's block and trace, and the composition. Pinned from one run each.
pub(crate) const BACKGROUND_1024: [(Block, u64, Composition); 2] = [
    (
        (
            29,
            34,
            [[330, 323], [315, 314]],
            [1728, 1525],
            [2491, 2252],
            [258, 256],
            62,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        0x60f6be387fbce9cd,
        (
            [[581, 11652], [6687, 10983]],
            [
                [
                    [214620, -5235, 318258, -506972],
                    [239131, -5599, 344531, -493105],
                ],
                [
                    [227460, -5372, 281672, -462205],
                    [208207, -4723, 325571, -442474],
                ],
            ],
            [[1100, 1646], [1009, 1645], [3350, 0]],
            53,
            5817133415553407245,
        ),
    ),
    (
        (
            33,
            34,
            [[2350, 2341], [1962, 1984]],
            [1646, 1458],
            [14811, 13425],
            [4034, 3942],
            51,
            213866726,
            106938185,
            0,
            [[3149340, 3346376], [3184652, 3239633]],
            0,
        ),
        0x4f57a261e13951af,
        (
            [[12794, -3720], [16587, 4124]],
            [
                [
                    [642005, -263256, 13907673, -14109941],
                    [673222, -274308, 14247814, -14654024],
                ],
                [
                    [527047, -194836, 12652268, -12809414],
                    [522192, -192938, 12744444, -12996071],
                ],
            ],
            [[16366, 16789], [16287, 16797], [4774, 14]],
            47,
            7693778306876544192,
        ),
    ),
];

/// The candidates' rows, trial by trial.
pub(crate) const BACKGROUND_ROWS_1024: [[Row; BLOCK]; 2] = [
    [
        (0, 7598, [1697, -8, 23569, -17210]),
        (0, -2383, [12050, -769, 17987, -36979]),
        (1, -6090, [2763, -39, 14205, -21100]),
        (0, -2456, [10353, -667, 16060, -25683]),
        (0, -454, [10021, -441, 12026, -20435]),
        (0, -7759, [10709, -621, 11043, -28055]),
        (0, -13372, [16659, -179, 9072, -32647]),
        (0, -4528, [14866, -125, 15795, -25648]),
        (0, 7348, [14792, -446, 17526, -19882]),
        (0, 18601, [14335, -633, 13605, -14830]),
        (1, -16059, [5376, -374, 12930, -25513]),
        (1, 3652, [17725, -90, 16035, -17233]),
        (0, 838, [3032, -45, 16627, -21107]),
        (0, 11626, [15775, -208, 12297, -16731]),
        (1, 14348, [14753, -145, 21871, -22674]),
        (0, 13840, [12707, -2, 14192, -21079]),
        (1, 14547, [9785, -191, 15872, -23401]),
        (0, 12148, [5710, -1, 19740, -27635]),
        (0, 21765, [16159, -442, 17086, -19234]),
        (0, 18891, [14052, -129, 12440, -24473]),
        (1, 1630, [4067, -329, 14056, -18700]),
        (0, 21586, [12416, -188, 17817, -17128]),
        (1, 1870, [12471, -203, 15991, -25751]),
        (1, 6831, [13141, -328, 17304, -25694]),
        (1, 8685, [14022, -37, 14031, -25093]),
        (0, -1825, [9281, -847, 16360, -24261]),
        (1, 10727, [10223, -119, 18597, -19081]),
        (1, 9842, [17057, -344, 17005, -31877]),
        (1, 23169, [14179, -230, 18195, -18838]),
        (0, 2036, [4048, -70, 19625, -18346]),
        (0, 7338, [20601, -262, 15013, -30099]),
        (0, 17078, [16564, -150, 12818, -16691]),
        (0, 29797, [21735, -945, 16592, -20087]),
        (0, 26264, [13869, -439, 13450, -25618]),
        (1, 5460, [8616, -1488, 21987, -22567]),
        (1, 23470, [20368, -1033, 16905, -17795]),
        (1, 20850, [14229, -544, 8680, -19107]),
        (0, 8943, [4587, -258, 15659, -21148]),
        (1, 4248, [9720, -363, 11899, -26114]),
        (1, 14096, [23196, -234, 17524, -29648]),
        (0, 8751, [10497, -199, 17832, -18697]),
        (1, 17298, [14462, -134, 21218, -23447]),
        (0, 4308, [6965, -92, 12816, -21525]),
        (1, 17165, [10962, -90, 13347, -19118]),
        (0, 5683, [7132, -100, 15969, -20550]),
        (1, 12999, [7860, -311, 20594, -28997]),
        (0, 11964, [16034, -354, 16135, -21761]),
        (0, 20776, [15499, -147, 12502, -15772]),
        (0, 23776, [15176, -100, 19430, -27498]),
        (1, 10871, [8451, -308, 16730, -18977]),
        (1, 22312, [17288, -19, 16691, -19084]),
        (0, 10455, [5341, -300, 13620, -15470]),
        (1, 11288, [7590, -403, 12849, -24619]),
        (1, 15428, [22215, -454, 13109, -28758]),
        (0, -4526, [7711, -228, 20506, -35729]),
        (1, 12125, [11515, -157, 18751, -25445]),
        (1, 19383, [20084, -988, 12989, -21716]),
        (1, 24423, [14030, -65, 15041, -19726]),
        (0, -14641, [6796, -684, 19670, -30921]),
        (1, 21547, [10958, -244, 14464, -17882]),
        (0, 4513, [14280, -747, 24128, -21376]),
        (0, 10174, [15309, -8, 13316, -22090]),
        (1, 16525, [3891, -34, 18755, -16728]),
        (1, 17670, [18524, -796, 10017, -22968]),
    ],
    [
        (0, 44457, [41310, -12664, 303198, -280213]),
        (0, 4856, [32926, -11724, 281258, -329500]),
        (1, 28762, [35654, -11312, 318589, -318262]),
        (0, 13555, [39475, -11681, 351155, -362635]),
        (0, -9970, [25102, -9615, 319931, -359727]),
        (0, 3145, [33947, -9526, 333761, -341822]),
        (0, 11807, [35164, -15605, 325208, -335736]),
        (0, 28690, [42639, -14684, 418734, -424101]),
        (0, -438, [37420, -20221, 655938, -695581]),
        (0, -11255, [47019, -28156, 674473, -702759]),
        (1, 22202, [46168, -23536, 596325, -606274]),
        (1, 3014, [43350, -26442, 559120, -588673]),
        (0, -27063, [48245, -25345, 683938, -701975]),
        (0, -19827, [41384, -21174, 679390, -697402]),
        (1, -14134, [34868, -18245, 611865, -651672]),
        (0, 6947, [43667, -20612, 520837, -547932]),
        (1, 15531, [33184, -9974, 268441, -267920]),
        (0, 22988, [42840, -13990, 276868, -283072]),
        (0, 24776, [34474, -12330, 253419, -267297]),
        (0, 41112, [40180, -10437, 291305, -294435]),
        (1, 69577, [34223, -11560, 280732, -261934]),
        (0, 30121, [36344, -10032, 250125, -273647]),
        (1, 39310, [33433, -9469, 254400, -270780]),
        (1, 28220, [29308, -8914, 269061, -290683]),
        (1, 2837, [34984, -13493, 486518, -530919]),
        (0, 1050, [43272, -22792, 633249, -661129]),
        (1, 10898, [38740, -21475, 595824, -595859]),
        (1, 8950, [32588, -14441, 573223, -589113]),
        (1, 652, [33570, -15444, 554967, -575704]),
        (0, -5436, [33746, -14998, 611306, -654101]),
        (0, -14314, [46019, -24284, 614972, -646385]),
        (0, -15552, [41772, -22818, 604617, -629006]),
        (0, 36457, [28813, -4813, 249217, -222771]),
        (0, 29976, [32159, -6668, 201484, -224853]),
        (1, 16888, [34335, -9526, 207884, -215842]),
        (1, 11741, [23758, -4975, 185856, -210694]),
        (1, 21128, [35206, -7000, 200362, -213759]),
        (0, 19544, [30941, -8594, 249339, -266035]),
        (1, 29402, [39215, -12439, 241432, -237906]),
        (1, 18073, [24888, -5827, 225371, -249617]),
        (0, 12791, [35286, -17530, 513150, -532752]),
        (1, 10148, [45838, -23111, 530044, -559041]),
        (0, -18330, [37182, -20633, 524820, -548585]),
        (1, 31835, [46327, -19151, 548826, -549900]),
        (0, -26268, [44939, -19644, 624392, -658698]),
        (1, -3075, [37748, -17449, 516165, -549997]),
        (0, -17063, [42285, -19616, 546826, -571892]),
        (0, -25142, [42269, -21205, 438371, -472470]),
        (0, 19395, [33662, -6920, 238011, -222311]),
        (1, 3963, [28956, -6689, 147362, -168935]),
        (1, 21213, [30048, -5625, 199133, -203399]),
        (0, 39283, [35838, -10442, 243176, -242211]),
        (1, 30672, [26789, -5443, 186459, -197518]),
        (1, 41903, [30350, -5713, 179932, -184886]),
        (0, 46473, [36898, -11455, 198187, -217338]),
        (1, 33953, [31243, -9343, 226559, -242897]),
        (1, -9209, [36680, -14170, 414419, -475550]),
        (1, 25249, [34009, -14686, 466932, -448246]),
        (0, 4367, [43365, -17540, 461881, -480280]),
        (1, 24932, [37499, -15973, 413099, -400065]),
        (0, -19862, [36437, -15989, 475830, -523456]),
        (0, 1475, [44992, -23826, 526682, -525175]),
        (1, -2006, [29289, -9188, 448902, -488990]),
        (1, 20711, [44266, -17158, 411226, -413452]),
    ],
];

/// Every trial's readout counts under each candidate.
pub(crate) const BACKGROUND_COUNTED_1024: [[Counted; BLOCK]; 2] = [
    [
        (0, [3, 3]),
        (0, [13, 12]),
        (1, [10, 10]),
        (0, [1, 10]),
        (0, [6, 9]),
        (0, [14, 7]),
        (0, [16, 15]),
        (0, [15, 10]),
        (0, [9, 11]),
        (0, [5, 8]),
        (1, [16, 14]),
        (1, [12, 14]),
        (0, [6, 7]),
        (0, [8, 11]),
        (1, [16, 19]),
        (0, [5, 7]),
        (1, [11, 12]),
        (0, [10, 8]),
        (0, [12, 8]),
        (0, [8, 12]),
        (1, [9, 12]),
        (0, [7, 15]),
        (1, [10, 15]),
        (1, [7, 8]),
        (1, [5, 5]),
        (0, [9, 9]),
        (1, [7, 9]),
        (1, [14, 10]),
        (1, [12, 11]),
        (0, [12, 13]),
        (0, [9, 10]),
        (0, [10, 15]),
        (0, [9, 9]),
        (0, [9, 13]),
        (1, [19, 10]),
        (1, [8, 5]),
        (1, [8, 7]),
        (0, [9, 9]),
        (1, [13, 18]),
        (1, [24, 13]),
        (0, [9, 6]),
        (1, [5, 9]),
        (0, [8, 3]),
        (1, [6, 16]),
        (0, [13, 11]),
        (1, [12, 5]),
        (0, [17, 15]),
        (0, [7, 9]),
        (0, [11, 8]),
        (1, [7, 16]),
        (1, [15, 5]),
        (0, [13, 4]),
        (1, [8, 9]),
        (1, [4, 6]),
        (0, [17, 11]),
        (1, [11, 8]),
        (1, [11, 11]),
        (1, [9, 15]),
        (0, [12, 6]),
        (1, [9, 7]),
        (0, [9, 10]),
        (0, [9, 9]),
        (1, [16, 8]),
        (1, [1, 7]),
    ],
    [
        (0, [45, 36]),
        (0, [60, 58]),
        (1, [59, 61]),
        (0, [57, 68]),
        (0, [59, 47]),
        (0, [66, 51]),
        (0, [69, 55]),
        (0, [56, 60]),
        (0, [77, 82]),
        (0, [86, 91]),
        (1, [100, 83]),
        (1, [106, 105]),
        (0, [104, 97]),
        (0, [94, 91]),
        (1, [88, 101]),
        (0, [87, 107]),
        (1, [43, 42]),
        (0, [53, 55]),
        (0, [55, 62]),
        (0, [49, 53]),
        (1, [51, 43]),
        (0, [55, 51]),
        (1, [56, 68]),
        (1, [43, 42]),
        (1, [102, 80]),
        (0, [90, 79]),
        (1, [91, 81]),
        (1, [72, 80]),
        (1, [86, 82]),
        (0, [75, 82]),
        (0, [92, 101]),
        (0, [99, 81]),
        (0, [52, 51]),
        (0, [51, 49]),
        (1, [39, 53]),
        (1, [50, 41]),
        (1, [53, 43]),
        (0, [48, 52]),
        (1, [51, 36]),
        (1, [49, 54]),
        (0, [81, 76]),
        (1, [78, 75]),
        (0, [83, 88]),
        (1, [83, 86]),
        (0, [90, 92]),
        (1, [81, 88]),
        (0, [85, 83]),
        (0, [81, 78]),
        (0, [48, 45]),
        (1, [42, 58]),
        (1, [51, 40]),
        (0, [35, 56]),
        (1, [47, 45]),
        (1, [58, 32]),
        (0, [40, 41]),
        (1, [39, 51]),
        (1, [74, 76]),
        (1, [64, 94]),
        (0, [69, 81]),
        (1, [77, 82]),
        (0, [70, 65]),
        (0, [89, 77]),
        (1, [60, 83]),
        (1, [69, 79]),
    ],
];

/// The volley-tick census under each candidate.
pub(crate) const BACKGROUND_CENSUS_1024: [&[(u32, u64)]; 2] = [
    &[
        (1, 158),
        (2, 982),
        (3, 1578),
        (4, 497),
        (5, 31),
        (6, 6),
        (7, 1),
    ],
    &[
        (0, 2),
        (1, 1174),
        (2, 1571),
        (3, 330),
        (4, 24),
        (5, 1),
        (6, 1),
        (94, 1),
    ],
];

/// Where ADR-0055's criterion first held, per candidate: the settled network at the
/// ninety-sixth window — past the eightieth, so the lead-in ran on under the extended bound —
/// with the excitatory sum at 0.925 of the prior's; the controller at the thirty-fifth, at
/// 0.453 of the prior's, the fixed point ADR-0055's day read (0.452), the gain swinging
/// between 2.2 and 2.8 every window and the rate with it.
pub(crate) const SETTLED_AT_1024: [Option<usize>; 2] = [Some(96), Some(35)];

/// The three measures under each candidate, as read over the pinned tables: the stimulus
/// still fires once, the sight, the sign. The settled network fires once and sees (62 of 64)
/// and its sign reads 53 of 64, three short of the mark; the controller fires the volley
/// short (48.4 of 51 per presentation, the units its background had within their refractory
/// window at the injection), is not seen (51) and its sign reads 47.
pub(crate) const BACKGROUND_MEASURES_1024: [[bool; 3]; 2] =
    [[true, true, false], [false, false, false]];

/// The configuration the rules pick over the pinned tables: none. No candidate passes all
/// three measures, so there is no rewarded run and H-12's stopping rule reaches its step 3.
pub(crate) const BACKGROUND_PICKED_1024: Option<u16> = None;

/// The prediction for the controller, as read: every clause held — the gain 2.47 after the
/// lead-in and 2.31 after the run, never settled, the readout sets firing 8.6 times more in
/// the lead-in's last window than under the settled network, and all three measures failed.
pub(crate) const CONTROLLER_READ: [bool; 4] = [true; 4];

/// Deliverable D under each candidate: the offset the criterion needs over the frozen run's
/// sixty-four trials at 40, and the instrument's bias per stimulus, `(difference, trials)`.
pub(crate) const OFFSETS_1024: [Option<u32>; 2] = [Some(2), Some(3)];

pub(crate) const BIASES_1024: [[(i64, u32); 2]; 2] = [[(7, 34), (-1, 30)], [(9, 34), (22, 30)]];

// ------------------------------------------------ written before the run (brief 036)

/// H-13's network (ADR-0078): ADR-0077's settled candidate, `BACKGROUNDS[SETTLED]`, the gain
/// held at 1.75 and the controller off; its image built as `background_candidate` builds it
/// and held to ADR-0077's tables step by step before any rewarded run.
pub(crate) const SETTLED: usize = 0;

const _: () = assert!(BACKGROUNDS[SETTLED] == 0);

/// The arms of H-13, in the order run and no other: the reward withheld — the frozen run of
/// `TRIALS` trials from the image, whose first block is the calibration and whose every trial
/// is the reference a rewarded arm's trial is paired with — then the assignment (A onto
/// readout 0, B onto readout 1), then the mirrored assignment (A onto 1, B onto 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Arm {
    Withheld,
    Assignment,
    Mirrored,
}

pub(crate) const ARMS: [Arm; 3] = [Arm::Withheld, Arm::Assignment, Arm::Mirrored];

/// The rewarded arms in `ARMS`'s order, the criterion's `[assignment, mirrored]`.
pub(crate) const REWARDED_ARMS: [Arm; 2] = [Arm::Assignment, Arm::Mirrored];

/// ADR-0078's prediction for the couplings clause, a Hypothesis written before the run: it
/// holds in both rewarded arms, since ADR-0077's trace is positive after 53 trials of 64 and
/// every pair's sum after its frozen block is positive. No prediction is written for the
/// response clause.
pub(crate) const COUPLINGS_PREDICTED: [bool; 2] = [true, true];

/// The dopamine signal under a reward of `REWARD_Q16` after every trial, by the two rules
/// alone (`NeuromodulatorState::reward`, saturating, and `decay_dopamine` by
/// `DOPAMINE_TAU_SHIFT` once per tick), computed apart from the tree before the run and held
/// here to the oracle below, which every run holds to the record: after the first reward 1.0
/// and at the first trial's end 0.458; the per-trial map's fixed point 1.712 after a reward and
/// 0.712 at a trial's end, reached within eleven trials; from it the addressed modulation is at
/// the ceiling for 9 694 ticks of the trial's 16 384 and never below 0.712. ADR-0078's
/// arithmetic from a decay of exactly $2^{-14}$ per tick said about 1.58 and 0.58: the rule's
/// step is the floor of that fraction, so below every power of two the signal decays more
/// slowly than the exponent, and the reading is the record's.
pub(crate) const SIGNAL_END_FIRST_Q16: i32 = 30_036;

pub(crate) const SIGNAL_AFTER_FIXED_Q16: i32 = 112_227;

pub(crate) const SIGNAL_END_FIXED_Q16: i32 = 46_691;

pub(crate) const SIGNAL_FIXED_WITHIN_TRIALS: usize = 11;

pub(crate) const SIGNAL_CEILING_TICKS_FIXED: u32 = 9_694;

const _: () = assert!(SIGNAL_AFTER_FIXED_Q16 == SIGNAL_END_FIXED_Q16 + ONE);

const _: () = assert!(SIGNAL_FIXED_WITHIN_TRIALS < BLOCK);

// ------------------------------------------------------------- the oracle (brief 036)

/// The dopamine signal one tick on, as the executor decays it after publishing each tick's
/// modulations (`decay_dopamine` with `DOPAMINE_TAU_SHIFT`, ADR-0032): toward zero by the
/// floor of $2^{-14}$ of itself and by at least one LSB, so that it reaches rest exactly;
/// written a second time as the oracle's.
pub(crate) fn decayed_signal(signal: i32) -> i32 {
    let d = i64::from(signal);
    let step = (d.abs() >> DOPAMINE_TAU_SHIFT).max(1).min(d.abs());
    d.saturating_sub(d.signum().saturating_mul(step)) as i32
}

/// The signal's course over one trial from `first`, the signal at the trial's first tick:
/// entry `k` is the signal the trial's `k`-th tick publishes its modulation from, and entry
/// `TRIAL_TICKS` the signal at the trial's end, `TRIAL_TICKS` decays on.
pub(crate) fn signal_course(first: i32) -> Vec<i32> {
    let mut course = Vec::with_capacity(TRIAL_TICKS as usize);
    let mut signal = first;
    for _ in 0..TRIAL_TICKS {
        course.push(signal);
        signal = decayed_signal(signal);
    }
    course.push(signal);
    course
}

/// The signal at the trial's end, the course's last entry.
pub(crate) fn signal_end(course: &[i32]) -> i32 {
    course.last().copied().unwrap_or(0)
}

/// The ticks of a course at which the modulation is at the ceiling, the signal at or above
/// 1.0.
pub(crate) fn ceiling_ticks(course: &[i32]) -> u32 {
    course
        .iter()
        .take(TRIAL_TICKS as usize)
        .filter(|&&s| s >= ONE)
        .count() as u32
}

/// The consolidation of one excitatory slot at a presynaptic spike, as `cortex-core`'s
/// `consolidate` moves it (ADR-0032, ADR-0049): `round(|trace| × m)` of the trace, signed as
/// the trace, into the magnitude, clamped to $[0, 2^{15})$, and what the magnitude absorbed
/// taken out of the trace, `m` clamped to $[0, 1]$. Returns the trace after, the magnitude
/// after and the amount absorbed; written a second time as the oracle's.
pub(crate) fn consolidated(trace: i16, magnitude: i32, modulation_q16: i32) -> (i16, i32, i32) {
    let m = i64::from(modulation_q16.clamp(0, ONE));
    let trace = i32::from(trace);
    let amount = (i64::from(trace)
        .abs()
        .saturating_mul(m)
        .saturating_add(0x8000)
        >> 16) as i32;
    let transfer = amount.saturating_mul(trace.signum());
    let before = magnitude.max(0);
    let after = before
        .saturating_add(transfer)
        .clamp(0, i32::from(i16::MAX));
    let absorbed = after.saturating_sub(before);
    (trace.saturating_sub(absorbed) as i16, after, absorbed)
}

/// The delivery as the oracle replays it (brief 036): the pair the last trial's delivery
/// addressed — the stimulus presented and its assigned readout under the taught delivery,
/// the readout the engine selected under the task's own (brief 037), none at a tie — and the
/// signal that delivery's reward left, which is the signal at the next trial's first tick;
/// before the first delivery none and zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Taught {
    pub(crate) addressed: Option<(usize, usize)>,
    pub(crate) signal: i32,
}

// ---------------------------------------------------------- the criterion (brief 036)

/// One trial's reading under an arm (brief 036): the stimulus presented; the readouts'
/// counts in the task's window, as the task read them; the selection; the trace consolidated
/// into the weights over the trial, `[stimulus][readout]`, by the oracle; and the dopamine
/// signal at the trial's end and after the delivery's reward (the same, in the withheld arm).
pub(crate) type TaughtTrial = (u8, [u32; 2], Option<u8>, [[i64; 2]; 2], i32, i32);

/// One block of an arm (brief 036): the paired tallies against the withheld arm over the
/// block — the trials in which the presented stimulus's assigned readout counted more than at
/// the same trial of the withheld arm, and the trials in which the other readout did — the
/// selections of the assigned readout, the ties, the trace consolidated into the weights over
/// the block, `[stimulus][readout]`, and the signal at the block's last trial's end and after
/// its reward.
pub(crate) type TaughtBlock = ([u32; 2], u32, u32, [[i64; 2]; 2], i32, i32);

/// Whether an arm rewards, and whether it mirrors the assignment.
pub(crate) fn rewards(arm: Arm) -> bool {
    arm != Arm::Withheld
}

pub(crate) fn mirrors(arm: Arm) -> bool {
    arm == Arm::Mirrored
}

/// The assigned pairs of an assignment, `(stimulus, readout)` for A and for B.
pub(crate) fn assigned_pairs(mirrored: bool) -> [(usize, usize); 2] {
    [(0, answer_of(0, mirrored)), (1, answer_of(1, mirrored))]
}

/// A readout set of the geometry by its index.
pub(crate) fn readout_set(sets: &[Set; 4], readout: usize) -> Set {
    if readout == 0 { sets[2] } else { sets[3] }
}

/// The paired comparison of a rewarded arm's trial with the withheld arm's (ADR-0078): for
/// the presented stimulus's assigned readout, then for the other readout, whether the arm's
/// count exceeds the withheld arm's at the same trial. A tie is not more.
pub(crate) fn paired(rewarded: &TaughtTrial, withheld: &TaughtTrial, mirrored: bool) -> [bool; 2] {
    assert_eq!(
        rewarded.0, withheld.0,
        "the same trial presents the same stimulus"
    );
    let assigned = answer_of(rewarded.0, mirrored);
    let other = assigned ^ 1;
    [
        rewarded.1[assigned] > withheld.1[assigned],
        rewarded.1[other] > withheld.1[other],
    ]
}

/// An arm's blocks from its trials and the withheld arm's, `BLOCK` trials each: the paired
/// tallies, the selections of the assigned readout, the ties, the consolidation summed and
/// the last trial's signals. The withheld arm against itself tallies nothing, since no count
/// exceeds itself.
pub(crate) fn taught_blocks(
    read: &[TaughtTrial],
    withheld: &[TaughtTrial],
    mirrored: bool,
) -> Vec<TaughtBlock> {
    assert_eq!(read.len(), withheld.len(), "the arms run the same trials");
    read.chunks(BLOCK)
        .zip(withheld.chunks(BLOCK))
        .map(|(mine, theirs)| {
            let mut tallies = [0u32; 2];
            let mut selections = 0u32;
            let mut ties = 0u32;
            let mut transferred = [[0i64; 2]; 2];
            for (r, w) in mine.iter().zip(theirs.iter()) {
                let more = paired(r, w, mirrored);
                tallies[0] = tallies[0].saturating_add(u32::from(more[0]));
                tallies[1] = tallies[1].saturating_add(u32::from(more[1]));
                let assigned = answer_of(r.0, mirrored) as u8;
                selections = selections.saturating_add(u32::from(r.2 == Some(assigned)));
                ties = ties.saturating_add(u32::from(r.2.is_none()));
                for (s, readouts) in r.3.iter().enumerate() {
                    for (k, &amount) in readouts.iter().enumerate() {
                        transferred[s][k] = transferred[s][k].saturating_add(amount);
                    }
                }
            }
            let last = mine.last().expect("a block holds a trial");
            (tallies, selections, ties, transferred, last.4, last.5)
        })
        .collect()
}

/// The response clause's count: the assigned readout's paired tally over the last
/// `LAST_BLOCKS` blocks.
pub(crate) fn last_paired(blocks: &[TaughtBlock]) -> u32 {
    blocks
        .iter()
        .rev()
        .take(LAST_BLOCKS)
        .fold(0u32, |sum, b| sum.saturating_add(b.0[0]))
}

/// The selections of the assigned readout over the last `LAST_BLOCKS` blocks, a reading
/// beside the response clause and no clause.
pub(crate) fn last_selected(blocks: &[TaughtBlock]) -> u32 {
    blocks
        .iter()
        .rev()
        .take(LAST_BLOCKS)
        .fold(0u32, |sum, b| sum.saturating_add(b.1))
}

/// The couplings clause: each assigned pair's excitatory coupling sum after the arm's last
/// block above the image's, strict; false for an arm of no block.
pub(crate) fn couplings_rose(image: &[[i64; 2]; 2], blocks: &[Block], mirrored: bool) -> bool {
    blocks.last().is_some_and(|last| {
        assigned_pairs(mirrored)
            .iter()
            .all(|&(s, r)| last.10[s][r] > image[s][r])
    })
}

/// H-13's criterion (ADR-0078), clause by clause in each rewarded arm, `[assignment,
/// mirrored]`: the couplings (`couplings_rose`) and the response (`last_paired` at least
/// `REWARDED_MIN`). `yes` is all four.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Direction {
    pub(crate) couplings: [bool; 2],
    pub(crate) response: [bool; 2],
    pub(crate) yes: bool,
}

pub(crate) fn direction(image: &[[i64; 2]; 2], arms: [(&[Block], &[TaughtBlock]); 2]) -> Direction {
    let couplings = [
        couplings_rose(image, arms[0].0, mirrors(REWARDED_ARMS[0])),
        couplings_rose(image, arms[1].0, mirrors(REWARDED_ARMS[1])),
    ];
    let response = [
        last_paired(arms[0].1) >= REWARDED_MIN,
        last_paired(arms[1].1) >= REWARDED_MIN,
    ];
    Direction {
        couplings,
        response,
        yes: couplings[0] && couplings[1] && response[0] && response[1],
    }
}

// ---------------------------------------------------------------- the run (brief 036)

/// An arm's run: the blocks, the accuracy-sequence trace, the composed trials, every trial's
/// reading, the volley-tick census over the run, and the first block's trace and census
/// (the calibration's, held to ADR-0077's).
pub(crate) type TaughtRun = (
    Vec<Block>,
    u64,
    Vec<Composed>,
    Vec<TaughtTrial>,
    Vec<u64>,
    (u64, Vec<u64>),
);

/// The word `run_on` hashes for a trial, `(stimulus, selection, correct)`, written a second
/// time so that a block of a longer run can be held to the trace of a run of that block.
pub(crate) fn sequence_word(outcome: &Outcome) -> i32 {
    i32::from(outcome.stimulus)
        | i32::from(outcome.selection.map_or(3, |r| r)) << 1
        | i32::from(outcome.correct) << 3
}

/// The FNV-1a hash of every trial's reading, each number as its `i32` words.
pub(crate) fn taught_hash(read: &[TaughtTrial]) -> u64 {
    let mut words: Vec<i32> = Vec::new();
    let mut wide = |x: i64| {
        words.push(x as i32);
        words.push((x >> 32) as i32);
    };
    for t in read {
        wide(i64::from(t.0));
        wide(i64::from(t.1[0]));
        wide(i64::from(t.1[1]));
        wide(t.2.map_or(3, i64::from));
        for readouts in &t.3 {
            for &amount in readouts {
                wide(amount);
            }
        }
        wide(i64::from(t.4));
        wide(i64::from(t.5));
    }
    fnv1a_64(&words)
}

/// An arm's run from the frozen engine (brief 036): `trials` trials of the calibration's task
/// with ADR-0076's stimulus, the addressed delivery and the reward withheld by the task, the
/// composer seeded from the record and replaying the consolidation; after every trial of a
/// rewarded arm the taught delivery — the addressed set rewritten, between the trial's last
/// tick and the reward, to the synapses from the presented stimulus's units onto its assigned
/// readout's units, whatever the selection, then `REWARD_Q16` into the signal — and after
/// every trial of the withheld arm nothing, the task's own addressing standing with the signal
/// at rest. At every trial the record's traces and weights are held to the oracle and the
/// signal at the trial's end to its course; after every block of the withheld arm the sums by
/// polarity are asserted unchanged.
pub(crate) fn taught_run(exec: &mut Engine, arm: Arm, units: u32, trials: usize) -> TaughtRun {
    assert_eq!(
        units, 1024,
        "the cancel and the synapse counts below are pinned at 1 024 units"
    );
    assert_eq!(
        exec.modulation_baseline_q16(),
        0,
        "the baseline is the image's zero"
    );
    assert_eq!(
        exec.modulator().dopamine_rpe,
        0,
        "the signal is at rest before the first trial"
    );
    let sums = weights_by_polarity(exec);
    let sets = geometry(units, rotation(units));
    let mut composer = Composer::new(units);
    composer.enumerate(exec);
    composer.cursor = exec.ticks() as u32;
    if rewards(arm) {
        composer.taught = Some(Taught {
            addressed: None,
            signal: 0,
        });
    }
    let mirrored = mirrors(arm);
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    let task = task(
        SHAPE_F46,
        Some(cancel_of(picked)),
        units,
        Feedback::Withheld,
        mirrored,
        Delivery::Addressed,
    );
    let mut read: Vec<TaughtTrial> = Vec::with_capacity(trials);
    let mut sequence: Vec<i32> = Vec::with_capacity(trials);
    let mut first_block: (u64, Vec<u64>) = (0, Vec::new());
    let (blocks, trace) = run_on(
        exec,
        task,
        units,
        trials,
        &mut |exec, trial, start, outcome| {
            composer.observe(exec, trial, start, outcome);
            assert_eq!(
                outcome.selection,
                selected(outcome.counts),
                "trial {trial}: the selection is the sign of the count difference"
            );
            let end = exec.modulator().dopamine_rpe;
            assert_eq!(
                outcome.signal_q16, end,
                "trial {trial}: the task read the signal at the trial's end"
            );
            let transferred = composer.transferred.last().copied().unwrap_or([[0; 2]; 2]);
            let after = if rewards(arm) {
                let stimulus = usize::from(outcome.stimulus);
                let assigned = answer_of(outcome.stimulus, mirrored);
                let targets = readout_set(&sets, assigned);
                exec.address(sets[stimulus].units(), targets.units())
                    .expect("the sets are inside the arena");
                assert_eq!(
                    exec.addressed_counts(),
                    (sets[stimulus].len() as usize, targets.len() as usize),
                    "trial {trial}: the addressed set is the presented stimulus onto its assigned readout"
                );
                let after = exec.reward(REWARD_Q16);
                composer.taught = Some(Taught {
                    addressed: Some((stimulus, assigned)),
                    signal: after,
                });
                after
            } else {
                assert_eq!(
                    end, 0,
                    "trial {trial}: the withheld arm's signal stays at rest"
                );
                end
            };
            sequence.push(sequence_word(outcome));
            read.push((
                outcome.stimulus,
                outcome.counts,
                outcome.selection,
                transferred,
                end,
                after,
            ));
            if sequence.len() == BLOCK {
                first_block = (fnv1a_64(&sequence), composer.volley_ticks.clone());
            }
        },
    );
    assert_eq!(fnv1a_64(&sequence), trace, "the sequence is the run's");
    if !rewards(arm) {
        for block in &blocks {
            assert_eq!(
                (block.7, block.8),
                sums,
                "no weight moves with the reward withheld"
            );
        }
    }
    assert_eq!(composer.out.len(), trials);
    assert_eq!(composer.counts(), SYNAPSES_1024);
    (
        blocks,
        trace,
        composer.out,
        read,
        composer.volley_ticks,
        first_block,
    )
}

/// Every weight of the arena in the arena's order, block by block: what the reach assertion
/// compares bit for bit.
pub(crate) fn weights_of(exec: &Engine) -> Vec<Vec<i16>> {
    exec.blocks()
        .iter()
        .map(|b| b.weights_q1_15.to_vec())
        .collect()
}

/// The reach of a delivery over the arena (brief 036): the synapses whose weight differs
/// from `before`, counted inside `pairs` — a synapse from the stimulus set onto the readout
/// set a pair names — and outside them. Every occupied slot is walked once, through its
/// unit's chain.
pub(crate) fn reach(
    exec: &Engine,
    before: &[Vec<i16>],
    units: u32,
    pairs: &[(usize, usize)],
) -> (u64, u64) {
    let sets = geometry(units, rotation(units));
    let mut inside = 0u64;
    let mut outside = 0u64;
    for unit in exec.units() {
        let id = unit.id as u32;
        for s in unit.fan_out(exec.blocks()) {
            let was = before
                .get(s.block_idx as usize)
                .and_then(|b| b.get(usize::from(s.slot)))
                .copied();
            if was == Some(s.weight_q1_15) {
                continue;
            }
            let in_pair = pairs.iter().any(|&(stimulus, readout)| {
                sets[stimulus].contains(id) && readout_set(&sets, readout).contains(s.target)
            });
            if in_pair {
                inside = inside.saturating_add(1);
            } else {
                outside = outside.saturating_add(1);
            }
        }
    }
    (inside, outside)
}

/// The settled engine (brief 036; brief 038): ADR-0077's settled candidate built as
/// `background_candidate` builds it, each step held to ADR-0077's pinned tables — the lead-in
/// to its table and its length, the quiet run to its ticks and sums — and returned quiescent,
/// the state an image is written from, with the sums the quiet run left. A mismatch stops the
/// round here, before any rewarded run (H-13's stopping rule, step 2, and every rule after
/// it).
pub(crate) fn settled_engine(name: &str) -> (Engine, (i64, i64)) {
    let step = BACKGROUNDS[SETTLED];
    let mut exec = candidate(1024, step);
    assert_eq!(weights_by_polarity(&exec), PRIOR_SUMS_1024);
    let table = lead_in_until_settled(&mut exec, 1024, PRIOR_SUMS_1024.1, LEAD_IN_EXTENDED);
    let settled = settled_within(PRIOR_SUMS_1024.1, &lead_in_sums(&table));
    eprintln!(
        "DUMP {name} lead-in settled {settled:?} windows {} last {:?}",
        table.len(),
        table.last()
    );
    assert_eq!(
        table.as_slice(),
        BACKGROUND_LEAD_IN_1024[SETTLED],
        "{name}: the lead-in is ADR-0077's"
    );
    assert_eq!(
        settled, SETTLED_AT_1024[SETTLED],
        "{name}: the criterion holds where ADR-0077 read it"
    );
    let ticks = quiet(&mut exec);
    let quieted = weights_by_polarity(&exec);
    eprintln!("DUMP {name} quiet {ticks} sums {quieted:?}");
    assert_eq!(
        (ticks, quieted),
        QUIET_1024[SETTLED],
        "{name}: the quiet run is ADR-0077's"
    );
    (exec, quieted)
}

/// The settled engine's frozen image (brief 036): the modulator's baseline patched to zero,
/// decoded and asserted to carry the sums the quiet run left, the gain and the step; the one
/// image every arm of H-13 and H-14 decodes, and H-15's calibration.
pub(crate) fn frozen_image_checked(name: &str, exec: &Engine, quieted: (i64, i64)) -> Vec<u8> {
    let image = frozen_image(exec);
    let frozen = frozen_from(&image, 1024);
    assert_eq!(
        weights_by_polarity(&frozen),
        quieted,
        "{name}: the image carries the weights"
    );
    assert_eq!(
        frozen.homeostasis().synaptic_gain_q16,
        GAIN_1024,
        "{name}: and the gain"
    );
    assert_eq!(
        frozen.homeostasis().control_step_q0_16,
        BACKGROUNDS[SETTLED],
        "{name}: and the step"
    );
    image
}

/// The settled image (brief 036): the settled engine's frozen image, its bytes, the one image
/// every arm of H-13 and H-14 decodes.
pub(crate) fn settled_image(name: &str) -> Vec<u8> {
    let (exec, quieted) = settled_engine(name);
    frozen_image_checked(name, &exec, quieted)
}

/// The calibration (brief 036): the withheld arm's first block held to ADR-0077's frozen run
/// of the settled candidate — the sight's block and trace, the rows and the composition (the
/// stimulus firing once, the sight 62, the sign 53 of 64), the counts and the volley's census
/// — before any rewarded run. The first block of a run of `TRIALS` trials from the image is
/// that run of `BLOCK` trials, the inputs being the same.
pub(crate) fn calibration_holds(name: &str, run: &TaughtRun) {
    let (blocks, _, trials, read, _, (first_trace, first_census)) = run;
    let (block, pin, composed) = &BACKGROUND_1024[SETTLED];
    assert_eq!(
        blocks.first(),
        Some(block),
        "{name}: the first block is ADR-0077's"
    );
    assert_eq!(
        *first_trace, *pin,
        "{name}: the first block's sequence is ADR-0077's"
    );
    let first = trials.get(..BLOCK).expect("a block of trials");
    pinned_composition(name, first, &BACKGROUND_ROWS_1024[SETTLED], Some(composed));
    let counted: Vec<Counted> = read.iter().take(BLOCK).map(|t| (t.0, t.1)).collect();
    assert_eq!(
        counted.as_slice(),
        &BACKGROUND_COUNTED_1024[SETTLED],
        "{name}: the counts are ADR-0077's"
    );
    assert_eq!(
        census_of(first_census),
        BACKGROUND_CENSUS_1024[SETTLED].to_vec(),
        "{name}: the volley's ticks are ADR-0077's"
    );
    let read_measures = [
        fires_once(1024, block, composed),
        calibrated(block),
        composed.3 >= SIGN_MIN,
    ];
    assert_eq!(
        read_measures, BACKGROUND_MEASURES_1024[SETTLED],
        "{name}: the three measures as ADR-0077 read them"
    );
}

/// Dumps an arm's run: the sight's blocks and trace, the rows, the composition per block, the
/// taught blocks, every trial's reading with its hash, and the census.
pub(crate) fn dump_direction(name: &str, run: &TaughtRun, taught: &[TaughtBlock]) {
    let (blocks, trace, trials, read, volley_ticks, _) = run;
    eprintln!(
        "DUMP {name} sight {blocks:?} trace {trace:#018x} curve {:?}",
        curve(blocks)
    );
    let rows: Vec<Row> = trials.iter().map(trial_row).collect();
    eprintln!("DUMP {name} rows {rows:?}");
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    eprintln!("DUMP {name} compositions {compositions:?}");
    eprintln!("DUMP {name} taught {taught:?}");
    eprintln!("DUMP {name} read {read:?} hash {:#018x}", taught_hash(read));
    eprintln!("DUMP {name} census {:?}", census_of(volley_ticks));
}

/// Holds an arm's run to its pinned tables: the blocks and the trace, the composition per
/// block, the taught blocks, the readings' hash and the census.
pub(crate) fn pinned_direction(name: &str, k: usize, run: &TaughtRun, taught: &[TaughtBlock]) {
    let (blocks, trace, trials, read, volley_ticks, _) = run;
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        DIRECTION_BLOCKS_1024[k],
        DIRECTION_TRACES_1024[k],
    );
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    assert_eq!(
        compositions.as_slice(),
        DIRECTION_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        taught, DIRECTION_TAUGHT_1024[k],
        "{name}: the taught blocks"
    );
    assert_eq!(
        taught_hash(read),
        DIRECTION_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        DIRECTION_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

// ----------------------------------------------------------- the measurement (brief 036)

/// The three arms at 1 024 units, in `ARMS`'s order, each pinned from one run: the sight's
/// blocks and trace, the composition per block, the taught blocks, the readings' hash and the
/// volley's census.
pub(crate) const DIRECTION_BLOCKS_1024: [&[Block]; 3] = [
    &[
        (
            29,
            34,
            [[330, 323], [315, 314]],
            [1728, 1525],
            [2491, 2252],
            [258, 256],
            62,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        (
            29,
            31,
            [[284, 309], [327, 332]],
            [1574, 1677],
            [2397, 2398],
            [242, 228],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            7,
        ),
        (
            25,
            31,
            [[287, 327], [335, 340]],
            [1570, 1678],
            [2384, 2380],
            [237, 226],
            63,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            5,
        ),
        (
            24,
            28,
            [[249, 336], [351, 377]],
            [1427, 1829],
            [2258, 2565],
            [247, 251],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            6,
        ),
        (
            29,
            30,
            [[283, 338], [326, 358]],
            [1524, 1724],
            [2332, 2390],
            [245, 255],
            63,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            8,
        ),
        (
            28,
            36,
            [[344, 368], [288, 307]],
            [1829, 1424],
            [2649, 2110],
            [263, 239],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            7,
        ),
        (
            30,
            30,
            [[282, 333], [358, 346]],
            [1520, 1728],
            [2310, 2432],
            [244, 257],
            63,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            4,
        ),
        (
            29,
            32,
            [[306, 340], [316, 337]],
            [1629, 1628],
            [2433, 2362],
            [253, 238],
            64,
            165876268,
            218243354,
            0,
            [[6249552, 6698611], [6584205, 6815470]],
            2,
        ),
    ],
    &[
        (
            32,
            34,
            [[339, 322], [315, 322]],
            [1728, 1525],
            [2491, 2255],
            [258, 257],
            62,
            165876268,
            218355944,
            112227,
            [[6284035, 6698611], [6584205, 6893577]],
            6,
        ),
        (
            32,
            31,
            [[316, 314], [334, 354]],
            [1574, 1677],
            [2399, 2398],
            [243, 228],
            64,
            165876268,
            218474361,
            112227,
            [[6330824, 6698611], [6584205, 6965205]],
            7,
        ),
        (
            27,
            31,
            [[324, 331], [344, 374]],
            [1570, 1678],
            [2379, 2384],
            [240, 226],
            64,
            165876268,
            218645864,
            112227,
            [[6401794, 6698611], [6584205, 7065738]],
            7,
        ),
        (
            34,
            28,
            [[313, 339], [359, 454]],
            [1427, 1829],
            [2262, 2567],
            [251, 251],
            64,
            165876268,
            218794658,
            112227,
            [[6439245, 6698611], [6584205, 7177081]],
            4,
        ),
        (
            43,
            30,
            [[376, 351], [330, 480]],
            [1524, 1724],
            [2330, 2391],
            [251, 258],
            63,
            165876268,
            219110202,
            112227,
            [[6545900, 6698611], [6584205, 7385970]],
            4,
        ),
        (
            49,
            36,
            [[507, 376], [296, 433]],
            [1829, 1424],
            [2648, 2115],
            [263, 240],
            64,
            165876268,
            219435608,
            112227,
            [[6735165, 6698611], [6584205, 7522111]],
            5,
        ),
        (
            46,
            30,
            [[436, 344], [374, 519]],
            [1520, 1728],
            [2311, 2430],
            [250, 259],
            64,
            165876268,
            219696028,
            112227,
            [[6838841, 6698611], [6584205, 7678855]],
            7,
        ),
        (
            60,
            32,
            [[553, 377], [334, 542]],
            [1629, 1628],
            [2435, 2371],
            [259, 236],
            64,
            165876268,
            219923242,
            112227,
            [[6988811, 6698611], [6584205, 7756099]],
            1,
        ),
    ],
    &[
        (
            28,
            34,
            [[330, 329], [323, 315]],
            [1728, 1525],
            [2491, 2251],
            [258, 256],
            62,
            165876268,
            218391844,
            112227,
            [[6249552, 6786836], [6644470, 6815470]],
            5,
        ),
        (
            33,
            31,
            [[285, 334], [363, 339]],
            [1574, 1677],
            [2396, 2398],
            [241, 229],
            64,
            165876268,
            218618093,
            112227,
            [[6249552, 6903534], [6754021, 6815470]],
            7,
        ),
        (
            41,
            31,
            [[291, 373], [382, 349]],
            [1570, 1678],
            [2379, 2378],
            [238, 228],
            63,
            165876268,
            218825745,
            112227,
            [[6249552, 7012368], [6852839, 6815470]],
            5,
        ),
        (
            44,
            28,
            [[250, 403], [438, 391]],
            [1427, 1829],
            [2255, 2565],
            [246, 251],
            64,
            165876268,
            219036885,
            112227,
            [[6249552, 7092753], [6983594, 6815470]],
            8,
        ),
        (
            45,
            30,
            [[286, 447], [443, 378]],
            [1524, 1724],
            [2336, 2388],
            [248, 252],
            64,
            165876268,
            219344217,
            112227,
            [[6249552, 7266213], [7117466, 6815470]],
            3,
        ),
        (
            49,
            36,
            [[345, 534], [426, 326]],
            [1829, 1424],
            [2650, 2115],
            [265, 242],
            64,
            165876268,
            219670172,
            112227,
            [[6249552, 7445611], [7264023, 6815470]],
            2,
        ),
        (
            60,
            30,
            [[279, 510], [540, 376]],
            [1520, 1728],
            [2311, 2429],
            [245, 264],
            64,
            165876268,
            219927313,
            112227,
            [[6249552, 7580045], [7386730, 6815470]],
            1,
        ),
        (
            55,
            32,
            [[314, 564], [502, 359]],
            [1629, 1628],
            [2432, 2363],
            [261, 238],
            64,
            165876268,
            220183131,
            112227,
            [[6249552, 7723009], [7499584, 6815470]],
            4,
        ),
    ],
];

pub(crate) const DIRECTION_TRACES_1024: [u64; 3] =
    [0x6882536eb2efe02b, 0x8efcd277e7eac46b, 0x9d97c7cae502242b];

pub(crate) const DIRECTION_COMPOSITIONS_1024: [&[Composition]; 3] = [
    &[
        (
            [[581, 11652], [6687, 10983]],
            [
                [
                    [214620, -5235, 318258, -506972],
                    [239131, -5599, 344531, -493105],
                ],
                [
                    [227460, -5372, 281672, -462205],
                    [208207, -4723, 325571, -442474],
                ],
            ],
            [[1100, 1646], [1009, 1645], [3350, 0]],
            53,
            5817133415553407245,
        ),
        (
            [[-2901, 6687], [-1382, -3770]],
            [
                [
                    [183614, -4848, 311320, -490757],
                    [242170, -5630, 322780, -451060],
                ],
                [
                    [236853, -6402, 299222, -460036],
                    [223791, -5576, 312100, -482400],
                ],
            ],
            [[988, 1567], [1008, 1609], [3354, 0]],
            50,
            16045209168826706265,
        ),
        (
            [[-8529, 8394], [18417, 12490]],
            [
                [
                    [201505, -5725, 319226, -470946],
                    [226135, -4528, 337902, -464682],
                ],
                [
                    [214340, -5449, 288415, -438090],
                    [252654, -7017, 344915, -481233],
                ],
            ],
            [[965, 1585], [930, 1606], [3354, 1]],
            54,
            7381233627641188654,
        ),
        (
            [[-8212, 3329], [6909, 13514]],
            [
                [
                    [163052, -7806, 315500, -477929],
                    [216104, -5256, 314809, -483961],
                ],
                [
                    [260874, -4891, 319335, -490489],
                    [278227, -5749, 335746, -527326],
                ],
            ],
            [[1041, 1639], [1071, 1662], [3367, 0]],
            49,
            11210138435166375508,
        ),
        (
            [[2216, 9174], [16698, 13359]],
            [
                [
                    [200618, -6379, 316154, -462527],
                    [245142, -8272, 335595, -449909],
                ],
                [
                    [239104, -5315, 296632, -443489],
                    [276001, -5892, 322267, -456969],
                ],
            ],
            [[1038, 1616], [951, 1673], [3352, 0]],
            60,
            3247137162712829221,
        ),
        (
            [[-310, 3153], [-1956, -2020]],
            [
                [
                    [259432, -4791, 329634, -484625],
                    [260748, -6118, 356080, -504942],
                ],
                [
                    [192265, -2994, 254817, -388012],
                    [201502, -2863, 294264, -434048],
                ],
            ],
            [[938, 1676], [981, 1667], [3363, 0]],
            59,
            764532996799968190,
        ),
        (
            [[10166, 3088], [-3968, 3394]],
            [
                [
                    [197984, -5827, 311656, -462568],
                    [234859, -3039, 299910, -466501],
                ],
                [
                    [238872, -3262, 312584, -479852],
                    [254346, -5494, 342900, -498446],
                ],
            ],
            [[1013, 1528], [969, 1622], [3347, 0]],
            54,
            3404416904895450140,
        ),
        (
            [[-4233, 9490], [7933, 13205]],
            [
                [
                    [214989, -4418, 306165, -478033],
                    [254123, -6916, 344424, -493629],
                ],
                [
                    [226391, -6585, 299274, -464476],
                    [241971, -6434, 344037, -491916],
                ],
            ],
            [[1032, 1580], [1014, 1649], [3351, 0]],
            60,
            8384656864237310999,
        ),
    ],
    &[
        (
            [[-2159, 11652], [6724, -50]],
            [
                [
                    [222686, -5012, 326708, -510884],
                    [236897, -5160, 344403, -493906],
                ],
                [
                    [227047, -5372, 282402, -465650],
                    [211228, -4704, 332351, -447995],
                ],
            ],
            [[1098, 1660], [1010, 1647], [3350, 0]],
            52,
            15242842349370337392,
        ),
        (
            [[1289, 7520], [883, -5170]],
            [
                [
                    [209268, -5369, 335220, -499522],
                    [244199, -5637, 324581, -451118],
                ],
                [
                    [240871, -6615, 300756, -463032],
                    [233261, -5968, 327502, -491359],
                ],
            ],
            [[990, 1615], [1007, 1645], [3354, 0]],
            52,
            15161987032918355244,
        ),
        (
            [[-7369, 8554], [17442, -199]],
            [
                [
                    [225730, -5467, 340137, -491260],
                    [225916, -4555, 340027, -464627],
                ],
                [
                    [219994, -5227, 287965, -447821],
                    [268017, -7735, 387010, -510767],
                ],
            ],
            [[971, 1641], [931, 1663], [3355, 1]],
            54,
            12712124857853946605,
        ),
        (
            [[-5086, 2885], [6708, 1190]],
            [
                [
                    [212679, -10548, 353697, -507251],
                    [218571, -5278, 314641, -488634],
                ],
                [
                    [264108, -5018, 323876, -496454],
                    [320858, -7394, 393784, -571004],
                ],
            ],
            [[1043, 1726], [1072, 1760], [3368, 0]],
            54,
            2347284019407618795,
        ),
        (
            [[4090, 9527], [12727, 4091]],
            [
                [
                    [263069, -8385, 367856, -499793],
                    [251652, -8434, 340335, -456498],
                ],
                [
                    [235427, -5365, 294849, -447070],
                    [364159, -9322, 408897, -490701],
                ],
            ],
            [[1048, 1734], [958, 1843], [3353, 0]],
            61,
            17973165208132369960,
        ),
        (
            [[3244, 5792], [-166, 202]],
            [
                [
                    [362049, -8754, 414695, -543810],
                    [265462, -6661, 361742, -508102],
                ],
                [
                    [193534, -3770, 260130, -398269],
                    [294653, -4635, 410216, -529188],
                ],
            ],
            [[951, 1867], [991, 1825], [3362, 0]],
            58,
            12760567381758701610,
        ),
        (
            [[5022, 1195], [-3965, 11646]],
            [
                [
                    [286405, -7195, 442672, -579224],
                    [236352, -3903, 302596, -471787],
                ],
                [
                    [241506, -3495, 313518, -483735],
                    [352785, -11737, 484994, -586159],
                ],
            ],
            [[1029, 1731], [981, 1839], [3347, 0]],
            60,
            14319541334190284163,
        ),
        (
            [[-406, 8589], [8471, 7293]],
            [
                [
                    [357039, -8838, 471902, -582284],
                    [267838, -7070, 356029, -510220],
                ],
                [
                    [227444, -6241, 306978, -478929],
                    [349639, -10148, 492308, -620043],
                ],
            ],
            [[1055, 1875], [1017, 1935], [3351, 0]],
            63,
            4876945349722782605,
        ),
    ],
    &[
        (
            [[280, 267], [153, 11321]],
            [
                [
                    [215607, -5251, 318582, -508157],
                    [244605, -5592, 349888, -496497],
                ],
                [
                    [232779, -5568, 287688, -464250],
                    [207641, -4723, 324585, -440479],
                ],
            ],
            [[1101, 1650], [1009, 1653], [3350, 0]],
            51,
            16165550050043189585,
        ),
        (
            [[-2280, 2226], [-3554, -2959]],
            [
                [
                    [183300, -4848, 311636, -490238],
                    [251707, -6126, 350215, -461911],
                ],
                [
                    [270359, -7956, 320375, -473439],
                    [226900, -5697, 314243, -483162],
                ],
            ],
            [[989, 1599], [1010, 1638], [3353, 0]],
            40,
            1530281515048583347,
        ),
        (
            [[-8039, 6367], [-326, 12290]],
            [
                [
                    [202378, -5694, 321371, -470989],
                    [251723, -5986, 388798, -504770],
                ],
                [
                    [244625, -5877, 317756, -456529],
                    [254007, -7163, 349159, -488123],
                ],
            ],
            [[968, 1640], [933, 1660], [3354, 1]],
            56,
            5775214775904447558,
        ),
        (
            [[-8476, 5486], [3902, 14083]],
            [
                [
                    [165573, -7727, 307729, -473651],
                    [254389, -5936, 375716, -528019],
                ],
                [
                    [328199, -6648, 368180, -528284],
                    [282253, -6261, 336037, -532614],
                ],
            ],
            [[1042, 1744], [1073, 1747], [3368, 0]],
            48,
            5296857175929719305,
        ),
        (
            [[2001, 2298], [212, 13050]],
            [
                [
                    [203104, -6491, 314139, -470606],
                    [325451, -10408, 412148, -493927],
                ],
                [
                    [324382, -6606, 381839, -514974],
                    [283986, -5419, 327066, -461616],
                ],
            ],
            [[1052, 1733], [956, 1803], [3353, 0]],
            59,
            14476312193724415126,
        ),
        (
            [[714, -149], [-3924, -1511]],
            [
                [
                    [254419, -5538, 333426, -489755],
                    [366402, -9501, 489018, -604056],
                ],
                [
                    [286221, -3709, 350529, -458423],
                    [214244, -2622, 307808, -438111],
                ],
            ],
            [[954, 1831], [991, 1862], [3364, 0]],
            61,
            10057758972030701247,
        ),
        (
            [[10922, 3415], [-2041, 5108]],
            [
                [
                    [190208, -5545, 315035, -467795],
                    [342947, -5721, 442854, -577751],
                ],
                [
                    [351637, -7971, 425258, -584227],
                    [269699, -5410, 352932, -505103],
                ],
            ],
            [[1021, 1715], [983, 1813], [3347, 0]],
            61,
            10970552946675300088,
        ),
        (
            [[-4760, 2049], [7027, 13146]],
            [
                [
                    [214984, -4114, 305122, -482973],
                    [357147, -10379, 506542, -630902],
                ],
                [
                    [345663, -9761, 439680, -552550],
                    [247490, -6031, 350668, -500826],
                ],
            ],
            [[1048, 1778], [1024, 1914], [3351, 0]],
            61,
            8614354661603916758,
        ),
    ],
];

pub(crate) const DIRECTION_TAUGHT_1024: [&[TaughtBlock]; 3] = [
    &[
        ([0, 0], 29, 8, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 29, 7, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 25, 5, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 24, 6, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 29, 8, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 28, 7, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 30, 4, [[0, 0], [0, 0]], 0, 0),
        ([0, 0], 29, 2, [[0, 0], [0, 0]], 0, 0),
    ],
    &[
        ([17, 2], 32, 6, [[34483, 0], [0, 78107]], 46691, 112227),
        ([36, 9], 32, 7, [[46789, 0], [0, 71628]], 46691, 112227),
        ([44, 12], 27, 7, [[70970, 0], [0, 100533]], 46691, 112227),
        ([57, 15], 34, 4, [[37451, 0], [0, 111343]], 46691, 112227),
        ([63, 21], 43, 4, [[106655, 0], [0, 208889]], 46691, 112227),
        ([61, 21], 49, 5, [[189265, 0], [0, 136141]], 46691, 112227),
        ([64, 25], 46, 7, [[103676, 0], [0, 156744]], 46691, 112227),
        ([64, 34], 60, 1, [[149970, 0], [0, 77244]], 46691, 112227),
    ],
    &[
        ([15, 2], 28, 5, [[0, 88225], [60265, 0]], 46691, 112227),
        ([38, 6], 33, 7, [[0, 116698], [109551, 0]], 46691, 112227),
        ([52, 13], 41, 5, [[0, 108834], [98818, 0]], 46691, 112227),
        ([56, 14], 44, 8, [[0, 80385], [130755, 0]], 46691, 112227),
        ([61, 21], 45, 3, [[0, 173460], [133872, 0]], 46691, 112227),
        ([63, 24], 49, 2, [[0, 179398], [146557, 0]], 46691, 112227),
        ([64, 21], 60, 1, [[0, 134434], [122707, 0]], 46691, 112227),
        ([63, 24], 55, 4, [[0, 142964], [112854, 0]], 46691, 112227),
    ],
];

pub(crate) const DIRECTION_READ_1024: [u64; 3] =
    [0xb82310568dfe19e2, 0xae96ac18cddd3cb2, 0x47e927406cb50d3c];

pub(crate) const DIRECTION_CENSUS_1024: [&[(u32, u64)]; 3] = [
    &[
        (1, 1059),
        (2, 8070),
        (3, 12698),
        (4, 3908),
        (5, 224),
        (6, 40),
        (7, 9),
        (8, 4),
        (9, 1),
        (11, 1),
    ],
    &[
        (1, 1059),
        (2, 8067),
        (3, 12699),
        (4, 3912),
        (5, 220),
        (6, 41),
        (7, 10),
        (8, 4),
        (9, 1),
        (10, 1),
    ],
    &[
        (1, 1058),
        (2, 8059),
        (3, 12700),
        (4, 3913),
        (5, 228),
        (6, 41),
        (7, 9),
        (8, 4),
        (9, 1),
        (11, 1),
    ],
];

/// The verdict, as the rules compute it over the pinned tables, written from the run: both
/// clauses hold in both rewarded arms. **H-13 is yes** for this network, this taught delivery,
/// this regime and this rule.
pub(crate) const DIRECTION_1024: Direction = Direction {
    couplings: [true, true],
    response: [true, true],
    yes: true,
};

/// ADR-0078's prediction for the couplings, as read: it held in both arms.
pub(crate) const COUPLINGS_READ_1024: [bool; 2] = [true, true];

/// The assigned readout's paired tally over the last 128 trials, per rewarded arm: 128 of
/// 128 in the assignment and 127 in the mirrored (one tie), against the mark of 80.
pub(crate) const PAIRED_1024: [u32; 2] = [128, 127];

/// The other readout's paired tally over the last 128 trials, a reading: 59 and 45, with 62
/// and 71 ties — the delivery never reaches those synapses, so what rises there is the
/// network's.
pub(crate) const OTHER_PAIRED_1024: [u32; 2] = [59, 45];

/// The selections of the assigned readout over the last 128 trials, a reading beside 80 that
/// reopens nothing of H-12: 106 and 115.
pub(crate) const SELECTED_1024: [u32; 2] = [106, 115];

/// The image's couplings, `[stimulus][readout]`, the frozen block's: A→R0 6 249 552, A→R1
/// 6 698 611, B→R0 6 584 205, B→R1 6 815 470.
pub(crate) const IMAGE_COUPLINGS_1024: [[i64; 2]; 2] =
    [[6_249_552, 6_698_611], [6_584_205, 6_815_470]];

const _: () = assert!(
    IMAGE_COUPLINGS_1024[0][0] == BACKGROUND_1024[0].0.10[0][0]
        && IMAGE_COUPLINGS_1024[0][1] == BACKGROUND_1024[0].0.10[0][1]
        && IMAGE_COUPLINGS_1024[1][0] == BACKGROUND_1024[0].0.10[1][0]
        && IMAGE_COUPLINGS_1024[1][1] == BACKGROUND_1024[0].0.10[1][1]
);

/// The reach of each arm's delivery, `(inside the assigned pairs, outside)`, as read: none
/// under the withheld arm; every one of the assignment's 1 584 synapses (775 of A→R0 and 809
/// of B→R1) and 1 603 of the mirrored assignment's 1 604 (806 of A→R1 and 798 of B→R0)
/// moved, and no synapse outside the pairs in any arm.
pub(crate) const REACH_1024: [(u64, u64); 3] = [(0, 0), (1_584, 0), (1_603, 0)];

const _: () = assert!(
    REACH_1024[1].0 as u32 == SYNAPSES_1024[0][0] + SYNAPSES_1024[1][1]
        && REACH_1024[2].0 as u32 + 1 == SYNAPSES_1024[0][1] + SYNAPSES_1024[1][0]
);

// ------------------------------------------------ written before the run (brief 037)

/// H-14's run (ADR-0080): three of the instrument's runs, twenty-four blocks, the length
/// derived from H-13's blocks before any run and not this round's to move.
pub(crate) const REINFORCED_TRIALS: usize = 3 * TRIALS;

const _: () = assert!(REINFORCED_TRIALS == 1_536 && REINFORCED_TRIALS / BLOCK == 24);

/// The arms of H-14 (ADR-0080), in the order run and no other, every one `Delivery::Addressed`
/// from the one image — the task as ADR-0059 and ADR-0068 built it, the reward reaching the
/// synapses from the presented stimulus onto the readout the engine selected: the assignment
/// (`Feedback::Answer`, A's answer readout 0 and B's readout 1), the mirrored assignment
/// (`Feedback::Answer`, `mirrored`), and the shuffled reward (`Feedback::Shuffled`, the
/// reward's sign a coin the stimulus does not read), a reading of lock-in and no clause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Earned {
    Assignment,
    Mirrored,
    Shuffled,
}

pub(crate) const EARNED_ARMS: [Earned; 3] =
    [Earned::Assignment, Earned::Mirrored, Earned::Shuffled];

/// The rewarded arms in `EARNED_ARMS`'s order, the criterion's `[assignment, mirrored]`.
pub(crate) const EARNED_REWARDED: [Earned; 2] = [Earned::Assignment, Earned::Mirrored];

/// ADR-0080's prediction, a Hypothesis written before the run: H-14 is yes.
pub(crate) const REINFORCED_PREDICTED: bool = true;

/// ADR-0080's Hypothesis for the course, from arithmetic on H-13's blocks and not held to
/// the run: the selection passes `CROSSING_MARK` per block (40 of 64, the criterion's rate
/// over one block) by about trial `CROSSING_PREDICTED_BY_TRIAL` in both assignments, the
/// mirrored first. Read beside the run, never asserted.
pub(crate) const CROSSING_MARK: u32 = OFFSET_MARK_64;

const _: () = assert!(CROSSING_MARK == 40);

pub(crate) const CROSSING_PREDICTED_BY_TRIAL: usize = 900;

pub(crate) const MIRRORED_CROSSES_FIRST_PREDICTED: bool = true;

const _: () = assert!(REINFORCED_PREDICTED && MIRRORED_CROSSES_FIRST_PREDICTED);

const _: () = assert!(CROSSING_PREDICTED_BY_TRIAL < REINFORCED_TRIALS - LAST_BLOCKS * BLOCK);

/// The four stimulus–readout pairs, `(stimulus, readout)`: the shuffled arm's reach, since a
/// reward without information addresses whichever readout the engine selected.
pub(crate) const ALL_PAIRS: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

// ------------------------------------------------------------- the readings (brief 037)

/// One trial's reading under an arm (brief 037): the stimulus presented; the readouts'
/// counts in the task's window, as the task read them; the selection; whether it was
/// correct (the stimulus's answer; a tie is not); the reward the task delivered, signed; the
/// trace consolidated into the weights over the trial, `[stimulus][readout]`, by the oracle;
/// and the dopamine signal at the trial's end, before the reward, and after it.
pub(crate) type EarnedTrial = (u8, [u32; 2], Option<u8>, bool, i32, [[i64; 2]; 2], i32, i32);

/// One block of an arm (brief 037): the selections per stimulus, `[stimulus][readout 0,
/// readout 1, tie]`; the positive rewards; the trace consolidated into the weights over the
/// block, `[stimulus][readout]`; and the signal at the block's last trial's end and after
/// its reward.
pub(crate) type EarnedBlock = ([[u32; 3]; 2], u32, [[i64; 2]; 2], i32, i32);

/// Where an arm's reward takes its sign from: the answer, or the shuffled coin.
pub(crate) fn feedback_of(arm: Earned) -> Feedback {
    if arm == Earned::Shuffled {
        Feedback::Shuffled
    } else {
        Feedback::Answer
    }
}

/// Whether an arm mirrors the assignment.
pub(crate) fn earned_mirrors(arm: Earned) -> bool {
    arm == Earned::Mirrored
}

/// True for an arm whose reward carries the answer, the criterion's two.
pub(crate) fn answers(arm: Earned) -> bool {
    arm != Earned::Shuffled
}

/// The pairs an arm's delivery can reach: the two answer pairs where the reward carries the
/// answer — ADR-0080's derivation, a wrong selection's pair spending the next trial under a
/// signal below zero and consolidating nothing — and all four under the shuffled reward.
pub(crate) fn reachable_pairs(arm: Earned) -> Vec<(usize, usize)> {
    if answers(arm) {
        assigned_pairs(earned_mirrors(arm)).to_vec()
    } else {
        ALL_PAIRS.to_vec()
    }
}

/// The selections per stimulus over `read`: to readout 0, to readout 1, and the ties.
pub(crate) fn splits(read: &[EarnedTrial]) -> [[u32; 3]; 2] {
    let mut out = [[0u32; 3]; 2];
    for t in read {
        let into = &mut out[usize::from(t.0)][t.2.map_or(2, usize::from)];
        *into = into.saturating_add(1);
    }
    out
}

/// The selections per stimulus over the last `LAST_BLOCKS` blocks, the criterion's window.
pub(crate) fn last_splits(read: &[EarnedTrial]) -> [[u32; 3]; 2] {
    let from = read.len().saturating_sub(LAST_BLOCKS.saturating_mul(BLOCK));
    splits(read.get(from..).unwrap_or(&[]))
}

/// An arm's blocks from its trials, `BLOCK` trials each: the splits, the positive rewards,
/// the consolidation summed and the last trial's signals.
pub(crate) fn earned_blocks(read: &[EarnedTrial]) -> Vec<EarnedBlock> {
    read.chunks(BLOCK)
        .map(|mine| {
            let mut positive = 0u32;
            let mut transferred = [[0i64; 2]; 2];
            for t in mine {
                positive = positive.saturating_add(u32::from(t.4 > 0));
                for (s, readouts) in t.5.iter().enumerate() {
                    for (k, &amount) in readouts.iter().enumerate() {
                        transferred[s][k] = transferred[s][k].saturating_add(amount);
                    }
                }
            }
            let last = mine.last().expect("a block holds a trial");
            (splits(mine), positive, transferred, last.6, last.7)
        })
        .collect()
}

/// Where the selection first passes `CROSSING_MARK` per block, as the trial at that block's
/// end; none when no block does.
pub(crate) fn crossing(blocks: &[Block]) -> Option<usize> {
    blocks
        .iter()
        .position(|b| b.0 >= CROSSING_MARK)
        .map(|k| k.saturating_add(1).saturating_mul(BLOCK))
}

/// The lock-in reading (ADR-0080): per stimulus, the shuffled arm goes to one readout over
/// the last 128 trials as often as a rewarded arm goes to that stimulus's answer — the larger
/// of its two readouts' counts at least the smaller of the two rewarded arms' correct counts
/// for the stimulus (the same trials present the same stimuli in every arm). A reading, not a
/// clause.
pub(crate) fn locked_in(shuffled: [[u32; 3]; 2], rewarded: [[[u32; 3]; 2]; 2]) -> [bool; 2] {
    let mut out = [false; 2];
    for (s, into) in out.iter_mut().enumerate() {
        let dominant = shuffled[s][0].max(shuffled[s][1]);
        let answered = EARNED_REWARDED
            .iter()
            .zip(rewarded.iter())
            .map(|(&arm, split)| split[s][answer_of(s as u8, earned_mirrors(arm))])
            .min()
            .unwrap_or(0);
        *into = dominant >= answered;
    }
    out
}

/// ADR-0080's derivation read over an arm's trials, clause by clause: the signal at every
/// trial's end at most the fixed point's (`SIGNAL_END_FIXED_Q16`, 0.712); after every
/// negative reward the signal below zero; and nothing consolidated in the trial after a
/// negative reward, the addressed pair — a wrong selection's, or none at a tie — spending
/// it under a signal below zero. Every clause true on every arm is the assertion; a false
/// one is a finding against the derivation, reported beside the verdict.
pub(crate) fn derivation(read: &[EarnedTrial]) -> [bool; 3] {
    let bounded = read.iter().all(|t| t.6 <= SIGNAL_END_FIXED_Q16);
    let negative = read.iter().filter(|t| t.4 < 0).all(|t| t.7 < 0);
    let nothing = read
        .windows(2)
        .filter(|w| w[0].4 < 0)
        .all(|w| w[1].5 == [[0; 2]; 2]);
    [bounded, negative, nothing]
}

// ------------------------------------------------------------ the criterion (brief 037)

/// H-14's criterion (ADR-0080), per rewarded arm `[assignment, mirrored]`: the correct
/// selections over the last `LAST_BLOCKS` blocks at least `REWARDED_MIN` — `last_correct`,
/// ADR-0066's rule over the same shape, the task counting a trial correct only when the
/// selection is the stimulus's answer, a tie not. `yes` is both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Reinforced {
    pub(crate) correct: [bool; 2],
    pub(crate) yes: bool,
}

pub(crate) fn reinforced(arms: [&[Block]; 2]) -> Reinforced {
    let correct = [
        last_correct(arms[0]) >= REWARDED_MIN,
        last_correct(arms[1]) >= REWARDED_MIN,
    ];
    Reinforced {
        correct,
        yes: correct[0] && correct[1],
    }
}

// ---------------------------------------------------------------- the run (brief 037)

/// An arm's run: the blocks, the accuracy-sequence trace, the composed trials, every trial's
/// reading and the volley-tick census over the run.
pub(crate) type EarnedRun = (Vec<Block>, u64, Vec<Composed>, Vec<EarnedTrial>, Vec<u64>);

/// The FNV-1a hash of every trial's reading, each number as its `i32` words.
pub(crate) fn earned_hash(read: &[EarnedTrial]) -> u64 {
    let mut words: Vec<i32> = Vec::new();
    let mut wide = |x: i64| {
        words.push(x as i32);
        words.push((x >> 32) as i32);
    };
    for t in read {
        wide(i64::from(t.0));
        wide(i64::from(t.1[0]));
        wide(i64::from(t.1[1]));
        wide(t.2.map_or(3, i64::from));
        wide(i64::from(t.3));
        wide(i64::from(t.4));
        for readouts in &t.5 {
            for &amount in readouts {
                wide(amount);
            }
        }
        wide(i64::from(t.6));
        wide(i64::from(t.7));
    }
    fnv1a_64(&words)
}

/// An arm's run from the frozen engine (brief 037): `trials` trials of the calibration's task
/// with ADR-0076's stimulus, `Delivery::Addressed` and the arm's feedback — the task itself
/// writing the addressed set after every trial to the presented stimulus's units onto the
/// selected readout's units, none at a tie, and delivering the reward, `REWARD_Q16` signed by
/// the outcome or by the coin — the composer seeded from the record and replaying the
/// consolidation under the pair the task addressed and the signal its reward left. At every
/// trial the record's traces and weights are held to the oracle and the record's signal, read
/// after the task's reward, to the course's end plus that reward; the task's contract is
/// asserted trial by trial — the selection the sign of the count difference, correct the
/// answer and a tie not, the reward's sign the outcome's, the addressed set the presented
/// stimulus onto the selected readout, the signal the task read the record's. Nothing of
/// ADR-0080's derivation is asserted here: it is read after the run (`derivation`) so that a
/// failure is reported beside the verdict and not in place of it.
pub(crate) fn earned_run(exec: &mut Engine, arm: Earned, units: u32, trials: usize) -> EarnedRun {
    earned_run_under(
        exec,
        feedback_of(arm),
        earned_mirrors(arm),
        units,
        trials,
        0,
    )
}

/// `earned_run` under a feedback, an assignment and a modulation baseline (brief 038): H-14's
/// arms pass their feedback and zero, the image's, and are the runs they were; H-15's pass
/// theirs and 0.5, under which the oracle consolidates every stimulus–readout synapse under
/// the baseline plus the signal where the synapse is addressed and under the baseline alone
/// elsewhere (`Composer::baseline`); and under `Feedback::Withheld` the task delivers no
/// reward and the signal stays at rest, asserted trial by trial, so every synapse consolidates
/// under the baseline alone.
pub(crate) fn earned_run_under(
    exec: &mut Engine,
    feedback: Feedback,
    mirrored: bool,
    units: u32,
    trials: usize,
    baseline_q16: i32,
) -> EarnedRun {
    earned_run_flipped(
        exec,
        feedback,
        mirrored,
        units,
        trials,
        baseline_q16,
        None,
        &mut |_, _| {},
    )
}

/// `earned_run_under` with the mapping flipped once (brief 040): before the trial of index
/// `flip`, if one is given, `run_on_flipped` negates the task's `mirrored` and touches nothing
/// else, and every trial's contract is asserted under the mapping in force at it — `mirrored`
/// before the flip, the other after it. `after` is called at the end of every trial's reading
/// with the executor and the trial's index; it reads and never writes. With no flip and an
/// `after` that reads nothing it is `earned_run_under`, which calls it so.
#[allow(clippy::too_many_arguments)]
pub(crate) fn earned_run_flipped(
    exec: &mut Engine,
    feedback: Feedback,
    mirrored: bool,
    units: u32,
    trials: usize,
    baseline_q16: i32,
    flip: Option<usize>,
    after: &mut dyn FnMut(&Engine, usize),
) -> EarnedRun {
    assert_eq!(
        units, 1024,
        "the cancel and the synapse counts below are pinned at 1 024 units"
    );
    assert_eq!(
        exec.modulation_baseline_q16(),
        baseline_q16,
        "the baseline is the image's"
    );
    assert_eq!(
        exec.modulator().dopamine_rpe,
        0,
        "the signal is at rest before the first trial"
    );
    let sets = geometry(units, rotation(units));
    let mut composer = Composer::new(units);
    composer.enumerate(exec);
    composer.cursor = exec.ticks() as u32;
    composer.baseline = baseline_q16;
    composer.taught = Some(Taught {
        addressed: None,
        signal: 0,
    });
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    let task = task(
        SHAPE_F46,
        Some(cancel_of(picked)),
        units,
        feedback,
        mirrored,
        Delivery::Addressed,
    );
    // The task is `Copy`: a copy reads the coin the run's own task drew.
    let probe = task;
    let mut read: Vec<EarnedTrial> = Vec::with_capacity(trials);
    let (blocks, trace) = run_on_flipped(
        exec,
        task,
        units,
        trials,
        flip,
        &mut |exec, trial, start, outcome| {
            // The task rewarded at the trial's end, before this reading: the oracle holds the
            // record's signal to the course's end plus it.
            composer.rewarded = outcome.reward_q16;
            composer.observe(exec, trial, start, outcome);
            assert_eq!(
                outcome.selection,
                selected(outcome.counts),
                "trial {trial}: the selection is the sign of the count difference"
            );
            // The mapping in force: the run's before the flip, the other from it on.
            let in_force = mirrored != flip.is_some_and(|f| trial >= f);
            let answer = answer_of(outcome.stimulus, in_force) as u8;
            assert_eq!(
                outcome.correct,
                outcome.selection == Some(answer),
                "trial {trial}: correct is the answer, a tie not"
            );
            let signed = |positive: bool| {
                if positive {
                    REWARD_Q16
                } else {
                    REWARD_Q16.saturating_neg()
                }
            };
            let expected = match feedback {
                Feedback::Answer => signed(outcome.correct),
                Feedback::Shuffled => signed(probe.coin_at(trial as u64)),
                Feedback::Withheld => 0,
            };
            assert_eq!(
                outcome.reward_q16, expected,
                "trial {trial}: the reward's sign is the outcome's, and none is withheld"
            );
            let stimulus = usize::from(outcome.stimulus);
            let targets = outcome
                .selection
                .map_or(0, |r| readout_set(&sets, usize::from(r)).len() as usize);
            assert_eq!(
                exec.addressed_counts(),
                (sets[stimulus].len() as usize, targets),
                "trial {trial}: the addressed set is the presented stimulus onto the selected readout, onto none at a tie"
            );
            let signal = exec.modulator().dopamine_rpe;
            assert_eq!(
                outcome.signal_q16, signal,
                "trial {trial}: the task read the signal after its reward"
            );
            if feedback == Feedback::Withheld {
                assert_eq!(
                    signal, 0,
                    "trial {trial}: the withheld arm's signal stays at rest"
                );
            }
            let end = signal.saturating_sub(outcome.reward_q16);
            let transferred = composer.transferred.last().copied().unwrap_or([[0; 2]; 2]);
            composer.taught = Some(Taught {
                addressed: outcome.selection.map(|r| (stimulus, usize::from(r))),
                signal,
            });
            read.push((
                outcome.stimulus,
                outcome.counts,
                outcome.selection,
                outcome.correct,
                outcome.reward_q16,
                transferred,
                end,
                signal,
            ));
            after(exec, trial);
        },
    );
    assert_eq!(composer.out.len(), trials);
    assert_eq!(composer.counts(), SYNAPSES_1024);
    (blocks, trace, composer.out, read, composer.volley_ticks)
}

/// Dumps an arm's run: the sight's blocks and trace, the rows, the composition per block, the
/// earned blocks, every trial's reading with its hash, the census, the crossing and the last
/// blocks' splits.
pub(crate) fn dump_earned(name: &str, run: &EarnedRun, earned: &[EarnedBlock]) {
    let (blocks, trace, trials, read, volley_ticks) = run;
    eprintln!(
        "DUMP {name} sight {blocks:?} trace {trace:#018x} curve {:?}",
        curve(blocks)
    );
    let rows: Vec<Row> = trials.iter().map(trial_row).collect();
    eprintln!("DUMP {name} rows {rows:?}");
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    eprintln!("DUMP {name} compositions {compositions:?}");
    eprintln!("DUMP {name} earned {earned:?}");
    eprintln!("DUMP {name} read {read:?} hash {:#018x}", earned_hash(read));
    eprintln!("DUMP {name} census {:?}", census_of(volley_ticks));
    eprintln!(
        "DUMP {name} crossing {:?} last splits {:?} last correct {}",
        crossing(blocks),
        last_splits(read),
        last_correct(blocks)
    );
}

/// Holds an arm's run to its pinned tables: the blocks and the trace, the composition per
/// block, the earned blocks, the readings' hash and the census.
pub(crate) fn pinned_earned(name: &str, k: usize, run: &EarnedRun, earned: &[EarnedBlock]) {
    let (blocks, trace, trials, read, volley_ticks) = run;
    pinned(
        &format!("{name} sight"),
        blocks,
        *trace,
        REINFORCED_BLOCKS_1024[k],
        REINFORCED_TRACES_1024[k],
    );
    let compositions: Vec<Composition> = trials.chunks(BLOCK).map(composition).collect();
    assert_eq!(
        compositions.as_slice(),
        REINFORCED_COMPOSITIONS_1024[k],
        "{name}: the composition per block"
    );
    assert_eq!(
        earned, REINFORCED_EARNED_1024[k],
        "{name}: the earned blocks"
    );
    assert_eq!(
        earned_hash(read),
        REINFORCED_READ_1024[k],
        "{name}: the readings"
    );
    assert_eq!(
        census_of(volley_ticks),
        REINFORCED_CENSUS_1024[k].to_vec(),
        "{name}: the volley's ticks"
    );
}

// ----------------------------------------------------------- the measurement (brief 037)

/// The three arms at 1 024 units, in `EARNED_ARMS`'s order, each pinned from one run: the
/// sight's blocks and trace, the composition per block, the earned blocks, the readings' hash
/// and the volley's census.
pub(crate) const REINFORCED_BLOCKS_1024: [&[Block]; 3] = [
    &[
        (
            31,
            34,
            [[335, 322], [314, 321]],
            [1728, 1525],
            [2491, 2253],
            [258, 257],
            62,
            165876268,
            218311227,
            20605,
            [[6260538, 6698611], [6584205, 6872357]],
            8,
        ),
        (
            32,
            31,
            [[302, 313], [330, 347]],
            [1574, 1677],
            [2397, 2397],
            [242, 228],
            64,
            165876268,
            218371968,
            108546,
            [[6276433, 6698611], [6584205, 6917203]],
            5,
        ),
        (
            26,
            31,
            [[300, 330], [341, 366]],
            [1570, 1678],
            [2381, 2381],
            [238, 226],
            63,
            165876268,
            218470728,
            -93914,
            [[6316895, 6698611], [6584205, 6975501]],
            6,
        ),
        (
            29,
            28,
            [[269, 339], [356, 423]],
            [1427, 1829],
            [2257, 2567],
            [249, 251],
            64,
            165876268,
            218545241,
            103139,
            [[6316899, 6698611], [6584205, 7050010]],
            8,
        ),
        (
            37,
            30,
            [[315, 345], [326, 413]],
            [1524, 1724],
            [2328, 2392],
            [249, 256],
            63,
            165876268,
            218693797,
            93860,
            [[6368828, 6698611], [6584205, 7146637]],
            6,
        ),
        (
            37,
            36,
            [[405, 372], [293, 363]],
            [1829, 1424],
            [2646, 2112],
            [263, 240],
            64,
            165876268,
            218856167,
            44954,
            [[6470633, 6698611], [6584205, 7207202]],
            7,
        ),
        (
            41,
            30,
            [[352, 339], [378, 441]],
            [1520, 1728],
            [2310, 2430],
            [248, 257],
            64,
            165876268,
            219022507,
            46759,
            [[6545081, 6698611], [6584205, 7299094]],
            2,
        ),
        (
            46,
            32,
            [[424, 360], [322, 442]],
            [1629, 1628],
            [2432, 2369],
            [257, 236],
            64,
            165876268,
            219211033,
            -79183,
            [[6636930, 6698611], [6584205, 7395771]],
            5,
        ),
        (
            50,
            34,
            [[477, 366], [351, 454]],
            [1729, 1524],
            [2541, 2221],
            [254, 222],
            64,
            165876268,
            219480336,
            112153,
            [[6794294, 6698611], [6584205, 7507710]],
            3,
        ),
        (
            50,
            28,
            [[382, 305], [355, 586]],
            [1424, 1831],
            [2281, 2534],
            [246, 225],
            64,
            165876268,
            219699555,
            47376,
            [[6857770, 6698611], [6584205, 7663453]],
            3,
        ),
        (
            58,
            33,
            [[531, 379], [299, 538]],
            [1675, 1578],
            [2452, 2270],
            [232, 241],
            64,
            165876268,
            219918803,
            61055,
            [[6952166, 6698611], [6584205, 7788305]],
            1,
        ),
        (
            56,
            32,
            [[492, 358], [336, 565]],
            [1625, 1629],
            [2466, 2382],
            [255, 248],
            64,
            165876268,
            220117249,
            112219,
            [[7055395, 6698611], [6584205, 7883522]],
            3,
        ),
        (
            58,
            32,
            [[503, 340], [324, 606]],
            [1631, 1627],
            [2424, 2341],
            [246, 221],
            64,
            165876268,
            220331943,
            112203,
            [[7139834, 6698611], [6584205, 8013777]],
            1,
        ),
        (
            59,
            33,
            [[655, 377], [317, 571]],
            [1676, 1579],
            [2513, 2339],
            [262, 242],
            64,
            165876268,
            220578432,
            112219,
            [[7306613, 6698611], [6584205, 8093487]],
            2,
        ),
        (
            60,
            37,
            [[731, 417], [301, 568]],
            [1883, 1375],
            [2685, 2155],
            [253, 257],
            64,
            165876268,
            220821989,
            112225,
            [[7487000, 6698611], [6584205, 8156657]],
            1,
        ),
        (
            60,
            35,
            [[720, 421], [326, 616]],
            [1782, 1472],
            [2562, 2227],
            [238, 261],
            64,
            165876268,
            221072870,
            110207,
            [[7645734, 6698611], [6584205, 8248804]],
            1,
        ),
        (
            63,
            33,
            [[772, 410], [342, 660]],
            [1678, 1573],
            [2510, 2235],
            [250, 250],
            64,
            165876268,
            221367367,
            111555,
            [[7820216, 6698611], [6584205, 8368819]],
            0,
        ),
        (
            64,
            38,
            [[868, 438], [302, 597]],
            [1932, 1322],
            [2719, 2040],
            [256, 263],
            64,
            165876268,
            221627102,
            112227,
            [[8003523, 6698611], [6584205, 8445247]],
            0,
        ),
        (
            64,
            31,
            [[727, 374], [383, 788]],
            [1577, 1680],
            [2395, 2409],
            [243, 260],
            64,
            165876268,
            221831320,
            112227,
            [[8102115, 6698611], [6584205, 8550873]],
            0,
        ),
        (
            64,
            34,
            [[892, 446], [367, 780]],
            [1726, 1524],
            [2471, 2274],
            [285, 269],
            64,
            165876268,
            222085687,
            112227,
            [[8212803, 6698611], [6584205, 8694552]],
            0,
        ),
        (
            64,
            30,
            [[810, 407], [377, 896]],
            [1525, 1730],
            [2284, 2445],
            [270, 227],
            64,
            165876268,
            222358932,
            112227,
            [[8331215, 6698611], [6584205, 8849385]],
            0,
        ),
        (
            64,
            29,
            [[802, 350], [408, 976]],
            [1472, 1781],
            [2266, 2470],
            [244, 236],
            64,
            165876268,
            222623670,
            112227,
            [[8426440, 6698611], [6584205, 9018898]],
            0,
        ),
        (
            64,
            32,
            [[880, 395], [347, 880]],
            [1627, 1629],
            [2419, 2402],
            [289, 245],
            64,
            165876268,
            222769677,
            112227,
            [[8473552, 6698611], [6584205, 9117793]],
            0,
        ),
        (
            64,
            29,
            [[857, 387], [430, 982]],
            [1469, 1779],
            [2232, 2471],
            [260, 248],
            64,
            165876268,
            222902041,
            112227,
            [[8539069, 6698611], [6584205, 9184640]],
            0,
        ),
    ],
    &[
        (
            28,
            34,
            [[331, 327], [319, 315]],
            [1728, 1525],
            [2491, 2253],
            [258, 256],
            62,
            165876268,
            218322302,
            -41361,
            [[6249552, 6753869], [6607895, 6815470]],
            6,
        ),
        (
            31,
            31,
            [[284, 328], [338, 333]],
            [1574, 1677],
            [2395, 2400],
            [241, 228],
            64,
            165876268,
            218446848,
            -110127,
            [[6249552, 6832670], [6653640, 6815470]],
            6,
        ),
        (
            38,
            31,
            [[288, 357], [354, 344]],
            [1570, 1678],
            [2379, 2379],
            [236, 227],
            64,
            165876268,
            218591900,
            93919,
            [[6249552, 6915303], [6716059, 6815470]],
            5,
        ),
        (
            39,
            28,
            [[250, 377], [393, 381]],
            [1427, 1829],
            [2253, 2566],
            [246, 251],
            64,
            165876268,
            218719288,
            34826,
            [[6249552, 6977226], [6781524, 6815470]],
            8,
        ),
        (
            39,
            30,
            [[286, 418], [370, 362]],
            [1524, 1724],
            [2337, 2386],
            [245, 253],
            63,
            165876268,
            218954186,
            -81964,
            [[6249552, 7130542], [6863106, 6815470]],
            5,
        ),
        (
            44,
            36,
            [[348, 482], [358, 310]],
            [1829, 1424],
            [2648, 2114],
            [264, 240],
            64,
            165876268,
            219195375,
            110197,
            [[6249552, 7270845], [6963992, 6815470]],
            4,
        ),
        (
            53,
            30,
            [[279, 470], [447, 363]],
            [1520, 1728],
            [2309, 2428],
            [247, 261],
            64,
            165876268,
            219395998,
            -20865,
            [[6249552, 7390767], [7044693, 6815470]],
            4,
        ),
        (
            46,
            32,
            [[311, 505], [436, 351]],
            [1629, 1628],
            [2430, 2361],
            [259, 238],
            64,
            165876268,
            219603004,
            94752,
            [[6249552, 7516972], [7125494, 6815470]],
            3,
        ),
        (
            53,
            34,
            [[351, 548], [450, 330]],
            [1729, 1524],
            [2534, 2219],
            [257, 229],
            64,
            165876268,
            219905394,
            34885,
            [[6249552, 7725445], [7219411, 6815470]],
            3,
        ),
        (
            51,
            28,
            [[255, 520], [493, 407]],
            [1424, 1831],
            [2274, 2525],
            [246, 226],
            64,
            165876268,
            220165607,
            112227,
            [[6249552, 7884337], [7320732, 6815470]],
            2,
        ),
        (
            53,
            33,
            [[348, 692], [454, 347]],
            [1675, 1578],
            [2453, 2270],
            [230, 240],
            64,
            165876268,
            220454543,
            112227,
            [[6249552, 8055461], [7438544, 6815470]],
            3,
        ),
        (
            57,
            32,
            [[289, 669], [527, 377]],
            [1625, 1629],
            [2461, 2380],
            [249, 246],
            64,
            165876268,
            220732267,
            106077,
            [[6249552, 8247916], [7523813, 6815470]],
            3,
        ),
        (
            62,
            32,
            [[281, 724], [543, 364]],
            [1631, 1627],
            [2420, 2339],
            [246, 221],
            64,
            165876268,
            221020248,
            112003,
            [[6249552, 8423231], [7636479, 6815470]],
            1,
        ),
        (
            59,
            33,
            [[361, 800], [509, 330]],
            [1676, 1579],
            [2513, 2331],
            [257, 255],
            64,
            165876268,
            221235189,
            111555,
            [[6249552, 8567623], [7707028, 6815470]],
            2,
        ),
        (
            61,
            37,
            [[359, 931], [512, 323]],
            [1883, 1375],
            [2682, 2140],
            [246, 256],
            64,
            165876268,
            221470699,
            112227,
            [[6249552, 8736444], [7773717, 6815470]],
            2,
        ),
        (
            63,
            35,
            [[321, 926], [563, 353]],
            [1782, 1472],
            [2561, 2223],
            [236, 259],
            64,
            165876268,
            221746920,
            112227,
            [[6249552, 8900532], [7885850, 6815470]],
            0,
        ),
        (
            63,
            33,
            [[358, 947], [598, 312]],
            [1678, 1572],
            [2505, 2232],
            [244, 252],
            64,
            165876268,
            221971013,
            112153,
            [[6249552, 9029308], [7981167, 6815470]],
            0,
        ),
        (
            63,
            38,
            [[380, 1077], [538, 303]],
            [1932, 1322],
            [2701, 2030],
            [255, 254],
            64,
            165876268,
            222190446,
            63075,
            [[6249552, 9175284], [8054624, 6815470]],
            0,
        ),
        (
            63,
            31,
            [[307, 939], [704, 381]],
            [1578, 1680],
            [2377, 2396],
            [237, 275],
            64,
            165876268,
            222428354,
            112227,
            [[6249552, 9278323], [8189493, 6815470]],
            0,
        ),
        (
            64,
            34,
            [[341, 1073], [665, 383]],
            [1726, 1524],
            [2473, 2276],
            [278, 276],
            64,
            165876268,
            222653575,
            112227,
            [[6249552, 9392284], [8300753, 6815470]],
            0,
        ),
        (
            62,
            30,
            [[305, 991], [734, 401]],
            [1525, 1730],
            [2288, 2442],
            [261, 228],
            64,
            165876268,
            222937437,
            110207,
            [[6249552, 9495007], [8481892, 6815470]],
            1,
        ),
        (
            64,
            29,
            [[261, 950], [846, 431]],
            [1472, 1781],
            [2277, 2467],
            [238, 238],
            64,
            165876268,
            223110795,
            112227,
            [[6249552, 9547724], [8602533, 6815470]],
            0,
        ),
        (
            64,
            32,
            [[311, 1049], [810, 368]],
            [1628, 1629],
            [2425, 2402],
            [288, 237],
            64,
            165876268,
            223344501,
            112227,
            [[6249552, 9609881], [8774082, 6815470]],
            0,
        ),
        (
            64,
            29,
            [[312, 979], [1012, 401]],
            [1470, 1779],
            [2230, 2473],
            [264, 248],
            64,
            165876268,
            223555595,
            112227,
            [[6249552, 9678609], [8916448, 6815470]],
            0,
        ),
    ],
    &[
        (
            30,
            34,
            [[330, 323], [314, 318]],
            [1728, 1525],
            [2491, 2253],
            [257, 256],
            62,
            165876268,
            218289725,
            105959,
            [[6240445, 6712128], [6602158, 6839478]],
            7,
        ),
        (
            30,
            31,
            [[287, 312], [340, 343]],
            [1574, 1677],
            [2395, 2398],
            [241, 228],
            64,
            165876268,
            218359904,
            -47354,
            [[6244270, 6719853], [6640407, 6859858]],
            6,
        ),
        (
            25,
            31,
            [[290, 337], [349, 351]],
            [1570, 1678],
            [2382, 2377],
            [237, 229],
            63,
            165876268,
            218475969,
            49150,
            [[6251411, 6736551], [6676517, 6915974]],
            2,
        ),
        (
            27,
            28,
            [[255, 350], [373, 411]],
            [1427, 1829],
            [2257, 2566],
            [246, 250],
            64,
            165876268,
            218564660,
            111552,
            [[6251906, 6747560], [6695971, 6973707]],
            4,
        ),
        (
            29,
            30,
            [[289, 361], [357, 396]],
            [1524, 1724],
            [2336, 2388],
            [245, 259],
            63,
            165876268,
            218815837,
            -25029,
            [[6273958, 6841827], [6766247, 7038289]],
            6,
        ),
        (
            25,
            36,
            [[355, 416], [339, 340]],
            [1829, 1424],
            [2642, 2110],
            [263, 242],
            64,
            165876268,
            218911127,
            19035,
            [[6296741, 6877683], [6804228, 7036959]],
            5,
        ),
        (
            26,
            30,
            [[292, 379], [417, 398]],
            [1520, 1728],
            [2308, 2427],
            [245, 261],
            64,
            165876268,
            219059891,
            94524,
            [[6329032, 6915564], [6838426, 7081353]],
            3,
        ),
        (
            24,
            32,
            [[337, 399], [368, 382]],
            [1629, 1628],
            [2435, 2362],
            [258, 237],
            64,
            165876268,
            219204307,
            -47100,
            [[6336274, 6964913], [6874089, 7133515]],
            2,
        ),
        (
            26,
            34,
            [[374, 417], [383, 381]],
            [1729, 1524],
            [2542, 2215],
            [253, 226],
            64,
            165876268,
            219352760,
            18861,
            [[6353341, 7041671], [6919359, 7142873]],
            5,
        ),
        (
            26,
            28,
            [[274, 368], [410, 459]],
            [1424, 1831],
            [2273, 2532],
            [246, 218],
            64,
            165876268,
            219531809,
            94751,
            [[6364080, 7137867], [6958321, 7176025]],
            2,
        ),
        (
            27,
            33,
            [[388, 468], [376, 423]],
            [1675, 1578],
            [2452, 2276],
            [237, 240],
            64,
            165876268,
            219721108,
            -89710,
            [[6388482, 7210210], [6990489, 7236411]],
            3,
        ),
        (
            22,
            32,
            [[324, 433], [389, 445]],
            [1625, 1629],
            [2469, 2384],
            [246, 244],
            64,
            165876268,
            219817751,
            -89739,
            [[6394934, 7266004], [7002609, 7258688]],
            6,
        ),
        (
            27,
            32,
            [[318, 451], [402, 455]],
            [1631, 1627],
            [2429, 2341],
            [245, 220],
            64,
            165876268,
            219940884,
            -54005,
            [[6391080, 7318988], [7019449, 7315851]],
            3,
        ),
        (
            29,
            33,
            [[388, 488], [368, 425]],
            [1676, 1579],
            [2507, 2335],
            [255, 247],
            63,
            165876268,
            220075320,
            -102696,
            [[6406692, 7379666], [7038021, 7355425]],
            6,
        ),
        (
            27,
            37,
            [[420, 557], [363, 434]],
            [1883, 1375],
            [2676, 2137],
            [246, 257],
            64,
            165876268,
            220285097,
            -68709,
            [[6438068, 7466633], [7052093, 7432787]],
            3,
        ),
        (
            27,
            35,
            [[373, 558], [390, 489]],
            [1782, 1472],
            [2556, 2218],
            [235, 253],
            64,
            165876268,
            220445664,
            19139,
            [[6455937, 7538516], [7064024, 7491671]],
            3,
        ),
        (
            22,
            33,
            [[423, 589], [407, 449]],
            [1678, 1572],
            [2507, 2237],
            [245, 248],
            64,
            165876268,
            220576618,
            94549,
            [[6475903, 7637650], [7073584, 7493965]],
            4,
        ),
        (
            21,
            38,
            [[420, 663], [358, 412]],
            [1932, 1322],
            [2714, 2038],
            [250, 257],
            64,
            165876268,
            220815374,
            -89722,
            [[6476446, 7829735], [7091712, 7521965]],
            3,
        ),
        (
            22,
            31,
            [[341, 604], [488, 556]],
            [1578, 1680],
            [2380, 2392],
            [234, 264],
            64,
            165876268,
            221079718,
            -58752,
            [[6476446, 7940323], [7160917, 7606516]],
            2,
        ),
        (
            22,
            34,
            [[405, 698], [432, 536]],
            [1726, 1524],
            [2459, 2260],
            [279, 273],
            64,
            165876268,
            221255619,
            53131,
            [[6476446, 8064734], [7165201, 7653722]],
            1,
        ),
        (
            28,
            30,
            [[342, 675], [464, 632]],
            [1525, 1730],
            [2290, 2443],
            [260, 218],
            64,
            165876268,
            221534139,
            28222,
            [[6476446, 8206760], [7165201, 7790216]],
            2,
        ),
        (
            27,
            29,
            [[333, 639], [478, 665]],
            [1472, 1781],
            [2270, 2465],
            [238, 241],
            64,
            165876268,
            221712768,
            36996,
            [[6476446, 8286983], [7182883, 7870940]],
            5,
        ),
        (
            24,
            32,
            [[349, 741], [436, 566]],
            [1627, 1629],
            [2420, 2402],
            [282, 242],
            64,
            165876268,
            221885795,
            -69713,
            [[6476446, 8389975], [7195128, 7928730]],
            3,
        ),
        (
            26,
            29,
            [[352, 677], [553, 660]],
            [1470, 1780],
            [2225, 2460],
            [249, 245],
            64,
            165876268,
            221970702,
            -32899,
            [[6476446, 8425420], [7211184, 7962136]],
            2,
        ),
    ],
];

pub(crate) const REINFORCED_TRACES_1024: [u64; 3] =
    [0x9474c8083ab4ae4f, 0x4d214bd78e3b9be5, 0x0290191298d465cd];

pub(crate) const REINFORCED_COMPOSITIONS_1024: [&[Composition]; 3] = [
    &[
        (
            [[3819, 11657], [6673, 6777]],
            [
                [
                    [218067, -5236, 323929, -506973],
                    [237656, -5604, 344679, -492098],
                ],
                [
                    [226935, -5372, 282374, -464126],
                    [211112, -4712, 330912, -447106],
                ],
            ],
            [[1099, 1649], [1009, 1649], [3350, 0]],
            54,
            15036548761748942277,
        ),
        (
            [[-1837, 6694], [-1006, -6933]],
            [
                [
                    [194106, -4863, 316927, -492413],
                    [243666, -5648, 325275, -451592],
                ],
                [
                    [237883, -6528, 299527, -459856],
                    [231576, -6020, 319871, -488675],
                ],
            ],
            [[984, 1594], [1007, 1636], [3354, 0]],
            50,
            2571468004623703461,
        ),
        (
            [[-8315, 8375], [18628, 7308]],
            [
                [
                    [210528, -5421, 328304, -476309],
                    [225922, -4555, 340589, -465695],
                ],
                [
                    [219099, -5324, 290156, -442727],
                    [260758, -7165, 370242, -499693],
                ],
            ],
            [[966, 1613], [930, 1645], [3355, 1]],
            57,
            18249067392286807042,
        ),
        (
            [[-5001, 3005], [6306, 5403]],
            [
                [
                    [178652, -8286, 325227, -486168],
                    [217605, -5256, 318348, -488386],
                ],
                [
                    [264531, -4892, 319613, -494699],
                    [296985, -6518, 370608, -554965],
                ],
            ],
            [[1036, 1668], [1069, 1726], [3367, 0]],
            52,
            16149683004068638369,
        ),
        (
            [[3230, 10114], [12754, 4988]],
            [
                [
                    [217291, -6514, 329654, -476117],
                    [250380, -8538, 334919, -453503],
                ],
                [
                    [236725, -5332, 297600, -444612],
                    [305958, -6292, 368368, -471315],
                ],
            ],
            [[1047, 1645], [955, 1742], [3353, 0]],
            60,
            4838038999898272206,
        ),
        (
            [[725, 4929], [217, -2961]],
            [
                [
                    [298949, -5522, 363682, -506749],
                    [263972, -6167, 359234, -508795],
                ],
                [
                    [196720, -3639, 260813, -389472],
                    [237945, -3718, 363355, -480303],
                ],
            ],
            [[950, 1758], [988, 1749], [3362, 0]],
            59,
            2630245420733550210,
        ),
        (
            [[5588, 1777], [-2356, 5646]],
            [
                [
                    [247835, -6985, 364535, -510542],
                    [236533, -3772, 300074, -471276],
                ],
                [
                    [245484, -3438, 312710, -480087],
                    [309727, -9265, 419893, -545524],
                ],
            ],
            [[1018, 1635], [977, 1736], [3347, 0]],
            58,
            17274721750952142758,
        ),
        (
            [[-3724, 9032], [8121, 2523]],
            [
                [
                    [294071, -6225, 378520, -530779],
                    [262562, -6944, 348128, -499083],
                ],
                [
                    [222392, -6040, 306873, -474957],
                    [302408, -8237, 431892, -550457],
                ],
            ],
            [[1046, 1717], [1010, 1798], [3351, 0]],
            59,
            18127218763834056659,
        ),
        (
            [[4779, 10891], [-8766, 7976]],
            [
                [
                    [334837, -6433, 418035, -515656],
                    [286393, -7368, 380594, -515308],
                ],
                [
                    [219527, -4711, 287265, -456639],
                    [300373, -5851, 434786, -505386],
                ],
            ],
            [[1010, 1824], [1001, 1876], [3349, 0]],
            62,
            12706958741913770889,
        ),
        (
            [[5319, 4146], [6529, 7278]],
            [
                [
                    [266934, -3904, 376411, -562888],
                    [219619, -4425, 338822, -444291],
                ],
                [
                    [228009, -4835, 300098, -487135],
                    [377555, -9245, 468551, -570167],
                ],
            ],
            [[986, 1692], [997, 1845], [3362, 0]],
            60,
            10991343384262718516,
        ),
        (
            [[22482, 15746], [10166, 11435]],
            [
                [
                    [317279, -8683, 459113, -616922],
                    [263864, -4089, 368004, -495258],
                ],
                [
                    [206743, -3847, 254962, -425709],
                    [349438, -6997, 429657, -536221],
                ],
            ],
            [[955, 1826], [945, 1974], [3350, 0]],
            61,
            2436636863731234283,
        ),
        (
            [[5666, 1843], [-5996, 14845]],
            [
                [
                    [327152, -9946, 451972, -629204],
                    [245071, -9217, 352088, -489894],
                ],
                [
                    [212964, -6167, 313245, -473137],
                    [347939, -9661, 543369, -577064],
                ],
            ],
            [[1029, 1866], [1005, 1948], [3361, 0]],
            63,
            4722633333217011147,
        ),
        (
            [[1682, 848], [11301, 18818]],
            [
                [
                    [337550, -10972, 477100, -628493],
                    [248898, -6656, 350291, -495356],
                ],
                [
                    [216118, -4604, 303636, -445580],
                    [390812, -12490, 488856, -590557],
                ],
            ],
            [[1024, 1841], [985, 1963], [3362, 1]],
            63,
            5516763122217089513,
        ),
        (
            [[9858, 5054], [2563, 9786]],
            [
                [
                    [415864, -15036, 542099, -630868],
                    [257604, -6067, 366696, -526897],
                ],
                [
                    [209581, -4627, 293810, -479079],
                    [388367, -17924, 504271, -590186],
                ],
            ],
            [[1032, 2011], [989, 2030], [3362, 0]],
            64,
            1784854241759486201,
        ),
        (
            [[21287, 3078], [7829, 18526]],
            [
                [
                    [483519, -15305, 581986, -741187],
                    [291077, -6534, 370588, -545967],
                ],
                [
                    [186824, -4451, 291995, -461584],
                    [340904, -12686, 536360, -593744],
                ],
            ],
            [[1025, 2097], [1031, 1960], [3357, 0]],
            61,
            3529766411703680106,
        ),
        (
            [[18725, 14175], [2967, 22117]],
            [
                [
                    [476865, -16587, 548021, -644304],
                    [288442, -5954, 360132, -513803],
                ],
                [
                    [193785, -4432, 293768, -433099],
                    [369572, -9218, 532679, -622121],
                ],
            ],
            [[992, 2121], [1000, 2084], [3364, 0]],
            64,
            6452870744085959934,
        ),
        (
            [[15520, 2206], [6515, 23575]],
            [
                [
                    [469368, -14832, 614885, -732662],
                    [266929, -6927, 356831, -516942],
                ],
                [
                    [194747, -4201, 300329, -452474],
                    [413287, -8078, 559143, -659259],
                ],
            ],
            [[984, 2146], [1015, 2139], [3361, 0]],
            63,
            13327792463900867273,
        ),
        (
            [[1250, 7230], [4522, 22774]],
            [
                [
                    [574012, -16870, 644643, -780879],
                    [334589, -9588, 360908, -535244],
                ],
                [
                    [178687, -4765, 272966, -443093],
                    [357905, -12896, 534455, -651086],
                ],
            ],
            [[1067, 2279], [1046, 2117], [3358, 0]],
            61,
            2547686857498946268,
        ),
        (
            [[14517, 10262], [11562, 20536]],
            [
                [
                    [467888, -16383, 551716, -689252],
                    [263999, -8246, 353363, -485450],
                ],
                [
                    [245598, -5138, 300475, -477610],
                    [487215, -14090, 634266, -730978],
                ],
            ],
            [[1004, 2171], [1026, 2184], [3347, 0]],
            64,
            862246602657320560,
        ),
        (
            [[22774, 13430], [5300, 25429]],
            [
                [
                    [539610, -19447, 612899, -772848],
                    [301505, -6769, 339990, -517841],
                ],
                [
                    [212583, -5806, 326196, -478625],
                    [480019, -17587, 623476, -719294],
                ],
            ],
            [[1092, 2334], [1054, 2286], [3337, 0]],
            63,
            13071417196332166262,
        ),
        (
            [[22813, 18463], [8612, 22282]],
            [
                [
                    [493137, -14070, 592808, -710718],
                    [264734, -6281, 357170, -463917],
                ],
                [
                    [237624, -6252, 319203, -490706],
                    [571118, -18234, 667215, -783992],
                ],
            ],
            [[1046, 2197], [991, 2400], [3365, 0]],
            62,
            2264810445104752471,
        ),
        (
            [[25014, 5736], [3937, 34328]],
            [
                [
                    [469970, -21376, 660150, -729023],
                    [243363, -5729, 351495, -475560],
                ],
                [
                    [258094, -4846, 305998, -506229],
                    [638085, -12279, 659363, -751295],
                ],
            ],
            [[984, 2320], [964, 2497], [3327, 0]],
            64,
            4887776617019058337,
        ),
        (
            [[21940, 14506], [4493, 22881]],
            [
                [
                    [524852, -17661, 667527, -840722],
                    [265039, -7986, 353501, -527898],
                ],
                [
                    [231525, -6355, 338756, -490233],
                    [557172, -18253, 680454, -789240],
                ],
            ],
            [[1092, 2310], [1050, 2433], [3373, 0]],
            63,
            18055424635222886545,
        ),
        (
            [[26739, 3505], [5595, 18612]],
            [
                [
                    [508637, -12782, 617548, -750862],
                    [250460, -5450, 339050, -491882],
                ],
                [
                    [243084, -4229, 311554, -501614],
                    [614747, -16090, 728563, -884754],
                ],
            ],
            [[1049, 2354], [1053, 2467], [3353, 1]],
            63,
            16516092726070875228,
        ),
    ],
    &[
        (
            [[479, 3665], [1690, 11096]],
            [
                [
                    [215607, -5251, 318967, -506867],
                    [242512, -5625, 346163, -495080],
                ],
                [
                    [229625, -5392, 285358, -463223],
                    [208102, -4782, 325493, -442747],
                ],
            ],
            [[1100, 1649], [1009, 1652], [3350, 0]],
            53,
            17346928473575302743,
        ),
        (
            [[-2900, 5957], [54, -3970]],
            [
                [
                    [183439, -4852, 309997, -490092],
                    [248103, -5904, 340993, -457611],
                ],
                [
                    [248964, -6569, 310192, -464168],
                    [224425, -5615, 310621, -482284],
                ],
            ],
            [[988, 1572], [1008, 1623], [3354, 0]],
            48,
            1072871152948510114,
        ),
        (
            [[-7700, 4973], [8807, 11410]],
            [
                [
                    [202988, -5689, 321298, -468762],
                    [250617, -5678, 369476, -486159],
                ],
                [
                    [229286, -5676, 297560, -443976],
                    [251392, -7157, 343381, -482558],
                ],
            ],
            [[967, 1606], [932, 1644], [3353, 1]],
            56,
            6026122658691073754,
        ),
        (
            [[-5808, 3186], [1109, 13405]],
            [
                [
                    [164043, -7724, 315201, -473283],
                    [238741, -5305, 357475, -513874],
                ],
                [
                    [294092, -6943, 337489, -505979],
                    [277483, -6240, 334354, -532274],
                ],
            ],
            [[1039, 1692], [1071, 1717], [3367, 0]],
            48,
            15580442499966287363,
        ),
        (
            [[2226, 2175], [9484, 13909]],
            [
                [
                    [202979, -6505, 315866, -465420],
                    [300998, -9780, 387183, -478722],
                ],
                [
                    [281124, -6246, 334923, -475502],
                    [279665, -5393, 323810, -460444],
                ],
            ],
            [[1048, 1665], [953, 1759], [3353, 0]],
            58,
            3547722734443959921,
        ),
        (
            [[764, -258], [-3972, -2616]],
            [
                [
                    [260153, -5586, 333287, -484729],
                    [337396, -8404, 451779, -579952],
                ],
                [
                    [243628, -3438, 296406, -417890],
                    [206586, -2892, 301486, -437845],
                ],
            ],
            [[947, 1766], [988, 1802], [3363, 0]],
            58,
            14321502010761633522,
        ),
        (
            [[9026, 110], [-3942, 5441]],
            [
                [
                    [191069, -5495, 314201, -463928],
                    [323792, -4451, 409329, -546213],
                ],
                [
                    [300455, -3754, 359605, -531459],
                    [265759, -5337, 348968, -502532],
                ],
            ],
            [[1016, 1600], [978, 1766], [3346, 0]],
            59,
            2458654573236507887,
        ),
        (
            [[-3762, 3085], [4317, 12504]],
            [
                [
                    [215517, -4097, 306168, -478961],
                    [332833, -9442, 473881, -593943],
                ],
                [
                    [306087, -8494, 376488, -516737],
                    [245228, -6194, 344617, -499468],
                ],
            ],
            [[1045, 1723], [1019, 1841], [3351, 0]],
            59,
            9398226789374585406,
        ),
        (
            [[5617, 8716], [-13611, 17031]],
            [
                [
                    [252141, -4306, 325970, -461091],
                    [401131, -10675, 542609, -628919],
                ],
                [
                    [278178, -5507, 367280, -525415],
                    [232765, -3687, 334904, -448942],
                ],
            ],
            [[1019, 1771], [1023, 1876], [3348, 0]],
            61,
            4618149336319272221,
        ),
        (
            [[-391, 18885], [216, 7486]],
            [
                [
                    [183441, -2342, 273308, -457234],
                    [346123, -7742, 512988, -584376],
                ],
                [
                    [335267, -7940, 413582, -565979],
                    [271913, -6687, 350924, -492828],
                ],
            ],
            [[986, 1700], [1008, 1859], [3362, 0]],
            60,
            4762281915269386385,
        ),
        (
            [[7149, 6667], [3994, 3725]],
            [
                [
                    [207522, -4865, 331249, -495714],
                    [446032, -10394, 550410, -650706],
                ],
                [
                    [311653, -7258, 349822, -497496],
                    [237006, -3403, 311231, -452527],
                ],
            ],
            [[938, 1729], [955, 2072], [3348, 1]],
            61,
            9674942047717775421,
        ),
        (
            [[-318, 1995], [9839, 10627]],
            [
                [
                    [203098, -6827, 309844, -489193],
                    [441137, -17389, 611166, -727400],
                ],
                [
                    [334523, -9899, 442465, -577393],
                    [235504, -4835, 375443, -483266],
                ],
            ],
            [[1031, 1841], [1013, 2032], [3360, 0]],
            62,
            18204928859625416726,
        ),
        (
            [[-7919, 9714], [7879, 21489]],
            [
                [
                    [180584, -7290, 321428, -510474],
                    [490716, -15144, 628861, -705289],
                ],
                [
                    [344792, -8375, 439584, -574134],
                    [251551, -5410, 324054, -481677],
                ],
            ],
            [[1027, 1819], [1003, 2099], [3361, 1]],
            63,
            6300352944478699380,
        ),
        (
            [[12249, 21152], [14267, 8189]],
            [
                [
                    [237281, -7068, 322524, -497699],
                    [510397, -21396, 697605, -807158],
                ],
                [
                    [339726, -8525, 434399, -559009],
                    [236097, -9582, 306194, -495043],
                ],
            ],
            [[1013, 1851], [1036, 2165], [3365, 0]],
            62,
            4747114063805132591,
        ),
        (
            [[11387, 28304], [10499, 6092]],
            [
                [
                    [242826, -8412, 336416, -517980],
                    [620445, -18254, 751254, -854796],
                ],
                [
                    [312611, -10693, 434188, -568649],
                    [202683, -5177, 322428, -441316],
                ],
            ],
            [[1004, 1848], [1032, 2213], [3355, 0]],
            61,
            11201915121492478147,
        ),
        (
            [[6041, 38155], [5026, 11059]],
            [
                [
                    [220878, -5953, 298560, -491873],
                    [600603, -21208, 755217, -812869],
                ],
                [
                    [333685, -7811, 448462, -518067],
                    [222178, -4355, 312193, -478600],
                ],
            ],
            [[978, 1899], [999, 2306], [3363, 0]],
            63,
            14546701495867470098,
        ),
        (
            [[3822, 22699], [17204, 762]],
            [
                [
                    [212559, -5376, 338503, -486276],
                    [602036, -18337, 801680, -909641],
                ],
                [
                    [355049, -12328, 477006, -596168],
                    [214898, -4461, 317029, -494210],
                ],
            ],
            [[970, 1885], [1007, 2276], [3359, 0]],
            60,
            16367764305608960127,
        ),
        (
            [[-11341, 33974], [20222, 12151]],
            [
                [
                    [263331, -4667, 333740, -514993],
                    [716625, -25676, 813535, -917219],
                ],
                [
                    [303927, -10905, 452216, -561889],
                    [192834, -5471, 301896, -451295],
                ],
            ],
            [[1056, 1931], [1051, 2404], [3359, 0]],
            62,
            8395685091893471768,
        ),
        (
            [[83, 37300], [16119, 12044]],
            [
                [
                    [213155, -6255, 262325, -457420],
                    [580369, -25025, 772978, -799513],
                ],
                [
                    [456452, -13645, 512410, -593673],
                    [251370, -5329, 339550, -510707],
                ],
            ],
            [[982, 1989], [1048, 2267], [3349, 0]],
            64,
            13189848228229492698,
        ),
        (
            [[9298, 51490], [15486, 10603]],
            [
                [
                    [236698, -7378, 317270, -523621],
                    [644739, -23856, 799982, -840216],
                ],
                [
                    [399855, -13347, 541934, -670247],
                    [231932, -7931, 341740, -494664],
                ],
            ],
            [[1085, 1979], [1073, 2415], [3337, 0]],
            61,
            16016424761914191652,
        ),
        (
            [[545, 24658], [23411, 10878]],
            [
                [
                    [190341, -4247, 286092, -467313],
                    [574801, -14896, 747398, -800426],
                ],
                [
                    [489620, -18400, 577552, -641958],
                    [265318, -5620, 343325, -506555],
                ],
            ],
            [[1015, 2019], [997, 2409], [3367, 0]],
            63,
            4431377502906861628,
        ),
        (
            [[6018, 39563], [24734, 14478]],
            [
                [
                    [194292, -5492, 280872, -438026],
                    [553495, -16789, 755868, -845035],
                ],
                [
                    [533821, -17594, 572165, -688918],
                    [300487, -6438, 348085, -517370],
                ],
            ],
            [[951, 2103], [980, 2472], [3330, 0]],
            64,
            5300827239671981763,
        ),
        (
            [[-3967, 58937], [10087, 6498]],
            [
                [
                    [192694, -5451, 310836, -508716],
                    [622609, -25605, 830593, -909478],
                ],
                [
                    [522609, -15972, 633804, -740213],
                    [242176, -5512, 359888, -494063],
                ],
            ],
            [[1088, 2152], [1026, 2508], [3374, 0]],
            64,
            2300913807397639989,
        ),
        (
            [[4371, 36100], [23206, 7597]],
            [
                [
                    [205661, -4050, 284766, -475610],
                    [584040, -20273, 753424, -883172],
                ],
                [
                    [584423, -17221, 658974, -801070],
                    [277438, -6669, 384349, -547113],
                ],
            ],
            [[1055, 2330], [1057, 2400], [3357, 1]],
            63,
            686020851557934111,
        ),
    ],
    &[
        (
            [[3079, 11464], [291, 8735]],
            [
                [
                    [216002, -5230, 320301, -505228],
                    [239263, -5620, 346493, -493813],
                ],
                [
                    [228022, -5376, 283259, -463790],
                    [209239, -4570, 328864, -445002],
                ],
            ],
            [[1098, 1643], [1008, 1647], [3350, 0]],
            53,
            15236450503883526646,
        ),
        (
            [[-2212, 7214], [368, -3764]],
            [
                [
                    [184326, -4825, 307676, -487766],
                    [240857, -5471, 326048, -451877],
                ],
                [
                    [251149, -6624, 306851, -466041],
                    [227227, -5864, 317556, -490179],
                ],
            ],
            [[985, 1581], [1007, 1619], [3354, 0]],
            51,
            8984340169713383918,
        ),
        (
            [[-7943, 7285], [13713, 250]],
            [
                [
                    [201383, -5748, 318950, -471160],
                    [229927, -4818, 344303, -472355],
                ],
                [
                    [232314, -5593, 295110, -441036],
                    [256066, -7124, 349089, -484533],
                ],
            ],
            [[965, 1605], [933, 1636], [3354, 1]],
            55,
            426570834934383897,
        ),
        (
            [[-7097, -52], [7395, 66]],
            [
                [
                    [164846, -6991, 312071, -473766],
                    [223570, -5198, 326896, -487184],
                ],
                [
                    [284577, -6310, 330988, -498928],
                    [288499, -6296, 354998, -542860],
                ],
            ],
            [[1034, 1673], [1070, 1717], [3368, 0]],
            50,
            8795450498359411776,
        ),
        (
            [[1330, -2683], [8400, -93]],
            [
                [
                    [206577, -6341, 321179, -464746],
                    [269891, -8220, 354316, -461431],
                ],
                [
                    [268489, -5896, 319589, -460495],
                    [300044, -6076, 348625, -467082],
                ],
            ],
            [[1048, 1650], [965, 1736], [3353, 0]],
            59,
            13003448747187484546,
        ),
        (
            [[741, 7584], [1774, 1607]],
            [
                [
                    [264806, -5335, 338194, -491226],
                    [291467, -6464, 386147, -523065],
                ],
                [
                    [240590, -3113, 288265, -405930],
                    [217825, -3283, 331360, -457453],
                ],
            ],
            [[951, 1754], [983, 1761], [3363, 0]],
            60,
            11583534627127096961,
        ),
        (
            [[5989, 6956], [-8003, 7143]],
            [
                [
                    [204090, -6687, 326624, -468799],
                    [268971, -3111, 343863, -495198],
                ],
                [
                    [276803, -3550, 333451, -499465],
                    [283927, -6672, 374945, -526451],
                ],
            ],
            [[1011, 1587], [973, 1726], [3347, 0]],
            60,
            3958694330184955602,
        ),
        (
            [[-2998, 8751], [8589, 17065]],
            [
                [
                    [237885, -4560, 321962, -494179],
                    [277101, -7708, 386757, -531050],
                ],
                [
                    [263783, -7132, 339070, -492670],
                    [268043, -6969, 380922, -517889],
                ],
            ],
            [[1042, 1671], [1016, 1773], [3351, 0]],
            58,
            5779611465417849826,
        ),
        (
            [[9066, 14230], [-10978, 16699]],
            [
                [
                    [268692, -5161, 343635, -472983],
                    [320980, -8367, 440380, -541389],
                ],
                [
                    [241618, -4837, 308894, -473571],
                    [267265, -4465, 370149, -472033],
                ],
            ],
            [[1012, 1739], [1013, 1818], [3348, 0]],
            62,
            1148010550240391199,
        ),
        (
            [[61, 13586], [-1449, 8446]],
            [
                [
                    [199364, -2182, 291620, -475243],
                    [245935, -6065, 390220, -475899],
                ],
                [
                    [269401, -6468, 341706, -528365],
                    [307560, -7952, 382382, -523002],
                ],
            ],
            [[978, 1617], [991, 1755], [3361, 0]],
            57,
            15416854259208552314,
        ),
        (
            [[10291, 19414], [14541, 6750]],
            [
                [
                    [231863, -5109, 350098, -515326],
                    [313509, -6735, 418589, -548080],
                ],
                [
                    [265395, -6460, 292079, -460860],
                    [288071, -4655, 353594, -478231],
                ],
            ],
            [[946, 1710], [942, 1946], [3351, 0]],
            57,
            11892995809050304065,
        ),
        (
            [[1387, -270], [-2387, 15004]],
            [
                [
                    [226857, -7438, 333765, -500204],
                    [294258, -10449, 439267, -569764],
                ],
                [
                    [252532, -8843, 346488, -506212],
                    [277171, -6753, 429886, -521777],
                ],
            ],
            [[1020, 1711], [1002, 1861], [3361, 0]],
            63,
            922379138131687760,
        ),
        (
            [[-936, 2106], [14984, 19601]],
            [
                [
                    [213527, -8474, 352681, -529846],
                    [328991, -8746, 443816, -565459],
                ],
                [
                    [269342, -5939, 351709, -488846],
                    [312168, -7616, 397403, -518198],
                ],
            ],
            [[1017, 1731], [986, 1953], [3362, 1]],
            61,
            10211252234319773758,
        ),
        (
            [[16501, 16805], [6661, -3146]],
            [
                [
                    [256793, -7919, 352149, -509699],
                    [326547, -9686, 462259, -601253],
                ],
                [
                    [251177, -5428, 330010, -496641],
                    [303315, -13285, 382644, -528751],
                ],
            ],
            [[1012, 1743], [1002, 1939], [3363, 0]],
            61,
            10296770055855482217,
        ),
        (
            [[5713, 3675], [16341, 14955]],
            [
                [
                    [298201, -9380, 373669, -537872],
                    [398420, -9398, 489915, -621750],
                ],
                [
                    [241912, -5988, 335725, -488383],
                    [273748, -8108, 425747, -495522],
                ],
            ],
            [[1014, 1767], [1033, 1928], [3356, 0]],
            62,
            1113478050617747008,
        ),
        (
            [[11786, 28225], [2290, 18765]],
            [
                [
                    [268846, -8046, 327653, -511689],
                    [389152, -10180, 491190, -584329],
                ],
                [
                    [242122, -6253, 345073, -454746],
                    [305776, -6241, 410476, -529347],
                ],
            ],
            [[972, 1789], [987, 2089], [3362, 0]],
            63,
            9979351342246730660,
        ),
        (
            [[6489, -192], [7875, 12766]],
            [
                [
                    [270123, -7624, 390265, -508978],
                    [398337, -11942, 514607, -660912],
                ],
                [
                    [257378, -9045, 346132, -496996],
                    [309132, -7118, 443696, -573040],
                ],
            ],
            [[970, 1785], [1005, 2055], [3360, 0]],
            62,
            10017868140145594790,
        ),
        (
            [[-13837, 1863], [5818, 23509]],
            [
                [
                    [301769, -4633, 362721, -541738],
                    [488361, -14018, 538141, -675334],
                ],
                [
                    [214479, -5961, 321340, -457683],
                    [276891, -8263, 411561, -517532],
                ],
            ],
            [[1030, 1788], [1035, 2114], [3358, 0]],
            62,
            8585987325298217291,
        ),
        (
            [[3272, 26362], [16857, 21461]],
            [
                [
                    [251253, -7621, 307861, -476286],
                    [410605, -17682, 544868, -618376],
                ],
                [
                    [326953, -7016, 368816, -487180],
                    [370574, -8337, 479004, -577883],
                ],
            ],
            [[971, 1814], [1029, 2093], [3348, 0]],
            62,
            5120516945479747889,
        ),
        (
            [[17353, 22788], [14599, 20006]],
            [
                [
                    [288030, -7329, 351781, -541298],
                    [460459, -14849, 555883, -673846],
                ],
                [
                    [271314, -5873, 401859, -528910],
                    [346177, -12967, 469929, -569533],
                ],
            ],
            [[1081, 1800], [1065, 2205], [3337, 0]],
            62,
            13062500970133695193,
        ),
        (
            [[5784, 27927], [13641, 6706]],
            [
                [
                    [234300, -5537, 320565, -474244],
                    [419711, -10494, 563303, -599898],
                ],
                [
                    [323449, -10584, 389147, -505156],
                    [406885, -10053, 491708, -621921],
                ],
            ],
            [[1006, 1728], [975, 2332], [3365, 0]],
            64,
            9442043580155334915,
        ),
        (
            [[15819, 35168], [14822, 33332]],
            [
                [
                    [239724, -6468, 331197, -445625],
                    [411451, -11554, 578280, -671784],
                ],
                [
                    [324610, -8860, 363612, -519598],
                    [446833, -10930, 511748, -589403],
                ],
            ],
            [[958, 1760], [972, 2388], [3327, 0]],
            64,
            15320121413049906015,
        ),
        (
            [[-1148, 39689], [6713, 6500]],
            [
                [
                    [225987, -6232, 337684, -514511],
                    [476948, -17276, 615077, -699978],
                ],
                [
                    [299515, -8261, 410880, -541377],
                    [377696, -11309, 522277, -647900],
                ],
            ],
            [[1066, 1726], [1032, 2355], [3373, 0]],
            63,
            13997241649692441661,
        ),
        (
            [[-1476, 31084], [10145, 4481]],
            [
                [
                    [238068, -5251, 309056, -478119],
                    [431780, -14327, 566208, -688914],
                ],
                [
                    [351302, -6750, 396390, -540839],
                    [428037, -11525, 565392, -690645],
                ],
            ],
            [[1012, 1870], [1052, 2335], [3355, 1]],
            63,
            15195799509969962823,
        ),
    ],
];

pub(crate) const REINFORCED_EARNED_1024: [&[EarnedBlock]; 3] = [
    &[
        (
            [[15, 14, 5], [11, 16, 3]],
            31,
            [[10986, 0], [0, 56887]],
            -44931,
            20605,
        ),
        (
            [[16, 14, 1], [13, 16, 4]],
            32,
            [[15895, 0], [0, 44846]],
            43010,
            108546,
        ),
        (
            [[10, 19, 2], [13, 16, 4]],
            26,
            [[40462, 0], [0, 58298]],
            -28378,
            -93914,
        ),
        (
            [[7, 18, 3], [9, 22, 5]],
            29,
            [[4, 0], [0, 74509]],
            37603,
            103139,
        ),
        (
            [[13, 13, 4], [8, 24, 2]],
            37,
            [[51929, 0], [0, 96627]],
            28324,
            93860,
        ),
        (
            [[20, 11, 5], [9, 17, 2]],
            37,
            [[101805, 0], [0, 60565]],
            -20582,
            44954,
        ),
        (
            [[16, 12, 2], [9, 25, 0]],
            41,
            [[74448, 0], [0, 91892]],
            -18777,
            46759,
        ),
        (
            [[20, 8, 4], [5, 26, 1]],
            46,
            [[91849, 0], [0, 96677]],
            -13647,
            -79183,
        ),
        (
            [[27, 4, 3], [7, 23, 0]],
            50,
            [[157364, 0], [0, 111939]],
            46617,
            112153,
        ),
        (
            [[18, 8, 2], [3, 32, 1]],
            50,
            [[63476, 0], [0, 155743]],
            -18160,
            47376,
        ),
        (
            [[28, 4, 1], [1, 30, 0]],
            58,
            [[94396, 0], [0, 124852]],
            -4481,
            61055,
        ),
        (
            [[26, 4, 2], [1, 30, 1]],
            56,
            [[103229, 0], [0, 95217]],
            46683,
            112219,
        ),
        (
            [[26, 5, 1], [0, 32, 0]],
            58,
            [[84439, 0], [0, 130255]],
            46667,
            112203,
        ),
        (
            [[29, 2, 2], [1, 30, 0]],
            59,
            [[166779, 0], [0, 79710]],
            46683,
            112219,
        ),
        (
            [[35, 1, 1], [2, 25, 0]],
            60,
            [[180387, 0], [0, 63170]],
            46689,
            112225,
        ),
        (
            [[33, 2, 0], [1, 27, 1]],
            60,
            [[158734, 0], [0, 92147]],
            44671,
            110207,
        ),
        (
            [[32, 1, 0], [0, 31, 0]],
            63,
            [[174482, 0], [0, 120015]],
            46019,
            111555,
        ),
        (
            [[38, 0, 0], [0, 26, 0]],
            64,
            [[183307, 0], [0, 76428]],
            46691,
            112227,
        ),
        (
            [[31, 0, 0], [0, 33, 0]],
            64,
            [[98592, 0], [0, 105626]],
            46691,
            112227,
        ),
        (
            [[34, 0, 0], [0, 30, 0]],
            64,
            [[110688, 0], [0, 143679]],
            46691,
            112227,
        ),
        (
            [[30, 0, 0], [0, 34, 0]],
            64,
            [[118412, 0], [0, 154833]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            64,
            [[95225, 0], [0, 169513]],
            46691,
            112227,
        ),
        (
            [[32, 0, 0], [0, 32, 0]],
            64,
            [[47112, 0], [0, 98895]],
            46691,
            112227,
        ),
        (
            [[29, 0, 0], [0, 35, 0]],
            64,
            [[65517, 0], [0, 66847]],
            46691,
            112227,
        ),
    ],
    &[
        (
            [[14, 16, 4], [12, 16, 2]],
            28,
            [[0, 55258], [23690, 0]],
            24175,
            -41361,
        ),
        (
            [[10, 17, 4], [14, 17, 2]],
            31,
            [[0, 78801], [45745, 0]],
            -44591,
            -110127,
        ),
        (
            [[7, 21, 3], [17, 14, 2]],
            38,
            [[0, 82633], [62419, 0]],
            28383,
            93919,
        ),
        (
            [[4, 23, 1], [16, 13, 7]],
            39,
            [[0, 61923], [65465, 0]],
            -30710,
            34826,
        ),
        (
            [[5, 23, 2], [16, 15, 3]],
            39,
            [[0, 153316], [81582, 0]],
            -16428,
            -81964,
        ),
        (
            [[6, 28, 2], [16, 10, 2]],
            44,
            [[0, 140303], [100886, 0]],
            44661,
            110197,
        ),
        (
            [[1, 28, 1], [25, 6, 3]],
            53,
            [[0, 119922], [80701, 0]],
            44671,
            -20865,
        ),
        (
            [[5, 27, 0], [19, 10, 3]],
            46,
            [[0, 126205], [80801, 0]],
            29216,
            94752,
        ),
        (
            [[3, 30, 1], [23, 5, 2]],
            53,
            [[0, 208473], [93917, 0]],
            -30651,
            34885,
        ),
        (
            [[0, 27, 1], [24, 11, 1]],
            51,
            [[0, 158892], [101321, 0]],
            46691,
            112227,
        ),
        (
            [[2, 30, 1], [23, 6, 2]],
            53,
            [[0, 171124], [117812, 0]],
            46691,
            112227,
        ),
        (
            [[0, 32, 0], [25, 4, 3]],
            57,
            [[0, 192455], [85269, 0]],
            40541,
            106077,
        ),
        (
            [[0, 32, 0], [30, 1, 1]],
            62,
            [[0, 175315], [112666, 0]],
            46467,
            112003,
        ),
        (
            [[0, 33, 0], [26, 3, 2]],
            59,
            [[0, 144392], [70549, 0]],
            46019,
            111555,
        ),
        (
            [[0, 37, 0], [24, 1, 2]],
            61,
            [[0, 168821], [66689, 0]],
            46691,
            112227,
        ),
        (
            [[0, 35, 0], [28, 1, 0]],
            63,
            [[0, 164088], [112133, 0]],
            46691,
            112227,
        ),
        (
            [[0, 33, 0], [30, 1, 0]],
            63,
            [[0, 128776], [95317, 0]],
            46617,
            112153,
        ),
        (
            [[0, 38, 0], [25, 1, 0]],
            63,
            [[0, 145976], [73457, 0]],
            -2461,
            63075,
        ),
        (
            [[0, 31, 0], [32, 1, 0]],
            63,
            [[0, 103039], [134869, 0]],
            46691,
            112227,
        ),
        (
            [[0, 34, 0], [30, 0, 0]],
            64,
            [[0, 113961], [111260, 0]],
            46691,
            112227,
        ),
        (
            [[0, 30, 0], [32, 1, 1]],
            62,
            [[0, 102723], [181139, 0]],
            44671,
            110207,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            64,
            [[0, 52717], [120641, 0]],
            46691,
            112227,
        ),
        (
            [[0, 32, 0], [32, 0, 0]],
            64,
            [[0, 62157], [171549, 0]],
            46691,
            112227,
        ),
        (
            [[0, 29, 0], [35, 0, 0]],
            64,
            [[0, 68728], [142366, 0]],
            46691,
            112227,
        ),
    ],
    &[
        (
            [[14, 15, 5], [12, 16, 2]],
            27,
            [[-9107, 13517], [17953, 24008]],
            40423,
            105959,
        ),
        (
            [[13, 13, 5], [15, 17, 1]],
            24,
            [[3825, 7725], [38249, 20380]],
            18182,
            -47354,
        ),
        (
            [[9, 20, 2], [17, 16, 0]],
            32,
            [[7141, 16698], [36110, 56116]],
            -16386,
            49150,
        ),
        (
            [[4, 21, 3], [12, 23, 1]],
            33,
            [[495, 11009], [19454, 57733]],
            46016,
            111552,
        ),
        (
            [[10, 17, 3], [12, 19, 3]],
            39,
            [[22052, 94267], [70276, 64582]],
            40507,
            -25029,
        ),
        (
            [[13, 20, 3], [14, 12, 2]],
            26,
            [[22783, 35856], [37981, -1330]],
            -46501,
            19035,
        ),
        (
            [[11, 18, 1], [17, 15, 2]],
            37,
            [[32291, 37881], [34198, 44394]],
            28988,
            94524,
        ),
        (
            [[9, 22, 1], [16, 15, 1]],
            33,
            [[7242, 49349], [35663, 52162]],
            18436,
            -47100,
        ),
        (
            [[12, 21, 1], [12, 14, 4]],
            33,
            [[17067, 76758], [45270, 9358]],
            -46675,
            18861,
        ),
        (
            [[7, 21, 0], [15, 19, 2]],
            33,
            [[10739, 96196], [38962, 33152]],
            29215,
            94751,
        ),
        (
            [[7, 24, 2], [10, 20, 1]],
            37,
            [[24402, 72343], [32168, 60386]],
            -24174,
            -89710,
        ),
        (
            [[4, 25, 3], [11, 18, 3]],
            32,
            [[6452, 55794], [12120, 22277]],
            -24203,
            -89739,
        ),
        (
            [[6, 24, 2], [10, 21, 1]],
            31,
            [[-3854, 52984], [16840, 57163]],
            11531,
            -54005,
        ),
        (
            [[8, 22, 3], [7, 21, 3]],
            32,
            [[15612, 60678], [18572, 39574]],
            -37160,
            -102696,
        ),
        (
            [[7, 28, 2], [6, 20, 1]],
            35,
            [[31376, 86967], [14072, 77362]],
            -3173,
            -68709,
        ),
        (
            [[4, 29, 2], [5, 23, 1]],
            29,
            [[17869, 71883], [11931, 58884]],
            -46397,
            19139,
        ),
        (
            [[4, 28, 1], [10, 18, 3]],
            27,
            [[19966, 99134], [9560, 2294]],
            29013,
            94549,
        ),
        (
            [[5, 31, 2], [9, 16, 1]],
            33,
            [[543, 192085], [18128, 28000]],
            -24186,
            -89722,
        ),
        (
            [[0, 30, 1], [10, 22, 1]],
            32,
            [[0, 110588], [69205, 84551]],
            6784,
            -58752,
        ),
        (
            [[1, 33, 0], [8, 21, 1]],
            32,
            [[0, 124411], [4284, 47206]],
            -12405,
            53131,
        ),
        (
            [[0, 30, 0], [4, 28, 2]],
            37,
            [[0, 142026], [0, 136494]],
            -37314,
            28222,
        ),
        (
            [[0, 29, 0], [3, 27, 5]],
            33,
            [[0, 80223], [17682, 80724]],
            -28540,
            36996,
        ),
        (
            [[0, 31, 1], [6, 24, 2]],
            32,
            [[0, 102992], [12245, 57790]],
            -4177,
            -69713,
        ),
        (
            [[0, 29, 0], [7, 26, 2]],
            33,
            [[0, 35445], [16056, 33406]],
            32637,
            -32899,
        ),
    ],
];

pub(crate) const REINFORCED_READ_1024: [u64; 3] =
    [0x441addd48f1caf0b, 0x0424f857bd0976b5, 0x4132717393c1a6b5];

pub(crate) const REINFORCED_CENSUS_1024: [&[(u32, u64)]; 3] = [
    &[
        (1, 3183),
        (2, 24366),
        (3, 37989),
        (4, 11659),
        (5, 686),
        (6, 133),
        (7, 36),
        (8, 13),
        (9, 6),
        (10, 3),
        (12, 3),
        (16, 1),
    ],
    &[
        (1, 3174),
        (2, 24325),
        (3, 38040),
        (4, 11664),
        (5, 685),
        (6, 125),
        (7, 39),
        (8, 15),
        (9, 5),
        (10, 3),
        (11, 1),
        (12, 3),
        (15, 1),
    ],
    &[
        (1, 3187),
        (2, 24351),
        (3, 37996),
        (4, 11664),
        (5, 687),
        (6, 131),
        (7, 37),
        (8, 13),
        (9, 7),
        (10, 2),
        (11, 1),
        (12, 3),
        (16, 1),
    ],
];

/// The verdict, as the rules compute it over the pinned tables, written from the run: the
/// criterion holds in both rewarded arms. **H-14 is yes** for this network, this delivery,
/// this regime, this rule and 1 536 trials.
pub(crate) const REINFORCED_1024: Reinforced = Reinforced {
    correct: [true, true],
    yes: true,
};

/// The correct selections over the last 128 trials, per rewarded arm, against `REWARDED_MIN`:
/// 128 of 128 in the assignment and 128 in the mirrored assignment.
pub(crate) const CORRECT_LAST_1024: [u32; 2] = [128, 128];

/// Where each arm's selection first passed 40 of 64 per block, as read: the assignment at
/// trial 448 (the seventh block), the mirrored assignment at 384 (the sixth), the mirrored
/// first as ADR-0080's Hypothesis said and earlier than its estimate of about trial 900; the
/// shuffled arm, whose correct trials read the assignment's answers, never.
pub(crate) const CROSSED_1024: [Option<usize>; 3] = [Some(448), Some(384), None];

/// The selections per stimulus over the last 128 trials, per arm, `[stimulus][readout 0,
/// readout 1, tie]`: every presentation to the answer in both rewarded arms (A 61 of 61, B 67
/// of 67); under the shuffled reward A to readout 1 in 60 of 61 with one tie, and B to
/// readout 1 in 50 of 67, to readout 0 in 13, with four ties.
pub(crate) const SPLITS_LAST_1024: [[[u32; 3]; 2]; 3] = [
    [[61, 0, 0], [0, 67, 0]],
    [[0, 61, 0], [67, 0, 0]],
    [[0, 60, 1], [13, 50, 4]],
];

/// The lock-in reading of the shuffled arm by the rule, per stimulus: neither stimulus goes
/// to one readout as often as the rewarded arms go to its answer (61 of 61 and 67 of 67) — A
/// falls one tie short at 60 of 61, and is read as locked onto readout 1 beside the rule; B
/// leans to readout 1 at 50 of 67.
pub(crate) const LOCKED_1024: [bool; 2] = [false, false];

/// The reach of each arm's delivery, `(inside the pairs it can reach, outside)`, as read:
/// every one of the assignment's 1 584 synapses (775 of A→R0 and 809 of B→R1) and of the
/// mirrored assignment's 1 604 (806 of A→R1 and 798 of B→R0) moved; under the shuffled reward
/// 3 183 of the four pairs' 3 188; no synapse outside the pairs in any arm.
pub(crate) const REINFORCED_REACH_1024: [(u64, u64); 3] = [(1584, 0), (1604, 0), (3183, 0)];

const _: () = assert!(
    REINFORCED_REACH_1024[0].0 as u32 == SYNAPSES_1024[0][0] + SYNAPSES_1024[1][1]
        && REINFORCED_REACH_1024[1].0 as u32 == SYNAPSES_1024[0][1] + SYNAPSES_1024[1][0]
        && REINFORCED_REACH_1024[2].0 as u32 + 5
            == SYNAPSES_1024[0][0]
                + SYNAPSES_1024[0][1]
                + SYNAPSES_1024[1][0]
                + SYNAPSES_1024[1][1]
);

/// ADR-0080's derivation as read on each arm, clause by clause: the signal at every trial's
/// end at most 0.712, below zero after every negative reward (295, 252 and 764 of 1 536), and
/// nothing consolidated in the trial after one — true on every arm.
pub(crate) const DERIVATION_1024: [[bool; 3]; 3] =
    [[true, true, true], [true, true, true], [true, true, true]];
