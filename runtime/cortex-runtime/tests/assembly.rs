//! Brief 048 (ADR-0112) measures, and changes nothing, whether an assembly of the reference
//! network can hold a context: on ADR-0077's settled image with every weight frozen — the
//! excitatory baseline zero, the inhibitory baseline unset, the signed gate unset, no reward —
//! a grid of assemblies of 16, 32 and 64 excitatory units, each wired among its members as
//! the test's own network at four recurrent weights, each run through one protocol under
//! ADR-0044's drive: a lead-in of one epoch, then an unkicked, a kicked and a released epoch
//! of $2^{14}$ ticks, eight times over. Whether a cell holds, ignites, lets go and spills are
//! integer rules over its tables; the grid, the protocol, the kick, the release, the measures
//! and the arithmetic were written before any run.
//!
//! The assembly's synapses are blocks appended to the image's synapse section and chained
//! after each member's own (`grown`, `wire`): the prior, the image format and every rule of
//! the engine are untouched, as ADR-0097's controls were wired to nothing. The kick is
//! derived from the membrane rule rather than taken from the task: ADR-0076's stimulus fires
//! each unit once only because its cancel then holds the unit below rest for about 3 500
//! ticks, which is a release, and is this round's release. The kick is a ramp that each member
//! stops taking when it fires, and a reset on the tick its refractory window ends that puts its
//! basal compartment back at the drive's mean standing: the reset was derived again after the
//! first reading of the kick on the engine, when the ramp alone left the members of sixteen one
//! spike over the measure's mark in the pair window after the span.
//!
//! The harness is `tests/instrument.rs`'s, shared as one module and not copied (ADR-0083).

#![deny(clippy::arithmetic_side_effects)]

// The harness — the network, the settled image, the oracle's scaling, every rule and table of
// ADR-0065 to ADR-0110 — is `tests/instrument.rs`'s module, compiled into this binary as well.
// What this binary does not call is that binary's, so the module's unused items and imports
// are allowed here and nowhere else in this file.
#[allow(dead_code, unused_imports)]
#[path = "instrument/harness.rs"]
mod harness;
use harness::*;

use cortex_connectome::SECTION_SYNAPSE;
use cortex_core::{BASAL_LEAK_SHIFT, STP_MAX, STP_U, SYNAPSES_PER_BLOCK, synaptic_efficacy_q16};
use cortex_runtime::{Inject, mix64};

// ------------------------------------------------ the grid (brief 048), before any run

/// The assemblies' sizes, in units: 16, 32 and 64, the brief's.
const SIZES: [u32; 3] = [16, 32, 64];

/// The recurrent weights, Q1.15: a quarter, a half and three quarters of the range, and its
/// top, `i16::MAX`, one LSB below 1.0. The prior's excitatory weights are 6 000 to 12 000.
const WEIGHTS: [i16; 4] = [0x2000, 0x4000, 0x6000, i16::MAX];

/// A member sends one synapse to each of the members after it in the assembly's order,
/// wrapping, as many as there are others and at most 32, the prior's own fan-out: all the
/// others at 16 and 32 units, the next 32 of 63 at 64 units. Every member receives as many
/// as it sends.
const FAN_MAX: u32 = 32;

/// Where the members sit: places 5 and 16 of each period of twenty of the task's geometry,
/// from the ring's start, the first `size / 2` periods. Both places are excitatory by the
/// prior's rule (every fifth unit from the fifth is inhibitory: places 4, 9, 14 and 19),
/// neither is a stimulus place (0 and 11), and they are eleven and nine places apart, beyond
/// the prior's window of eight, so no local synapse of the prior joins two members. Place 5 is
/// readout 0's and place 16 readout 1's: the geometry puts 1 020 of the 1 024 units in its
/// four sets, so no assembly of sixteen units or more is disjoint from the readouts
/// (F-54), and this placement splits every assembly evenly between them. The sizes nest: the
/// assembly of 16 is the first sixteen members of 32's, and 32's of 64's.
const PLACES: u32 = (1 << 5) | (1 << 16);

const _: () = assert!(PLACES & ((1 << A_OFFSET) | (1 << B_OFFSET)) == 0);
const _: () = assert!(PLACES & (R0_MASK | R1_MASK) == PLACES);
const _: () = assert!((PLACES & R0_MASK).count_ones() == 1 && (PLACES & R1_MASK).count_ones() == 1);
const _: () = assert!(16 - 5 > PRIOR_WINDOW && PERIOD + 5 - 16 > PRIOR_WINDOW);

/// The seed of the delays' draw: each synapse's delay is drawn from the prior's local band,
/// 100 to 300 ticks inclusive, by `mix64` of the seed, its source and its target.
const DELAY_SEED: u64 = 48;

/// The assembly of `size` units, as a set of the task's shape.
fn assembly(size: u32) -> Set {
    Set {
        first: 0,
        period: PERIOD,
        mask: PLACES,
        count: size.checked_div(PLACES.count_ones()).unwrap_or(0),
    }
}

/// The members, in the assembly's order.
fn members(size: u32) -> Vec<u32> {
    assembly(size).units().collect()
}

/// The synapses a member sends, and receives.
fn fan(size: u32) -> u32 {
    size.saturating_sub(1).min(FAN_MAX)
}

/// The blocks a member's synapses take.
fn blocks_per_member(size: u32) -> u32 {
    fan(size).div_ceil(SYNAPSES_PER_BLOCK as u32)
}

/// The blocks an assembly of `size` appends to the arena.
fn extra_blocks(size: u32) -> u64 {
    u64::from(size).saturating_mul(u64::from(blocks_per_member(size)))
}

/// The delay of the synapse from `from` to `to`: the prior's local band, drawn.
fn delay_of(from: u32, to: u32) -> u16 {
    let p = prior(1024);
    let width = u64::from(p.delay_max.saturating_sub(p.delay_min)).saturating_add(1);
    let draw = mix64(DELAY_SEED ^ (u64::from(from) << 32 | u64::from(to)));
    p.delay_min
        .saturating_add(draw.checked_rem(width).unwrap_or(0) as u16)
}

// ------------------------------------------------ the protocol (brief 048), before any run

/// An epoch: one trial's length, $2^{14}$ ticks.
const EPOCH_TICKS: u32 = TRIAL_TICKS;

/// The windows an epoch is read in: eight of 2 048 ticks.
const WINDOWS: usize = 8;

const WINDOW_SPAN: u32 = EPOCH_TICKS / WINDOWS as u32;

const _: () = assert!(WINDOW_SPAN == 2_048 && WINDOW_SPAN == PAIR_WINDOW);

/// The kinds of epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Epoch {
    /// No kick and no release.
    Unkicked,
    /// The kick at the epoch's start.
    Kicked,
    /// The kick at the start and the release at `RELEASE_AT`.
    Released,
}

/// The epochs' order: an unkicked epoch, a kicked one and a released one, `ROUNDS` times, so
/// that every unkicked epoch but the first follows a release and every kicked epoch follows
/// an unkicked one.
const ORDER: [Epoch; 3] = [Epoch::Unkicked, Epoch::Kicked, Epoch::Released];

const ROUNDS: usize = 8;

/// The lead-in: one epoch under the drive, no kick, read as the rows' first.
const LEAD_IN_EPOCHS: usize = 1;

/// The rows of a run: the lead-in and the twenty-four epochs.
const ROWS: usize = LEAD_IN_EPOCHS + ROUNDS * ORDER.len();

const _: () = assert!(ROWS == 25);

/// The kind of the run's row `row`: the lead-in is unkicked.
fn kind_of(row: usize) -> Epoch {
    match row.checked_sub(LEAD_IN_EPOCHS) {
        None => Epoch::Unkicked,
        Some(e) => ORDER[e.checked_rem(ORDER.len()).unwrap_or(0)],
    }
}

/// The kick: one basal message of `KICK_MESSAGE_Q16` into every member before each of the
/// epoch's first `KICK_TICKS` ticks, a ramp. A member fires when its soma crosses the
/// threshold and drops the rest of the ramp in its refractory window, so it fires once
/// whatever its standing potential, with about what firing takes left in its basal
/// compartment, which the reset (`KICK_RESET_Q16`) takes back on the tick the window ends.
/// The message is derived by `kick_rule`: the least power of two, of the five
/// from $2^{-8}$ to $2^{-4}$, under which the oracle fires a unit exactly once within the span
/// and not again within a pair window after it from every standing of `KICK_STANDINGS`
/// under the drive's mean input: 1/64.
const KICK_MESSAGE_Q16: i32 = 0x0400;

/// The span a kick's volley is read in: the epoch's ticks 1 to `REFRACTORY_TICKS`, in which
/// no unit fires twice.
const KICK_SPAN: u32 = REFRACTORY_TICKS as u32;

/// The ramp's ticks: the span less five of the soma's time constants toward its compartments
/// (eight ticks, the coupling's sixteenth to each), so that a unit the ramp's last message
/// brings to the threshold fires inside the span.
const KICK_TICKS: u32 = KICK_SPAN - 5 * 8;

const _: () = assert!(KICK_TICKS == 160);

/// The standings the kick is derived over, `(basal, soma)`: at rest; the drive's mean
/// standing (`drive_standing`, 0.875 and 0.436, written here as the oracle reads it and held
/// to it by the gate); the most a unit can stand at without firing (ADR-0076's extreme); and
/// one and two thresholds below rest, the soma at half the basal.
const KICK_STANDINGS: [(i32, i32); 5] = [
    (0, 0),
    DRIVE_STANDING,
    EXTREME_STANDING,
    (-THRESHOLD_BASE, -THRESHOLD_BASE / 2),
    (-2 * THRESHOLD_BASE, -THRESHOLD_BASE),
];

/// The messages the kick's rule scans, in order.
const KICK_SCAN: [i32; 5] = [0x0100, 0x0200, 0x0400, 0x0800, 0x1000];

/// The kick's reset, derived again after the first reading of the kick on the engine
/// (ADR-0112): one basal message into each member on the tick after its kick spike's
/// refractory window, whose efficacy the gain takes nearest to putting the member's basal
/// compartment back at the drive's mean standing from what the ramp left in it
/// (`reset_rule`). The ramp alone left a member about 0.27 below its threshold with its basal
/// at about 1.49 as its window ended, where the drive alone holds a unit at 0.875, and at
/// sixteen units the members' spikes in the pair window after the span read 26 over sixteen
/// kicks against the measure's 25.6. With the reset the kick fires each member once and leaves
/// nothing of itself in the member.
const KICK_RESET_Q16: i32 = -23_062;

/// The release: ADR-0076's cancel as built — `CANCEL_AT_THE_EXTREME` messages of
/// `CANCEL_MESSAGE_Q16` into every member before each of `CANCEL_TICKS` ticks — from the
/// epoch's quarter. A member refractory through all nine ticks escapes it: at a rate of $r$ Hz
/// about $192 r / 10^5$ of the members. Its direct effect on a member it reaches is gone about
/// 3 500 ticks later (`RELEASE_RECOVERED`), before the tail opens at the epoch's half.
const RELEASE_AT: u32 = EPOCH_TICKS / 4;

fn release() -> Cancel {
    Cancel {
        offset: RELEASE_AT,
        ticks: CANCEL_TICKS,
        messages: CANCEL_AT_THE_EXTREME,
        efficacy_q16: CANCEL_MESSAGE_Q16,
    }
}

// ------------------------------------------------ the measures (brief 048), before any run

/// Holds: the members' rate over the last half of a kicked epoch at least five times their
/// background, in at least seven of the eight kicked epochs. Ignites: an unkicked epoch whose
/// last half reaches that rate; usable with at most one of eight. Lets go: the members' rate
/// over the released epoch's tail, its last half, at most twice their background, in at least
/// seven of eight. Spills: the rest of the network's rate over the last halves of the kicked
/// epochs that held more than twice its background.
const HOLD_TIMES: u64 = 5;

const LET_GO_TIMES: u64 = 2;

const SPILL_TIMES: u64 = 2;

const OF_EIGHT_MIN: u32 = 7;

const IGNITIONS_MAX: u32 = 1;

/// The last half of an epoch, in windows: the hold's window and the tail's.
const HALF_FROM: usize = WINDOWS / 2;

/// The window before the release, read as whether the released epoch held before it.
const BEFORE_RELEASE: usize = RELEASE_AT as usize / WINDOW_SPAN as usize - 1;

const _: () = assert!(BEFORE_RELEASE == 1 && RELEASE_AT * 2 == EPOCH_TICKS / 2);

/// One window of a run: the members' spikes, the rest of the network's, and the sums over the
/// members of the pair `(u, R)` a spike on the next tick would release with (`step_stp` on a
/// copy of each member's factors, with the ticks since its last spike).
type WindowRow = [u32; 4];

/// One epoch of a run: its windows, and the kick's reading — the members' spikes in the span
/// and in the pair window after it (read in every epoch; a kick is in the kicked and released
/// ones only).
type EpochRow = ([WindowRow; WINDOWS], [u32; 2]);

/// A background: the members' and the rest's spikes over the background run's twenty-four
/// epochs, and their ticks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Background {
    members: u64,
    rest: u64,
    ticks: u64,
}

/// The background over a run's rows: the twenty-four epochs after the lead-in.
fn background_of(rows: &[EpochRow]) -> Background {
    let mut b = Background {
        members: 0,
        rest: 0,
        ticks: 0,
    };
    for (windows, _) in rows.iter().skip(LEAD_IN_EPOCHS) {
        for w in windows {
            b.members = b.members.saturating_add(u64::from(w[0]));
            b.rest = b.rest.saturating_add(u64::from(w[1]));
        }
        b.ticks = b.ticks.saturating_add(u64::from(EPOCH_TICKS));
    }
    b
}

/// The members' and the rest's spikes over an epoch's last half.
fn last_half(row: &EpochRow) -> (u64, u64) {
    row.0
        .iter()
        .skip(HALF_FROM)
        .fold((0u64, 0u64), |(m, r), w| {
            (
                m.saturating_add(u64::from(w[0])),
                r.saturating_add(u64::from(w[1])),
            )
        })
}

/// The ticks of an epoch's last half.
const HALF_TICKS: u64 = EPOCH_TICKS as u64 / 2;

/// `spikes` over `ticks` at least `times` the rate of `base` over `base_ticks`, in integers:
/// `spikes × base_ticks ≥ times × base × ticks`.
fn at_least(spikes: u64, ticks: u64, base: u64, base_ticks: u64, times: u64) -> bool {
    spikes.saturating_mul(base_ticks) >= times.saturating_mul(base).saturating_mul(ticks)
}

/// A cell's reading by the rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
    /// Kicked epochs whose last half held at least five times the background, of eight.
    held: u32,
    /// Unkicked epochs whose last half reached that rate, of eight.
    ignited: u32,
    /// Released epochs whose tail stood at most twice the background, of eight.
    let_go: u32,
    /// Released epochs whose window before the release held at least five times the
    /// background, of eight: a reading, no clause.
    before: u32,
    /// Whether the rest of the network fired more than twice its background over the last
    /// halves of the kicked epochs that held; none when none held.
    spills: Option<bool>,
}

impl Cell {
    fn holds(&self) -> bool {
        self.held >= OF_EIGHT_MIN
    }

    fn lets_go(&self) -> bool {
        self.let_go >= OF_EIGHT_MIN
    }

    fn quiet_unkicked(&self) -> bool {
        self.ignited <= IGNITIONS_MAX
    }

    /// Usable: holds, lets go, ignites in at most one unkicked epoch of eight, does not spill.
    fn usable(&self) -> bool {
        self.holds() && self.lets_go() && self.quiet_unkicked() && self.spills == Some(false)
    }
}

/// A run's rows read by the rules against `bg`.
fn cell(rows: &[EpochRow], bg: Background) -> Cell {
    let mut c = Cell {
        held: 0,
        ignited: 0,
        let_go: 0,
        before: 0,
        spills: None,
    };
    let (mut rest, mut held_ticks) = (0u64, 0u64);
    for (row, r) in rows.iter().enumerate().skip(LEAD_IN_EPOCHS) {
        let (m, rest_half) = last_half(r);
        let high = at_least(m, HALF_TICKS, bg.members, bg.ticks, HOLD_TIMES);
        match kind_of(row) {
            Epoch::Unkicked => c.ignited = c.ignited.saturating_add(u32::from(high)),
            Epoch::Kicked => {
                if high {
                    c.held = c.held.saturating_add(1);
                    rest = rest.saturating_add(rest_half);
                    held_ticks = held_ticks.saturating_add(HALF_TICKS);
                }
            }
            Epoch::Released => {
                let quiet = m.saturating_mul(bg.ticks)
                    <= LET_GO_TIMES
                        .saturating_mul(bg.members)
                        .saturating_mul(HALF_TICKS);
                c.let_go = c.let_go.saturating_add(u32::from(quiet));
                let before = u64::from(r.0[BEFORE_RELEASE][0]);
                c.before = c.before.saturating_add(u32::from(at_least(
                    before,
                    u64::from(WINDOW_SPAN),
                    bg.members,
                    bg.ticks,
                    HOLD_TIMES,
                )));
            }
        }
    }
    if c.held > 0 {
        c.spills = Some(
            rest.saturating_mul(bg.ticks)
                > SPILL_TIMES
                    .saturating_mul(bg.rest)
                    .saturating_mul(held_ticks),
        );
    }
    c
}

/// A kick's reading over a run: the members' spikes in the span and in the pair window after
/// it, summed over the kicked and released epochs; the kicks; and the kicks whose volley was
/// every member.
fn kick_reading(rows: &[EpochRow], size: u32) -> (u64, u64, u64, u32) {
    let (mut volley, mut after, mut kicks, mut full) = (0u64, 0u64, 0u64, 0u32);
    for (row, (_, k)) in rows.iter().enumerate() {
        if kind_of(row) != Epoch::Unkicked {
            volley = volley.saturating_add(u64::from(k[0]));
            after = after.saturating_add(u64::from(k[1]));
            kicks = kicks.saturating_add(1);
            full = full.saturating_add(u32::from(k[0] == size));
        }
    }
    (volley, after, kicks, full)
}

/// The kick fires every member once, by the measure that picked ADR-0076's stimulus
/// (`fires_once`, ADR-0074) read per kick: the volley within `VOLLEY_TOLERANCE_TENTHS` tenths
/// of one spike per member per kick and at most one, and the members' spikes in the pair
/// window after the span at most `AFTER_MAX_TENTHS` tenths per member per kick.
fn kicked_once(rows: &[EpochRow], size: u32) -> bool {
    let (volley, after, kicks, _) = kick_reading(rows, size);
    let once = u64::from(size).saturating_mul(10).saturating_mul(kicks);
    let tolerance = VOLLEY_TOLERANCE_TENTHS.saturating_mul(kicks);
    kicks > 0
        && volley.saturating_mul(10) <= once
        && volley.saturating_mul(10) >= once.saturating_sub(tolerance)
        && after.saturating_mul(10)
            <= u64::from(size)
                .saturating_mul(kicks)
                .saturating_mul(AFTER_MAX_TENTHS)
}

// ------------------------------------------------ the arithmetic, before any run

/// The membrane rule stepped alone, as the harness's `alone` steps it: `cortex-core`'s
/// `integrate` on one unit at the base threshold with no synapse, from `standing` basal and
/// somatic potentials, `input(k)` the basal input after the gain that lands on tick `k + 1`.
struct Stepped {
    fires: Vec<u32>,
    basal: Vec<i32>,
    soma: Vec<i32>,
}

fn stepped(standing: (i32, i32), input: impl Fn(u32) -> i32, ticks: u32) -> Stepped {
    let mut unit = DendriticSuperNeuron::new(0);
    unit.v_thresh = THRESHOLD_BASE;
    unit.v_basal = standing.0;
    unit.v_soma = standing.1;
    let mut out = Stepped {
        fires: Vec::new(),
        basal: Vec::with_capacity(ticks as usize),
        soma: Vec::with_capacity(ticks as usize),
    };
    for k in 0..ticks {
        let now = k.saturating_add(1);
        if unit.integrate(input(k), 0, now) {
            out.fires.push(now);
        }
        out.basal.push(unit.v_basal);
        out.soma.push(unit.v_soma);
    }
    out
}

/// The pairs `(u, R)` a unit's spikes release with when it fires every `interval` ticks from
/// short-term plasticity at rest: `step_stp` itself on a unit whose factors are the prior's at
/// rest, the first spike after a rest of `u32::MAX` ticks as the executor reads a first spike.
fn stp_course(interval: u32, spikes: usize) -> Vec<(u8, u8)> {
    let mut unit = DendriticSuperNeuron::new(0);
    unit.stp_u_rel = STP_U;
    unit.stp_r_ves = STP_MAX;
    let mut elapsed = u32::MAX;
    let mut out = Vec::with_capacity(spikes);
    for _ in 0..spikes {
        out.push(unit.step_stp(elapsed));
        elapsed = interval;
    }
    out
}

/// The spikes a course runs before its pair is read as steady.
const STP_SPIKES: usize = 256;

/// The steady pair at `interval`: the last of a course of `STP_SPIKES` spikes, whose last two
/// pairs are equal (asserted: the integer rule reaches a fixed point).
fn steady(interval: u32) -> (u8, u8) {
    let course = stp_course(interval, STP_SPIKES);
    let last = course[STP_SPIKES.wrapping_sub(1)];
    assert_eq!(
        course[STP_SPIKES.wrapping_sub(2)],
        last,
        "the course at {interval} ticks is steady"
    );
    last
}

/// The pair a spike after a long rest releases with.
fn at_rest() -> (u8, u8) {
    stp_course(0, 1)[0]
}

/// The factor a release takes from short-term plasticity: `u × R`, both Q0.8.
fn factor(pair: (u8, u8)) -> u32 {
    u32::from(pair.0).saturating_mul(u32::from(pair.1))
}

/// The intervals the arithmetic reads the steady pair at, in ticks: the background's 1.76 Hz
/// (ADR-0097), 5, 10, 20, 40, 100, 200 and 400 Hz. 20 Hz, 5 000 ticks, is the brief's.
const INTERVALS: [u32; 8] = [56_818, 20_000, 10_000, 5_000, 2_500, 1_000, 500, 250];

const AT_20_HZ: u32 = 5_000;

/// What one spike of a member delivers to one target's basal compartment: the efficacy of
/// `weight` under `pair` (`synaptic_efficacy_q16`, ADR-0012), scaled by the gain as the
/// executor scales a turn's sum (F-47).
fn delivered(weight: i16, pair: (u8, u8)) -> i32 {
    scaled_q16(synaptic_efficacy_q16(weight, pair.0, pair.1), GAIN_1024)
}

/// The drive's mean input to one unit per tick, after the gain: ADR-0044's drive sends
/// `messages` messages of `efficacy_q16` a tick among `units` units, each scaled by the gain.
fn drive_mean_per_tick() -> i32 {
    let d = drive(1024);
    i64::from(scaled_q16(d.efficacy_q16, GAIN_1024))
        .saturating_mul(i64::from(d.messages))
        .checked_div(i64::from(d.units))
        .unwrap_or(0) as i32
}

/// The ticks the drive's mean standing is stepped to.
const STANDING_TICKS: u32 = 1 << 14;

/// The drive's mean standing as the oracle reads it: the basal and somatic potentials a unit
/// settles at under the drive's mean input every tick, stepped from rest (the drive's shot
/// noise is not in it). Written as a constant so that the kick's standings can name it.
const DRIVE_STANDING: (i32, i32) = (57_344, 28_561);

fn drive_standing() -> (i32, i32) {
    let mean = drive_mean_per_tick();
    let s = stepped((0, 0), |_| mean, STANDING_TICKS);
    let last = STANDING_TICKS.wrapping_sub(1) as usize;
    (s.basal[last], s.soma[last])
}

/// The highest somatic potential a unit at the drive's mean standing reaches under the drive's
/// mean input when `basal` (after the gain) lands on tick one, within a pair window; and
/// whether it fires.
fn rise(basal: i32) -> (i32, bool) {
    let mean = drive_mean_per_tick();
    let s = stepped(
        DRIVE_STANDING,
        |k| {
            if k == 0 {
                mean.saturating_add(basal)
            } else {
                mean
            }
        },
        PAIR_WINDOW,
    );
    let peak = s.soma.iter().copied().max().unwrap_or(DRIVE_STANDING.1);
    (peak, !s.fires.is_empty())
}

/// The most messages the least count below is searched over.
const LEAST_SCAN_MESSAGES: u32 = 64;

/// The least count of spikes landing together, each one member's at `weight` under `pair`,
/// that fires a unit at the drive's mean standing: the executor sums the batch's efficacies and
/// scales the sum by the gain; none up to `LEAST_SCAN_MESSAGES`.
fn least_together(weight: i16, pair: (u8, u8)) -> Option<u32> {
    let efficacy = i64::from(synaptic_efficacy_q16(weight, pair.0, pair.1));
    (1..=LEAST_SCAN_MESSAGES).find(|&n| {
        let sum = efficacy
            .saturating_mul(i64::from(n))
            .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        rise(scaled_q16(sum, GAIN_1024)).1
    })
}

/// The members' own mean input to one member at `weight` while every member fires every
/// `interval` ticks, as the basal potential it would hold and the somatic potential that
/// holds against it (the soma leaks and is coupled by a sixteenth to each compartment, the
/// apical at zero: `soma = basal × 128 / 257`), both Q16.16: `fan × delivered × 512 /
/// interval`, 512 the basal leak's time constant.
fn recurrent(size: u32, weight: i16, interval: u32) -> (i64, i64) {
    let per_spike = i64::from(delivered(weight, steady(interval)));
    let basal = i64::from(fan(size))
        .saturating_mul(per_spike)
        .saturating_mul(1 << BASAL_LEAK_SHIFT)
        .checked_div(i64::from(interval))
        .unwrap_or(0);
    (
        basal,
        basal.saturating_mul(128).checked_div(257).unwrap_or(0),
    )
}

/// The kick's input to one unit: `mean` every tick (the drive's mean input, or nothing), and
/// the ramp's message scaled by the gain before each of the ramp's ticks.
fn kick_input(message: i32, mean: i32) -> impl Fn(u32) -> i32 {
    let per = scaled_q16(batch_q16(1, message), GAIN_1024);
    move |k| {
        if k < KICK_TICKS {
            mean.saturating_add(per)
        } else {
            mean
        }
    }
}

/// The kick by the oracle from `standing` under `mean` every tick, the ramp of `message` and,
/// when given, the reset of
/// `reset` landing on the first tick after the unit's first spike's refractory window: the
/// stepped unit over the span and a pair window after it, and the tick of its first spike.
fn kicked(
    standing: (i32, i32),
    message: i32,
    reset: Option<i32>,
    mean: i32,
) -> (Stepped, Option<u32>) {
    let ticks = KICK_SPAN.saturating_add(PAIR_WINDOW);
    let first = stepped(standing, kick_input(message, mean), ticks)
        .fires
        .first()
        .copied();
    let ramp = kick_input(message, mean);
    let per = reset.map_or(0, |r| scaled_q16(batch_q16(1, r), GAIN_1024));
    // A spike on tick `t` leaves the unit refractory through tick `t + 200`; the reset lands on
    // tick `t + 201`, the input of index `t + 200`.
    let at = first.map(|t| t.saturating_add(u32::from(REFRACTORY_TICKS)));
    let s = stepped(
        standing,
        |k| {
            if Some(k) == at {
                ramp(k).saturating_add(per)
            } else {
                ramp(k)
            }
        },
        ticks,
    );
    (s, first)
}

/// The ticks after the reset at which the kick's oracle reads the soma again.
const SETTLED_AFTER_RESET: usize = 64;

/// The kick by the oracle from `standing` (`kicked` with `KICK_RESET_Q16`): the ticks the unit
/// fires on; its basal potential on the tick it first fires; its somatic and basal potentials
/// on the last tick of its refractory window; its basal potential on the tick the reset lands;
/// and its somatic potential `SETTLED_AFTER_RESET` ticks after it.
fn kick_oracle(standing: (i32, i32), message: i32) -> KickRead {
    let (s, first) = kicked(
        standing,
        message,
        Some(KICK_RESET_Q16),
        drive_mean_per_tick(),
    );
    let at = first.unwrap_or(0).saturating_sub(1) as usize;
    let last = s.soma.len().saturating_sub(1);
    let end = at.saturating_add(usize::from(REFRACTORY_TICKS)).min(last);
    let reset = end.saturating_add(1).min(last);
    let later = reset.saturating_add(SETTLED_AFTER_RESET).min(last);
    (
        s.fires.clone(),
        s.basal.get(at).copied().unwrap_or(0),
        s.soma[end],
        s.basal[end],
        s.basal[reset],
        s.soma[later],
    )
}

/// The reset by the rule: the one message, before the gain, that the gain takes nearest to the
/// input putting a unit's basal compartment at the drive's mean standing on the tick it lands,
/// from what the ramp left there, by the oracle kicked from the drive's mean standing with no
/// reset: the basal after the refractory window's last tick, leaked by the rule, less the
/// drive's mean input, less the standing, negated.
fn reset_rule() -> i32 {
    let (s, first) = kicked(
        DRIVE_STANDING,
        KICK_MESSAGE_Q16,
        None,
        drive_mean_per_tick(),
    );
    let t = first.expect("the kick fires the unit");
    let b = s.basal[t
        .saturating_add(u32::from(REFRACTORY_TICKS))
        .saturating_sub(1) as usize];
    let leaked = b.saturating_sub((b >> BASAL_LEAK_SHIFT).max(1));
    let wanted = DRIVE_STANDING
        .0
        .saturating_sub(leaked)
        .saturating_sub(drive_mean_per_tick());
    let estimate = (i64::from(wanted) << 16)
        .checked_div(i64::from(GAIN_1024))
        .unwrap_or(0) as i32;
    // The nearest of the estimate and its two neighbours, by what the gain makes of each.
    [
        estimate.saturating_sub(1),
        estimate,
        estimate.saturating_add(1),
    ]
    .into_iter()
    .min_by_key(|&e| {
        scaled_q16(batch_q16(1, e), GAIN_1024)
            .saturating_sub(wanted)
            .unsigned_abs()
    })
    .unwrap_or(estimate)
}

/// The kick fires a unit once by the oracle: one spike, inside the span.
fn once_by_oracle(fires: &[u32]) -> bool {
    matches!(fires, [t] if (1..=KICK_SPAN).contains(t))
}

/// The kick's message by the rule: the first of `KICK_SCAN` under which every standing of
/// `KICK_STANDINGS` fires once by the oracle.
fn kick_rule() -> Option<i32> {
    KICK_SCAN.iter().copied().find(|&m| {
        KICK_STANDINGS
            .iter()
            .all(|&s| once_by_oracle(&kicked(s, m, None, drive_mean_per_tick()).0.fires))
    })
}

