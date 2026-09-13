//! Brief 022's exit test (ADR-0044; whitepaper §8.8, §11.1): the reference network as the
//! runtime synthesizes, drives, forks and measures it. The prior of `cortex-connectome` is
//! written into an executor's arenas and read back whole through the image; a driven run is
//! bit-identical on one and four workers; at 256 units and fixed gains the lag-one estimate
//! of ADR-0036 is held beside the causal branching ratio the oracle attributes through the
//! connectome, gross and net; under a control step of an eighth the gain's trajectory is
//! pinned; a night of the stages of ADR-0037 with two episodes tagged (ADR-0038) moves the
//! synapses among each pattern by the pinned amount, and a cue of half a pattern on a fork
//! of the image after the night fires the rest of the local pattern and not of the random
//! one. The same harness at 1 024 units is the `exhaustive` test the weekly job runs.
//!
//! Every number here is the engine's own, pinned from one run and held on every worker
//! count and every architecture, as the determinism pin is; a deliberate change to the
//! dynamics moves them and says why. What these tests decide, and at what scale, is stated
//! in ADR-0044 and in the dispositions of H-8 and H-9.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{CortexFileHeader, Prior, SECTION_HOMEOSTASIS, SectionEntry, crc64};
use cortex_core::{FLAG_INHIBITORY, MODULATION_ONE_Q16, WorkerWheel};
use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool, PRESSURE_MAX_Q16,
    SIGMA_MAX_Q16, STAGE_AWAKE, STAGE_SWS,
};
use cortex_runtime::{
    Attribution, Config, Drive, Executor, Image, Perturbation, blocks_for, blocks_per_unit,
    cascade, fork, run_driven, synthesize,
};

type Engine = Executor<2048>;

const WINDOW: u64 = 1 << (ACTIVITY_BIN_SHIFT + ACTIVITY_WINDOW_SHIFT);
const BIN: u64 = 1 << ACTIVITY_BIN_SHIFT;
/// A descendant fires within this many ticks of its message's arrival.
const LATENCY: u32 = 128;
/// A baseline spike within this many ticks after a descendant is the same spike, advanced.
const ADVANCE: u32 = 512;
/// The kick of the oracle: one message of 1.5, which fires a unit the drive holds near its
/// threshold once or twice and a unit the drive left low not at all.
const KICK_Q16: i32 = 0x0001_8000;
/// The cue of the readout and the experience: two messages of 1.25, the replay drive's.
const CUE_Q16: i32 = 0x0001_4000;

