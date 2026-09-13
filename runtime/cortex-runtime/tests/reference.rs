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
/// window, and the replays so far.
type DayWindow = (u64, u32, u32, u8, u64, i64, i64, u64);

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
/// the start so that a night has something to replay; per window the reading above.
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
        let (gain, estimate, spikes, descendants) = windows(&mut exec, &drive, 1)[0];
        let (inhibitory, excitatory) = weights_by_polarity(&exec);
        out.push((
            spikes,
            gain,
            estimate,
            exec.sleep_stage(),
            descendants,
            inhibitory,
            excitatory,
            exec.replays(),
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
    let slow = day(&prior(256), 8, 0, 0, 20_000);
    let fast = day(&prior(256), 8, 0, 0, 5_000);
    eprintln!("DUMP day256 slow {slow:?}");
    eprintln!("DUMP day256 fast {fast:?}");
    assert_eq!(
        slow,
        vec![
            (3_039, 131_072, 39_504, 0, 1_625, 53_227_953, 56_684_837, 0),
            (2_464, 131_072, 0, 0, 1_103, 53_024_805, 55_219_648, 0),
            (2_619, 131_072, 6_109, 0, 1_199, 52_872_788, 53_552_474, 0),
            (2_507, 131_072, 3_075, 0, 1_102, 52_737_333, 52_050_975, 0),
            (2_452, 131_072, 9_902, 0, 1_029, 52_549_297, 50_693_718, 0),
            (2_434, 131_072, 15_312, 0, 1_040, 52_355_577, 49_372_795, 0),
            (2_370, 131_072, 2_087, 0, 979, 52_168_376, 48_144_023, 0),
            (2_299, 131_072, 11_186, 0, 896, 52_007_040, 46_951_567, 0),
        ]
    );
    assert_eq!(
        fast,
        vec![
            (3_075, 131_072, 36_887, 0, 1_631, 49_247_248, 56_625_123, 0),
            (2_485, 131_072, 0, 0, 1_109, 45_402_693, 55_138_166, 0),
            (2_666, 131_072, 7_804, 0, 1_224, 41_431_295, 53_460_792, 0),
            (2_610, 131_072, 0, 0, 1_178, 37_469_897, 51_828_909, 0),
            (2_566, 131_072, 2_393, 0, 1_102, 33_550_912, 50_334_185, 0),
            (2_619, 131_072, 4_519, 0, 1_163, 29_576_920, 48_766_442, 0),
            (2_563, 131_072, 8_357, 0, 1_080, 25_521_567, 47_324_922, 0),
            (2_566, 131_072, 11_655, 0, 1_077, 21_649_131, 45_843_315, 0),
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
                12_424,
                135_332,
                48_493,
                0,
                6_993,
                212_996_332,
                225_908_044,
                0
            ),
            (
                13_899,
                149_713,
                9_822,
                0,
                7_116,
                212_719_459,
                214_870_652,
                0
            ),
            (
                25_332,
                156_781,
                40_787,
                0,
                15_116,
                213_678_669,
                183_539_023,
                0
            ),
            (
                29_304,
                168_286,
                27_064,
                0,
                17_636,
                213_863_479,
                142_896_484,
                0
            ),
            (
                37_060,
                147_250,
                1_048_576,
                0,
                23_702,
                213_901_913,
                86_360_998,
                0
            ),
            (16_090, 165_656, 0, 0, 5_947, 213_830_948, 74_896_090, 0),
            (30_076, 186_363, 0, 0, 16_113, 213_891_174, 45_524_275, 0),
            (
                46_974,
                163_068,
                1_048_576,
                0,
                32_222,
                213_902_976,
                11_822_530,
                0
            ),
            (25_189, 183_452, 0, 0, 10_961, 213_891_639, 6_955_080, 0),
            (
                42_212,
                160_521,
                1_048_576,
                0,
                26_815,
                213_902_434,
                1_667_307,
                0
            ),
            (22_676, 178_955, 5_325, 0, 9_078, 213_889_700, 1_118_879, 0),
            (
                37_594,
                156_586,
                1_048_576,
                0,
                21_903,
                213_900_445,
                394_874,
                0
            ),
            (19_727, 176_159, 0, 0, 7_011, 213_840_137, 386_416, 0),
            (
                35_240,
                154_139,
                1_048_576,
                0,
                19_550,
                213_894_634,
                131_993,
                0
            ),
            (18_146, 173_406, 0, 0, 5_964, 213_815_663, 195_814, 0),
            (
                32_792,
                151_730,
                1_048_576,
                0,
                17_194,
                213_877_160,
                68_284,
                0
            ),
            (16_588, 167_191, 12_114, 0, 5_037, 213_715_041, 189_024, 0),
            (27_625, 184_725, 10_554, 0, 12_646, 213_841_262, 96_493, 0),
            (43_254, 161_634, 1_048_576, 0, 27_498, 213_899_078, 5_844, 0),
            (23_378, 179_458, 7_718, 0, 9_505, 213_872_015, 33_646, 0),
            (38_216, 157_026, 1_048_576, 0, 22_636, 213_881_930, 5_338, 0),
            (20_229, 175_315, 4_474, 0, 7_353, 213_813_087, 84_303, 0),
            (
                34_298,
                153_401,
                1_048_576,
                0,
                18_780,
                213_876_624,
                19_474,
                0
            ),
            (17_648, 171_565, 3_454, 0, 5_612, 213_720_505, 120_878, 0),
            (31_384, 192_414, 1_824, 0, 16_153, 213_874_402, 44_264, 0),
            (50_607, 168_362, 1_048_576, 0, 35_425, 213_902_976, 353, 0),
            (28_840, 189_407, 0, 0, 13_964, 213_901_669, 9_385, 0),
            (47_561, 165_731, 1_048_576, 0, 32_292, 213_902_976, 301, 0),
            (26_919, 186_139, 973, 0, 12_252, 213_896_957, 17_714, 0),
            (44_562, 162_872, 1_048_576, 0, 28_870, 213_897_396, 619, 0),
            (24_423, 174_247, 28_922, 0, 10_351, 213_888_752, 27_996, 0),
            (
                33_432,
                152_466,
                1_048_576,
                0,
                17_963,
                213_898_794,
                14_362,
                0
            ),
            (16_848, 166_018, 18_935, 0, 5_246, 213_757_125, 138_686, 0),
            (26_849, 186_770, 0, 0, 12_271, 213_804_969, 81_781, 0),
            (45_115, 163_424, 1_048_576, 0, 29_685, 213_902_701, 1_659, 0),
            (25_202, 180_156, 11_853, 0, 10_904, 213_887_249, 29_013, 0),
            (38_854, 157_637, 1_048_576, 0, 23_169, 213_902_769, 7_866, 0),
            (20_632, 177_342, 0, 0, 7_624, 213_838_689, 63_861, 0),
            (
                36_364,
                155_174,
                1_048_576,
                0,
                20_641,
                213_897_602,
                13_667,
                0
            ),
            (18_868, 173_579, 3_348, 0, 6_388, 213_807_008, 108_267, 0),
            (
                33_043,
                151_882,
                1_048_576,
                0,
                17_425,
                213_885_014,
                35_436,
                0
            ),
            (16_591, 170_867, 0, 0, 5_073, 213_755_079, 159_357, 0),
            (30_303, 188_312, 12_007, 0, 15_154, 213_867_751, 54_192, 0),
            (46_285, 164_773, 1_048_576, 0, 30_740, 213_902_824, 639, 0),
            (25_948, 183_318, 6_529, 0, 11_578, 213_897_013, 18_787, 0),
            (41_762, 160_403, 1_048_576, 0, 26_126, 213_901_305, 2_246, 0),
            (22_633, 176_647, 12_439, 0, 9_036, 213_882_327, 54_307, 0),
            (
                35_812,
                154_566,
                1_048_576,
                0,
                20_195,
                213_892_988,
                15_156,
                0
            ),
            (18_405, 173_887, 0, 0, 6_162, 213_798_882, 92_708, 0),
            (
                33_195,
                152_151,
                1_048_576,
                0,
                17_694,
                213_889_818,
                26_170,
                0
            ),
            (16_783, 170_021, 3_963, 0, 5_119, 213_754_351, 162_112, 0),
            (29_914, 189_172, 6_481, 0, 14_545, 213_837_923, 50_907, 0),
            (47_535, 165_526, 1_048_576, 0, 32_298, 213_902_976, 179, 0),
            (26_682, 186_217, 0, 0, 12_017, 213_891_629, 7_904, 0),
            (44_253, 162_940, 1_048_576, 0, 28_756, 213_900_833, 1_218, 0),
            (24_563, 177_644, 18_225, 0, 10_322, 213_893_378, 23_037, 0),
            (36_512, 155_439, 1_048_576, 0, 20_798, 213_900_157, 6_063, 0),
            (19_065, 170_068, 16_194, 0, 6_491, 213_826_548, 84_350, 0),
            (30_092, 191_327, 0, 0, 15_062, 213_868_701, 48_930, 0),
            (49_188, 167_411, 1_048_576, 0, 34_146, 213_902_976, 116, 0),
            (27_848, 188_021, 988, 0, 12_958, 213_899_750, 8_059, 0),
            (46_272, 164_518, 1_048_576, 0, 30_951, 213_901_632, 843, 0),
            (25_729, 185_083, 0, 0, 11_314, 213_890_202, 23_572, 0),
            (43_530, 161_948, 1_048_576, 0, 27_838, 213_901_620, 325, 0),
            (24_066, 179_612, 8_351, 0, 10_107, 213_881_202, 29_480, 0),
            (38_452, 157_161, 1_048_576, 1, 22_743, 213_902_850, 4_212, 0),
            (22_770, 176_806, 0, 1, 10_028, 213_860_390, 1_354_048, 64),
            (
                38_264,
                154_705,
                1_048_576,
                1,
                22_878,
                213_897_140,
                3_180_378,
                128
            ),
            (
                20_710,
                171_713,
                7_898,
                1,
                8_372,
                213_806_295,
                4_278_048,
                192
            ),
            (
                33_792,
                150_249,
                1_048_576,
                2,
                18_725,
                213_884_299,
                4_599_265,
                256
            ),
            (15_950, 169_030, 0, 2, 5_006, 213_699_891, 4_389_979, 256),
            (29_461, 190_159, 0, 1, 14_620, 213_813_326, 3_506_757, 256),
            (
                50_887,
                166_389,
                1_048_576,
                0,
                36_126,
                213_902_909,
                4_549_552,
                320
            ),
            (27_600, 187_188, 0, 0, 13_114, 213_899_920, 4_036_675, 320),
            (
                45_801,
                163_790,
                1_048_576,
                0,
                30_436,
                213_902_976,
                2_925_219,
                320
            ),
            (25_583, 184_264, 0, 0, 11_427, 213_890_274, 2_390_015, 320),
            (
                42_924,
                161_231,
                1_048_576,
                0,
                27_387,
                213_902_575,
                1_640_876,
                320
            ),
            (
                23_355,
                178_162,
                10_480,
                0,
                9_523,
                213_876_542,
                1_432_722,
                320
            ),
            (
                37_145,
                155_892,
                1_048_576,
                0,
                21_455,
                213_897_995,
                998_198,
                320
            ),
            (19_534, 172_971, 8_096, 0, 6_896, 213_832_916, 993_455, 320),
        ]
    );
    assert_eq!(
        fast,
        vec![
            (
                12_529,
                135_184,
                49_091,
                0,
                7_030,
                196_786_033,
                225_748_127,
                0
            ),
            (
                14_045,
                151_131,
                3_686,
                0,
                7_234,
                178_474_227,
                214_445_433,
                0
            ),
            (
                28_282,
                160_365,
                33_503,
                0,
                17_742,
                152_388_035,
                176_458_249,
                0
            ),
            (
                35_109,
                140_319,
                1_048_576,
                0,
                22_944,
                126_799_770,
                121_230_219,
                0
            ),
            (
                14_271,
                157_182,
                2_525,
                0,
                5_440,
                108_762_956,
                110_445_709,
                0
            ),
            (
                29_104, 170_105, 22_434, 0, 16_526, 83_608_550, 74_265_552, 0
            ),
            (
                41_206, 148_842, 1_048_576, 0, 27_518, 61_317_666, 27_329_604, 0
            ),
            (18_707, 167_447, 0, 0, 7_160, 41_440_228, 19_856_414, 0),
            (
                37_826, 146_516, 1_048_576, 0, 23_745, 21_713_856, 5_176_427, 0
            ),
            (17_329, 164_831, 0, 0, 6_296, 9_770_004, 3_588_018, 0),
            (36_118, 144_227, 1_048_576, 0, 22_176, 2_879_472, 678_098, 0),
            (15_497, 162_255, 0, 0, 5_091, 705_040, 565_074, 0),
            (33_177, 141_973, 1_048_576, 0, 19_587, 86_016, 93_505, 0),
            (13_812, 159_720, 0, 0, 4_198, 23_988, 201_810, 0),
            (30_381, 179_685, 0, 0, 16_694, 66, 36_850, 0),
            (54_142, 157_224, 1_048_576, 0, 41_616, 238_598, 0, 0),
            (27_635, 176_877, 0, 0, 14_317, 202, 4_271, 0),
            (50_864, 154_767, 1_048_576, 0, 37_909, 110_053, 383, 0),
            (25_233, 173_995, 402, 0, 12_195, 0, 6_991, 0),
            (47_138, 152_246, 1_048_576, 0, 33_782, 38_479, 0, 0),
            (22_783, 171_277, 0, 0, 10_251, 0, 19_937, 0),
            (44_018, 149_867, 1_048_576, 0, 30_576, 19_372, 861, 0),
            (20_447, 168_600, 0, 0, 8_527, 0, 36_702, 0),
            (40_668, 147_525, 1_048_576, 0, 27_040, 12_133, 1_290, 0),
            (18_514, 165_966, 0, 0, 7_176, 0, 54_890, 0),
            (37_718, 145_220, 1_048_576, 0, 23_753, 1_070, 2_327, 0),
            (16_312, 163_373, 0, 0, 5_686, 0, 84_419, 0),
            (34_530, 142_951, 1_048_576, 0, 20_592, 394, 12_775, 0),
            (14_749, 160_820, 0, 0, 4_822, 0, 122_702, 0),
            (31_760, 140_718, 1_048_576, 0, 17_975, 1_320, 23_954, 0),
            (12_877, 158_059, 931, 0, 3_683, 0, 193_879, 0),
            (28_516, 177_816, 0, 0, 14_960, 528, 54_963, 0),
            (51_936, 155_589, 1_048_576, 0, 39_074, 173_052, 0, 0),
            (26_039, 175_038, 0, 0, 12_945, 769, 7_004, 0),
            (48_626, 153_158, 1_048_576, 0, 35_564, 77_590, 0, 0),
            (23_939, 168_624, 12_593, 0, 11_196, 0, 14_422, 0),
            (40_731, 147_546, 1_048_576, 0, 26_913, 9_314, 1_528, 0),
            (18_397, 161_709, 15_204, 0, 7_167, 0, 49_555, 0),
            (32_614, 141_495, 1_048_576, 0, 18_714, 0, 12_912, 0),
            (13_502, 159_182, 0, 0, 3_930, 0, 152_771, 0),
            (30_061, 179_080, 0, 0, 16_418, 378, 41_094, 0),
            (53_580, 156_695, 1_048_576, 0, 40_938, 208_569, 0, 0),
            (27_057, 176_282, 0, 0, 13_784, 1_404, 3_512, 0),
            (49_920, 154_247, 1_048_576, 0, 36_882, 105_331, 10, 0),
            (24_643, 173_481, 157, 0, 11_861, 0, 8_797, 0),
            (46_543, 151_796, 1_048_576, 0, 33_127, 26_621, 194, 0),
            (22_289, 170_771, 0, 0, 9_942, 54, 22_321, 0),
            (43_579, 149_425, 1_048_576, 0, 30_083, 21_300, 157, 0),
            (20_048, 168_103, 0, 0, 8_221, 0, 34_331, 0),
            (40_119, 147_090, 1_048_576, 0, 26_187, 10_199, 2_129, 0),
            (18_044, 160_590, 17_419, 0, 6_748, 0, 60_763, 0),
            (31_526, 180_664, 0, 0, 17_746, 814, 20_515, 0),
            (55_673, 158_081, 1_048_576, 0, 43_293, 276_314, 0, 0),
            (28_705, 177_841, 0, 0, 15_265, 5_351, 2_201, 0),
            (51_861, 155_611, 1_048_576, 0, 39_187, 163_166, 0, 0),
            (26_242, 173_258, 6_076, 0, 13_074, 667, 5_768, 0),
            (46_281, 151_601, 1_048_576, 0, 32_794, 39_895, 9, 0),
            (22_152, 169_080, 5_085, 0, 9_629, 0, 21_281, 0),
            (41_222, 147_945, 1_048_576, 0, 27_630, 13_012, 1_188, 0),
            (18_706, 166_438, 0, 0, 7_301, 381, 44_291, 0),
            (37_932, 145_633, 1_048_576, 0, 24_049, 3_674, 7_306, 0),
            (16_812, 163_837, 0, 0, 6_126, 0, 85_974, 0),
            (35_092, 143_357, 1_048_576, 0, 21_224, 149, 13_054, 0),
            (14_996, 161_277, 0, 0, 4_926, 0, 112_100, 0),
            (32_262, 141_117, 1_048_576, 0, 18_494, 2_384, 24_708, 0),
            (13_043, 157_493, 4_693, 1, 3_838, 0, 167_546, 0),
            (30_514, 177_180, 0, 1, 17_244, 77_564, 1_595_506, 64),
            (
                53_591, 155_033, 1_048_576, 1, 41_067, 664_784, 3_659_455, 128
            ),
            (27_676, 173_397, 3_428, 1, 14_639, 662_628, 4_563_209, 192),
            (
                48_904, 151_722, 1_048_576, 2, 35_825, 1_128_610, 4_685_921, 256
            ),
            (22_616, 170_687, 0, 2, 10_306, 994_485, 4_356_832, 256),
            (
                43_564, 149_351, 1_048_576, 1, 30_182, 950_289, 3_446_254, 256
            ),
            (22_260, 168_020, 0, 0, 10_252, 1_003_592, 4_522_756, 320),
            (
                40_226, 147_018, 1_048_576, 0, 26_628, 955_878, 3_775_655, 320
            ),
            (18_145, 164_547, 3_026, 0, 7_061, 830_203, 3_389_487, 320),
            (
                36_350, 143_979, 1_048_576, 0, 22_569, 725_680, 2_583_512, 320
            ),
            (15_618, 160_799, 4_289, 0, 5_434, 563_594, 2_402_089, 320),
            (
                32_037, 140_699, 1_048_576, 0, 18_277, 414_653, 1_879_916, 320
            ),
            (12_941, 158_286, 0, 0, 3_818, 315_221, 1_914_562, 320),
            (29_058, 177_062, 3_346, 0, 15_650, 214_081, 1_510_424, 320),
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
    assert_eq!(n.cluster, ((147, 1_281_024), (147, 147 * 32_767)));
    assert_eq!(n.random, ((5, 40_203), (5, 5 * 32_767)));
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
    assert_eq!(c.spikes, 437);
    assert_eq!(
        c.predicates,
        (INVENTED_BASE, INVENTED_BASE + 1),
        "the search's two commits; the episode is bound to the first"
    );
    assert_eq!(c.tagged.0, 0);
    assert_eq!(c.pattern, vec![7, 0, 1, 8, 10, 12, 2, 5, 6, 11, 13, 163]);
    assert_eq!(
        c.tagged.1,
        Burst {
            from: 8_088,
            spikes: 62
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
    assert_eq!(c.invention, ((127, 1_117_808), (127, 3_605_986)));
    assert_eq!(c.own, ((45, 175_290), (45, 846_292)));
    assert_eq!(
        c.readouts,
        [
            (vec![], 0, 18),
            (vec![2, 5, 6, 11, 13], 0, 30),
            (vec![], 0, 18),
            (vec![], 0, 18)
        ]
    );
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
    assert_eq!(n.cluster, ((143, 1_268_100), (143, 143 * 32_767)));
    assert_eq!(n.random, ((2, 18_841), (2, 2 * 32_767)));
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
    assert_eq!(c.spikes, 1_370);
    assert_eq!(c.predicates, (INVENTED_BASE, INVENTED_BASE + 1));
    assert_eq!(c.tagged.0, 0);
    assert_eq!(c.pattern, vec![2, 0, 3, 10, 11, 13, 1, 5, 7, 12, 6, 8]);
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
            from: 4_238,
            spikes: 153
        }
    );
    assert_eq!(
        c.background,
        vec![149, 494, 499, 465, 488, 463, 435, 486, 462, 1_014, 484, 960]
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
    assert_eq!(c.invention, ((143, 1_268_100), (143, 143 * 32_767)));
    assert_eq!(c.own, ((20, -77_818), (20, 8 * 32_767)));
    assert_eq!(
        c.readouts,
        [
            (vec![], 0, 18),
            (vec![5, 6, 7, 8, 12], 0, 31),
            (vec![], 0, 18),
            (vec![], 0, 18)
        ]
    );
}