/// The release by the oracle from `standing` under the drive's mean input: the ticks the unit
/// fires on within the rest of the epoch after the release, its lowest basal potential, and
/// the first tick after which its somatic potential is back within a tenth of the threshold of
/// the drive's mean standing.
fn release_oracle(standing: (i32, i32)) -> (Vec<u32>, i32, Option<u32>) {
    let mean = drive_mean_per_tick();
    let per = scaled_q16(
        batch_q16(CANCEL_AT_THE_EXTREME, CANCEL_MESSAGE_Q16),
        GAIN_1024,
    );
    let s = stepped(
        standing,
        |k| {
            if k < CANCEL_TICKS {
                mean.saturating_add(per)
            } else {
                mean
            }
        },
        EPOCH_TICKS.saturating_sub(RELEASE_AT),
    );
    let near = DRIVE_STANDING.1.saturating_sub(RECOVERED_WITHIN);
    let back = s
        .soma
        .iter()
        .position(|&v| v > near)
        .map(|k| k.saturating_add(1) as u32);
    (s.fires, s.basal.iter().copied().min().unwrap_or(0), back)
}

/// A tenth of the threshold: how near the drive's mean standing a released soma is back.
const RECOVERED_WITHIN: i32 = THRESHOLD_BASE / 10;

/// The ticks after the release's first message by which a member it reached at the drive's
/// mean standing is back within a tenth of the threshold of that standing, by the oracle.
const RELEASE_RECOVERED: u32 = 3_510;

const _: () = assert!(RELEASE_AT + RELEASE_RECOVERED < EPOCH_TICKS / 2);

// ------------------------------------------------ the wiring

/// `image` with `extra` empty blocks appended to its synapse section, laid out again as
/// `Image::encode` lays an image out: the header with the blocks counted and sealed, the
/// directory, every section at its aligned offset with its CRC. Every record is the image's;
/// an empty block is in no chain until one is linked to it.
fn grown(image: &[u8], extra: u64) -> Vec<u8> {
    let header = CortexFileHeader::decode(image[0..64].try_into().unwrap());
    let mut sections: Vec<(u32, u32, Vec<u8>)> = Vec::new();
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let entry = SectionEntry::decode(image[at..][..64].try_into().unwrap());
        let mut bytes = image[entry.offset as usize..][..entry.length as usize].to_vec();
        if entry.kind == SECTION_SYNAPSE {
            bytes.resize(
                bytes
                    .len()
                    .saturating_add(extra.saturating_mul(64) as usize),
                0,
            );
        }
        sections.push((entry.kind, entry.record_size, bytes));
    }
    let aligned = |n: u64| n.div_ceil(64).saturating_mul(64);
    let mut offset = (sections.len() as u64)
        .saturating_mul(64)
        .saturating_add(64);
    let mut entries = Vec::with_capacity(sections.len());
    for (kind, record_size, bytes) in &sections {
        entries.push(SectionEntry::new(
            *kind,
            *record_size,
            offset,
            bytes.len() as u64,
            crc64(bytes),
        ));
        offset = offset.saturating_add(aligned(bytes.len() as u64));
    }
    let sealed = CortexFileHeader::new(
        header.num_columns,
        header.num_neurons,
        header.num_synapses.saturating_add(extra),
        header.section_count,
        header.tick_ns,
        header.written_tick,
    );
    let mut out = Vec::with_capacity(offset as usize);
    out.extend_from_slice(&sealed.encode());
    for entry in &entries {
        out.extend_from_slice(&entry.encode());
    }
    for (_, _, bytes) in &sections {
        out.extend_from_slice(bytes);
        out.resize(aligned(out.len() as u64) as usize, 0);
    }
    out
}

/// Wires the assembly of `size` at `weight` into `exec`, whose arena ends with the assembly's
/// `extra_blocks(size)` empty blocks after the prior's: member `k`'s synapses fill its
/// `blocks_per_member(size)` blocks in order, one to each of the next `fan(size)` members,
/// basal, with the drawn delays; its blocks are chained in order, the first after the last
/// block of its own chain.
fn wire(exec: &mut Engine, size: u32, weight: i16) {
    let base = blocks_for(&prior(1024)) as u32;
    let m = members(size);
    let per = blocks_per_member(size);
    let (units, blocks) = exec.arenas_mut();
    assert_eq!(
        blocks.len() as u64,
        u64::from(base).saturating_add(extra_blocks(size)),
        "the arena holds the prior's blocks and the assembly's"
    );
    for (k, &unit) in m.iter().enumerate() {
        let first = base.saturating_add((k as u32).saturating_mul(per));
        let last = units[unit as usize]
            .chain(blocks)
            .last()
            .expect("a unit of the prior has a chain");
        assert!(blocks[last as usize].link(first), "the first block chained");
        for d in 0..fan(size) {
            let target = m[k
                .saturating_add(1)
                .saturating_add(d as usize)
                .checked_rem(m.len())
                .unwrap_or(0)];
            let block = first.saturating_add(d.checked_div(SYNAPSES_PER_BLOCK as u32).unwrap_or(0));
            let slot = d.checked_rem(SYNAPSES_PER_BLOCK as u32).unwrap_or(0) as usize;
            if slot == 0 && d > 0 {
                assert!(
                    blocks[block.saturating_sub(1) as usize].link(block),
                    "the next block chained"
                );
            }
            assert!(
                blocks[block as usize].set_synapse(
                    slot,
                    target,
                    weight,
                    delay_of(unit, target),
                    false
                ),
                "the synapse written"
            );
        }
    }
}

// ------------------------------------------------ the protocol, run

/// Whether each unit of the arena is in `set`.
fn membership(set: &[u32]) -> Vec<bool> {
    let mut of = vec![false; 1024];
    for &u in set {
        of[u as usize] = true;
    }
    of
}

/// The sums over `set` of the pair a spike on tick `now` would release with: `step_stp` on a
/// copy of each member's two factors, with the ticks since its last spike as the executor
/// reads them.
fn stp_sums(units: &[DendriticSuperNeuron], set: &[u32], now: u32) -> (u32, u32) {
    let (mut su, mut sr) = (0u32, 0u32);
    for &m in set {
        let unit = &units[m as usize];
        let mut copy = DendriticSuperNeuron::new(0);
        copy.stp_u_rel = unit.stp_u_rel;
        copy.stp_r_ves = unit.stp_r_ves;
        let elapsed = if unit.last_soma_spike_tick == NO_SPIKE_ON_RECORD {
            u32::MAX
        } else {
            now.wrapping_sub(unit.last_soma_spike_tick)
        };
        let (u, r) = copy.step_stp(elapsed);
        su = su.saturating_add(u32::from(u));
        sr = sr.saturating_add(u32::from(r));
    }
    (su, sr)
}

/// The protocol from the executor's clock: the lead-in and the twenty-four epochs under
/// ADR-0044's drive, the kick and the release into `kicks` when given (none: every epoch is
/// run unkicked, the background), each set of `reads` read per window. Returns each set's rows.
/// What the protocol injects into `set` before the epoch's tick `k` of an epoch of `kind`: the
/// kick's ramp, the resets `resets` holds due at `k`, and the release.
fn inject_before(inject: &Inject, set: &[u32], kind: Epoch, k: u32, resets: &[(u32, u32)]) {
    let r = release();
    if kind != Epoch::Unkicked && k < KICK_TICKS {
        for &m in set {
            inject
                .inject(m, spike_message(KICK_MESSAGE_Q16, false))
                .expect("the ring has room");
        }
    }
    for &(_, m) in resets.iter().filter(|&&(due, _)| due == k) {
        inject
            .inject(m, spike_message(KICK_RESET_Q16, false))
            .expect("the ring has room");
    }
    if kind == Epoch::Released && r.is_due(k) {
        for &m in set {
            for _ in 0..r.messages {
                inject
                    .inject(m, spike_message(r.efficacy_q16, false))
                    .expect("the ring has room");
            }
        }
    }
}

/// After the epoch's tick `k`, stamped `start + k`, of an epoch of `kind`: each member of `set`
/// whose kick spike it was, inside the span, has its reset due before the epoch's tick after its
/// refractory window, read from its record.
fn note_kick_spikes(
    exec: &Engine,
    set: &[u32],
    kind: Epoch,
    k: u32,
    start: u32,
    resets: &mut Vec<(u32, u32)>,
) {
    if kind == Epoch::Unkicked || !(1..=KICK_SPAN).contains(&k) {
        return;
    }
    let stamp = start.wrapping_add(k);
    for &m in set {
        if exec.units()[m as usize].last_soma_spike_tick == stamp {
            resets.push((k.saturating_add(u32::from(REFRACTORY_TICKS)), m));
        }
    }
}

fn protocol(exec: &mut Engine, reads: &[Vec<u32>], kicks: Option<&[u32]>) -> Vec<Vec<EpochRow>> {
    let drive = drive(1024);
    let inject = exec.injector();
    let of: Vec<Vec<bool>> = reads.iter().map(|s| membership(s)).collect();
    let mut rows: Vec<Vec<EpochRow>> = vec![Vec::with_capacity(ROWS); reads.len()];
    let mut seen = (exec.train().len() as u64).saturating_add(exec.train_overwritten());
    for row in 0..ROWS {
        let kind = kicks.map_or(Epoch::Unkicked, |_| kind_of(row));
        let start = exec.ticks() as u32;
        let mut epoch: Vec<EpochRow> = vec![([[0u32; 4]; WINDOWS], [0u32; 2]); reads.len()];
        // Each member's reset, due before the epoch's tick after its kick spike's refractory
        // window, in the order the members fired.
        let mut resets: Vec<(u32, u32)> = Vec::new();
        for w in 0..WINDOWS {
            for j in 0..WINDOW_SPAN {
                let k = (w as u32).saturating_mul(WINDOW_SPAN).saturating_add(j);
                drive.step(&inject, exec.ticks()).expect("the drive runs");
                if let Some(set) = kicks {
                    inject_before(&inject, set, kind, k, &resets);
                }
                exec.tick();
                if let Some(set) = kicks {
                    note_kick_spikes(exec, set, kind, k, start, &mut resets);
                }
            }
            let total = (exec.train().len() as u64).saturating_add(exec.train_overwritten());
            let new = total.saturating_sub(seen);
            let train = exec.train();
            assert!(new <= train.len() as u64, "the train held the window");
            for &(stamp, unit) in &train[train.len().saturating_sub(new as usize)..] {
                let t = stamp.wrapping_sub(start);
                for (read, member) in epoch.iter_mut().zip(of.iter()) {
                    if member[unit as usize] {
                        read.0[w][0] = read.0[w][0].saturating_add(1);
                        if (1..=KICK_SPAN).contains(&t) {
                            read.1[0] = read.1[0].saturating_add(1);
                        } else if t > KICK_SPAN && t <= KICK_SPAN.saturating_add(PAIR_WINDOW) {
                            read.1[1] = read.1[1].saturating_add(1);
                        }
                    } else {
                        read.0[w][1] = read.0[w][1].saturating_add(1);
                    }
                }
            }
            seen = total;
            let now = exec.ticks() as u32;
            for (read, set) in epoch.iter_mut().zip(reads.iter()) {
                let (su, sr) = stp_sums(exec.units(), set, now);
                read.0[w][2] = su;
                read.0[w][3] = sr;
            }
        }
        for (out, read) in rows.iter_mut().zip(epoch) {
            out.push(read);
        }
    }
    rows
}

// ------------------------------------------------ the runs

/// ADR-0077's settled image, frozen, built and held to ADR-0077's tables by the harness; the
/// inhibitory baseline and the signed gate asserted unset on an engine decoded from it.
fn settled(name: &str) -> Vec<u8> {
    let image = settled_image(name);
    let exec = frozen_from(&image, 1024);
    assert_eq!(
        exec.modulation_baseline_q16(),
        0,
        "{name}: the baseline zero"
    );
    assert_eq!(
        exec.inhibitory_baseline_q16(),
        None,
        "{name}: the inhibitory baseline unset"
    );
    assert!(!exec.signed_gate(), "{name}: the signed gate unset");
    image
}

/// A cell's engine: the frozen image grown by the assembly's blocks, decoded, and wired at
/// `weight`; with no weight, the grown image unwired.
fn cell_engine(image: &[u8], size: u32, weight: Option<i16>) -> Engine {
    let mut exec = frozen_from(&grown(image, extra_blocks(size)), 1024);
    if let Some(w) = weight {
        wire(&mut exec, size, w);
    }
    exec
}

/// One run of a cell or a control: the protocol with the kick and the release into the
/// assembly of `size`, read for it, the weights shown unchanged at its end.
fn cell_run(image: &[u8], size: u32, weight: Option<i16>, name: &str) -> Vec<EpochRow> {
    let mut exec = cell_engine(image, size, weight);
    let set = members(size);
    let before = weights_of(&exec);
    let rows = protocol(&mut exec, core::slice::from_ref(&set), Some(&set))
        .pop()
        .expect("one set read");
    eprintln!("DUMP {name} = {rows:?}");
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    rows
}

// ------------------------------------------------ the tests

/// The arithmetic as the oracles compute it, in the order the constants below hold it.
#[allow(clippy::type_complexity)]
fn arithmetic() -> (
    (u8, u8),
    Vec<(u8, u8)>,
    u64,
    (i32, i32),
    Vec<Delivered>,
    Vec<Vec<Vec<(i64, i64)>>>,
    Option<i32>,
    Vec<KickRead>,
    Vec<(Vec<u32>, i32, Option<u32>)>,
) {
    let steadies: Vec<(u8, u8)> = INTERVALS.iter().map(|&i| steady(i)).collect();
    let per_mille = u64::from(factor(steady(AT_20_HZ)))
        .saturating_mul(1_000)
        .checked_div(u64::from(factor(at_rest())))
        .unwrap_or(0);
    let delivered_by_weight = WEIGHTS
        .iter()
        .map(|&w| {
            let rest = delivered(w, at_rest());
            let hz20 = delivered(w, steady(AT_20_HZ));
            (
                rest,
                rise(rest).0,
                least_together(w, at_rest()),
                hz20,
                rise(hz20).0,
                least_together(w, steady(AT_20_HZ)),
            )
        })
        .collect();
    let recurrent_by_cell = SIZES
        .iter()
        .map(|&size| {
            WEIGHTS
                .iter()
                .map(|&w| {
                    [INTERVALS[0], AT_20_HZ, AT_100_HZ]
                        .iter()
                        .map(|&i| recurrent(size, w, i))
                        .collect()
                })
                .collect()
        })
        .collect();
    let kicks = KICK_STANDINGS
        .iter()
        .map(|&s| kick_oracle(s, KICK_MESSAGE_Q16))
        .collect();
    let releases = [DRIVE_STANDING, EXTREME_STANDING]
        .iter()
        .map(|&s| release_oracle(s))
        .collect();
    (
        at_rest(),
        steadies,
        per_mille,
        drive_standing(),
        delivered_by_weight,
        recurrent_by_cell,
        kick_rule(),
        kicks,
        releases,
    )
}

const AT_100_HZ: u32 = 1_000;

#[test]
fn the_grid_the_arithmetic_the_rules_and_an_assembly_kicked_on_the_engine() {
    // The grid and the members.
    let p = prior(1024);
    let [a, b, r0, r1] = geometry(1024, rotation(1024));
    for (i, &size) in SIZES.iter().enumerate() {
        let m = members(size);
        assert_eq!(m.len() as u32, size);
        for &u in &m {
            assert!(!p.is_inhibitory(u), "member {u} is excitatory");
            assert!(
                !a.contains(u) && !b.contains(u),
                "member {u} is no stimulus unit"
            );
        }
        let half = size.checked_div(2).unwrap_or(0) as usize;
        assert_eq!(m.iter().filter(|&&u| r0.contains(u)).count(), half);
        assert_eq!(m.iter().filter(|&&u| r1.contains(u)).count(), half);
        for pair in m.windows(2) {
            assert!(ring_distance(1024, pair[0], pair[1]) > PRIOR_WINDOW);
        }
        if let Some(smaller) = i.checked_sub(1).map(|j| SIZES[j]) {
            assert_eq!(
                &m[..smaller as usize],
                members(smaller).as_slice(),
                "nested"
            );
        }
    }
    assert_eq!(members(16).first(), Some(&5));
    assert_eq!(members(64).last(), Some(&636));
    assert_eq!([fan(16), fan(32), fan(64)], [15, 31, 32]);
    assert_eq!(
        [extra_blocks(16), extra_blocks(32), extra_blocks(64)],
        [64, 256, 512]
    );
    // The protocol.
    let kinds: Vec<Epoch> = (0..ROWS).map(kind_of).collect();
    assert_eq!(kinds[0], Epoch::Unkicked);
    for kind in ORDER {
        assert_eq!(
            kinds[LEAD_IN_EPOCHS..]
                .iter()
                .filter(|&&k| k == kind)
                .count(),
            ROUNDS
        );
    }
    assert_eq!(kinds[1..4], ORDER);
    assert_eq!(kinds[22..25], ORDER);
    let r = release();
    assert!(!r.is_due(RELEASE_AT.wrapping_sub(1)) && r.is_due(RELEASE_AT));
    let end = RELEASE_AT.saturating_add(CANCEL_TICKS);
    assert!(r.is_due(end.wrapping_sub(1)) && !r.is_due(end));
    // The arithmetic, as ADR-0112 writes it: dumped, then held.
    let (rest, steadies, per_mille, standing, by_weight, by_cell, rule, kicks, releases) =
        arithmetic();
    eprintln!("DUMP STP_AT_REST = {rest:?}");
    eprintln!("DUMP STP_STEADY = {steadies:?}");
    eprintln!("DUMP STEADY_20_HZ_PER_MILLE = {per_mille}");
    eprintln!("DUMP DRIVE_STANDING = {standing:?}");
    eprintln!("DUMP DELIVERED = {by_weight:?}");
    eprintln!("DUMP RECURRENT = {by_cell:?}");
    eprintln!("DUMP KICK_RULE = {rule:?}");
    eprintln!("DUMP KICK_RESET_Q16 = {}", reset_rule());
    eprintln!("DUMP KICK_ORACLE = {kicks:?}");
    eprintln!("DUMP RELEASE_ORACLE = {releases:?}");
    assert_eq!(rest, STP_AT_REST);
    assert_eq!(steadies.as_slice(), STP_STEADY.as_slice());
    assert_eq!(steady(AT_20_HZ), STP_STEADY[3]);
    assert_eq!(per_mille, STEADY_20_HZ_PER_MILLE);
    assert_eq!(drive_mean_per_tick(), 112);
    assert_eq!(standing, DRIVE_STANDING);
    assert_eq!(by_weight.as_slice(), DELIVERED.as_slice());
    for (k, &w) in WEIGHTS.iter().enumerate() {
        assert!(
            !rise(delivered(w, at_rest())).1,
            "one spike alone fires no unit at the drive's mean standing"
        );
        for (s, row) in by_cell.iter().enumerate() {
            assert_eq!(row[k].as_slice(), RECURRENT[s][k].as_slice());
        }
    }
    // The kick, derived; the release.
    assert_eq!(rule, Some(KICK_MESSAGE_Q16));
    assert_eq!(reset_rule(), KICK_RESET_Q16);
    for (k, (fires, basal, soma, basal_end, basal_reset, soma_later)) in kicks.iter().enumerate() {
        assert_eq!(
            (
                fires.as_slice(),
                *basal,
                *soma,
                *basal_end,
                *basal_reset,
                *soma_later
            ),
            KICK_ORACLE[k],
            "the kick from {:?}",
            KICK_STANDINGS[k]
        );
        assert!(once_by_oracle(fires));
        assert!(
            *soma < THRESHOLD_BASE,
            "below the base threshold as the window ends"
        );
    }
    assert!(
        !KICK_STANDINGS.iter().all(|&s| once_by_oracle(
            &kicked(s, KICK_SCAN[1], None, drive_mean_per_tick()).0.fires
        )),
        "the message below fails"
    );
    for (k, (fires, lowest, back)) in releases.iter().enumerate() {
        assert_eq!((fires.as_slice(), *lowest, *back), RELEASE_ORACLE[k]);
        assert!(
            fires.is_empty(),
            "a released unit fires no more in the epoch"
        );
    }
    assert_eq!(RELEASE_ORACLE[0].2, Some(RELEASE_RECOVERED));
    // The rules at their edges, over rows written by hand.
    rules_at_their_edges();
    // The wiring, the growth and the kick on the engine: the instrument's network at 1 024
    // units, at rest, frozen.
    wiring_on_the_prior();
    // One assembly wired and kicked for a few hundred ticks on that network.
    let kicked = an_assembly_kicked_for_a_few_hundred_ticks();
    eprintln!("DUMP GATE_KICKED = {kicked:?}");
    assert_eq!(kicked.as_slice(), GATE_KICKED, "the assembly kicked");
    // The rules over the kick's and the background's pinned tables.
    over_the_kick_tables();
    // The rules over the grid's pinned tables, and the burst reading.
    over_the_grid_tables();
}

/// A run's burst reading (`bursts`).
type BurstRead = (u32, u32, u32, u32, Option<u32>, u32);

/// A burst window: one whose members' spikes are at least the members' count, one spike a
/// member in 2 048 ticks, about 28 times the background (ADR-0112's reading, written after the
/// runs from what the tables showed; no clause reads it).
fn is_burst(w: &WindowRow, size: u32) -> bool {
    w[0] >= size
}

/// A quiet window: the members' spikes over it at most twice the background over a window.
fn is_quiet(w: &WindowRow, bg: Background) -> bool {
    u64::from(w[0]).saturating_mul(bg.ticks)
        <= LET_GO_TIMES
            .saturating_mul(bg.members)
            .saturating_mul(u64::from(WINDOW_SPAN))
}

/// The burst reading of a run over its twenty-four epochs (ADR-0112, after the runs): the burst
/// windows; those of them that are not a kicked or released epoch's first window, where the
/// kick's own burst falls; the kicked epochs whose last half the rule reads as held; those of
/// them with no quiet window in their last half, which a persistent hold would be; the fewest
/// quiet windows in a held last half; and the members' mean pool $R$ at the burst windows'
/// ends, in 255ths, rounded down.
fn bursts(rows: &[EpochRow], size: u32, bg: Background) -> BurstRead {
    let (mut windows, mut unkicked, mut held, mut persistent) = (0u32, 0u32, 0u32, 0u32);
    let mut quiet_min: Option<u32> = None;
    let mut pool = 0u64;
    for (row, (w, _)) in rows.iter().enumerate().skip(LEAD_IN_EPOCHS) {
        let kind = kind_of(row);
        for (i, window) in w.iter().enumerate() {
            if is_burst(window, size) {
                windows = windows.saturating_add(1);
                pool = pool.saturating_add(u64::from(window[3]));
                if kind == Epoch::Unkicked || i != 0 {
                    unkicked = unkicked.saturating_add(1);
                }
            }
        }
        let (m, _) = last_half(&rows[row]);
        if kind == Epoch::Kicked && at_least(m, HALF_TICKS, bg.members, bg.ticks, HOLD_TIMES) {
            held = held.saturating_add(1);
            let quiet = w[HALF_FROM..].iter().filter(|x| is_quiet(x, bg)).count() as u32;
            persistent = persistent.saturating_add(u32::from(quiet == 0));
            quiet_min = Some(quiet_min.map_or(quiet, |q| q.min(quiet)));
        }
    }
    let mean_pool = pool
        .checked_div(u64::from(windows).saturating_mul(u64::from(size)))
        .unwrap_or(0) as u32;
    (windows, unkicked, held, persistent, quiet_min, mean_pool)
}

/// The rules over the grid's pinned tables: each cell's twenty-five rows and its reading by the
/// rules against the background pinned before any cell ran; no cell usable; the three cells the
/// rule reads as holding named; and the burst reading of every cell and control, no held last
/// half of any cell without a quiet window.
fn over_the_grid_tables() {
    let mut usable = 0u32;
    let mut holding = Vec::new();
    for (s, &size) in SIZES.iter().enumerate() {
        let bg = BACKGROUNDS_1024[s];
        for (k, rows) in CELL_ROWS_1024[s].iter().enumerate() {
            assert_eq!(rows.len(), ROWS);
            let read = cell(rows, bg);
            assert_eq!(read, GRID_1024[s][k], "the cell of {size} at weight {k}");
            usable = usable.saturating_add(u32::from(read.usable()));
            if read.holds() {
                holding.push((size, WEIGHTS[k]));
            }
        }
    }
    assert_eq!(usable, 0, "no cell is usable");
    assert_eq!(
        holding,
        [(32, i16::MAX), (64, 0x6000), (64, i16::MAX)],
        "the cells the rule reads as holding"
    );
    let read: Vec<Vec<BurstRead>> = SIZES
        .iter()
        .enumerate()
        .map(|(s, &size)| {
            core::iter::once(CONTROL_ROWS_1024[s])
                .chain(CELL_ROWS_1024[s].iter().copied())
                .map(|rows| bursts(rows, size, BACKGROUNDS_1024[s]))
                .collect()
        })
        .collect();
    eprintln!("DUMP BURSTS_1024 = {read:?}");
    for (s, cells) in read.iter().enumerate() {
        assert_eq!(
            cells.as_slice(),
            BURSTS_1024[s].as_slice(),
            "size {}",
            SIZES[s]
        );
        assert!(
            cells.iter().all(|c| c.3 == 0),
            "no held last half without a quiet window"
        );
    }
}

/// The ticks the gate runs one assembly kicked: the span and three refractory windows, past
/// the longest delay of the local band after the latest kick spike.
const GATE_KICK_TICKS: u32 = KICK_SPAN + 3 * REFRACTORY_TICKS as u32;

const _: () = assert!(GATE_KICK_TICKS == 800);

/// The assembly of sixteen at the top weight, wired on the instrument's network at rest and
/// frozen, kicked at the first tick under ADR-0044's drive through the protocol's own kick
/// (`inject_before`, `note_kick_spikes`) and run for `GATE_KICK_TICKS`: every member fires once in
/// the span, sixteen resets are scheduled, no weight moves; the members' spikes as `(tick after
/// the start, unit)`.
fn an_assembly_kicked_for_a_few_hundred_ticks() -> Vec<(u32, u32)> {
    let image = prior_image();
    let mut exec = cell_engine(&image, 16, Some(i16::MAX));
    let set = members(16);
    let before = weights_of(&exec);
    let drive = drive(1024);
    let inject = exec.injector();
    let start = exec.ticks() as u32;
    let mut resets = Vec::new();
    for k in 0..GATE_KICK_TICKS {
        drive.step(&inject, exec.ticks()).expect("the drive runs");
        inject_before(&inject, &set, Epoch::Kicked, k, &resets);
        exec.tick();
        note_kick_spikes(&exec, &set, Epoch::Kicked, k, start, &mut resets);
    }
    assert_eq!(weights_of(&exec), before, "no weight moved");
    assert_eq!(resets.len(), set.len(), "every member's reset scheduled");
    let of = membership(&set);
    let spikes: Vec<(u32, u32)> = exec
        .train()
        .iter()
        .filter(|&&(_, unit)| of[unit as usize])
        .map(|&(tick, unit)| (tick.wrapping_sub(start), unit))
        .collect();
    for &m in &set {
        assert_eq!(
            spikes
                .iter()
                .filter(|&&(t, u)| u == m && (1..=KICK_SPAN).contains(&t))
                .count(),
            1,
            "member {m} fires once in the span"
        );
    }
    spikes
}

/// The rules over the kick's and the background's pinned tables: each run's twenty-five rows;
/// the background, the kick's reading and the controls' cells as the rules read them; the kick
/// firing every member once at every size; each control's lead-in and first epoch the
/// background's, the growth changing nothing unkicked; and the background run read three ways
/// one run, its members and rest summing to the same spikes in every window.
fn over_the_kick_tables() {
    for (k, &size) in SIZES.iter().enumerate() {
        let (bg, ctl) = (BACKGROUND_ROWS_1024[k], CONTROL_ROWS_1024[k]);
        assert_eq!((bg.len(), ctl.len()), (ROWS, ROWS));
        assert_eq!(background_of(bg), BACKGROUNDS_1024[k]);
        assert_eq!(kick_reading(ctl, size), KICKS_1024[k]);
        assert_eq!(kicked_once(ctl, size), KICKED_ONCE_1024[k]);
        assert!(
            KICKED_ONCE_1024[k],
            "the kick fires every member of {size} once"
        );
        assert_eq!(cell(ctl, BACKGROUNDS_1024[k]), CONTROLS_1024[k]);
        assert_eq!(ctl[..2], bg[..2], "the growth changes nothing unkicked");
        assert!(
            bg.iter().all(|(_, kick)| kick[0] <= 1),
            "no kick in the background"
        );
    }
    for row in 0..ROWS {
        for w in 0..WINDOWS {
            let totals: Vec<u32> = BACKGROUND_ROWS_1024
                .iter()
                .map(|rows| rows[row].0[w][0].saturating_add(rows[row].0[w][1]))
                .collect();
            assert!(totals.windows(2).all(|t| t[0] == t[1]), "one run");
        }
    }
}

/// An epoch's row written by hand: `members` and `rest` spikes in the first window of the
/// last half, `before` members' spikes in the window before the release, the kick's reading
/// `kick`.
fn hand_row(members: u32, rest: u32, before: u32, kick: [u32; 2]) -> EpochRow {
    let mut windows = [[0u32; 4]; WINDOWS];
    windows[HALF_FROM][0] = members;
    windows[HALF_FROM][1] = rest;
    windows[BEFORE_RELEASE][0] = before;
    (windows, kick)
}

/// A run written by hand: the lead-in quiet, every unkicked epoch's last half at `unkicked`
/// members' spikes, every kicked one's at `kicked` with `rest` of the rest's, every released
/// one's tail at `released` and its window before the release at `before`; each kick's
/// reading `kick`.
fn hand_run(unkicked: u32, kicked: u32, rest: u32, released: u32, before: u32) -> Vec<EpochRow> {
    (0..ROWS)
        .map(|row| match (row, kind_of(row)) {
            (0, _) => hand_row(0, 0, 0, [0; 2]),
            (_, Epoch::Unkicked) => hand_row(unkicked, 0, 0, [0; 2]),
            (_, Epoch::Kicked) => hand_row(kicked, rest, 0, [16, 1]),
            (_, Epoch::Released) => hand_row(released, 0, before, [16, 1]),
        })
        .collect()
}

/// The rows of `run` of kind `kind`, from the first after the lead-in.
fn rows_of(run: &mut [EpochRow], kind: Epoch) -> Vec<&mut EpochRow> {
    run.iter_mut()
        .enumerate()
        .skip(LEAD_IN_EPOCHS)
        .filter(|(row, _)| kind_of(*row) == kind)
        .map(|(_, r)| r)
        .collect()
}