/// The prior of ADR-0044 at `units`: a fifth inhibitory at the rail, 32 synapses per unit,
/// a window of eight, a quarter rewired, local delays of 1 to 3 ms and far ones of 14 to
/// 25.6 ms, excitatory weights in [6 000, 12 000].
fn prior(units: u32) -> Prior {
    Prior {
        units,
        inhibitory_every: 5,
        synapses_per_unit: 32,
        window: 8,
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
fn drive(units: u32) -> Drive {
    Drive {
        every: 1,
        messages: units / 128,
        efficacy_q16: 0x2000,
        units,
        seed: 3,
    }
}

fn config(units: u32, workers: usize, step: u16, baseline_q16: i32) -> Config {
    Config {
        workers,
        units: units as usize,
        blocks: blocks_for(&prior(units)) as usize,
        nodes_per_worker: 1 << 16,
        injector_capacity: 1 << 12,
        trace_capacity: 1 << 22,
        modulation_baseline_q16: baseline_q16,
        control_step_q0_16: step,
        episodes: 4,
        ..Config::default()
    }
}

/// The image of `exec` with its homeostasis record patched, decoded under `config`.
fn reload_with(exec: &Engine, config: Config, patch: impl Fn(&mut HomeostaticDrivePool)) -> Engine {
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

/// The network at `gain`, synthesized from the prior and reloaded with the gain in its
/// record.
fn at_gain(units: u32, config: Config, gain: u32) -> Engine {
    let mut exec = Engine::new(config.clone()).unwrap();
    let (u, b) = exec.arenas_mut();
    synthesize(u, b, &prior(units)).unwrap();
    reload_with(&exec, config, |p| p.synaptic_gain_q16 = gain)
}

/// The kicked unit's synapses as the oracle reads them.
fn synapses_of(exec: &Engine, unit: u32) -> Vec<(u32, u16)> {
    exec.units()[unit as usize]
        .fan_out(exec.blocks())
        .map(|s| (s.target, s.delay_ticks))
        .collect()
}

/// Runs `windows` windows under the drive; returns `(gain, estimate, spikes)` per window,
/// the gain and the estimate as the window's regulation left them.
fn windows(exec: &mut Engine, drive: &Drive, windows: u64) -> Vec<Window> {
    let mut out = Vec::new();
    for _ in 0..windows {
        let mut spikes = 0u64;
        for _ in 0..(1u64 << ACTIVITY_WINDOW_SHIFT) {
            let until = exec.ticks().wrapping_add(BIN);
            run_driven(exec, drive, until).unwrap();
            let h = exec.homeostasis();
            let bin = if h.window_bins == 0 {
                h.first_activity
            } else {
                h.last_activity
            };
            spikes = spikes.wrapping_add(bin as u64);
        }
        let h = exec.homeostasis();
        out.push((h.synaptic_gain_q16, h.branching_ratio_q16, spikes));
    }
    out
}

/// The attribution of `kicks` kicks, one every 64 ticks from `at`, into units 61 apart, on
/// forks of `image`.
fn attribution(
    image: &[u8],
    config: &Config,
    units: u32,
    synapses: &[Vec<(u32, u16)>],
    kicks: u32,
    at: u64,
) -> Attribution {
    let drive = drive(units);
    let horizon = at.wrapping_add(3 * 2559);
    let baseline = fork::<2048>(image, config, &drive, &[], horizon).unwrap();
    let mut cascades = Vec::new();
    for j in 0..kicks {
        // `units` is at least 256 here: never `None`.
        let unit = j.wrapping_mul(61).checked_rem(units).unwrap_or(0);
        let tick = at.wrapping_add((j as u64).wrapping_mul(64));
        let kick = Perturbation {
            unit,
            tick,
            messages: 1,
            efficacy_q16: KICK_Q16,
        };
        let perturbed = fork::<2048>(image, config, &drive, &[kick], horizon).unwrap();
        cascades.push(cascade(
            &baseline,
            &perturbed,
            unit,
            tick as u32,
            &synapses[unit as usize],
            LATENCY,
            ADVANCE,
        ));
    }
    Attribution::of(&cascades)
}

/// H-8's measurement at `units`: per fixed gain of `gains` the estimates and spikes of `w`
/// windows and the attribution of `kicks` kicks; then the closed loop from `from` under a
/// step of an eighth, its gains and estimates over `loop_windows`.
/// One window as `windows` reports it: `(gain, estimate, spikes)`.
type Window = (u32, u32, u64);
/// One fixed gain's measurement: the gain, its windows, the kicks' attribution.
type Fixed = (u32, Vec<Window>, Attribution);

struct Criticality {
    fixed: Vec<Fixed>,
    closed: Vec<Window>,
}

fn criticality(
    units: u32,
    gains: &[u32],
    w: u64,
    kicks: u32,
    from: u32,
    loop_windows: u64,
) -> Criticality {
    let drive = drive(units);
    let mut fixed = Vec::new();
    for &gain in gains {
        let cfg = config(units, 2, 0, 0);
        let mut exec = at_gain(units, cfg.clone(), gain);
        let image = Image::encode(&exec).unwrap();
        let synapses: Vec<Vec<(u32, u16)>> = (0..units).map(|u| synapses_of(&exec, u)).collect();
        let per_window = windows(&mut exec, &drive, w);
        drop(exec);
        let a = attribution(&image, &cfg, units, &synapses, kicks, 4 * BIN);
        fixed.push((gain, per_window, a));
    }
    let cfg = config(units, 2, 0x2000, 0);
    let mut exec = at_gain(units, cfg, from);
    let closed = windows(&mut exec, &drive, loop_windows);
    Criticality { fixed, closed }
}

/// The three gains of the fixed-gain measurement.
const GAINS: [u32; 3] = [0x0001_C000, 0x0002_0000, 0x0002_4000];

/// The synapses among `pattern`: `(count, sum of weights)`.
fn among(exec: &Engine, pattern: &[u32]) -> (u32, i64) {
    let mut count = 0u32;
    let mut sum = 0i64;
    for &u in pattern {
        for s in exec.units()[u as usize].fan_out(exec.blocks()) {
            if pattern.contains(&s.target) {
                count = count.wrapping_add(1);
                sum = sum.wrapping_add(s.weight_q1_15 as i64);
            }
        }
    }
    (count, sum)
}

/// A cue of the first six units of `pattern` on a fork of `image` with no drive: the units
/// of the rest that fired within two horizons, the other units that fired, and the spikes.
fn readout(image: &[u8], config: &Config, pattern: &[u32]) -> (usize, usize, usize) {
    let quiet = Drive {
        every: 0,
        messages: 0,
        efficacy_q16: 0,
        units: 0,
        seed: 0,
    };
    let start = Image::decode::<2048>(image, config.clone())
        .unwrap()
        .ticks();
    let cue: Vec<Perturbation> = pattern[..6]
        .iter()
        .map(|&unit| Perturbation {
            unit,
            tick: start,
            messages: 2,
            efficacy_q16: CUE_Q16,
        })
        .collect();
    let trace = fork::<2048>(
        image,
        config,
        &quiet,
        &cue,
        start.wrapping_add(WorkerWheel::horizon_ticks().wrapping_mul(2)),
    )
    .unwrap();
    let mut rest = std::collections::BTreeSet::new();
    let mut others = std::collections::BTreeSet::new();
    for &(_, u) in &trace {
        if pattern[6..].contains(&u) {
            rest.insert(u);
        } else if !pattern[..6].contains(&u) {
            others.insert(u);
        }
    }
    (rest.len(), others.len(), trace.len())
}

/// H-9's measurement at `units`: the plastic network awake for three bins with an experience
/// in the third, two patterns tagged, a night, the synapses among each pattern before and
/// after, and the readouts.
struct Night {
    stages: Vec<u8>,
    replays: u64,
    depotentiations: u64,
    cluster: ((u32, i64), (u32, i64)),
    random: ((u32, i64), (u32, i64)),
    readouts: [(usize, usize, usize); 4],
}

fn night(units: u32) -> Night {
    let p = prior(units);
    let mut cfg = config(units, 2, 0, MODULATION_ONE_Q16);
    cfg.sleep_shift = 5;
    let mut exec = at_gain(units, cfg.clone(), 0x0002_0000);
    let drive = drive(units);
    let cluster: Vec<u32> = (0..15u32).filter(|u| !p.is_inhibitory(*u)).collect();
    assert_eq!(cluster.len(), 12);
    assert!(
        cluster
            .iter()
            .all(|&u| exec.units()[u as usize].flags & FLAG_INHIBITORY == 0)
    );
    run_driven(&mut exec, &drive, 2 * BIN).unwrap();
    let experience = exec.ticks();
    let inject = exec.injector();
    for &unit in &cluster {
        Perturbation {
            unit,
            tick: experience,
            messages: 2,
            efficacy_q16: CUE_Q16,
        }
        .inject(&inject)
        .unwrap();
    }
    run_driven(&mut exec, &drive, 3 * BIN).unwrap();
    let mut waited = 0u32;
    while !exec.is_quiescent() {
        exec.tick();
        waited = waited.wrapping_add(1);
        assert!(waited < 40_000, "the network falls quiet without the drive");
    }
    let random: Vec<u32> = (0..12u32)
        .map(|i| i.wrapping_mul(units / 13).wrapping_add(21))
        .map(|u| {
            if p.is_inhibitory(u) {
                u.wrapping_add(1)
            } else {
                u
            }
        })
        .collect();
    let before = (among(&exec, &cluster), among(&exec, &random));
    let pre = Image::encode(&exec).unwrap();
    assert_eq!(exec.tag_episode(&cluster, 200), Ok(0));
    assert_eq!(exec.tag_episode(&random, 200), Ok(1));
    let mut exec = reload_with(&exec, cfg.clone(), |p| {
        p.sleep_stage = STAGE_SWS;
        p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
    });
    let mut stages = Vec::new();
    loop {
        exec.run(WINDOW);
        stages.push(exec.sleep_stage());
        if exec.sleep_stage() == STAGE_AWAKE || stages.len() > 20 {
            break;
        }
    }
    let mut waited = 0u32;
    while !exec.is_quiescent() {
        exec.tick();
        waited = waited.wrapping_add(1);
        assert!(waited < 40_000);
    }
    let after = (among(&exec, &cluster), among(&exec, &random));
    let post = Image::encode(&exec).unwrap();
    let replays = exec.replays();
    let depotentiations = exec.depotentiations();
    drop(exec);
    let mut fork_cfg = cfg;
    fork_cfg.episodes = 0;
    let readouts = [
        readout(&pre, &fork_cfg, &cluster),
        readout(&post, &fork_cfg, &cluster),
        readout(&pre, &fork_cfg, &random),
        readout(&post, &fork_cfg, &random),
    ];
    Night {
        stages,
        replays,
        depotentiations,
        cluster: (before.0, after.0),
        random: (before.1, after.1),
        readouts,
    }
}

#[test]
fn the_prior_is_written_and_read_back_whole_and_a_driven_run_is_bit_identical_on_one_and_four_workers()
 {
    let units = 256;
    let p = prior(units);
    let mut exec = Engine::new(config(units, 1, 0, 0)).unwrap();
    let (u, b) = exec.arenas_mut();
    let census = synthesize(u, b, &p).unwrap();
    assert_eq!(census, p.census());
    assert_eq!(
        (
            census.inhibitory_units,
            census.synapses,
            census.inhibitory_synapses,
            census.apical
        ),
        (51, 8192, 51 * 32, 0)
    );
    assert!(
        census.long_range > 1500 && census.long_range < 2200,
        "{}",
        census.long_range
    );
    assert_eq!(blocks_per_unit(&p), 8);
    for (i, unit) in exec.units().iter().enumerate() {
        assert_eq!(unit.flags & FLAG_INHIBITORY != 0, p.is_inhibitory(i as u32));
        assert_eq!(unit.chain(exec.blocks()).count(), 8);
        for s in unit.fan_out(exec.blocks()) {
            assert!((s.delay_ticks as u64) < WorkerWheel::horizon_ticks());
        }
    }
    let image = Image::encode(&exec).unwrap();
    let loaded = Image::decode::<2048>(&image, config(units, 1, 0, 0)).unwrap();
    assert_eq!(loaded.blocks(), exec.blocks());
    assert_eq!(
        loaded
            .units()
            .iter()
            .map(|u| u.encode())
            .collect::<Vec<_>>(),
        exec.units().iter().map(|u| u.encode()).collect::<Vec<_>>()
    );
    // The driven run at the gain of the measurements (at 1.0 the drive fires nothing), then
    // quiet until quiescent: a mailbox node's index is a position in its worker's pool, so
    // the arenas are compared where the writer would write them, with every mailbox empty.
    let outcome = |workers: usize| {
        let mut exec = at_gain(units, config(units, workers, 0, 0), 0x0002_0000);
        run_driven(&mut exec, &drive(units), 2 * BIN).unwrap();
        let mut waited = 0u32;
        while !exec.is_quiescent() {
            exec.tick();
            waited = waited.wrapping_add(1);
            assert!(waited < 40_000, "the network falls quiet without the drive");
        }
        let units: Vec<[u8; 64]> = exec.units().iter().map(|u| u.encode()).collect();
        let blocks = exec.blocks().to_vec();
        let pool = *exec.homeostasis();
        (units, blocks, pool, cortex_runtime::trace(exec).unwrap())
    };
    let one = outcome(1);
    let four = outcome(4);
    assert!(
        one.3.len() > 100,
        "the drive fires the network: {}",
        one.3.len()
    );
    assert_eq!(one.0, four.0, "the unit arenas");
    assert_eq!(one.1, four.1, "the synapse arenas");
    assert_eq!(one.2, four.2, "the homeostasis record");
    assert_eq!(one.3, four.3, "the spike trains");
}

/// The gate's form at 256 units: one fixed gain (2.0), one window and four kicks, then the
/// loop from 2.0 for four windows; a debug-profile run of seconds, since the mutation gate
/// reruns every test of this crate per mutant. The sweep over three gains with two windows
/// and eight kicks, and the loop from 1.0, are the `exhaustive` form below.
#[test]
fn the_estimate_the_causal_ratio_and_the_loop_at_256_units() {
    let c = criticality(256, &GAINS[1..2], 1, 4, 0x0002_0000, 4);
    // At a gain of 2.0: a window's estimate of 0.659 over 3 156 spikes; four kicks, eleven
    // ancestor spikes, five first-generation descendants of which two advanced spikes the
    // drive would have produced: a gross ratio of 0.455 and a net one of 0.273.
    let fixed: Vec<_> = c
        .fixed
        .iter()
        .map(|(g, w, a)| {
            (
                *g,
                w.clone(),
                *a,
                a.branching_ratio_q16(),
                a.net_branching_ratio_q16(),
            )
        })
        .collect();
    assert_eq!(
        fixed,
        vec![(
            0x0002_0000,
            vec![(0x0002_0000, 43_210, 3_156)],
            Attribution {
                kicks: 4,
                ancestors: 11,
                descendants: 5,
                advanced: 2,
                extra: 61,
                missing: 67
            },
            Some(29_789),
            Some(17_873)
        )]
    );
    // The loop from 2.0 under a step of an eighth: up while the slope reads below 1, the
    // fourth window at the ceiling (11 846 spikes over 8 192) read as 16 and the gain down.
    assert_eq!(
        c.closed,
        vec![
            (136_654, 43_210, 3_156),
            (151_386, 9_016, 3_778),
            (167_295, 10_442, 7_473),
            (146_383, SIGMA_MAX_Q16, 11_846),
        ]
    );
    let _ = MODULATION_ONE_Q16;
}

/// The weekly job's form at 256 units: three gains, two windows and eight kicks each, the
/// loop from 1.0 for ten windows.
#[test]
#[ignore]
fn the_estimate_the_causal_ratios_and_the_loop_at_256_units_exhaustive() {
    let c = criticality(256, &GAINS, 2, 8, MODULATION_ONE_Q16 as u32, 10);
    // At fixed gains, two windows each: the lag-one estimate per window (Q16.16) and the
    // window's spikes, then eight kicks attributed through the connectome. The estimate of
    // one window of thirty-two bins varies by more than itself between two windows at one
    // gain (0.659 then 0.000 at 2.0); the causal ratios rise with the gain (gross 0.389,
    // 0.300, 0.583; net 0.389, 0.200, 0.306) and stay below 1 up to the ceiling's
    // neighbourhood; the two forks drift apart (`extra`, `missing`) far beyond the first
    // generation, which is why the oracle attributes through the synapses and not by time.
    let fixed: Vec<_> = c
        .fixed
        .iter()
        .map(|(g, w, a)| {
            (
                *g,
                w.clone(),
                *a,
                a.branching_ratio_q16(),
                a.net_branching_ratio_q16(),
            )
        })
        .collect();
    assert_eq!(
        fixed,
        vec![
            (
                0x0001_C000,
                vec![(0x0001_C000, 12_462, 555), (0x0001_C000, 14_075, 489)],
                Attribution {
                    kicks: 8,
                    ancestors: 18,
                    descendants: 7,
                    advanced: 0,
                    extra: 36,
                    missing: 40
                },
                Some(25_486),
                Some(25_486)
            ),
            (
                0x0002_0000,
                vec![(0x0002_0000, 43_210, 3_156), (0x0002_0000, 0, 2_563)],
                Attribution {
                    kicks: 8,
                    ancestors: 20,
                    descendants: 6,
                    advanced: 2,
                    extra: 127,
                    missing: 136
                },
                Some(19_660),
                Some(13_107)
            ),
            (
                0x0002_4000,
                vec![(0x0002_4000, 35_054, 6_845), (0x0002_4000, 2_292, 6_223)],
                Attribution {
                    kicks: 8,
                    ancestors: 36,
                    descendants: 21,
                    advanced: 10,
                    extra: 504,
                    missing: 551
                },
                Some(38_229),
                Some(20_025)
            ),
        ]
    );
    // The loop from a gain of 1.0 under a step of an eighth: an eighth up per window while
    // the estimate reads below 1 (silence is 0), the network waking from the fourth window,
    // the ninth window at the ceiling (9 940 spikes over 8 192, one per unit per bin) read as
    // 16 and the gain lowered by an eighth, the tenth below it and the gain raised again:
    // the ceiling, not a slope of 1, is where the loop turns at this scale.
    assert_eq!(
        c.closed,
        vec![
            (73_728, 0, 0),
            (82_944, 0, 0),
            (93_312, 0, 0),
            (104_976, 0, 15),
            (114_723, 16_852, 127),
            (129_063, 0, 537),
            (142_565, 10_690, 2_606),
            (160_386, 0, 5_253),
            (140_338, SIGMA_MAX_Q16, 9_940),
            (157_880, 0, 4_643),
        ]
    );
    let _ = MODULATION_ONE_Q16;
}

#[test]
fn a_night_consolidates_the_synapses_among_a_tagged_pattern_and_a_cue_completes_the_local_one_at_256_units()
 {
    let n = night(256);
    // Three windows of slow-wave sleep, two of REM, two more of slow-wave, awake: 376
    // replays of the two episodes in turn, 128 depotentiations (the tags of 200 survive).
    assert_eq!(n.stages, vec![1, 1, 1, 2, 2, 1, 1, 0]);
    assert_eq!((n.replays, n.depotentiations), (376, 128));
    // The cluster of twelve neighbours holds 147 synapses among itself, the random pattern
    // five; every one of them ends at the rail (32 767) after the night's replays.
    assert_eq!(n.cluster, ((147, 1_281_021), (147, 147 * 32_767)));
    assert_eq!(n.random, ((5, 40_203), (5, 5 * 32_767)));
    // A cue of six units on the image before the night fires no other unit (the cued six
    // fire, 18 spikes); on the image after it fires all six of the cluster's rest and no
    // other unit (39 spikes); the random pattern's rest never fires: completion needs the
    // synapses among the pattern, which a local pattern has and a random one at this
    // density does not.
    assert_eq!(n.readouts, [(0, 0, 18), (6, 0, 39), (0, 0, 18), (0, 0, 18)]);
}

/// The weekly job's form of the measurement (`cargo test -p cortex-runtime --release -- --ignored
/// exhaustive`): 1 024 units, sixteen kicks per gain, twelve windows of closed loop. The same
/// picture as at 256: the estimate of one window swings by more than itself at one gain
/// (0.560 then 0.032 at 2.0), the gross causal ratio rises to 1.000 at 2.25 while the net
/// stays near 0.4, and the loop crosses the ceiling twice in twelve windows.
#[test]
#[ignore]
fn the_estimate_the_causal_ratios_and_the_loop_at_1024_units_exhaustive() {
    let c = criticality(1024, &GAINS, 2, 16, MODULATION_ONE_Q16 as u32, 12);
    let fixed: Vec<_> = c
        .fixed
        .iter()
        .map(|(g, w, a)| {
            (
                *g,
                w.clone(),
                *a,
                a.branching_ratio_q16(),
                a.net_branching_ratio_q16(),
            )
        })
        .collect();
    assert_eq!(
        fixed,
        vec![
            (
                0x0001_C000,
                vec![(0x0001_C000, 21_657, 2_406), (0x0001_C000, 17_656, 2_297)],
                Attribution {
                    kicks: 16,
                    ancestors: 34,
                    descendants: 18,
                    advanced: 3,
                    extra: 92,
                    missing: 102
                },
                Some(34_695),
                Some(28_913)
            ),
            (
                0x0002_0000,
                vec![(0x0002_0000, 36_723, 12_757), (0x0002_0000, 2_124, 11_328)],
                Attribution {
                    kicks: 16,
                    ancestors: 47,
                    descendants: 30,
                    advanced: 14,
                    extra: 892,
                    missing: 931
                },
                Some(41_831),
                Some(22_310)
            ),
            (
                0x0002_4000,
                vec![(0x0002_4000, 48_229, 27_945), (0x0002_4000, 0, 25_754)],
                Attribution {
                    kicks: 16,
                    ancestors: 61,
                    descendants: 61,
                    advanced: 32,
                    extra: 1_777,
                    missing: 1_795
                },
                Some(0x0001_0000),
                Some(31_156)
            ),
        ]
    );
    assert_eq!(
        c.closed,
        vec![
            (73_728, 0, 0),
            (82_944, 0, 0),
            (88_896, 27_913, 4),
            (100_008, 0, 22),
            (111_804, 3_693, 224),
            (124_558, 5_728, 1_504),
            (132_976, 30_101, 7_656),
            (146_183, 13_467, 13_086),
            (160_251, 15_083, 25_060),
            (140_220, SIGMA_MAX_Q16, 39_759),
            (154_121, 13_559, 18_361),
            (134_856, SIGMA_MAX_Q16, 33_001),
        ]
    );
}

/// The weekly job's form of the night at 1 024 units: the cluster's 143 synapses at the rail
/// and its rest complete on the cue after the night and not before; the random pattern's
/// two synapses at the rail and its rest never fire.
#[test]
#[ignore]
fn a_night_at_1024_units_exhaustive() {
    let n = night(1024);
    assert_eq!(n.stages, vec![1, 1, 1, 2, 2, 1, 1, 0]);
    assert_eq!((n.replays, n.depotentiations), (376, 128));
    assert_eq!(n.cluster, ((143, 1_268_100), (143, 143 * 32_767)));
    assert_eq!(n.random, ((2, 18_841), (2, 2 * 32_767)));
    assert_eq!(n.readouts, [(0, 0, 18), (6, 0, 39), (0, 0, 18), (0, 0, 18)]);
}
