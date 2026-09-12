//! Brief 017's exit test (ADR-0032; whitepaper §8.8): three-factor plasticity as the executor
//! composes it. A presynaptic unit and a postsynaptic one, each made to fire by the injector,
//! with the modulation's baseline at 0: the pairings accumulate in the synapse's eligibility
//! trace and the weight does not move; a reward between ticks consolidates the pending trace
//! at the next presynaptic spike; a reward before any pairing consolidates nothing; a reward
//! long after the pairings consolidates less, because the trace decayed; the consolidated
//! trace is not consolidated again; the modulator is written to and read from the image and a
//! loaded engine continues identically; the baseline is refused outside $[0, 1]$. With the
//! baseline at 1.0 the rule is ADR-0022's, which `differential.rs` pins.

#![deny(clippy::arithmetic_side_effects)]

use cortex_core::{
    MODULATION_ONE_Q16, STP_MAX, STP_U, SynapseBlock, THRESHOLD_BASE, spike_message,
    synaptic_efficacy_q16,
};
use cortex_runtime::{Config, ConfigError, Executor, Image};

/// Unit 0 fans out to unit 1 through slot 0 of block 0: weight 1 000, one tick of delay.
const WEIGHT: i16 = 1000;
/// Messages that make an armed unit at rest fire.
const KICK: usize = 13;

fn config(baseline_q16: i32) -> Config {
    Config {
        workers: 2,
        units: 2,
        blocks: 1,
        nodes_per_worker: 64,
        injector_capacity: 64,
        modulation_baseline_q16: baseline_q16,
        ..Config::default()
    }
}

fn network(baseline_q16: i32) -> Executor<64> {
    let mut exec = Executor::<64>::new(config(baseline_q16)).expect("a valid configuration");
    assert!(exec.blocks_mut()[0].set_synapse(0, 1, WEIGHT, 1, false));
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
    assert!(exec.units_mut()[0].set_first_block(0));
    exec
}

/// Makes `unit` fire within the next few ticks, and runs until it has.
fn fire(exec: &mut Executor<64>, unit: u32) -> u32 {
    let before = exec.units()[unit as usize].last_soma_spike_tick;
    let inject = exec.injector();
    let strong = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..KICK {
        inject.inject(unit, spike_message(strong, false)).unwrap();
    }
    for _ in 0..40 {
        exec.tick();
        let now = exec.units()[unit as usize].last_soma_spike_tick;
        if now != before {
            return now;
        }
    }
    panic!("unit {unit} did not fire");
}

/// The synapse's weight and trace, between ticks.
fn synapse(exec: &Executor<64>) -> (i16, i16) {
    let block = &exec.blocks()[0];
    (block.weights_q1_15[0], block.eligibility_q1_15[0])
}

/// A presynaptic spike, the postsynaptic one twenty ticks later, then quiet until the next
/// pairing: returns the two spike ticks.
fn pairing(exec: &mut Executor<64>) -> (u32, u32) {
    let pre = fire(exec, 0);
    exec.run(20);
    let post = fire(exec, 1);
    exec.run(4000);
    (pre, post)
}

#[test]
fn pairings_accumulate_in_the_trace_and_a_reward_consolidates_them_at_the_next_spike() {
    let mut exec = network(0);
    // The rule as an oracle for the composition: the same spike ticks on a block of its own
    // give the trace the executor's block must hold.
    let mut oracle = SynapseBlock::new();
    assert!(oracle.set_synapse(0, 1, WEIGHT, 1, false));
    let mut post_last = 0;
    for _ in 0..3 {
        let (pre, post) = pairing(&mut exec);
        assert!(post > pre, "the post fired after the pre");
        oracle.step_stdp_all(pre, [post_last, 0, 0, 0]);
        oracle.consolidate_all(0);
        post_last = post;
    }
    let (weight, trace) = synapse(&exec);
    assert_eq!(
        weight, WEIGHT,
        "with the baseline at 0 no pairing reaches the weight"
    );
    assert!(
        trace > 300,
        "three presynaptic spikes, two pairings: {trace}"
    );
    assert_eq!(
        (weight, trace),
        (oracle.weights_q1_15[0], oracle.eligibility_q1_15[0]),
        "the executor pairs at the presynaptic spike against the target's last spike"
    );
    assert_eq!(exec.modulator().dopamine_rpe, 0, "no reward yet");

    // A reward between ticks, then the next presynaptic spike within a few ticks: the
    // modulation is near 1.0, so the whole pending trace moves into the weight, and what the
    // weight gained is exactly what the trace lost.
    assert_eq!(exec.reward(MODULATION_ONE_Q16), MODULATION_ONE_Q16);
    let pre = fire(&mut exec, 0);
    let (weight_after, trace_after) = synapse(&exec);
    oracle.step_stdp_all(pre, [post_last, 0, 0, 0]);
    let pending = oracle.eligibility_q1_15[0];
    assert_eq!(
        (weight_after as i32) - (WEIGHT as i32) + (trace_after as i32),
        pending as i32,
        "conserved: the weight gained what the trace lost"
    );
    assert!(
        trace_after <= 1 && weight_after > WEIGHT + 300,
        "consolidated nearly whole: weight {weight_after}, trace {trace_after}"
    );
    assert!(
        exec.modulator().dopamine_rpe < MODULATION_ONE_Q16,
        "the signal decays tick by tick"
    );

    // The consolidated trace is gone: a further presynaptic spike, with the signal still well
    // above rest, moves the weight by what it pairs now and not by the old trace again. No post
    // spike since the last one: potentiation against a post that fired before this block's
    // stamp is nothing, and after 20 000 ticks the depression window has closed.
    exec.run(20_000);
    assert!(
        exec.modulator().dopamine_rpe > 0x2000,
        "the signal is still well above rest"
    );
    fire(&mut exec, 0);
    let (weight_again, trace_again) = synapse(&exec);
    assert!(
        (weight_again as i32 - weight_after as i32).abs() <= 2 && trace_again.abs() <= 2,
        "nothing pending to consolidate twice: {weight_again}, {trace_again}"
    );
}