fn rules_at_their_edges() {
    // A background of 96 members' spikes and 9 600 of the rest's over the twenty-four epochs:
    // two and two hundred in an epoch's last half, half a spike in one window.
    let bg = Background {
        members: 96,
        rest: 9_600,
        ticks: 24 * u64::from(EPOCH_TICKS),
    };
    let mut quiet_bg = hand_run(2, 2, 200, 2, 0);
    quiet_bg[0] = hand_row(7, 70, 0, [0; 2]);
    assert_eq!(
        background_of(&quiet_bg),
        Background {
            members: 48,
            rest: 1_600,
            ticks: bg.ticks
        },
        "the lead-in is not in a background"
    );
    // Holds at five times exactly (10 in a last half), lets go at twice exactly (4), ignites
    // at nothing below five times (9), spills at nothing above twice (400 of the rest's in a
    // held last half), and holds before the release at 3 in one window (2.5 is five times).
    let usable = hand_run(9, 10, 400, 4, 3);
    let read = cell(&usable, bg);
    assert_eq!(
        read,
        Cell {
            held: 8,
            ignited: 0,
            let_go: 8,
            before: 8,
            spills: Some(false)
        }
    );
    assert!(read.usable());
    assert_eq!(cell(&hand_run(9, 10, 400, 4, 2), bg).before, 0);
    // One LSB past each edge.
    let mut run = hand_run(9, 10, 400, 4, 3);
    rows_of(&mut run, Epoch::Kicked)[0].0[HALF_FROM][0] = 9;
    assert_eq!(cell(&run, bg).held, 7);
    assert!(cell(&run, bg).usable(), "seven of eight hold");
    rows_of(&mut run, Epoch::Kicked)[1].0[HALF_FROM][0] = 9;
    assert_eq!(cell(&run, bg).held, 6);
    assert!(!cell(&run, bg).holds() && !cell(&run, bg).usable());
    let mut run = hand_run(9, 10, 400, 4, 3);
    rows_of(&mut run, Epoch::Released)[0].0[HALF_FROM][0] = 5;
    assert_eq!(cell(&run, bg).let_go, 7);
    assert!(cell(&run, bg).usable());
    rows_of(&mut run, Epoch::Released)[7].0[HALF_FROM][0] = 5;
    assert!(!cell(&run, bg).lets_go() && !cell(&run, bg).usable());
    let mut run = hand_run(9, 10, 400, 4, 3);
    rows_of(&mut run, Epoch::Unkicked)[0].0[HALF_FROM][0] = 10;
    assert_eq!(cell(&run, bg).ignited, 1);
    assert!(cell(&run, bg).usable());
    rows_of(&mut run, Epoch::Unkicked)[4].0[HALF_FROM][0] = 10;
    assert!(!cell(&run, bg).quiet_unkicked() && !cell(&run, bg).usable());
    // Spills: the rest's spikes over the held last halves more than twice its background.
    let mut run = hand_run(9, 10, 400, 4, 3);
    rows_of(&mut run, Epoch::Kicked)[3].0[HALF_FROM][1] = 401;
    assert_eq!(cell(&run, bg).spills, Some(true));
    assert!(!cell(&run, bg).usable());
    // Read over the held epochs alone: a kicked epoch that did not hold is not read.
    let mut run = hand_run(9, 10, 400, 4, 3);
    let mut kicked = rows_of(&mut run, Epoch::Kicked);
    kicked[0].0[HALF_FROM] = [9, 100_000, 0, 0];
    assert_eq!(cell(&run, bg).spills, Some(false));
    // Nothing held: no spill is read, and the cell is not usable.
    let none = cell(&hand_run(0, 9, 100_000, 0, 0), bg);
    assert_eq!((none.held, none.spills, none.let_go), (0, None, 8));
    assert!(!none.usable());
    // The kick's measure: sixteen kicks of sixteen members; the volley at 14.0 a kick on
    // average passes and one spike fewer fails; every member twice fails; the after at 25
    // spikes (0.098 a member a kick) passes and 26 fails.
    let kicked_rows = |volley: [u32; 2], after_first: u32| -> Vec<EpochRow> {
        let mut run = hand_run(0, 0, 0, 0, 0);
        let mut first = true;
        for (row, r) in run.iter_mut().enumerate() {
            if kind_of(row) != Epoch::Unkicked {
                r.1 = [volley[usize::from(!first)], 0];
                if first {
                    r.1[1] = after_first;
                }
                first = false;
            }
        }
        run
    };
    assert_eq!(
        kick_reading(&kicked_rows([14, 14], 25), 16),
        (224, 25, 16, 0)
    );
    assert!(kicked_once(&kicked_rows([14, 14], 25), 16));
    assert!(!kicked_once(&kicked_rows([13, 14], 25), 16));
    assert!(!kicked_once(&kicked_rows([14, 14], 26), 16));
    assert!(kicked_once(&kicked_rows([16, 16], 0), 16));
    assert_eq!(kick_reading(&kicked_rows([16, 16], 0), 16).3, 16);
    assert!(!kicked_once(&kicked_rows([17, 16], 0), 16));
    assert!(!kicked_once(&hand_run(0, 0, 0, 0, 0)[..1], 16), "no kick");
}

/// The instrument's network at 1 024 units at rest at the gain 1.75, frozen, as an image.
fn prior_image() -> Vec<u8> {
    let exec = at_gain(&prior(1024), config(1024, 2, 0), GAIN_1024);
    assert!(exec.is_quiescent());
    Image::encode(&exec).expect("quiescent")
}

/// The kick, its reset included, into one unit at rest with no drive, by the oracle: the unit
/// stepped, and the tick of its spike.
fn kick_alone() -> (Stepped, Option<u32>) {
    kicked((0, 0), KICK_MESSAGE_Q16, Some(KICK_RESET_Q16), 0)
}

fn wiring_on_the_prior() {
    let image = prior_image();
    let plain = frozen_from(&image, 1024);
    let base = blocks_for(&prior(1024));
    // Grown, unwired: every record the image's, the blocks after it empty and in no chain.
    for &size in &SIZES {
        let grown_exec = frozen_from(&grown(&image, extra_blocks(size)), 1024);
        assert_eq!(
            grown_exec.blocks().len() as u64,
            base.saturating_add(extra_blocks(size))
        );
        assert_eq!(
            &grown_exec.blocks()[..base as usize],
            plain.blocks(),
            "the prior's blocks"
        );
        assert!(
            grown_exec.blocks()[base as usize..]
                .iter()
                .all(|b| *b == cortex_core::SynapseBlock::new()),
            "the appended blocks empty"
        );
        assert_eq!(grown_exec.ticks(), plain.ticks(), "the clock resumes alike");
    }
    // Wired: each member's fan-out is its own, then its assembly's in order; every other
    // unit's is its own.
    for &size in &SIZES {
        for &w in &WEIGHTS {
            let exec = cell_engine(&image, size, Some(w));
            let m = members(size);
            let of = membership(&m);
            for unit in exec.units() {
                let id = unit.id as u32;
                let own: Vec<_> = plain.units()[id as usize].fan_out(plain.blocks()).collect();
                let now: Vec<_> = unit.fan_out(exec.blocks()).collect();
                assert_eq!(&now[..own.len()], own.as_slice(), "unit {id}'s own");
                if !of[id as usize] {
                    assert_eq!(now.len(), own.len(), "unit {id} is no member");
                    continue;
                }
                let k = m.iter().position(|&u| u == id).expect("a member");
                let added = &now[own.len()..];
                assert_eq!(added.len() as u32, fan(size));
                for (d, s) in added.iter().enumerate() {
                    let target = m[k
                        .saturating_add(1)
                        .saturating_add(d)
                        .checked_rem(m.len())
                        .unwrap_or(0)];
                    assert_eq!(
                        (s.target, s.weight_q1_15, s.delay_ticks, s.apical),
                        (target, w, delay_of(id, target), false)
                    );
                    assert!((100..=300).contains(&s.delay_ticks));
                    assert!(s.block_idx as u64 >= base);
                }
            }
        }
    }
    // Every member receives as many synapses of its assembly as it sends.
    let exec = cell_engine(&image, 64, Some(i16::MAX));
    let m = members(64);
    let mut received = vec![0u32; 1024];
    for &u in &m {
        for s in exec.units()[u as usize].fan_out(exec.blocks()) {
            if s.block_idx as u64 >= base {
                received[s.target as usize] = received[s.target as usize].saturating_add(1);
            }
        }
    }
    assert!(m.iter().all(|&u| received[u as usize] == FAN_MAX));
    // The growth changes nothing: the grown image unwired runs as the image under the drive.
    let drive = drive(1024);
    let mut a = frozen_from(&image, 1024);
    let mut b = frozen_from(&grown(&image, extra_blocks(64)), 1024);
    let until = a.ticks().saturating_add(u64::from(GROWTH_TICKS));
    run_driven(&mut a, &drive, until).expect("the drive runs");
    run_driven(&mut b, &drive, until).expect("the drive runs");
    assert!(!a.train().is_empty(), "the network fired");
    assert_eq!(
        a.train(),
        b.train(),
        "the grown image unwired runs as the image"
    );
    // The kick on the engine: into member 5 of the network at rest with no drive, alone, it
    // fires on the tick the oracle says, once, and its basal potential after every tick is the
    // oracle's, the reset's tick among them.
    let (oracle, first) = kick_alone();
    let due = first
        .expect("the oracle fires the unit")
        .saturating_add(u32::from(REFRACTORY_TICKS));
    let mut exec = frozen_from(&image, 1024);
    let inject = exec.injector();
    let start = exec.ticks() as u32;
    let mut basal = Vec::new();
    for k in 0..KICK_SPAN.saturating_add(PAIR_WINDOW) {
        if k < KICK_TICKS {
            inject
                .inject(5, spike_message(KICK_MESSAGE_Q16, false))
                .expect("the ring has room");
        }
        if k == due {
            inject
                .inject(5, spike_message(KICK_RESET_Q16, false))
                .expect("the ring has room");
        }
        exec.tick();
        basal.push(exec.units()[5].v_basal);
    }
    let fires: Vec<u32> = exec
        .train()
        .iter()
        .filter(|&&(_, unit)| unit == 5)
        .map(|&(tick, _)| tick.wrapping_sub(start))
        .collect();
    // The engine's tick `start + k` is the oracle's tick `k`: the oracle's first input lands on
    // its tick one, the engine's on `start + 1`.
    assert_eq!(fires, oracle.fires, "the engine fires as the oracle");
    assert_eq!(fires.len(), 1);
    assert_eq!(
        basal[1..],
        oracle.basal[..oracle.basal.len().saturating_sub(1)],
        "the engine's basal potential is the oracle's"
    );
}

/// The ticks the growth's check runs under the drive.
const GROWTH_TICKS: u32 = 4_096;

// ------------------------------------------------ the runs, weekly

/// The kick and the background (brief 048, the order ADR-0112 writes): on ADR-0077's settled
/// image frozen, the background — the protocol's ticks under the drive with no kick, read for
/// each size's members — and each size's control — the grown image unwired, the kick and the
/// release into the members — before any cell is run. The control's lead-in and first epoch
/// are the background's bit for bit, since the growth changes nothing, and the kick fires every
/// member once by the measure, or no cell is run.
#[test]
#[ignore]
fn the_kick_and_the_background_at_1024_units_exhaustive() {
    let name = "the kick and the background";
    let image = settled(name);
    let reads: Vec<Vec<u32>> = SIZES.iter().map(|&s| members(s)).collect();
    let mut exec = frozen_from(&image, 1024);
    let before = weights_of(&exec);
    let backgrounds = protocol(&mut exec, &reads, None);
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    for (k, rows) in backgrounds.iter().enumerate() {
        eprintln!("DUMP BACKGROUND_ROWS_1024[{k}] = {rows:?}");
    }
    let controls: Vec<Vec<EpochRow>> = SIZES
        .iter()
        .enumerate()
        .map(|(k, &size)| cell_run(&image, size, None, &format!("CONTROL_ROWS_1024[{k}]")))
        .collect();
    let mut readings = Vec::new();
    for (k, &size) in SIZES.iter().enumerate() {
        let bg = background_of(&backgrounds[k]);
        let kick = kick_reading(&controls[k], size);
        let once = kicked_once(&controls[k], size);
        let read = cell(&controls[k], bg);
        eprintln!(
            "DUMP {name} {size}: background {bg:?} kick {kick:?} once {once} control {read:?}"
        );
        readings.push((bg, kick, once, read));
    }
    for (k, &size) in SIZES.iter().enumerate() {
        assert_eq!(
            controls[k][..2],
            backgrounds[k][..2],
            "{name} {size}: the growth changes nothing unkicked"
        );
    }
    for (k, &size) in SIZES.iter().enumerate() {
        assert!(
            readings[k].2,
            "{name} {size}: the kick fires every member once, or it is derived again"
        );
    }
    for (k, rows) in backgrounds.iter().enumerate() {
        assert_eq!(
            rows.as_slice(),
            BACKGROUND_ROWS_1024[k],
            "the background {k}"
        );
    }
    for (k, rows) in controls.iter().enumerate() {
        assert_eq!(rows.as_slice(), CONTROL_ROWS_1024[k], "the control {k}");
    }
    for (k, (bg, kick, once, read)) in readings.iter().enumerate() {
        assert_eq!(*bg, BACKGROUNDS_1024[k]);
        assert_eq!(*kick, KICKS_1024[k]);
        assert_eq!(*once, KICKED_ONCE_1024[k]);
        assert_eq!(*read, CONTROLS_1024[k]);
    }
}

/// The four weights of one size (brief 048): on ADR-0077's settled image frozen, grown by the
/// assembly's blocks and wired at each weight in `WEIGHTS`' order, the protocol, the weights
/// shown unchanged at each run's end; each run read against the background pinned before any
/// cell ran, and every run dumped before any is held to its table.
fn four_weights(s: usize) {
    let size = SIZES[s];
    let name = format!("the assembly of {size}");
    let image = settled(&name);
    let bg = background_of(BACKGROUND_ROWS_1024[s]);
    assert_eq!(
        bg, BACKGROUNDS_1024[s],
        "{name}: the background, pinned first"
    );
    let mut runs = Vec::new();
    for (k, &w) in WEIGHTS.iter().enumerate() {
        let rows = cell_run(&image, size, Some(w), &format!("CELL_ROWS_1024[{s}][{k}]"));
        let read = cell(&rows, bg);
        eprintln!(
            "DUMP {name} at {w:#x}: {read:?} holds {} lets go {} quiet {} usable {} kick {:?}",
            read.holds(),
            read.lets_go(),
            read.quiet_unkicked(),
            read.usable(),
            kick_reading(&rows, size)
        );
        runs.push((rows, read));
    }
    for (k, (rows, read)) in runs.iter().enumerate() {
        assert_eq!(rows.as_slice(), CELL_ROWS_1024[s][k], "{name}: weight {k}");
        assert_eq!(*read, GRID_1024[s][k], "{name}: weight {k}");
    }
}

#[test]
#[ignore]
fn an_assembly_of_16_units_at_four_weights_exhaustive() {
    four_weights(0);
}

#[test]
#[ignore]
fn an_assembly_of_32_units_at_four_weights_exhaustive() {
    four_weights(1);
}

#[test]
#[ignore]
fn an_assembly_of_64_units_at_four_weights_exhaustive() {
    four_weights(2);
}

// ------------------------------------------------ the readings, pinned from the runs

/// The burst reading (`bursts`) of each size's control and of its cells in `WEIGHTS`' order.
const BURSTS_1024: [[BurstRead; 5]; 3] = [
    [
        (14, 0, 0, 0, None, 102),
        (16, 0, 0, 0, None, 88),
        (16, 0, 0, 0, None, 27),
        (19, 3, 0, 0, None, 19),
        (25, 9, 1, 0, Some(3), 14),
    ],
    [
        (15, 0, 0, 0, None, 104),
        (18, 2, 0, 0, None, 27),
        (21, 5, 1, 0, Some(3), 14),
        (36, 20, 3, 0, Some(2), 13),
        (42, 28, 7, 0, Some(2), 10),
    ],
    [
        (15, 0, 0, 0, None, 103),
        (16, 0, 0, 0, None, 24),
        (23, 7, 0, 0, None, 14),
        (37, 22, 7, 0, Some(2), 11),
        (48, 32, 7, 0, Some(1), 9),
    ],
];

/// The members' spikes, `(tick after the start, unit)`, of the assembly of sixteen at the top
/// weight kicked on the instrument's network at rest for `GATE_KICK_TICKS`.
const GATE_KICKED: &[(u32, u32)] = &[
    (78, 5),
    (78, 25),
    (78, 56),
    (78, 85),
    (78, 105),
    (79, 116),
    (85, 16),
    (86, 125),
    (87, 36),
    (87, 45),
    (87, 65),
    (87, 76),
    (87, 96),
    (87, 136),
    (87, 145),
    (87, 156),
    (288, 116),
    (296, 5),
    (301, 156),
    (302, 36),
    (310, 125),
    (313, 65),
    (315, 16),
    (315, 96),
    (315, 105),
    (321, 85),
    (328, 45),
    (329, 25),
    (337, 145),
    (338, 76),
    (346, 56),
    (347, 136),
    (494, 116),
    (510, 5),
    (516, 96),
    (519, 125),
    (529, 45),
    (530, 25),
    (532, 36),
    (536, 16),
    (543, 156),
    (546, 65),
    (546, 76),
    (558, 105),
    (559, 145),
    (560, 56),
    (566, 136),
    (570, 85),
    (716, 116),
    (722, 125),
    (739, 96),
    (744, 5),
    (747, 36),
    (747, 156),
    (758, 25),
    (759, 76),
    (767, 16),
    (780, 145),
    (785, 105),
    (786, 56),
    (796, 65),
];

/// The background run's rows, read for each size's members.
const BACKGROUND_ROWS_1024: [&[EpochRow]; 3] = [
    &[
        (
            [
                [0, 14, 1578, 3492],
                [0, 19, 1564, 3525],
                [2, 31, 1611, 3399],
                [0, 25, 1597, 3441],
                [0, 32, 1582, 3481],
                [2, 52, 1617, 3385],
                [0, 29, 1602, 3428],
                [1, 63, 1617, 3379],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 1599, 3423],
                [1, 40, 1614, 3373],
                [3, 51, 1681, 3176],
                [1, 58, 1681, 3163],
                [0, 43, 1659, 3220],
                [0, 36, 1636, 3270],
                [0, 37, 1618, 3320],
                [2, 23, 1658, 3204],
            ],
            [0, 0],
        ),
        (
            [
                [1, 32, 1665, 3180],
                [0, 39, 1641, 3235],
                [0, 24, 1621, 3287],
                [1, 33, 1636, 3247],
                [0, 46, 1615, 3297],
                [0, 35, 1600, 3347],
                [0, 38, 1585, 3391],
                [3, 40, 1661, 3202],
            ],
            [1, 0],
        ),
        (
            [
                [0, 25, 1638, 3256],
                [1, 25, 1643, 3240],
                [1, 45, 1655, 3200],
                [1, 30, 1665, 3167],
                [0, 46, 1637, 3223],
                [0, 41, 1619, 3273],
                [1, 29, 1628, 3243],
                [0, 31, 1610, 3296],
            ],
            [0, 0],
        ),
        (
            [
                [0, 36, 1593, 3344],
                [0, 29, 1577, 3387],
                [0, 42, 1567, 3431],
                [1, 28, 1586, 3381],
                [0, 32, 1573, 3423],
                [2, 43, 1615, 3307],
                [1, 24, 1626, 3279],
                [1, 36, 1636, 3251],
            ],
            [0, 0],
        ),
        (
            [
                [1, 38, 1645, 3223],
                [2, 32, 1687, 3107],
                [1, 50, 1690, 3095],
                [0, 21, 1665, 3153],
                [0, 42, 1642, 3211],
                [0, 29, 1622, 3263],
                [1, 27, 1632, 3237],
                [3, 43, 1692, 3085],
            ],
            [0, 1],
        ),
        (
            [
                [0, 50, 1668, 3145],
                [2, 40, 1701, 3035],
                [0, 38, 1675, 3099],
                [0, 35, 1650, 3158],
                [1, 39, 1658, 3136],
                [0, 36, 1634, 3195],
                [1, 51, 1644, 3171],
                [1, 48, 1652, 3148],
            ],
            [0, 0],
        ),
        (
            [
                [0, 48, 1629, 3207],
                [0, 50, 1612, 3261],
                [0, 33, 1596, 3309],
                [0, 24, 1578, 3356],
                [1, 32, 1592, 3335],
                [0, 35, 1577, 3381],
                [0, 38, 1564, 3425],
                [0, 31, 1552, 3462],
            ],
            [0, 0],
        ),
        (
            [
                [0, 37, 1543, 3499],
                [0, 40, 1535, 3537],
                [0, 39, 1528, 3567],
                [0, 24, 1522, 3600],
                [2, 40, 1576, 3465],
                [1, 51, 1595, 3425],
                [0, 25, 1581, 3465],
                [0, 17, 1566, 3502],
            ],
            [0, 0],
        ),
        (
            [
                [0, 30, 1556, 3537],
                [0, 44, 1547, 3569],
                [0, 31, 1538, 3601],
                [0, 28, 1530, 3630],
                [2, 36, 1580, 3496],
                [0, 46, 1566, 3530],
                [0, 39, 1555, 3564],
                [0, 49, 1544, 3596],
            ],
            [0, 0],
        ),
        (
            [
                [0, 32, 1535, 3626],
                [1, 40, 1556, 3585],
                [0, 44, 1546, 3613],
                [0, 33, 1538, 3640],
                [0, 40, 1530, 3671],
                [0, 38, 1523, 3693],
                [1, 29, 1547, 3631],
                [0, 37, 1536, 3658],
            ],
            [0, 0],
        ),
        (
            [
                [1, 38, 1559, 3596],
                [0, 41, 1548, 3625],
                [0, 22, 1539, 3654],
                [2, 37, 1596, 3507],
                [0, 32, 1579, 3543],
                [0, 21, 1566, 3577],
                [1, 30, 1581, 3539],
                [1, 41, 1599, 3489],
            ],
            [0, 1],
        ),
        (
            [
                [1, 46, 1615, 3440],
                [0, 31, 1597, 3478],
                [0, 44, 1582, 3514],
                [0, 40, 1571, 3548],
                [0, 31, 1557, 3581],
                [1, 28, 1579, 3521],
                [0, 32, 1567, 3557],
                [0, 39, 1557, 3586],
            ],
            [0, 1],
        ),
        (
            [
                [0, 35, 1545, 3618],
                [1, 49, 1566, 3557],
                [0, 33, 1557, 3588],
                [1, 30, 1578, 3532],
                [0, 42, 1565, 3565],
                [0, 25, 1551, 3593],
                [0, 42, 1545, 3627],
                [0, 37, 1535, 3654],
            ],
            [0, 0],
        ),
        (
            [
                [2, 34, 1588, 3508],
                [1, 37, 1607, 3454],
                [0, 46, 1589, 3492],
                [1, 42, 1602, 3447],
                [2, 33, 1641, 3340],
                [0, 33, 1621, 3384],
                [0, 37, 1603, 3425],
                [1, 38, 1615, 3382],
            ],
            [1, 1],
        ),
        (
            [
                [0, 33, 1601, 3426],
                [1, 43, 1615, 3383],
                [1, 27, 1624, 3364],
                [1, 36, 1637, 3321],
                [0, 18, 1616, 3366],
                [1, 36, 1632, 3324],
                [0, 44, 1613, 3373],
                [0, 31, 1596, 3414],
            ],
            [0, 0],
        ),
        (
            [
                [2, 29, 1638, 3300],
                [0, 46, 1622, 3346],
                [0, 56, 1604, 3391],
                [0, 55, 1587, 3432],
                [0, 32, 1571, 3474],
                [1, 43, 1594, 3429],
                [0, 26, 1577, 3469],
                [2, 41, 1622, 3331],
            ],
            [0, 2],
        ),
        (
            [
                [1, 43, 1629, 3299],
                [0, 34, 1611, 3347],
                [1, 29, 1626, 3307],
                [0, 50, 1607, 3356],
                [0, 30, 1593, 3400],
                [0, 41, 1576, 3442],
                [0, 37, 1563, 3479],
                [0, 28, 1551, 3515],
            ],
            [0, 1],
        ),
        (
            [
                [0, 33, 1543, 3550],
                [2, 35, 1595, 3425],
                [2, 38, 1642, 3301],
                [1, 30, 1652, 3266],
                [0, 38, 1632, 3313],
                [0, 19, 1613, 3361],
                [1, 39, 1627, 3323],
                [0, 49, 1612, 3369],
            ],
            [0, 0],
        ),
        (
            [
                [1, 37, 1622, 3338],
                [2, 40, 1660, 3252],
                [0, 26, 1636, 3304],
                [1, 38, 1646, 3268],
                [1, 44, 1655, 3241],
                [1, 43, 1659, 3226],
                [2, 48, 1693, 3120],
                [0, 48, 1667, 3178],
            ],
            [0, 1],
        ),
        (
            [
                [0, 43, 1644, 3233],
                [0, 36, 1624, 3286],
                [0, 30, 1606, 3333],
                [0, 22, 1589, 3379],
                [0, 44, 1575, 3420],
                [0, 34, 1565, 3460],
                [0, 25, 1553, 3497],
                [0, 35, 1541, 3535],
            ],
            [0, 0],
        ),
        (
            [
                [1, 31, 1561, 3491],
                [1, 21, 1578, 3442],
                [2, 32, 1624, 3317],
                [0, 40, 1605, 3362],
                [0, 43, 1590, 3406],
                [1, 43, 1607, 3371],
                [0, 28, 1590, 3415],
                [1, 35, 1605, 3375],
            ],
            [0, 1],
        ),
        (
            [
                [2, 28, 1647, 3264],
                [0, 25, 1627, 3314],
                [1, 31, 1633, 3291],
                [2, 38, 1670, 3178],
                [0, 40, 1646, 3230],
                [1, 32, 1652, 3200],
                [1, 22, 1661, 3169],
                [1, 38, 1670, 3139],
            ],
            [0, 2],
        ),
        (
            [
                [1, 29, 1681, 3117],
                [0, 29, 1654, 3174],
                [1, 27, 1659, 3152],
                [2, 31, 1697, 3055],
                [1, 41, 1699, 3032],
                [1, 43, 1699, 3020],
                [0, 48, 1672, 3082],
                [0, 39, 1649, 3143],
            ],
            [0, 1],
        ),
        (
            [
                [0, 34, 1628, 3200],
                [2, 27, 1672, 3089],
                [0, 31, 1647, 3150],
                [0, 38, 1625, 3205],
                [0, 29, 1606, 3259],
                [0, 40, 1590, 3311],
                [1, 52, 1608, 3277],
                [2, 50, 1648, 3177],
            ],
            [0, 0],
        ),
    ],
    &[
        (
            [
                [0, 14, 3131, 7094],
                [0, 19, 3108, 7154],
                [3, 30, 3176, 6984],
                [0, 25, 3150, 7056],
                [0, 32, 3124, 7125],
                [3, 51, 3183, 6966],
                [0, 29, 3157, 7040],
                [3, 61, 3223, 6845],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 3189, 6925],
                [1, 40, 3190, 6910],
                [4, 50, 3278, 6663],
                [1, 58, 3261, 6687],
                [1, 42, 3256, 6707],
                [0, 36, 3219, 6794],
                [0, 37, 3187, 6876],
                [3, 22, 3244, 6718],
            ],
            [0, 0],
        ),
        (
            [
                [3, 30, 3295, 6575],
                [0, 39, 3253, 6671],
                [0, 24, 3216, 6761],
                [1, 33, 3214, 6758],
                [0, 46, 3181, 6844],
                [0, 35, 3155, 6927],
                [2, 36, 3191, 6832],
                [4, 39, 3281, 6596],
            ],
            [1, 2],
        ),
        (
            [
                [0, 25, 3242, 6690],
                [1, 25, 3229, 6713],
                [1, 45, 3227, 6708],
                [3, 28, 3288, 6547],
                [1, 45, 3271, 6558],
                [1, 40, 3264, 6572],
                [1, 29, 3253, 6588],
                [0, 31, 3218, 6686],
            ],
            [0, 0],
        ),
        (
            [
                [2, 34, 3239, 6628],
                [0, 29, 3203, 6718],
                [1, 41, 3204, 6736],
                [1, 28, 3203, 6735],
                [0, 32, 3172, 6822],
                [2, 43, 3200, 6747],
                [1, 24, 3200, 6760],
                [1, 36, 3196, 6767],
            ],
            [0, 2],
        ),
        (
            [
                [3, 36, 3251, 6625],
                [3, 31, 3305, 6463],
                [3, 48, 3347, 6328],
                [0, 21, 3300, 6437],
                [0, 42, 3258, 6543],
                [0, 29, 3218, 6641],
                [1, 27, 3214, 6658],
                [5, 41, 3322, 6375],
            ],
            [0, 4],
        ),
        (
            [
                [1, 49, 3306, 6411],
                [2, 40, 3321, 6350],
                [0, 38, 3276, 6462],
                [0, 35, 3237, 6564],
                [1, 39, 3234, 6584],
                [1, 35, 3222, 6614],
                [1, 51, 3219, 6631],
                [3, 46, 3275, 6472],
            ],
            [0, 1],
        ),
        (
            [
                [0, 48, 3233, 6575],
                [2, 48, 3255, 6541],
                [0, 33, 3217, 6638],
                [0, 24, 3180, 6731],
                [1, 32, 3179, 6752],
                [1, 34, 3177, 6763],
                [1, 37, 3182, 6761],
                [0, 31, 3149, 6846],
            ],
            [0, 1],
        ),
        (
            [
                [0, 37, 3123, 6927],
                [0, 40, 3103, 7003],
                [0, 39, 3085, 7071],
                [0, 24, 3069, 7138],
                [3, 39, 3145, 6950],
                [3, 49, 3199, 6829],
                [1, 24, 3198, 6827],
                [0, 17, 3168, 6908],
            ],
            [0, 0],
        ),
        (
            [
                [0, 30, 3141, 6984],
                [0, 44, 3120, 7054],
                [0, 31, 3099, 7122],
                [0, 28, 3081, 7184],
                [2, 36, 3119, 7083],
                [2, 44, 3153, 6991],
                [0, 39, 3129, 7062],
                [1, 48, 3137, 7039],
            ],
            [0, 0],
        ),
        (
            [
                [1, 31, 3138, 7032],
                [1, 40, 3142, 7032],
                [0, 44, 3117, 7096],
                [1, 32, 3128, 7077],
                [0, 40, 3105, 7146],
                [0, 38, 3086, 7206],
                [1, 29, 3101, 7178],
                [0, 37, 3080, 7238],
            ],
            [0, 1],
        ),
        (
            [
                [2, 37, 3123, 7127],
                [0, 41, 3101, 7193],
                [0, 22, 3081, 7252],
                [2, 37, 3131, 7134],
                [0, 32, 3107, 7197],
                [0, 21, 3089, 7258],
                [1, 30, 3097, 7243],
                [2, 40, 3140, 7132],
            ],
            [0, 2],
        ),
        (
            [
                [4, 43, 3239, 6858],
                [1, 30, 3234, 6848],
                [0, 44, 3200, 6926],
                [1, 39, 3197, 6933],
                [2, 29, 3225, 6835],
                [1, 28, 3225, 6827],
                [0, 32, 3192, 6910],
                [1, 38, 3196, 6893],
            ],
            [0, 4],
        ),
        (
            [
                [0, 35, 3163, 6973],
                [1, 49, 3168, 6954],
                [1, 32, 3174, 6947],
                [2, 29, 3206, 6845],
                [0, 42, 3177, 6924],
                [0, 25, 3143, 6997],
                [1, 41, 3158, 6983],
                [0, 37, 3130, 7050],
            ],
            [0, 0],
        ),
        (
            [
                [5, 31, 3253, 6713],
                [2, 36, 3279, 6624],
                [3, 43, 3324, 6486],
                [2, 41, 3334, 6427],
                [3, 32, 3367, 6319],
                [1, 32, 3349, 6347],
                [0, 37, 3301, 6455],
                [1, 38, 3283, 6476],
            ],
            [1, 5],
        ),
        (
            [
                [0, 33, 3246, 6580],
                [2, 42, 3270, 6510],
                [1, 27, 3258, 6550],
                [1, 36, 3252, 6561],
                [0, 18, 3213, 6659],
                [4, 33, 3296, 6432],
                [1, 43, 3284, 6463],
                [0, 31, 3244, 6561],
            ],
            [0, 0],
        ),
        (
            [
                [2, 29, 3266, 6505],
                [1, 45, 3261, 6517],
                [1, 55, 3251, 6537],
                [0, 55, 3213, 6632],
                [1, 31, 3210, 6647],
                [3, 41, 3271, 6495],
                [0, 26, 3229, 6597],
                [2, 41, 3252, 6518],
            ],
            [0, 2],
        ),
        (
            [
                [1, 43, 3238, 6541],
                [1, 33, 3234, 6564],
                [1, 29, 3229, 6577],
                [1, 49, 3222, 6601],
                [0, 30, 3191, 6697],
                [0, 41, 3160, 6784],
                [0, 37, 3131, 6868],
                [1, 27, 3139, 6865],
            ],
            [0, 1],
        ),
        (
            [
                [0, 33, 3116, 6944],
                [3, 34, 3182, 6794],
                [3, 37, 3247, 6634],
                [2, 29, 3269, 6569],
                [1, 37, 3254, 6602],
                [0, 19, 3216, 6697],
                [1, 39, 3214, 6705],
                [0, 49, 3186, 6794],
            ],
            [0, 0],
        ),
        (
            [
                [1, 37, 3183, 6803],
                [2, 40, 3210, 6753],
                [0, 26, 3178, 6841],
                [1, 38, 3178, 6838],
                [1, 44, 3182, 6843],
                [1, 43, 3179, 6855],
                [2, 48, 3207, 6779],
                [0, 48, 3177, 6861],
            ],
            [0, 1],
        ),
        (
            [
                [2, 41, 3209, 6781],
                [1, 35, 3203, 6797],
                [2, 28, 3234, 6707],
                [1, 21, 3228, 6712],
                [0, 44, 3195, 6800],
                [0, 34, 3167, 6884],
                [1, 24, 3166, 6883],
                [1, 34, 3165, 6882],
            ],
            [0, 2],
        ),
        (
            [
                [1, 31, 3169, 6884],
                [2, 20, 3197, 6801],
                [2, 32, 3227, 6720],
                [1, 39, 3220, 6718],
                [0, 43, 3189, 6805],
                [2, 42, 3219, 6752],
                [2, 26, 3240, 6672],
                [2, 34, 3261, 6595],
            ],
            [0, 1],
        ),
        (
            [
                [3, 27, 3310, 6465],
                [0, 25, 3268, 6569],
                [4, 28, 3341, 6385],
                [2, 38, 3349, 6331],
                [0, 40, 3297, 6439],
                [2, 31, 3309, 6404],
                [1, 22, 3296, 6424],
                [2, 37, 3314, 6366],
            ],
            [0, 3],
        ),
        (
            [
                [1, 29, 3304, 6395],
                [1, 28, 3292, 6417],
                [2, 26, 3310, 6358],
                [3, 30, 3359, 6230],
                [2, 40, 3365, 6177],
                [1, 43, 3343, 6223],
                [1, 47, 3328, 6250],
                [0, 39, 3283, 6367],
            ],
            [0, 1],
        ),
        (
            [
                [1, 33, 3271, 6390],
                [2, 27, 3295, 6334],
                [0, 31, 3250, 6446],
                [1, 37, 3240, 6481],
                [0, 29, 3204, 6584],
                [0, 40, 3173, 6682],
                [2, 51, 3210, 6611],
                [5, 47, 3318, 6333],
            ],
            [0, 1],
        ),
    ],
    &[
        (
            [
                [1, 13, 6297, 14133],
                [0, 19, 6245, 14261],
                [4, 29, 6324, 14065],
                [1, 24, 6305, 14123],
                [0, 32, 6253, 14259],
                [5, 49, 6350, 14011],
                [0, 29, 6295, 14151],
                [6, 58, 6424, 13787],
            ],
            [0, 1],
        ),
        (
            [
                [0, 35, 6358, 13940],
                [2, 39, 6357, 13929],
                [7, 47, 6509, 13501],
                [3, 56, 6519, 13439],
                [3, 40, 6535, 13380],
                [1, 35, 6484, 13476],
                [3, 34, 6496, 13425],
                [3, 22, 6507, 13362],
            ],
            [0, 0],
        ),
        (
            [
                [5, 28, 6581, 13146],
                [1, 38, 6531, 13262],
                [1, 23, 6486, 13354],
                [2, 32, 6471, 13371],
                [2, 44, 6460, 13381],
                [1, 34, 6425, 13495],
                [4, 34, 6485, 13333],
                [6, 37, 6596, 13023],
            ],
            [1, 4],
        ),
        (
            [
                [3, 22, 6598, 13026],
                [3, 23, 6583, 13025],
                [1, 45, 6531, 13128],
                [3, 28, 6551, 13072],
                [1, 45, 6497, 13183],
                [1, 40, 6455, 13291],
                [1, 29, 6414, 13395],
                [2, 29, 6416, 13403],
            ],
            [0, 3],
        ),
        (
            [
                [2, 34, 6404, 13430],
                [0, 29, 6344, 13603],
                [3, 39, 6378, 13549],
                [1, 28, 6349, 13630],
                [1, 31, 6324, 13717],
                [3, 42, 6357, 13642],
                [1, 24, 6331, 13732],
                [3, 34, 6369, 13650],
            ],
            [0, 2],
        ),
        (
            [
                [3, 36, 6397, 13585],
                [3, 31, 6427, 13494],
                [5, 46, 6504, 13270],
                [1, 20, 6466, 13361],
                [1, 41, 6418, 13491],
                [2, 27, 6416, 13502],
                [2, 26, 6402, 13551],
                [5, 41, 6483, 13341],
            ],
            [0, 4],
        ),
        (
            [
                [3, 47, 6493, 13306],
                [5, 37, 6567, 13085],
                [2, 36, 6540, 13122],
                [2, 33, 6524, 13155],
                [3, 37, 6538, 13115],
                [2, 34, 6508, 13178],
                [4, 48, 6553, 13059],
                [6, 43, 6650, 12750],
            ],
            [0, 3],
        ),
        (
            [
                [1, 47, 6584, 12884],
                [3, 47, 6588, 12870],
                [0, 33, 6508, 13080],
                [0, 24, 6430, 13276],
                [3, 30, 6451, 13234],
                [1, 34, 6409, 13349],
                [1, 37, 6383, 13444],
                [3, 28, 6400, 13391],
            ],
            [1, 1],
        ),
        (
            [
                [1, 36, 6369, 13481],
                [1, 39, 6342, 13567],
                [1, 38, 6318, 13655],
                [0, 24, 6271, 13818],
                [6, 36, 6408, 13480],
                [5, 47, 6479, 13302],
                [1, 24, 6438, 13405],
                [3, 14, 6454, 13376],
            ],
            [0, 1],
        ),
        (
            [
                [0, 30, 6385, 13554],
                [1, 43, 6354, 13652],
                [0, 31, 6295, 13814],
                [1, 27, 6278, 13898],
                [3, 35, 6307, 13823],
                [6, 40, 6425, 13497],
                [0, 39, 6361, 13668],
                [4, 45, 6412, 13538],
            ],
            [0, 0],
        ),
        (
            [
                [2, 30, 6406, 13549],
                [2, 39, 6404, 13563],
                [1, 43, 6371, 13642],
                [2, 31, 6377, 13636],
                [0, 40, 6318, 13805],
                [1, 37, 6296, 13872],
                [3, 27, 6340, 13778],
                [2, 35, 6345, 13784],
            ],
            [0, 2],
        ),
        (
            [
                [4, 35, 6403, 13635],
                [3, 38, 6417, 13606],
                [1, 21, 6386, 13682],
                [5, 34, 6473, 13460],
                [0, 32, 6403, 13636],
                [0, 21, 6340, 13803],
                [1, 30, 6311, 13885],
                [5, 37, 6411, 13615],
            ],
            [0, 4],
        ),
        (
            [
                [5, 42, 6504, 13357],
                [1, 30, 6458, 13448],
                [0, 44, 6394, 13621],
                [3, 37, 6418, 13555],
                [3, 28, 6441, 13471],
                [2, 27, 6435, 13496],
                [1, 31, 6402, 13591],
                [2, 37, 6406, 13580],
            ],
            [0, 5],
        ),
        (
            [
                [1, 34, 6372, 13662],
                [2, 48, 6376, 13644],
                [1, 32, 6351, 13726],
                [4, 27, 6412, 13541],
                [0, 42, 6353, 13707],
                [0, 25, 6289, 13866],
                [3, 39, 6341, 13783],
                [1, 36, 6311, 13856],
            ],
            [0, 1],
        ),
        (
            [
                [6, 30, 6435, 13527],
                [4, 34, 6488, 13378],
                [4, 42, 6522, 13289],
                [4, 39, 6557, 13156],
                [3, 32, 6560, 13138],
                [2, 31, 6534, 13180],
                [1, 36, 6491, 13282],
                [2, 37, 6470, 13310],
            ],
            [1, 6],
        ),
        (
            [
                [1, 32, 6434, 13413],
                [4, 40, 6492, 13260],
                [2, 26, 6475, 13305],
                [3, 34, 6495, 13230],
                [0, 18, 6417, 13420],
                [4, 33, 6473, 13277],
                [2, 42, 6461, 13312],
                [1, 30, 6422, 13411],
            ],
            [0, 1],
        ),
        (
            [
                [3, 28, 6445, 13354],
                [5, 41, 6527, 13147],
                [2, 54, 6510, 13169],
                [2, 53, 6495, 13187],
                [2, 30, 6481, 13225],
                [3, 41, 6503, 13167],
                [0, 26, 6423, 13358],
                [4, 39, 6475, 13224],
            ],
            [0, 3],
        ),
        (
            [
                [3, 41, 6491, 13173],
                [5, 29, 6564, 12984],
                [2, 28, 6542, 13027],
                [1, 49, 6489, 13153],
                [1, 29, 6453, 13264],
                [2, 39, 6436, 13310],
                [3, 34, 6443, 13315],
                [3, 25, 6465, 13247],
            ],
            [0, 4],
        ),
        (
            [
                [2, 31, 6444, 13348],
                [5, 32, 6523, 13138],
                [4, 36, 6573, 13008],
                [2, 29, 6546, 13051],
                [2, 36, 6523, 13106],
                [0, 19, 6445, 13302],
                [2, 38, 6442, 13315],
                [5, 44, 6514, 13131],
            ],
            [0, 2],
        ),
        (
            [
                [2, 36, 6490, 13191],
                [2, 40, 6477, 13246],
                [2, 24, 6456, 13297],
                [3, 36, 6480, 13231],
                [1, 44, 6440, 13341],
                [2, 42, 6429, 13377],
                [3, 47, 6455, 13315],
                [2, 46, 6440, 13365],
            ],
            [0, 2],
        ),
        (
            [
                [3, 40, 6464, 13305],
                [2, 34, 6453, 13352],
                [2, 28, 6448, 13361],
                [1, 21, 6410, 13458],
                [1, 43, 6374, 13559],
                [1, 33, 6346, 13651],
                [2, 23, 6344, 13657],
                [3, 32, 6381, 13569],
            ],
            [0, 3],
        ),
        (
            [
                [2, 30, 6382, 13581],
                [5, 17, 6470, 13347],
                [2, 32, 6461, 13363],
                [3, 37, 6478, 13296],
                [2, 41, 6462, 13319],
                [3, 41, 6487, 13276],
                [2, 26, 6468, 13297],
                [2, 34, 6455, 13313],
            ],
            [0, 2],
        ),
        (
            [
                [4, 26, 6505, 13184],
                [0, 25, 6436, 13376],
                [5, 27, 6509, 13187],
                [3, 37, 6520, 13136],
                [3, 37, 6526, 13096],
                [3, 30, 6540, 13076],
                [1, 22, 6490, 13187],
                [2, 37, 6477, 13214],
            ],
            [0, 4],
        ),
        (
            [
                [1, 29, 6443, 13323],
                [1, 28, 6407, 13418],
                [2, 26, 6406, 13432],
                [3, 30, 6436, 13371],
                [5, 37, 6511, 13149],
                [3, 41, 6527, 13104],
                [3, 45, 6537, 13061],
                [0, 39, 6463, 13260],
            ],
            [0, 1],
        ),
        (
            [
                [1, 33, 6423, 13362],
                [3, 26, 6447, 13300],
                [0, 31, 6376, 13488],
                [1, 37, 6347, 13593],
                [1, 28, 6323, 13673],
                [0, 40, 6272, 13835],
                [3, 50, 6317, 13753],
                [5, 47, 6406, 13537],
            ],
            [0, 1],
        ),
    ],
];

