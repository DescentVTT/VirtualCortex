//! Brief 022's exit test (ADR-0044; whitepaper §8.8, §11.1) and, since brief 023, the
//! measurements of ADR-0047 and ADR-0048: the reference network as the runtime synthesizes,
//! drives, forks and measures it. The prior of `cortex-connectome` is written into an
//! executor's arenas and read back whole through the image; a driven run is bit-identical on
//! one and four workers; at 256 units and fixed gains the lag-one estimate of ADR-0036 is
//! held beside the causal branching ratio the oracle attributes through the connectome,
//! gross and net, and beside the slopes at a bin near the generation time that a caller
//! reads from the train (ADR-0047), on the lattice and on the sparse random network; under a
//! control step of an eighth the gain's trajectory is pinned; a night of the stages of
//! ADR-0037 with two episodes tagged (ADR-0038) moves the synapses among each pattern by the
//! pinned amount, and a cue of half a pattern on a fork of the image after the night fires
//! the rest of the local pattern and not of the random one; and a night with two episodes
//! tagged from the train itself (ADR-0048), one the pattern active in the ripple before a
//! rewarded invention and one the network's own densest coincidence, holds what each does.
//! The same harness at 1 024 units is the `exhaustive` test the weekly job runs.
//!
//! Every number here is the engine's own, pinned from one run and held on every worker
//! count and every architecture, as the determinism pin is; a deliberate change to the
//! dynamics moves them and says why. What these tests decide, and at what scale, is stated
//! in the ADRs and in the dispositions of H-8, H-9 and H-11.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{CortexFileHeader, Prior, SECTION_HOMEOSTASIS, SectionEntry, crc64};
use cortex_core::{FLAG_INHIBITORY, MODULATION_ONE_Q16, WorkerWheel};
use cortex_hippocampus::{Burst, PATTERN_MAX, RIPPLE_SHIFT};
use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool, PRESSURE_MAX_Q16,
    SIGMA_MAX_Q16, STAGE_AWAKE, STAGE_SWS, count_bins, slope_at_lag,
};
use cortex_reasoning::{INVENTED_BASE, TermNode};
use cortex_runtime::{
    Attribution, COINCIDENCE_TICKS, Config, Drive, Executor, Image, Perturbation, blocks_for,
    blocks_per_unit, cascade, fork, run_driven, synthesize, tag_burst_in, trace,
};

type Engine = Executor<2048>;

const WINDOW: u64 = 1 << (ACTIVITY_BIN_SHIFT + ACTIVITY_WINDOW_SHIFT);
const BIN: u64 = 1 << ACTIVITY_BIN_SHIFT;
const RIPPLE: u64 = 1 << RIPPLE_SHIFT;
/// A descendant fires within this many ticks of its message's arrival.
const LATENCY: u32 = 128;
/// A baseline spike within this many ticks after a descendant is the same spike, advanced.
const ADVANCE: u32 = 512;
/// The kick of the oracle: one message of 1.5, which fires a unit the drive holds near its
/// threshold once or twice and a unit the drive left low not at all.
const KICK_Q16: i32 = 0x0001_8000;
/// The cue of the readout and the experience: two messages of 1.25, the replay drive's.
const CUE_Q16: i32 = 0x0001_4000;
/// The span the capture rules rank within: a basal time constant (ADR-0018), the span the
/// cued units' messages must land within for a pattern to complete (ADR-0044); the loop's
/// own constant since ADR-0052.
const COINCIDENCE: u32 = COINCIDENCE_TICKS;
/// The bin a caller reads the train at (ADR-0047): $2^8$ ticks, near one generation of the
/// lattice's local delays with the unit's latency, a sixteenth of the record's bin.
const FINE: u32 = 256;
/// The lags of the fine-bin slopes.
const LAGS: [usize; 5] = [1, 2, 4, 8, 16];
const ONE: i32 = MODULATION_ONE_Q16;

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

/// The reference prior with its inhibitory gain at 2.0 (ADR-0057): the inhibitory weights
/// drawn in [-24 000, -12 000], a third to three quarters of the rail instead of on it, so
/// that the inhibitory rule has room both ways; nothing else changes.
fn prior_below_rail(units: u32) -> Prior {
    Prior {
        inhibitory_gain_q4_4: 32,
        ..prior(units)
    }
}

