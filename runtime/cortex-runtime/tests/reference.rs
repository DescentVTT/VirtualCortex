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

/// One window as `windows` reports it: `(gain, estimate, spikes)`.
type Window = (u32, u32, u64);

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

/// The fixed part of a measurement with the ratios beside it, for the assertions.
type Reading = (
    u32,
    Vec<Window>,
    Attribution,
    Option<u32>,
    Option<i32>,
    [Option<u32>; 5],
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
            )
        })
        .collect()
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
            Some(17_873),
            [
                Some(37_281),
                Some(30_645),
                Some(22_151),
                Some(12_893),
                Some(15_540)
            ],
            vec![Some(43_210)]
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
}

/// The gate's form of ADR-0047's measurement: the random prior at 256 units and a gain of
/// 2.0, two windows and four kicks, no loop.
#[test]
fn the_estimate_the_causal_ratio_and_the_fine_slopes_on_the_random_prior_at_256_units() {
    let c = criticality(&random_prior(256), &GAINS[1..2], 2, 4, 0x0002_0000, 0);
    // At a gain of 2.0 the random network spikes 2 738 then 2 256 times per window, its
    // estimate 0.232 then 0.040 (the train's bins give the same); four kicks are twelve
    // ancestor spikes and one descendant, itself an advanced spike: a gross ratio of 0.083
    // and a net one of 0; the fine slopes fall from 0.676 at lag one to 0 at lag eight.
    assert_eq!(
        readings(&c),
        vec![(
            0x0002_0000,
            vec![(0x0002_0000, 15_190, 2_738), (0x0002_0000, 2_620, 2_256)],
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
            vec![Some(15_190), Some(2_620)]
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
                Some(25_486),
                [Some(12_044), Some(8_048), Some(1_055), Some(3_577), Some(0)],
                vec![Some(12_462), Some(14_075)]
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
                Some(13_107),
                [
                    Some(31_814),
                    Some(25_483),
                    Some(19_245),
                    Some(10_115),
                    Some(11_099)
                ],
                vec![Some(43_210), Some(0)]
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
                Some(20_025),
                [
                    Some(30_074),
                    Some(19_430),
                    Some(4_164),
                    Some(7_824),
                    Some(3_152)
                ],
                vec![Some(35_054), Some(2_292)]
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
    assert_eq!(
        readings(&r),
        vec![
            (
                0x0001_C000,
                vec![(0x0001_C000, 2_822, 468), (0x0001_C000, 8_523, 407)],
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
                vec![Some(2_822), Some(8_523)]
            ),
            (
                0x0002_0000,
                vec![(0x0002_0000, 15_190, 2_738), (0x0002_0000, 2_620, 2_256)],
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
                vec![Some(15_190), Some(2_620)]
            ),
            (
                0x0002_4000,
                vec![(0x0002_4000, 25_166, 6_542), (0x0002_4000, 0, 5_884)],
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
                vec![Some(25_166), Some(0)]
            ),
        ]
    );
    // The loop on the random network: the same climb, the ceiling at the ninth window
    // (10 878 spikes over 8 192) and the gain down.
    assert_eq!(
        r.closed,
        vec![
            (73_728, 0, 0),
            (82_944, 0, 0),
            (93_312, 0, 0),
            (104_976, 0, 14),
            (116_871, 6_128, 123),
            (130_073, 6_309, 567),
            (146_332, 0, 2_339),
            (164_624, 0, 5_857),
            (144_046, SIGMA_MAX_Q16, 10_878),
            (162_052, 0, 5_168),
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
    assert_eq!(
        readings(&c),
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
                Some(28_913),
                [
                    Some(22_736),
                    Some(13_751),
                    Some(5_621),
                    Some(5_480),
                    Some(5_506)
                ],
                vec![Some(21_657), Some(17_656)]
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
                Some(22_310),
                [
                    Some(41_945),
                    Some(35_187),
                    Some(28_100),
                    Some(19_920),
                    Some(14_295)
                ],
                vec![Some(36_723), Some(2_124)]
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
                Some(31_156),
                [
                    Some(45_873),
                    Some(38_632),
                    Some(29_603),
                    Some(24_696),
                    Some(19_592)
                ],
                vec![Some(48_229), Some(0)]
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
    assert_eq!(
        readings(&r),
        vec![
            (
                0x0001_C000,
                vec![(0x0001_C000, 750, 1_838), (0x0001_C000, 7_396, 1_800)],
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
                vec![Some(750), Some(7_396)]
            ),
            (
                0x0002_0000,
                vec![(0x0002_0000, 41_482, 10_534), (0x0002_0000, 0, 9_794)],
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
                vec![Some(41_482), Some(0)]
            ),
            (
                0x0002_4000,
                vec![(0x0002_4000, 9_983, 26_290), (0x0002_4000, 0, 24_075)],
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
                vec![Some(9_983), Some(0)]
            ),
        ]
    );
    assert_eq!(
        r.closed,
        vec![
            (73_728, 0, 0),
            (82_944, 0, 0),
            (88_896, 27_913, 4),
            (100_008, 0, 22),
            (112_509, 0, 216),
            (125_896, 3_151, 1_295),
            (140_634, 4_162, 6_752),
            (153_936, 15_943, 18_033),
            (172_903, 932, 31_394),
            (151_290, SIGMA_MAX_Q16, 53_441),
            (170_201, 0, 27_800),
            (148_926, SIGMA_MAX_Q16, 50_081),
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
    assert_eq!(
        readings(&c),
        vec![
            (
                0x0001_C000,
                vec![(0x0001_C000, 0, 9_851), (0x0001_C000, 0, 9_207)],
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
                vec![Some(0), Some(0)]
            ),
            (
                0x0002_0000,
                vec![(0x0002_0000, 48_797, 51_087), (0x0002_0000, 0, 45_854)],
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
                vec![Some(48_797), Some(0)]
            ),
            (
                0x0002_4000,
                vec![(0x0002_4000, 46_156, 112_272), (0x0002_4000, 0, 102_769)],
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
                vec![Some(46_156), Some(0)]
            ),
        ]
    );
    assert_eq!(
        c.closed,
        vec![
            (73_728, 0, 0),
            (82_944, 0, 0),
            (93_312, 0, 19),
            (104_976, 0, 215),
            (118_098, 0, 1_958),
            (124_645, 36_472, 15_727),
            (132_941, 30_642, 29_421),
            (141_662, 31_141, 52_639),
            (149_887, 35_095, 81_839),
            (161_338, 25_483, 114_046),
            (141_171, SIGMA_MAX_Q16, 163_418),
            (152_616, 23_033, 77_632),
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
    assert_eq!(
        readings(&c),
        vec![
            (
                0x0001_C000,
                vec![(0x0001_C000, 0, 7_562), (0x0001_C000, 5_643, 7_516)],
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
                vec![Some(0), Some(5_643)]
            ),
            (
                0x0002_0000,
                vec![(0x0002_0000, 47_994, 42_321), (0x0002_0000, 0, 38_544)],
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
                vec![Some(47_994), Some(0)]
            ),
            (
                0x0002_4000,
                vec![(0x0002_4000, 7_848, 105_042), (0x0002_4000, 0, 95_996)],
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
                vec![Some(7_848), Some(0)]
            ),
        ]
    );
    assert_eq!(
        c.closed,
        vec![
            (73_728, 0, 0),
            (82_944, 0, 0),
            (93_312, 0, 19),
            (104_976, 0, 203),
            (116_395, 8_503, 1_828),
            (130_944, 0, 9_280),
            (136_515, 43_230, 40_153),
            (151_532, 7_863, 56_072),
            (164_161, 21_839, 114_896),
            (143_641, SIGMA_MAX_Q16, 171_658),
            (158_287, 12_080, 79_973),
            (138_501, SIGMA_MAX_Q16, 144_274),
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