/// Each size's control's rows: the grown image unwired, the kick and the release.
const CONTROL_ROWS_1024: [&[EpochRow]; 3] = [
    &[
        (
            [
                [0, 14, 1578, 3492],
                [0, 19, 1564, 3525],
                [2, 31, 1611, 3399],
                [0, 25, 1597, 3441],
                [0, 32, 1582, 3481],
                [2, 52, 1617, 3385],
                [0, 29, 1602, 3428],
                [1, 63, 1617, 3379],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 1599, 3423],
                [1, 40, 1614, 3373],
                [3, 51, 1681, 3176],
                [1, 58, 1681, 3163],
                [0, 43, 1659, 3220],
                [0, 36, 1636, 3270],
                [0, 37, 1618, 3320],
                [2, 23, 1658, 3204],
            ],
            [0, 0],
        ),
        (
            [
                [15, 46, 2043, 2132],
                [0, 41, 1973, 2249],
                [0, 28, 1914, 2361],
                [1, 35, 1893, 2387],
                [0, 44, 1840, 2490],
                [0, 35, 1795, 2588],
                [0, 36, 1759, 2678],
                [3, 41, 1806, 2565],
            ],
            [15, 0],
        ),
        (
            [
                [17, 34, 2192, 1557],
                [1, 22, 2126, 1673],
                [0, 41, 2048, 1819],
                [0, 30, 1977, 1957],
                [0, 46, 1919, 2088],
                [0, 43, 1868, 2207],
                [1, 28, 1842, 2266],
                [0, 31, 1800, 2376],
            ],
            [16, 1],
        ),
        (
            [
                [0, 36, 1758, 2479],
                [0, 29, 1727, 2577],
                [0, 40, 1696, 2669],
                [1, 29, 1699, 2679],
                [0, 32, 1673, 2763],
                [2, 42, 1700, 2717],
                [1, 24, 1698, 2732],
                [1, 34, 1697, 2750],
            ],
            [0, 0],
        ),
        (
            [
                [16, 39, 2096, 1787],
                [2, 32, 2071, 1801],
                [1, 49, 2020, 1887],
                [0, 21, 1955, 2020],
                [0, 42, 1904, 2144],
                [0, 29, 1846, 2263],
                [1, 27, 1834, 2302],
                [3, 41, 1862, 2239],
            ],
            [16, 0],
        ),
        (
            [
                [16, 56, 2207, 1420],
                [2, 39, 2164, 1476],
                [0, 41, 2083, 1632],
                [0, 35, 2011, 1781],
                [1, 39, 1973, 1869],
                [0, 33, 1913, 2003],
                [1, 51, 1886, 2077],
                [1, 48, 1862, 2142],
            ],
            [16, 0],
        ),
        (
            [
                [0, 47, 1816, 2259],
                [0, 49, 1774, 2371],
                [0, 33, 1741, 2476],
                [0, 24, 1705, 2571],
                [1, 32, 1706, 2610],
                [0, 35, 1676, 2699],
                [0, 38, 1653, 2784],
                [0, 31, 1627, 2862],
            ],
            [0, 0],
        ),
        (
            [
                [16, 40, 2042, 1877],
                [0, 39, 1976, 2010],
                [0, 39, 1914, 2136],
                [0, 24, 1864, 2256],
                [2, 40, 1871, 2239],
                [1, 51, 1848, 2288],
                [0, 25, 1807, 2395],
                [0, 17, 1761, 2499],
            ],
            [16, 0],
        ),
        (
            [
                [17, 36, 2162, 1540],
                [0, 45, 2075, 1697],
                [0, 34, 2005, 1841],
                [0, 28, 1941, 1976],
                [2, 35, 1937, 1994],
                [0, 46, 1882, 2119],
                [0, 38, 1829, 2241],
                [0, 49, 1791, 2350],
            ],
            [16, 1],
        ),
        (
            [
                [0, 32, 1750, 2456],
                [1, 40, 1745, 2503],
                [0, 43, 1715, 2598],
                [0, 33, 1679, 2688],
                [0, 40, 1662, 2773],
                [0, 38, 1637, 2851],
                [1, 29, 1640, 2855],
                [0, 37, 1619, 2932],
            ],
            [0, 0],
        ),
        (
            [
                [18, 47, 2085, 1804],
                [0, 41, 2011, 1940],
                [0, 21, 1945, 2068],
                [2, 37, 1945, 2063],
                [0, 32, 1890, 2184],
                [0, 20, 1836, 2299],
                [1, 30, 1821, 2354],
                [1, 41, 1803, 2391],
            ],
            [16, 2],
        ),
        (
            [
                [18, 48, 2213, 1420],
                [0, 32, 2123, 1584],
                [0, 44, 2045, 1736],
                [0, 40, 1977, 1879],
                [0, 31, 1917, 2012],
                [1, 28, 1898, 2073],
                [0, 31, 1843, 2198],
                [0, 39, 1803, 2312],
            ],
            [16, 2],
        ),
        (
            [
                [0, 35, 1760, 2418],
                [1, 47, 1754, 2450],
                [0, 33, 1723, 2551],
                [1, 30, 1719, 2572],
                [0, 42, 1695, 2663],
                [0, 25, 1664, 2750],
                [0, 42, 1640, 2834],
                [0, 37, 1620, 2906],
            ],
            [0, 0],
        ),
        (
            [
                [17, 39, 2066, 1845],
                [1, 37, 2019, 1919],
                [0, 46, 1952, 2049],
                [1, 42, 1920, 2111],
                [2, 33, 1918, 2112],
                [0, 32, 1864, 2233],
                [0, 37, 1820, 2344],
                [1, 38, 1801, 2384],
            ],
            [16, 1],
        ),
        (
            [
                [16, 49, 2166, 1513],
                [1, 39, 2109, 1618],
                [0, 27, 2031, 1768],
                [0, 35, 1967, 1906],
                [0, 18, 1909, 2042],
                [1, 36, 1886, 2101],
                [0, 44, 1832, 2223],
                [0, 31, 1794, 2334],
            ],
            [16, 0],
        ),
        (
            [
                [2, 29, 1807, 2320],
                [0, 48, 1769, 2427],
                [0, 57, 1735, 2530],
                [0, 55, 1699, 2621],
                [0, 32, 1680, 2710],
                [1, 43, 1677, 2726],
                [0, 26, 1653, 2809],
                [2, 41, 1684, 2746],
            ],
            [0, 2],
        ),
        (
            [
                [17, 52, 2105, 1734],
                [0, 34, 2032, 1878],
                [1, 28, 1988, 1951],
                [0, 51, 1929, 2083],
                [0, 30, 1875, 2201],
                [0, 41, 1825, 2317],
                [0, 37, 1786, 2423],
                [0, 28, 1743, 2523],
            ],
            [16, 1],
        ),
        (
            [
                [16, 42, 2127, 1614],
                [2, 36, 2092, 1660],
                [0, 38, 2020, 1806],
                [0, 30, 1956, 1944],
                [0, 38, 1900, 2074],
                [0, 19, 1854, 2195],
                [1, 39, 1833, 2250],
                [0, 49, 1793, 2357],
            ],
            [16, 0],
        ),
        (
            [
                [1, 37, 1776, 2405],
                [2, 40, 1792, 2405],
                [0, 26, 1755, 2508],
                [1, 38, 1742, 2536],
                [1, 44, 1748, 2558],
                [1, 43, 1736, 2595],
                [2, 48, 1762, 2552],
                [0, 48, 1724, 2646],
            ],
            [0, 1],
        ),
        (
            [
                [16, 55, 2114, 1715],
                [0, 37, 2036, 1859],
                [0, 30, 1966, 1993],
                [0, 22, 1911, 2123],
                [0, 44, 1860, 2239],
                [0, 34, 1809, 2353],
                [0, 25, 1772, 2456],
                [0, 35, 1732, 2557],
            ],
            [16, 0],
        ),
        (
            [
                [17, 40, 2137, 1594],
                [1, 25, 2078, 1693],
                [0, 31, 2006, 1837],
                [0, 42, 1945, 1971],
                [0, 43, 1888, 2101],
                [1, 43, 1869, 2162],
                [0, 28, 1817, 2283],
                [1, 34, 1806, 2324],
            ],
            [16, 1],
        ),
        (
            [
                [2, 27, 1817, 2310],
                [0, 25, 1777, 2417],
                [1, 30, 1765, 2471],
                [2, 38, 1777, 2442],
                [0, 40, 1748, 2541],
                [1, 32, 1741, 2564],
                [1, 23, 1737, 2582],
                [1, 38, 1734, 2600],
            ],
            [0, 2],
        ),
        (
            [
                [15, 39, 2097, 1732],
                [0, 29, 2025, 1874],
                [1, 27, 1977, 1951],
                [2, 31, 1969, 1970],
                [1, 41, 1935, 2034],
                [1, 43, 1905, 2099],
                [0, 47, 1857, 2219],
                [0, 39, 1807, 2333],
            ],
            [15, 0],
        ),
        (
            [
                [17, 50, 2191, 1433],
                [2, 31, 2154, 1486],
                [0, 31, 2073, 1644],
                [0, 38, 2004, 1793],
                [0, 29, 1939, 1931],
                [0, 40, 1886, 2060],
                [1, 52, 1866, 2126],
                [2, 50, 1867, 2135],
            ],
            [16, 1],
        ),
    ],
    &[
        (
            [
                [0, 14, 3131, 7094],
                [0, 19, 3108, 7154],
                [3, 30, 3176, 6984],
                [0, 25, 3150, 7056],
                [0, 32, 3124, 7125],
                [3, 51, 3183, 6966],
                [0, 29, 3157, 7040],
                [3, 61, 3223, 6845],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 3189, 6925],
                [1, 40, 3190, 6910],
                [4, 50, 3278, 6663],
                [1, 58, 3261, 6687],
                [1, 42, 3256, 6707],
                [0, 36, 3219, 6794],
                [0, 37, 3187, 6876],
                [3, 22, 3244, 6718],
            ],
            [0, 0],
        ),
        (
            [
                [32, 51, 4078, 4382],
                [0, 43, 3939, 4611],
                [0, 28, 3821, 4827],
                [1, 35, 3752, 4948],
                [0, 44, 3648, 5146],
                [0, 35, 3565, 5333],
                [2, 34, 3545, 5355],
                [4, 40, 3587, 5248],
            ],
            [31, 1],
        ),
        (
            [
                [33, 46, 4343, 3244],
                [1, 22, 4198, 3507],
                [0, 41, 4050, 3787],
                [0, 27, 3910, 4053],
                [1, 46, 3825, 4239],
                [1, 42, 3752, 4414],
                [1, 28, 3673, 4588],
                [0, 31, 3591, 4805],
            ],
            [32, 1],
        ),
        (
            [
                [2, 34, 3556, 4892],
                [0, 29, 3492, 5095],
                [1, 39, 3452, 5215],
                [1, 29, 3420, 5313],
                [0, 32, 3368, 5488],
                [2, 42, 3369, 5525],
                [1, 24, 3340, 5616],
                [1, 34, 3319, 5708],
            ],
            [0, 2],
        ),
        (
            [
                [33, 49, 4158, 3682],
                [3, 37, 4086, 3765],
                [3, 46, 4015, 3850],
                [0, 21, 3890, 4112],
                [0, 42, 3788, 4355],
                [0, 29, 3676, 4589],
                [1, 27, 3623, 4735],
                [5, 39, 3666, 4634],
            ],
            [32, 2],
        ),
        (
            [
                [33, 65, 4394, 2893],
                [2, 37, 4263, 3110],
                [0, 34, 4109, 3414],
                [0, 35, 3974, 3701],
                [1, 38, 3877, 3923],
                [1, 32, 3791, 4131],
                [1, 51, 3711, 4327],
                [2, 43, 3677, 4435],
            ],
            [32, 1],
        ),
        (
            [
                [0, 46, 3585, 4663],
                [2, 47, 3560, 4771],
                [0, 32, 3490, 4979],
                [0, 24, 3416, 5171],
                [1, 32, 3395, 5299],
                [1, 34, 3366, 5401],
                [1, 37, 3344, 5498],
                [0, 31, 3290, 5661],
            ],
            [0, 1],
        ),
        (
            [
                [32, 51, 4113, 3698],
                [0, 42, 3972, 3967],
                [0, 39, 3846, 4223],
                [0, 24, 3744, 4464],
                [3, 39, 3733, 4495],
                [1, 45, 3659, 4657],
                [1, 26, 3607, 4801],
                [0, 16, 3520, 5007],
            ],
            [32, 0],
        ),
        (
            [
                [33, 48, 4299, 3139],
                [0, 45, 4130, 3450],
                [0, 32, 3991, 3734],
                [0, 28, 3865, 4001],
                [2, 35, 3809, 4143],
                [2, 44, 3763, 4274],
                [0, 38, 3657, 4517],
                [1, 48, 3603, 4666],
            ],
            [32, 1],
        ),
        (
            [
                [1, 30, 3546, 4833],
                [1, 40, 3501, 4982],
                [0, 43, 3439, 5173],
                [1, 32, 3394, 5286],
                [0, 40, 3357, 5460],
                [0, 38, 3304, 5623],
                [1, 29, 3277, 5707],
                [0, 37, 3235, 5858],
            ],
            [0, 1],
        ),
        (
            [
                [35, 62, 4144, 3677],
                [0, 43, 4000, 3944],
                [0, 21, 3867, 4198],
                [2, 38, 3817, 4312],
                [0, 32, 3719, 4544],
                [0, 19, 3616, 4765],
                [1, 31, 3567, 4918],
                [2, 39, 3540, 4979],
            ],
            [32, 3],
        ),
        (
            [
                [34, 54, 4333, 3072],
                [1, 32, 4187, 3327],
                [0, 44, 4038, 3621],
                [0, 39, 3914, 3897],
                [2, 29, 3853, 4033],
                [1, 28, 3780, 4218],
                [0, 30, 3671, 4462],
                [1, 38, 3623, 4617],
            ],
            [32, 2],
        ),
        (
            [
                [0, 35, 3534, 4829],
                [1, 47, 3491, 4963],
                [1, 32, 3461, 5096],
                [2, 28, 3449, 5138],
                [1, 42, 3429, 5262],
                [0, 25, 3360, 5439],
                [1, 41, 3339, 5535],
                [0, 36, 3290, 5689],
            ],
            [0, 0],
        ),
        (
            [
                [37, 45, 4234, 3453],
                [2, 36, 4127, 3615],
                [3, 42, 4055, 3713],
                [2, 41, 3975, 3873],
                [3, 32, 3925, 3980],
                [1, 31, 3835, 4168],
                [0, 37, 3733, 4409],
                [1, 38, 3659, 4573],
            ],
            [32, 6],
        ),
        (
            [
                [32, 67, 4369, 2896],
                [2, 39, 4250, 3110],
                [0, 28, 4093, 3416],
                [0, 35, 3960, 3700],
                [0, 18, 3840, 3977],
                [4, 32, 3837, 4003],
                [1, 43, 3750, 4199],
                [0, 31, 3658, 4441],
            ],
            [32, 0],
        ),
        (
            [
                [2, 28, 3625, 4546],
                [1, 45, 3574, 4699],
                [0, 52, 3499, 4911],
                [0, 55, 3429, 5105],
                [1, 31, 3409, 5226],
                [3, 41, 3430, 5200],
                [1, 27, 3397, 5323],
                [2, 41, 3396, 5356],
            ],
            [0, 2],
        ),
        (
            [
                [33, 59, 4204, 3428],
                [1, 38, 4078, 3666],
                [1, 29, 3969, 3878],
                [1, 46, 3877, 4090],
                [0, 29, 3766, 4332],
                [0, 42, 3663, 4570],
                [0, 37, 3583, 4786],
                [1, 27, 3526, 4927],
            ],
            [32, 1],
        ),
        (
            [
                [32, 60, 4282, 3146],
                [3, 41, 4181, 3316],
                [0, 38, 4036, 3608],
                [0, 27, 3909, 3888],
                [0, 33, 3796, 4147],
                [0, 18, 3702, 4387],
                [1, 39, 3634, 4559],
                [0, 49, 3557, 4775],
            ],
            [32, 0],
        ),
        (
            [
                [1, 37, 3502, 4925],
                [2, 41, 3492, 5017],
                [0, 26, 3426, 5211],
                [1, 38, 3385, 5323],
                [1, 44, 3378, 5420],
                [1, 43, 3347, 5536],
                [2, 48, 3349, 5561],
                [0, 48, 3295, 5718],
            ],
            [0, 1],
        ),
        (
            [
                [34, 61, 4167, 3624],
                [2, 49, 4069, 3775],
                [2, 33, 3983, 3913],
                [1, 22, 3884, 4128],
                [0, 44, 3777, 4371],
                [0, 34, 3667, 4603],
                [1, 24, 3611, 4763],
                [1, 34, 3551, 4909],
            ],
            [32, 2],
        ),
        (
            [
                [33, 49, 4317, 3087],
                [2, 24, 4192, 3301],
                [0, 31, 4043, 3594],
                [1, 41, 3943, 3810],
                [0, 43, 3824, 4075],
                [2, 41, 3777, 4219],
                [2, 26, 3720, 4340],
                [2, 33, 3686, 4439],
            ],
            [32, 1],
        ),
        (
            [
                [3, 26, 3674, 4485],
                [0, 25, 3586, 4709],
                [3, 27, 3588, 4764],
                [3, 38, 3579, 4786],
                [0, 40, 3517, 4992],
                [2, 31, 3496, 5061],
                [1, 20, 3456, 5177],
                [2, 35, 3450, 5216],
            ],
            [1, 2],
        ),
        (
            [
                [31, 45, 4206, 3433],
                [1, 28, 4083, 3659],
                [2, 26, 3990, 3810],
                [3, 30, 3944, 3899],
                [2, 36, 3882, 4029],
                [1, 40, 3792, 4223],
                [1, 46, 3722, 4392],
                [0, 39, 3627, 4624],
            ],
            [31, 0],
        ),
        (
            [
                [33, 62, 4371, 2883],
                [2, 30, 4247, 3096],
                [0, 31, 4092, 3404],
                [0, 37, 3959, 3692],
                [0, 29, 3837, 3962],
                [0, 40, 3735, 4218],
                [2, 50, 3697, 4344],
                [5, 46, 3737, 4281],
            ],
            [32, 1],
        ),
    ],
    &[
        (
            [
                [1, 13, 6297, 14133],
                [0, 19, 6245, 14261],
                [4, 29, 6324, 14065],
                [1, 24, 6305, 14123],
                [0, 32, 6253, 14259],
                [5, 49, 6350, 14011],
                [0, 29, 6295, 14151],
                [6, 58, 6424, 13787],
            ],
            [0, 1],
        ),
        (
            [
                [0, 35, 6358, 13940],
                [2, 39, 6357, 13929],
                [7, 47, 6509, 13501],
                [3, 56, 6519, 13439],
                [3, 40, 6535, 13380],
                [1, 35, 6484, 13476],
                [3, 34, 6496, 13425],
                [3, 22, 6507, 13362],
            ],
            [0, 0],
        ),
        (
            [
                [65, 75, 8196, 8631],
                [1, 40, 7934, 9038],
                [1, 28, 7721, 9407],
                [2, 34, 7572, 9667],
                [2, 43, 7412, 9928],
                [1, 33, 7261, 10259],
                [4, 32, 7206, 10334],
                [6, 38, 7223, 10269],
            ],
            [63, 2],
        ),
        (
            [
                [68, 61, 8756, 6281],
                [4, 19, 8506, 6710],
                [0, 41, 8198, 7290],
                [0, 27, 7911, 7839],
                [1, 46, 7699, 8294],
                [1, 38, 7522, 8717],
                [1, 29, 7337, 9129],
                [2, 30, 7231, 9433],
            ],
            [64, 5],
        ),
        (
            [
                [2, 34, 7109, 9734],
                [0, 29, 6972, 10139],
                [3, 37, 6925, 10322],
                [1, 29, 6825, 10603],
                [1, 31, 6754, 10883],
                [3, 41, 6732, 11023],
                [1, 24, 6648, 11275],
                [3, 32, 6641, 11377],
            ],
            [0, 2],
        ),
        (
            [
                [65, 76, 8294, 7373],
                [3, 40, 8079, 7723],
                [5, 41, 7923, 7949],
                [1, 20, 7717, 8390],
                [1, 41, 7536, 8829],
                [2, 26, 7376, 9162],
                [2, 26, 7257, 9486],
                [5, 38, 7209, 9591],
            ],
            [64, 2],
        ),
        (
            [
                [66, 72, 8709, 6024],
                [4, 33, 8449, 6460],
                [0, 32, 8148, 7058],
                [1, 33, 7910, 7558],
                [3, 35, 7755, 7918],
                [2, 30, 7588, 8333],
                [4, 47, 7479, 8592],
                [5, 39, 7432, 8745],
            ],
            [64, 2],
        ),
        (
            [
                [1, 43, 7264, 9143],
                [3, 46, 7184, 9404],
                [0, 33, 7035, 9830],
                [0, 24, 6878, 10218],
                [3, 30, 6866, 10403],
                [1, 34, 6768, 10693],
                [1, 37, 6685, 10962],
                [3, 28, 6667, 11096],
            ],
            [0, 2],
        ),
        (
            [
                [65, 66, 8314, 7169],
                [1, 41, 8046, 7659],
                [1, 37, 7808, 8123],
                [0, 25, 7588, 8627],
                [6, 35, 7554, 8704],
                [3, 43, 7418, 8993],
                [1, 26, 7272, 9371],
                [2, 14, 7151, 9680],
            ],
            [64, 1],
        ),
        (
            [
                [65, 58, 8655, 6116],
                [2, 48, 8357, 6645],
                [0, 33, 8066, 7229],
                [0, 25, 7809, 7781],
                [2, 33, 7637, 8187],
                [6, 40, 7582, 8353],
                [0, 38, 7370, 8846],
                [4, 44, 7296, 9085],
            ],
            [64, 1],
        ),
        (
            [
                [2, 29, 7177, 9413],
                [2, 38, 7078, 9718],
                [1, 43, 6972, 10049],
                [2, 31, 6878, 10288],
                [0, 39, 6789, 10656],
                [1, 35, 6702, 10929],
                [3, 26, 6676, 11055],
                [2, 35, 6632, 11247],
            ],
            [0, 2],
        ),
        (
            [
                [70, 72, 8402, 7024],
                [2, 37, 8144, 7499],
                [1, 20, 7890, 7965],
                [5, 35, 7790, 8190],
                [0, 30, 7571, 8680],
                [0, 20, 7354, 9146],
                [1, 30, 7217, 9530],
                [5, 37, 7176, 9614],
            ],
            [64, 6],
        ),
        (
            [
                [68, 65, 8735, 5913],
                [1, 27, 8412, 6494],
                [0, 44, 8115, 7091],
                [0, 34, 7854, 7652],
                [3, 27, 7701, 8003],
                [3, 28, 7567, 8344],
                [1, 28, 7382, 8771],
                [2, 37, 7276, 9095],
            ],
            [64, 4],
        ),
        (
            [
                [1, 34, 7129, 9465],
                [2, 46, 7037, 9748],
                [1, 32, 6938, 10086],
                [4, 26, 6909, 10190],
                [1, 42, 6839, 10503],
                [0, 25, 6714, 10858],
                [3, 41, 6690, 10994],
                [1, 35, 6615, 11254],
            ],
            [0, 1],
        ),
        (
            [
                [70, 62, 8398, 7043],
                [3, 43, 8161, 7450],
                [4, 37, 7966, 7793],
                [4, 40, 7826, 8088],
                [3, 33, 7674, 8437],
                [2, 29, 7502, 8802],
                [1, 36, 7348, 9191],
                [2, 37, 7210, 9501],
            ],
            [64, 7],
        ),
        (
            [
                [65, 89, 8687, 5997],
                [3, 41, 8421, 6466],
                [0, 26, 8117, 7060],
                [0, 32, 7862, 7622],
                [0, 18, 7630, 8155],
                [4, 32, 7538, 8419],
                [2, 42, 7379, 8790],
                [1, 30, 7240, 9180],
            ],
            [64, 1],
        ),
        (
            [
                [3, 27, 7152, 9423],
                [6, 39, 7162, 9478],
                [1, 50, 7041, 9830],
                [2, 48, 6950, 10082],
                [2, 30, 6903, 10325],
                [3, 41, 6849, 10483],
                [1, 27, 6757, 10785],
                [4, 37, 6756, 10865],
            ],
            [0, 3],
        ),
        (
            [
                [66, 86, 8391, 6955],
                [7, 33, 8252, 7169],
                [2, 28, 8018, 7610],
                [1, 46, 7797, 8092],
                [1, 28, 7607, 8521],
                [2, 40, 7435, 8893],
                [3, 33, 7329, 9205],
                [3, 24, 7222, 9452],
            ],
            [64, 3],
        ),
        (
            [
                [65, 78, 8701, 6004],
                [5, 40, 8467, 6395],
                [0, 38, 8165, 6992],
                [0, 26, 7897, 7560],
                [1, 32, 7689, 8042],
                [0, 17, 7484, 8540],
                [2, 37, 7338, 8890],
                [5, 43, 7301, 9072],
            ],
            [64, 1],
        ),
        (
            [
                [2, 36, 7174, 9408],
                [2, 46, 7073, 9722],
                [2, 27, 6990, 10021],
                [2, 30, 6893, 10271],
                [1, 43, 6833, 10563],
                [2, 42, 6767, 10787],
                [4, 47, 6762, 10860],
                [1, 46, 6677, 11133],
            ],
            [0, 2],
        ),
        (
            [
                [68, 85, 8391, 7068],
                [2, 45, 8130, 7545],
                [2, 24, 7901, 7950],
                [1, 22, 7698, 8401],
                [1, 43, 7517, 8823],
                [1, 33, 7330, 9222],
                [2, 23, 7223, 9530],
                [3, 29, 7131, 9743],
            ],
            [64, 4],
        ),
        (
            [
                [65, 67, 8632, 6161],
                [7, 20, 8459, 6436],
                [0, 31, 8154, 7032],
                [1, 38, 7919, 7534],
                [2, 40, 7726, 7962],
                [3, 41, 7599, 8299],
                [2, 26, 7432, 8670],
                [2, 33, 7311, 9000],
            ],
            [64, 2],
        ),
        (
            [
                [4, 24, 7240, 9199],
                [0, 24, 7078, 9629],
                [4, 26, 7053, 9817],
                [4, 38, 6998, 9960],
                [3, 36, 6975, 10154],
                [3, 30, 6913, 10340],
                [1, 20, 6815, 10634],
                [2, 35, 6757, 10835],
            ],
            [1, 3],
        ),
        (
            [
                [63, 61, 8331, 7098],
                [1, 26, 8068, 7593],
                [2, 27, 7844, 8002],
                [4, 30, 7720, 8268],
                [5, 33, 7635, 8459],
                [3, 37, 7496, 8752],
                [3, 43, 7391, 9030],
                [0, 39, 7200, 9482],
            ],
            [63, 0],
        ),
        (
            [
                [65, 73, 8684, 5979],
                [3, 32, 8413, 6464],
                [0, 35, 8111, 7064],
                [0, 37, 7854, 7625],
                [1, 28, 7649, 8087],
                [0, 39, 7452, 8588],
                [3, 47, 7341, 8895],
                [5, 46, 7301, 9051],
            ],
            [64, 1],
        ),
    ],
];