/// The second prior (ADR-0047): the same generator at its widest window, every unit but the
/// source and its antipode a target with equal probability, nothing rewired, so that every
/// delay is from the local band: the sparse random excitatory-inhibitory network of Brunel
/// (2000) with short delays.
fn random_prior(units: u32) -> Prior {
    Prior {
        window: (units >> 1).wrapping_sub(1),
        rewire_q0_8: 0,
        ..prior(units)
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
        train_capacity: 1 << 22,
        modulation_baseline_q16: baseline_q16,
        control_step_q0_16: step,
        episodes: 4,
        // The engine's own arena and store (ADR-0052): room for the exit store of ADR-0045
        // and its commits; a search between ticks spends at most 64 attempts and tags with
        // the priority of the nights' tags.
        terms: 1024,
        clauses: 32,
        search_budget: 64,
        discovery_tag: 200,
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

/// The network at `gain`, synthesized from `p` and reloaded with the gain in its record.
fn at_gain(p: &Prior, config: Config, gain: u32) -> Engine {
    let mut exec = Engine::new(config.clone()).unwrap();
    let (u, b) = exec.arenas_mut();
    synthesize(u, b, p).unwrap();
    reload_with(&exec, config, |h| h.synaptic_gain_q16 = gain)
}

/// The kicked unit's synapses as the oracle reads them.
fn synapses_of(exec: &Engine, unit: u32) -> Vec<(u32, u16)> {
    exec.units()[unit as usize]
        .fan_out(exec.blocks())
        .map(|s| (s.target, s.delay_ticks))
        .collect()
}

/// One window as `windows` reports it: `(gain, estimate, spikes, descendants)`, the last the
/// in-loop count of ADR-0054 (the spikes within the oracle's latency of a synapse's message).
type Window = (u32, u32, u64, u64);

/// Runs `windows` windows under the drive; returns `(gain, estimate, spikes, descendants)`
/// per window, the gain and the estimate as the window's regulation left them.
fn windows(exec: &mut Engine, drive: &Drive, windows: u64) -> Vec<Window> {
    let mut out = Vec::new();
    for _ in 0..windows {
        let mut spikes = 0u64;
        let before = exec.descendants();
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
        out.push((
            h.synaptic_gain_q16,
            h.branching_ratio_q16,
            spikes,
            exec.descendants().wrapping_sub(before),
        ));
    }
    out
}

/// The in-loop ratio of a window (ADR-0054), Q16.16: descendants over spikes; `None` for a
/// window without a spike.
fn in_loop_q16(w: &Window) -> Option<u32> {
    let (_, _, spikes, descendants) = *w;
    // Below the spikes, which are below the cap: the shift cannot wrap and the quotient
    // fits `u32`; a window without a spike has no ratio.
    descendants
        .wrapping_shl(16)
        .checked_div(spikes)
        .map(|ratio| ratio as u32)
}

/// One window of a day (ADR-0053): the spikes, the gain and the estimate as the window's
/// regulation left them, the sleep stage, the descendants (ADR-0054), the sum of the
/// inhibitory magnitudes and the sum of the excitatory weights over the arena after the
/// window, the replays so far, and the fraction of units at the target (ADR-0057).
type DayWindow = (u64, u32, u32, u8, u64, i64, i64, u64, u32);

/// The fraction of units at the inhibitory rule's target over the window `[from, to)`
/// (ADR-0057), Q16.16: a unit is at the target when its spikes in the window are within a
/// factor of two of the target's count per window, `WINDOW / period` (six at 20 000 ticks,
/// twenty-six at 5 000), inclusive both ways; read from the executor's own train.
fn at_target_q16(exec: &mut Engine, from: u64, to: u64, period: u32) -> u32 {
    let units = exec.units().len();
    // The period is at least `ISTDP_PERIOD_MIN_TICKS`, so the quotient exists.
    let target = WINDOW.checked_div(u64::from(period)).unwrap_or(0) as u32;
    let mut counts = vec![0u32; units];
    for &(tick, unit) in exec.train() {
        // The day's ticks are below the width, so the train's stamp is the tick itself.
        let tick = u64::from(tick);
        if tick >= from && tick < to {
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
    (at_target << 16).checked_div(units as u64).unwrap_or(0) as u32
}

/// The weights over the arena by polarity: the sum of the inhibitory magnitudes and the sum
/// of the excitatory weights.
fn weights_by_polarity(exec: &Engine) -> (i64, i64) {
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

/// A day on prior `p` (ADR-0053): `w` windows under the drive from a gain of 2.0, held
/// (`step` 0) or regulated by the controller, the pressure rising at `shift` (0 never
/// sleeps) from zero, the inhibitory rule at `period`, the local cluster of twelve tagged at
/// the start so that a night has something to replay; per window the reading above, the
/// fraction at the target over the window (ADR-0057) last.
fn day(p: &Prior, w: u64, step: u16, shift: u8, period: u32) -> Vec<DayWindow> {
    let units = p.units;
    let mut cfg = config(units, 2, step, MODULATION_ONE_Q16);
    cfg.sleep_shift = shift;
    cfg.istdp_target_period_ticks = period;
    let mut exec = at_gain(p, cfg, 0x0002_0000);
    assert_eq!(exec.istdp_target_period_ticks(), period);
    let cluster: Vec<u32> = (0..15u32).filter(|u| !p.is_inhibitory(*u)).collect();
    assert_eq!(exec.tag_episode(&cluster, 200), Ok(0));
    let drive = drive(units);
    let mut out = Vec::new();
    for _ in 0..w {
        let from = exec.ticks();
        let (gain, estimate, spikes, descendants) = windows(&mut exec, &drive, 1)[0];
        let to = exec.ticks();
        let (inhibitory, excitatory) = weights_by_polarity(&exec);
        let at_target = at_target_q16(&mut exec, from, to, period);
        out.push((
            spikes,
            gain,
            estimate,
            exec.sleep_stage(),
            descendants,
            inhibitory,
            excitatory,
            exec.replays(),
            at_target,
        ));
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

/// The slopes of the train's population count at `FINE`-tick bins over `ticks` ticks, at the
/// five lags (ADR-0047).
fn fine_slopes(train: &[(u32, u32)], ticks: u64) -> [Option<u32>; 5] {
    let bins = ticks.checked_div(FINE as u64).unwrap_or(0) as usize;
    let mut series = vec![0u32; bins];
    count_bins(train, 0, FINE, &mut series);
    let mut out = [None; 5];
    for (slot, &lag) in LAGS.iter().enumerate() {
        out[slot] = slope_at_lag(&series, lag);
    }
    out
}

/// The record's estimate per window, recomputed from the train's bins of $2^{12}$ ticks: the
/// cross-check of `estimate_branching_ratio` against the train (equal wherever the window
/// is below the ceiling; at it the record reports `SIGMA_MAX_Q16`).
fn coarse_slopes(train: &[(u32, u32)], w: u64) -> Vec<Option<u32>> {
    (0..w)
        .map(|k| {
            let mut bins = [0u32; 32];
            count_bins(
                train,
                (k as u32).wrapping_mul(WINDOW as u32),
                BIN as u32,
                &mut bins,
            );
            slope_at_lag(&bins, 1)
        })
        .collect()
}

/// One fixed gain's measurement: the gain, its windows, the kicks' attribution, the
/// fine-bin slopes over the windows' train at the five lags, and the coarse cross-check per
/// window.
type Fixed = (
    u32,
    Vec<Window>,
    Attribution,
    [Option<u32>; 5],
    Vec<Option<u32>>,
);

struct Criticality {
    fixed: Vec<Fixed>,
    closed: Vec<Window>,
}

/// H-8's measurement on prior `p`: per fixed gain of `gains` the estimates and spikes of `w`
/// windows, the slopes from the train, and the attribution of `kicks` kicks; then the
/// closed loop from `from` under a step of an eighth, its gains and estimates over
/// `loop_windows` (none for zero).
fn criticality(
    p: &Prior,
    gains: &[u32],
    w: u64,
    kicks: u32,
    from: u32,
    loop_windows: u64,
) -> Criticality {
    let units = p.units;
    let drive = drive(units);
    let mut fixed = Vec::new();
    for &gain in gains {
        let cfg = config(units, 2, 0, 0);
        let mut exec = at_gain(p, cfg.clone(), gain);
        let image = Image::encode(&exec).unwrap();
        let synapses: Vec<Vec<(u32, u16)>> = (0..units).map(|u| synapses_of(&exec, u)).collect();
        let per_window = windows(&mut exec, &drive, w);
        let ticks = exec.ticks();
        assert_eq!(exec.train_overwritten(), 0, "the ring holds the run");
        let train = exec.train().to_vec();
        drop(exec);
        let fine = fine_slopes(&train, ticks);
        let coarse = coarse_slopes(&train, w);
        let a = attribution(&image, &cfg, units, &synapses, kicks, 4 * BIN);
        fixed.push((gain, per_window, a, fine, coarse));
    }
    let closed = if loop_windows == 0 {
        Vec::new()
    } else {
        let cfg = config(units, 2, 0x2000, 0);
        let mut exec = at_gain(p, cfg, from);
        windows(&mut exec, &drive, loop_windows)
    };
    Criticality { fixed, closed }
}

/// The three gains of the fixed-gain measurement.
const GAINS: [u32; 3] = [0x0001_C000, 0x0002_0000, 0x0002_4000];

/// The fixed part of a measurement with the ratios beside it, for the assertions: the gain,
/// its windows, the attribution, the gross and net causal ratios, the fine slopes, the
/// coarse cross-check, and the in-loop ratio per window (ADR-0054).
type Reading = (
    u32,
    Vec<Window>,
    Attribution,
    Option<u32>,
    Option<i32>,
    [Option<u32>; 5],
    Vec<Option<u32>>,
    Vec<Option<u32>>,
);

fn readings(c: &Criticality) -> Vec<Reading> {
    c.fixed
        .iter()
        .map(|(g, w, a, fine, coarse)| {
            (
                *g,
                w.clone(),
                *a,
                a.branching_ratio_q16(),
                a.net_branching_ratio_q16(),
                *fine,
                coarse.clone(),
                w.iter().map(in_loop_q16).collect(),
            )
        })
        .collect()
}

/// The readings as Rust literals, for a pin taken from the run: one line per gain with the
/// windows and the in-loop ratios, then the loop's windows.
fn dump(name: &str, c: &Criticality) {
    for (g, w, _, _, _, _, _, in_loop) in readings(c) {
        eprintln!("DUMP {name} gain {g:#x} windows {w:?} in_loop {in_loop:?}");
    }
    eprintln!("DUMP {name} closed {:?}", c.closed);
}

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
/// of the rest that fired within two horizons (sorted), the other units that fired, and the
/// spikes.
fn readout(image: &[u8], config: &Config, pattern: &[u32]) -> (Vec<u32>, usize, usize) {
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
    (rest.into_iter().collect(), others.len(), trace.len())
}

/// Runs `exec` quiet until it is quiescent.
fn settle(exec: &mut Engine) {
    let mut waited = 0u32;
    while !exec.is_quiescent() {
        exec.tick();
        waited = waited.wrapping_add(1);
        assert!(waited < 40_000, "the network falls quiet without the drive");
    }
}

/// The night of ADR-0044 from a pressure of 1.0 at shift 5: the stages until the wake, then
/// quiet until quiescent.
fn sleep(exec: &Engine, cfg: &Config) -> (Engine, Vec<u8>) {
    let mut exec = reload_with(exec, cfg.clone(), |h| {
        h.sleep_stage = STAGE_SWS;
        h.sleep_pressure_q16 = PRESSURE_MAX_Q16;
    });
    let mut stages = Vec::new();
    loop {
        exec.run(WINDOW);
        stages.push(exec.sleep_stage());
        if exec.sleep_stage() == STAGE_AWAKE || stages.len() > 20 {
            break;
        }
    }
    settle(&mut exec);
    (exec, stages)
}

/// H-9's measurement at `units`: the plastic network awake for three bins with an experience
/// in the third, two patterns tagged, a night, the synapses among each pattern before and
/// after, and the readouts.
struct Night {
    stages: Vec<u8>,
    replays: u64,
    depotentiations: u64,
    /// The two episodes' tags after the night.
    tags: (u8, u8),
    cluster: ((u32, i64), (u32, i64)),
    random: ((u32, i64), (u32, i64)),
    readouts: [(Vec<u32>, usize, usize); 4],
}

fn night(units: u32) -> Night {
    let p = prior(units);
    let mut cfg = config(units, 2, 0, MODULATION_ONE_Q16);
    cfg.sleep_shift = 5;
    let mut exec = at_gain(&p, cfg.clone(), 0x0002_0000);
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
    settle(&mut exec);
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
    let (exec, stages) = sleep(&exec, &cfg);
    let after = (among(&exec, &cluster), among(&exec, &random));
    let post = Image::encode(&exec).unwrap();
    let replays = exec.replays();
    let depotentiations = exec.depotentiations();
    let tags = (exec.episodes()[0].tag, exec.episodes()[1].tag);
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
        tags,
        cluster: (before.0, after.0),
        random: (before.1, after.1),
        readouts,
    }
}

// ------------------------------------------------- ADR-0048: the store of ADR-0045's exit test

/// The vocabulary of the search's store, as `tests/discovery.rs` has it: two heads, eight
/// literals, four constants, as concept ids; no word enters the runtime.
const P: u32 = 0x100;
const R: u32 = 0x101;
const LIT: [u32; 8] = [0x200, 0x201, 0x202, 0x203, 0x204, 0x205, 0x206, 0x207];
const K: [u32; 4] = [0x300, 0x301, 0x302, 0x303];

/// A fresh variable in the executor's own arena (ADR-0052), numbered after the record's.
fn var(exec: &mut Engine) -> u32 {
    let next = exec.induction().next_variable;
    exec.term(TermNode::variable(next)).unwrap()
}

/// `head(X) ← l1(X), ..., lk(X)` over a fresh variable, asserted into the engine's store.
fn rule(exec: &mut Engine, head: u32, literals: &[u32]) -> u32 {
    let x = var(exec);
    let h = exec.term(TermNode::compound(head, &[x]).unwrap()).unwrap();
    let mut body = [0u32; 7];
    for (i, &l) in literals.iter().enumerate() {
        body[i] = exec.term(TermNode::compound(l, &[x]).unwrap()).unwrap();
    }
    exec.assert_clause(h, &body[..literals.len()]).unwrap()
}

/// The fact `l(k)`, asserted.
fn fact(exec: &mut Engine, literal: u32, k: u32) -> u32 {
    let c = exec.term(TermNode::constant(k)).unwrap();
    let h = exec
        .term(TermNode::compound(literal, &[c]).unwrap())
        .unwrap();
    exec.assert_clause(h, &[]).unwrap()
}

/// The store of ADR-0045's exit test, asserted into the engine's own store: three clauses
/// of `p` sharing `LIT[0..4]` and differing in `LIT[4]`, `LIT[5]`, `LIT[6]`; one clause of
/// `r`; the facts of four constants.
fn exit_store(exec: &mut Engine) -> Vec<u32> {
    let mut out = vec![
        rule(exec, P, &[LIT[0], LIT[1], LIT[2], LIT[3], LIT[4]]),
        rule(exec, P, &[LIT[0], LIT[1], LIT[2], LIT[3], LIT[5]]),
        rule(exec, P, &[LIT[0], LIT[1], LIT[2], LIT[3], LIT[6]]),
        rule(exec, R, &[LIT[0], LIT[7]]),
    ];
    for (i, &k) in K.iter().enumerate() {
        for &l in &LIT[..4] {
            out.push(fact(exec, l, k));
        }
        out.push(fact(exec, [LIT[4], LIT[5], LIT[6], LIT[7]][i], k));
    }
    out
}

/// H-9's open item and H-11's synaptic half at `units` (ADR-0048, ADR-0052): the plastic
/// network awake under the drive for two bins, an experience at the third bin's start, the
/// store of ADR-0045 asserted into the engine's own store and searched a quarter of a ripple
/// later by the loop between ticks, its reward into the modulator; the executor's own
/// train; the pattern active in the ripple before the reward tagged as the invention's and
/// bound to the first invented predicate, the densest coincidence of the two bins before the
/// experience tagged as the network's own; a second search over the store, with no pair
/// left, committing nothing and tagging nothing; the night; the synapses among each pattern
/// before and after; the readouts.
struct Capture {
    /// The train's spikes up to the reward's tick.
    spikes: usize,
    /// The two commits' predicates, in the search's order.
    predicates: (u32, u32),
    /// The rewarded moment's episode: its index and its span, as the loop reports them.
    tagged: (u32, Burst),
    /// The invention's pattern, from the ledger.
    pattern: Vec<u32>,
    burst: Burst,
    background: Vec<u32>,
    /// The signal the reward left in the modulator, and what a second search commits.
    signal: i32,
    control: (u32, bool),
    /// The store's length after the search: two inputs became three outputs, twice.
    clauses: usize,
    stages: Vec<u8>,
    replays: u64,
    depotentiations: u64,
    tags: (u8, u8),
    /// The two episodes' symbols after the night: the invention's bound, the network's own
    /// not.
    symbols: (Option<u32>, Option<u32>),
    invention: ((u32, i64), (u32, i64)),
    own: ((u32, i64), (u32, i64)),
    readouts: [(Vec<u32>, usize, usize); 4],
    /// The nodes the night's slow-wave onset reclaimed from the engine's arena (ADR-0056).
    reclaimed: u64,
}

fn capture_night(units: u32) -> Capture {
    let p = prior(units);
    let mut cfg = config(units, 2, 0, MODULATION_ONE_Q16);
    cfg.sleep_shift = 5;
    let mut exec = at_gain(&p, cfg.clone(), 0x0002_0000);
    let drive = drive(units);
    let cluster: Vec<u32> = (0..15u32).filter(|u| !p.is_inhibitory(*u)).collect();
    run_driven(&mut exec, &drive, 2 * BIN).unwrap();
    let experience = exec.ticks();
    let cues: Vec<Perturbation> = cluster
        .iter()
        .map(|&unit| Perturbation {
            unit,
            tick: experience,
            messages: 2,
            efficacy_q16: CUE_Q16,
        })
        .collect();
    let inject = exec.injector();
    for cue in &cues {
        cue.inject(&inject).unwrap();
    }
    let at = experience.wrapping_add(RIPPLE / 4);
    run_driven(&mut exec, &drive, at).unwrap();
    let spikes = exec.train().len();
    assert_eq!(exec.train_overwritten(), 0);
    // The search between ticks over the engine's own store (ADR-0052): its reward into the
    // modulator, the pattern active in the ripple before it tagged from the executor's own
    // train and bound to the first invented predicate.
    let clauses = exit_store(&mut exec);
    assert_eq!(clauses.len(), 24);
    assert_eq!(exec.clauses(), &clauses[..]);
    let report = exec.discover().unwrap();
    assert_eq!(
        (report.search.commits, report.search.reward_total_q16),
        (2, 3 * ONE / 2)
    );
    let signal = report.signal_q16;
    let tagged = report.tagged.expect("a positive reward tags");
    let predicates = (
        exec.discoveries()[0].invention.predicate,
        exec.discoveries()[1].invention.predicate,
    );
    assert_eq!(
        exec.episodes()[tagged.0 as usize].symbol(),
        Some(predicates.0),
        "the episode is bound to the first commit's predicate"
    );
    let pattern = exec.episodes()[tagged.0 as usize].pattern().to_vec();
    let (burst, index, own, len) =
        tag_burst_in(&mut exec, 0, experience as u32, COINCIDENCE, 200).unwrap();
    assert_eq!(index, 1);
    let background = own[..len as usize].to_vec();
    // A second search over the store: no pair is left, nothing is committed, nothing
    // tagged, the signal as it stood.
    let again = exec.discover().unwrap();
    let control = (again.search.commits, again.tagged.is_none());
    assert_eq!(
        again.signal_q16, signal,
        "no commit: the signal as it stood"
    );
    assert_eq!(exec.episodes().len(), 2);
    assert_eq!((exec.searches(), exec.inventions()), (2, 2));
    let clauses = exec.clauses().len();
    let mut fork_cfg = cfg.clone();
    fork_cfg.episodes = 0;
    run_driven(&mut exec, &drive, 3 * BIN).unwrap();
    settle(&mut exec);
    let invention = pattern.clone();
    let before = (among(&exec, &invention), among(&exec, &background));
    let pre = Image::encode(&exec).unwrap();
    let (exec, stages) = sleep(&exec, &cfg);
    let after = (among(&exec, &invention), among(&exec, &background));
    let post = Image::encode(&exec).unwrap();
    let replays = exec.replays();
    let depotentiations = exec.depotentiations();
    let reclaimed = exec.reclaimed();
    let tags = (exec.episodes()[0].tag, exec.episodes()[1].tag);
    let symbols = (exec.episodes()[0].symbol(), exec.episodes()[1].symbol());
    drop(exec);
    let readouts = [
        readout(&pre, &fork_cfg, &invention),
        readout(&post, &fork_cfg, &invention),
        readout(&pre, &fork_cfg, &background),
        readout(&post, &fork_cfg, &background),
    ];
    Capture {
        spikes,
        predicates,
        tagged,
        pattern,
        burst,
        background,
        signal,
        control,
        clauses,
        stages,
        replays,
        depotentiations,
        tags,
        symbols,
        invention: (before.0, after.0),
        own: (before.1, after.1),
        readouts,
        reclaimed,
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
    // The second prior: the same census but for the long-range count, which its window
    // makes zero by definition; every unit has 32 synapses whose targets are neither itself
    // nor its antipode.
    let r = random_prior(units);
    assert!(r.is_well_formed());
    let rc = r.census();
    assert_eq!(
        (
            rc.inhibitory_units,
            rc.synapses,
            rc.inhibitory_synapses,
            rc.long_range,
            rc.apical
        ),
        (51, 8192, 51 * 32, 0, 0)
    );
    for s in r.synapses() {
        assert!(s.target != s.source && s.target != s.source.wrapping_add(128) % 256);
        assert!((100..=300).contains(&s.delay_ticks));
    }
    // The driven run at the gain of the measurements (at 1.0 the drive fires nothing), then
    // quiet until quiescent: a mailbox node's index is a position in its worker's pool, so
    // the arenas are compared where the writer would write them, with every mailbox empty.
    // With the exit store asserted and the loop on a cadence of one bin (ADR-0052): the
    // search at the first bin's end commits two inventions, rewards the modulator and binds
    // the coincidence before it; the search at the second finds no pair.
    let outcome = |workers: usize| {
        let cfg = Config {
            trace_capacity: 1 << 20,
            search_shift: ACTIVITY_BIN_SHIFT as u8,
            ..config(units, workers, 0, 0)
        };
        let mut exec = at_gain(&p, cfg, 0x0002_0000);
        exit_store(&mut exec);
        run_driven(&mut exec, &drive(units), 2 * BIN).unwrap();
        settle(&mut exec);
        assert_eq!((exec.searches(), exec.inventions()), (2, 2));
        assert_eq!((exec.untagged(), exec.search_failures()), (0, 0));
        assert_eq!(exec.episodes().len(), 1);
        assert_eq!(exec.episodes()[0].symbol(), Some(INVENTED_BASE));
        let units: Vec<[u8; 64]> = exec.units().iter().map(|u| u.encode()).collect();
        let blocks = exec.blocks().to_vec();
        let pool = *exec.homeostasis();
        let store = (
            exec.terms().to_vec(),
            exec.clauses().to_vec(),
            *exec.induction(),
            *exec.affect(),
            exec.episodes().to_vec(),
        );
        // The executor's own train (ADR-0050) is the workers' traces, sorted, on every
        // worker count.
        let train = exec.train().to_vec();
        assert_eq!(exec.train_overwritten(), 0);
        assert_eq!(
            train,
            trace(exec).unwrap(),
            "the ring and the reports agree"
        );
        (units, blocks, pool, train, store)
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
    assert_eq!(
        one.4, four.4,
        "the arena, the store, the records and the ledger"
    );
    assert_eq!(one.4.1.len(), 26);
}

/// The gate's form at 256 units: one fixed gain (2.0), one window and four kicks, then the
/// loop from 2.0 for four windows; a debug-profile run of seconds, since the mutation gate
/// reruns every test of this crate per mutant. The sweep over three gains with two windows
/// and eight kicks, and the loop from 1.0, are the `exhaustive` form below.
#[test]
fn the_estimate_the_causal_ratio_and_the_loop_at_256_units() {
    let c = criticality(&prior(256), &GAINS[1..2], 1, 4, 0x0002_0000, 4);
    dump(
        "the_estimate_the_causal_ratio_and_the_loop_at_256_units",
        &c,
    );
    // At a gain of 2.0: a window's estimate of 0.659 over 3 156 spikes; four kicks, eleven
    // ancestor spikes, five first-generation descendants of which two advanced spikes the
    // drive would have produced: a gross ratio of 0.455 and a net one of 0.273. The record's
    // estimate recomputed from the train's bins is the same 0.659; the slopes at bins of 256
    // ticks fall from 0.569 at lag one to 0.197 at lag eight and rise again at sixteen, where
    // the far band's descendants land (ADR-0047).
    assert_eq!(
        readings(&c),
        vec![(
            0x0002_0000,
            vec![(0x0002_0000, 43_210, 3_156, 1_754)],
            Attribution {
                kicks: 4,
                ancestors: 11,
                descendants: 5,
                advanced: 2,
                extra: 61,
                missing: 67
            },
            Some(29_789),
            Some(17_873),
            [
                Some(37_281),
                Some(30_645),
                Some(22_151),
                Some(12_893),
                Some(15_540)
            ],
            vec![Some(43_210)],
            vec![Some(36_422)]
        )]
    );
    // The loop from 2.0 under a step of an eighth: up while the slope reads below 1, the
    // fourth window at the ceiling (11 846 spikes over 8 192) read as 16 and the gain down.
    assert_eq!(
        c.closed,
        vec![
            (136_654, 43_210, 3_156, 1_754),
            (151_386, 9_016, 3_778, 1_979),
            (167_295, 10_442, 7_473, 4_954),
            (146_383, SIGMA_MAX_Q16, 11_846, 9_251),
        ]
    );
}

/// The gate's form of ADR-0053's measurement: the lattice at 256 units at a gain of 2.0
/// held, no controller, no sleep, eight windows at the default target period (20 000 ticks,
/// 5 Hz) and at one in the reference regime (5 000 ticks, 20 Hz); per window the spikes,
/// the estimate, the descendants, the inhibitory magnitudes' sum, the excitatory weights'
/// sum.
#[test]
fn a_waking_day_at_256_units_at_two_target_periods() {
    let slow = day(&prior(256), 16, 0, 0, 20_000);
    let fast = day(&prior(256), 16, 0, 0, 5_000);
    eprintln!("DUMP day256 slow {slow:?}");
    eprintln!("DUMP day256 fast {fast:?}");
    assert_eq!(
        slow,
        vec![
            (
                3083, 131_072, 35_972, 0, 1679, 53_228_754, 56_266_621, 0, 38_400
            ),
            (
                2437, 131_072, 11_349, 0, 1064, 52_997_589, 54_774_022, 0, 51_712
            ),
            (
                2574, 131_072, 9337, 0, 1143, 52_802_056, 53_228_742, 0, 48_640
            ),
            (2470, 131_072, 0, 0, 1066, 52_681_131, 51_906_655, 0, 48_640),
            (
                2470, 131_072, 11_630, 0, 1052, 52_523_464, 50_732_883, 0, 51_456
            ),
            (
                2446, 131_072, 18_587, 0, 1044, 52_329_080, 49_664_652, 0, 51_200
            ),
            (2349, 131_072, 0, 0, 949, 52_148_736, 48_709_908, 0, 52_992),
            (
                2332, 131_072, 8320, 0, 911, 52_001_615, 47_829_950, 0, 53_248
            ),
            (
                2247, 131_072, 6617, 0, 877, 51_870_615, 47_095_338, 0, 52_736
            ),
            (2377, 131_072, 0, 0, 1001, 51_758_278, 46_247_121, 0, 51_200),
            (2281, 131_072, 0, 0, 946, 51_578_547, 45_488_084, 0, 54_528),
            (2309, 131_072, 0, 0, 936, 51_405_712, 44_770_088, 0, 52_480),
            (2221, 131_072, 0, 0, 823, 51_197_779, 44_158_578, 0, 55_808),
            (2202, 131_072, 0, 0, 757, 50_985_082, 43_623_076, 0, 54_784),
            (2119, 131_072, 0, 0, 701, 50_742_903, 43_110_145, 0, 55_040),
            (
                2229, 131_072, 4364, 0, 822, 50_564_212, 42_538_075, 0, 55_552
            )
        ]
    );
    assert_eq!(
        fast,
        vec![
            (
                3058, 131_072, 34_485, 0, 1625, 49_257_617, 56_346_123, 0, 28_672
            ),
            (
                2459, 131_072, 6868, 0, 1120, 45_443_349, 54_793_871, 0, 13_824
            ),
            (2591, 131_072, 0, 0, 1150, 41_448_062, 53_238_199, 0, 18_432),
            (2618, 131_072, 0, 0, 1161, 37_470_842, 51_743_538, 0, 20_480),
            (
                2598, 131_072, 2445, 0, 1107, 33_453_135, 50_405_761, 0, 16_640
            ),
            (
                2585, 131_072, 10_680, 0, 1147, 29_368_407, 49_210_708, 0, 16_128
            ),
            (
                2618, 131_072, 9549, 0, 1126, 25_329_764, 48_023_404, 0, 18_688
            ),
            (
                2616, 131_072, 12_856, 0, 1137, 21_593_234, 46_934_898, 0, 17_152
            ),
            (
                2539, 131_072, 4870, 0, 1076, 17_857_052, 45_919_195, 0, 17_152
            ),
            (2715, 131_072, 0, 0, 1189, 13_947_664, 44_853_205, 0, 19_968),
            (2764, 131_072, 0, 0, 1208, 10_241_837, 43_763_154, 0, 23_040),
            (2781, 131_072, 0, 0, 1212, 6_830_294, 42_798_423, 0, 22_272),
            (2743, 131_072, 0, 0, 1120, 4_233_925, 41_826_923, 0, 20_224),
            (2722, 131_072, 0, 0, 1122, 2_182_381, 40_993_143, 0, 21_504),
            (
                2670, 131_072, 4546, 0, 1090, 1_089_748, 40_198_960, 0, 20_992
            ),
            (2785, 131_072, 591, 0, 1200, 524_004, 39_373_781, 0, 23_040)
        ]
    );
}

/// The inhibitory rule read from below the rail (ADR-0057): the reference prior with its
/// inhibitory gain at 2.0 at 256 units, sixteen windows with the gain held, at each of the
/// two periods; the ninth reading is the fraction of units at the target.
#[test]
fn a_waking_day_from_below_the_rail_at_256_units_at_two_target_periods() {
    let slow = day(&prior_below_rail(256), 16, 0, 0, 20_000);
    let fast = day(&prior_below_rail(256), 16, 0, 0, 5_000);
    eprintln!("DUMP below256 slow {slow:?}");
    eprintln!("DUMP below256 fast {fast:?}");
    assert_eq!(
        slow,
        vec![
            (
                3302, 131_072, 33_737, 0, 1800, 30_092_499, 55_864_630, 0, 31_232
            ),
            (
                2623, 131_072, 16_018, 0, 1182, 30_220_398, 54_181_937, 0, 48_384
            ),
            (
                2774, 131_072, 2176, 0, 1272, 30_378_090, 52_441_738, 0, 44_032
            ),
            (2655, 131_072, 0, 0, 1151, 30_505_158, 50_967_395, 0, 45_824),
            (2617, 131_072, 0, 0, 1115, 30_570_736, 49_650_885, 0, 47_104),
            (
                2543, 131_072, 13_047, 0, 1089, 30_580_209, 48_542_347, 0, 47_360
            ),
            (
                2524, 131_072, 2170, 0, 1070, 30_614_889, 47_484_079, 0, 49_664
            ),
            (
                2476, 131_072, 19_694, 0, 1029, 30_657_149, 46_501_422, 0, 51_200
            ),
            (2345, 131_072, 0, 0, 936, 30_681_402, 45_695_308, 0, 51_968),
            (
                2487, 131_072, 2203, 0, 1031, 30_736_643, 44_812_213, 0, 51_456
            ),
            (2426, 131_072, 0, 0, 984, 30_729_867, 43_998_623, 0, 50_944),
            (2392, 131_072, 0, 0, 943, 30_705_793, 43_310_459, 0, 50_432),
            (2345, 131_072, 0, 0, 852, 30_626_759, 42_640_454, 0, 55_296),
            (2292, 131_072, 0, 0, 815, 30_523_548, 42_062_582, 0, 53_504),
            (
                2231, 131_072, 5667, 0, 753, 30_380_864, 41_532_843, 0, 55_552
            ),
            (
                2310, 131_072, 8737, 0, 817, 30_329_484, 41_001_769, 0, 54_272
            )
        ]
    );
    assert_eq!(
        fast,
        vec![
            (
                3384, 131_072, 34_517, 0, 1870, 25_055_860, 55_696_273, 0, 36_352
            ),
            (
                2712, 131_072, 12_219, 0, 1235, 21_105_498, 53_897_733, 0, 20_480
            ),
            (
                2937, 131_072, 15_870, 0, 1422, 17_173_137, 51_937_395, 0, 25_088
            ),
            (2881, 131_072, 0, 0, 1320, 13_077_615, 50_261_800, 0, 27_648),
            (2891, 131_072, 0, 0, 1299, 9_181_431, 48_681_841, 0, 26_112),
            (
                2905, 131_072, 29_884, 0, 1315, 5_811_907, 47_225_046, 0, 23_808
            ),
            (
                2979, 131_072, 21_071, 0, 1378, 3_235_436, 45_736_668, 0, 26_624
            ),
            (2904, 131_072, 0, 0, 1315, 1_650_530, 44_455_334, 0, 25_600),
            (2803, 131_072, 3569, 0, 1226, 730_468, 43_366_130, 0, 23_296),
            (2965, 131_072, 0, 0, 1352, 240_911, 42_154_786, 0, 26_368),
            (2911, 131_072, 0, 0, 1313, 66_734, 41_086_316, 0, 25_856),
            (2840, 131_072, 1724, 0, 1199, 23_326, 40_217_613, 0, 25_600),
            (2748, 131_072, 0, 0, 1088, 8141, 39_318_109, 0, 19_968),
            (2677, 131_072, 0, 0, 1059, 1274, 38_641_654, 0, 19_456),
            (2628, 131_072, 6687, 0, 1038, 0, 37_977_598, 0, 17_920),
            (2679, 131_072, 3085, 0, 1092, 0, 37_370_332, 0, 20_480)
        ]
    );
}

/// The weekly job's form of ADR-0053's measurement: the lattice at 1 024 units, eighty
/// windows (105 s of simulated time) under the controller's step of an eighth with the
/// pressure rising at shift 5 from zero, so that the day holds one night, at each of the two
/// periods.
#[test]
#[ignore]
fn a_waking_day_at_1024_units_at_two_target_periods_exhaustive() {
    let slow = day(&prior(1024), 80, 0x2000, 5, 20_000);
    let fast = day(&prior(1024), 80, 0x2000, 5, 5_000);
    eprintln!("DUMP day1024 slow {slow:?}");
    eprintln!("DUMP day1024 fast {fast:?}");
    assert_eq!(
        slow,
        vec![
            (
                12_348,
                137_040,
                41_666,
                0,
                6939,
                213_007_979,
                224_703_994,
                0,
                34_368
            ),
            (
                15_144,
                151_025,
                12_032,
                0,
                7923,
                212_942_578,
                211_653_188,
                0,
                20_672
            ),
            (
                26_443,
                162_543,
                25_553,
                0,
                16_014,
                213_754_269,
                184_433_713,
                0,
                640
            ),
            (
                35_621,
                142_225,
                1_048_576,
                0,
                23_552,
                213_898_981,
                153_757_307,
                0,
                0
            ),
            (
                15_829,
                160_003,
                0,
                0,
                6934,
                213_803_369,
                148_898_070,
                0,
                15_296
            ),
            (
                30_706,
                177_601,
                7874,
                0,
                18_563,
                213_881_550,
                135_596_026,
                0,
                192
            ),
            (
                47_909,
                155_401,
                1_048_576,
                0,
                35_208,
                213_902_976,
                120_093_347,
                0,
                0
            ),
            (
                24_803,
                174_826,
                0,
                0,
                12_853,
                213_899_305,
                117_199_819,
                0,
                1024
            ),
            (
                43_724,
                152_973,
                1_048_576,
                0,
                30_381,
                213_902_766,
                111_858_641,
                0,
                0
            ),
            (
                22_477,
                170_914,
                4047,
                0,
                10_863,
                213_893_162,
                111_147_013,
                0,
                1472
            ),
            (
                39_221,
                149_550,
                1_048_576,
                0,
                25_841,
                213_902_976,
                109_073_353,
                0,
                0
            ),
            (
                19_274,
                168_244,
                0,
                0,
                8439,
                213_864_723,
                109_156_327,
                0,
                5632
            ),
            (
                36_289,
                147_214,
                1_048_576,
                0,
                22_899,
                213_891_670,
                107_823_194,
                0,
                128
            ),
            (
                17_449,
                165_616,
                0,
                0,
                7040,
                213_798_783,
                108_053_146,
                0,
                10_240
            ),
            (
                33_660,
                144_914,
                1_048_576,
                0,
                20_216,
                213_882_167,
                107_326_113,
                0,
                128
            ),
            (
                15_715,
                163_028,
                0,
                0,
                6054,
                213_764_943,
                107_654_483,
                0,
                15_936
            ),
            (
                31_229,
                183_407,
                0,
                0,
                18_188,
                213_879_211,
                107_123_934,
                0,
                128
            ),
            (
                52_291,
                160_481,
                1_048_576,
                0,
                39_278,
                213_902_976,
                107_402_077,
                0,
                0
            ),
            (
                28_650,
                180_541,
                0,
                0,
                15_753,
                213_901_163,
                107_113_797,
                0,
                192
            ),
            (
                48_902,
                157_973,
                1_048_576,
                0,
                35_820,
                213_902_976,
                106_992_771,
                0,
                0
            ),
            (
                26_389,
                177_264,
                1515,
                0,
                13_950,
                213_894_843,
                106_996_848,
                0,
                832
            ),
            (
                45_499,
                155_106,
                1_048_576,
                0,
                32_053,
                213_902_976,
                106_587_198,
                0,
                0
            ),
            (
                23_835,
                174_494,
                0,
                0,
                11_992,
                213_889_133,
                106_821_192,
                0,
                1472
            ),
            (
                42_676,
                152_682,
                1_048_576,
                0,
                29_368,
                213_902_562,
                106_529_167,
                0,
                0
            ),
            (
                21_904,
                170_863,
                3100,
                0,
                10_302,
                213_889_344,
                106_747_050,
                0,
                1792
            ),
            (
                38_966,
                149_505,
                1_048_576,
                0,
                25_550,
                213_901_211,
                106_365_821,
                0,
                64
            ),
            (
                19_081,
                168_004,
                664,
                0,
                8157,
                213_847_798,
                106_598_309,
                0,
                5824
            ),
            (
                36_040,
                147_004,
                1_048_576,
                0,
                22_547,
                213_896_773,
                106_340_203,
                0,
                64
            ),
            (
                17_402,
                164_206,
                4180,
                0,
                7169,
                213_820_149,
                106_734_521,
                0,
                9728
            ),
            (
                32_466,
                143_680,
                1_048_576,
                0,
                18_951,
                213_888_611,
                106_642_818,
                0,
                64
            ),
            (
                14_721,
                157_898,
                13_656,
                0,
                5418,
                213_712_100,
                106_967_750,
                0,
                19_072
            ),
            (
                26_114,
                176_327,
                4342,
                0,
                13_649,
                213_803_914,
                106_829_779,
                0,
                768
            ),
            (
                44_489,
                154_286,
                1_048_576,
                0,
                31_094,
                213_902_059,
                106_610_033,
                0,
                0
            ),
            (
                23_202,
                173_572,
                0,
                0,
                11_415,
                213_886_377,
                106_812_142,
                0,
                1984
            ),
            (
                41_584,
                151_876,
                1_048_576,
                0,
                28_154,
                213_902_817,
                106_474_393,
                0,
                0
            ),
            (
                21_299,
                170_861,
                0,
                0,
                9886,
                213_878_713,
                106_783_953,
                0,
                2560
            ),
            (
                38_769,
                149_503,
                1_048_576,
                0,
                25_212,
                213_901_209,
                106_536_604,
                0,
                0
            ),
            (
                19_223,
                168_191,
                0,
                0,
                8363,
                213_847_248,
                106_738_365,
                0,
                5824
            ),
            (
                36_200,
                147_167,
                1_048_576,
                0,
                22_594,
                213_898_796,
                106_196_835,
                0,
                0
            ),
            (
                17_384,
                165_563,
                0,
                0,
                6931,
                213_804_884,
                106_724_266,
                0,
                9152
            ),
            (
                33_578,
                144_868,
                1_048_576,
                0,
                20_225,
                213_897_580,
                106_270_913,
                0,
                128
            ),
            (
                15_546,
                162_977,
                0,
                0,
                5915,
                213_779_830,
                106_810_723,
                0,
                15_040
            ),
            (
                30_877,
                183_349,
                0,
                0,
                17_806,
                213_886_623,
                106_407_237,
                0,
                64
            ),
            (
                51_771,
                160_430,
                1_048_576,
                0,
                38_733,
                213_902_976,
                106_941_270,
                0,
                0
            ),
            (
                28_615,
                180_484,
                0,
                0,
                15_695,
                213_901_240,
                106_939_712,
                0,
                192
            ),
            (
                49_080,
                157_924,
                1_048_576,
                0,
                36_031,
                213_902_976,
                106_813_690,
                0,
                0
            ),
            (
                26_443,
                177_665,
                0,
                0,
                14_050,
                213_898_423,
                106_808_385,
                0,
                128
            ),
            (
                46_094,
                155_457,
                1_048_576,
                0,
                32_782,
                213_902_976,
                106_404_697,
                0,
                0
            ),
            (
                24_070,
                174_889,
                0,
                0,
                11_961,
                213_889_859,
                106_750_833,
                0,
                1280
            ),
            (
                42_950,
                153_028,
                1_048_576,
                0,
                29_470,
                213_902_830,
                106_395_739,
                0,
                0
            ),
            (
                22_165,
                172_157,
                0,
                0,
                10_457,
                213_882_159,
                106_541_187,
                0,
                2624
            ),
            (
                40_137,
                150_637,
                1_048_576,
                0,
                26_533,
                213_901_907,
                106_214_718,
                0,
                0
            ),
            (
                20_185,
                169_467,
                0,
                0,
                9034,
                213_873_917,
                106_758_153,
                0,
                4096
            ),
            (
                37_462,
                148_284,
                1_048_576,
                0,
                23_881,
                213_900_813,
                106_294_682,
                0,
                0
            ),
            (
                18_117,
                166_820,
                0,
                0,
                7558,
                213_825_809,
                106_632_212,
                0,
                8384
            ),
            (
                35_017,
                145_968,
                1_048_576,
                0,
                21_428,
                213_897_368,
                106_427_250,
                0,
                64
            ),
            (
                16_463,
                164_214,
                0,
                0,
                6534,
                213_789_353,
                106_941_354,
                0,
                13_376
            ),
            (
                32_192,
                143_687,
                1_048_576,
                0,
                19_105,
                213_872_939,
                106_455_738,
                0,
                256
            ),
            (
                14_786,
                159_881,
                6449,
                0,
                5494,
                213_698_039,
                106_974_648,
                0,
                18_816
            ),
            (
                28_141,
                179_866,
                0,
                0,
                15_501,
                213_824_345,
                106_657_636,
                0,
                384
            ),
            (
                47_993,
                157_383,
                1_048_576,
                0,
                34_721,
                213_902_553,
                106_642_161,
                0,
                0
            ),
            (
                25_815,
                175_279,
                5919,
                0,
                13_294,
                213_897_135,
                106_511_492,
                0,
                576
            ),
            (
                43_410,
                153_369,
                1_048_576,
                0,
                29_959,
                213_902_976,
                106_123_719,
                0,
                0
            ),
            (
                22_158,
                172_540,
                0,
                0,
                10_558,
                213_884_110,
                106_493_441,
                0,
                1920
            ),
            (
                40_715,
                150_973,
                1_048_576,
                0,
                27_296,
                213_902_719,
                106_373_994,
                0,
                0
            ),
            (
                20_324,
                169_845,
                0,
                1,
                9197,
                213_868_235,
                106_661_201,
                0,
                3712
            ),
            (
                40_178,
                148_614,
                1_048_576,
                1,
                27_047,
                213_898_373,
                107_073_064,
                64,
                0
            ),
            (
                20_788,
                167_191,
                0,
                1,
                10_034,
                213_829_001,
                107_290_615,
                128,
                7360
            ),
            (
                37_096,
                146_292,
                1_048_576,
                1,
                23_722,
                213_887_202,
                106_828_376,
                192,
                0
            ),
            (
                18_711,
                163_145,
                5135,
                2,
                8591,
                213_790_845,
                107_153_590,
                256,
                11_264
            ),
            (
                31_158,
                180_068,
                11_155,
                2,
                18_013,
                213_890_420,
                106_403_261,
                256,
                128
            ),
            (
                48_430,
                157_560,
                1_048_576,
                1,
                35_175,
                213_902_976,
                106_624_398,
                256,
                0
            ),
            (
                28_261,
                177_255,
                0,
                0,
                15_816,
                213_896_718,
                107_015_294,
                320,
                576
            ),
            (
                45_427,
                155_098,
                1_048_576,
                0,
                32_135,
                213_902_976,
                106_737_693,
                320,
                0
            ),
            (
                23_711,
                168_387,
                20_612,
                0,
                11_718,
                213_895_251,
                107_017_922,
                320,
                896
            ),
            (
                36_253,
                147_339,
                1_048_576,
                0,
                22_809,
                213_894_909,
                106_374_193,
                320,
                64
            ),
            (
                17_509,
                165_756,
                0,
                0,
                7299,
                213_814_913,
                106_680_832,
                320,
                9344
            ),
            (
                33_620,
                145_037,
                1_048_576,
                0,
                20_168,
                213_893_850,
                106_198_083,
                320,
                64
            ),
            (
                15_733,
                160_318,
                10_299,
                0,
                6161,
                213_754_866,
                106_614_560,
                320,
                15_360
            ),
            (
                28_360,
                180_358,
                0,
                0,
                15_508,
                213_860_060,
                106_663_406,
                320,
                256
            )
        ]
    );
    assert_eq!(
        fast,
        vec![
            (
                12_406,
                135_634,
                47_290,
                0,
                6885,
                196_957_504,
                224_615_789,
                0,
                31_680
            ),
            (
                14_330,
                151_318,
                4914,
                0,
                7382,
                178_501_613,
                212_506_276,
                0,
                42_240
            ),
            (
                28_372,
                160_524,
                33_642,
                0,
                17_794,
                152_239_524,
                181_721_720,
                0,
                65_152
            ),
            (
                36_671,
                140_459,
                1_048_576,
                0,
                24_801,
                127_211_318,
                150_772_259,
                0,
                64_192
            ),
            (
                15_800,
                155_627,
                8922,
                0,
                6817,
                108_171_936,
                146_127_107,
                0,
                51_584
            ),
            (
                31_154,
                175_080,
                0,
                0,
                19_470,
                83_304_927,
                133_215_997,
                0,
                65_472
            ),
            (
                55_164,
                153_195,
                1_048_576,
                0,
                44_077,
                69_491_277,
                117_965_643,
                0,
                22_784
            ),
            (
                28_450,
                172_167,
                604,
                0,
                16_666,
                47_147_861,
                114_883_600,
                0,
                65_536
            ),
            (
                53_491,
                150_646,
                1_048_576,
                0,
                42_360,
                34_880_109,
                111_437_201,
                0,
                28_288
            ),
            (
                26_967,
                164_109,
                18_681,
                0,
                15_408,
                18_462_997,
                110_309_882,
                0,
                65_536
            ),
            (
                45_352,
                143_595,
                1_048_576,
                0,
                33_812,
                9_902_897,
                109_568_182,
                0,
                54_656
            ),
            (
                19_937,
                157_121,
                16_150,
                0,
                9691,
                3_954_604,
                109_542_773,
                0,
                62_848
            ),
            (
                36_762,
                137_481,
                1_048_576,
                0,
                24_975,
                1_337_798,
                108_458_088,
                0,
                64_960
            ),
            (
                14_079,
                151_301,
                12_830,
                0,
                5571,
                492_512,
                108_582_758,
                0,
                42_624
            ),
            (
                29_563,
                165_926,
                14_859,
                0,
                18_001,
                126_833,
                108_041_847,
                0,
                65_536
            ),
            (
                48_395,
                145_185,
                1_048_576,
                0,
                37_024,
                158_719,
                108_768_385,
                0,
                46_464
            ),
            (
                21_854,
                161_769,
                5651,
                0,
                11_250,
                8118,
                108_730_271,
                0,
                64_640
            ),
            (
                43_261,
                141_548,
                1_048_576,
                0,
                31_617,
                27_824,
                108_326_118,
                0,
                59_200
            ),
            (
                17_961,
                151_736,
                27_798,
                0,
                8217,
                951,
                108_248_562,
                0,
                58_752
            ),
            (
                29_870,
                166_313,
                15_168,
                0,
                18_317,
                1795,
                107_803_684,
                0,
                65_408
            ),
            (
                49_159,
                145_524,
                1_048_576,
                0,
                38_023,
                132_523,
                108_620_682,
                0,
                42_048
            ),
            (22_412, 163_715, 0, 0, 11_758, 2637, 108_530_403, 0, 64_768),
            (
                45_772,
                143_251,
                1_048_576,
                0,
                34_466,
                60_384,
                108_514_549,
                0,
                54_144
            ),
            (19_864, 157_977, 11_640, 0, 9759, 0, 108_476_414, 0, 62_400),
            (
                38_246,
                138_230,
                1_048_576,
                0,
                26_507,
                6016,
                108_054_442,
                0,
                64_384
            ),
            (14_944, 155_509, 0, 0, 6101, 0, 108_265_008, 0, 48_768),
            (
                34_727,
                136_070,
                1_048_576,
                0,
                22_936,
                2763,
                107_741_600,
                0,
                65_088
            ),
            (12_876, 153_079, 0, 0, 4854, 0, 108_012_999, 0, 34_880),
            (
                32_074,
                133_944,
                1_048_576,
                0,
                20_404,
                1811,
                107_545_195,
                0,
                65_536
            ),
            (11_098, 150_687, 0, 0, 3821, 0, 107_891_144, 0, 22_016),
            (
                28_870,
                158_433,
                38_587,
                0,
                17_391,
                230,
                107_410_549,
                0,
                65_472
            ),
            (
                38_515,
                138_629,
                1_048_576,
                0,
                26_693,
                6013,
                107_578_292,
                0,
                64_512
            ),
            (15_014, 155_958, 0, 0, 6338, 0, 107_796_882, 0, 48_448),
            (
                35_480,
                136_463,
                1_048_576,
                0,
                23_703,
                12_143,
                107_523_745,
                0,
                65_280
            ),
            (13_197, 150_412, 11_945, 0, 5151, 0, 107_835_040, 0, 36_032),
            (
                28_856,
                161_348,
                27_413,
                0,
                17_411,
                0,
                107_257_948,
                0,
                65_536
            ),
            (
                42_245,
                141_180,
                1_048_576,
                0,
                30_438,
                17_694,
                107_870_717,
                0,
                61_376
            ),
            (17_724, 151_298, 27_961, 0, 8248, 0, 107_850_408, 0, 59_264),
            (
                29_485,
                162_079,
                28_177,
                0,
                17_861,
                0,
                107_359_995,
                0,
                65_536
            ),
            (
                43_213,
                141_819,
                1_048_576,
                0,
                31_601,
                17_640,
                107_929_588,
                0,
                59_520
            ),
            (18_234, 159_520, 98, 0, 8496, 0, 108_075_951, 0, 60_288),
            (
                40_253,
                139_580,
                1_048_576,
                0,
                28_535,
                11_318,
                107_991_768,
                0,
                63_360
            ),
            (15_890, 148_662, 31_422, 0, 6818, 0, 108_236_806, 0, 52_096),
            (
                26_418,
                163_588,
                12_899,
                0,
                15_142,
                0,
                107_670_245,
                0,
                65_280
            ),
            (
                45_383,
                143_140,
                1_048_576,
                0,
                34_002,
                47_391,
                108_508_863,
                0,
                55_168
            ),
            (19_784, 161_033, 0, 0, 9816, 62, 108_489_193, 0, 62_272),
            (
                42_122,
                140_904,
                1_048_576,
                0,
                30_517,
                18_655,
                108_398_295,
                0,
                61_056
            ),
            (17_641, 157_592, 3440, 0, 8086, 362, 108_318_020, 0, 57_664),
            (
                37_672,
                137_893,
                1_048_576,
                0,
                25_857,
                6597,
                108_283_178,
                0,
                64_640
            ),
            (14_569, 155_130, 0, 0, 5976, 0, 108_358_192, 0, 46_720),
            (
                34_440,
                135_739,
                1_048_576,
                0,
                22_704,
                216,
                107_722_735,
                0,
                65_344
            ),
            (12_645, 146_721, 23_122, 0, 4748, 0, 108_077_798, 0, 32_704),
            (
                24_247,
                157_933,
                25_474,
                0,
                13_412,
                0,
                107_671_347,
                0,
                65_088
            ),
            (
                38_007,
                138_191,
                1_048_576,
                0,
                26_175,
                6542,
                107_584_573,
                0,
                64_384
            ),
            (14_622, 155_465, 0, 0, 5956, 0, 107_817_708, 0, 47_040),
            (
                35_057,
                136_032,
                1_048_576,
                0,
                23_153,
                1766,
                107_615_428,
                0,
                65_408
            ),
            (12_695, 153_036, 0, 0, 4745, 0, 107_964_833, 0, 33_152),
            (
                31_819,
                133_907,
                1_048_576,
                0,
                20_092,
                4270,
                107_634_538,
                0,
                65_536
            ),
            (11_024, 146_189, 17_451, 0, 3873, 0, 108_079_394, 0, 21_632),
            (
                23_650,
                158_128,
                22_718,
                0,
                12_924,
                260,
                107_803_550,
                0,
                65_152
            ),
            (
                37_875,
                138_362,
                1_048_576,
                0,
                25_969,
                8688,
                107_585_545,
                0,
                64_576
            ),
            (15_096, 154_281, 5212, 0, 6394, 0, 107_816_818, 0, 48_832),
            (
                33_366,
                134_996,
                1_048_576,
                0,
                21_553,
                1717,
                107_453_814,
                0,
                65_472
            ),
            (11_920, 151_871, 0, 0, 4320, 0, 107_673_179, 0, 28_672),
            (
                30_675,
                159_115,
                40_530,
                0,
                19_118,
                657,
                107_397_444,
                0,
                65_536
            ),
            (
                39_481,
                139_226,
                1_048_576,
                1,
                27_620,
                15_135,
                107_727_052,
                0,
                63_296
            ),
            (
                18_090,
                153_674,
                11_131,
                1,
                8791,
                10_101,
                108_193_733,
                64,
                52_480
            ),
            (
                34_646,
                134_465,
                1_048_576,
                1,
                23_104,
                207_608,
                107_677_262,
                128,
                64_768
            ),
            (
                13_558,
                147_079,
                16_354,
                1,
                5809,
                229_358,
                107_984_831,
                192,
                23_360
            ),
            (
                26_571,
                157_679,
                27_752,
                2,
                15_684,
                351_081,
                107_608_062,
                256,
                64_448
            ),
            (
                37_426,
                137_969,
                1_048_576,
                2,
                25_719,
                159_764,
                107_558_756,
                256,
                64_704
            ),
            (
                14_406,
                153_647,
                5958,
                1,
                5783,
                34_054,
                107_898_466,
                256,
                43_840
            ),
            (
                34_651,
                134_441,
                1_048_576,
                0,
                22_893,
                236_518,
                107_786_429,
                320,
                64_640
            ),
            (
                11_530,
                150_555,
                2693,
                0,
                4120,
                119_813,
                108_112_474,
                320,
                25_728
            ),
            (
                28_812,
                157_681,
                40_723,
                0,
                17_473,
                17_635,
                107_787_823,
                320,
                65_472
            ),
            (
                37_528,
                137_971,
                1_048_576,
                0,
                25_735,
                9578,
                107_331_457,
                320,
                64_512
            ),
            (
                14_560,
                153_443,
                6745,
                0,
                5973,
                638,
                107_681_859,
                320,
                45_760
            ),
            (
                32_358,
                134_263,
                1_048_576,
                0,
                20_607,
                1472,
                107_098_826,
                320,
                65_408
            ),
            (11_260, 151_046, 0, 0, 3960, 0, 107_456_207, 320, 21_696),
            (
                29_539,
                160_037,
                34_326,
                0,
                18_121,
                4,
                107_440_092,
                320,
                65_536
            )
        ]
    );
}

/// The weekly job's form of ADR-0057's measurement: the below-rail prior at 1 024 units,
/// eighty windows under the controller's step of an eighth with the pressure rising at
/// shift 5 from zero, at each of the two periods.
#[test]
#[ignore]
fn a_waking_day_from_below_the_rail_at_1024_units_at_two_target_periods_exhaustive() {
    let slow = day(&prior_below_rail(1024), 80, 0x2000, 5, 20_000);
    let fast = day(&prior_below_rail(1024), 80, 0x2000, 5, 5_000);
    eprintln!("DUMP below1024 slow {slow:?}");
    eprintln!("DUMP below1024 fast {fast:?}");
    assert_eq!(
        slow,
        vec![
            (
                13_804,
                135_484,
                47_889,
                0,
                8023,
                121_485_462,
                221_765_390,
                0,
                25_856
            ),
            (
                15_113,
                152_420,
                0,
                0,
                7991,
                124_627_128,
                208_788_813,
                0,
                19_648
            ),
            (
                30_491,
                160_609,
                37_368,
                0,
                19_693,
                144_636_628,
                175_817_224,
                0,
                64
            ),
            (
                35_430,
                140_533,
                1_048_576,
                0,
                23_360,
                170_742_643,
                148_497_805,
                0,
                0
            ),
            (
                14_903,
                154_915,
                11_883,
                0,
                6210,
                172_743_786,
                144_722_693,
                0,
                19_456
            ),
            (
                26_699,
                172_580,
                5751,
                0,
                15_122,
                184_025_744,
                134_976_759,
                0,
                448
            ),
            (
                43_674,
                151_008,
                1_048_576,
                0,
                30_675,
                204_944_951,
                121_151_483,
                0,
                0
            ),
            (
                21_219,
                169_884,
                0,
                0,
                10_068,
                206_822_053,
                118_856_881,
                0,
                2880
            ),
            (
                38_849,
                148_649,
                1_048_576,
                0,
                25_503,
                211_734_153,
                113_474_326,
                0,
                0
            ),
            (
                18_949,
                167_230,
                0,
                0,
                8332,
                212_029_803,
                112_879_614,
                0,
                5376
            ),
            (
                35_732,
                146_326,
                1_048_576,
                0,
                22_518,
                213_132_328,
                110_142_986,
                0,
                0
            ),
            (
                16_972,
                164_617,
                0,
                0,
                6877,
                213_113_877,
                110_263_206,
                0,
                10_560
            ),
            (
                32_784,
                144_040,
                1_048_576,
                0,
                19_554,
                213_527_006,
                108_684_283,
                0,
                128
            ),
            (
                15_147,
                162_045,
                0,
                0,
                5677,
                213_388_671,
                108_937_893,
                0,
                19_072
            ),
            (
                30_202,
                182_301,
                0,
                0,
                17_165,
                213_662_102,
                108_091_563,
                0,
                320
            ),
            (
                50_830,
                159_513,
                1_048_576,
                0,
                37_885,
                213_866_255,
                107_624_529,
                0,
                0
            ),
            (
                27_747,
                179_452,
                0,
                0,
                15_078,
                213_877_056,
                107_390_582,
                0,
                256
            ),
            (
                47_821,
                157_021,
                1_048_576,
                0,
                34_467,
                213_900_485,
                106_944_671,
                0,
                0
            ),
            (
                25_604,
                176_649,
                0,
                0,
                13_241,
                213_885_996,
                106_896_553,
                0,
                640
            ),
            (
                44_673,
                154_568,
                1_048_576,
                0,
                31_161,
                213_902_976,
                106_660_294,
                0,
                0
            ),
            (
                23_343,
                173_889,
                0,
                0,
                11_422,
                213_886_334,
                106_739_180,
                0,
                1920
            ),
            (
                41_932,
                152_153,
                1_048_576,
                0,
                28_354,
                213_902_654,
                106_447_852,
                0,
                0
            ),
            (
                21_334,
                171_172,
                0,
                0,
                9928,
                213_872_991,
                106_652_869,
                0,
                3136
            ),
            (
                39_080,
                149_776,
                1_048_576,
                0,
                25_650,
                213_901_686,
                106_289_878,
                0,
                0
            ),
            (
                19_428,
                167_042,
                5098,
                0,
                8464,
                213_867_978,
                106_721_202,
                0,
                5312
            ),
            (
                35_086,
                146_162,
                1_048_576,
                0,
                21_671,
                213_895_421,
                106_275_469,
                0,
                128
            ),
            (
                16_549,
                161_607,
                10_136,
                0,
                6427,
                213_787_926,
                106_721_745,
                0,
                12_608
            ),
            (
                29_731,
                177_456,
                14_116,
                0,
                16_632,
                213_870_759,
                106_399_910,
                0,
                64
            ),
            (
                45_886,
                155_274,
                1_048_576,
                0,
                32_573,
                213_902_976,
                106_333_386,
                0,
                0
            ),
            (
                24_067,
                172_309,
                8013,
                0,
                11_894,
                213_889_099,
                106_572_953,
                0,
                1088
            ),
            (
                40_369,
                150_770,
                1_048_576,
                0,
                26_783,
                213_902_976,
                106_357_000,
                0,
                0
            ),
            (
                19_976,
                169_616,
                0,
                0,
                8889,
                213_875_294,
                106_770_568,
                0,
                4672
            ),
            (
                37_477,
                148_414,
                1_048_576,
                0,
                23_967,
                213_901_570,
                106_446_346,
                0,
                0
            ),
            (
                18_369,
                166_966,
                0,
                0,
                7874,
                213_837_831,
                106_804_609,
                0,
                7680
            ),
            (
                34_953,
                146_095,
                1_048_576,
                0,
                21_674,
                213_898_113,
                106_375_834,
                0,
                0
            ),
            (
                16_640,
                164_357,
                0,
                0,
                6558,
                213_798_844,
                106_748_692,
                0,
                11_520
            ),
            (
                32_249,
                143_812,
                1_048_576,
                0,
                19_010,
                213_889_801,
                106_325_107,
                0,
                0
            ),
            (
                14_908,
                161_067,
                2628,
                0,
                5544,
                213_713_713,
                106_711_111,
                0,
                18_432
            ),
            (
                29_368,
                179_942,
                4098,
                0,
                16_344,
                213_862_442,
                106_183_408,
                0,
                256
            ),
            (
                48_306,
                157_449,
                1_048_576,
                0,
                34_855,
                213_901_666,
                106_392_608,
                0,
                0
            ),
            (
                25_896,
                177_130,
                0,
                0,
                13_420,
                213_897_344,
                106_544_896,
                0,
                384
            ),
            (
                45_304,
                154_989,
                1_048_576,
                0,
                31_934,
                213_902_976,
                106_309_990,
                0,
                0
            ),
            (
                23_519,
                174_363,
                0,
                0,
                11_527,
                213_895_159,
                106_684_580,
                0,
                960
            ),
            (
                42_183,
                152_568,
                1_048_576,
                0,
                28_514,
                213_901_529,
                106_551_972,
                0,
                0
            ),
            (
                21_630,
                171_639,
                0,
                0,
                10_120,
                213_885_075,
                106_906_333,
                0,
                2560
            ),
            (
                39_648,
                150_184,
                1_048_576,
                0,
                26_113,
                213_902_941,
                106_190_556,
                0,
                0
            ),
            (
                19_794,
                168_957,
                0,
                0,
                8727,
                213_858_732,
                106_537_327,
                0,
                4736
            ),
            (
                36_835,
                147_837,
                1_048_576,
                0,
                23_385,
                213_896_625,
                106_008_514,
                0,
                64
            ),
            (
                17_788,
                165_917,
                1412,
                0,
                7323,
                213_827_303,
                106_556_015,
                0,
                7616
            ),
            (
                33_745,
                145_177,
                1_048_576,
                0,
                20_302,
                213_896_256,
                106_167_045,
                0,
                0
            ),
            (
                15_836,
                162_981,
                1238,
                0,
                6082,
                213_769_296,
                106_671_568,
                0,
                14_464
            ),
            (
                31_113,
                183_354,
                0,
                0,
                17_711,
                213_872_632,
                106_189_433,
                0,
                0
            ),
            (
                52_241,
                160_435,
                1_048_576,
                0,
                39_435,
                213_902_976,
                106_803_882,
                0,
                0
            ),
            (
                28_666,
                180_489,
                0,
                0,
                15_738,
                213_900_896,
                106_680_566,
                0,
                384
            ),
            (
                48_746,
                157_928,
                1_048_576,
                0,
                35_596,
                213_902_476,
                106_948_739,
                0,
                0
            ),
            (
                26_387,
                175_127,
                8441,
                0,
                13_757,
                213_901_297,
                107_012_305,
                0,
                384
            ),
            (
                43_184,
                153_236,
                1_048_576,
                0,
                29_690,
                213_902_865,
                106_592_991,
                0,
                0
            ),
            (
                22_218,
                171_502,
                3040,
                0,
                10_390,
                213_885_956,
                106_785_925,
                0,
                2048
            ),
            (
                39_644,
                150_064,
                1_048_576,
                0,
                26_259,
                213_902_532,
                106_432_133,
                0,
                0
            ),
            (
                19_690,
                168_822,
                0,
                0,
                8802,
                213_864_542,
                106_828_505,
                0,
                4544
            ),
            (
                36_603,
                147_719,
                1_048_576,
                0,
                23_113,
                213_899_691,
                106_311_589,
                0,
                0
            ),
            (
                17_850,
                165_918,
                942,
                0,
                7450,
                213_823_851,
                106_580_407,
                0,
                7808
            ),
            (
                33_831,
                145_178,
                1_048_576,
                0,
                20_343,
                213_891_231,
                106_188_665,
                0,
                0
            ),
            (
                15_744,
                163_325,
                0,
                0,
                6089,
                213_781_518,
                106_543_992,
                0,
                16_000
            ),
            (
                31_568,
                183_741,
                0,
                0,
                18_549,
                213_875_375,
                106_192_940,
                0,
                384
            ),
            (
                52_460,
                160_773,
                1_048_576,
                1,
                39_566,
                213_902_976,
                106_916_791,
                0,
                0
            ),
            (
                31_321,
                180_870,
                0,
                1,
                18_707,
                213_902_024,
                107_378_558,
                64,
                192
            ),
            (
                51_694,
                158_261,
                1_048_576,
                1,
                38_873,
                213_902_976,
                107_209_076,
                128,
                0
            ),
            (
                28_589,
                178_044,
                0,
                1,
                16_095,
                213_897_402,
                106_934_956,
                192,
                384
            ),
            (
                48_528,
                155_789,
                1_048_576,
                2,
                35_444,
                213_902_976,
                106_931_776,
                256,
                0
            ),
            (
                24_378,
                172_367,
                9745,
                2,
                12_235,
                213_897_287,
                106_777_167,
                256,
                832
            ),
            (
                40_243,
                150_821,
                1_048_576,
                1,
                26_733,
                213_898_819,
                106_406_681,
                256,
                0
            ),
            (
                22_465,
                169_674,
                0,
                0,
                11_160,
                213_876_860,
                106_838_674,
                320,
                3968
            ),
            (
                37_598,
                148_465,
                1_048_576,
                0,
                24_050,
                213_902_505,
                106_441_033,
                320,
                0
            ),
            (
                18_116,
                159_457,
                26_719,
                0,
                7582,
                213_843_710,
                106_859_807,
                320,
                6848
            ),
            (
                27_820,
                179_389,
                0,
                0,
                15_203,
                213_851_767,
                106_291_317,
                320,
                320
            ),
            (
                47_785,
                156_965,
                1_048_576,
                0,
                34_439,
                213_902_976,
                106_144_086,
                320,
                0
            ),
            (
                25_445,
                175_970,
                2053,
                0,
                12_945,
                213_891_740,
                106_169_089,
                320,
                1024
            ),
            (
                44_001,
                153_974,
                1_048_576,
                0,
                30_375,
                213_902_854,
                106_177_857,
                320,
                0
            ),
            (
                22_779,
                173_221,
                0,
                0,
                10_967,
                213_887_151,
                106_580_663,
                320,
                1792
            )
        ]
    );
    assert_eq!(
        fast,
        vec![
            (
                14_024,
                136_814,
                42_566,
                0,
                8181,
                100_972_360,
                221_402_896,
                0,
                40_064
            ),
            (
                17_130,
                152_807,
                4250,
                0,
                9383,
                81_432_836,
                205_244_201,
                0,
                54_784
            ),
            (
                34_047,
                133_706,
                1_048_576,
                0,
                23_349,
                58_042_270,
                167_875_407,
                0,
                65_088
            ),
            (
                12_111,
                143_332,
                27_792,
                0,
                4899,
                42_466_571,
                163_664_689,
                0,
                28_352
            ),
            (
                22_607,
                155_451,
                21_206,
                0,
                12_670,
                24_461_597,
                151_600_524,
                0,
                64_640
            ),
            (
                36_464,
                136_020,
                1_048_576,
                0,
                25_232,
                11_822_915,
                133_248_033,
                0,
                64_960
            ),
            (
                13_770,
                152_475,
                2111,
                0,
                5798,
                5_598_602,
                131_136_452,
                0,
                41_088
            ),
            (
                32_794,
                133_416,
                1_048_576,
                0,
                21_564,
                1_637_913,
                122_245_537,
                0,
                65_536
            ),
            (
                11_151,
                145_138,
                19_472,
                0,
                4074,
                547_075,
                121_501_264,
                0,
                23_616
            ),
            (
                23_339,
                153_374,
                35_785,
                0,
                12_852,
                96_411,
                117_962_682,
                0,
                65_280
            ),
            (
                32_789,
                134_202,
                1_048_576,
                0,
                21_221,
                14_010,
                114_053_829,
                0,
                65_408
            ),
            (11_536, 150_977, 0, 0, 4111, 1383, 113_916_988, 0, 25_216),
            (
                29_771,
                160_597,
                32_129,
                0,
                18_409,
                1531,
                111_525_384,
                0,
                65_536
            ),
            (
                41_635,
                140_522,
                1_048_576,
                0,
                29_910,
                13_275,
                109_950_565,
                0,
                61_824
            ),
            (17_113, 150_881, 26_885, 0, 7737, 0, 109_780_503, 0, 57_024),
            (
                28_899,
                166_394,
                11_629,
                0,
                17_483,
                152,
                108_905_611,
                0,
                65_536
            ),
            (
                49_462,
                145_595,
                1_048_576,
                0,
                38_331,
                135_414,
                109_251_990,
                0,
                42_112
            ),
            (
                22_461,
                160_855,
                10_586,
                0,
                11_696,
                0,
                108_956_197,
                0,
                64_256
            ),
            (
                41_945,
                140_748,
                1_048_576,
                0,
                30_239,
                29_946,
                108_496_653,
                0,
                61_760
            ),
            (17_464, 156_613, 6438, 0, 7961, 0, 108_533_240, 0, 57_280),
            (
                36_344,
                137_036,
                1_048_576,
                0,
                24_582,
                6404,
                108_020_348,
                0,
                64_896
            ),
            (13_727, 154_166, 0, 0, 5506, 0, 108_297_678, 0, 41_600),
            (
                33_206,
                134_895,
                1_048_576,
                0,
                21_671,
                3114,
                107_759_603,
                0,
                65_472
            ),
            (11_842, 151_757, 0, 0, 4227, 114, 108_050_570, 0, 28_032),
            (
                30_601,
                162_152,
                29_626,
                0,
                19_101,
                348,
                107_641_495,
                0,
                65_536
            ),
            (
                43_612,
                141_883,
                1_048_576,
                0,
                31_948,
                29_720,
                108_029_945,
                0,
                58_304
            ),
            (18_262, 153_046, 24_289, 0, 8539, 0, 108_058_358, 0, 59_584),
            (
                31_622,
                165_407,
                23_195,
                0,
                19_897,
                473,
                107_621_325,
                0,
                65_536
            ),
            (
                47_980,
                144_731,
                1_048_576,
                0,
                36_684,
                100_798,
                108_372_289,
                0,
                47_104
            ),
            (21_615, 162_822, 0, 0, 11_115, 2007, 108_207_808, 0, 64_000),
            (
                44_448,
                142_469,
                1_048_576,
                0,
                32_837,
                49_151,
                108_320_817,
                0,
                56_704
            ),
            (18_909, 158_362, 7049, 0, 8950, 0, 108_271_477, 0, 60_864),
            (
                38_468,
                138_567,
                1_048_576,
                0,
                26_622,
                13_231,
                107_857_248,
                0,
                64_256
            ),
            (14_935, 150_357, 20_927, 0, 6212, 59, 108_162_202, 0, 48_128),
            (
                28_436,
                163_127,
                21_005,
                0,
                16_996,
                15,
                107_626_306,
                0,
                65_472
            ),
            (
                45_105,
                142_736,
                1_048_576,
                0,
                33_535,
                51_340,
                107_988_753,
                0,
                56_320
            ),
            (19_372, 160_578, 0, 0, 9365, 381, 108_051_787, 0, 62_336),
            (
                41_463,
                140_506,
                1_048_576,
                0,
                29_815,
                14_120,
                108_355_012,
                0,
                61_824
            ),
            (
                17_053,
                151_880,
                23_097,
                0,
                7548,
                248,
                108_245_626,
                0,
                56_640
            ),
            (
                30_135,
                164_578,
                21_700,
                0,
                18_415,
                1078,
                107_834_679,
                0,
                65_536
            ),
            (
                46_881,
                144_006,
                1_048_576,
                0,
                35_506,
                66_443,
                108_480_068,
                0,
                51_328
            ),
            (20_633, 162_007, 0, 0, 10_257, 1507, 108_517_380, 0, 63_616),
            (
                43_334,
                141_756,
                1_048_576,
                0,
                31_709,
                25_623,
                108_456_414,
                0,
                59_264
            ),
            (18_328, 153_772, 21_094, 0, 8549, 0, 108_452_588, 0, 60_160),
            (
                32_520,
                134_551,
                1_048_576,
                0,
                20_839,
                263,
                108_165_187,
                0,
                65_536
            ),
            (11_757, 151_370, 0, 0, 4232, 0, 108_547_281, 0, 28_224),
            (
                29_844,
                159_678,
                36_762,
                0,
                18_342,
                1192,
                108_058_559,
                0,
                65_536
            ),
            (
                40_380,
                139_718,
                1_048_576,
                0,
                28_635,
                9535,
                107_966_982,
                0,
                62_848
            ),
            (16_151, 156_262, 3457, 0, 6982, 0, 108_225_150, 0, 53_696),
            (
                35_927,
                136_729,
                1_048_576,
                0,
                24_038,
                4174,
                107_781_877,
                0,
                65_024
            ),
            (13_579, 146_843, 26_752, 0, 5258, 0, 108_133_959, 0, 39_936),
            (
                24_223,
                164_271,
                3310,
                0,
                13_165,
                418,
                107_630_055,
                0,
                65_216
            ),
            (
                46_685,
                143_737,
                1_048_576,
                0,
                35_199,
                64_931,
                108_306_449,
                0,
                51_392
            ),
            (
                20_419,
                156_791,
                17_922,
                0,
                10_084,
                40,
                108_253_764,
                0,
                63_552
            ),
            (
                36_337,
                137_192,
                1_048_576,
                0,
                24_588,
                5546,
                107_955_814,
                0,
                64_640
            ),
            (13_978, 150_389, 15_102, 0, 5507, 0, 108_264_161, 0, 42_432),
            (
                28_447,
                162_629,
                22_862,
                0,
                16_998,
                0,
                107_829_112,
                0,
                65_536
            ),
            (
                44_243,
                142_300,
                1_048_576,
                0,
                32_713,
                43_561,
                108_201_151,
                0,
                56_448
            ),
            (18_907, 153_706, 23_508, 0, 9158, 0, 108_211_917, 0, 61_312),
            (
                32_471,
                134_493,
                1_048_576,
                0,
                20_718,
                141,
                107_861_425,
                0,
                65_408
            ),
            (11_432, 150_359, 3685, 0, 4093, 0, 108_260_493, 0, 24_832),
            (
                28_774,
                162_372,
                23_644,
                0,
                17_454,
                178,
                107_587_237,
                0,
                65_536
            ),
            (
                43_767,
                142_076,
                1_048_576,
                0,
                32_073,
                45_056,
                108_110_588,
                0,
                58_112
            ),
            (18_710, 159_836, 0, 0, 8864, 214, 108_061_616, 0, 60_032),
            (
                40_588,
                139_857,
                1_048_576,
                0,
                28_792,
                10_939,
                108_235_306,
                0,
                62_976
            ),
            (16_310, 152_979, 16_341, 1, 7155, 54, 108_332_204, 0, 53_696),
            (
                33_921,
                133_857,
                1_048_576,
                1,
                22_296,
                103_237,
                108_269_528,
                64,
                64_768
            ),
            (
                13_222,
                150_589,
                0,
                1,
                5651,
                132_589,
                108_558_181,
                128,
                20_736
            ),
            (
                30_725,
                160_506,
                31_004,
                1,
                19_232,
                268_428,
                108_029_801,
                192,
                64_768
            ),
            (
                43_283,
                140_443,
                1_048_576,
                2,
                31_633,
                615_200,
                108_284_527,
                256,
                61_568
            ),
            (
                16_870,
                154_904,
                11_550,
                2,
                7539,
                407_868,
                108_310_994,
                256,
                56_256
            ),
            (
                33_986,
                135_541,
                1_048_576,
                1,
                22_285,
                134_358,
                107_850_961,
                256,
                65_344
            ),
            (
                14_613,
                150_327,
                8345,
                0,
                6392,
                141_347,
                108_345_405,
                320,
                31_680
            ),
            (
                28_386,
                162_220,
                24_056,
                0,
                17_056,
                40_235,
                107_927_859,
                320,
                65_536
            ),
            (
                43_658,
                141_943,
                1_048_576,
                0,
                32_053,
                42_091,
                108_326_106,
                320,
                59_584
            ),
            (
                18_549,
                157_804,
                6952,
                0,
                8680,
                3482,
                108_157_363,
                320,
                61_120
            ),
            (
                37_926,
                138_079,
                1_048_576,
                0,
                26_117,
                16_141,
                107_759_147,
                320,
                64_384
            ),
            (14_752, 155_339, 0, 0, 5995, 0, 107_987_966, 320, 46_336),
            (
                34_676,
                135_922,
                1_048_576,
                0,
                22_931,
                4175,
                107_629_553,
                320,
                65_280
            ),
            (
                12_828,
                146_887,
                23_241,
                0,
                4901,
                0,
                108_037_013,
                320,
                34_944
            )
        ]
    );
}

/// The gate's form of ADR-0047's measurement: the random prior at 256 units and a gain of
/// 2.0, two windows and four kicks, no loop.
#[test]
fn the_estimate_the_causal_ratio_and_the_fine_slopes_on_the_random_prior_at_256_units() {
    let c = criticality(&random_prior(256), &GAINS[1..2], 2, 4, 0x0002_0000, 0);
    dump(
        "the_estimate_the_causal_ratio_and_the_fine_slopes_on_the_random_prior_at_256_units",
        &c,
    );
    // At a gain of 2.0 the random network spikes 2 738 then 2 256 times per window, its
    // estimate 0.232 then 0.040 (the train's bins give the same); four kicks are twelve
    // ancestor spikes and one descendant, itself an advanced spike: a gross ratio of 0.083
    // and a net one of 0; the fine slopes fall from 0.676 at lag one to 0 at lag eight.
    assert_eq!(
        readings(&c),
        vec![(
            0x0002_0000,
            vec![
                (0x0002_0000, 15_190, 2_738, 1_308),
                (0x0002_0000, 2_620, 2_256, 928)
            ],
            Attribution {
                kicks: 4,
                ancestors: 12,
                descendants: 1,
                advanced: 1,
                extra: 22,
                missing: 24
            },
            Some(5_461),
            Some(0),
            [Some(44_309), Some(34_199), Some(12_333), Some(0), Some(0)],
            vec![Some(15_190), Some(2_620)],
            vec![Some(31_307), Some(26_958)]
        )]
    );
    assert!(c.closed.is_empty());
}

/// The weekly job's form at 256 units: three gains, two windows and eight kicks each, the
/// loop from 1.0 for ten windows, on both priors.
#[test]
#[ignore]
fn the_estimate_the_causal_ratios_and_the_loop_at_256_units_exhaustive() {
    let c = criticality(&prior(256), &GAINS, 2, 8, MODULATION_ONE_Q16 as u32, 10);
    dump(
        "the_estimate_the_causal_ratios_and_the_loop_at_256_units_exhaustive",
        &c,
    );
    // At fixed gains, two windows each: the lag-one estimate per window (Q16.16) and the
    // window's spikes, then eight kicks attributed through the connectome. The estimate of
    // one window of thirty-two bins varies by more than itself between two windows at one
    // gain (0.659 then 0.000 at 2.0); the causal ratios are of its order and share no trend
    // with the gain (gross 0.389, 0.300, 0.583; net 0.389, 0.200, 0.306: both dip at 2.0)
    // and stay below 1 up to the ceiling's neighbourhood; the two forks drift apart
    // (`extra`, `missing`) far beyond the first generation, which is why the oracle
    // attributes through the synapses and not by time.
    assert_eq!(
        readings(&c),
        vec![
            (
                0x0001_C000,
                vec![
                    (0x0001_C000, 12_462, 555, 152),
                    (0x0001_C000, 14_075, 489, 122)
                ],
                Attribution {
                    kicks: 8,
                    ancestors: 18,
                    descendants: 7,
                    advanced: 0,
                    extra: 36,
                    missing: 40
                },
                Some(25_486),
                Some(25_486),
                [Some(12_044), Some(8_048), Some(1_055), Some(3_577), Some(0)],
                vec![Some(12_462), Some(14_075)],
                vec![Some(17_948), Some(16_350)]
            ),
            (
                0x0002_0000,
                vec![
                    (0x0002_0000, 43_210, 3_156, 1_754),
                    (0x0002_0000, 0, 2_563, 1_220)
                ],
                Attribution {
                    kicks: 8,
                    ancestors: 20,
                    descendants: 6,
                    advanced: 2,
                    extra: 127,
                    missing: 136
                },
                Some(19_660),
                Some(13_107),
                [
                    Some(31_814),
                    Some(25_483),
                    Some(19_245),
                    Some(10_115),
                    Some(11_099)
                ],
                vec![Some(43_210), Some(0)],
                vec![Some(36_422), Some(31_195)]
            ),
            (
                0x0002_4000,
                vec![
                    (0x0002_4000, 35_054, 6_845, 4_546),
                    (0x0002_4000, 2_292, 6_223, 3_839)
                ],
                Attribution {
                    kicks: 8,
                    ancestors: 36,
                    descendants: 21,
                    advanced: 10,
                    extra: 504,
                    missing: 551
                },
                Some(38_229),
                Some(20_025),
                [
                    Some(30_074),
                    Some(19_430),
                    Some(4_164),
                    Some(7_824),
                    Some(3_152)
                ],
                vec![Some(35_054), Some(2_292)],
                vec![Some(43_524), Some(40_429)]
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
            (73_728, 0, 0, 0),
            (82_944, 0, 0, 0),
            (93_312, 0, 0, 0),
            (104_976, 0, 15, 1),
            (114_723, 16_852, 127, 8),
            (129_063, 0, 537, 161),
            (142_565, 10_690, 2_606, 1_362),
            (160_386, 0, 5_253, 3_131),
            (140_338, SIGMA_MAX_Q16, 9_940, 7_240),
            (157_880, 0, 4_643, 2_629),
        ]
    );
    // The random network under the same sweep (ADR-0047): quieter at 1.75 (468 and 407
    // spikes), at 2.0 and 2.25 near the lattice's counts; the estimate of one window swings
    // as on the lattice (0.232 then 0.040; 0.384 then 0.000), the cross-check holds on every
    // window, and the fine lag-one slope reads 0.09, 0.68 and 0.59 while the gross ratio
    // reads 0.19, 0.10 and 0.46.
    let r = criticality(
        &random_prior(256),
        &GAINS,
        2,
        8,
        MODULATION_ONE_Q16 as u32,
        10,
    );
    dump(
        "the_estimate_the_causal_ratios_and_the_loop_at_256_units_exhaustive",
        &r,
    );
    assert_eq!(
        readings(&r),
        vec![
            (
                0x0001_C000,
                vec![(0x0001_C000, 2_822, 468, 75), (0x0001_C000, 8_523, 407, 40)],
                Attribution {
                    kicks: 8,
                    ancestors: 16,
                    descendants: 3,
                    advanced: 1,
                    extra: 23,
                    missing: 21
                },
                Some(12_288),
                Some(8_192),
                [Some(5_624), Some(4_231), Some(0), Some(0), Some(0)],
                vec![Some(2_822), Some(8_523)],
                vec![Some(10_502), Some(6_440)]
            ),
            (
                0x0002_0000,
                vec![
                    (0x0002_0000, 15_190, 2_738, 1_308),
                    (0x0002_0000, 2_620, 2_256, 928)
                ],
                Attribution {
                    kicks: 8,
                    ancestors: 21,
                    descendants: 2,
                    advanced: 1,
                    extra: 77,
                    missing: 79
                },
                Some(6_241),
                Some(3_121),
                [Some(44_309), Some(34_199), Some(12_333), Some(0), Some(0)],
                vec![Some(15_190), Some(2_620)],
                vec![Some(31_307), Some(26_958)]
            ),
            (
                0x0002_4000,
                vec![
                    (0x0002_4000, 25_166, 6_542, 4_396),
                    (0x0002_4000, 0, 5_884, 3_687)
                ],
                Attribution {
                    kicks: 8,
                    ancestors: 28,
                    descendants: 13,
                    advanced: 10,
                    extra: 818,
                    missing: 793
                },
                Some(30_427),
                Some(7_022),
                [
                    Some(38_894),
                    Some(31_727),
                    Some(16_376),
                    Some(6_015),
                    Some(4_456)
                ],
                vec![Some(25_166), Some(0)],
                vec![Some(44_037), Some(41_065)]
            ),
        ]
    );
    // The loop on the random network: the same climb, the ceiling at the ninth window
    // (10 878 spikes over 8 192) and the gain down.
    assert_eq!(
        r.closed,
        vec![
            (73_728, 0, 0, 0),
            (82_944, 0, 0, 0),
            (93_312, 0, 0, 0),
            (104_976, 0, 14, 0),
            (116_871, 6_128, 123, 3),
            (130_073, 6_309, 567, 96),
            (146_332, 0, 2_339, 957),
            (164_624, 0, 5_857, 3_744),
            (144_046, SIGMA_MAX_Q16, 10_878, 8_680),
            (162_052, 0, 5_168, 2_995),
        ]
    );
}

#[test]
fn a_night_consolidates_the_synapses_among_a_tagged_pattern_and_a_cue_completes_the_local_one_at_256_units()
 {
    let n = night(256);
    // Three windows of slow-wave sleep, two of REM, two more of slow-wave, awake: 376
    // replays of the two episodes in turn, 128 depotentiations, sixty-four to each, so the
    // tags of 200 end at 136 and neither episode is spent.
    assert_eq!(n.stages, vec![1, 1, 1, 2, 2, 1, 1, 0]);
    assert_eq!((n.replays, n.depotentiations), (376, 128));
    assert_eq!(n.tags, (136, 136));
    // The cluster of twelve neighbours holds 147 synapses among itself, the random pattern
    // five; every one of them ends at the rail (32 767) after the night's replays.
    assert_eq!(n.cluster, ((147, 1_271_353), (147, 147 * 32_767)));
    assert_eq!(n.random, ((5, 40_212), (5, 5 * 32_767)));
    // A cue of six units on the image before the night fires no other unit (the cued six
    // fire, 18 spikes); on the image after it fires all six of the cluster's rest and no
    // other unit (39 spikes); the random pattern's rest never fires: completion needs the
    // synapses among the pattern, which a local pattern has and a random one at this
    // density does not.
    assert_eq!(
        n.readouts,
        [
            (vec![], 0, 18),
            (vec![7, 8, 10, 11, 12, 13], 0, 39),
            (vec![], 0, 18),
            (vec![], 0, 18)
        ]
    );
}

/// The gate's form of ADR-0048's measurement at 256 units.
#[test]
fn an_experience_and_a_rewarded_invention_are_tagged_from_the_train_and_the_night_completes_the_invention_s_pattern_at_256_units()
 {
    let c = capture_night(256);
    // 437 spikes to the reward's tick. The invention's pattern is the densest basal time
    // constant of the ripple before the reward (a span from tick 8 088 holding 62 spikes,
    // the experience at 8 192 inside it): eleven of the twelve cued neighbours (ten firing
    // three times in the span, unit 13 twice) and one unit the drive fired twice in the same
    // span, ranked by their spikes; the network's own pattern is the densest span of the two
    // bins before the experience, a cascade of 64 spikes around the ring's wrap, twelve
    // units of which two (149, 254) are inhibitory and one, unit 0, is in both patterns. The
    // store of two facts commits nothing and tags nothing.
    assert_eq!(c.spikes, 447);
    assert_eq!(
        c.predicates,
        (INVENTED_BASE, INVENTED_BASE + 1),
        "the search's two commits; the episode is bound to the first"
    );
    assert_eq!(c.tagged.0, 0);
    assert_eq!(c.pattern, vec![7, 0, 1, 8, 10, 12, 2, 5, 6, 11, 13, 155]);
    assert_eq!(
        c.tagged.1,
        Burst {
            from: 8090,
            spikes: 58
        }
    );
    assert_eq!(c.clauses, 26, "two inputs became three outputs, twice");
    assert_eq!(c.symbols, (Some(INVENTED_BASE), None));
    assert_eq!(
        c.burst,
        Burst {
            from: 3_218,
            spikes: 64
        }
    );
    assert_eq!(
        c.background,
        vec![241, 18, 151, 149, 247, 245, 0, 253, 254, 255, 35, 110]
    );
    assert_eq!(c.signal, 3 * ONE / 2);
    assert_eq!(c.control, (0, true));
    // The night as ADR-0044's: the same stages, replays and tags.
    assert_eq!(c.stages, vec![1, 1, 1, 2, 2, 1, 1, 0]);
    assert_eq!((c.replays, c.depotentiations), (376, 128));
    assert_eq!(c.tags, (136, 136));
    // The invention's pattern holds 127 synapses among itself; after the night 107 are at
    // the rail, the eleven from unit 0 are at zero (unit 0 fires in the other episode's
    // replays too, the pair rule at those spikes finds its targets' last spikes a ripple
    // earlier, and the half-range holds the depression at zero; ADR-0049) and the nine onto
    // unit 0 end near their prior weights (8 300 to 14 091), since unit 0's own spike at each
    // replay comes a ripple after theirs; the network's own pattern holds 45, the five from
    // its inhibitory unit 254 at the negative rail after the night. A cue of six of the
    // invention's pattern fires five of the other six after the night (units 2, 5, 6, 11 and
    // 13; not 163, which no synapse of the cued six reaches; 30 spikes) and none before; the
    // network's own pattern never completes.
    assert_eq!(c.invention, ((126, 1_099_720), (126, 3_592_468)));
    assert_eq!(c.own, ((45, 173_699), (45, 874_182)));
    assert_eq!(
        c.readouts,
        [
            (vec![], 0, 18),
            (vec![2, 5, 6, 11, 13], 0, 30),
            (vec![], 0, 18),
            (vec![], 0, 18)
        ]
    );
    // The night's onset compacts the engine's arena (ADR-0056): the two commits' thirty
    // nodes, as in `tests/store.rs`.
    assert_eq!(c.reclaimed, 30);
    let _ = PATTERN_MAX;
}

/// The weekly job's form of the measurement (`cargo test -p cortex-runtime --release -- --ignored
/// exhaustive`): 1 024 units, sixteen kicks per gain, twelve windows of closed loop, on both
/// priors. The same picture as at 256: the estimate of one window swings by more than itself
/// at one gain (0.560 then 0.032 at 2.0), the gross causal ratio rises to 1.000 at 2.25
/// while the net stays near 0.4, and the loop crosses the ceiling twice in twelve windows.
#[test]
#[ignore]
fn the_estimate_the_causal_ratios_and_the_loop_at_1024_units_exhaustive() {
    let c = criticality(&prior(1024), &GAINS, 2, 16, MODULATION_ONE_Q16 as u32, 12);
    dump(
        "the_estimate_the_causal_ratios_and_the_loop_at_1024_units_exhaustive",
        &c,
    );
    assert_eq!(
        readings(&c),
        vec![
            (
                0x0001_C000,
                vec![
                    (0x0001_C000, 21_657, 2_406, 825),
                    (0x0001_C000, 17_656, 2_297, 722)
                ],
                Attribution {
                    kicks: 16,
                    ancestors: 34,
                    descendants: 18,
                    advanced: 3,
                    extra: 92,
                    missing: 102
                },
                Some(34_695),
                Some(28_913),
                [
                    Some(22_736),
                    Some(13_751),
                    Some(5_621),
                    Some(5_480),
                    Some(5_506)
                ],
                vec![Some(21_657), Some(17_656)],
                vec![Some(22_471), Some(20_599)]
            ),
            (
                0x0002_0000,
                vec![
                    (0x0002_0000, 36_723, 12_757, 7_337),
                    (0x0002_0000, 2_124, 11_328, 5_707)
                ],
                Attribution {
                    kicks: 16,
                    ancestors: 47,
                    descendants: 30,
                    advanced: 14,
                    extra: 892,
                    missing: 931
                },
                Some(41_831),
                Some(22_310),
                [
                    Some(41_945),
                    Some(35_187),
                    Some(28_100),
                    Some(19_920),
                    Some(14_295)
                ],
                vec![Some(36_723), Some(2_124)],
                vec![Some(37_692), Some(33_016)]
            ),
            (
                0x0002_4000,
                vec![
                    (0x0002_4000, 48_229, 27_945, 18_843),
                    (0x0002_4000, 0, 25_754, 16_439)
                ],
                Attribution {
                    kicks: 16,
                    ancestors: 61,
                    descendants: 61,
                    advanced: 32,
                    extra: 1_777,
                    missing: 1_795
                },
                Some(0x0001_0000),
                Some(31_156),
                [
                    Some(45_873),
                    Some(38_632),
                    Some(29_603),
                    Some(24_696),
                    Some(19_592)
                ],
                vec![Some(48_229), Some(0)],
                vec![Some(44_190), Some(41_832)]
            ),
        ]
    );
    assert_eq!(
        c.closed,
        vec![
            (73_728, 0, 0, 0),
            (82_944, 0, 0, 0),
            (88_896, 27_913, 4, 0),
            (100_008, 0, 22, 0),
            (111_804, 3_693, 224, 12),
            (124_558, 5_728, 1_504, 419),
            (132_976, 30_101, 7_656, 3_863),
            (146_183, 13_467, 13_086, 7_174),
            (160_251, 15_083, 25_060, 15_831),
            (140_220, SIGMA_MAX_Q16, 39_759, 29_100),
            (154_121, 13_559, 18_361, 10_404),
            (134_856, SIGMA_MAX_Q16, 33_001, 22_888),
        ]
    );
    // The random network at 1 024 units: the fine lag-one slope rises with the gain (0.12,
    // 0.55, 0.86) as the gross ratio does (0.21, 0.58, 0.71), unlike at 256; the coarse
    // estimate still swings (0.633 then 0.000 at 2.0); the loop crosses the ceiling twice.
    let r = criticality(
        &random_prior(1024),
        &GAINS,
        2,
        16,
        MODULATION_ONE_Q16 as u32,
        12,
    );
    dump(
        "the_estimate_the_causal_ratios_and_the_loop_at_1024_units_exhaustive",
        &r,
    );
    assert_eq!(
        readings(&r),
        vec![
            (
                0x0001_C000,
                vec![
                    (0x0001_C000, 750, 1_838, 296),
                    (0x0001_C000, 7_396, 1_800, 271)
                ],
                Attribution {
                    kicks: 16,
                    ancestors: 34,
                    descendants: 7,
                    advanced: 3,
                    extra: 92,
                    missing: 103
                },
                Some(13_492),
                Some(7_710),
                [Some(8_112), Some(6_167), Some(0), Some(0), Some(0)],
                vec![Some(750), Some(7_396)],
                vec![Some(10_554), Some(9_866)]
            ),
            (
                0x0002_0000,
                vec![
                    (0x0002_0000, 41_482, 10_534, 4_773),
                    (0x0002_0000, 0, 9_794, 4_243)
                ],
                Attribution {
                    kicks: 16,
                    ancestors: 45,
                    descendants: 26,
                    advanced: 16,
                    extra: 2_334,
                    missing: 2_381
                },
                Some(37_865),
                Some(14_564),
                [
                    Some(35_944),
                    Some(28_811),
                    Some(20_886),
                    Some(9_484),
                    Some(12_187)
                ],
                vec![Some(41_482), Some(0)],
                vec![Some(29_694), Some(28_391)]
            ),
            (
                0x0002_4000,
                vec![
                    (0x0002_4000, 9_983, 26_290, 17_700),
                    (0x0002_4000, 0, 24_075, 15_500)
                ],
                Attribution {
                    kicks: 16,
                    ancestors: 62,
                    descendants: 44,
                    advanced: 32,
                    extra: 5_857,
                    missing: 5_848
                },
                Some(46_509),
                Some(12_684),
                [Some(56_263), Some(44_730), Some(17_666), Some(0), Some(386)],
                vec![Some(9_983), Some(0)],
                vec![Some(44_122), Some(42_193)]
            ),
        ]
    );
    assert_eq!(
        r.closed,
        vec![
            (73_728, 0, 0, 0),
            (82_944, 0, 0, 0),
            (88_896, 27_913, 4, 0),
            (100_008, 0, 22, 0),
            (112_509, 0, 216, 3),
            (125_896, 3_151, 1_295, 140),
            (140_634, 4_162, 6_752, 2_407),
            (153_936, 15_943, 18_033, 10_288),
            (172_903, 932, 31_394, 22_199),
            (151_290, SIGMA_MAX_Q16, 53_441, 46_080),
            (170_201, 0, 27_800, 18_761),
            (148_926, SIGMA_MAX_Q16, 50_081, 42_553),
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
    assert_eq!(n.tags, (136, 136));
    assert_eq!(n.cluster, ((143, 1_261_514), (143, 143 * 32_767)));
    assert_eq!(n.random, ((2, 18_776), (2, 2 * 32_767)));
    assert_eq!(
        n.readouts,
        [
            (vec![], 0, 18),
            (vec![7, 8, 10, 11, 12, 13], 0, 39),
            (vec![], 0, 18),
            (vec![], 0, 18)
        ]
    );
}

/// The weekly job's form of ADR-0051's measurement: 4 096 units, thirty-two kicks per gain,
/// twelve windows of closed loop, on both priors, with the decision rule of brief 024 applied
/// in the ADR. The coarse estimate reads zero in seven windows of twelve and never the gross
/// ratio (0.000 and 0.000 where the oracle attributes 0.515); the fine lag-one slope is within
/// 0.15 of the gross ratio in five rows of six and 0.28 above it on the lattice at 2.25, where
/// the window holds 112 272 spikes against a ceiling of 131 072; the loop crosses the ceiling
/// once on the lattice and twice on the random network within twelve windows.
#[test]
#[ignore]
fn the_estimate_the_causal_ratios_and_the_loop_at_4096_units_exhaustive() {
    let c = criticality(&prior(4096), &GAINS, 2, 32, MODULATION_ONE_Q16 as u32, 12);
    dump(
        "the_estimate_the_causal_ratios_and_the_loop_at_4096_units_exhaustive",
        &c,
    );
    assert_eq!(
        readings(&c),
        vec![
            (
                0x0001_C000,
                vec![
                    (0x0001_C000, 0, 9_851, 3_275),
                    (0x0001_C000, 0, 9_207, 2_727)
                ],
                Attribution {
                    kicks: 32,
                    ancestors: 66,
                    descendants: 34,
                    advanced: 4,
                    extra: 172,
                    missing: 140
                },
                Some(33_760),
                Some(29_789),
                [
                    Some(24_112),
                    Some(13_336),
                    Some(1_790),
                    Some(6_237),
                    Some(0)
                ],
                vec![Some(0), Some(0)],
                vec![Some(21_787), Some(19_410)]
            ),
            (
                0x0002_0000,
                vec![
                    (0x0002_0000, 48_797, 51_087, 29_328),
                    (0x0002_0000, 0, 45_854, 23_527)
                ],
                Attribution {
                    kicks: 32,
                    ancestors: 97,
                    descendants: 86,
                    advanced: 32,
                    extra: 2_341,
                    missing: 2_427
                },
                Some(58_104),
                Some(36_484),
                [
                    Some(55_777),
                    Some(51_033),
                    Some(42_201),
                    Some(33_225),
                    Some(18_970)
                ],
                vec![Some(48_797), Some(0)],
                vec![Some(37_622), Some(33_625)]
            ),
            (
                0x0002_4000,
                vec![
                    (0x0002_4000, 46_156, 112_272, 76_170),
                    (0x0002_4000, 0, 102_769, 64_886)
                ],
                Attribution {
                    kicks: 30,
                    ancestors: 103,
                    descendants: 63,
                    advanced: 35,
                    extra: 2_407,
                    missing: 2_492
                },
                Some(40_085),
                Some(17_816),
                [
                    Some(58_158),
                    Some(51_550),
                    Some(37_077),
                    Some(32_847),
                    Some(22_644)
                ],
                vec![Some(46_156), Some(0)],
                vec![Some(44_462), Some(41_377)]
            ),
        ]
    );
    assert_eq!(
        c.closed,
        vec![
            (73_728, 0, 0, 0),
            (82_944, 0, 0, 0),
            (93_312, 0, 19, 0),
            (104_976, 0, 215, 9),
            (118_098, 0, 1_958, 220),
            (124_645, 36_472, 15_727, 6_565),
            (132_941, 30_642, 29_421, 13_991),
            (141_662, 31_141, 52_639, 28_605),
            (149_887, 35_095, 81_839, 49_023),
            (161_338, 25_483, 114_046, 74_803),
            (141_171, SIGMA_MAX_Q16, 163_418, 120_633),
            (152_616, 23_033, 77_632, 44_770),
        ]
    );
    let c = criticality(
        &random_prior(4096),
        &GAINS,
        2,
        32,
        MODULATION_ONE_Q16 as u32,
        12,
    );
    dump(
        "the_estimate_the_causal_ratios_and_the_loop_at_4096_units_exhaustive",
        &c,
    );
    assert_eq!(
        readings(&c),
        vec![
            (
                0x0001_C000,
                vec![
                    (0x0001_C000, 0, 7_562, 1_130),
                    (0x0001_C000, 5_643, 7_516, 1_113)
                ],
                Attribution {
                    kicks: 32,
                    ancestors: 62,
                    descendants: 11,
                    advanced: 1,
                    extra: 115,
                    missing: 137
                },
                Some(11_627),
                Some(10_570),
                [Some(7_731), Some(6_453), Some(0), Some(126), Some(1_254)],
                vec![Some(0), Some(5_643)],
                vec![Some(9_793), Some(9_704)]
            ),
            (
                0x0002_0000,
                vec![
                    (0x0002_0000, 47_994, 42_321, 19_780),
                    (0x0002_0000, 0, 38_544, 16_001)
                ],
                Attribution {
                    kicks: 31,
                    ancestors: 77,
                    descendants: 52,
                    advanced: 22,
                    extra: 12_024,
                    missing: 12_135
                },
                Some(44_258),
                Some(25_534),
                [
                    Some(51_644),
                    Some(46_974),
                    Some(37_003),
                    Some(20_295),
                    Some(6_267)
                ],
                vec![Some(47_994), Some(0)],
                vec![Some(30_630), Some(27_206)]
            ),
            (
                0x0002_4000,
                vec![
                    (0x0002_4000, 7_848, 105_042, 71_485),
                    (0x0002_4000, 0, 95_996, 61_097)
                ],
                Attribution {
                    kicks: 29,
                    ancestors: 104,
                    descendants: 89,
                    advanced: 51,
                    extra: 25_702,
                    missing: 25_336
                },
                Some(56_083),
                Some(23_946),
                [
                    Some(59_393),
                    Some(48_738),
                    Some(21_682),
                    Some(0),
                    Some(1_413)
                ],
                vec![Some(7_848), Some(0)],
                vec![Some(44_599), Some(41_710)]
            ),
        ]
    );
    assert_eq!(
        c.closed,
        vec![
            (73_728, 0, 0, 0),
            (82_944, 0, 0, 0),
            (93_312, 0, 19, 0),
            (104_976, 0, 203, 0),
            (116_395, 8_503, 1_828, 99),
            (130_944, 0, 9_280, 1_607),
            (136_515, 43_230, 40_153, 17_889),
            (151_532, 7_863, 56_072, 28_098),
            (164_161, 21_839, 114_896, 79_591),
            (143_641, SIGMA_MAX_Q16, 171_658, 138_358),
            (158_287, 12_080, 79_973, 46_486),
            (138_501, SIGMA_MAX_Q16, 144_274, 109_021),
        ]
    );
}

/// The weekly job's form of ADR-0048's measurement at 1 024 units.
#[test]
#[ignore]
fn an_experience_and_a_rewarded_invention_are_tagged_from_the_train_at_1024_units_exhaustive() {
    let c = capture_night(1024);
    // 1 370 spikes to the reward's tick. The invention's pattern is all twelve cued
    // neighbours in rank order; the network's own is a span of 153 spikes at tick 4 238,
    // twelve units of which five (149, 494, 499, 484, 1 014) are inhibitory.
    assert_eq!(c.spikes, 1340);
    assert_eq!(c.predicates, (INVENTED_BASE, INVENTED_BASE + 1));
    assert_eq!(c.tagged.0, 0);
    assert_eq!(c.pattern, vec![2, 0, 3, 7, 10, 13, 1, 5, 11, 12, 6, 8]);
    assert_eq!(
        c.tagged.1,
        Burst {
            from: 8_172,
            spikes: 140
        }
    );
    assert_eq!(c.clauses, 26);
    assert_eq!(c.symbols, (Some(INVENTED_BASE), None));
    assert_eq!(
        c.burst,
        Burst {
            from: 4238,
            spikes: 151
        }
    );
    assert_eq!(
        c.background,
        vec![149, 494, 499, 465, 488, 463, 486, 435, 462, 1014, 484, 960]
    );
    assert_eq!(c.signal, 3 * ONE / 2);
    assert_eq!(c.control, (0, true));
    assert_eq!(c.stages, vec![1, 1, 1, 2, 2, 1, 1, 0]);
    assert_eq!((c.replays, c.depotentiations), (376, 128));
    assert_eq!(c.tags, (136, 136));
    // The cluster's 143 synapses (ADR-0044's count at this size) every one at the rail after
    // the night; the network's own twenty, summing to a negative weight before the night
    // (five of its units are inhibitory), fourteen at the positive rail after it and six, the
    // ones from its inhibitory units 494, 499 and 484, at the negative rail: the symmetric
    // rule potentiates inhibition within a replayed pattern and the half-range keeps its
    // sign (ADR-0049; finding F-36 resolved). A cue of six in rank order fires five of the
    // invention's other six (a cue in index order fired all six in ADR-0044's night); the
    // network's own pattern never completes.
    assert_eq!(c.invention, ((143, 1_261_514), (143, 143 * 32_767)));
    assert_eq!(c.own, ((20, -78_714), (20, 262_136)));
    assert_eq!(
        c.readouts,
        [
            (vec![], 0, 17),
            (vec![1, 5, 6, 8, 11, 12], 0, 38),
            (vec![], 0, 18),
            (vec![], 0, 18)
        ]
    );
    // The night's onset compacts the engine's arena (ADR-0056): the two commits' thirty
    // nodes, as in `tests/store.rs`.
    assert_eq!(c.reclaimed, 30);
}

/// The sums over the arena before any window, for the two priors of the days at both sizes
/// (ADR-0053, ADR-0055, ADR-0057): what a day's drift is read against.
#[test]
fn the_priors_sums_before_any_window() {
    let mut sums = Vec::new();
    for (name, p) in [
        ("lattice 256", prior(256)),
        ("below the rail 256", prior_below_rail(256)),
        ("lattice 1024", prior(1024)),
        ("below the rail 1024", prior_below_rail(1024)),
    ] {
        let cfg = config(p.units, 1, 0, MODULATION_ONE_Q16);
        let exec = at_gain(&p, cfg, 0x0002_0000);
        let (inhibitory, excitatory) = weights_by_polarity(&exec);
        eprintln!("DUMP prior {name}: inhibitory {inhibitory} excitatory {excitatory}");
        sums.push((inhibitory, excitatory));
    }
    // The lattice's inhibition is at the rail (1 632 and 6 528 synapses times 32 767); the
    // below-rail prior's is 0.55 of it; the excitatory sums are the same generator's.
    assert_eq!(
        sums,
        vec![
            (53_475_744, 58_968_247),
            (29_320_198, 58_968_247),
            (213_902_976, 235_822_619),
            (117_719_176, 235_822_619),
        ]
    );
}