#[test]
fn a_reward_before_any_pairing_consolidates_nothing() {
    let mut exec = network(0);
    assert_eq!(exec.reward(MODULATION_ONE_Q16), MODULATION_ONE_Q16);
    // The signal decays to rest before the first pairing.
    exec.run(70_000);
    assert_eq!(exec.modulator().dopamine_rpe, 0, "at rest again");
    for _ in 0..3 {
        pairing(&mut exec);
    }
    let (weight, trace) = synapse(&exec);
    assert_eq!(weight, WEIGHT, "nothing was pending when the reward came");
    assert!(trace > 300, "the pairings are pending now: {trace}");
}

#[test]
fn a_late_reward_consolidates_less_than_a_prompt_one_because_the_trace_decayed() {
    let mut prompt = network(0);
    let mut late = network(0);
    for _ in 0..3 {
        pairing(&mut prompt);
        pairing(&mut late);
    }
    assert_eq!(synapse(&prompt), synapse(&late), "the same history so far");
    let (_, pending) = synapse(&prompt);
    // Two time constants of the trace before the late reward.
    late.run(131_072);
    assert_eq!(
        synapse(&late).1,
        pending,
        "a trace decays at the presynaptic spike, not by the clock"
    );
    assert_eq!(prompt.reward(MODULATION_ONE_Q16), MODULATION_ONE_Q16);
    assert_eq!(late.reward(MODULATION_ONE_Q16), MODULATION_ONE_Q16);
    fire(&mut prompt, 0);
    fire(&mut late, 0);
    let prompt_gain = synapse(&prompt).0 as i32 - WEIGHT as i32;
    let late_gain = synapse(&late).0 as i32 - WEIGHT as i32;
    assert!(
        prompt_gain > late_gain && late_gain > 0,
        "prompt {prompt_gain}, late {late_gain}: the pending trace decayed to a fraction before the late spike paired again"
    );
    assert!(
        late_gain < pending as i32,
        "less than what was pending: {late_gain} of {pending}"
    );
}

#[test]
fn the_modulator_is_written_to_and_read_from_the_image_and_a_loaded_engine_continues_alike() {
    let mut exec = network(0);
    for _ in 0..2 {
        pairing(&mut exec);
    }
    let at_rest = Image::encode(&exec).unwrap();
    assert_eq!(
        cortex_connectome::CortexFileHeader::decode(at_rest[0..64].try_into().unwrap())
            .section_count,
        4,
        "the modulation state and the homeostasis state are always written: the image defines the run"
    );
    assert_eq!(exec.reward(0x4000), 0x4000);
    let raised = Image::encode(&exec).unwrap();
    let header = cortex_connectome::CortexFileHeader::decode(raised[0..64].try_into().unwrap());
    assert_eq!(
        header.section_count, 4,
        "the modulator and homeostasis sections"
    );
    // The image's baseline outranks the configuration's: loaded under a configuration that
    // says 1.0, the engine keeps the 0 it was written with.
    let under_one = Image::decode::<64>(&raised, config(MODULATION_ONE_Q16)).unwrap();
    assert_eq!(under_one.modulation_baseline_q16(), 0);
    assert_eq!(
        header.tick_ns,
        cortex_core::TICK_NS,
        "the header says what a tick is"
    );
    let mut loaded = Image::decode::<64>(&raised, config(0)).unwrap();
    assert_eq!(loaded.modulator(), exec.modulator());
    assert_eq!(synapse(&loaded), synapse(&exec));
    assert_eq!(
        (loaded.ticks(), header.written_tick),
        (exec.ticks(), exec.ticks()),
        "the clock resumes where the image was written, so the stamps keep their meaning"
    );
    assert!(
        exec.ticks() > 8000,
        "a clock that could not be mistaken for a fresh one"
    );
    // Both continue with the same reward pending: the same consolidation at the next spike.
    let original = fire(&mut exec, 0);
    let copy = fire(&mut loaded, 0);
    assert_eq!(
        synapse(&loaded),
        synapse(&exec),
        "the same weight and trace after the same spike ({original} and {copy})"
    );
    assert_eq!(loaded.modulator(), exec.modulator());
}

#[test]
fn the_baseline_is_refused_outside_the_unit_interval_and_the_default_is_one() {
    assert!(matches!(
        Executor::<64>::new(config(-1)),
        Err(ConfigError::ModulationOutOfRange)
    ));
    assert!(matches!(
        Executor::<64>::new(config(MODULATION_ONE_Q16 + 1)),
        Err(ConfigError::ModulationOutOfRange)
    ));
    assert_eq!(
        Executor::<64>::new(config(0))
            .unwrap()
            .modulation_baseline_q16(),
        0
    );
    assert_eq!(
        Executor::<64>::new(config(MODULATION_ONE_Q16))
            .unwrap()
            .modulation_baseline_q16(),
        MODULATION_ONE_Q16
    );
    assert_eq!(
        Config::default().modulation_baseline_q16,
        MODULATION_ONE_Q16,
        "the default is ADR-0022's rule"
    );
    // With the baseline at 1.0 the pairing is consolidated at the spike that pairs it.
    let mut exec = network(MODULATION_ONE_Q16);
    for _ in 0..3 {
        pairing(&mut exec);
    }
    let (weight, trace) = synapse(&exec);
    assert!(weight > WEIGHT + 300, "the weight moved: {weight}");
    assert_eq!(trace, 0, "nothing pending");
}