/// Each size's background, as `background_of` reads it.
const BACKGROUNDS_1024: [Background; 3] = [
    Background {
        members: 108,
        rest: 6954,
        ticks: 393216,
    },
    Background {
        members: 218,
        rest: 6844,
        ticks: 393216,
    },
    Background {
        members: 440,
        rest: 6622,
        ticks: 393216,
    },
];

/// Each size's kick over its control's sixteen kicks: the volley, the after, the kicks, the
/// kicks whose volley was every member.
const KICKS_1024: [(u64, u64, u64, u32); 3] =
    [(254, 10, 16, 14), (510, 22, 16, 14), (1022, 42, 16, 14)];

/// Whether the kick fires every member once, per size.
const KICKED_ONCE_1024: [bool; 3] = [true, true, true];

/// Each size's control read by the rules.
const CONTROLS_1024: [Cell; 3] = [
    Cell {
        held: 0,
        ignited: 0,
        let_go: 8,
        before: 0,
        spills: None,
    },
    Cell {
        held: 0,
        ignited: 0,
        let_go: 8,
        before: 0,
        spills: None,
    },
    Cell {
        held: 0,
        ignited: 0,
        let_go: 8,
        before: 0,
        spills: None,
    },
];

/// Each cell's rows, by size and weight.
const CELL_ROWS_1024: [[&[EpochRow]; 4]; 3] = [CELL_ROWS_16, CELL_ROWS_32, CELL_ROWS_64];

/// Each cell read by the rules, by size and weight.
const GRID_1024: [[Cell; 4]; 3] = [GRID_16, GRID_32, GRID_64];

/// A kick by the oracle: the ticks the unit fires on, its basal potential on the tick it fires,
/// its somatic and basal potentials on the last tick of its refractory window, its basal
/// potential on the tick the reset lands, and its soma `SETTLED_AFTER_RESET` ticks after.
type KickRead = (Vec<u32>, i32, i32, i32, i32, i32);

/// The assembly of 16's rows at each weight.
const CELL_ROWS_16: [&[EpochRow]; 4] = [
    &[
        (
            [
                [0, 14, 1578, 3492],
                [0, 19, 1564, 3525],
                [2, 31, 1611, 3399],
                [0, 25, 1597, 3441],
                [0, 32, 1582, 3481],
                [3, 52, 1647, 3306],
                [0, 29, 1629, 3355],
                [1, 63, 1640, 3310],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 1620, 3358],
                [1, 40, 1631, 3312],
                [5, 52, 1745, 2984],
                [1, 58, 1737, 2999],
                [0, 43, 1707, 3064],
                [0, 36, 1678, 3124],
                [0, 37, 1655, 3183],
                [2, 23, 1690, 3074],
            ],
            [0, 0],
        ),
        (
            [
                [25, 48, 2286, 1506],
                [0, 43, 2187, 1659],
                [0, 28, 2102, 1809],
                [1, 35, 2059, 1867],
                [0, 44, 1986, 2002],
                [0, 35, 1926, 2128],
                [0, 36, 1871, 2247],
                [4, 41, 1931, 2114],
            ],
            [15, 10],
        ),
        (
            [
                [20, 44, 2339, 1162],
                [1, 21, 2256, 1311],
                [0, 41, 2160, 1480],
                [0, 30, 2079, 1637],
                [0, 46, 2004, 1787],
                [0, 43, 1943, 1927],
                [1, 28, 1909, 2009],
                [0, 31, 1860, 2136],
            ],
            [15, 5],
        ),
        (
            [
                [0, 36, 1812, 2254],
                [0, 29, 1773, 2367],
                [0, 40, 1734, 2470],
                [1, 29, 1735, 2502],
                [0, 32, 1702, 2599],
                [2, 42, 1727, 2562],
                [1, 24, 1726, 2589],
                [1, 34, 1721, 2614],
            ],
            [0, 0],
        ),
        (
            [
                [23, 40, 2259, 1374],
                [2, 33, 2213, 1436],
                [1, 49, 2141, 1560],
                [0, 21, 2064, 1713],
                [0, 42, 1998, 1856],
                [0, 29, 1932, 1992],
                [1, 27, 1906, 2062],
                [3, 40, 1923, 2029],
            ],
            [16, 7],
        ),
        (
            [
                [21, 66, 2358, 1063],
                [1, 40, 2270, 1212],
                [0, 43, 2176, 1387],
                [0, 35, 2094, 1549],
                [1, 39, 2045, 1652],
                [0, 33, 1980, 1799],
                [1, 51, 1941, 1891],
                [1, 48, 1910, 1977],
            ],
            [16, 5],
        ),
        (
            [
                [0, 47, 1856, 2105],
                [0, 49, 1811, 2225],
                [0, 33, 1772, 2338],
                [0, 24, 1733, 2444],
                [1, 32, 1728, 2488],
                [0, 34, 1698, 2585],
                [1, 40, 1700, 2612],
                [0, 29, 1673, 2701],
            ],
            [0, 0],
        ),
        (
            [
                [18, 43, 2119, 1648],
                [0, 39, 2041, 1796],
                [0, 39, 1975, 1935],
                [0, 24, 1917, 2062],
                [2, 40, 1914, 2061],
                [1, 51, 1888, 2129],
                [0, 26, 1840, 2249],
                [0, 17, 1792, 2360],
            ],
            [16, 2],
        ),
        (
            [
                [19, 40, 2226, 1363],
                [0, 45, 2134, 1528],
                [0, 34, 2058, 1685],
                [0, 28, 1985, 1832],
                [2, 35, 1975, 1858],
                [0, 46, 1917, 1990],
                [0, 38, 1863, 2121],
                [0, 49, 1817, 2239],
            ],
            [16, 3],
        ),
        (
            [
                [0, 32, 1773, 2353],
                [1, 40, 1767, 2403],
                [0, 42, 1733, 2504],
                [0, 33, 1696, 2599],
                [0, 40, 1673, 2688],
                [0, 38, 1651, 2776],
                [1, 29, 1651, 2787],
                [0, 37, 1630, 2864],
            ],
            [0, 0],
        ),
        (
            [
                [18, 47, 2092, 1758],
                [0, 42, 2018, 1898],
                [0, 21, 1951, 2028],
                [2, 37, 1950, 2030],
                [0, 32, 1893, 2154],
                [0, 20, 1840, 2270],
                [1, 30, 1826, 2325],
                [1, 41, 1806, 2364],
            ],
            [16, 2],
        ),
        (
            [
                [22, 48, 2294, 1266],
                [0, 32, 2194, 1441],
                [0, 44, 2108, 1598],
                [0, 40, 2034, 1750],
                [0, 31, 1967, 1891],
                [1, 28, 1936, 1973],
                [0, 31, 1877, 2103],
                [0, 39, 1835, 2222],
            ],
            [16, 6],
        ),
        (
            [
                [0, 35, 1787, 2332],
                [1, 47, 1779, 2374],
                [0, 33, 1744, 2478],
                [1, 30, 1738, 2503],
                [0, 42, 1712, 2601],
                [0, 25, 1680, 2689],
                [0, 42, 1653, 2777],
                [0, 37, 1633, 2853],
            ],
            [0, 0],
        ),
        (
            [
                [20, 42, 2138, 1661],
                [1, 37, 2082, 1744],
                [0, 46, 2008, 1886],
                [1, 42, 1970, 1958],
                [3, 32, 1982, 1933],
                [0, 32, 1918, 2066],
                [0, 37, 1869, 2187],
                [1, 38, 1847, 2237],
            ],
            [16, 4],
        ),
        (
            [
                [17, 49, 2221, 1369],
                [1, 39, 2157, 1481],
                [0, 27, 2075, 1638],
                [0, 35, 2004, 1786],
                [0, 18, 1941, 1927],
                [2, 37, 1939, 1938],
                [0, 42, 1883, 2070],
                [0, 30, 1837, 2191],
            ],
            [16, 1],
        ),
        (
            [
                [2, 28, 1843, 2200],
                [0, 48, 1798, 2314],
                [0, 57, 1761, 2421],
                [0, 56, 1724, 2521],
                [0, 31, 1699, 2614],
                [2, 43, 1724, 2566],
                [0, 26, 1695, 2661],
                [2, 41, 1724, 2607],
            ],
            [0, 2],
        ),
        (
            [
                [21, 54, 2221, 1435],
                [0, 34, 2134, 1595],
                [1, 28, 2074, 1702],
                [0, 50, 2007, 1849],
                [0, 30, 1945, 1982],
                [0, 41, 1887, 2111],
                [0, 37, 1839, 2229],
                [0, 28, 1792, 2342],
            ],
            [16, 5],
        ),
        (
            [
                [19, 41, 2225, 1351],
                [2, 36, 2179, 1429],
                [0, 42, 2094, 1592],
                [0, 29, 2019, 1742],
                [0, 38, 1957, 1883],
                [0, 19, 1901, 2016],
                [1, 39, 1878, 2083],
                [0, 49, 1831, 2201],
            ],
            [16, 3],
        ),
        (
            [
                [1, 37, 1811, 2260],
                [2, 40, 1823, 2265],
                [0, 26, 1781, 2377],
                [1, 38, 1767, 2412],
                [1, 44, 1767, 2444],
                [1, 43, 1756, 2489],
                [2, 48, 1777, 2462],
                [0, 48, 1738, 2561],
            ],
            [0, 1],
        ),
        (
            [
                [18, 54, 2170, 1537],
                [0, 38, 2086, 1692],
                [0, 30, 2013, 1837],
                [0, 22, 1952, 1973],
                [0, 44, 1898, 2101],
                [0, 34, 1844, 2222],
                [0, 25, 1799, 2333],
                [0, 35, 1760, 2439],
            ],
            [16, 2],
        ),
        (
            [
                [20, 35, 2216, 1402],
                [2, 23, 2176, 1461],
                [0, 31, 2091, 1622],
                [0, 42, 2018, 1770],
                [0, 43, 1954, 1911],
                [1, 43, 1926, 1985],
                [0, 28, 1867, 2114],
                [1, 33, 1848, 2174],
            ],
            [16, 4],
        ),
        (
            [
                [2, 29, 1855, 2168],
                [0, 25, 1814, 2284],
                [1, 30, 1794, 2345],
                [2, 38, 1806, 2329],
                [0, 40, 1769, 2435],
                [1, 32, 1763, 2464],
                [1, 23, 1754, 2498],
                [2, 39, 1775, 2474],
            ],
            [0, 2],
        ),
        (
            [
                [20, 43, 2234, 1402],
                [0, 29, 2145, 1562],
                [1, 27, 2087, 1661],
                [2, 31, 2062, 1699],
                [1, 41, 2016, 1776],
                [1, 43, 1979, 1871],
                [0, 47, 1918, 2004],
                [0, 39, 1865, 2132],
            ],
            [15, 5],
        ),
        (
            [
                [17, 43, 2229, 1308],
                [2, 31, 2191, 1377],
                [0, 31, 2102, 1542],
                [0, 38, 2026, 1695],
                [0, 29, 1962, 1840],
                [0, 40, 1907, 1977],
                [1, 52, 1880, 2047],
                [2, 50, 1882, 2065],
            ],
            [16, 1],
        ),
    ],
    &[
        (
            [
                [0, 14, 1578, 3492],
                [0, 19, 1564, 3525],
                [2, 31, 1611, 3399],
                [0, 25, 1597, 3441],
                [0, 32, 1582, 3481],
                [27, 55, 2316, 1547],
                [41, 54, 2919, 353],
                [1, 62, 2769, 550],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 2613, 767],
                [1, 38, 2499, 934],
                [5, 51, 2468, 967],
                [0, 57, 2345, 1157],
                [0, 43, 2245, 1335],
                [0, 36, 2152, 1500],
                [0, 37, 2072, 1656],
                [2, 22, 2052, 1696],
            ],
            [0, 0],
        ),
        (
            [
                [39, 44, 2764, 414],
                [0, 41, 2614, 637],
                [0, 28, 2476, 847],
                [1, 35, 2379, 1000],
                [0, 43, 2271, 1189],
                [0, 34, 2177, 1364],
                [0, 36, 2093, 1530],
                [4, 41, 2117, 1490],
            ],
            [15, 24],
        ),
        (
            [
                [29, 36, 2620, 589],
                [0, 22, 2485, 802],
                [0, 40, 2365, 1001],
                [0, 30, 2259, 1187],
                [0, 46, 2166, 1363],
                [0, 43, 2081, 1528],
                [1, 28, 2033, 1639],
                [0, 31, 1966, 1787],
            ],
            [16, 13],
        ),
        (
            [
                [0, 34, 1906, 1926],
                [0, 30, 1858, 2058],
                [0, 42, 1813, 2180],
                [1, 28, 1794, 2236],
                [0, 32, 1761, 2346],
                [2, 42, 1774, 2337],
                [1, 24, 1772, 2379],
                [1, 35, 1762, 2418],
            ],
            [0, 0],
        ),
        (
            [
                [54, 41, 2836, 402],
                [2, 33, 2710, 575],
                [1, 49, 2578, 769],
                [0, 21, 2446, 970],
                [0, 42, 2330, 1161],
                [0, 29, 2230, 1335],
                [1, 27, 2163, 1457],
                [4, 40, 2171, 1441],
            ],
            [16, 38],
        ),
        (
            [
                [31, 56, 2694, 504],
                [1, 35, 2567, 700],
                [0, 34, 2438, 906],
                [0, 35, 2321, 1098],
                [1, 39, 2244, 1238],
                [0, 33, 2153, 1410],
                [1, 51, 2095, 1528],
                [1, 49, 2050, 1632],
            ],
            [16, 15],
        ),
        (
            [
                [0, 48, 1980, 1781],
                [0, 49, 1919, 1920],
                [0, 33, 1865, 2050],
                [0, 24, 1819, 2177],
                [1, 32, 1803, 2242],
                [0, 35, 1762, 2352],
                [0, 38, 1728, 2457],
                [0, 31, 1697, 2557],
            ],
            [0, 0],
        ),
        (
            [
                [55, 54, 2835, 367],
                [0, 42, 2678, 592],
                [0, 42, 2534, 803],
                [0, 24, 2406, 1005],
                [2, 40, 2337, 1109],
                [1, 50, 2254, 1244],
                [0, 26, 2165, 1418],
                [0, 17, 2083, 1578],
            ],
            [16, 39],
        ),
        (
            [
                [40, 35, 2785, 398],
                [0, 44, 2627, 621],
                [0, 34, 2493, 830],
                [0, 28, 2375, 1028],
                [4, 35, 2347, 1069],
                [0, 46, 2246, 1254],
                [0, 39, 2154, 1425],
                [0, 49, 2071, 1586],
            ],
            [16, 24],
        ),
        (
            [
                [0, 32, 2003, 1736],
                [1, 40, 1965, 1832],
                [1, 42, 1931, 1912],
                [0, 33, 1876, 2045],
                [0, 40, 1833, 2169],
                [0, 38, 1782, 2286],
                [4, 30, 1862, 2144],
                [1, 39, 1837, 2207],
            ],
            [0, 0],
        ),
        (
            [
                [49, 54, 2808, 382],
                [0, 41, 2649, 605],
                [0, 21, 2513, 817],
                [2, 36, 2426, 942],
                [0, 32, 2312, 1135],
                [0, 20, 2212, 1312],
                [1, 30, 2153, 1433],
                [1, 40, 2091, 1546],
            ],
            [16, 33],
        ),
        (
            [
                [36, 49, 2722, 478],
                [0, 32, 2576, 696],
                [0, 43, 2441, 902],
                [0, 40, 2328, 1094],
                [0, 31, 2226, 1275],
                [1, 28, 2158, 1403],
                [0, 27, 2077, 1569],
                [0, 40, 2003, 1721],
            ],
            [16, 20],
        ),
        (
            [
                [0, 35, 1943, 1866],
                [1, 47, 1916, 1939],
                [0, 33, 1861, 2067],
                [1, 30, 1843, 2130],
                [0, 42, 1800, 2249],
                [0, 25, 1761, 2360],
                [0, 42, 1726, 2463],
                [0, 37, 1696, 2562],
            ],
            [0, 0],
        ),
        (
            [
                [50, 49, 2758, 438],
                [1, 35, 2626, 632],
                [0, 44, 2488, 843],
                [1, 42, 2384, 1007],
                [3, 32, 2332, 1085],
                [0, 33, 2230, 1270],
                [0, 37, 2145, 1440],
                [1, 37, 2089, 1553],
            ],
            [16, 34],
        ),
        (
            [
                [33, 48, 2674, 526],
                [1, 38, 2553, 712],
                [0, 26, 2423, 915],
                [0, 35, 2312, 1109],
                [0, 18, 2211, 1287],
                [2, 35, 2170, 1367],
                [0, 42, 2089, 1533],
                [0, 30, 2014, 1688],
            ],
            [16, 17],
        ),
        (
            [
                [2, 28, 1998, 1739],
                [0, 48, 1936, 1881],
                [0, 57, 1883, 2014],
                [0, 56, 1830, 2142],
                [0, 32, 1792, 2259],
                [2, 43, 1804, 2243],
                [0, 26, 1767, 2357],
                [3, 42, 1815, 2270],
            ],
            [0, 2],
        ),
        (
            [
                [55, 58, 2881, 330],
                [0, 34, 2715, 559],
                [1, 28, 2585, 742],
                [0, 50, 2453, 945],
                [0, 30, 2341, 1134],
                [0, 41, 2232, 1312],
                [0, 37, 2147, 1482],
                [0, 28, 2065, 1639],
            ],
            [16, 39],
        ),
        (
            [
                [35, 39, 2692, 514],
                [1, 35, 2567, 713],
                [0, 38, 2435, 920],
                [0, 30, 2325, 1111],
                [0, 38, 2219, 1290],
                [0, 19, 2133, 1461],
                [1, 39, 2078, 1571],
                [0, 49, 2008, 1722],
            ],
            [16, 19],
        ),
        (
            [
                [1, 37, 1970, 1815],
                [2, 40, 1959, 1859],
                [0, 26, 1897, 1993],
                [1, 38, 1873, 2063],
                [1, 44, 1859, 2122],
                [1, 43, 1832, 2190],
                [2, 48, 1846, 2186],
                [0, 47, 1799, 2302],
            ],
            [0, 1],
        ),
        (
            [
                [52, 61, 2834, 388],
                [0, 39, 2673, 611],
                [0, 29, 2531, 822],
                [0, 22, 2402, 1018],
                [0, 44, 2291, 1206],
                [0, 34, 2194, 1381],
                [0, 25, 2110, 1546],
                [0, 35, 2037, 1699],
            ],
            [16, 36],
        ),
        (
            [
                [38, 39, 2720, 494],
                [2, 24, 2614, 646],
                [0, 30, 2474, 857],
                [0, 42, 2360, 1051],
                [0, 43, 2251, 1235],
                [1, 43, 2182, 1367],
                [0, 28, 2096, 1531],
                [1, 33, 2044, 1639],
            ],
            [16, 22],
        ),
        (
            [
                [3, 27, 2049, 1636],
                [0, 24, 1979, 1785],
                [1, 31, 1940, 1882],
                [2, 38, 1928, 1924],
                [0, 40, 1876, 2059],
                [1, 32, 1851, 2121],
                [1, 21, 1838, 2178],
                [2, 39, 1847, 2177],
            ],
            [0, 3],
        ),
        (
            [
                [48, 49, 2790, 426],
                [0, 28, 2635, 646],
                [1, 27, 2515, 832],
                [2, 29, 2431, 958],
                [1, 41, 2337, 1102],
                [1, 43, 2255, 1244],
                [0, 47, 2165, 1416],
                [0, 39, 2082, 1577],
            ],
            [15, 33],
        ),
        (
            [
                [38, 46, 2758, 413],
                [2, 31, 2648, 581],
                [0, 32, 2508, 796],
                [0, 38, 2383, 995],
                [0, 29, 2277, 1183],
                [0, 40, 2180, 1357],
                [1, 52, 2117, 1480],
                [2, 50, 2085, 1547],
            ],
            [16, 22],
        ),
    ],
    &[
        (
            [
                [0, 14, 1578, 3492],
                [0, 19, 1564, 3525],
                [2, 31, 1611, 3399],
                [0, 25, 1597, 3441],
                [0, 32, 1582, 3481],
                [41, 67, 2624, 783],
                [27, 57, 2909, 371],
                [1, 58, 2751, 577],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 2604, 789],
                [1, 38, 2487, 946],
                [5, 51, 2460, 973],
                [0, 57, 2341, 1161],
                [0, 43, 2241, 1340],
                [0, 36, 2145, 1505],
                [0, 37, 2068, 1663],
                [2, 22, 2046, 1701],
            ],
            [0, 0],
        ),
        (
            [
                [47, 44, 2867, 328],
                [0, 41, 2701, 554],
                [0, 28, 2558, 770],
                [1, 35, 2446, 936],
                [0, 43, 2335, 1128],
                [0, 34, 2231, 1307],
                [0, 37, 2141, 1477],
                [7, 43, 2227, 1303],
            ],
            [15, 32],
        ),
        (
            [
                [42, 33, 2893, 315],
                [0, 23, 2728, 546],
                [0, 40, 2578, 760],
                [0, 30, 2444, 962],
                [0, 46, 2331, 1151],
                [0, 43, 2227, 1328],
                [1, 28, 2160, 1450],
                [0, 31, 2077, 1609],
            ],
            [15, 27],
        ),
        (
            [
                [0, 36, 2009, 1760],
                [0, 29, 1945, 1902],
                [0, 40, 1889, 2032],
                [1, 29, 1863, 2097],
                [0, 32, 1818, 2220],
                [2, 42, 1827, 2224],
                [1, 24, 1811, 2275],
                [1, 34, 1796, 2321],
            ],
            [0, 0],
        ),
        (
            [
                [61, 41, 2954, 275],
                [2, 33, 2809, 474],
                [1, 49, 2666, 664],
                [0, 21, 2527, 872],
                [0, 42, 2400, 1069],
                [0, 29, 2287, 1252],
                [1, 27, 2221, 1374],
                [7, 40, 2278, 1248],
            ],
            [16, 45],
        ),
        (
            [
                [43, 56, 2926, 297],
                [1, 37, 2769, 510],
                [0, 41, 2615, 726],
                [0, 35, 2478, 932],
                [1, 39, 2378, 1085],
                [0, 33, 2277, 1266],
                [1, 53, 2200, 1394],
                [1, 48, 2133, 1510],
            ],
            [16, 27],
        ),
        (
            [
                [0, 48, 2055, 1666],
                [0, 49, 1986, 1811],
                [0, 33, 1926, 1951],
                [0, 24, 1872, 2080],
                [3, 35, 1894, 2061],
                [0, 35, 1840, 2181],
                [0, 38, 1803, 2299],
                [0, 31, 1757, 2408],
            ],
            [0, 0],
        ),
        (
            [
                [64, 50, 2985, 244],
                [0, 41, 2807, 477],
                [0, 40, 2652, 696],
                [0, 23, 2509, 902],
                [4, 41, 2467, 951],
                [0, 46, 2343, 1139],
                [0, 26, 2244, 1319],
                [0, 16, 2149, 1487],
            ],
            [16, 48],
        ),
        (
            [
                [50, 38, 2958, 262],
                [0, 44, 2781, 493],
                [0, 34, 2630, 711],
                [0, 28, 2493, 916],
                [4, 35, 2443, 980],
                [0, 46, 2331, 1166],
                [0, 38, 2227, 1344],
                [0, 49, 2138, 1512],
            ],
            [16, 34],
        ),
        (
            [
                [0, 32, 2061, 1667],
                [1, 40, 2015, 1767],
                [21, 43, 2465, 846],
                [29, 51, 2839, 372],
                [0, 45, 2678, 598],
                [0, 35, 2536, 809],
                [1, 29, 2429, 979],
                [0, 37, 2313, 1166],
            ],
            [0, 0],
        ),
        (
            [
                [45, 44, 2968, 257],
                [0, 42, 2789, 487],
                [0, 21, 2637, 706],
                [2, 36, 2539, 844],
                [0, 32, 2411, 1041],
                [0, 20, 2302, 1227],
                [1, 30, 2224, 1356],
                [1, 40, 2158, 1474],
            ],
            [16, 29],
        ),
        (
            [
                [50, 48, 2958, 266],
                [0, 31, 2780, 498],
                [0, 42, 2630, 717],
                [0, 39, 2493, 918],
                [0, 31, 2368, 1110],
                [1, 28, 2286, 1250],
                [0, 30, 2191, 1421],
                [0, 39, 2102, 1585],
            ],
            [16, 34],
        ),
        (
            [
                [0, 35, 2027, 1735],
                [1, 47, 1987, 1822],
                [0, 33, 1925, 1960],
                [1, 30, 1898, 2030],
                [0, 42, 1853, 2152],
                [0, 25, 1804, 2270],
                [0, 42, 1761, 2379],
                [0, 37, 1730, 2483],
            ],
            [0, 0],
        ),
        (
            [
                [64, 49, 2976, 239],
                [0, 37, 2799, 474],
                [0, 45, 2644, 692],
                [1, 42, 2521, 872],
                [3, 32, 2451, 965],
                [1, 32, 2348, 1116],
                [0, 37, 2251, 1297],
                [1, 37, 2177, 1420],
            ],
            [16, 48],
        ),
        (
            [
                [47, 47, 2934, 279],
                [1, 40, 2776, 494],
                [0, 27, 2622, 710],
                [0, 35, 2488, 913],
                [0, 18, 2366, 1107],
                [2, 35, 2307, 1203],
                [0, 42, 2204, 1379],
                [0, 30, 2117, 1543],
            ],
            [16, 31],
        ),
        (
            [
                [2, 28, 2089, 1604],
                [0, 48, 2016, 1756],
                [0, 57, 1950, 1896],
                [0, 56, 1893, 2030],
                [0, 32, 1845, 2153],
                [2, 43, 1851, 2149],
                [0, 26, 1808, 2266],
                [5, 43, 1890, 2090],
            ],
            [0, 2],
        ),
        (
            [
                [58, 56, 2954, 278],
                [0, 34, 2779, 509],
                [1, 28, 2643, 703],
                [0, 50, 2503, 907],
                [0, 30, 2381, 1102],
                [0, 41, 2270, 1282],
                [0, 37, 2179, 1453],
                [0, 28, 2094, 1612],
            ],
            [16, 42],
        ),
        (
            [
                [52, 40, 2959, 259],
                [1, 35, 2803, 473],
                [0, 38, 2642, 693],
                [0, 30, 2505, 899],
                [0, 38, 2383, 1091],
                [0, 19, 2273, 1273],
                [1, 39, 2201, 1397],
                [0, 49, 2114, 1559],
            ],
            [16, 36],
        ),
        (
            [
                [1, 37, 2063, 1660],
                [2, 40, 2041, 1713],
                [0, 26, 1972, 1859],
                [12, 40, 2219, 1340],
                [44, 53, 2916, 300],
                [0, 40, 2747, 528],
                [3, 49, 2651, 664],
                [0, 42, 2512, 872],
            ],
            [0, 1],
        ),
        (
            [
                [28, 47, 2840, 405],
                [0, 39, 2677, 628],
                [0, 30, 2534, 838],
                [0, 22, 2408, 1032],
                [0, 44, 2297, 1221],
                [0, 34, 2198, 1393],
                [0, 25, 2113, 1556],
                [0, 35, 2035, 1711],
            ],
            [16, 12],
        ),
        (
            [
                [54, 40, 2961, 254],
                [1, 23, 2799, 472],
                [0, 31, 2644, 693],
                [0, 42, 2506, 898],
                [0, 43, 2382, 1090],
                [1, 43, 2291, 1233],
                [0, 28, 2198, 1406],
                [1, 33, 2132, 1522],
            ],
            [16, 38],
        ),
        (
            [
                [4, 27, 2148, 1475],
                [0, 24, 2066, 1633],
                [1, 31, 2016, 1742],
                [5, 38, 2062, 1658],
                [0, 41, 1996, 1804],
                [1, 32, 1957, 1894],
                [1, 21, 1924, 1974],
                [2, 39, 1922, 1988],
            ],
            [0, 4],
        ),
        (
            [
                [57, 46, 2955, 269],
                [0, 28, 2778, 503],
                [1, 27, 2639, 699],
                [2, 29, 2537, 846],
                [1, 41, 2430, 1007],
                [1, 43, 2333, 1154],
                [0, 47, 2235, 1330],
                [0, 39, 2145, 1497],
            ],
            [15, 42],
        ),
        (
            [
                [49, 53, 2944, 267],
                [2, 31, 2800, 461],
                [0, 32, 2647, 683],
                [0, 38, 2506, 890],
                [0, 29, 2383, 1085],
                [0, 40, 2273, 1266],
                [1, 52, 2201, 1393],
                [2, 50, 2158, 1463],
            ],
            [16, 33],
        ),
    ],
    &[
        (
            [
                [0, 14, 1578, 3492],
                [0, 19, 1564, 3525],
                [70, 75, 3062, 179],
                [2, 32, 2898, 400],
                [0, 37, 2730, 624],
                [5, 41, 2666, 711],
                [0, 31, 2524, 912],
                [1, 62, 2417, 1059],
            ],
            [0, 0],
        ),
        (
            [
                [0, 35, 2309, 1245],
                [1, 39, 2232, 1369],
                [56, 57, 3111, 149],
                [1, 61, 2927, 380],
                [0, 45, 2758, 603],
                [0, 36, 2604, 816],
                [0, 37, 2472, 1014],
                [2, 22, 2391, 1122],
            ],
            [0, 0],
        ),
        (
            [
                [48, 36, 3042, 233],
                [0, 41, 2858, 466],
                [0, 29, 2696, 685],
                [1, 35, 2573, 860],
                [0, 44, 2439, 1055],
                [0, 34, 2325, 1238],
                [0, 36, 2221, 1413],
                [15, 43, 2474, 893],
            ],
            [15, 33],
        ),
        (
            [
                [38, 32, 2961, 292],
                [0, 23, 2784, 523],
                [0, 44, 2626, 740],
                [0, 30, 2496, 942],
                [0, 46, 2371, 1133],
                [0, 43, 2264, 1313],
                [1, 28, 2190, 1433],
                [0, 31, 2104, 1595],
            ],
            [12, 26],
        ),
        (
            [
                [0, 36, 2032, 1749],
                [0, 29, 1967, 1889],
                [0, 42, 1905, 2021],
                [1, 28, 1879, 2086],
                [0, 31, 1835, 2208],
                [4, 42, 1894, 2086],
                [58, 29, 2978, 250],
                [1, 37, 2814, 473],
            ],
            [0, 0],
        ),
        (
            [
                [24, 39, 2975, 333],
                [2, 33, 2826, 520],
                [1, 48, 2681, 712],
                [0, 21, 2539, 919],
                [0, 42, 2411, 1111],
                [0, 29, 2297, 1290],
                [1, 27, 2224, 1410],
                [8, 40, 2311, 1227],
            ],
            [16, 8],
        ),
        (
            [
                [48, 57, 3006, 253],
                [2, 37, 2850, 464],
                [0, 41, 2691, 683],
                [0, 35, 2545, 889],
                [1, 38, 2437, 1044],
                [0, 32, 2322, 1228],
                [1, 53, 2242, 1357],
                [3, 48, 2208, 1407],
            ],
            [16, 32],
        ),
        (
            [
                [0, 47, 2124, 1571],
                [0, 49, 2045, 1722],
                [0, 33, 1977, 1864],
                [0, 24, 1917, 1998],
                [63, 43, 3098, 131],
                [0, 35, 2907, 371],
                [0, 38, 2734, 596],
                [0, 31, 2590, 807],
            ],
            [0, 0],
        ),
        (
            [
                [43, 38, 3078, 230],
                [0, 40, 2887, 466],
                [0, 39, 2722, 683],
                [0, 24, 2571, 889],
                [5, 42, 2539, 895],
                [1, 52, 2433, 1050],
                [0, 24, 2317, 1233],
                [0, 17, 2217, 1407],
            ],
            [16, 27],
        ),
        (
            [
                [53, 37, 3027, 228],
                [0, 44, 2842, 461],
                [0, 34, 2682, 681],
                [0, 28, 2540, 890],
                [34, 37, 3007, 213],
                [13, 48, 2983, 322],
                [0, 37, 2807, 552],
                [0, 49, 2649, 765],
            ],
            [16, 37],
        ),
        (
            [
                [0, 32, 2511, 967],
                [1, 40, 2404, 1115],
                [25, 43, 2800, 390],
                [24, 38, 2988, 275],
                [0, 41, 2806, 509],
                [0, 37, 2652, 724],
                [3, 30, 2562, 837],
                [1, 38, 2453, 1008],
            ],
            [0, 0],
        ),
        (
            [
                [46, 45, 3049, 236],
                [0, 42, 2862, 474],
                [0, 22, 2697, 690],
                [2, 36, 2592, 830],
                [0, 33, 2459, 1031],
                [0, 21, 2340, 1214],
                [1, 30, 2255, 1344],
                [1, 40, 2190, 1462],
            ],
            [16, 30],
        ),
        (
            [
                [54, 48, 3026, 227],
                [0, 32, 2844, 458],
                [0, 44, 2679, 681],
                [0, 40, 2538, 888],
                [0, 31, 2410, 1082],
                [1, 28, 2321, 1221],
                [0, 27, 2221, 1396],
                [0, 40, 2131, 1559],
            ],
            [16, 38],
        ),
        (
            [
                [0, 35, 2052, 1711],
                [1, 47, 2008, 1800],
                [0, 33, 1944, 1941],
                [1, 30, 1914, 2010],
                [0, 42, 1864, 2134],
                [0, 25, 1818, 2253],
                [0, 42, 1776, 2365],
                [0, 37, 1739, 2469],
            ],
            [0, 0],
        ),
        (
            [
                [71, 50, 3061, 200],
                [0, 38, 2872, 433],
                [0, 45, 2709, 655],
                [1, 42, 2579, 839],
                [3, 32, 2501, 934],
                [1, 33, 2395, 1088],
                [0, 37, 2286, 1271],
                [1, 37, 2211, 1396],
            ],
            [16, 55],
        ),
        (
            [
                [51, 46, 3000, 240],
                [1, 40, 2833, 457],
                [0, 27, 2674, 678],
                [0, 35, 2532, 886],
                [0, 18, 2402, 1080],
                [2, 35, 2340, 1175],
                [0, 42, 2235, 1350],
                [0, 30, 2144, 1517],
            ],
            [16, 35],
        ),
        (
            [
                [13, 28, 2374, 1039],
                [47, 50, 3026, 240],
                [0, 58, 2845, 474],
                [0, 56, 2683, 690],
                [0, 32, 2543, 898],
                [2, 43, 2453, 1020],
                [0, 26, 2336, 1204],
                [3, 41, 2293, 1254],
            ],
            [0, 26],
        ),
        (
            [
                [55, 50, 3080, 207],
                [0, 34, 2892, 441],
                [1, 28, 2738, 638],
                [0, 50, 2590, 848],
                [0, 30, 2459, 1047],
                [0, 41, 2341, 1228],
                [0, 37, 2236, 1403],
                [0, 28, 2147, 1565],
            ],
            [16, 39],
        ),
        (
            [
                [54, 39, 3011, 234],
                [1, 35, 2845, 450],
                [0, 38, 2683, 672],
                [0, 30, 2539, 880],
                [0, 38, 2413, 1074],
                [0, 19, 2298, 1257],
                [1, 39, 2223, 1380],
                [0, 49, 2133, 1544],
            ],
            [16, 38],
        ),
        (
            [
                [1, 37, 2077, 1645],
                [2, 40, 2052, 1703],
                [0, 26, 1981, 1849],
                [54, 44, 3026, 154],
                [7, 45, 2933, 332],
                [0, 41, 2760, 560],
                [3, 50, 2658, 694],
                [0, 42, 2517, 900],
            ],
            [0, 1],
        ),
        (
            [
                [44, 49, 3054, 231],
                [0, 40, 2866, 467],
                [0, 30, 2704, 687],
                [0, 22, 2558, 895],
                [0, 44, 2429, 1085],
                [0, 34, 2311, 1270],
                [0, 25, 2213, 1437],
                [0, 35, 2126, 1600],
            ],
            [16, 28],
        ),
        (
            [
                [56, 39, 3022, 228],
                [1, 23, 2853, 448],
                [0, 31, 2691, 671],
                [0, 42, 2545, 877],
                [0, 43, 2416, 1072],
                [1, 43, 2325, 1214],
                [0, 28, 2226, 1387],
                [1, 33, 2156, 1504],
            ],
            [16, 40],
        ),
        (
            [
                [46, 32, 2989, 172],
                [14, 31, 2992, 287],
                [1, 30, 2825, 502],
                [2, 38, 2691, 678],
                [0, 40, 2545, 883],
                [1, 32, 2437, 1044],
                [1, 21, 2344, 1184],
                [2, 38, 2284, 1269],
            ],
            [0, 53],
        ),
        (
            [
                [51, 41, 3025, 243],
                [0, 28, 2844, 479],
                [1, 27, 2698, 673],
                [3, 29, 2604, 789],
                [1, 41, 2488, 954],
                [1, 43, 2384, 1119],
                [0, 47, 2275, 1298],
                [0, 39, 2176, 1469],
            ],
            [15, 36],
        ),
        (
            [
                [56, 53, 3047, 216],
                [2, 31, 2887, 417],
                [0, 32, 2721, 640],
                [0, 38, 2573, 849],
                [0, 29, 2442, 1047],
                [0, 40, 2327, 1228],
                [1, 52, 2249, 1360],
                [2, 50, 2196, 1433],
            ],
            [16, 40],
        ),
    ],
];

/// The assembly of 16 read by the rules at each weight.
const GRID_16: [Cell; 4] = [
    Cell {
        held: 0,
        ignited: 0,
        let_go: 8,
        before: 0,
        spills: None,
    },
    Cell {
        held: 0,
        ignited: 0,
        let_go: 8,
        before: 0,
        spills: None,
    },
    Cell {
        held: 0,
        ignited: 1,
        let_go: 8,
        before: 0,
        spills: None,
    },
    Cell {
        held: 1,
        ignited: 2,
        let_go: 6,
        before: 0,
        spills: Some(false),
    },
];

/// The assembly of 32's rows at each weight.
const CELL_ROWS_32: [&[EpochRow]; 4] = [
    &[
        (
            [
                [0, 14, 3131, 7094],
                [0, 19, 3108, 7154],
                [3, 30, 3176, 6984],
                [0, 25, 3150, 7056],
                [0, 32, 3124, 7125],
                [5, 51, 3245, 6800],
                [0, 29, 3212, 6887],
                [4, 61, 3302, 6622],
            ],
            [0, 0],
        ),
        (
            [
                [6, 37, 3429, 6244],
                [3, 41, 3445, 6149],
                [92, 97, 5568, 995],
                [33, 92, 5756, 780],
                [1, 40, 5439, 1207],
                [1, 36, 5165, 1594],
                [0, 36, 4901, 1993],
                [3, 22, 4723, 2255],
            ],
            [0, 6],
        ),
        (
            [
                [40, 36, 5257, 1248],
                [0, 42, 4984, 1672],
                [0, 24, 4737, 2065],
                [1, 33, 4548, 2391],
                [0, 44, 4360, 2746],
                [0, 33, 4189, 3075],
                [2, 34, 4090, 3282],
                [5, 40, 4078, 3320],
            ],
            [31, 9],
        ),
        (
            [
                [75, 45, 5437, 963],
                [1, 21, 5157, 1389],
                [0, 44, 4899, 1798],
                [0, 26, 4657, 2184],
                [2, 45, 4501, 2467],
                [1, 42, 4338, 2773],
                [2, 26, 4223, 3002],
                [0, 30, 4072, 3317],
            ],
            [31, 44],
        ),
        (
            [
                [2, 32, 3986, 3518],
                [0, 29, 3858, 3803],
                [1, 41, 3781, 4003],
                [1, 28, 3711, 4194],
                [0, 31, 3618, 4433],
                [2, 42, 3590, 4550],
                [1, 24, 3543, 4709],
                [1, 35, 3495, 4855],
            ],
            [0, 2],
        ),
        (
            [
                [107, 78, 5660, 750],
                [3, 37, 5382, 1146],
                [3, 45, 5138, 1510],
                [0, 21, 4882, 1918],
                [0, 42, 4648, 2297],
                [0, 29, 4444, 2651],
                [1, 27, 4294, 2939],
                [7, 38, 4292, 2935],
            ],
            [32, 76],
        ),
        (
            [
                [70, 67, 5490, 882],
                [2, 35, 5221, 1280],
                [0, 34, 4954, 1695],
                [0, 35, 4713, 2089],
                [1, 38, 4523, 2421],
                [1, 32, 4359, 2729],
                [1, 53, 4210, 3012],
                [3, 43, 4133, 3180],
            ],
            [32, 38],
        ),
        (
            [
                [0, 47, 3996, 3479],
                [2, 47, 3913, 3669],
                [0, 32, 3801, 3943],
                [0, 24, 3697, 4199],
                [1, 32, 3635, 4390],
                [1, 33, 3574, 4563],
                [1, 37, 3532, 4720],
                [0, 31, 3459, 4930],
            ],
            [0, 1],
        ),
        (
            [
                [101, 77, 5560, 858],
                [0, 45, 5252, 1301],
                [0, 43, 4980, 1718],
                [0, 24, 4733, 2110],
                [3, 39, 4589, 2357],
                [1, 43, 4409, 2667],
                [2, 27, 4289, 2910],
                [0, 16, 4127, 3227],
            ],
            [32, 69],
        ),
        (
            [
                [83, 56, 5596, 772],
                [0, 47, 5289, 1222],
                [0, 37, 5008, 1645],
                [0, 28, 4762, 2036],
                [2, 35, 4587, 2336],
                [2, 44, 4433, 2614],
                [0, 38, 4256, 2951],
                [1, 48, 4123, 3218],
            ],
            [32, 51],
        ),
        (
            [
                [1, 30, 4004, 3485],
                [1, 40, 3903, 3723],
                [0, 42, 3792, 3992],
                [1, 32, 3709, 4188],
                [0, 40, 3625, 4428],
                [0, 37, 3538, 4659],
                [3, 29, 3549, 4686],
                [0, 37, 3481, 4892],
            ],
            [0, 1],
        ),
        (
            [
                [97, 79, 5509, 898],
                [0, 46, 5203, 1336],
                [0, 21, 4938, 1753],
                [2, 37, 4736, 2069],
                [0, 32, 4527, 2442],
                [0, 19, 4330, 2791],
                [3, 31, 4246, 2980],
                [2, 38, 4139, 3196],
            ],
            [32, 65],
        ),
        (
            [
                [76, 51, 5502, 861],
                [1, 30, 5220, 1277],
                [0, 43, 4950, 1698],
                [0, 39, 4709, 2091],
                [2, 29, 4538, 2386],
                [1, 28, 4376, 2691],
                [0, 29, 4204, 3025],
                [1, 38, 4078, 3282],
            ],
            [32, 44],
        ),
        (
            [
                [0, 35, 3945, 3581],
                [1, 47, 3854, 3798],
                [1, 32, 3774, 4002],
                [2, 28, 3726, 4137],
                [1, 42, 3663, 4332],
                [0, 26, 3570, 4568],
                [1, 41, 3530, 4719],
                [0, 36, 3462, 4925],
            ],
            [0, 0],
        ),
        (
            [
                [100, 57, 5543, 855],
                [1, 35, 5256, 1271],
                [3, 39, 5039, 1599],
                [2, 41, 4823, 1936],
                [4, 31, 4677, 2181],
                [1, 31, 4492, 2502],
                [0, 37, 4317, 2842],
                [1, 36, 4179, 3115],
            ],
            [32, 68],
        ),
        (
            [
                [64, 54, 5318, 1087],
                [2, 30, 5077, 1467],
                [0, 26, 4822, 1873],
                [0, 35, 4604, 2254],
                [0, 18, 4403, 2613],
                [5, 30, 4341, 2738],
                [1, 41, 4199, 3021],
                [0, 30, 4046, 3334],
            ],
            [32, 32],
        ),
        (
            [
                [2, 28, 3965, 3533],
                [1, 43, 3872, 3760],
                [0, 53, 3762, 4029],
                [0, 56, 3658, 4280],
                [1, 30, 3612, 4450],
                [4, 41, 3631, 4428],
                [1, 27, 3576, 4608],
                [2, 41, 3553, 4699],
            ],
            [0, 2],
        ),
        (
            [
                [103, 78, 5626, 763],
                [1, 36, 5322, 1199],
                [1, 28, 5064, 1592],
                [1, 46, 4824, 1962],
                [0, 32, 4602, 2338],
                [0, 41, 4401, 2692],
                [0, 37, 4241, 3024],
                [1, 27, 4102, 3285],
            ],
            [32, 71],
        ),
        (
            [
                [83, 57, 5593, 762],
                [2, 38, 5315, 1180],
                [0, 42, 5033, 1607],
                [0, 28, 4785, 2005],
                [0, 34, 4562, 2379],
                [0, 17, 4370, 2730],
                [1, 39, 4222, 3009],
                [1, 49, 4098, 3277],
            ],
            [32, 51],
        ),
        (
            [
                [1, 37, 3990, 3522],
                [3, 41, 3941, 3651],
                [1, 26, 3846, 3868],
                [1, 38, 3761, 4073],
                [1, 44, 3697, 4260],
                [1, 43, 3625, 4447],
                [3, 50, 3631, 4491],
                [0, 47, 3544, 4717],
            ],
            [0, 1],
        ),
        (
            [
                [103, 82, 5624, 773],
                [2, 45, 5348, 1172],
                [2, 27, 5096, 1547],
                [1, 25, 4850, 1919],
                [0, 44, 4625, 2302],
                [0, 34, 4423, 2657],
                [1, 24, 4274, 2957],
                [3, 34, 4186, 3125],
            ],
            [32, 71],
        ),
        (
            [
                [73, 40, 5477, 880],
                [3, 23, 5230, 1251],
                [0, 31, 4956, 1674],
                [1, 41, 4745, 2031],
                [0, 43, 4525, 2405],
                [2, 42, 4378, 2670],
                [2, 26, 4255, 2909],
                [3, 32, 4169, 3084],
            ],
            [32, 41],
        ),
        (
            [
                [5, 26, 4147, 3141],
                [0, 24, 4006, 3444],
                [4, 28, 3974, 3536],
                [3, 39, 3912, 3696],
                [0, 40, 3806, 3966],
                [2, 31, 3747, 4113],
                [1, 21, 3684, 4298],
                [3, 38, 3676, 4360],
            ],
            [1, 4],
        ),
        (
            [
                [102, 64, 5669, 737],
                [2, 28, 5382, 1144],
                [2, 26, 5130, 1519],
                [3, 27, 4926, 1823],
                [2, 33, 4730, 2144],
                [1, 40, 4534, 2474],
                [1, 46, 4370, 2770],
                [0, 39, 4202, 3097],
            ],
            [31, 71],
        ),
        (
            [
                [72, 55, 5452, 952],
                [2, 29, 5203, 1317],
                [0, 32, 4933, 1736],
                [0, 37, 4698, 2127],
                [0, 29, 4486, 2490],
                [0, 40, 4305, 2838],
                [3, 50, 4213, 3019],
                [6, 47, 4201, 3049],
            ],
            [32, 40],
        ),
    ],
    &[
        (
            [
                [0, 14, 3131, 7094],
                [0, 19, 3108, 7154],
                [141, 125, 6142, 328],
                [10, 40, 5882, 715],
                [0, 31, 5538, 1170],
                [4, 44, 5310, 1482],
                [0, 28, 5029, 1888],
                [3, 56, 4837, 2154],
            ],
            [0, 0],
        ),
        (
            [
                [5, 35, 4714, 2323],
                [3, 41, 4560, 2577],
                [88, 59, 6034, 342],
                [27, 70, 6015, 576],
                [1, 45, 5667, 1022],
                [1, 34, 5361, 1425],
                [0, 37, 5078, 1837],
                [3, 22, 4878, 2110],
            ],
            [0, 5],
        ),
        (
            [
                [90, 40, 6056, 497],
                [0, 42, 5689, 961],
                [0, 29, 5362, 1396],
                [1, 35, 5100, 1773],
                [0, 43, 4842, 2159],
                [0, 33, 4615, 2523],
                [2, 33, 4458, 2782],
                [9, 41, 4489, 2659],
            ],
            [31, 59],
        ),
        (
            [
                [99, 41, 5975, 522],
                [1, 22, 5629, 972],
                [0, 44, 5310, 1407],
                [0, 26, 5032, 1819],
                [2, 45, 4825, 2132],
                [1, 42, 4619, 2454],
                [2, 28, 4464, 2703],
                [0, 31, 4285, 3037],
            ],
            [31, 68],
        ),
        (
            [
                [2, 34, 4172, 3265],
                [0, 29, 4027, 3561],
                [1, 41, 3921, 3782],
                [2, 28, 3855, 3930],
                [0, 30, 3752, 4187],
                [2, 41, 3701, 4326],
                [1, 24, 3641, 4491],
                [1, 35, 3579, 4653],
            ],
            [0, 2],
        ),
        (
            [
                [136, 73, 6084, 427],
                [2, 36, 5742, 865],
                [4, 45, 5468, 1235],
                [0, 21, 5169, 1656],
                [0, 42, 4905, 2053],
                [0, 29, 4669, 2423],
                [1, 27, 4487, 2726],
                [65, 43, 5657, 665],
            ],
            [32, 104],
        ),
        (
            [
                [43, 62, 5937, 592],
                [1, 34, 5594, 1040],
                [0, 34, 5285, 1471],
                [0, 35, 5007, 1880],
                [1, 38, 4778, 2222],
                [1, 31, 4580, 2544],
                [1, 51, 4405, 2835],
                [4, 44, 4325, 2966],
            ],
            [24, 19],
        ),
        (
            [
                [0, 47, 4165, 3286],
                [2, 46, 4063, 3486],
                [0, 32, 3930, 3771],
                [0, 24, 3813, 4037],
                [1, 32, 3735, 4236],
                [1, 33, 3663, 4423],
                [1, 37, 3603, 4589],
                [0, 31, 3523, 4806],
            ],
            [0, 1],
        ),
        (
            [
                [133, 76, 6032, 434],
                [0, 47, 5663, 910],
                [0, 42, 5342, 1345],
                [0, 24, 5059, 1755],
                [3, 39, 4867, 2045],
                [1, 43, 4649, 2377],
                [2, 27, 4500, 2638],
                [0, 16, 4310, 2974],
            ],
            [32, 101],
        ),
        (
            [
                [116, 56, 6123, 411],
                [0, 45, 5741, 883],
                [0, 32, 5418, 1326],
                [0, 28, 5122, 1740],
                [4, 35, 4938, 2002],
                [7, 47, 4842, 2097],
                [0, 38, 4617, 2462],
                [1, 48, 4438, 2761],
            ],
            [32, 84],
        ),
        (
            [
                [1, 30, 4277, 3059],
                [1, 40, 4147, 3320],
                [2, 42, 4049, 3505],
                [1, 33, 3943, 3729],
                [0, 40, 3825, 3999],
                [0, 38, 3724, 4252],
                [110, 52, 5977, 358],
                [24, 49, 5934, 610],
            ],
            [0, 1],
        ),
        (
            [
                [37, 40, 6037, 619],
                [0, 40, 5671, 1071],
                [0, 20, 5350, 1504],
                [2, 36, 5105, 1837],
                [0, 33, 4850, 2224],
                [0, 21, 4614, 2584],
                [4, 30, 4512, 2738],
                [2, 39, 4362, 2968],
            ],
            [32, 5],
        ),
        (
            [
                [109, 52, 6049, 458],
                [1, 29, 5702, 906],
                [0, 43, 5376, 1350],
                [0, 39, 5090, 1767],
                [2, 29, 4868, 2080],
                [1, 28, 4662, 2408],
                [0, 27, 4457, 2753],
                [1, 39, 4302, 3033],
            ],
            [32, 77],
        ),
        (
            [
                [0, 35, 4142, 3344],
                [1, 47, 4026, 3581],
                [1, 32, 3920, 3809],
                [6, 28, 3956, 3728],
                [1, 43, 3864, 3944],
                [0, 27, 3754, 4202],
                [1, 41, 3684, 4378],
                [0, 36, 3594, 4607],
            ],
            [0, 0],
        ),
        (
            [
                [139, 56, 6121, 407],
                [0, 34, 5745, 879],
                [2, 41, 5453, 1275],
                [2, 41, 5188, 1638],
                [4, 31, 4994, 1908],
                [2, 32, 4780, 2211],
                [0, 37, 4569, 2574],
                [1, 36, 4398, 2866],
            ],
            [32, 107],
        ),
        (
            [
                [107, 64, 6045, 460],
                [2, 32, 5706, 895],
                [0, 26, 5381, 1335],
                [0, 35, 5097, 1751],
                [0, 18, 4837, 2139],
                [7, 30, 4753, 2250],
                [1, 42, 4558, 2562],
                [0, 30, 4367, 2903],
            ],
            [32, 75],
        ),
        (
            [
                [2, 28, 4244, 3132],
                [1, 44, 4113, 3383],
                [0, 53, 3970, 3675],
                [0, 56, 3851, 3946],
                [1, 30, 3772, 4142],
                [4, 41, 3774, 4143],
                [1, 27, 3699, 4343],
                [2, 41, 3660, 4457],
            ],
            [0, 2],
        ),
        (
            [
                [132, 72, 6063, 433],
                [1, 36, 5706, 892],
                [1, 29, 5397, 1314],
                [1, 46, 5119, 1697],
                [0, 32, 4865, 2093],
                [0, 41, 4629, 2460],
                [0, 37, 4436, 2809],
                [1, 27, 4275, 3087],
            ],
            [32, 100],
        ),
        (
            [
                [110, 54, 6029, 459],
                [2, 37, 5690, 908],
                [0, 42, 5363, 1345],
                [0, 28, 5078, 1763],
                [0, 34, 4826, 2151],
                [0, 18, 4600, 2519],
                [1, 39, 4427, 2810],
                [1, 49, 4270, 3088],
            ],
            [32, 78],
        ),
        (
            [
                [1, 37, 4140, 3347],
                [3, 41, 4071, 3488],
                [1, 26, 3958, 3724],
                [1, 38, 3860, 3932],
                [1, 44, 3783, 4127],
                [1, 43, 3707, 4326],
                [4, 50, 3721, 4317],
                [7, 47, 3820, 4125],
            ],
            [0, 1],
        ),
        (
            [
                [128, 66, 6064, 453],
                [2, 43, 5732, 876],
                [2, 28, 5434, 1278],
                [2, 25, 5168, 1645],
                [0, 44, 4906, 2043],
                [0, 34, 4667, 2411],
                [1, 24, 4485, 2730],
                [3, 34, 4365, 2915],
            ],
            [32, 96],
        ),
        (
            [
                [108, 45, 6044, 457],
                [2, 22, 5705, 901],
                [0, 31, 5382, 1339],
                [1, 41, 5108, 1726],
                [0, 43, 4851, 2118],
                [3, 42, 4683, 2367],
                [3, 27, 4538, 2588],
                [5, 34, 4452, 2713],
            ],
            [32, 76],
        ),
        (
            [
                [104, 37, 6189, 248],
                [13, 37, 5972, 608],
                [3, 28, 5652, 1014],
                [2, 38, 5361, 1403],
                [0, 40, 5083, 1814],
                [3, 31, 4878, 2088],
                [1, 21, 4679, 2411],
                [3, 37, 4536, 2621],
            ],
            [2, 110],
        ),
        (
            [
                [104, 45, 6067, 459],
                [1, 28, 5714, 906],
                [2, 26, 5422, 1298],
                [4, 27, 5196, 1601],
                [2, 34, 4967, 1928],
                [1, 40, 4745, 2270],
                [1, 46, 4556, 2578],
                [0, 39, 4363, 2918],
            ],
            [31, 73],
        ),
        (
            [
                [111, 62, 6077, 445],
                [2, 29, 5735, 878],
                [0, 31, 5404, 1320],
                [0, 37, 5114, 1736],
                [0, 29, 4859, 2130],
                [0, 40, 4627, 2493],
                [3, 50, 4492, 2701],
                [110, 56, 6231, 267],
            ],
            [32, 79],
        ),
    ],
    &[
        (
            [
                [0, 14, 3131, 7094],
                [0, 19, 3108, 7154],
                [167, 119, 6438, 174],
                [5, 33, 6081, 633],
                [0, 36, 5703, 1087],
                [4, 39, 5445, 1423],
                [0, 31, 5149, 1834],
                [3, 66, 4947, 2101],
            ],
            [0, 0],
        ),
        (
            [
                [8, 42, 4856, 2187],
                [118, 49, 6399, 294],
                [6, 52, 6052, 726],
                [0, 56, 5684, 1178],
                [1, 38, 5373, 1573],
                [1, 37, 5106, 1935],
                [0, 38, 4852, 2308],
                [3, 22, 4683, 2536],
            ],
            [0, 12],
        ),
        (
            [
                [121, 41, 6309, 370],
                [0, 42, 5913, 843],
                [0, 29, 5564, 1285],
                [1, 35, 5277, 1671],
                [0, 42, 4994, 2070],
                [0, 33, 4748, 2438],
                [2, 33, 4580, 2697],
                [78, 46, 5928, 421],
            ],
            [31, 90],
        ),
        (
            [
                [34, 32, 6004, 587],
                [0, 21, 5646, 1044],
                [0, 41, 5325, 1481],
                [0, 26, 5046, 1885],
                [2, 46, 4832, 2192],
                [1, 39, 4627, 2511],
                [2, 28, 4474, 2754],
                [0, 31, 4291, 3085],
            ],
            [29, 5],
        ),
        (
            [
                [4, 34, 4228, 3200],
                [1, 31, 4096, 3458],
                [1, 39, 3984, 3690],
                [72, 37, 5519, 797],
                [65, 52, 6103, 489],
                [2, 46, 5746, 939],
                [1, 25, 5443, 1354],
                [1, 34, 5161, 1733],
            ],
            [0, 4],
        ),
        (
            [
                [104, 47, 6337, 386],
                [3, 35, 5970, 821],
                [5, 42, 5679, 1161],
                [0, 21, 5360, 1586],
                [0, 42, 5076, 1986],
                [0, 29, 4819, 2360],
                [1, 27, 4616, 2667],
                [120, 54, 6439, 155],
            ],
            [32, 73],
        ),
        (
            [
                [32, 55, 6343, 516],
                [1, 37, 5956, 973],
                [0, 38, 5601, 1400],
                [0, 35, 5284, 1816],
                [1, 37, 5028, 2159],
                [1, 31, 4796, 2481],
                [1, 53, 4596, 2784],
                [127, 58, 6447, 211],
            ],
            [30, 2],
        ),
        (
            [
                [2, 53, 6056, 678],
                [2, 45, 5721, 1099],
                [0, 31, 5386, 1526],
                [0, 24, 5097, 1932],
                [1, 32, 4858, 2270],
                [4, 34, 4714, 2458],
                [1, 37, 4519, 2756],
                [0, 31, 4344, 3082],
            ],
            [2, 1],
        ),
        (
            [
                [123, 54, 6202, 394],
                [0, 42, 5817, 869],
                [0, 40, 5481, 1312],
                [0, 24, 5179, 1726],
                [51, 43, 5850, 567],
                [55, 51, 6193, 486],
                [1, 26, 5824, 938],
                [0, 16, 5488, 1380],
            ],
            [32, 91],
        ),
        (
            [
                [94, 36, 6391, 400],
                [0, 45, 5986, 874],
                [0, 34, 5627, 1320],
                [0, 28, 5313, 1734],
                [2, 35, 5064, 2055],
                [1, 43, 4827, 2380],
                [0, 38, 4599, 2730],
                [1, 48, 4436, 3010],
            ],
            [32, 62],
        ),
        (
            [
                [1, 30, 4272, 3294],
                [1, 40, 4136, 3539],
                [135, 63, 6375, 215],
                [2, 30, 5991, 682],
                [0, 43, 5638, 1139],
                [0, 37, 5310, 1564],
                [24, 30, 5465, 1211],
                [83, 39, 6272, 437],
            ],
            [0, 1],
        ),
        (
            [
                [39, 39, 6286, 551],
                [0, 40, 5892, 1007],
                [0, 20, 5545, 1449],
                [2, 36, 5274, 1791],
                [0, 33, 5002, 2176],
                [0, 20, 4752, 2539],
                [4, 30, 4620, 2689],
                [2, 39, 4464, 2926],
            ],
            [32, 7],
        ),
        (
            [
                [126, 50, 6286, 369],
                [1, 29, 5908, 819],
                [0, 42, 5558, 1266],
                [0, 39, 5248, 1686],
                [2, 29, 5009, 2011],
                [1, 28, 4787, 2340],
                [0, 27, 4571, 2695],
                [1, 39, 4396, 2975],
            ],
            [32, 94],
        ),
        (
            [
                [0, 36, 4221, 3294],
                [1, 47, 4095, 3536],
                [1, 32, 3985, 3758],
                [94, 34, 5866, 452],
                [47, 57, 6110, 501],
                [0, 27, 5737, 972],
                [1, 43, 5431, 1377],
                [0, 35, 5137, 1789],
            ],
            [0, 0],
        ),
        (
            [
                [101, 30, 6292, 403],
                [1, 36, 5903, 865],
                [3, 41, 5602, 1240],
                [2, 41, 5322, 1600],
                [5, 31, 5127, 1850],
                [4, 34, 4933, 2111],
                [0, 37, 4699, 2484],
                [1, 36, 4509, 2779],
            ],
            [32, 70],
        ),
        (
            [
                [124, 59, 6284, 363],
                [1, 38, 5906, 820],
                [0, 25, 5561, 1266],
                [0, 35, 5245, 1686],
                [0, 18, 4972, 2077],
                [90, 38, 6267, 223],
                [29, 64, 6194, 534],
                [0, 36, 5813, 995],
            ],
            [32, 92],
        ),
        (
            [
                [2, 28, 5506, 1375],
                [1, 45, 5221, 1757],
                [0, 53, 4945, 2144],
                [0, 55, 4709, 2512],
                [1, 31, 4524, 2806],
                [92, 49, 6084, 311],
                [35, 30, 6141, 520],
                [2, 37, 5793, 956],
            ],
            [0, 2],
        ),
        (
            [
                [74, 45, 6341, 457],
                [1, 38, 5950, 912],
                [1, 30, 5610, 1326],
                [1, 45, 5309, 1713],
                [0, 32, 5029, 2105],
                [0, 40, 4786, 2471],
                [0, 37, 4564, 2815],
                [3, 28, 4440, 2986],
            ],
            [32, 42],
        ),
        (
            [
                [128, 57, 6293, 375],
                [2, 36, 5919, 820],
                [0, 42, 5569, 1264],
                [0, 28, 5260, 1685],
                [0, 33, 4984, 2082],
                [0, 19, 4747, 2446],
                [1, 39, 4551, 2747],
                [1, 49, 4379, 3029],
            ],
            [32, 96],
        ),
        (
            [
                [1, 37, 4229, 3291],
                [5, 40, 4194, 3327],
                [128, 51, 6233, 358],
                [1, 46, 5860, 819],
                [1, 42, 5527, 1244],
                [1, 38, 5245, 1634],
                [4, 48, 5048, 1891],
                [1, 48, 4810, 2231],
            ],
            [0, 1],
        ),
        (
            [
                [111, 51, 6250, 394],
                [1, 39, 5874, 855],
                [2, 25, 5550, 1251],
                [2, 22, 5281, 1610],
                [0, 44, 5001, 2012],
                [0, 34, 4752, 2384],
                [1, 24, 4559, 2702],
                [7, 34, 4531, 2686],
            ],
            [32, 79],
        ),
        (
            [
                [121, 40, 6261, 372],
                [2, 22, 5897, 819],
                [0, 31, 5551, 1265],
                [1, 41, 5259, 1655],
                [0, 44, 4981, 2055],
                [3, 41, 4799, 2305],
                [3, 27, 4638, 2531],
                [61, 40, 5679, 697],
            ],
            [31, 90],
        ),
        (
            [
                [60, 35, 6136, 494],
                [0, 22, 5761, 961],
                [4, 27, 5487, 1302],
                [7, 38, 5296, 1532],
                [0, 40, 5021, 1938],
                [3, 31, 4828, 2218],
                [1, 21, 4622, 2535],
                [51, 37, 5513, 877],
            ],
            [25, 35],
        ),
        (
            [
                [62, 36, 6065, 521],
                [1, 28, 5712, 968],
                [2, 26, 5418, 1355],
                [4, 27, 5194, 1651],
                [2, 34, 4967, 1976],
                [1, 40, 4742, 2310],
                [1, 46, 4555, 2615],
                [0, 41, 4364, 2951],
            ],
            [30, 31],
        ),
        (
            [
                [128, 59, 6266, 373],
                [2, 29, 5901, 812],
                [0, 31, 5551, 1257],
                [0, 37, 5244, 1678],
                [0, 29, 4970, 2075],
                [0, 40, 4729, 2444],
                [5, 50, 4632, 2561],
                [127, 56, 6387, 306],
            ],
            [32, 96],
        ),
    ],
    &[
        (
            [
                [0, 14, 3131, 7094],
                [0, 19, 3108, 7154],
                [177, 120, 6545, 136],
                [6, 28, 6178, 590],
                [0, 31, 5791, 1053],
                [7, 39, 5572, 1301],
                [100, 36, 6580, 219],
                [1, 68, 6157, 694],
            ],
            [0, 0],
        ),
        (
            [
                [0, 42, 5778, 1145],
                [1, 39, 5467, 1544],
                [108, 59, 6636, 153],
                [8, 65, 6281, 594],
                [1, 38, 5904, 1030],
                [1, 36, 5569, 1436],
                [0, 37, 5262, 1845],
                [4, 22, 5056, 2074],
            ],
            [0, 0],
        ),
        (
            [
                [115, 38, 6398, 364],
                [0, 39, 5989, 840],
                [0, 28, 5632, 1287],
                [1, 33, 5329, 1670],
                [0, 44, 5046, 2068],
                [0, 33, 4793, 2434],
                [2, 33, 4621, 2698],
                [113, 49, 6393, 154],
            ],
            [32, 83],
        ),
        (
            [
                [32, 29, 6318, 505],
                [0, 25, 5918, 976],
                [0, 40, 5567, 1408],
                [0, 26, 5260, 1823],
                [2, 45, 5026, 2131],
                [1, 40, 4787, 2450],
                [99, 35, 6295, 191],
                [28, 33, 6203, 531],
            ],
            [24, 7],
        ),
        (
            [
                [1, 33, 5837, 978],
                [0, 31, 5493, 1410],
                [1, 42, 5208, 1789],
                [3, 28, 5005, 2062],
                [4, 32, 4831, 2283],
                [2, 42, 4640, 2598],
                [2, 24, 4476, 2851],
                [103, 40, 6221, 218],
            ],
            [0, 1],
        ),
        (
            [
                [35, 41, 6218, 511],
                [3, 36, 5872, 927],
                [12, 43, 5707, 1064],
                [98, 24, 6567, 295],
                [0, 41, 6135, 774],
                [0, 29, 5760, 1222],
                [1, 27, 5447, 1608],
                [108, 46, 6642, 140],
            ],
            [28, 8],
        ),
        (
            [
                [30, 50, 6470, 508],
                [1, 37, 6064, 968],
                [0, 38, 5697, 1395],
                [0, 35, 5375, 1807],
                [1, 38, 5108, 2149],
                [1, 32, 4865, 2481],
                [1, 50, 4655, 2771],
                [136, 57, 6534, 211],
            ],
            [28, 2],
        ),
        (
            [
                [0, 47, 6110, 694],
                [1, 42, 5760, 1125],
                [0, 30, 5420, 1553],
                [0, 24, 5128, 1956],
                [10, 32, 5053, 1949],
                [121, 42, 6496, 305],
                [0, 39, 6081, 785],
                [0, 30, 5706, 1233],
            ],
            [0, 0],
        ),
        (
            [
                [90, 42, 6441, 408],
                [0, 42, 6031, 884],
                [0, 39, 5664, 1326],
                [0, 24, 5348, 1741],
                [106, 44, 6599, 130],
                [12, 49, 6281, 563],
                [0, 25, 5885, 1025],
                [0, 16, 5539, 1456],
            ],
            [32, 58],
        ),
        (
            [
                [98, 36, 6445, 392],
                [0, 45, 6035, 868],
                [0, 34, 5671, 1314],
                [0, 28, 5349, 1731],
                [84, 41, 6371, 196],
                [38, 51, 6357, 473],
                [0, 37, 5955, 940],
                [1, 48, 5615, 1353],
            ],
            [32, 66],
        ),
        (
            [
                [1, 30, 5312, 1752],
                [1, 40, 5056, 2103],
                [117, 52, 6510, 228],
                [1, 33, 6102, 699],
                [0, 40, 5731, 1152],
                [0, 38, 5396, 1578],
                [107, 34, 6590, 176],
                [1, 41, 6167, 651],
            ],
            [0, 1],
        ),
        (
            [
                [77, 41, 6567, 391],
                [0, 40, 6139, 865],
                [0, 20, 5763, 1310],
                [2, 36, 5460, 1659],
                [0, 33, 5173, 2054],
                [0, 21, 4903, 2424],
                [117, 39, 6522, 132],
                [12, 41, 6218, 558],
            ],
            [32, 45],
        ),
        (
            [
                [59, 40, 6438, 469],
                [1, 29, 6046, 915],
                [0, 44, 5677, 1354],
                [0, 39, 5357, 1767],
                [2, 29, 5106, 2077],
                [1, 28, 4866, 2404],
                [1, 27, 4660, 2711],
                [2, 39, 4499, 2939],
            ],
            [32, 27],
        ),
        (
            [
                [132, 53, 6418, 269],
                [0, 45, 6009, 750],
                [0, 32, 5641, 1201],
                [4, 28, 5399, 1500],
                [1, 41, 5122, 1874],
                [0, 25, 4863, 2255],
                [1, 41, 4651, 2568],
                [0, 36, 4456, 2910],
            ],
            [3, 129],
        ),
        (
            [
                [140, 33, 6423, 321],
                [1, 36, 6025, 790],
                [2, 41, 5695, 1189],
                [2, 41, 5397, 1559],
                [7, 31, 5225, 1737],
                [115, 37, 6597, 161],
                [1, 38, 6174, 644],
                [1, 37, 5809, 1082],
            ],
            [32, 109],
        ),
        (
            [
                [93, 40, 6518, 390],
                [2, 40, 6119, 832],
                [0, 26, 5749, 1277],
                [0, 35, 5414, 1698],
                [0, 18, 5126, 2088],
                [120, 40, 6584, 188],
                [2, 48, 6170, 661],
                [0, 33, 5789, 1122],
            ],
            [32, 61],
        ),
        (
            [
                [2, 26, 5492, 1487],
                [44, 43, 5945, 560],
                [70, 64, 6393, 427],
                [0, 53, 5985, 895],
                [1, 30, 5638, 1317],
                [4, 39, 5382, 1612],
                [0, 24, 5095, 2010],
                [3, 40, 4894, 2262],
            ],
            [0, 2],
        ),
        (
            [
                [124, 55, 6411, 351],
                [1, 39, 6016, 814],
                [1, 30, 5669, 1238],
                [1, 45, 5354, 1629],
                [0, 32, 5077, 2025],
                [0, 41, 4820, 2402],
                [0, 37, 4594, 2746],
                [7, 28, 4559, 2721],
            ],
            [32, 92],
        ),
        (
            [
                [130, 52, 6363, 348],
                [2, 37, 5977, 799],
                [0, 42, 5622, 1249],
                [0, 28, 5301, 1669],
                [0, 33, 5025, 2062],
                [0, 19, 4779, 2430],
                [1, 39, 4580, 2732],
                [1, 49, 4405, 3016],
            ],
            [32, 98],
        ),
        (
            [
                [1, 37, 4253, 3273],
                [40, 40, 5052, 1568],
                [104, 53, 6292, 397],
                [2, 38, 5928, 845],
                [1, 43, 5591, 1269],
                [1, 43, 5291, 1657],
                [7, 49, 5147, 1793],
                [115, 54, 6476, 294],
            ],
            [0, 1],
        ),
        (
            [
                [36, 38, 6400, 527],
                [1, 37, 6002, 974],
                [3, 26, 5678, 1337],
                [3, 22, 5406, 1664],
                [0, 44, 5113, 2061],
                [0, 34, 4849, 2425],
                [1, 24, 4644, 2739],
                [122, 44, 6491, 135],
            ],
            [32, 4],
        ),
        (
            [
                [30, 37, 6368, 508],
                [2, 22, 5986, 943],
                [0, 30, 5629, 1380],
                [1, 41, 5326, 1760],
                [0, 43, 5047, 2149],
                [4, 42, 4866, 2344],
                [121, 39, 6469, 258],
                [4, 33, 6087, 707],
            ],
            [24, 6],
        ),
        (
            [
                [6, 25, 5799, 1032],
                [0, 23, 5467, 1463],
                [4, 28, 5235, 1741],
                [115, 42, 6547, 226],
                [0, 40, 6122, 710],
                [3, 31, 5788, 1098],
                [1, 21, 5466, 1497],
                [49, 37, 6007, 489],
            ],
            [1, 5],
        ),
        (
            [
                [51, 37, 6239, 506],
                [1, 27, 5863, 952],
                [2, 26, 5550, 1342],
                [13, 27, 5461, 1359],
                [97, 39, 6412, 379],
                [1, 40, 6011, 842],
                [1, 46, 5665, 1262],
                [0, 39, 5340, 1678],
            ],
            [27, 24],
        ),
        (
            [
                [107, 43, 6436, 381],
                [2, 29, 6046, 817],
                [0, 32, 5683, 1263],
                [0, 37, 5359, 1684],
                [0, 29, 5073, 2077],
                [0, 40, 4822, 2447],
                [16, 50, 4929, 2071],
                [115, 56, 6362, 363],
            ],
            [32, 75],
        ),
    ],
];

/// The assembly of 32 read by the rules at each weight.
const GRID_32: [Cell; 4] = [
    Cell {
        held: 0,
        ignited: 0,
        let_go: 8,
        before: 0,
        spills: None,
    },
    Cell {
        held: 1,
        ignited: 1,
        let_go: 5,
        before: 0,
        spills: Some(false),
    },
    Cell {
        held: 3,
        ignited: 5,
        let_go: 4,
        before: 0,
        spills: Some(false),
    },
    Cell {
        held: 7,
        ignited: 5,
        let_go: 2,
        before: 0,
        spills: Some(false),
    },
];

/// The assembly of 64's rows at each weight.
const CELL_ROWS_64: [&[EpochRow]; 4] = [
    &[
        (
            [
                [1, 13, 6297, 14133],
                [0, 19, 6245, 14261],
                [5, 30, 6354, 13977],
                [2, 26, 6357, 13964],
                [1, 38, 6331, 14021],
                [6, 49, 6450, 13702],
                [1, 29, 6407, 13801],
                [8, 58, 6584, 13295],
            ],
            [0, 1],
        ),
        (
            [
                [2, 35, 6550, 13344],
                [2, 39, 6524, 13369],
                [19, 51, 7005, 12041],
                [24, 81, 7488, 10678],
                [4, 38, 7388, 10793],
                [2, 36, 7267, 10971],
                [4, 34, 7202, 11040],
                [3, 21, 7129, 11129],
            ],
            [0, 2],
        ),
        (
            [
                [225, 151, 11529, 1498],
                [1, 40, 10879, 2375],
                [1, 29, 10307, 3188],
                [2, 34, 9825, 3927],
                [2, 41, 9390, 4602],
                [3, 34, 9040, 5190],
                [4, 31, 8744, 5680],
                [9, 37, 8635, 5874],
            ],
            [63, 162],
        ),
        (
            [
                [142, 51, 11036, 1765],
                [4, 19, 10487, 2584],
                [0, 40, 9936, 3421],
                [0, 27, 9466, 4207],
                [1, 45, 9052, 4905],
                [1, 36, 8697, 5564],
                [3, 27, 8441, 6073],
                [2, 29, 8184, 6588],
            ],
            [64, 79],
        ),
        (
            [
                [2, 32, 7960, 7096],
                [0, 29, 7716, 7659],
                [3, 39, 7572, 8011],
                [1, 28, 7395, 8456],
                [1, 31, 7256, 8870],
                [3, 41, 7162, 9152],
                [1, 24, 7040, 9525],
                [3, 32, 6985, 9741],
            ],
            [0, 2],
        ),
        (
            [
                [218, 142, 11390, 1391],
                [4, 41, 10792, 2241],
                [5, 38, 10298, 2975],
                [1, 20, 9793, 3751],
                [1, 41, 9348, 4495],
                [2, 26, 8970, 5125],
                [2, 26, 8665, 5727],
                [7, 37, 8487, 6053],
            ],
            [64, 155],
        ),
        (
            [
                [155, 82, 11134, 1631],
                [4, 32, 10583, 2451],
                [0, 31, 10019, 3292],
                [1, 33, 9558, 4051],
                [3, 35, 9188, 4681],
                [3, 27, 8858, 5266],
                [6, 49, 8631, 5696],
                [5, 39, 8426, 6101],
            ],
            [64, 91],
        ),
        (
            [
                [2, 45, 8176, 6635],
                [3, 46, 7973, 7077],
                [0, 33, 7723, 7639],
                [0, 24, 7503, 8167],
                [4, 30, 7422, 8429],
                [1, 33, 7257, 8863],
                [1, 37, 7135, 9246],
                [4, 28, 7082, 9457],
            ],
            [0, 3],
        ),
        (
            [
                [202, 106, 11196, 1585],
                [1, 46, 10585, 2454],
                [1, 41, 10051, 3271],
                [0, 24, 9542, 4068],
                [9, 34, 9311, 4448],
                [2, 40, 8937, 5087],
                [1, 25, 8610, 5731],
                [2, 14, 8339, 6274],
            ],
            [64, 138],
        ),
        (
            [
                [170, 65, 11300, 1437],
                [2, 49, 10693, 2307],
                [0, 36, 10127, 3158],
                [0, 25, 9630, 3957],
                [3, 33, 9232, 4619],
                [6, 40, 8958, 5103],
                [0, 37, 8590, 5785],
                [5, 39, 8370, 6274],
            ],
            [64, 106],
        ),
        (
            [
                [2, 33, 8118, 6797],
                [2, 38, 7906, 7275],
                [1, 43, 7685, 7772],
                [2, 31, 7519, 8176],
                [0, 39, 7342, 8670],
                [1, 34, 7181, 9080],
                [6, 26, 7197, 9153],
                [3, 36, 7121, 9400],
            ],
            [0, 2],
        ),
        (
            [
                [211, 100, 11345, 1469],
                [1, 37, 10719, 2348],
                [1, 20, 10173, 3164],
                [5, 34, 9749, 3814],
                [0, 29, 9300, 4576],
                [0, 19, 8883, 5289],
                [3, 31, 8610, 5826],
                [5, 34, 8406, 6222],
            ],
            [64, 147],
        ),
        (
            [
                [160, 58, 11182, 1564],
                [1, 30, 10579, 2441],
                [0, 44, 10019, 3275],
                [0, 33, 9531, 4072],
                [3, 27, 9158, 4704],
                [3, 26, 8832, 5297],
                [1, 29, 8503, 5922],
                [2, 37, 8242, 6451],
            ],
            [64, 96],
        ),
        (
            [
                [1, 34, 7991, 6997],
                [3, 58, 7818, 7396],
                [1, 31, 7610, 7877],
                [6, 27, 7560, 8050],
                [1, 42, 7396, 8505],
                [0, 26, 7207, 8977],
                [3, 38, 7140, 9240],
                [1, 35, 7015, 9610],
            ],
            [0, 2],
        ),
        (
            [
                [208, 111, 11251, 1548],
                [4, 39, 10685, 2366],
                [4, 38, 10189, 3103],
                [5, 40, 9758, 3752],
                [3, 32, 9356, 4427],
                [2, 30, 8984, 5067],
                [1, 36, 8652, 5707],
                [2, 35, 8368, 6261],
            ],
            [64, 145],
        ),
        (
            [
                [155, 77, 11069, 1676],
                [4, 37, 10531, 2484],
                [0, 23, 9986, 3319],
                [0, 32, 9502, 4112],
                [0, 18, 9066, 4855],
                [4, 30, 8766, 5386],
                [2, 40, 8473, 5966],
                [1, 29, 8188, 6545],
            ],
            [64, 91],
        ),
        (
            [
                [3, 27, 7994, 6983],
                [7, 37, 7921, 7178],
                [2, 51, 7728, 7625],
                [2, 48, 7549, 8040],
                [2, 29, 7422, 8423],
                [8, 42, 7441, 8428],
                [0, 28, 7263, 8909],
                [5, 36, 7237, 9047],
            ],
            [0, 3],
        ),
        (
            [
                [215, 106, 11452, 1344],
                [6, 31, 10884, 2159],
                [2, 25, 10335, 2963],
                [1, 43, 9810, 3747],
                [1, 31, 9370, 4473],
                [3, 40, 9005, 5094],
                [4, 33, 8727, 5648],
                [3, 24, 8458, 6148],
            ],
            [64, 152],
        ),
        (
            [
                [142, 72, 10925, 1890],
                [6, 36, 10424, 2654],
                [0, 33, 9888, 3497],
                [0, 25, 9410, 4268],
                [1, 31, 9008, 4959],
                [0, 17, 8642, 5656],
                [2, 37, 8360, 6202],
                [7, 43, 8221, 6533],
            ],
            [64, 78],
        ),
        (
            [
                [2, 36, 7994, 7034],
                [3, 46, 7822, 7439],
                [4, 29, 7678, 7808],
                [4, 30, 7568, 8088],
                [1, 41, 7407, 8526],
                [2, 41, 7264, 8894],
                [5, 49, 7246, 9043],
                [2, 46, 7128, 9391],
            ],
            [0, 2],
        ),
        (
            [
                [208, 134, 11288, 1576],
                [2, 45, 10689, 2426],
                [2, 24, 10158, 3218],
                [2, 24, 9674, 3951],
                [1, 44, 9254, 4666],
                [1, 32, 8866, 5334],
                [3, 23, 8583, 5898],
                [3, 29, 8337, 6380],
            ],
            [64, 144],
        ),
        (
            [
                [166, 63, 11245, 1486],
                [8, 21, 10752, 2206],
                [0, 30, 10174, 3064],
                [1, 38, 9692, 3839],
                [4, 40, 9315, 4442],
                [4, 41, 8999, 4992],
                [3, 26, 8687, 5564],
                [2, 32, 8404, 6129],
            ],
            [64, 103],
        ),
        (
            [
                [5, 24, 8230, 6484],
                [0, 23, 7952, 7084],
                [5, 27, 7828, 7396],
                [4, 38, 7686, 7746],
                [4, 36, 7587, 8031],
                [3, 30, 7455, 8367],
                [1, 21, 7304, 8791],
                [3, 38, 7207, 9075],
            ],
            [1, 4],
        ),
        (
            [
                [210, 96, 11356, 1431],
                [2, 27, 10747, 2293],
                [2, 29, 10215, 3097],
                [3, 26, 9747, 3803],
                [6, 28, 9417, 4332],
                [3, 38, 9046, 4956],
                [4, 38, 8770, 5492],
                [0, 39, 8429, 6150],
            ],
            [63, 147],
        ),
        (
            [
                [146, 64, 10968, 1813],
                [3, 30, 10432, 2611],
                [0, 32, 9885, 3442],
                [0, 37, 9421, 4223],
                [1, 27, 9020, 4920],
                [1, 39, 8666, 5577],
                [4, 47, 8428, 6044],
                [6, 47, 8264, 6382],
            ],
            [64, 82],
        ),
    ],
    &[
        (
            [
                [1, 13, 6297, 14133],
                [0, 19, 6245, 14261],
                [5, 30, 6354, 13977],
                [307, 207, 12558, 540],
                [8, 54, 11862, 1440],
                [1, 41, 11163, 2335],
                [0, 28, 10539, 3186],
                [6, 60, 10089, 3801],
            ],
            [0, 1],
        ),
        (
            [
                [0, 40, 9596, 4565],
                [3, 40, 9206, 5149],
                [49, 43, 9869, 3813],
                [177, 90, 12188, 909],
                [3, 32, 11485, 1804],
                [2, 35, 10848, 2643],
                [3, 35, 10316, 3385],
                [3, 21, 9851, 4059],
            ],
            [0, 0],
        ),
        (
            [
                [180, 58, 12136, 983],
                [1, 37, 11412, 1909],
                [1, 28, 10788, 2750],
                [2, 33, 10233, 3511],
                [2, 41, 9765, 4209],
                [4, 31, 9383, 4788],
                [4, 32, 9052, 5308],
                [13, 38, 8986, 5323],
            ],
            [63, 117],
        ),
        (
            [
                [220, 55, 12241, 836],
                [2, 18, 11517, 1767],
                [0, 40, 10853, 2650],
                [0, 28, 10258, 3482],
                [1, 41, 9762, 4228],
                [1, 41, 9322, 4923],
                [4, 26, 9006, 5427],
                [2, 28, 8677, 5987],
            ],
            [63, 157],
        ),
        (
            [
                [3, 32, 8412, 6481],
                [0, 29, 8113, 7078],
                [3, 40, 7924, 7467],
                [2, 28, 7736, 7886],
                [1, 31, 7543, 8336],
                [5, 43, 7476, 8534],
                [1, 24, 7315, 8941],
                [4, 33, 7258, 9134],
            ],
            [0, 3],
        ),
        (
            [
                [275, 116, 12241, 808],
                [4, 42, 11544, 1688],
                [5, 38, 10949, 2477],
                [1, 20, 10373, 3287],
                [1, 41, 9855, 4056],
                [2, 26, 9423, 4721],
                [2, 26, 9045, 5353],
                [14, 42, 8972, 5391],
            ],
            [64, 212],
        ),
        (
            [
                [218, 74, 12204, 898],
                [5, 34, 11529, 1768],
                [0, 32, 10860, 2657],
                [1, 33, 10291, 3455],
                [3, 35, 9828, 4116],
                [5, 30, 9459, 4668],
                [5, 47, 9132, 5169],
                [35, 42, 9493, 4382],
            ],
            [64, 154],
        ),
        (
            [
                [195, 70, 12243, 837],
                [2, 49, 11529, 1743],
                [0, 33, 10856, 2632],
                [0, 24, 10277, 3461],
                [4, 29, 9823, 4099],
                [1, 33, 9376, 4808],
                [1, 37, 8985, 5460],
                [4, 28, 8698, 5939],
            ],
            [19, 176],
        ),
        (
            [
                [220, 66, 12120, 929],
                [0, 42, 11375, 1867],
                [1, 38, 10757, 2720],
                [0, 24, 10179, 3548],
                [10, 36, 9856, 3945],
                [8, 44, 9553, 4379],
                [2, 27, 9162, 5018],
                [2, 14, 8813, 5605],
            ],
            [64, 156],
        ),
        (
            [
                [224, 64, 12224, 848],
                [1, 49, 11489, 1779],
                [0, 34, 10828, 2664],
                [0, 25, 10243, 3497],
                [3, 33, 9771, 4189],
                [6, 39, 9429, 4706],
                [0, 37, 9009, 5410],
                [5, 42, 8734, 5919],
            ],
            [64, 160],
        ),
        (
            [
                [2, 31, 8434, 6469],
                [2, 38, 8178, 6966],
                [2, 43, 7954, 7426],
                [2, 34, 7761, 7848],
                [0, 39, 7545, 8365],
                [1, 35, 7371, 8790],
                [194, 71, 11527, 1152],
                [76, 81, 11881, 1206],
            ],
            [0, 2],
        ),
        (
            [
                [71, 41, 12060, 1253],
                [2, 34, 11355, 2120],
                [1, 19, 10728, 2965],
                [5, 33, 10249, 3615],
                [0, 30, 9739, 4387],
                [0, 18, 9263, 5112],
                [6, 30, 9010, 5521],
                [5, 34, 8745, 5931],
            ],
            [64, 7],
        ),
        (
            [
                [230, 57, 12247, 851],
                [1, 30, 11518, 1767],
                [0, 41, 10854, 2655],
                [0, 33, 10267, 3486],
                [4, 27, 9821, 4121],
                [3, 27, 9411, 4744],
                [1, 25, 9015, 5400],
                [2, 38, 8684, 5968],
            ],
            [64, 166],
        ),
        (
            [
                [1, 34, 8380, 6541],
                [3, 58, 8159, 6979],
                [1, 31, 7908, 7494],
                [50, 27, 8945, 5307],
                [212, 98, 12166, 894],
                [0, 29, 11426, 1830],
                [4, 40, 10831, 2614],
                [1, 33, 10272, 3415],
            ],
            [0, 2],
        ),
        (
            [
                [170, 37, 12231, 968],
                [4, 37, 11532, 1845],
                [4, 40, 10934, 2627],
                [5, 40, 10420, 3312],
                [4, 32, 9948, 3981],
                [2, 29, 9507, 4642],
                [1, 36, 9100, 5310],
                [2, 35, 8762, 5887],
            ],
            [64, 107],
        ),
        (
            [
                [215, 83, 12076, 937],
                [3, 33, 11396, 1821],
                [0, 22, 10748, 2702],
                [0, 32, 10172, 3533],
                [0, 18, 9661, 4310],
                [5, 30, 9307, 4843],
                [4, 40, 8995, 5364],
                [1, 29, 8633, 5994],
            ],
            [64, 151],
        ),
        (
            [
                [4, 27, 8404, 6435],
                [15, 39, 8476, 6224],
                [225, 85, 12212, 738],
                [2, 48, 11497, 1662],
                [2, 28, 10866, 2513],
                [6, 40, 10381, 3172],
                [1, 28, 9864, 3948],
                [5, 35, 9484, 4512],
            ],
            [0, 4],
        ),
        (
            [
                [198, 67, 12199, 899],
                [6, 32, 11528, 1753],
                [3, 25, 10917, 2553],
                [2, 42, 10363, 3329],
                [1, 30, 9845, 4082],
                [7, 38, 9508, 4565],
                [4, 33, 9152, 5147],
                [3, 24, 8842, 5686],
            ],
            [64, 135],
        ),
        (
            [
                [215, 63, 12108, 936],
                [5, 35, 11442, 1798],
                [0, 38, 10784, 2678],
                [0, 26, 10207, 3508],
                [1, 32, 9711, 4256],
                [0, 18, 9257, 4990],
                [2, 37, 8904, 5575],
                [7, 42, 8683, 5955],
            ],
            [64, 151],
        ),
        (
            [
                [4, 36, 8447, 6395],
                [3, 41, 8211, 6847],
                [6, 23, 8071, 7145],
                [18, 41, 8257, 6757],
                [250, 84, 12437, 638],
                [0, 41, 11665, 1588],
                [3, 40, 11038, 2416],
                [1, 41, 10438, 3229],
            ],
            [1, 3],
        ),
        (
            [
                [165, 55, 12256, 973],
                [3, 45, 11550, 1850],
                [2, 25, 10907, 2680],
                [3, 21, 10359, 3431],
                [1, 43, 9848, 4182],
                [1, 33, 9402, 4873],
                [7, 24, 9125, 5288],
                [5, 29, 8857, 5707],
            ],
            [64, 101],
        ),
        (
            [
                [226, 63, 12258, 840],
                [7, 19, 11609, 1665],
                [0, 29, 10936, 2558],
                [1, 41, 10355, 3367],
                [4, 41, 9902, 4014],
                [5, 41, 9525, 4543],
                [4, 26, 9173, 5112],
                [3, 32, 8846, 5648],
            ],
            [64, 162],
        ),
        (
            [
                [7, 24, 8654, 5948],
                [0, 23, 8331, 6576],
                [5, 27, 8154, 6926],
                [6, 38, 8015, 7226],
                [4, 36, 7867, 7553],
                [4, 30, 7739, 7857],
                [1, 22, 7545, 8302],
                [7, 39, 7541, 8364],
            ],
            [1, 6],
        ),
        (
            [
                [260, 91, 12164, 867],
                [2, 29, 11447, 1772],
                [2, 26, 10822, 2606],
                [5, 27, 10326, 3305],
                [6, 28, 9906, 3889],
                [3, 37, 9481, 4538],
                [4, 38, 9139, 5119],
                [0, 39, 8751, 5800],
            ],
            [64, 196],
        ),
        (
            [
                [227, 78, 12215, 863],
                [3, 30, 11513, 1750],
                [0, 32, 10850, 2638],
                [0, 37, 10265, 3469],
                [1, 27, 9768, 4213],
                [1, 39, 9322, 4915],
                [4, 47, 9003, 5421],
                [12, 46, 8885, 5541],
            ],
            [64, 163],
        ),
    ],
    &[
        (
            [
                [2, 13, 6323, 14071],
                [321, 157, 12808, 345],
                [30, 38, 12321, 1129],
                [2, 26, 11587, 2017],
                [0, 29, 10924, 2884],
                [6, 43, 10427, 3519],
                [1, 28, 9902, 4279],
                [9, 55, 9597, 4669],
            ],
            [0, 3],
        ),
        (
            [
                [230, 59, 12868, 386],
                [6, 40, 12103, 1318],
                [6, 47, 11454, 2111],
                [3, 53, 10850, 2902],
                [3, 36, 10322, 3612],
                [5, 35, 9880, 4192],
                [4, 36, 9476, 4802],
                [3, 22, 9107, 5357],
            ],
            [1, 233],
        ),
        (
            [
                [234, 66, 12437, 805],
                [1, 40, 11683, 1734],
                [1, 28, 11017, 2590],
                [2, 34, 10443, 3367],
                [2, 42, 9943, 4083],
                [4, 32, 9544, 4661],
                [4, 30, 9193, 5199],
                [147, 49, 11737, 918],
            ],
            [63, 171],
        ),
        (
            [
                [82, 34, 12095, 1081],
                [3, 20, 11402, 1972],
                [0, 40, 10751, 2846],
                [0, 29, 10180, 3663],
                [1, 43, 9684, 4400],
                [1, 40, 9256, 5077],
                [5, 28, 8960, 5529],
                [246, 57, 12805, 367],
            ],
            [50, 32],
        ),
        (
            [
                [7, 39, 12073, 1281],
                [0, 30, 11340, 2195],
                [3, 38, 10752, 2969],
                [1, 29, 10191, 3745],
                [1, 34, 9699, 4468],
                [5, 43, 9347, 4997],
                [1, 24, 8956, 5636],
                [4, 34, 8687, 6086],
            ],
            [2, 5],
        ),
        (
            [
                [264, 66, 12612, 710],
                [3, 40, 11853, 1626],
                [5, 36, 11221, 2409],
                [1, 20, 10607, 3228],
                [1, 41, 10066, 4004],
                [3, 27, 9624, 4632],
                [6, 26, 9308, 5089],
                [209, 57, 12613, 368],
            ],
            [64, 201],
        ),
        (
            [
                [60, 57, 12452, 1055],
                [4, 30, 11729, 1926],
                [0, 28, 11045, 2791],
                [1, 33, 10458, 3584],
                [3, 35, 9980, 4233],
                [11, 30, 9692, 4527],
                [242, 71, 12910, 496],
                [3, 37, 12115, 1433],
            ],
            [55, 5],
        ),
        (
            [
                [2, 42, 11404, 2315],
                [4, 45, 10820, 3048],
                [0, 31, 10237, 3861],
                [0, 26, 9711, 4613],
                [8, 31, 9423, 4989],
                [239, 55, 12864, 395],
                [6, 43, 12105, 1315],
                [2, 24, 11399, 2181],
            ],
            [0, 3],
        ),
        (
            [
                [159, 38, 12684, 893],
                [1, 38, 11895, 1814],
                [1, 38, 11193, 2672],
                [0, 24, 10575, 3506],
                [17, 37, 10338, 3646],
                [215, 59, 12836, 617],
                [1, 26, 12024, 1562],
                [2, 13, 11327, 2413],
            ],
            [64, 95],
        ),
        (
            [
                [178, 40, 12840, 799],
                [1, 42, 12030, 1733],
                [0, 35, 11303, 2621],
                [0, 25, 10661, 3458],
                [10, 33, 10281, 3891],
                [211, 55, 12997, 328],
                [9, 45, 12241, 1250],
                [4, 45, 11545, 2103],
            ],
            [64, 114],
        ),
        (
            [
                [2, 30, 10912, 2932],
                [3, 39, 10369, 3637],
                [27, 40, 10322, 3425],
                [203, 54, 12682, 723],
                [0, 37, 11879, 1674],
                [1, 33, 11185, 2545],
                [7, 27, 10677, 3182],
                [6, 35, 10217, 3762],
            ],
            [0, 2],
        ),
        (
            [
                [219, 52, 12732, 730],
                [1, 42, 11938, 1671],
                [1, 23, 11246, 2538],
                [16, 34, 10870, 2895],
                [194, 42, 12854, 649],
                [2, 24, 12056, 1583],
                [3, 31, 11364, 2418],
                [4, 34, 10784, 3143],
            ],
            [64, 155],
        ),
        (
            [
                [201, 41, 12805, 774],
                [1, 30, 11997, 1706],
                [0, 43, 11276, 2596],
                [0, 33, 10646, 3431],
                [5, 27, 10170, 4028],
                [8, 27, 9803, 4464],
                [2, 30, 9372, 5114],
                [3, 37, 9026, 5657],
            ],
            [64, 137],
        ),
        (
            [
                [3, 32, 8721, 6148],
                [254, 98, 12763, 436],
                [4, 36, 11992, 1379],
                [6, 25, 11355, 2160],
                [1, 41, 10736, 3004],
                [0, 25, 10161, 3812],
                [5, 37, 9743, 4375],
                [2, 33, 9325, 5021],
            ],
            [0, 4],
        ),
        (
            [
                [233, 49, 12513, 776],
                [4, 36, 11779, 1675],
                [4, 39, 11146, 2471],
                [9, 39, 10675, 3061],
                [6, 32, 10204, 3700],
                [2, 30, 9730, 4382],
                [1, 37, 9296, 5061],
                [2, 35, 8932, 5670],
            ],
            [63, 171],
        ),
        (
            [
                [249, 79, 12546, 743],
                [3, 36, 11807, 1646],
                [0, 24, 11109, 2536],
                [0, 32, 10489, 3374],
                [0, 18, 9942, 4161],
                [211, 45, 12866, 331],
                [15, 54, 12207, 1213],
                [1, 32, 11467, 2114],
            ],
            [64, 185],
        ),
        (
            [
                [3, 27, 10862, 2892],
                [15, 38, 10541, 3202],
                [194, 61, 12769, 600],
                [4, 42, 11997, 1527],
                [2, 28, 11305, 2384],
                [8, 41, 10797, 2993],
                [0, 26, 10220, 3803],
                [5, 34, 9798, 4370],
            ],
            [0, 3],
        ),
        (
            [
                [220, 64, 12558, 797],
                [6, 31, 11841, 1666],
                [3, 24, 11195, 2482],
                [1, 43, 10573, 3291],
                [1, 30, 10045, 4042],
                [219, 53, 12963, 325],
                [10, 34, 12237, 1231],
                [3, 24, 11533, 2098],
            ],
            [64, 157],
        ),
        (
            [
                [163, 35, 12806, 829],
                [6, 32, 12060, 1683],
                [0, 32, 11334, 2575],
                [0, 25, 10688, 3411],
                [1, 31, 10138, 4156],
                [0, 18, 9635, 4894],
                [2, 37, 9230, 5492],
                [173, 61, 12117, 682],
            ],
            [64, 99],
        ),
        (
            [
                [76, 58, 12284, 1060],
                [3, 39, 11566, 1937],
                [5, 22, 10962, 2711],
                [5, 35, 10457, 3376],
                [2, 41, 9947, 4100],
                [2, 42, 9496, 4775],
                [7, 46, 9216, 5162],
                [240, 64, 12782, 444],
            ],
            [41, 35],
        ),
        (
            [
                [67, 46, 12640, 1047],
                [1, 38, 11853, 1983],
                [2, 24, 11179, 2781],
                [3, 21, 10602, 3522],
                [1, 43, 10061, 4259],
                [1, 33, 9585, 4944],
                [69, 25, 10587, 2634],
                [172, 59, 12446, 870],
            ],
            [64, 3],
        ),
        (
            [
                [69, 33, 12412, 1164],
                [7, 16, 11739, 1936],
                [0, 30, 11056, 2828],
                [1, 41, 10467, 3615],
                [4, 40, 9985, 4233],
                [46, 42, 10431, 3071],
                [187, 47, 12535, 818],
                [2, 34, 11775, 1737],
            ],
            [64, 5],
        ),
        (
            [
                [6, 25, 11162, 2487],
                [0, 23, 10542, 3329],
                [5, 26, 10076, 3940],
                [7, 38, 9708, 4436],
                [4, 36, 9327, 4977],
                [4, 29, 9020, 5491],
                [1, 21, 8662, 6096],
                [9, 39, 8557, 6240],
            ],
            [1, 5],
        ),
        (
            [
                [257, 69, 12503, 752],
                [1, 25, 11742, 1680],
                [2, 26, 11076, 2521],
                [4, 27, 10530, 3250],
                [13, 28, 10225, 3590],
                [217, 57, 12886, 500],
                [1, 35, 12077, 1451],
                [0, 36, 11352, 2355],
            ],
            [64, 193],
        ),
        (
            [
                [174, 49, 12803, 832],
                [3, 28, 12023, 1717],
                [0, 33, 11301, 2603],
                [0, 36, 10665, 3438],
                [1, 27, 10121, 4184],
                [1, 39, 9633, 4876],
                [7, 47, 9335, 5258],
                [244, 70, 12744, 583],
            ],
            [64, 110],
        ),
    ],
    &[
        (
            [
                [4, 13, 6381, 13925],
                [358, 192, 12978, 466],
                [4, 30, 12185, 1384],
                [1, 26, 11462, 2266],
                [1, 29, 10819, 3092],
                [14, 43, 10488, 3422],
                [225, 45, 12973, 621],
                [7, 61, 12209, 1499],
            ],
            [0, 20],
        ),
        (
            [
                [0, 39, 11468, 2398],
                [11, 42, 10976, 2924],
                [215, 54, 13155, 505],
                [2, 53, 12319, 1450],
                [2, 34, 11585, 2311],
                [2, 34, 10953, 3108],
                [8, 36, 10478, 3673],
                [85, 26, 11487, 1507],
            ],
            [0, 0],
        ),
        (
            [
                [148, 49, 12657, 861],
                [1, 42, 11873, 1787],
                [1, 23, 11187, 2641],
                [2, 32, 10596, 3405],
                [2, 42, 10079, 4112],
                [177, 36, 12570, 435],
                [82, 44, 12659, 937],
                [12, 38, 12019, 1647],
            ],
            [57, 91],
        ),
        (
            [
                [173, 29, 13143, 737],
                [2, 18, 12307, 1672],
                [0, 41, 11552, 2565],
                [0, 28, 10880, 3398],
                [1, 43, 10312, 4149],
                [1, 42, 9808, 4840],
                [6, 28, 9456, 5254],
                [259, 67, 12932, 550],
            ],
            [63, 110],
        ),
        (
            [
                [3, 37, 12125, 1481],
                [0, 31, 11389, 2383],
                [3, 38, 10790, 3139],
                [2, 28, 10242, 3867],
                [1, 28, 9740, 4583],
                [246, 68, 13119, 254],
                [10, 25, 12382, 1176],
                [3, 34, 11658, 2031],
            ],
            [2, 1],
        ),
        (
            [
                [185, 43, 13051, 781],
                [4, 38, 12236, 1673],
                [7, 34, 11583, 2399],
                [1, 20, 10934, 3217],
                [1, 41, 10348, 3989],
                [8, 27, 9977, 4417],
                [242, 48, 12946, 576],
                [6, 40, 12181, 1466],
            ],
            [64, 122],
        ),
        (
            [
                [153, 54, 13039, 838],
                [4, 30, 12240, 1723],
                [0, 28, 11491, 2602],
                [1, 33, 10845, 3402],
                [3, 35, 10320, 4065],
                [220, 45, 13137, 241],
                [27, 45, 12556, 1083],
                [4, 37, 11816, 1948],
            ],
            [64, 89],
        ),
        (
            [
                [2, 45, 11140, 2788],
                [6, 41, 10620, 3408],
                [219, 41, 13224, 256],
                [12, 30, 12475, 1170],
                [4, 32, 11755, 2012],
                [1, 30, 11069, 2864],
                [0, 37, 10464, 3678],
                [5, 27, 10008, 4258],
            ],
            [0, 3],
        ),
        (
            [
                [235, 48, 12811, 738],
                [1, 47, 12004, 1668],
                [1, 37, 11300, 2531],
                [0, 23, 10661, 3374],
                [151, 39, 12565, 475],
                [86, 51, 12678, 950],
                [1, 23, 11889, 1871],
                [2, 13, 11216, 2703],
            ],
            [64, 171],
        ),
        (
            [
                [197, 40, 12943, 777],
                [0, 43, 12116, 1725],
                [0, 34, 11378, 2615],
                [0, 25, 10741, 3448],
                [128, 39, 12299, 682],
                [114, 48, 12781, 879],
                [0, 38, 11968, 1811],
                [5, 43, 11312, 2611],
            ],
            [64, 133],
        ),
        (
            [
                [2, 31, 10702, 3398],
                [4, 38, 10207, 4028],
                [238, 53, 13051, 492],
                [3, 32, 12239, 1426],
                [0, 37, 11490, 2332],
                [1, 35, 10842, 3154],
                [212, 40, 13220, 304],
                [11, 41, 12459, 1211],
            ],
            [0, 2],
        ),
        (
            [
                [148, 33, 13146, 795],
                [3, 40, 12315, 1703],
                [1, 20, 11575, 2554],
                [47, 34, 11647, 1955],
                [168, 36, 12926, 803],
                [1, 21, 12109, 1725],
                [7, 31, 11463, 2445],
                [5, 34, 10888, 3135],
            ],
            [64, 84],
        ),
        (
            [
                [217, 44, 12985, 739],
                [1, 30, 12156, 1664],
                [0, 43, 11417, 2557],
                [0, 35, 10765, 3394],
                [5, 27, 10273, 3991],
                [240, 42, 13158, 415],
                [4, 37, 12340, 1342],
                [2, 32, 11602, 2215],
            ],
            [64, 153],
        ),
        (
            [
                [1, 36, 10949, 3042],
                [3, 58, 10403, 3752],
                [1, 31, 9892, 4474],
                [171, 31, 12410, 536],
                [86, 58, 12595, 965],
                [0, 31, 11809, 1895],
                [5, 39, 11183, 2644],
                [2, 32, 10593, 3407],
            ],
            [0, 2],
        ),
        (
            [
                [216, 37, 12851, 753],
                [4, 33, 12077, 1648],
                [5, 40, 11423, 2421],
                [207, 48, 13415, 289],
                [11, 33, 12618, 1214],
                [2, 31, 11841, 2088],
                [1, 36, 11166, 2923],
                [2, 35, 10572, 3676],
            ],
            [63, 154],
        ),
        (
            [
                [219, 51, 12861, 757],
                [4, 35, 12088, 1634],
                [0, 23, 11365, 2530],
                [0, 32, 10712, 3371],
                [0, 18, 10147, 4155],
                [239, 47, 13120, 385],
                [7, 49, 12336, 1303],
                [1, 30, 11585, 2192],
            ],
            [64, 155],
        ),
        (
            [
                [6, 26, 11011, 2873],
                [214, 42, 13217, 427],
                [3, 53, 12386, 1377],
                [2, 50, 11646, 2243],
                [2, 29, 10999, 3042],
                [13, 40, 10624, 3396],
                [223, 35, 13004, 605],
                [6, 35, 12237, 1492],
            ],
            [0, 7],
        ),
        (
            [
                [156, 49, 13081, 838],
                [6, 32, 12301, 1691],
                [3, 25, 11588, 2494],
                [1, 42, 10927, 3308],
                [1, 31, 10351, 4053],
                [242, 49, 13185, 415],
                [4, 33, 12367, 1347],
                [3, 22, 11635, 2205],
            ],
            [64, 93],
        ),
        (
            [
                [186, 35, 13063, 776],
                [6, 31, 12282, 1638],
                [0, 32, 11530, 2526],
                [0, 26, 10866, 3367],
                [2, 31, 10302, 4075],
                [0, 19, 9782, 4827],
                [3, 37, 9385, 5372],
                [263, 71, 13130, 299],
            ],
            [64, 122],
        ),
        (
            [
                [6, 44, 12329, 1235],
                [3, 40, 11613, 2094],
                [6, 22, 11029, 2820],
                [51, 36, 11310, 1992],
                [178, 46, 12841, 774],
                [0, 38, 12016, 1721],
                [5, 48, 11369, 2489],
                [197, 49, 13330, 271],
            ],
            [3, 3],
        ),
        (
            [
                [68, 45, 13032, 990],
                [1, 37, 12203, 1928],
                [5, 25, 11527, 2646],
                [3, 21, 10904, 3389],
                [2, 43, 10338, 4113],
                [1, 33, 9830, 4827],
                [219, 36, 12931, 293],
                [39, 37, 12506, 1056],
            ],
            [62, 6],
        ),
        (
            [
                [126, 30, 12990, 873],
                [7, 16, 12240, 1694],
                [1, 30, 11501, 2571],
                [1, 41, 10854, 3369],
                [5, 40, 10357, 3969],
                [242, 71, 13202, 401],
                [6, 35, 12401, 1328],
                [2, 32, 11656, 2204],
            ],
            [64, 62],
        ),
        (
            [
                [8, 23, 11096, 2832],
                [0, 23, 10477, 3656],
                [5, 27, 10036, 4234],
                [259, 57, 13172, 444],
                [3, 37, 12345, 1380],
                [2, 29, 11608, 2249],
                [1, 22, 10949, 3068],
                [7, 38, 10475, 3621],
            ],
            [1, 7],
        ),
        (
            [
                [232, 44, 12949, 714],
                [1, 26, 12126, 1642],
                [2, 26, 11419, 2489],
                [4, 27, 10824, 3222],
                [35, 28, 10868, 2806],
                [200, 50, 12851, 745],
                [1, 36, 12044, 1677],
                [0, 38, 11318, 2562],
            ],
            [64, 168],
        ),
        (
            [
                [201, 51, 13036, 747],
                [3, 28, 12232, 1639],
                [0, 33, 11482, 2535],
                [0, 35, 10827, 3371],
                [1, 27, 10262, 4119],
                [6, 39, 9852, 4611],
                [247, 61, 12958, 560],
                [6, 47, 12195, 1469],
            ],
            [64, 137],
        ),
    ],
];

/// The assembly of 64 read by the rules at each weight.
const GRID_64: [Cell; 4] = [
    Cell {
        held: 0,
        ignited: 0,
        let_go: 8,
        before: 0,
        spills: None,
    },
    Cell {
        held: 0,
        ignited: 3,
        let_go: 7,
        before: 0,
        spills: None,
    },
    Cell {
        held: 7,
        ignited: 2,
        let_go: 1,
        before: 0,
        spills: Some(false),
    },
    Cell {
        held: 7,
        ignited: 6,
        let_go: 0,
        before: 0,
        spills: Some(false),
    },
];

// ------------------------------------------------ the arithmetic, pinned before any run

/// The pair a spike after a long rest releases with.
const STP_AT_REST: (u8, u8) = (92, 255);

/// The steady pairs at `INTERVALS`.
const STP_STEADY: [(u8, u8); 8] = [
    (93, 237),
    (105, 172),
    (122, 110),
    (149, 57),
    (182, 26),
    (214, 10),
    (234, 4),
    (242, 2),
];

/// The steady factor at 20 Hz per mille of the factor at rest.
const STEADY_20_HZ_PER_MILLE: u64 = 362;

/// One weight's delivery: one spike's delivery after the gain at rest, the soma's peak it
/// raises a unit at the drive's mean standing to, the least count of such spikes together
/// that fires it; the same at the steady pair of 20 Hz.
type Delivered = (i32, i32, Option<u32>, i32, i32, Option<u32>);

/// Per weight: one spike's delivery after the gain at rest, the soma's peak it raises a unit
/// at the drive's mean standing to, the least count of such spikes together that fires it;
/// the same at the steady pair of 20 Hz.
const DELIVERED: [Delivered; 4] = [
    (10264, 33367, Some(8), 3715, 30304, Some(22)),
    (20528, 38177, Some(4), 7431, 32043, Some(11)),
    (30791, 42985, Some(3), 11146, 33782, Some(8)),
    (41053, 47788, Some(2), 14861, 35523, Some(6)),
];

/// Per size and weight: the members' mean input to a member, basal and somatic, at the
/// background's interval, at 20 Hz and at 100 Hz.
const RECURRENT: [[[(i64, i64); 3]; 4]; 3] = [
    [
        [(1303, 648), (5706, 2841), (7188, 3580)],
        [(2606, 1297), (11414, 5684), (14384, 7164)],
        [(3910, 1947), (17120, 8526), (21573, 10744)],
        [(5213, 2596), (22826, 11368), (28746, 14317)],
    ],
    [
        [(2693, 1341), (11792, 5873), (14856, 7399)],
        [(5387, 2683), (23588, 11748), (29728, 14806)],
        [(8080, 4024), (35381, 17621), (44584, 22205)],
        [(10774, 5366), (47174, 23495), (59408, 29588)],
    ],
    [
        [(2780, 1384), (12173, 6062), (15335, 7637)],
        [(5561, 2769), (24349, 12127), (30687, 15283)],
        [(8341, 4154), (36523, 18190), (46022, 22921)],
        [(11122, 5539), (48696, 24253), (61325, 30543)],
    ],
];

/// A kick by the oracle as pinned: `KickRead` with its ticks borrowed.
type KickPinned = (&'static [u32], i32, i32, i32, i32, i32);

/// The kick by the oracle from each standing: the ticks it fires on, its basal potential on
/// the tick it fires, its somatic and basal potentials on the last tick of its refractory
/// window, its basal potential on the tick the reset lands, and its soma 64 ticks after.
const KICK_ORACLE: [KickPinned; 5] = [
    (&[82], 144438, 49369, 97779, 57343, 28563),
    (&[51], 144439, 49369, 97780, 57344, 28563),
    (&[1], 132720, 45365, 89853, 49432, 25047),
    (&[115], 143968, 49203, 97456, 57020, 28432),
    (&[146], 143553, 49064, 97180, 56745, 28317),
];

/// The release by the oracle at the drive's mean standing and at the extreme: the ticks it
/// fires on, its lowest basal potential, the tick its soma is back within a tenth of the
/// threshold of the drive's mean standing.
const RELEASE_ORACLE: [(&[u32], i32, Option<u32>); 2] =
    [(&[], -12232635, Some(3510)), (&[], -12160192, Some(3507))];
