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

// The tables the rounds pinned from their runs, a file a round and a row a line (ADR-0227).
#[rustfmt::skip]
#[path = "assembly/pins/mod.rs"]
mod pins;
use pins::*;
// The harness holds a table of this name too; this binary's is the round's pin.
use pins::BACKGROUND_ROWS_1024;

use menteion_connectome::{SECTION_NEURON, SECTION_SYNAPSE};
use menteion_core::{
    BASAL_LEAK_SHIFT, FLAG_FACILITATING, FLAG_SLOW, STP_MAX, STP_U, SYNAPSES_PER_BLOCK,
    SlowCurrent, StpClass, V_RESET, synaptic_efficacy_q16,
};
use menteion_runtime::{Inject, mix64};

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
    delay_drawn(DELAY_SEED, from, to)
}

/// `delay_of` drawn by `mix64` of `seed` in place of `DELAY_SEED` (brief 050's condition (b)).
fn delay_drawn(seed: u64, from: u32, to: u32) -> u16 {
    let p = prior(1024);
    let width = u64::from(p.delay_max.saturating_sub(p.delay_min)).saturating_add(1);
    let draw = mix64(seed ^ (u64::from(from) << 32 | u64::from(to)));
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
    fires_every_member_once(kick_reading(rows, size), size)
}

/// `kicked_once`'s measure over a kick's reading `(volley, after, kicks, full)`, shared with
/// brief 049's reading over rounds.
fn fires_every_member_once((volley, after, kicks, _): (u64, u64, u64, u32), size: u32) -> bool {
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

/// The membrane rule stepped alone, as the harness's `alone` steps it: `menteion-core`'s
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

/// One spike of `unit` after `elapsed` ticks, as the executor's turn steps it (ADR-0114):
/// under `class` when the unit is marked facilitating and a class is set, under ADR-0019's
/// constants (`step_stp`) otherwise.
fn stepped_pair(
    unit: &mut DendriticSuperNeuron,
    elapsed: u32,
    class: Option<StpClass>,
) -> (u8, u8) {
    match class {
        Some(c) if unit.flags & FLAG_FACILITATING != 0 => unit.step_stp_class(elapsed, c),
        _ => unit.step_stp(elapsed),
    }
}

/// The pairs `(u, R)` a unit's spikes release with when it fires every `interval` ticks from
/// short-term plasticity at rest, the first spike after a rest of `u32::MAX` ticks as the
/// executor reads a first spike: `step_stp` itself on a unit whose factors are the prior's at
/// rest when `class` is none, and since brief 049 the class's step on a unit marked
/// facilitating from the class's rest when it is set (`stepped_pair`).
fn stp_course(interval: u32, spikes: usize, class: Option<StpClass>) -> Vec<(u8, u8)> {
    course_unit(interval, spikes, class).0
}

/// `stp_course_under`'s pairs and the unit its last spike leaves.
fn course_unit(
    interval: u32,
    spikes: usize,
    class: Option<StpClass>,
) -> (Vec<(u8, u8)>, DendriticSuperNeuron) {
    let mut unit = DendriticSuperNeuron::new(0);
    unit.stp_u_rel = class.map_or(STP_U, |c| c.u);
    unit.stp_r_ves = STP_MAX;
    if class.is_some() {
        unit.flags = FLAG_FACILITATING;
    }
    let mut elapsed = u32::MAX;
    let mut out = Vec::with_capacity(spikes);
    for _ in 0..spikes {
        out.push(stepped_pair(&mut unit, elapsed, class));
        elapsed = interval;
    }
    (out, unit)
}

/// The spikes a course runs before its pair is read as steady.
const STP_SPIKES: usize = 256;

/// The steady pair at `interval`: the last of a course of `STP_SPIKES` spikes, whose last two
/// pairs are equal (asserted: the integer rule reaches a fixed point).
fn steady(interval: u32) -> (u8, u8) {
    steady_under(interval, None)
}

/// `steady` for a member marked facilitating under `class`, or unmarked with none.
fn steady_under(interval: u32, class: Option<StpClass>) -> (u8, u8) {
    let course = stp_course(interval, STP_SPIKES, class);
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
    at_rest_under(None)
}

/// `at_rest` for a member marked facilitating under `class`, or unmarked with none.
fn at_rest_under(class: Option<StpClass>) -> (u8, u8) {
    stp_course(0, 1, class)[0]
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
    let header = MenteionFileHeader::decode(image[0..64].try_into().unwrap());
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
    let sealed = MenteionFileHeader::new(
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
    wire_drawn(exec, size, weight, DELAY_SEED);
}

/// `wire` with the delays drawn by `seed` (`delay_drawn`); every target, weight and block the
/// same.
fn wire_drawn(exec: &mut Engine, size: u32, weight: i16, seed: u64) {
    let base = blocks_for(&prior(1024)) as u32;
    assert_eq!(
        exec.blocks().len() as u64,
        u64::from(base).saturating_add(extra_blocks(size)),
        "the arena holds the prior's blocks and the assembly's"
    );
    wire_among(exec, size, weight, seed);
}

/// `wire_drawn` into an arena that holds the prior's blocks, the assembly's and possibly more
/// after them (brief 051's inhibition): the assembly's blocks are the first after the prior's.
fn wire_among(exec: &mut Engine, size: u32, weight: i16, seed: u64) {
    let base = blocks_for(&prior(1024)) as u32;
    let m = members(size);
    let per = blocks_per_member(size);
    let (units, blocks) = exec.arenas_mut();
    assert!(
        blocks.len() as u64 >= u64::from(base).saturating_add(extra_blocks(size)),
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
                    delay_drawn(seed, unit, target),
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

/// The sums over `set` of the pair a spike on tick `now` would release with: the step on a
/// copy of each member's two factors, with the ticks since its last spike as the executor
/// reads them, under the image's `class` for a member marked facilitating (`next_pair`,
/// ADR-0114) and `step_stp` otherwise.
fn stp_sums(
    units: &[DendriticSuperNeuron],
    set: &[u32],
    now: u32,
    class: Option<StpClass>,
) -> (u32, u32) {
    let (mut su, mut sr) = (0u32, 0u32);
    for &m in set {
        let (u, r) = next_pair(&units[m as usize], now, class);
        su = su.saturating_add(u32::from(u));
        sr = sr.saturating_add(u32::from(r));
    }
    (su, sr)
}

/// The pair `unit`'s spike on tick `now` would release with, on a copy of its factors, as the
/// executor's turn would step it (`stepped_pair`), with the ticks since its last spike.
fn next_pair(unit: &DendriticSuperNeuron, now: u32, class: Option<StpClass>) -> (u8, u8) {
    let mut copy = DendriticSuperNeuron::new(0);
    copy.stp_u_rel = unit.stp_u_rel;
    copy.stp_r_ves = unit.stp_r_ves;
    copy.flags = unit.flags;
    let elapsed = if unit.last_soma_spike_tick == NO_SPIKE_ON_RECORD {
        u32::MAX
    } else {
        now.wrapping_sub(unit.last_soma_spike_tick)
    };
    stepped_pair(&mut copy, elapsed, class)
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
                let (su, sr) = stp_sums(exec.units(), set, now, exec.stp_class());
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
                .all(|b| *b == menteion_core::SynapseBlock::new()),
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
fn an_assembly_of_16_units_at_four_weights_paused() {
    four_weights(0);
}

#[test]
#[ignore]
fn an_assembly_of_32_units_at_four_weights_paused() {
    four_weights(1);
}

#[test]
#[ignore]
fn an_assembly_of_64_units_at_four_weights_paused() {
    four_weights(2);
}

// ------------------------------------------------ the readings, pinned from the runs

/// Whether the kick fires every member once, per size.
const KICKED_ONCE_1024: [bool; 3] = [true, true, true];

/// Each cell's rows, by size and weight.
const CELL_ROWS_1024: [[&[EpochRow]; 4]; 3] = [CELL_ROWS_16, CELL_ROWS_32, CELL_ROWS_64];

/// Each cell read by the rules, by size and weight.
const GRID_1024: [[Cell; 4]; 3] = [GRID_16, GRID_32, GRID_64];

/// A kick by the oracle: the ticks the unit fires on, its basal potential on the tick it fires,
/// its somatic and basal potentials on the last tick of its refractory window, its basal
/// potential on the tick the reset lands, and its soma `SETTLED_AFTER_RESET` ticks after.
type KickRead = (Vec<u32>, i32, i32, i32, i32, i32);

// ------------------------------------------------ the arithmetic, pinned before any run

/// The pair a spike after a long rest releases with.
const STP_AT_REST: (u8, u8) = (92, 255);

/// The steady factor at 20 Hz per mille of the factor at rest.
const STEADY_20_HZ_PER_MILLE: u64 = 362;

/// One weight's delivery: one spike's delivery after the gain at rest, the soma's peak it
/// raises a unit at the drive's mean standing to, the least count of such spikes together
/// that fires it; the same at the steady pair of 20 Hz.
type Delivered = (i32, i32, Option<u32>, i32, i32, Option<u32>);

/// A kick by the oracle as pinned: `KickRead` with its ticks borrowed.
type KickPinned = (&'static [u32], i32, i32, i32, i32, i32);

// ================================================ brief 049 (ADR-0115): the facilitating class

// ------------------------------------------------ the grid (brief 049), before any run

/// ADR-0113's two sets of the class, `(U, τ_f shift, τ_d shift)`: (i) $(51, 16, 13)$, ADR-0019's
/// release fraction with facilitation over $2^{16}$ ticks, the longest the factor resolves, and
/// depression over $2^{13}$; (ii) $(26, 16, 13)$, half the release fraction.
const SETS: [StpClass; 2] = [
    StpClass {
        u: 51,
        tau_f_shift: 16,
        tau_d_shift: 13,
    },
    StpClass {
        u: 26,
        tau_f_shift: 16,
        tau_d_shift: 13,
    },
];

const _: () = assert!(SETS[0].is_valid() && SETS[1].is_valid());

/// The sets' names, as ADR-0113 numbers them.
const SET_NAMES: [&str; 2] = ["(i)", "(ii)"];

/// Brief 049's weights, Q1.15: a quarter to three quarters of the range by eighths, and its top,
/// `i16::MAX`. ADR-0112's four are among them.
const WEIGHTS_049: [i16; 6] = [0x2000, 0x3000, 0x4000, 0x5000, 0x6000, i16::MAX];

// ------------------------------------------------ the spans (brief 049), before any run

/// A span of a run (brief 049).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Span {
    /// The drive alone, read by no rule.
    LeadIn,
    /// No kick and no release.
    Unkicked,
    /// ADR-0112's kick, its ramp and its reset, from the span's first tick.
    Hold,
    /// The members kept from firing: ADR-0076's cancel at the start of every window of the span
    /// but its last two (`cancel_due`).
    Release,
    /// The drive alone, after the release.
    Tail,
}

/// The lead-in, in epochs: sixteen, $2^{18}$ ticks, four of the class's $\tau_f$, over which the
/// short-term state the image's members carry from ADR-0019's constants relaxes to within two
/// per cent of the class's own.
const LEAD_IN_EPOCHS_049: u32 = 16;

/// The unkicked span, in epochs: the brief's sixteen.
const UNKICKED_EPOCHS: u32 = 16;

/// The hold span, in epochs: the brief's sixteen. By ADR-0113's arithmetic a kick's priming has
/// faded below the unkicked product by $2^{17}$ ticks, eight epochs, so a hold read in its
/// second half is not what the kick left fading.
const HOLD_EPOCHS: u32 = 16;

/// The tail, in epochs: four, two stretches, opening two windows after the release's last cancel,
/// past `RELEASE_RECOVERED`.
const TAIL_EPOCHS: u32 = 4;

/// A stretch: two consecutive epochs of a span, from its first, 32 768 ticks, sixteen windows.
const STRETCH_EPOCHS: u32 = 2;

const STRETCH_WINDOWS: usize = 16;

const STRETCH_TICKS: u64 = 32_768;

const _: () = assert!(STRETCH_TICKS == (STRETCH_EPOCHS * EPOCH_TICKS) as u64);
const _: () = assert!(STRETCH_WINDOWS == STRETCH_EPOCHS as usize * WINDOWS);
const _: () = assert!(
    LEAD_IN_EPOCHS_049 % STRETCH_EPOCHS == 0
        && UNKICKED_EPOCHS % STRETCH_EPOCHS == 0
        && HOLD_EPOCHS % STRETCH_EPOCHS == 0
        && TAIL_EPOCHS % STRETCH_EPOCHS == 0
);

/// The hold span's stretches, and the first of its second half.
const HOLD_STRETCHES: u32 = HOLD_EPOCHS / STRETCH_EPOCHS;

const SECOND_HALF_FROM: u32 = HOLD_STRETCHES / 2;

const _: () = assert!(HOLD_STRETCHES == 8 && SECOND_HALF_FROM == 4);

/// A run's spans in order, `(span, round, epochs)`: the lead-in, then `ROUNDS` rounds of an
/// unkicked span, a hold span, a release span of `release` stretches and a tail.
fn layout(release: u32) -> Vec<(Span, usize, u32)> {
    let mut out = vec![(Span::LeadIn, 0, LEAD_IN_EPOCHS_049)];
    for round in 0..ROUNDS {
        out.push((Span::Unkicked, round, UNKICKED_EPOCHS));
        out.push((Span::Hold, round, HOLD_EPOCHS));
        out.push((Span::Release, round, release.saturating_mul(STRETCH_EPOCHS)));
        out.push((Span::Tail, round, TAIL_EPOCHS));
    }
    out
}

/// A run's stretches in order: `(span, round, the stretch's index within its span)`.
fn stretches_of(spans: &[(Span, usize, u32)]) -> Vec<(Span, usize, u32)> {
    spans
        .iter()
        .flat_map(|&(span, round, epochs)| {
            (0..epochs.checked_div(STRETCH_EPOCHS).unwrap_or(0)).map(move |k| (span, round, k))
        })
        .collect()
}

/// The release's cancel is due before the span's tick `k` of a span of `windows` windows: its
/// first `CANCEL_TICKS` ticks of every window but the last two.
fn cancel_due(k: u32, windows: u32) -> bool {
    let w = k.checked_div(WINDOW_SPAN).unwrap_or(0);
    let j = k.checked_rem(WINDOW_SPAN).unwrap_or(0);
    w.saturating_add(2) < windows && j < CANCEL_TICKS
}

// ------------------------------------------------ the release, derived from the class

/// The ceiling of both factors: the pair from which a silence leaves the most a spike can
/// release with, since the pair after any silence is non-decreasing in each factor it starts
/// from (relaxation toward a target keeps the order, and facilitation $u + U(1 - u)$ grows with
/// $u$).
const CEILING: (u8, u8) = (STP_MAX, STP_MAX);

/// The pair a member marked under `class` releases with after `silence` ticks from `state`.
fn after_silence(state: (u8, u8), silence: u32, class: StpClass) -> (u8, u8) {
    let mut unit = DendriticSuperNeuron::new(0);
    (unit.stp_u_rel, unit.stp_r_ves) = state;
    unit.step_stp_class(silence, class)
}

/// The unkicked product under `class`: $uR$ of the steady pair at the background's interval,
/// ADR-0113's reference.
fn unkicked_product(class: StpClass) -> u32 {
    factor(steady_under(INTERVALS[0], Some(class)))
}

/// The longest silence the fade is searched over, $2^{20}$ ticks.
const FADE_HORIZON: u32 = 1 << 20;

/// The fade under `class`: the least silence after which a member at the ceiling of both
/// factors releases with a product below the unkicked one. From the ceiling the pool stays full
/// and $u$ only relaxes, so the product is non-increasing in the silence and every longer silence
/// is below it too; and from any other pair it is lower still. So a member kept silent this long
/// is unprimed, whatever its spikes before.
fn fade(class: StpClass) -> Option<u32> {
    let unkicked = unkicked_product(class);
    (0..=FADE_HORIZON).find(|&t| factor(after_silence(CEILING, t, class)) < unkicked)
}

/// The most stretches the release's rule searches.
const RELEASE_STRETCHES_MAX: u32 = 16;

/// The silence a release of `stretches` stretches guarantees a member: from the span's second
/// window, whose cancel catches a member the first found refractory, to `RELEASE_RECOVERED`
/// after its last cancel, at the start of its third window from the end: $(16S - 4) \cdot 2\,048
/// + 3\,510$ ticks.
fn silence_of(stretches: u32) -> u64 {
    u64::from(stretches)
        .saturating_mul(STRETCH_WINDOWS as u64)
        .saturating_sub(4)
        .saturating_mul(u64::from(WINDOW_SPAN))
        .saturating_add(u64::from(RELEASE_RECOVERED))
}

/// The release's span under `class`, in stretches: the least whole number whose silence is at
/// least the fade.
fn release_rule(class: StpClass) -> Option<u32> {
    let need = u64::from(fade(class)?);
    (1..=RELEASE_STRETCHES_MAX).find(|&s| silence_of(s) >= need)
}

/// The release by the oracle over a span of `stretches` stretches, from `standing`, under the
/// drive's mean input and the cancel at the start of every window but the last two: the ticks it
/// fires on, its lowest basal potential, and the first tick after the span's last cancel at which
/// its soma is back within a tenth of the threshold of the drive's mean standing.
fn release_span_oracle(standing: (i32, i32), stretches: u32) -> (Vec<u32>, i32, Option<u32>) {
    let mean = drive_mean_per_tick();
    let per = scaled_q16(
        batch_q16(CANCEL_AT_THE_EXTREME, CANCEL_MESSAGE_Q16),
        GAIN_1024,
    );
    let windows = stretches.saturating_mul(STRETCH_WINDOWS as u32);
    let ticks = windows.saturating_mul(WINDOW_SPAN);
    let s = stepped(
        standing,
        |k| {
            if cancel_due(k, windows) {
                mean.saturating_add(per)
            } else {
                mean
            }
        },
        ticks,
    );
    let last = windows.saturating_sub(3).saturating_mul(WINDOW_SPAN) as usize;
    let near = DRIVE_STANDING.1.saturating_sub(RECOVERED_WITHIN);
    let back = s.soma[last..]
        .iter()
        .position(|&v| v > near)
        .map(|k| k.saturating_add(1) as u32);
    (s.fires, s.basal.iter().copied().min().unwrap_or(0), back)
}

// ------------------------------------------------ the arithmetic (brief 049), before any run

/// The ticks after a burst the course is read at: ADR-0113's.
const COURSE_TICKS: [u32; 8] = [220, 2_048, 8_192, 16_384, 32_768, 65_536, 131_072, 262_144];

/// The burst of ADR-0113's arithmetic: four spikes 220 ticks apart, the first one background
/// interval after the unkicked steady state's last spike.
const BURST_SPIKES: usize = 4;

const BURST_INTERVAL: u32 = 220;

/// The ticks after the burst the primed window is searched over, $2^{18}$.
const COURSE_HORIZON: u32 = 1 << 18;

/// A member marked under `class` after ADR-0113's burst: the unkicked steady course, then the
/// burst.
fn after_burst(class: StpClass) -> DendriticSuperNeuron {
    let (_, mut unit) = course_unit(INTERVALS[0], STP_SPIKES, Some(class));
    let mut elapsed = INTERVALS[0];
    for _ in 0..BURST_SPIKES {
        stepped_pair(&mut unit, elapsed, Some(class));
        elapsed = BURST_INTERVAL;
    }
    unit
}

/// The pair a member's next spike releases with `t` ticks after ADR-0113's burst.
fn after_burst_at(class: StpClass, t: u32) -> (u8, u8) {
    let unit = after_burst(class);
    after_silence((unit.stp_u_rel, unit.stp_r_ves), t, class)
}

/// A product per mille of the unkicked one, rounded down.
fn per_mille(product: u32, class: StpClass) -> u32 {
    u64::from(product)
        .saturating_mul(1_000)
        .checked_div(u64::from(unkicked_product(class)))
        .unwrap_or(0) as u32
}

/// The course after the burst: at each of `COURSE_TICKS`, the pair and its product per mille of
/// the unkicked one.
fn burst_course(class: StpClass) -> Vec<((u8, u8), u32)> {
    COURSE_TICKS
        .iter()
        .map(|&t| {
            let pair = after_burst_at(class, t);
            (pair, per_mille(factor(pair), class))
        })
        .collect()
}

/// The primed window after the burst, over `COURSE_HORIZON`: the first and the last tick at which
/// the product is above the unkicked one; the last at which it is at or above it; the first tick
/// of the highest product, the pair there, and that product per mille of the unkicked one.
type Primed = (u32, u32, u32, u32, (u8, u8), u32);

fn primed(class: StpClass) -> Primed {
    let unit = after_burst(class);
    let state = (unit.stp_u_rel, unit.stp_r_ves);
    let unkicked = unkicked_product(class);
    let (mut first, mut last, mut last_at) = (0u32, 0u32, 0u32);
    let (mut peak_at, mut peak) = (0u32, (0u8, 0u8));
    for t in 1..=COURSE_HORIZON {
        let pair = after_silence(state, t, class);
        let p = factor(pair);
        if p > unkicked {
            if first == 0 {
                first = t;
            }
            last = t;
        }
        if p >= unkicked {
            last_at = t;
        }
        if p > factor(peak) {
            peak_at = t;
            peak = pair;
        }
    }
    (
        first,
        last,
        last_at,
        peak_at,
        peak,
        per_mille(factor(peak), class),
    )
}

/// What one spike of a marked member delivers at each of `WEIGHTS_049` under `class`, at the
/// unkicked steady pair and at the primed peak's pair: ADR-0112's `Delivered` of each.
fn delivered_049(class: StpClass) -> Vec<Delivered> {
    let unkicked = steady_under(INTERVALS[0], Some(class));
    let peak = primed(class).4;
    WEIGHTS_049
        .iter()
        .map(|&w| {
            let at = delivered(w, unkicked);
            let primed = delivered(w, peak);
            (
                at,
                rise(at).0,
                least_together(w, unkicked),
                primed,
                rise(primed).0,
                least_together(w, peak),
            )
        })
        .collect()
}

/// ADR-0113's steady pairs, per set, at rest and at `INTERVALS` but 200 Hz, which its table
/// does not hold: 1.76, 5, 10, 20, 40, 100 and 400 Hz.
const ADR_0113_STEADY: [[(u8, u8); 8]; 2] = [
    [
        (92, 255),
        (113, 255),
        (149, 242),
        (182, 197),
        (208, 130),
        (226, 74),
        (242, 31),
        (250, 8),
    ],
    [
        (49, 255),
        (63, 255),
        (93, 247),
        (128, 211),
        (163, 146),
        (199, 81),
        (225, 33),
        (243, 8),
    ],
];

/// ADR-0113's course after the burst, per set, per mille at `COURSE_TICKS`, as its table rounds
/// them to hundredths.
const ADR_0113_COURSE: [[u32; 8]; 2] = [
    [110, 430, 1_050, 1_330, 1_350, 1_150, 930, 820],
    [430, 750, 1_330, 1_550, 1_520, 1_240, 950, 810],
];

// ------------------------------------------------ the measures (brief 049), before any run

/// One window of a run under the class: the members' spikes, the rest's, and the sums over the
/// members of $u$, of $R$ and of $uR$ a spike on the next tick would release with (`next_pair`).
type ClassWindow = [u32; 5];

/// One stretch's row: the members' spikes, the rest's, the members' burst windows (ADR-0112's:
/// at least one spike a member in a window), the sums over the members of $u$ and of $R$ at the
/// stretch's last window's end, and the sum over its windows of the members' sum of $uR$.
type StretchRow = [u32; 6];

/// The stretches of a run's windows, for an assembly of `size`.
fn stretch_rows(windows: &[ClassWindow], size: u32) -> Vec<StretchRow> {
    windows
        .chunks(STRETCH_WINDOWS)
        .map(|c| {
            let mut row = [0u32; 6];
            for w in c {
                row[0] = row[0].saturating_add(w[0]);
                row[1] = row[1].saturating_add(w[1]);
                row[2] = row[2].saturating_add(u32::from(w[0] >= size));
                row[5] = row[5].saturating_add(w[4]);
            }
            if let Some(w) = c.last() {
                row[3] = w[2];
                row[4] = w[3];
            }
            row
        })
        .collect()
}

/// The background of a run's stretches: every stretch after the lead-in.
fn background_049(rows: &[StretchRow], spans: &[(Span, usize, u32)]) -> Background {
    let mut b = Background {
        members: 0,
        rest: 0,
        ticks: 0,
    };
    for (row, &(span, _, _)) in rows.iter().zip(stretches_of(spans).iter()) {
        if span != Span::LeadIn {
            b.members = b.members.saturating_add(u64::from(row[0]));
            b.rest = b.rest.saturating_add(u64::from(row[1]));
            b.ticks = b.ticks.saturating_add(STRETCH_TICKS);
        }
    }
    b
}

/// A held stretch: the members' spikes over it at least `HOLD_TIMES` the background.
fn held_stretch(row: &StretchRow, bg: Background) -> bool {
    at_least(
        u64::from(row[0]),
        STRETCH_TICKS,
        bg.members,
        bg.ticks,
        HOLD_TIMES,
    )
}

/// A quiet stretch: the members' spikes over it at most `LET_GO_TIMES` the background.
fn quiet_stretch(row: &StretchRow, bg: Background) -> bool {
    u64::from(row[0]).saturating_mul(bg.ticks)
        <= LET_GO_TIMES
            .saturating_mul(bg.members)
            .saturating_mul(STRETCH_TICKS)
}

/// A run's reading by the rules (brief 049).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Holding {
    /// Rounds whose hold span's first half was held in every stretch, of eight: a reading, no
    /// clause.
    first_half: u32,
    /// Rounds whose hold span's second half was held in every stretch, of eight.
    held: u32,
    /// Rounds whose unkicked span held any stretch, of eight.
    ignited: u32,
    /// Rounds whose tail stood at most twice the background in every stretch, of eight.
    let_go: u32,
    /// Whether the rest of the network fired more than twice its background over the held
    /// stretches of the hold spans' second halves; none when none was held.
    spills: Option<bool>,
}

impl Holding {
    fn holds(&self) -> bool {
        self.held >= OF_EIGHT_MIN
    }

    fn lets_go(&self) -> bool {
        self.let_go >= OF_EIGHT_MIN
    }

    fn quiet_unkicked(&self) -> bool {
        self.ignited <= IGNITIONS_MAX
    }

    /// Usable: holds, lets go, ignites in at most one round of eight, does not spill.
    fn usable(&self) -> bool {
        self.holds() && self.lets_go() && self.quiet_unkicked() && self.spills == Some(false)
    }
}

/// A run's stretches read by the rules against `bg`.
fn holding(rows: &[StretchRow], spans: &[(Span, usize, u32)], bg: Background) -> Holding {
    let (mut first, mut second, mut ignited, mut quiet) = (
        [true; ROUNDS],
        [true; ROUNDS],
        [false; ROUNDS],
        [true; ROUNDS],
    );
    let (mut rest, mut held_ticks) = (0u64, 0u64);
    for (row, &(span, round, k)) in rows.iter().zip(stretches_of(spans).iter()) {
        let high = held_stretch(row, bg);
        match span {
            Span::Hold if k < SECOND_HALF_FROM => first[round] &= high,
            Span::Hold => {
                second[round] &= high;
                if high {
                    rest = rest.saturating_add(u64::from(row[1]));
                    held_ticks = held_ticks.saturating_add(STRETCH_TICKS);
                }
            }
            Span::Unkicked => ignited[round] |= high,
            Span::Tail => quiet[round] &= quiet_stretch(row, bg),
            Span::LeadIn | Span::Release => {}
        }
    }
    let count = |flags: [bool; ROUNDS]| flags.iter().filter(|&&f| f).count() as u32;
    Holding {
        first_half: count(first),
        held: count(second),
        ignited: count(ignited),
        let_go: count(quiet),
        spills: (held_ticks > 0).then(|| {
            rest.saturating_mul(bg.ticks)
                > SPILL_TIMES
                    .saturating_mul(bg.rest)
                    .saturating_mul(held_ticks)
        }),
    }
}

/// What failed in a cell that is not usable, as ADR-0112 names it: never holding, running away
/// (ignites in more than one round, or spills), not letting go.
fn failed(h: &Holding) -> [bool; 3] {
    [
        !h.holds(),
        !h.quiet_unkicked() || h.spills == Some(true),
        !h.lets_go(),
    ]
}

/// The kick's reading over a run's rounds: the members' spikes in each hold span's first
/// `KICK_SPAN` ticks and in the pair window after, summed; the kicks; the kicks whose volley was
/// every member — ADR-0112's `kick_reading` over rounds.
fn kick_reading_049(kicks: &[[u32; 2]], size: u32) -> (u64, u64, u64, u32) {
    let (mut volley, mut after, mut full) = (0u64, 0u64, 0u32);
    for k in kicks {
        volley = volley.saturating_add(u64::from(k[0]));
        after = after.saturating_add(u64::from(k[1]));
        full = full.saturating_add(u32::from(k[0] == size));
    }
    (volley, after, kicks.len() as u64, full)
}

/// The kick fires every member once, by ADR-0112's measure (`kicked_once`) over a run's rounds.
fn kicked_once_049(kicks: &[[u32; 2]], size: u32) -> bool {
    fires_every_member_once(kick_reading_049(kicks, size), size)
}

/// A run's burst reading (brief 049): the burst windows in the lead-in, the unkicked spans, the
/// hold spans, the release spans and the tails; the intervals between consecutive burst windows
/// within a hold span, in windows, counted in the bins 1, 2, 3–4, 5–8, 9–16, 17–32, 33–64 and
/// 65–128; and the members' mean product a spike would release with, per member per window, per
/// mille of the unkicked product, over the held stretches of the hold spans' second halves (none
/// when none was held) and over the unkicked spans.
type BurstRead049 = ([u32; 5], [u32; 8], Option<u32>, u32);

/// The bin of an interval of `windows` between two burst windows.
fn interval_bin(windows: u32) -> usize {
    let bin = u32::BITS.saturating_sub(windows.saturating_sub(1).leading_zeros());
    bin.min(7) as usize
}

fn bursts_049(
    windows: &[ClassWindow],
    spans: &[(Span, usize, u32)],
    size: u32,
    bg: Background,
    class: StpClass,
) -> BurstRead049 {
    let (mut by_span, mut bins) = ([0u32; 5], [0u32; 8]);
    let (mut held_sum, mut held_windows, mut unkicked_sum, mut unkicked_windows) =
        (0u64, 0u64, 0u64, 0u64);
    let mut at = 0usize;
    for &(span, _, epochs) in spans {
        let n = epochs.saturating_mul(WINDOWS as u32) as usize;
        let these = &windows[at..at.saturating_add(n)];
        at = at.saturating_add(n);
        let kind = match span {
            Span::LeadIn => 0,
            Span::Unkicked => 1,
            Span::Hold => 2,
            Span::Release => 3,
            Span::Tail => 4,
        };
        let mut last: Option<u32> = None;
        for (i, w) in these.iter().enumerate() {
            if w[0] >= size {
                by_span[kind] = by_span[kind].saturating_add(1);
                if span == Span::Hold {
                    if let Some(l) = last {
                        let b = interval_bin((i as u32).saturating_sub(l));
                        bins[b] = bins[b].saturating_add(1);
                    }
                    last = Some(i as u32);
                }
            }
        }
        match span {
            Span::Unkicked => {
                for w in these {
                    unkicked_sum = unkicked_sum.saturating_add(u64::from(w[4]));
                }
                unkicked_windows = unkicked_windows.saturating_add(n as u64);
            }
            Span::Hold => {
                for (k, c) in these.chunks(STRETCH_WINDOWS).enumerate() {
                    let row = stretch_rows(c, size)[0];
                    if k as u32 >= SECOND_HALF_FROM && held_stretch(&row, bg) {
                        held_sum = held_sum.saturating_add(u64::from(row[5]));
                        held_windows = held_windows.saturating_add(STRETCH_WINDOWS as u64);
                    }
                }
            }
            _ => {}
        }
    }
    let mean = |sum: u64, n: u64| {
        sum.saturating_mul(1_000)
            .checked_div(
                n.saturating_mul(u64::from(size))
                    .saturating_mul(u64::from(unkicked_product(class))),
            )
            .unwrap_or(0) as u32
    };
    (
        by_span,
        bins,
        (held_windows > 0).then(|| mean(held_sum, held_windows)),
        mean(unkicked_sum, unkicked_windows),
    )
}

// ------------------------------------------------ the image, marked (brief 049)

/// `image` with `class` set in its modulator record and the members of `size` marked
/// facilitating in their records, each section re-sealed: the test's own marks, as `grown` and
/// `wire` are its own wiring, and every other byte the image's.
fn marked(image: &[u8], class: StpClass, size: u32) -> Vec<u8> {
    let mut img = image.to_vec();
    let header = MenteionFileHeader::decode(img[0..64].try_into().unwrap());
    let m = members(size);
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        let (offset, length) = (entry.offset as usize, entry.length as usize);
        let section = &mut img[offset..][..length];
        match entry.kind {
            SECTION_MODULATOR => {
                assert_eq!(&section[32..36], &[0u8; 4], "no class before");
                section[32..36].copy_from_slice(&[
                    1,
                    class.u,
                    class.tau_f_shift,
                    class.tau_d_shift,
                ]);
            }
            SECTION_NEURON => {
                for &u in &m {
                    let flags = (u as usize).saturating_mul(64).saturating_add(57);
                    section[flags] |= FLAG_FACILITATING;
                }
            }
            _ => continue,
        }
        entry.crc64 = crc64(&img[offset..][..length]);
        img[at..][..64].copy_from_slice(&entry.encode());
    }
    img
}

/// The engine of a run under set `set` for the assembly of `size`: the frozen image marked;
/// grown by the assembly's blocks when `grow` (a control or a cell), and wired at `weight` when
/// given. The class and the marks asserted on the engine.
fn marked_engine(image: &[u8], set: usize, size: u32, grow: bool, weight: Option<i16>) -> Engine {
    marked_engine_drawn(image, set, size, grow, weight, DELAY_SEED)
}

/// `marked_engine` with the assembly's delays drawn by `seed` (`wire_drawn`, brief 050).
fn marked_engine_drawn(
    image: &[u8],
    set: usize,
    size: u32,
    grow: bool,
    weight: Option<i16>,
    seed: u64,
) -> Engine {
    let img = marked(image, SETS[set], size);
    let mut exec = if grow {
        frozen_from(&grown(&img, extra_blocks(size)), 1024)
    } else {
        frozen_from(&img, 1024)
    };
    assert_eq!(exec.stp_class(), Some(SETS[set]), "the image's class");
    let of = membership(&members(size));
    for unit in exec.units() {
        assert_eq!(
            unit.flags & FLAG_FACILITATING != 0,
            of[unit.id as usize],
            "unit {} marked as a member",
            unit.id
        );
    }
    if let Some(w) = weight {
        wire_drawn(&mut exec, size, w, seed);
    }
    exec
}

// ------------------------------------------------ the protocol (brief 049), run

/// The sums over `set` of $u$, $R$ and $uR$ a spike on tick `now` would release with
/// (`next_pair`).
fn stp_reading(
    units: &[DendriticSuperNeuron],
    set: &[u32],
    now: u32,
    class: Option<StpClass>,
) -> (u32, u32, u32) {
    let (mut su, mut sr, mut sp) = (0u32, 0u32, 0u32);
    for &m in set {
        let pair = next_pair(&units[m as usize], now, class);
        su = su.saturating_add(u32::from(pair.0));
        sr = sr.saturating_add(u32::from(pair.1));
        sp = sp.saturating_add(factor(pair));
    }
    (su, sr, sp)
}

/// A run under the protocol: every window's row, and each hold span's kick reading — the
/// members' spikes in the span's first `KICK_SPAN` ticks and in the pair window after; every
/// window's spikes of the units counted beside the members (`span_protocol_counting`, brief
/// 051's sources), none counted before brief 051; while the engine carries a slow current
/// (brief 052), every window's sums over the members and its ticks of the slow potential and of
/// the gate's opening at the soma as each tick left them, none without one; and, when read
/// (brief 053), the members' somatic potentials over every tick after the lead-in
/// (`SomaRead`), none before brief 053.
struct SpanRun {
    windows: Vec<ClassWindow>,
    kicks: Vec<[u32; 2]>,
    sources: Vec<u32>,
    slow: Vec<[u64; 2]>,
    soma: Option<SomaRead>,
}

/// The members' somatic potentials over a run's ticks after the lead-in, as each tick left them
/// (brief 053): how many were above each of `SOMA_LEVELS_053`, their sum, Q16.16, and how many
/// were read.
type SomaRead = ([u64; 6], i64, u64);

/// The levels the soma is read at: 0.5, 0.6, 0.7, 0.8, 0.9 and 0.95 of the threshold's base,
/// floored in Q16.16. The three in the middle are brief 053's lower gate voltages
/// (`V_LOS_053`), so a soma above one of them is a soma the gate at that voltage is open on.
const SOMA_LEVELS_053: [i32; 6] = [32_768, 39_321, 45_875, 52_428, 58_982, 62_259];

/// `read` with one member's somatic potential `v_soma` added: above a level is strictly above
/// it, as the gate is shut at its lower voltage.
fn soma_add(read: &mut SomaRead, v_soma: i32) {
    for (count, &level) in read.0.iter_mut().zip(SOMA_LEVELS_053.iter()) {
        *count = count.saturating_add(u64::from(v_soma > level));
    }
    read.1 = read.1.saturating_add(i64::from(v_soma));
    read.2 = read.2.saturating_add(1);
}

/// The protocol from the executor's clock under ADR-0044's drive: `spans` in order, `set`'s
/// windows read; with `kick`, the kick into `set` at each hold span's start and the release's
/// cancels in each release span; without, the drive alone over the same ticks (a background).
fn span_protocol(
    exec: &mut Engine,
    set: &[u32],
    spans: &[(Span, usize, u32)],
    kick: bool,
) -> SpanRun {
    span_protocol_under(exec, set, spans, kick, None).0
}

/// `span_protocol` with, when `task` is given (brief 050's condition (d)), the task's stimulus
/// presented at every epoch's middle as `Task::trial` presents it at a trial's first tick: the
/// stimulus the task draws for the epoch's index in the run injected before the epoch's tick
/// `STIMULUS_AT`, and its cancel before each tick it is due at, both before the drive's step;
/// and each epoch's reading (`TaskEpoch`). With none it is `span_protocol`, which calls it so:
/// nothing is injected and nothing the engine reads is read.
fn span_protocol_under(
    exec: &mut Engine,
    set: &[u32],
    spans: &[(Span, usize, u32)],
    kick: bool,
    task: Option<&Task>,
) -> (SpanRun, Vec<TaskEpoch>) {
    span_protocol_counting(exec, set, spans, kick, task, &[])
}

/// `span_protocol_under` with the spikes of `sources` counted in every window beside the rows
/// (brief 051): a reading of the train, which the run's inputs do not see. With none it is
/// `span_protocol_under`, which calls it so.
fn span_protocol_counting(
    exec: &mut Engine,
    set: &[u32],
    spans: &[(Span, usize, u32)],
    kick: bool,
    task: Option<&Task>,
    sources: &[u32],
) -> (SpanRun, Vec<TaskEpoch>) {
    span_protocol_reading(exec, set, spans, kick, task, sources, false)
}

/// `span_protocol_counting` with, when `soma` (brief 053), the members' somatic potentials read
/// over every tick after the lead-in as each tick left them (`SomaRead`): a reading of the
/// records, which the run's inputs do not see. Without it is `span_protocol_counting`, which
/// calls it so.
fn span_protocol_reading(
    exec: &mut Engine,
    set: &[u32],
    spans: &[(Span, usize, u32)],
    kick: bool,
    task: Option<&Task>,
    sources: &[u32],
    soma: bool,
) -> (SpanRun, Vec<TaskEpoch>) {
    let drive = drive(1024);
    let inject = exec.injector();
    let class = exec.stp_class();
    let current = exec.slow_current();
    let of = membership(set);
    let of_sources = membership(sources);
    let counted = task.map(|_| counted_sets());
    let mut run = SpanRun {
        windows: Vec::new(),
        kicks: Vec::new(),
        sources: Vec::new(),
        slow: Vec::new(),
        soma: soma.then_some(([0; 6], 0, 0)),
    };
    let mut read: Vec<TaskEpoch> = Vec::new();
    let mut seen = (exec.train().len() as u64).saturating_add(exec.train_overwritten());
    for &(span, _, epochs) in spans {
        let start = exec.ticks() as u32;
        let windows = epochs.saturating_mul(WINDOWS as u32);
        let mut resets: Vec<(u32, u32)> = Vec::new();
        let mut kick_read = [0u32; 2];
        // The epochs of the span under a task: the stimulus each draws by its index in the run.
        let first = read.len() as u64;
        let mut here: Vec<TaskEpoch> = match task {
            Some(t) => (0..u64::from(epochs))
                .map(|e| (t.stimulus_at(first.saturating_add(e)), [[0u32; 4]; 2]))
                .collect(),
            None => Vec::new(),
        };
        for w in 0..windows {
            // The window's slow reading (brief 052): a reading of the records, which no input of
            // the run sees.
            let mut slow = [0u64; 2];
            for j in 0..WINDOW_SPAN {
                let k = w.saturating_mul(WINDOW_SPAN).saturating_add(j);
                if let Some(t) = task {
                    let e = k.checked_div(EPOCH_TICKS).unwrap_or(0) as usize;
                    let at = k.checked_rem(EPOCH_TICKS).unwrap_or(0);
                    let presented = t.stimuli[usize::from(here[e].0)];
                    if at == STIMULUS_AT {
                        presented.inject(&inject).expect("the ring has room");
                    }
                    if let Some(trial_k) = at.checked_sub(STIMULUS_AT) {
                        presented
                            .cancel_at(&inject, trial_k)
                            .expect("the ring has room");
                    }
                }
                drive.step(&inject, exec.ticks()).expect("the drive runs");
                if kick && span == Span::Hold {
                    inject_before(&inject, set, Epoch::Kicked, k, &resets);
                }
                if kick && span == Span::Release && cancel_due(k, windows) {
                    for &m in set {
                        for _ in 0..CANCEL_AT_THE_EXTREME {
                            inject
                                .inject(m, spike_message(CANCEL_MESSAGE_Q16, false))
                                .expect("the ring has room");
                        }
                    }
                }
                exec.tick();
                if kick && span == Span::Hold {
                    note_kick_spikes(exec, set, Epoch::Kicked, k, start, &mut resets);
                }
                if let Some(c) = current {
                    let units = exec.units();
                    for &m in set {
                        let u = &units[m as usize];
                        slow[0] = slow[0].saturating_add(u64::from(u.v_slow.max(0).unsigned_abs()));
                        slow[1] =
                            slow[1].saturating_add(u64::from(c.gate_q16(u.v_soma).unsigned_abs()));
                    }
                }
                // The soma's reading (brief 053), after the lead-in.
                if let Some(read) = run.soma.as_mut().filter(|_| span != Span::LeadIn) {
                    let units = exec.units();
                    for &m in set {
                        soma_add(read, units[m as usize].v_soma);
                    }
                }
            }
            if current.is_some() {
                run.slow.push(slow);
            }
            let total = (exec.train().len() as u64).saturating_add(exec.train_overwritten());
            let new = total.saturating_sub(seen);
            let train = exec.train();
            assert!(new <= train.len() as u64, "the train held the window");
            let mut row = [0u32; 5];
            let mut from_sources = 0u32;
            for &(stamp, unit) in &train[train.len().saturating_sub(new as usize)..] {
                from_sources = from_sources.saturating_add(u32::from(of_sources[unit as usize]));
                if of[unit as usize] {
                    row[0] = row[0].saturating_add(1);
                    let t = stamp.wrapping_sub(start);
                    if span == Span::Hold && (1..=KICK_SPAN).contains(&t) {
                        kick_read[0] = kick_read[0].saturating_add(1);
                    } else if span == Span::Hold
                        && t > KICK_SPAN
                        && t <= KICK_SPAN.saturating_add(PAIR_WINDOW)
                    {
                        kick_read[1] = kick_read[1].saturating_add(1);
                    }
                } else {
                    row[1] = row[1].saturating_add(1);
                }
                if let Some(sets) = &counted {
                    let t = stamp.wrapping_sub(start);
                    let e = t.checked_div(EPOCH_TICKS).unwrap_or(0) as usize;
                    let at = t.checked_rem(EPOCH_TICKS).unwrap_or(0);
                    if let (Some(side), Some(epoch)) = (task_side(at), here.get_mut(e)) {
                        for (count, of_set) in epoch.1[side].iter_mut().zip(sets.iter()) {
                            if of_set[unit as usize] {
                                *count = count.saturating_add(1);
                            }
                        }
                    }
                }
            }
            seen = total;
            let (su, sr, sp) = stp_reading(exec.units(), set, exec.ticks() as u32, class);
            row[2] = su;
            row[3] = sr;
            row[4] = sp;
            run.windows.push(row);
            run.sources.push(from_sources);
        }
        if span == Span::Hold {
            run.kicks.push(kick_read);
        }
        read.extend(here);
    }
    (run, read)
}

/// A run's windows as one number: FNV-1a over every window's five words in order, each as the
/// `i32` of its bits.
fn windows_hash(windows: &[ClassWindow]) -> u64 {
    let words: Vec<i32> = windows
        .iter()
        .flat_map(|w| w.iter().map(|&v| v as i32))
        .collect();
    fnv1a_64(&words)
}

/// A run under set `set` for the assembly of `size`, dumped: a background (`grow` false, no
/// kick), a control (grown, unwired, the protocol) or a cell (wired at `weight`); the weights
/// shown unchanged at its end.
fn class_run(
    image: &[u8],
    set: usize,
    size: u32,
    weight: Option<i16>,
    background: bool,
    name: &str,
) -> SpanRun {
    let spans = layout(RELEASE_STRETCHES_049[set]);
    let mut exec = marked_engine(image, set, size, !background, weight);
    let m = members(size);
    let before = weights_of(&exec);
    let run = span_protocol(&mut exec, &m, &spans, !background);
    let rows = stretch_rows(&run.windows, size);
    eprintln!("DUMP {name} stretches = {rows:?}");
    eprintln!("DUMP {name} kicks = {:?}", run.kicks);
    eprintln!(
        "DUMP {name} windows hash = {:#018x}",
        windows_hash(&run.windows)
    );
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    run
}

// ------------------------------------------------ the gate (brief 049)

/// The arithmetic of brief 049 as the oracles compute it with the class's step, per set: the
/// pair at rest; the steady pairs at `INTERVALS`; the unkicked product; the course after the
/// burst; the primed window; the delivery at each weight; the fade and the release's stretches;
/// and the release by the oracle from the drive's mean standing and from the extreme.
#[allow(clippy::type_complexity)]
fn arithmetic_049(
    set: usize,
) -> (
    (u8, u8),
    Vec<(u8, u8)>,
    u32,
    Vec<((u8, u8), u32)>,
    Primed,
    Vec<Delivered>,
    Option<u32>,
    Option<u32>,
    Vec<(Vec<u32>, i32, Option<u32>)>,
) {
    let class = SETS[set];
    let release = release_rule(class);
    (
        at_rest_under(Some(class)),
        INTERVALS
            .iter()
            .map(|&i| steady_under(i, Some(class)))
            .collect(),
        unkicked_product(class),
        burst_course(class),
        primed(class),
        delivered_049(class),
        fade(class),
        release,
        [DRIVE_STANDING, EXTREME_STANDING]
            .iter()
            .map(|&s| release_span_oracle(s, release.unwrap_or(0)))
            .collect(),
    )
}

/// The ticks the gate runs a marked assembly kicked: ADR-0112's 800.
const GATE_KICK_TICKS_049: u32 = GATE_KICK_TICKS;

#[test]
fn the_facilitating_class_the_arithmetic_the_spans_the_rules_and_a_marked_assembly_on_the_engine() {
    // The grid.
    assert_eq!(
        SETS[0].u, STP_U,
        "set (i) keeps ADR-0019's release fraction"
    );
    assert_eq!(SETS[1].u, 26);
    for class in SETS {
        assert_eq!((class.tau_f_shift, class.tau_d_shift), (16, 13));
    }
    assert!(WEIGHTS_049.windows(2).all(|w| w[0] < w[1]));
    for w in WEIGHTS {
        assert!(WEIGHTS_049.contains(&w), "ADR-0112's weight {w:#x}");
    }
    // The arithmetic with the class's step, as ADR-0115 writes it: dumped, then held, and held to
    // ADR-0113's replica.
    let arithmetic: Vec<_> = (0..SETS.len()).map(arithmetic_049).collect();
    for (set, a) in arithmetic.iter().enumerate() {
        let (rest, steadies, unkicked, course, primed_read, by_weight, fade_read, release, oracle) =
            a;
        let name = SET_NAMES[set];
        eprintln!("DUMP {name} CLASS_AT_REST = {rest:?}");
        eprintln!("DUMP {name} CLASS_STEADY = {steadies:?}");
        eprintln!("DUMP {name} UNKICKED_PRODUCT = {unkicked}");
        eprintln!("DUMP {name} BURST_COURSE = {course:?}");
        eprintln!("DUMP {name} PRIMED = {primed_read:?}");
        eprintln!("DUMP {name} DELIVERED = {by_weight:?}");
        eprintln!("DUMP {name} FADE = {fade_read:?} RELEASE = {release:?}");
        eprintln!("DUMP {name} RELEASE_SPAN_ORACLE = {oracle:?}");
        eprintln!(
            "DUMP {name} RUN_EPOCHS = {:?}",
            release.map(|r| layout(r).iter().map(|s| s.2).sum::<u32>())
        );
    }
    for (set, class) in SETS.iter().enumerate() {
        let (rest, steadies, unkicked, course, primed_read, by_weight, fade_read, release, oracle) =
            arithmetic[set].clone();
        let name = SET_NAMES[set];
        assert_eq!(rest, CLASS_AT_REST_049[set]);
        assert_eq!(steadies.as_slice(), CLASS_STEADY_049[set].as_slice());
        assert_eq!(unkicked, UNKICKED_PRODUCT_049[set]);
        assert_eq!(course.as_slice(), BURST_COURSE_049[set].as_slice());
        assert_eq!(primed_read, PRIMED_049[set]);
        assert_eq!(by_weight.as_slice(), DELIVERED_049[set].as_slice());
        assert_eq!(fade_read, Some(FADE_049[set]));
        assert_eq!(release, Some(RELEASE_STRETCHES_049[set]));
        for (k, (fires, lowest, back)) in oracle.iter().enumerate() {
            assert_eq!(
                (fires.as_slice(), *lowest, *back),
                RELEASE_SPAN_ORACLE_049[set][k]
            );
            assert!(
                fires.is_empty(),
                "{name}: a released unit fires in no tick of the span"
            );
            assert!(
                back.is_some_and(|b| b <= 2 * WINDOW_SPAN),
                "{name}: and is back before the tail"
            );
        }
        // ADR-0113's replica: the steady table pair for pair, at rest and at every rate its
        // table holds, and the course to its hundredths.
        let held_rates = [0usize, 1, 2, 3, 4, 5, 7];
        assert_eq!(rest, ADR_0113_STEADY[set][0]);
        for (j, &i) in held_rates.iter().enumerate() {
            assert_eq!(steadies[i], ADR_0113_STEADY[set][j.saturating_add(1)]);
        }
        for (k, &(_, pm)) in course.iter().enumerate() {
            assert!(
                pm.abs_diff(ADR_0113_COURSE[set][k]) <= 5,
                "{name}: the course at {} ticks, {pm} per mille",
                COURSE_TICKS[k]
            );
        }
        // The release's rule at its edge: one stretch fewer leaves the fade uncovered.
        let s = RELEASE_STRETCHES_049[set];
        assert!(silence_of(s) >= u64::from(FADE_049[set]));
        assert!(silence_of(s.saturating_sub(1)) < u64::from(FADE_049[set]));
        // The fade from the ceiling bounds the fade from any pair: at the fade no pair of the
        // lattice releases at or above the unkicked product, and one tick before the ceiling
        // does.
        for &u in U8_LATTICE.iter() {
            for &r in U8_LATTICE.iter() {
                assert!(
                    factor(after_silence((u, r), FADE_049[set], *class)) < unkicked,
                    "{name}: ({u}, {r})"
                );
            }
        }
        assert!(
            factor(after_silence(
                CEILING,
                FADE_049[set].saturating_sub(1),
                *class
            )) >= unkicked
        );
    }
    // The spans.
    for set in 0..SETS.len() {
        let spans = layout(RELEASE_STRETCHES_049[set]);
        assert_eq!(spans.len(), 1usize.saturating_add(ROUNDS.saturating_mul(4)));
        assert_eq!(spans[0], (Span::LeadIn, 0, LEAD_IN_EPOCHS_049));
        let kinds: Vec<Span> = spans[1..5].iter().map(|s| s.0).collect();
        assert_eq!(
            kinds,
            [Span::Unkicked, Span::Hold, Span::Release, Span::Tail]
        );
        assert_eq!(spans[3].2, RELEASE_STRETCHES_049[set] * STRETCH_EPOCHS);
        let stretches = stretches_of(&spans);
        let epochs: u32 = spans.iter().map(|s| s.2).sum();
        assert_eq!(
            stretches.len() as u32,
            epochs / STRETCH_EPOCHS,
            "whole stretches"
        );
        assert_eq!(epochs, RUN_EPOCHS_049[set]);
    }
    let windows = 16u32;
    assert!(cancel_due(0, windows) && cancel_due(CANCEL_TICKS - 1, windows));
    assert!(!cancel_due(CANCEL_TICKS, windows));
    assert!(cancel_due(13 * WINDOW_SPAN, windows) && !cancel_due(14 * WINDOW_SPAN, windows));
    assert!(!cancel_due(15 * WINDOW_SPAN, windows));
    assert_eq!(
        (1..=128).map(interval_bin).fold([0u32; 8], |mut b, i| {
            b[i] += 1;
            b
        }),
        [1, 1, 2, 4, 8, 16, 32, 64]
    );
    // The rules at their edges, over rows written by hand.
    rules_049_at_their_edges();
    // The marks and one marked assembly on the engine, and the release on the engine.
    marked_on_the_prior();
    let kicked = a_marked_assembly_kicked_for_a_few_hundred_ticks();
    eprintln!("DUMP GATE_KICKED_049 = {kicked:?}");
    assert_eq!(
        kicked.as_slice(),
        GATE_KICKED_049,
        "the marked assembly kicked"
    );
    // The rules over the pinned tables.
    over_the_049_tables();
}

/// Hand-written stretches: a run of set `set` whose every stretch reads `unkicked` members'
/// spikes in the unkicked spans, `first` and `second` in the hold spans' halves with `rest` of
/// the rest's in the second, `tail` in the tails, and nothing elsewhere.
fn hand_stretches(
    spans: &[(Span, usize, u32)],
    unkicked: u32,
    first: u32,
    second: u32,
    rest: u32,
    tail: u32,
) -> Vec<StretchRow> {
    stretches_of(spans)
        .iter()
        .map(|&(span, _, k)| match span {
            Span::Unkicked => [unkicked, 0, 0, 0, 0, 0],
            Span::Hold if k < SECOND_HALF_FROM => [first, 0, 0, 0, 0, 0],
            Span::Hold => [second, rest, 0, 0, 0, 0],
            Span::Tail => [tail, 0, 0, 0, 0, 0],
            _ => [0; 6],
        })
        .collect()
}

/// The index in a run's stretches of round `round`'s span `span`, stretch `k`.
fn stretch_at(spans: &[(Span, usize, u32)], span: Span, round: usize, k: u32) -> usize {
    stretches_of(spans)
        .iter()
        .position(|&s| s == (span, round, k))
        .expect("a stretch of the run")
}

fn rules_049_at_their_edges() {
    let spans = layout(5);
    // A background of one member's spike per stretch and a hundred of the rest's: five in a
    // stretch is five times it, two is twice it.
    let n = stretches_of(&spans).len() as u64;
    let lead = u64::from(LEAD_IN_EPOCHS_049 / STRETCH_EPOCHS);
    let bg = Background {
        members: n.saturating_sub(lead),
        rest: n.saturating_sub(lead).saturating_mul(100),
        ticks: n.saturating_sub(lead).saturating_mul(STRETCH_TICKS),
    };
    let mut quiet: Vec<StretchRow> = stretches_of(&spans)
        .iter()
        .map(|_| [1, 100, 0, 0, 0, 0])
        .collect();
    quiet[0] = [9, 900, 0, 0, 0, 0];
    assert_eq!(
        background_049(&quiet, &spans),
        bg,
        "the lead-in is not in a background"
    );
    // Holds at five times exactly, lets go at twice exactly, ignites at nothing below five
    // times, spills at nothing above twice.
    let usable = hand_stretches(&spans, 4, 5, 5, 200, 2);
    let read = holding(&usable, &spans, bg);
    assert_eq!(
        read,
        Holding {
            first_half: 8,
            held: 8,
            ignited: 0,
            let_go: 8,
            spills: Some(false)
        }
    );
    assert!(read.usable());
    assert_eq!(failed(&read), [false; 3]);
    // One LSB past each edge, in one round, then in two.
    let mut run = usable.clone();
    run[stretch_at(&spans, Span::Hold, 0, 7)][0] = 4;
    assert_eq!(holding(&run, &spans, bg).held, 7);
    assert!(holding(&run, &spans, bg).usable(), "seven of eight hold");
    run[stretch_at(&spans, Span::Hold, 5, SECOND_HALF_FROM)][0] = 4;
    let read = holding(&run, &spans, bg);
    assert_eq!(read.held, 6);
    assert!(!read.holds() && !read.usable());
    assert_eq!(failed(&read), [true, false, false], "never holding");
    let mut run = usable.clone();
    run[stretch_at(&spans, Span::Hold, 2, SECOND_HALF_FROM - 1)][0] = 4;
    let read = holding(&run, &spans, bg);
    assert_eq!(
        (read.first_half, read.held),
        (7, 8),
        "the first half is a reading"
    );
    assert!(read.usable());
    let mut run = usable.clone();
    run[stretch_at(&spans, Span::Tail, 3, 1)][0] = 3;
    assert_eq!(holding(&run, &spans, bg).let_go, 7);
    assert!(holding(&run, &spans, bg).usable());
    run[stretch_at(&spans, Span::Tail, 4, 0)][0] = 3;
    let read = holding(&run, &spans, bg);
    assert!(!read.lets_go() && !read.usable());
    assert_eq!(failed(&read), [false, false, true], "not letting go");
    let mut run = usable.clone();
    run[stretch_at(&spans, Span::Unkicked, 0, 7)][0] = 5;
    assert_eq!(holding(&run, &spans, bg).ignited, 1);
    assert!(holding(&run, &spans, bg).usable());
    run[stretch_at(&spans, Span::Unkicked, 6, 0)][0] = 5;
    run[stretch_at(&spans, Span::Unkicked, 6, 3)][0] = 5;
    let read = holding(&run, &spans, bg);
    assert_eq!(
        read.ignited, 2,
        "a round ignites once however many stretches"
    );
    assert_eq!(failed(&read), [false, true, false], "running away");
    // Spills: the rest over the held stretches of the second halves more than twice its
    // background; a held stretch of a first half is not read.
    let mut run = usable.clone();
    run[stretch_at(&spans, Span::Hold, 1, 6)][1] = 201;
    assert_eq!(holding(&run, &spans, bg).spills, Some(true));
    assert_eq!(failed(&holding(&run, &spans, bg)), [false, true, false]);
    let mut run = usable.clone();
    run[stretch_at(&spans, Span::Hold, 1, 0)][1] = 100_000;
    run[stretch_at(&spans, Span::Hold, 2, 5)] = [4, 100_000, 0, 0, 0, 0];
    let read = holding(&run, &spans, bg);
    assert_eq!((read.spills, read.held), (Some(false), 7));
    // Nothing held: no spill is read.
    let none = holding(&hand_stretches(&spans, 0, 0, 4, 100_000, 0), &spans, bg);
    assert_eq!((none.held, none.spills, none.let_go), (0, None, 8));
    assert!(!none.usable());
    // The stretches of windows: sums, the burst windows, and the sums of u and R at the last
    // window's end, zero when that window read none.
    let mut w = vec![[0u32; 5]; STRETCH_WINDOWS * 2];
    w[0] = [16, 3, 7, 8, 9];
    w[15] = [15, 1, 70, 80, 90];
    w[16] = [17, 2, 1, 2, 3];
    assert_eq!(
        stretch_rows(&w, 16),
        [[31, 4, 1, 70, 80, 99], [17, 2, 1, 0, 0, 3]]
    );
    // The kick's measure over eight kicks of sixteen members: ADR-0112's.
    assert!(kicked_once_049(&[[16, 1]; 8], 16));
    assert!(
        !kicked_once_049(&[[16, 2]; 8], 16),
        "an after of 0.125 a member"
    );
    assert!(!kicked_once_049(&[[17, 0]; 8], 16));
    assert!(kicked_once_049(&[[14, 0]; 8], 16), "within the tolerance");
    assert!(!kicked_once_049(&[[13, 0]; 8], 16));
    assert_eq!(kick_reading_049(&[[16, 1], [15, 0]], 16), (31, 1, 2, 1));
}

/// The marks on the instrument's network at rest: the loader takes the marked image with the
/// class set and every member marked and no other unit, grown or not, and the marked image with
/// no class is refused at its first member; unwired, one marked member's short-term state steps
/// under the class and an unmarked unit's under ADR-0019's constants in one run under the drive;
/// and the release on the engine: one member at rest with no drive keeps the oracle's basal
/// potential through two windows of cancels.
fn marked_on_the_prior() {
    let image = prior_image();
    for set in 0..SETS.len() {
        for &size in &SIZES {
            let exec = marked_engine(&image, set, size, true, None);
            let plain = frozen_from(&image, 1024);
            assert_eq!(&exec.blocks()[..plain.blocks().len()], plain.blocks());
        }
    }
    let mut unclassed = marked(&image, SETS[0], 16);
    let header = MenteionFileHeader::decode(unclassed[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(unclassed[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_MODULATOR {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            unclassed[offset..][32..36].copy_from_slice(&[0; 4]);
            entry.crc64 = crc64(&unclassed[offset..][..length]);
            unclassed[at..][..64].copy_from_slice(&entry.encode());
        }
    }
    assert!(matches!(
        Image::decode::<2048>(&unclassed, config(1024, 2, 0)),
        Err(menteion_runtime::ImageError::MarkWithoutClass(5))
    ));
    // Under the drive, unwired: each member's pair is the class's step on a copy with its own
    // spikes' intervals, and each other unit's `step_stp`.
    let mut exec = marked_engine(&image, 1, 64, false, None);
    let drive = drive(1024);
    let until = exec.ticks().saturating_add(u64::from(GROWTH_TICKS) * 4);
    let start = exec.ticks() as u32;
    let rested: Vec<(u8, u8, u32)> = exec
        .units()
        .iter()
        .map(|u| (u.stp_u_rel, u.stp_r_ves, u.last_soma_spike_tick))
        .collect();
    run_driven(&mut exec, &drive, until).expect("the drive runs");
    let of = membership(&members(64));
    let mut checked = [0u32; 2];
    let train = exec.train().to_vec();
    for unit in exec.units() {
        let id = unit.id as usize;
        let spikes: Vec<u32> = train
            .iter()
            .filter(|&&(_, u)| u as usize == id)
            .map(|&(t, _)| t)
            .collect();
        if spikes.is_empty() {
            continue;
        }
        let (u0, r0, last) = rested[id];
        let mut copy = DendriticSuperNeuron::new(0);
        copy.stp_u_rel = u0;
        copy.stp_r_ves = r0;
        copy.flags = unit.flags;
        let mut previous = (last != NO_SPIKE_ON_RECORD).then_some(last);
        for &t in &spikes {
            let elapsed = previous.map_or(u32::MAX, |p| t.wrapping_sub(p));
            stepped_pair(&mut copy, elapsed, Some(SETS[1]));
            previous = Some(t);
        }
        assert_eq!(
            (unit.stp_u_rel, unit.stp_r_ves),
            (copy.stp_u_rel, copy.stp_r_ves),
            "unit {id}"
        );
        checked[usize::from(of[id])] = checked[usize::from(of[id])].saturating_add(1);
    }
    assert!(
        checked[0] > 100 && checked[1] > 0,
        "the drive fired members and others: {checked:?} since {start}"
    );
    // The release on the engine: member 5 at rest with no drive, two windows of the cancel as the
    // protocol lands it, its basal potential the oracle's after every tick.
    let windows = 16u32;
    let mut exec = marked_engine(&image, 0, 16, false, None);
    let inject = exec.injector();
    let per = scaled_q16(
        batch_q16(CANCEL_AT_THE_EXTREME, CANCEL_MESSAGE_Q16),
        GAIN_1024,
    );
    let oracle = stepped(
        (0, 0),
        |k| if cancel_due(k, windows) { per } else { 0 },
        2 * WINDOW_SPAN,
    );
    let mut basal = Vec::new();
    for k in 0..2 * WINDOW_SPAN {
        if cancel_due(k, windows) {
            for _ in 0..CANCEL_AT_THE_EXTREME {
                inject
                    .inject(5, spike_message(CANCEL_MESSAGE_Q16, false))
                    .expect("the ring has room");
            }
        }
        exec.tick();
        basal.push(exec.units()[5].v_basal);
    }
    assert!(oracle.fires.is_empty());
    assert_eq!(
        basal[1..],
        oracle.basal[..oracle.basal.len().saturating_sub(1)],
        "the engine's basal potential is the oracle's"
    );
}

/// The assembly of sixteen at the top weight under set (i), marked and wired on the instrument's
/// network at rest and frozen, kicked at the first tick under ADR-0044's drive through the
/// protocol's own kick and run for `GATE_KICK_TICKS_049`: every member fires once in the span,
/// sixteen resets are scheduled, no weight moves, and every member's short-term state is the
/// class's step over its spikes; the members' spikes as `(tick after the start, unit)`.
fn a_marked_assembly_kicked_for_a_few_hundred_ticks() -> Vec<(u32, u32)> {
    marked_assembly_kicked(0, 16, i16::MAX)
}

/// `a_marked_assembly_kicked_for_a_few_hundred_ticks` for the assembly of `size` marked under
/// set `set` and wired at `weight` (brief 050's gate kicks one on the new geometry).
fn marked_assembly_kicked(set_index: usize, size: u32, weight: i16) -> Vec<(u32, u32)> {
    let image = prior_image();
    kicked_on(
        marked_engine(&image, set_index, size, true, Some(weight)),
        set_index,
        size,
    )
}

/// The body of `marked_assembly_kicked` on an engine already built — its members of `size`
/// marked under set `set_index` and wired (brief 051's gate builds one with its inhibition).
fn kicked_on(exec: Engine, set_index: usize, size: u32) -> Vec<(u32, u32)> {
    kicked_on_keeping(exec, set_index, size).0
}

/// `kicked_on`, returning the engine it kicked beside the members' spikes (brief 052's gate reads
/// the members' slow potentials after the kick).
fn kicked_on_keeping(mut exec: Engine, set_index: usize, size: u32) -> (Vec<(u32, u32)>, Engine) {
    let set = members(size);
    let before = weights_of(&exec);
    let rested: Vec<(u8, u8, u32)> = set
        .iter()
        .map(|&m| {
            let u = &exec.units()[m as usize];
            (u.stp_u_rel, u.stp_r_ves, u.last_soma_spike_tick)
        })
        .collect();
    let drive = drive(1024);
    let inject = exec.injector();
    let start = exec.ticks() as u32;
    let mut resets = Vec::new();
    for k in 0..GATE_KICK_TICKS_049 {
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
    for (k, &m) in set.iter().enumerate() {
        assert_eq!(
            spikes
                .iter()
                .filter(|&&(t, u)| u == m && (1..=KICK_SPAN).contains(&t))
                .count(),
            1,
            "member {m} fires once in the span"
        );
        let (u0, r0, last) = rested[k];
        let mut copy = DendriticSuperNeuron::new(0);
        copy.stp_u_rel = u0;
        copy.stp_r_ves = r0;
        copy.flags = FLAG_FACILITATING;
        let mut previous = (last != NO_SPIKE_ON_RECORD).then_some(last);
        for &(t, _) in spikes.iter().filter(|&&(_, u)| u == m) {
            let stamp = start.wrapping_add(t);
            let elapsed = previous.map_or(u32::MAX, |p| stamp.wrapping_sub(p));
            copy.step_stp_class(elapsed, SETS[set_index]);
            previous = Some(stamp);
        }
        let unit = &exec.units()[m as usize];
        assert_eq!(
            (unit.stp_u_rel, unit.stp_r_ves),
            (copy.stp_u_rel, copy.stp_r_ves),
            "member {m} under the class"
        );
    }
    (spikes, exec)
}

/// The rules over the pinned tables of brief 049: each run's stretches and kicks as the rules
/// read them; each set's backgrounds; the kick firing every member once at every size under each
/// set; each control's lead-in and first unkicked span the background's; every cell's reading;
/// and the usable region as ADR-0115 reads it, one cell, with what failed in every other.
fn over_the_049_tables() {
    let mut usable = Vec::new();
    let mut failures = [0u32; 3];
    for set in 0..SETS.len() {
        let spans = layout(RELEASE_STRETCHES_049[set]);
        let n = stretches_of(&spans).len();
        let first = stretches_of(&spans)
            .iter()
            .position(|s| s.0 == Span::Hold)
            .expect("a hold span");
        for (s, &size) in SIZES.iter().enumerate() {
            let (bg_rows, ctl_rows) = (BACKGROUND_049[set][s], CONTROL_049[set][s]);
            assert_eq!((bg_rows.len(), ctl_rows.len()), (n, n));
            let bg = background_049(bg_rows, &spans);
            assert_eq!(bg, BACKGROUNDS_049[set][s]);
            assert_eq!(
                kick_reading_049(CONTROL_KICKS_049[set][s], size),
                KICKS_049[set][s]
            );
            assert!(
                kicked_once_049(CONTROL_KICKS_049[set][s], size),
                "the kick fires every member of {size} once under {}",
                SET_NAMES[set]
            );
            assert_eq!(holding(ctl_rows, &spans, bg), CONTROLS_049[set][s]);
            assert_eq!(
                ctl_rows[..first],
                bg_rows[..first],
                "the growth changes nothing unkicked"
            );
            for (k, rows) in CELLS_049[set][s].iter().enumerate() {
                assert_eq!(rows.len(), n);
                let read = holding(rows, &spans, bg);
                assert_eq!(
                    read, GRID_049[set][s][k],
                    "{} {size} at weight {k}",
                    SET_NAMES[set]
                );
                assert_eq!(CELL_KICKS_049[set][s][k].len(), ROUNDS, "a kick a round");
                if read.usable() {
                    usable.push((set, size, WEIGHTS_049[k]));
                }
                for (count, f) in failures.iter_mut().zip(failed(&read)) {
                    *count = count.saturating_add(u32::from(f));
                }
            }
        }
    }
    assert_eq!(
        usable,
        [(1, 64, 0x2000)],
        "one cell usable: 64 units under set (ii) at a quarter of the range"
    );
    assert_eq!(
        failures,
        [5, 33, 30],
        "never holding, running away, not letting go, over the 36 cells"
    );
}

// ------------------------------------------------ the runs, weekly (brief 049)

/// The kick, the backgrounds and the controls under set `set` (brief 049, the order ADR-0115
/// writes): on ADR-0077's settled image frozen, for each size, the background — the members
/// marked, unwired, the protocol's ticks under the drive with no kick and no release — and the
/// control — the grown image, the members marked and unwired, the kick and the release — before
/// any cell is run. Each control's lead-in and first unkicked span are its background's bit for
/// bit, and the kick fires every member once by ADR-0112's measure, or no cell is run.
fn kick_backgrounds_controls(set: usize) {
    let name = format!("the backgrounds and the controls under {}", SET_NAMES[set]);
    let image = settled(&name);
    let spans = layout(RELEASE_STRETCHES_049[set]);
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let mut read = Vec::new();
    for (s, &size) in SIZES.iter().enumerate() {
        let bg = class_run(
            &image,
            set,
            size,
            None,
            true,
            &format!("BACKGROUND_049[{set}][{s}]"),
        );
        let ctl = class_run(
            &image,
            set,
            size,
            None,
            false,
            &format!("CONTROL_049[{set}][{s}]"),
        );
        let (bg_rows, ctl_rows) = (
            stretch_rows(&bg.windows, size),
            stretch_rows(&ctl.windows, size),
        );
        let background = background_049(&bg_rows, &spans);
        let kick = kick_reading_049(&ctl.kicks, size);
        let once = kicked_once_049(&ctl.kicks, size);
        let control = holding(&ctl_rows, &spans, background);
        let bursts = bursts_049(&ctl.windows, &spans, size, background, SETS[set]);
        eprintln!(
            "DUMP {name} {size}: background {background:?} kick {kick:?} once {once} control {control:?} bursts {bursts:?} hashes {:#018x} {:#018x}",
            windows_hash(&bg.windows),
            windows_hash(&ctl.windows)
        );
        read.push((
            bg, ctl, bg_rows, ctl_rows, background, kick, once, control, bursts,
        ));
    }
    for (s, &size) in SIZES.iter().enumerate() {
        let (_, _, bg_rows, ctl_rows, _, _, once, _, _) = &read[s];
        assert_eq!(
            ctl_rows[..first],
            bg_rows[..first],
            "{name} {size}: the growth changes nothing unkicked"
        );
        assert!(
            *once,
            "{name} {size}: the kick fires every member once, or it is derived again"
        );
    }
    for (s, (bg, ctl, bg_rows, ctl_rows, background, kick, once, control, bursts)) in
        read.iter().enumerate()
    {
        assert_eq!(
            bg_rows.as_slice(),
            BACKGROUND_049[set][s],
            "the background {s}"
        );
        assert_eq!(ctl_rows.as_slice(), CONTROL_049[set][s], "the control {s}");
        assert_eq!(ctl.kicks.as_slice(), CONTROL_KICKS_049[set][s]);
        assert_eq!(
            [windows_hash(&bg.windows), windows_hash(&ctl.windows)],
            WINDOWS_HASH_049[set][s],
            "every window"
        );
        assert_eq!(*background, BACKGROUNDS_049[set][s]);
        assert_eq!(*kick, KICKS_049[set][s]);
        assert!(*once);
        assert_eq!(*control, CONTROLS_049[set][s]);
        assert_eq!(*bursts, CONTROL_BURSTS_049[set][s]);
    }
}

/// The six weights of one size under set `set` (brief 049): on ADR-0077's settled image frozen,
/// the members marked, grown by the assembly's blocks and wired at each weight in `WEIGHTS_049`'s
/// order, the protocol, the weights shown unchanged at each run's end; each run read against the
/// background pinned before any cell ran, and every run dumped before any is held to its table.
fn six_weights(set: usize, s: usize) {
    let size = SIZES[s];
    let name = format!("the assembly of {size} under {}", SET_NAMES[set]);
    let image = settled(&name);
    let spans = layout(RELEASE_STRETCHES_049[set]);
    let bg = background_049(BACKGROUND_049[set][s], &spans);
    assert_eq!(
        bg, BACKGROUNDS_049[set][s],
        "{name}: the background, pinned first"
    );
    let mut runs = Vec::new();
    for (k, &w) in WEIGHTS_049.iter().enumerate() {
        let run = class_run(
            &image,
            set,
            size,
            Some(w),
            false,
            &format!("CELLS_049[{set}][{s}][{k}]"),
        );
        let rows = stretch_rows(&run.windows, size);
        let read = holding(&rows, &spans, bg);
        let bursts = bursts_049(&run.windows, &spans, size, bg, SETS[set]);
        eprintln!(
            "DUMP {name} at {w:#x}: {read:?} holds {} lets go {} quiet {} usable {} failed {:?} kick {:?} bursts {bursts:?}",
            read.holds(),
            read.lets_go(),
            read.quiet_unkicked(),
            read.usable(),
            failed(&read),
            kick_reading_049(&run.kicks, size)
        );
        runs.push((run, rows, read, bursts));
    }
    for (k, (run, rows, read, bursts)) in runs.iter().enumerate() {
        assert_eq!(rows.as_slice(), CELLS_049[set][s][k], "{name}: weight {k}");
        assert_eq!(run.kicks.as_slice(), CELL_KICKS_049[set][s][k]);
        assert_eq!(windows_hash(&run.windows), CELL_HASH_049[set][s][k]);
        assert_eq!(*read, GRID_049[set][s][k], "{name}: weight {k}");
        assert_eq!(*bursts, CELL_BURSTS_049[set][s][k], "{name}: weight {k}");
    }
}

#[test]
#[ignore]
fn the_kick_the_backgrounds_and_the_controls_under_set_i_at_1024_units_paused() {
    kick_backgrounds_controls(0);
}

#[test]
#[ignore]
fn the_kick_the_backgrounds_and_the_controls_under_set_ii_at_1024_units_paused() {
    kick_backgrounds_controls(1);
}

#[test]
#[ignore]
fn a_marked_assembly_of_16_units_at_six_weights_under_set_i_paused() {
    six_weights(0, 0);
}

#[test]
#[ignore]
fn a_marked_assembly_of_32_units_at_six_weights_under_set_i_paused() {
    six_weights(0, 1);
}

#[test]
#[ignore]
fn a_marked_assembly_of_64_units_at_six_weights_under_set_i_paused() {
    six_weights(0, 2);
}

#[test]
#[ignore]
fn a_marked_assembly_of_16_units_at_six_weights_under_set_ii_paused() {
    six_weights(1, 0);
}

#[test]
#[ignore]
fn a_marked_assembly_of_32_units_at_six_weights_under_set_ii_paused() {
    six_weights(1, 1);
}

#[test]
#[ignore]
fn a_marked_assembly_of_64_units_at_six_weights_under_set_ii_paused() {
    six_weights(1, 2);
}

// ------------------------------------------------ brief 049's arithmetic, pinned before any run

/// The pair a marked member's spike after a long rest releases with, per set.
const CLASS_AT_REST_049: [(u8, u8); 2] = [(92, 255), (49, 255)];

/// The unkicked product, per set: $uR$ of the steady pair at the background's interval.
const UNKICKED_PRODUCT_049: [u32; 2] = [28_815, 16_065];

/// The fade from the ceiling, per set, in ticks.
const FADE_049: [u32; 2] = [135_785, 180_192];

/// The release's span, per set, in stretches.
const RELEASE_STRETCHES_049: [u32; 2] = [5, 6];

/// A run's epochs, per set: the lead-in and eight rounds.
const RUN_EPOCHS_049: [u32; 2] = [384, 400];

/// The release by the oracle over its span, per set, from the drive's mean standing and from the
/// extreme: the ticks it fires on, its lowest basal potential, and the tick after the last cancel
/// its soma is back within a tenth of the threshold of the drive's mean standing.
const RELEASE_SPAN_ORACLE_049: [[ReleasePinned; 2]; 2] = [[(&[], -12_461_267, Some(3_519)); 2]; 2];

/// A release by the oracle as pinned: its ticks borrowed.
type ReleasePinned = (&'static [u32], i32, Option<u32>);

// ------------------------------------------------ brief 049's readings, pinned from the runs

// The runs' stretches, per set and size (brief 049).

// The runs' stretches, per set and size and weight (brief 049).

// ================================================ brief 050 (ADR-0117): the region before the readout

// ------------------------------------------------ the geometry (ADR-0116), before any run

/// Readout 0 of the geometry beside the task's (ADR-0116, F-54 decided): `R0_MASK` without
/// place 5, the eight odd places but 5 and 11 — 1, 3, 7, 9, 13, 15, 17 and 19.
const R0_MASK_050: u32 = R0_MASK & !PLACES;

/// Readout 1 of that geometry: `R1_MASK` without place 16, the eight even places but 0 and 16 —
/// 2, 4, 6, 8, 10, 12, 14 and 18.
const R1_MASK_050: u32 = R1_MASK & !PLACES;

/// The two places the readouts give up, one mask each: 5, readout 0's, and 16, readout 1's.
const CONTEXT_PLACE_MASKS: [u32; 2] = [1 << 5, 1 << 16];

const _: () = assert!(R0_MASK_050 == 0xAA28A && R1_MASK_050 == 0x45554);
const _: () = assert!(R0_MASK_050.count_ones() == 8 && R1_MASK_050.count_ones() == 8);
const _: () = assert!(CONTEXT_PLACE_MASKS[0] | CONTEXT_PLACE_MASKS[1] == PLACES);
const _: () =
    assert!((R0_MASK_050 & R1_MASK_050) == 0 && (PLACES & (R0_MASK_050 | R1_MASK_050)) == 0);
const _: () = assert!(
    ((1 << A_OFFSET) | (1 << B_OFFSET) | R0_MASK_050 | R1_MASK_050 | PLACES) == (1 << PERIOD) - 1
);

/// The geometry beside the task's (ADR-0116): `geometry`'s two stimuli over the same whole
/// periods from `rotation`, and its two readouts without the places the context takes, eight
/// places each. `geometry` is unchanged and every earlier test keeps it; this one lives beside
/// the only runs that read it (ADR-0117).
fn context_geometry(units: u32, rotation: u32) -> [Set; 4] {
    let [a, b, r0, r1] = geometry(units, rotation);
    [
        a,
        b,
        Set {
            mask: R0_MASK_050,
            ..r0
        },
        Set {
            mask: R1_MASK_050,
            ..r1
        },
    ]
}

/// The places the readouts give up, over the geometry's whole periods, both places or one
/// (`mask`): at 1 024 units, 102 units, fifty-one periods of two, in no set of
/// `context_geometry`, and every assembly of the grid among them.
fn context_places(units: u32, rotation: u32, mask: u32) -> Set {
    Set {
        mask,
        ..geometry(units, rotation)[0]
    }
}

// ------------------------------------------------ the grid, the subset and the conditions (brief 050)

/// The core grid's sizes: 48, 64, 80 and 96 units at ADR-0112's placement (`members`), nested.
/// Each member sends and receives 32 synapses at every one of them (`fan`).
const SIZES_050: [u32; 4] = [48, 64, 80, 96];

/// The core grid's weights, Q1.15: an eighth to three eighths of the range by sixteenths.
/// ADR-0115's 0.25 and 0.375 are among them.
const WEIGHTS_050: [i16; 5] = [0x1000, 0x1800, 0x2000, 0x2800, 0x3000];

/// The set every run of brief 050 marks its members under: ADR-0113's (ii).
const SET_050: usize = 1;

/// ADR-0115's two cells in the grid, `(size, weight)` by index — 64 units at 0.25 and at 0.375 —
/// and the places of their weights in ADR-0115's grid at 64 units (`WEIGHTS_049`).
const ADR_0115_CELLS: [(usize, usize); 2] = [(1, 2), (1, 4)];

const ADR_0115_WEIGHTS: [usize; 2] = [0, 1];

/// The subset the three conditions run on, by index into the grid: sizes 64 and 96, weights
/// 0.1875, 0.25 and 0.3125.
const SUBSET_SIZES: [usize; 2] = [1, 3];

const SUBSET_WEIGHTS: [usize; 3] = [1, 2, 3];

/// Condition (b)'s seed of the assembly's delays: the next after `DELAY_SEED`.
const DELAY_SEED_050: u64 = 49;

const _: () = assert!(DELAY_SEED_050 == DELAY_SEED + 1);

/// The three conditions, in the order ADR-0116 names them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Condition {
    /// (b): the settled image, the assembly's delays drawn with `DELAY_SEED_050`.
    Seed,
    /// (c): the image H-20's arm from the assignment leaves at its last trial, frozen.
    Drained,
    /// (d): the settled image, the task's stimulus at every epoch's middle.
    Stimulus,
}

const CONDITIONS: [Condition; 3] = [Condition::Seed, Condition::Drained, Condition::Stimulus];

/// The conditions' names, as ADR-0116 letters them.
const CONDITION_NAMES: [&str; 3] = ["(b)", "(c)", "(d)"];

// ------------------------------------------------ the rules (brief 050), before any run

/// The core grid's readings by ADR-0115's rules, `[size][weight]` over `SIZES_050` and
/// `WEIGHTS_050`.
type Grid050 = [[Holding; 5]; 4];

/// A condition's readings by ADR-0115's rules, `[size][weight]` over `SUBSET_SIZES` and
/// `SUBSET_WEIGHTS`.
type Subset050 = [[Holding; 3]; 2];

/// Robust: the subset's cells, `(size, weight)` by index into the grid, usable in the core grid
/// and under each of (b), (c) and (d), in the grid's order.
fn robust_cells(core: &Grid050, under: &[Subset050; 3]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (i, &s) in SUBSET_SIZES.iter().enumerate() {
        for (j, &w) in SUBSET_WEIGHTS.iter().enumerate() {
            if core[s][w].usable() && under.iter().all(|c| c[i][j].usable()) {
                out.push((s, w));
            }
        }
    }
    out
}

/// A cell's usable neighbours in the core grid: of the cells one weight step either way at its
/// size and one size step either way at its weight, at most four, those usable. Over any grid of
/// readings: brief 051's steps are a recurrent weight and an inhibitory weight.
fn usable_neighbours<const S: usize, const W: usize>(
    core: &[[Holding; W]; S],
    (s, w): (usize, usize),
) -> u32 {
    let near = [
        (s.checked_sub(1), Some(w)),
        (s.checked_add(1), Some(w)),
        (Some(s), w.checked_sub(1)),
        (Some(s), w.checked_add(1)),
    ];
    let mut n = 0u32;
    for pair in near {
        if let (Some(ns), Some(nw)) = pair {
            let usable = core
                .get(ns)
                .and_then(|row| row.get(nw))
                .is_some_and(Holding::usable);
            n = n.saturating_add(u32::from(usable));
        }
    }
    n
}

/// The cell for the second round: of the robust cells, the one with the most usable neighbours
/// in the core grid; ties to the lighter weight, then the smaller size. None when no cell is
/// robust.
fn round_two_cell(core: &Grid050, robust: &[(usize, usize)]) -> Option<(usize, usize)> {
    let mut ordered = robust.to_vec();
    ordered.sort_by_key(|&(s, w)| (w, s));
    let mut best: Option<((usize, usize), u32)> = None;
    for cell in ordered {
        let n = usable_neighbours(core, cell);
        if best.is_none_or(|(_, most)| n > most) {
            best = Some((cell, n));
        }
    }
    best.map(|(cell, _)| cell)
}

/// What failed in each cell of the subset, `[size][weight]`, in the core grid and under (b),
/// (c) and (d) in that order, each by ADR-0115's `failed`: the reading ADR-0117 gives where no
/// cell is robust.
fn failures_050(core: &Grid050, under: &[Subset050; 3]) -> [[[[bool; 3]; 4]; 3]; 2] {
    let mut out = [[[[false; 3]; 4]; 3]; 2];
    for (i, &s) in SUBSET_SIZES.iter().enumerate() {
        for (j, &w) in SUBSET_WEIGHTS.iter().enumerate() {
            out[i][j][0] = failed(&core[s][w]);
            for (c, condition) in under.iter().enumerate() {
                out[i][j][c.saturating_add(1)] = failed(&condition[i][j]);
            }
        }
    }
    out
}

// ------------------------------------------------ the arithmetic (brief 050), before any run

/// What one spike of a marked member delivers at each of `weights` under `class`, at the
/// unkicked steady pair and at the primed peak's pair: ADR-0112's `Delivered` of each.
fn delivered_at(class: StpClass, weights: &[i16]) -> Vec<Delivered> {
    let unkicked = steady_under(INTERVALS[0], Some(class));
    let peak = primed(class).4;
    weights
        .iter()
        .map(|&w| {
            let at = delivered(w, unkicked);
            let primed = delivered(w, peak);
            (
                at,
                rise(at).0,
                least_together(w, unkicked),
                primed,
                rise(primed).0,
                least_together(w, peak),
            )
        })
        .collect()
}

/// The reach of the task's stimulus on `exec`'s synapses: for each size of the grid, the
/// synapses stimulus A's units and stimulus B's send onto its members and the members that
/// receive one, `[A, B, members]`; and over the whole ring, the synapses and the sum of their
/// weights from each stimulus onto each place the readouts give up, `[A, B][place 5, place 16]`.
#[allow(clippy::type_complexity)]
fn stimulus_reach(exec: &Engine) -> ([[u32; 3]; 4], [[(u32, i64); 2]; 2]) {
    let [a, b, _, _] = context_geometry(1024, ROTATION_1024);
    let places = CONTEXT_PLACE_MASKS.map(|m| context_places(1024, ROTATION_1024, m));
    let mut by_size = [[0u32; 3]; 4];
    let mut ring = [[(0u32, 0i64); 2]; 2];
    let mut reached = [vec![false; 1024], vec![false; 1024]];
    let of: Vec<Vec<bool>> = SIZES_050
        .iter()
        .map(|&size| membership(&members(size)))
        .collect();
    for unit in exec.units() {
        let id = unit.id as u32;
        let from = if a.contains(id) {
            0
        } else if b.contains(id) {
            1
        } else {
            continue;
        };
        for s in unit.fan_out(exec.blocks()) {
            for (p, place) in places.iter().enumerate() {
                if place.contains(s.target) {
                    let cell = &mut ring[from][p];
                    cell.0 = cell.0.saturating_add(1);
                    cell.1 = cell.1.saturating_add(i64::from(s.weight_q1_15));
                }
            }
            reached[from][s.target as usize] = true;
            for (k, of_size) in of.iter().enumerate() {
                if of_size[s.target as usize] {
                    by_size[k][from] = by_size[k][from].saturating_add(1);
                }
            }
        }
    }
    for (k, &size) in SIZES_050.iter().enumerate() {
        let hit = members(size)
            .iter()
            .filter(|&&m| reached[0][m as usize] || reached[1][m as usize])
            .count();
        by_size[k][2] = hit as u32;
    }
    (by_size, ring)
}

/// Condition (b)'s wiring on the instrument's network at `size`: the assembly wired at `weight`
/// with the delays of `DELAY_SEED` and with those of `DELAY_SEED_050`, every target, weight and
/// link of the two the same and every delay in the prior's local band; the synapses whose delay
/// differs, and all the assembly's synapses.
fn seeded_wiring(size: u32, weight: i16) -> (u32, u32) {
    let image = grown(&prior_image(), extra_blocks(size));
    let base = blocks_for(&prior(1024)) as usize;
    let mut first = frozen_from(&image, 1024);
    let mut second = frozen_from(&image, 1024);
    wire_drawn(&mut first, size, weight, DELAY_SEED);
    wire_drawn(&mut second, size, weight, DELAY_SEED_050);
    let p = prior(1024);
    let (mut differ, mut all) = (0u32, 0u32);
    for (x, y) in first.blocks()[base..].iter().zip(&second.blocks()[base..]) {
        assert_eq!(x.next(), y.next(), "the same links");
        for slot in 0..SYNAPSES_PER_BLOCK {
            assert_eq!(x.target(slot), y.target(slot), "the same targets");
            assert_eq!(
                x.weights_q1_15[slot], y.weights_q1_15[slot],
                "the same weights"
            );
            if x.target(slot).is_none() {
                continue;
            }
            for d in [x.delays_ticks[slot], y.delays_ticks[slot]] {
                assert!(
                    (p.delay_min..=p.delay_max).contains(&d),
                    "a delay in the local band"
                );
            }
            all = all.saturating_add(1);
            differ = differ.saturating_add(u32::from(x.delays_ticks[slot] != y.delays_ticks[slot]));
        }
    }
    (differ, all)
}

// ------------------------------------------------ condition (c): the image H-20's arm leaves

/// The modulator record's bytes condition (c) writes: the dopamine signal (`[0..4)`, the first
/// field of the modulator's sixteen bytes), the inhibitory baseline's flag and value (`[24]`,
/// `[28..32)`, ADR-0086) and the signed gate's flag (`[25]`, ADR-0094), as `Image::encode` lays
/// them.
const SIGNAL_BYTES: core::ops::Range<usize> = 0..4;

const INHIBITORY_FLAG_BYTE: usize = 24;

const INHIBITORY_VALUE_BYTES: core::ops::Range<usize> = 28..32;

const SIGNED_GATE_BYTE: usize = 25;

/// `image` with its modulator record rewritten by `write` and the section re-sealed; every other
/// byte the image's.
fn with_modulator(image: &[u8], write: impl Fn(&mut [u8])) -> Vec<u8> {
    let mut img = image.to_vec();
    let header = MenteionFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_MODULATOR {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            write(&mut img[offset..][..length]);
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            return img;
        }
    }
    panic!("the image holds a modulator section");
}

/// H-20's image (ADR-0109): the settled image with the inhibitory baseline set at 0.5 and the
/// signed gate set, the bytes `tests/inhibition.rs` writes (`inhibited_image`, then
/// `signed_image`), held to that file's CRC-64 (`H20_IMAGE_CRC`).
fn h20_image(settled: &[u8]) -> Vec<u8> {
    with_modulator(settled, |m| {
        assert_eq!(
            (m[INHIBITORY_FLAG_BYTE], m[SIGNED_GATE_BYTE]),
            (0, 0),
            "both unset before"
        );
        m[INHIBITORY_FLAG_BYTE] = 1;
        m[INHIBITORY_VALUE_BYTES].copy_from_slice(&BASELINE_Q16.to_le_bytes());
        m[SIGNED_GATE_BYTE] = 1;
    })
}

/// An image a learning run leaves, frozen (ADR-0117): the dopamine signal the last reward left
/// put at rest, the inhibitory baseline and the signed gate unset; every other byte the image's.
/// Under the baseline of zero H-20's image carries, no synapse then consolidates: an excitatory
/// one's modulation is the baseline plus the signal, zero, and an inhibitory one's the same with
/// its own baseline unset.
fn frozen_again(image: &[u8]) -> Vec<u8> {
    with_modulator(image, |m| {
        m[SIGNAL_BYTES].copy_from_slice(&[0; 4]);
        m[INHIBITORY_FLAG_BYTE] = 0;
        m[INHIBITORY_VALUE_BYTES].copy_from_slice(&[0; 4]);
        m[SIGNED_GATE_BYTE] = 0;
    })
}

/// H-20's image by its CRC-64: `PUNISHED_IMAGE_CRC_1024` of `tests/inhibition.rs`, restated;
/// ADR-0123 moved the format from 17 to 18, ADR-0131 from 18 to 19, ADR-0134 from 19 to 20,
/// ADR-0155 from 20 to 21, ADR-0180 from 21 to 22, ADR-0189 from 22 to 23, ADR-0194 from 23 to
/// 24 and ADR-0202 from 24 to 25, each re-pinning it (it was `0x766d_e462_f9b7_7576` at format
/// 17, `0xb548_6d72_bb9c_5818` at 18, `0xf454_ea7d_7a7a_bcc2` at 19, `0xa1db_d079_90c4_1c41` at
/// 20, `0xe0c7_5776_5122_f89b` at 21, `0x23e2_de66_1309_d5f5` at 22, `0x62fe_5969_d2ef_312f` at
/// 23 and `0x88fc_aa6f_c674_94f3` at 24). ADR-0221 moved the format to 26 for the magic alone, and
/// the pin holds the image at 25 since (`pinned_crc`, ADR-0222).
const H20_IMAGE_CRC: u64 = 0xc9e0_2d60_0792_7029;

/// An image with its header's version written as `version` and the header resealed, every other
/// byte as it was (ADR-0095): the image a writer of that version would have produced from the
/// same records, where no record's bytes differ between the two versions. `tests/inhibition.rs`'s,
/// written here a second time for the drained image. Below 26 the magic is written back too,
/// `VCORTEX1`, the magic of every format before ADR-0221's (ADR-0222).
fn with_version(image: &[u8], version: u32) -> Vec<u8> {
    let mut img = image.to_vec();
    let mut header = MenteionFileHeader::decode(img[0..64].try_into().unwrap());
    header.version = version;
    if version < 26 {
        header.magic = *b"VCORTEX1";
    }
    header.crc64 = header.checksum();
    img[0..64].copy_from_slice(&header.encode());
    img
}

/// The format this file's whole-image pins hold their images at, `tests/inhibition.rs`'s
/// (ADR-0222): 25, the last whose magic was `VCORTEX1`.
const PINNED_FORMAT: u32 = 25;

/// An image's CRC-64 at `PINNED_FORMAT`: the CRC its pin holds.
fn pinned_crc(image: &[u8]) -> u64 {
    crc64(&with_version(image, PINNED_FORMAT))
}

/// The frozen drained image's CRC-64 with its header's version written back to 17 and resealed:
/// the CRC ADR-0117 read (`DRAINED_050`'s at format 17), so that every byte of the image but the
/// version and the seal is the image ADR-0117 and ADR-0120 ran from (ADR-0123, as ADR-0095).
const DRAINED_IMAGE_CRC_FORMAT_17: u64 = 0xd763_b4a5_8364_291e;

/// H-20's run (ADR-0109): 7 680 trials, the mapping flipped before the trials of index 1 536,
/// 3 584 and 5 632 (`SCHEDULE_TRIALS` and `SCHEDULE_FLIPS` of `tests/inhibition.rs`).
const H20_TRIALS: usize = 7_680;

const H20_FLIPS: [usize; 3] = [1_536, 3_584, 5_632];

/// H-19's critic (ADR-0106): an expectation moves by a thirty-second of the error.
const H20_CRITIC_SHIFT: u32 = 5;

/// ADR-0110's arm from the assignment as its table pins it: the accuracy sequence's FNV-1a
/// (`SCHEDULE_TRACES_1024[0]`) and the arena's sums by polarity after its last block
/// (`SUMS_AFTER_SCHEDULE_1024[0]`), restated.
const H20_TRACE: u64 = 0xdd2a_9318_b3c5_d288;

const H20_SUMS: (i64, i64) = (11_999_067, 219_302_252);

/// The task of H-20's arm from the assignment: F-46's shape with ADR-0076's cancel, the answer's
/// feedback, the addressed delivery, stimulus A's answer readout 0 first, and H-19's critic from
/// expectations of zero; on the task's geometry, the one it learned on.
fn h20_task() -> Task {
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    Task {
        critic: Some(Critic::new(H20_CRITIC_SHIFT)),
        ..task(
            SHAPE_F46,
            Some(cancel_of(picked)),
            1024,
            Feedback::Answer,
            false,
            Delivery::Addressed,
        )
    }
}

/// What producing condition (c)'s image reads: the arm's accuracy sequence and the arena's sums
/// by polarity at its end; the ticks the quiet run before the image took; the dopamine signal
/// the image carried before it was frozen; the frozen image's sums and its CRC-64; and the
/// excitatory couplings from each stimulus onto each place the readouts give up, `[A, B][place
/// 5, place 16]`, on the settled image and on the drained one.
type DrainedRead = (
    u64,
    (i64, i64),
    u64,
    i32,
    (i64, i64),
    u64,
    [[[i64; 2]; 2]; 2],
);

/// The couplings from each stimulus onto each place the readouts give up, on `exec`.
fn context_couplings(exec: &Engine) -> [[i64; 2]; 2] {
    let [a, b, _, _] = context_geometry(1024, ROTATION_1024);
    let places = CONTEXT_PLACE_MASKS.map(|m| context_places(1024, ROTATION_1024, m));
    [a, b].map(|from| places.map(|into| coupling(exec, from, into)))
}

/// Condition (c)'s image (ADR-0117): H-20's arm from the assignment run from H-20's image as
/// `tests/inhibition.rs` runs it, without its oracle, which reads and never writes, and held to
/// ADR-0110's accuracy sequence and sums; then quieted as every image is before it is written,
/// written, and frozen again (`frozen_again`). Returns the frozen image and its reading.
fn drained_image(name: &str, settled: &[u8]) -> (Vec<u8>, DrainedRead) {
    let h20 = h20_image(settled);
    assert_eq!(
        pinned_crc(&h20),
        H20_IMAGE_CRC,
        "{name}: H-20's image, by its CRC"
    );
    let mut exec = Image::decode::<2048>(&h20, config(1024, 2, 0)).expect("a well-formed record");
    assert_eq!(
        (
            exec.modulation_baseline_q16(),
            exec.inhibitory_baseline_q16(),
            exec.signed_gate(),
            exec.modulator().dopamine_rpe
        ),
        (0, Some(BASELINE_Q16), true, 0),
        "{name}: H-20's configuration"
    );
    let (blocks, trace) = run_on_scheduled(
        &mut exec,
        h20_task(),
        1024,
        H20_TRIALS,
        &H20_FLIPS,
        &mut |_, _, _, _| {},
    );
    let sums = weights_by_polarity(&exec);
    eprintln!(
        "DUMP {name} H-20's arm: trace {trace:#018x} sums {sums:?} curve {:?}",
        curve(&blocks)
    );
    assert_eq!(
        (trace, sums),
        (H20_TRACE, H20_SUMS),
        "{name}: ADR-0110's arm from the assignment, its accuracy sequence and its sums"
    );
    let ticks = quiet(&mut exec);
    let signal = exec.modulator().dopamine_rpe;
    let image = frozen_again(&Image::encode(&exec).expect("quiescent"));
    assert_eq!(
        crc64(&with_version(&image, 17)),
        DRAINED_IMAGE_CRC_FORMAT_17,
        "{name}: every byte but the header's version and seal is the image ADR-0117 froze"
    );
    let frozen = frozen_from(&image, 1024);
    assert_eq!(
        (
            frozen.inhibitory_baseline_q16(),
            frozen.signed_gate(),
            frozen.modulator().dopamine_rpe,
            frozen.stp_class()
        ),
        (None, false, 0, None),
        "{name}: frozen, no class"
    );
    let settled_couplings = context_couplings(&frozen_from(settled, 1024));
    let read = (
        trace,
        sums,
        ticks,
        signal,
        weights_by_polarity(&frozen),
        pinned_crc(&image),
        [settled_couplings, context_couplings(&frozen)],
    );
    eprintln!("DUMP {name} DRAINED_050 = {read:?}");
    (image, read)
}

// ------------------------------------------------ condition (d): the task's stimulus

/// The tick of every epoch before which condition (d) presents the task's stimulus: the epoch's
/// middle. The kick's reading — its span and the pair window after it — closes 5 444 ticks before
/// the window the trial reads before its stimulus opens; the stimulus's cancel and the window it
/// reads after it end inside the epoch, as the prior's far band does (the gate); and the next
/// epoch's kick comes 8 192 ticks after the stimulus, past the far band.
const STIMULUS_AT: u32 = EPOCH_TICKS / 2;

/// The trial's window before its stimulus (`WINDOW.ticks` before it, as `run_on_scheduled`
/// reads it) and after it (`WINDOW`), in the epoch's ticks.
const BEFORE_FROM: u32 = STIMULUS_AT - WINDOW.ticks;

const AFTER_FROM: u32 = STIMULUS_AT + WINDOW.from;

const AFTER_END: u32 = AFTER_FROM + WINDOW.ticks;

const _: () = assert!(KICK_SPAN + PAIR_WINDOW < BEFORE_FROM);
const _: () = assert!(STIMULUS_AT + CANCEL_OFFSET + CANCEL_TICKS < EPOCH_TICKS);
const _: () = assert!(AFTER_END < EPOCH_TICKS && KICK_SPAN + PAIR_WINDOW == 2_248);

/// The task whose stimulus condition (d) presents: H-20's arms' — F-46's shape, ADR-0076's cancel,
/// A or B by the task's draw from `SEED` — and nothing else of it: no trial is run, no readout
/// selects and no reward is delivered.
fn stimulus_task() -> Task {
    let picked = CANCEL_PICKED_1024.expect("ADR-0076 picked a cancel");
    task(
        SHAPE_F46,
        Some(cancel_of(picked)),
        1024,
        Feedback::Withheld,
        false,
        Delivery::Addressed,
    )
}

/// One epoch's reading under (d): the stimulus presented, and the spikes in the trial's window
/// before the stimulus and in its window after it, `[before, after]`, of each of the eight-place
/// readouts and of the places they give up, `[R0, R1, place 5, place 16]`.
type TaskEpoch = (u8, [[u32; 4]; 2]);

/// The sets an epoch's reading counts, as memberships of the arena: `context_geometry`'s two
/// readouts and `context_places` by place.
fn counted_sets() -> [Vec<bool>; 4] {
    let [_, _, r0, r1] = context_geometry(1024, ROTATION_1024);
    let [p5, p16] = CONTEXT_PLACE_MASKS.map(|m| context_places(1024, ROTATION_1024, m));
    [r0, r1, p5, p16].map(|s| membership(&s.units().collect::<Vec<u32>>()))
}

/// Which window of the trial the epoch's tick `at` is in: 0 the window before the stimulus, 1 the
/// window after it, none elsewhere.
fn task_side(at: u32) -> Option<usize> {
    if (BEFORE_FROM..STIMULUS_AT).contains(&at) {
        Some(0)
    } else if (AFTER_FROM..AFTER_END).contains(&at) {
        Some(1)
    } else {
        None
    }
}

/// The kind of span an epoch is summed by under (d), from the epoch's index within its span: the
/// unkicked span, the hold's first half, its second half, the release, the tail; the lead-in is
/// summed by none.
fn task_kind(span: Span, epoch: u32) -> Option<usize> {
    match span {
        Span::LeadIn => None,
        Span::Unkicked => Some(0),
        Span::Hold if epoch < SECOND_HALF_FROM * STRETCH_EPOCHS => Some(1),
        Span::Hold => Some(2),
        Span::Release => Some(3),
        Span::Tail => Some(4),
    }
}

/// Under (d), a run's readout reading by kind of span (`task_kind`) and by stimulus `[A, B]`: the
/// trials, and the eight-place readouts' spikes in the window after the stimulus, `[R0, R1]`,
/// summed.
type TaskRead = [[(u32, [u64; 2]); 2]; 5];

fn task_read(epochs: &[TaskEpoch], spans: &[(Span, usize, u32)]) -> TaskRead {
    let mut out = [[(0u32, [0u64; 2]); 2]; 5];
    let mut at = 0usize;
    for &(span, _, n) in spans {
        for e in 0..n {
            if let (Some(kind), Some(&(stimulus, counts))) = (task_kind(span, e), epochs.get(at)) {
                let cell = &mut out[kind][usize::from(stimulus)];
                cell.0 = cell.0.saturating_add(1);
                for (sum, &count) in cell.1.iter_mut().zip(&counts[1]) {
                    *sum = sum.saturating_add(u64::from(count));
                }
            }
            at = at.saturating_add(1);
        }
    }
    out
}

/// ADR-0065's reading over one block of trials: the readouts' spikes after the stimulus and
/// before it, `[R0, R1]` each, and the trials in which the window after held more readout spikes
/// than the window before.
type Sight050 = ([u64; 2], [u64; 2], u32);

/// A run's trials after the lead-in read by ADR-0065's measure in blocks of `BLOCK`: for the
/// eight-place readouts, or when `nine` for the nine-place ones — each eight-place readout with
/// the place it gave up, the task's `R0_MASK` and `R1_MASK`.
fn sight_050(epochs: &[TaskEpoch], nine: bool) -> Vec<Sight050> {
    let lead = LEAD_IN_EPOCHS_049 as usize;
    epochs
        .get(lead..)
        .unwrap_or(&[])
        .chunks_exact(BLOCK)
        .map(|block| {
            let (mut after, mut before, mut seen) = ([0u64; 2], [0u64; 2], 0u32);
            for &(_, counts) in block {
                let spikes = |side: usize, r: usize| {
                    let place = if nine {
                        counts[side][r.saturating_add(2)]
                    } else {
                        0
                    };
                    u64::from(counts[side][r]).saturating_add(u64::from(place))
                };
                let mut totals = [0u64; 2];
                for r in 0..2 {
                    before[r] = before[r].saturating_add(spikes(0, r));
                    after[r] = after[r].saturating_add(spikes(1, r));
                    totals[0] = totals[0].saturating_add(spikes(0, r));
                    totals[1] = totals[1].saturating_add(spikes(1, r));
                }
                seen = seen.saturating_add(u32::from(totals[1] > totals[0]));
            }
            (after, before, seen)
        })
        .collect()
}

/// Whether a block's reading passes ADR-0065's measure: the harness's `calibrated` over a block
/// that holds those three readings and nothing else.
fn resolves(sight: &Sight050) -> bool {
    let &(after, before, seen) = sight;
    calibrated(&(
        0,
        0,
        [after, [0; 2]],
        [0; 2],
        [0; 2],
        before,
        seen,
        0,
        0,
        0,
        [[0; 2]; 2],
        0,
    ))
}

/// A run's epochs as one number: FNV-1a over every epoch's stimulus and eight counts in order.
fn epochs_hash(epochs: &[TaskEpoch]) -> u64 {
    let words: Vec<i32> = epochs
        .iter()
        .flat_map(|&(stimulus, counts)| {
            core::iter::once(i32::from(stimulus))
                .chain(counts.into_iter().flatten().map(|c| c as i32))
        })
        .collect();
    fnv1a_64(&words)
}

/// The hash of no epoch: a run with no task.
const NO_EPOCHS_HASH: u64 = 0xcbf2_9ce4_8422_2325;

// ------------------------------------------------ the runs (brief 050)

/// A run of brief 050 as read: the protocol's windows and kicks, its stretches, and its epochs
/// under (d).
struct Run050 {
    run: SpanRun,
    rows: Vec<StretchRow>,
    epochs: Vec<TaskEpoch>,
}

/// A run of brief 050 as pinned: its stretches in full, each hold span's kick reading (none for
/// a background), its windows' hash, and its epochs' hash (`NO_EPOCHS_HASH` without a task).
#[derive(Clone, Copy, Debug)]
struct Pinned050 {
    rows: &'static [StretchRow],
    kicks: &'static [[u32; 2]],
    windows: u64,
    epochs: u64,
}

impl Run050 {
    /// Holds the run to its pin: every stretch, the kicks unless it is a background, every
    /// window and every epoch.
    fn held_to(&self, pin: &Pinned050, background: bool, name: &str) {
        assert_eq!(self.rows.as_slice(), pin.rows, "{name}: the stretches");
        if !background {
            assert_eq!(self.run.kicks.as_slice(), pin.kicks, "{name}: the kicks");
        }
        assert_eq!(
            windows_hash(&self.run.windows),
            pin.windows,
            "{name}: every window"
        );
        assert_eq!(epochs_hash(&self.epochs), pin.epochs, "{name}: every epoch");
    }
}

/// A run of brief 050 under set (ii) for the assembly of `size` on `image`, dumped: a background
/// (not grown, no kick, no release), a control (grown, unwired, the kick and the release) or a
/// cell (wired at `weight` with the delays drawn by `seed`); with `stimulus`, the task's stimulus
/// at every epoch's middle (condition (d)). Every weight of the arena at its end is its value at
/// the start.
fn run_050(
    image: &[u8],
    size: u32,
    weight: Option<i16>,
    background: bool,
    seed: u64,
    stimulus: bool,
    name: &str,
) -> Run050 {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let mut exec = marked_engine_drawn(image, SET_050, size, !background, weight, seed);
    let m = members(size);
    let before = weights_of(&exec);
    let task = stimulus.then(stimulus_task);
    let (run, epochs) = span_protocol_under(&mut exec, &m, &spans, !background, task.as_ref());
    let rows = stretch_rows(&run.windows, size);
    eprintln!("DUMP {name} stretches = {rows:?}");
    eprintln!("DUMP {name} kicks = {:?}", run.kicks);
    eprintln!(
        "DUMP {name} hashes = {:#018x} {:#018x}",
        windows_hash(&run.windows),
        epochs_hash(&epochs)
    );
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    Run050 { run, rows, epochs }
}

/// A size's background and control read by the rules: the background, the kick's reading, the
/// control's holding and its bursts.
type BackgroundControl = (Background, (u64, u64, u64, u32), Holding, BurstRead049);

/// A cell read by the rules: its holding and its bursts.
type CellRead = (Holding, BurstRead049);

/// Reads a cell against `bg` and dumps the reading.
fn cell_read(run: &Run050, size: u32, bg: Background, name: &str) -> CellRead {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let read = holding(&run.rows, &spans, bg);
    let bursts = bursts_049(&run.run.windows, &spans, size, bg, SETS[SET_050]);
    eprintln!(
        "DUMP {name} read = {read:?} usable {} failed {:?} kick {:?} bursts = {bursts:?}",
        read.usable(),
        failed(&read),
        kick_reading_049(&run.run.kicks, size)
    );
    (read, bursts)
}

/// The backgrounds and the controls of brief 050 at `sizes` (by index into the grid) on
/// `image`, with the task's stimulus when `stimulus`: for each size the background and the
/// control, each dumped with its readings; then each control's lead-in and first unkicked span
/// shown to be its background's, and the kick read on the engine by ADR-0112's measure, which
/// fires every member once at every size or no cell of the condition runs.
fn backgrounds_controls_050(
    image: &[u8],
    sizes: &[usize],
    stimulus: bool,
    name: &str,
) -> Vec<(Run050, Run050, BackgroundControl)> {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let mut out = Vec::new();
    for &s in sizes {
        let size = SIZES_050[s];
        let bg = run_050(
            image,
            size,
            None,
            true,
            DELAY_SEED,
            stimulus,
            &format!("{name} background {size}"),
        );
        let ctl = run_050(
            image,
            size,
            None,
            false,
            DELAY_SEED,
            stimulus,
            &format!("{name} control {size}"),
        );
        let background = background_049(&bg.rows, &spans);
        let (control, bursts) =
            cell_read(&ctl, size, background, &format!("{name} control {size}"));
        let kick = kick_reading_049(&ctl.run.kicks, size);
        eprintln!(
            "DUMP {name} {size}: background {background:?} kick {kick:?} once {}",
            kicked_once_049(&ctl.run.kicks, size)
        );
        if stimulus {
            for (k, run) in [&bg, &ctl].into_iter().enumerate() {
                eprintln!(
                    "DUMP {name} {size} task[{k}] = {:?}",
                    task_read(&run.epochs, &spans)
                );
            }
            eprintln!(
                "DUMP {name} {size} sight = {:?} nine = {:?}",
                sight_050(&bg.epochs, false),
                sight_050(&bg.epochs, true)
            );
        }
        out.push((bg, ctl, (background, kick, control, bursts)));
    }
    for (&s, (bg, ctl, _)) in sizes.iter().zip(&out) {
        let size = SIZES_050[s];
        assert_eq!(
            ctl.rows[..first],
            bg.rows[..first],
            "{name} {size}: the growth changes nothing unkicked"
        );
        assert!(
            kicked_once_049(&ctl.run.kicks, size),
            "{name} {size}: the kick fires every member once, or it is derived again"
        );
    }
    out
}

// ------------------------------------------------ the gate (brief 050)

/// The gate's assembly on the new geometry, by index into the grid: the largest, 96 units, at the
/// heaviest weight, 0.375, marked under set (ii).
const GATE_SIZE_050: usize = 3;

const GATE_WEIGHT_050: usize = 4;

#[test]
fn the_region_before_the_readout_the_geometry_the_conditions_the_rules_and_a_marked_assembly_on_the_new_geometry()
 {
    // The geometry beside the task's, and the task's unchanged.
    let sets = context_geometry(1024, ROTATION_1024);
    let task_sets = geometry(1024, ROTATION_1024);
    assert_eq!(
        task_sets.map(|s| s.mask),
        [1 << A_OFFSET, 1 << B_OFFSET, R0_MASK, R1_MASK],
        "the task's geometry unchanged"
    );
    assert_eq!(sets[..2], task_sets[..2], "the same stimuli");
    for s in &sets {
        assert!(s.is_well_formed());
        assert_eq!((s.first, s.period, s.count), (ROTATION_1024, PERIOD, 51));
    }
    assert_eq!(sets.map(|s| s.len()), [51, 51, 408, 408]);
    for (i, a) in sets.iter().enumerate() {
        for b in sets.iter().skip(i.saturating_add(1)) {
            assert!(!a.overlaps(b), "no two sets share a unit");
        }
    }
    let places = context_places(1024, ROTATION_1024, PLACES);
    assert_eq!(places.len(), 102, "the places given up");
    for s in &sets {
        assert!(!places.overlaps(s));
    }
    let covered = (0..1024u32)
        .filter(|&u| places.contains(u) || sets.iter().any(|s| s.contains(u)))
        .count();
    assert_eq!(
        covered, 1020,
        "every unit of the whole periods in one set or given up"
    );
    let mut on_it = stimulus_task();
    on_it.readout = Readout::new([sets[2], sets[3]]);
    on_it
        .check(&frozen_from(&prior_image(), 1024))
        .expect("the task fits the new geometry");
    // Every member of every size in no set of it, the sizes nested.
    for (k, &size) in SIZES_050.iter().enumerate() {
        let m = members(size);
        assert_eq!(m.len(), size as usize);
        assert_eq!(fan(size), FAN_MAX, "32 synapses a member at {size}");
        for &u in &m {
            assert!(
                places.contains(u),
                "member {u} of {size} at a place given up"
            );
            for s in &sets {
                assert!(!s.contains(u), "member {u} of {size} in no set");
            }
        }
        if let Some(&next) = SIZES_050.get(k.saturating_add(1)) {
            assert_eq!(members(next)[..m.len()], m[..], "the sizes nest");
        }
    }
    // The grid, the subset and ADR-0115's two cells in it.
    assert!(
        WEIGHTS_050
            .windows(2)
            .all(|w| w[0].checked_add(0x800) == Some(w[1]))
    );
    assert!(
        SIZES_050
            .windows(2)
            .all(|s| s[0].checked_add(16) == Some(s[1]))
    );
    assert_eq!(SIZES[2], 64);
    for (&(s, w), &k) in ADR_0115_CELLS.iter().zip(&ADR_0115_WEIGHTS) {
        assert_eq!((SIZES_050[s], WEIGHTS_050[w]), (SIZES[2], WEIGHTS_049[k]));
    }
    assert_eq!(SUBSET_SIZES.map(|s| SIZES_050[s]), [64, 96]);
    assert_eq!(
        SUBSET_WEIGHTS.map(|w| WEIGHTS_050[w]),
        [0x1800, 0x2000, 0x2800]
    );
    assert_eq!(
        (SETS[SET_050].u, RELEASE_STRETCHES_049[SET_050]),
        (26, 6),
        "set (ii), its release of six stretches"
    );
    assert_eq!(CONDITIONS.len(), CONDITION_NAMES.len());
    // The arithmetic, dumped, then held: the delivery at the grid's weights under set (ii),
    // ADR-0115's where the weights are its; the stimulus's reach onto the members and the places
    // given up, on the prior whose synapses the settled image keeps; the far band inside an
    // epoch's second half; and condition (b)'s delays.
    let delivered_read = delivered_at(SETS[SET_050], &WEIGHTS_050);
    eprintln!("DUMP DELIVERED_050 = {delivered_read:?}");
    let reach = stimulus_reach(&frozen_from(&prior_image(), 1024));
    eprintln!("DUMP STIMULUS_REACH_050 = {reach:?}");
    let seeded = SIZES_050.map(|size| seeded_wiring(size, WEIGHTS_050[2]));
    eprintln!("DUMP SEEDED_WIRING_050 = {seeded:?}");
    assert_eq!(delivered_read.as_slice(), DELIVERED_050.as_slice());
    for (&(_, w), &k) in ADR_0115_CELLS.iter().zip(&ADR_0115_WEIGHTS) {
        assert_eq!(delivered_read[w], DELIVERED_049[SET_050][k]);
    }
    assert_eq!(reach, STIMULUS_REACH_050);
    assert_eq!(seeded, SEEDED_WIRING_050);
    for (&size, &(differ, all)) in SIZES_050.iter().zip(&seeded) {
        assert_eq!(all, size.saturating_mul(FAN_MAX));
        assert!(differ > 0 && differ < all, "another draw, the same band");
    }
    let far = u32::from(prior(1024).far_delay_max);
    assert!(STIMULUS_AT.saturating_add(far) < EPOCH_TICKS && far < STIMULUS_AT);
    // Condition (c)'s patches, and condition (d)'s presentation held to the task's.
    the_patches_on_the_prior();
    let presented = presented_as_the_task_presents();
    eprintln!("DUMP PRESENTED_050 = {presented:?}");
    assert_eq!(presented, PRESENTED_050);
    // The rules at their edges, over tables written by hand.
    rules_050_at_their_edges();
    // A marked assembly on the new geometry kicked for a few hundred ticks.
    let kicked = marked_assembly_kicked(
        SET_050,
        SIZES_050[GATE_SIZE_050],
        WEIGHTS_050[GATE_WEIGHT_050],
    );
    eprintln!("DUMP GATE_KICKED_050 = {kicked:?}");
    for &(_, u) in &kicked {
        assert!(sets.iter().all(|s| !s.contains(u)), "a member in no set");
    }
    assert_eq!(kicked.as_slice(), GATE_KICKED_050);
    // The rules over the pinned tables.
    over_the_050_tables();
}

/// Condition (c)'s patches on the instrument's network: H-20's two flags written and read back,
/// and nothing of the modulator record but them and the value; frozen again, the image before
/// them bit for bit; and a reward delivered and written into an image, which carries its signal
/// until `frozen_again` puts it at rest with the two flags unset.
fn the_patches_on_the_prior() {
    let image = prior_image();
    let h20 = h20_image(&image);
    let exec = Image::decode::<2048>(&h20, config(1024, 2, 0)).expect("a well-formed record");
    assert_eq!(
        (
            exec.modulation_baseline_q16(),
            exec.inhibitory_baseline_q16(),
            exec.signed_gate()
        ),
        (0, Some(BASELINE_Q16), true)
    );
    let differing = image.iter().zip(&h20).filter(|(a, b)| a != b).count();
    assert!(
        differing > 0 && differing <= 11,
        "the flags, the value's one byte and the section's CRC: {differing} bytes"
    );
    assert_eq!(frozen_again(&h20), image, "frozen again, the image before");
    let mut rewarded =
        Image::decode::<2048>(&h20, config(1024, 2, 0)).expect("a well-formed record");
    rewarded.reward(REWARD_Q16);
    assert_eq!(quiet(&mut rewarded), 0, "at rest");
    let carried = Image::encode(&rewarded).expect("quiescent");
    let read = Image::decode::<2048>(&carried, config(1024, 2, 0)).expect("a well-formed record");
    assert_eq!(
        (read.modulator().dopamine_rpe, read.signed_gate()),
        (REWARD_Q16, true),
        "an image carries the signal the last reward left"
    );
    let frozen = frozen_from(&frozen_again(&carried), 1024);
    assert_eq!(
        (
            frozen.modulator().dopamine_rpe,
            frozen.inhibitory_baseline_q16(),
            frozen.signed_gate()
        ),
        (0, None, false),
        "frozen again: the signal at rest, both unset"
    );
}

/// Condition (d)'s presentation held to the task's own on the instrument's network at rest: one
/// epoch of `span_protocol_under` with no member, the stimulus at its middle; beside it, the drive
/// alone to the same tick and then one `Task::trial` of the task whose stimulus it presents, the
/// reward withheld. Over the epoch the two trains are one train bit for bit, the stimulus is the
/// trial's, and the epoch's counts after the stimulus, each eight-place readout with the place it
/// gave up, are the trial's nine-place counts. Returns the epoch's reading.
fn presented_as_the_task_presents() -> TaskEpoch {
    let image = prior_image();
    let task = stimulus_task();
    let mut ours = frozen_from(&image, 1024);
    let start = ours.ticks() as u32;
    let (_, epochs) =
        span_protocol_under(&mut ours, &[], &[(Span::LeadIn, 0, 1)], false, Some(&task));
    let mut theirs = frozen_from(&image, 1024);
    let drive = drive(1024);
    let inject = theirs.injector();
    for _ in 0..STIMULUS_AT {
        drive.step(&inject, theirs.ticks()).expect("the drive runs");
        theirs.tick();
    }
    let mut trial = task;
    let outcome = trial.trial(&mut theirs, 0).expect("a trial runs");
    assert_eq!(
        (ours.train_overwritten(), theirs.train_overwritten()),
        (0, 0)
    );
    let theirs_epoch: Vec<(u32, u32)> = theirs
        .train()
        .iter()
        .copied()
        .filter(|&(t, _)| t.wrapping_sub(start) < EPOCH_TICKS)
        .collect();
    assert!(!theirs_epoch.is_empty(), "the network fired");
    assert_eq!(ours.train(), theirs_epoch.as_slice(), "one train");
    let (stimulus, counts) = epochs[0];
    assert_eq!(stimulus, outcome.stimulus, "the trial's stimulus");
    assert_eq!(
        [
            counts[1][0].saturating_add(counts[1][2]),
            counts[1][1].saturating_add(counts[1][3])
        ],
        outcome.counts,
        "the trial's counts, each readout with its place"
    );
    epochs[0]
}

/// The rules of brief 050 at their edges, over tables written by hand: robust, the neighbours,
/// the second round's cell and its ties, what failed; the windows and kinds of (d)'s reading, its
/// sums and ADR-0065's measure over it.
fn rules_050_at_their_edges() {
    let usable = Holding {
        first_half: 8,
        held: 8,
        ignited: 0,
        let_go: 8,
        spills: Some(false),
    };
    let never = Holding { held: 6, ..usable };
    let runs = Holding {
        ignited: 2,
        ..usable
    };
    assert!(usable.usable() && !never.usable() && !runs.usable());
    let all: [Subset050; 3] = [[[usable; 3]; 2]; 3];
    let mut core: Grid050 = [[never; 5]; 4];
    // Robust: usable in the core grid and under every condition; a cell outside the subset never.
    assert!(robust_cells(&core, &all).is_empty());
    assert_eq!(round_two_cell(&core, &[]), None);
    core[1][2] = usable;
    core[0][0] = usable;
    assert_eq!(robust_cells(&core, &all), [(1, 2)]);
    for (c, name) in CONDITION_NAMES.iter().enumerate() {
        let mut under = all;
        under[c][0][1] = runs;
        assert!(robust_cells(&core, &under).is_empty(), "{name} fails it");
    }
    // The neighbours: one weight step and one size step either way, a diagonal not.
    let mut core: Grid050 = [[never; 5]; 4];
    core[2][3] = usable;
    assert_eq!(
        usable_neighbours(&core, (1, 2)),
        0,
        "a diagonal is not a neighbour"
    );
    for cell in [(0, 2), (2, 2), (1, 1), (1, 3)] {
        core[cell.0][cell.1] = usable;
    }
    assert_eq!(usable_neighbours(&core, (1, 2)), 4);
    assert_eq!(usable_neighbours(&core, (0, 0)), 0);
    core[1][0] = usable;
    core[0][1] = usable;
    assert_eq!(usable_neighbours(&core, (0, 0)), 2, "a corner has two");
    assert_eq!(usable_neighbours(&core, (3, 4)), 0);
    // The second round's cell: the most usable neighbours, then the lighter weight, then the
    // smaller size.
    let mut core: Grid050 = [[never; 5]; 4];
    for cell in [(1, 1), (1, 2), (1, 3), (3, 3), (2, 3)] {
        core[cell.0][cell.1] = usable;
    }
    assert_eq!(
        [(1, 1), (1, 2), (3, 3)].map(|c| usable_neighbours(&core, c)),
        [1, 2, 1]
    );
    assert_eq!(
        round_two_cell(&core, &[(3, 3), (1, 1), (1, 2)]),
        Some((1, 2))
    );
    assert_eq!(
        round_two_cell(&core, &[(3, 3), (1, 1)]),
        Some((1, 1)),
        "a tie to the lighter weight"
    );
    let mut core: Grid050 = [[never; 5]; 4];
    for cell in [(1, 2), (3, 2), (0, 2), (2, 1)] {
        core[cell.0][cell.1] = usable;
    }
    assert_eq!(
        [(1, 2), (3, 2)].map(|c| usable_neighbours(&core, c)),
        [1, 0]
    );
    core[3][1] = usable;
    assert_eq!(usable_neighbours(&core, (3, 2)), 1);
    assert_eq!(
        round_two_cell(&core, &[(3, 2), (1, 2)]),
        Some((1, 2)),
        "at one weight a tie to the smaller size"
    );
    // What failed, per condition.
    let mut under = all;
    under[1][1][2] = runs;
    under[2][0][0] = never;
    let failures = failures_050(&[[never; 5]; 4], &under);
    assert_eq!(
        failures[1][2],
        [
            [true, false, false],
            [false; 3],
            [false, true, false],
            [false; 3]
        ]
    );
    assert_eq!(
        failures[0][0],
        [
            [true, false, false],
            [false; 3],
            [false; 3],
            [true, false, false]
        ]
    );
    // (d)'s windows at their edges, and the kinds of span.
    for (at, side) in [
        (BEFORE_FROM - 1, None),
        (BEFORE_FROM, Some(0)),
        (STIMULUS_AT - 1, Some(0)),
        (STIMULUS_AT, None),
        (AFTER_FROM - 1, None),
        (AFTER_FROM, Some(1)),
        (AFTER_END - 1, Some(1)),
        (AFTER_END, None),
    ] {
        assert_eq!(task_side(at), side, "tick {at}");
    }
    assert_eq!(
        (
            task_kind(Span::LeadIn, 0),
            task_kind(Span::Unkicked, 15),
            task_kind(Span::Hold, 7),
            task_kind(Span::Hold, 8),
            task_kind(Span::Release, 0),
            task_kind(Span::Tail, 3)
        ),
        (None, Some(0), Some(1), Some(2), Some(3), Some(4))
    );
    // (d)'s sums by kind and stimulus, the lead-in left out.
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let epochs: Vec<TaskEpoch> = (0..RUN_EPOCHS_049[SET_050])
        .map(|e| ((e & 1) as u8, [[1, 2, 3, 4], [5, 6, 7, 8]]))
        .collect();
    let read = task_read(&epochs, &spans);
    assert_eq!(
        read[0],
        [(64, [320, 384]), (64, [320, 384])],
        "unkicked: 128 trials"
    );
    assert_eq!(read[1][0], (32, [160, 192]), "the hold's first half");
    assert_eq!(read[3][1], (48, [240, 288]), "the release, six stretches");
    assert_eq!(read[4][0], (16, [80, 96]), "the tail");
    // ADR-0065's measure over the blocks after the lead-in: 56 of 64 seen and more after than
    // before at each readout, at its edges; the nine-place readouts add each place's spikes.
    let lead = LEAD_IN_EPOCHS_049 as usize;
    let mut epochs: Vec<TaskEpoch> = vec![(0, [[0; 4]; 2]); lead.saturating_add(BLOCK)];
    for (k, e) in epochs.iter_mut().skip(lead).enumerate() {
        e.1 = if k < 56 {
            [[1, 1, 0, 0], [2, 1, 0, 0]]
        } else {
            [[1, 1, 0, 0], [1, 1, 0, 0]]
        };
    }
    let sight = sight_050(&epochs, false);
    assert_eq!(sight, [([120, 64], [64, 64], 56)]);
    assert!(!resolves(&sight[0]), "readout 1 no more after than before");
    epochs[lead].1[1][1] = 2;
    let sight = sight_050(&epochs, false);
    assert_eq!(sight, [([120, 65], [64, 64], 56)]);
    assert!(resolves(&sight[0]));
    epochs[lead].1[1] = [1, 1, 0, 0];
    epochs[lead.saturating_add(60)].1 = [[2, 1, 0, 0], [0, 3, 0, 0]];
    let sight = sight_050(&epochs, false);
    assert_eq!(sight, [([118, 66], [65, 64], 55)]);
    assert!(
        !resolves(&sight[0]),
        "more after than before at each readout, but 55 of 64 seen"
    );
    epochs[lead].1[1] = [1, 1, 5, 1];
    assert_eq!(
        sight_050(&epochs, false),
        sight,
        "the eight-place readouts count no place"
    );
    let nine = sight_050(&epochs, true);
    assert_eq!(nine, [([123, 67], [65, 64], 56)]);
    assert!(
        resolves(&nine[0]),
        "the nine-place readouts add each place's spikes"
    );
    assert!(
        sight_050(&epochs[..lead.saturating_add(BLOCK - 1)], false).is_empty(),
        "a block is whole"
    );
}

/// The rules over brief 050's pinned tables: each run's stretches as the rules read them against
/// its own background; the kick firing every member once at every size under each condition;
/// each control's lead-in and first unkicked span its background's; and the reading ADR-0117
/// gives — the core grid's usable region, one weight at three sizes; no robust cell and so no
/// cell for the second round; and what failed in each cell of the subset under each condition.
fn over_the_050_tables() {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let n = stretches_of(&spans).len();
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let pair_read = |runs: &[Pinned050; 2], read: &BackgroundControl, size: u32| {
        let [bg, ctl] = runs;
        assert_eq!((bg.rows.len(), ctl.rows.len()), (n, n));
        let background = background_049(bg.rows, &spans);
        assert_eq!(background, read.0);
        assert_eq!(kick_reading_049(ctl.kicks, size), read.1);
        assert!(
            kicked_once_049(ctl.kicks, size),
            "the kick fires every member of {size} once"
        );
        assert_eq!(holding(ctl.rows, &spans, background), read.2);
        assert_eq!(
            ctl.rows[..first],
            bg.rows[..first],
            "the growth changes nothing unkicked"
        );
        background
    };
    let cells = |pins: &[Pinned050], reads: &[Holding], bg: Background, name: &str| {
        for (w, (pin, read)) in pins.iter().zip(reads).enumerate() {
            assert_eq!(pin.rows.len(), n);
            assert_eq!(pin.kicks.len(), ROUNDS, "a kick a round");
            assert_eq!(holding(pin.rows, &spans, bg), *read, "{name} at weight {w}");
        }
    };
    let mut backgrounds = Vec::new();
    for (s, &size) in SIZES_050.iter().enumerate() {
        let bg = pair_read(&CORE_RUNS_050[s], &CORE_READ_050[s], size);
        cells(
            &GRID_RUNS_050[s],
            &GRID_050[s],
            bg,
            &format!("the core grid at {size}"),
        );
        backgrounds.push(bg);
    }
    for (c, name) in CONDITION_NAMES.iter().enumerate() {
        for (i, &s) in SUBSET_SIZES.iter().enumerate() {
            let size = SIZES_050[s];
            let bg = match CONDITIONS[c] {
                Condition::Seed => backgrounds[s],
                Condition::Drained => pair_read(&DRAINED_RUNS_050[i], &DRAINED_READ_050[i], size),
                Condition::Stimulus => {
                    pair_read(&STIMULUS_RUNS_050[i], &STIMULUS_READ_050[i], size)
                }
            };
            cells(
                &CONDITION_RUNS_050[c][i],
                &CONDITION_GRID_050[c][i],
                bg,
                &format!("{name} at {size}"),
            );
        }
    }
    // The core grid's usable region: 0.25 at 64, 80 and 96 units.
    let usable: Vec<(usize, usize)> = (0..SIZES_050.len())
        .flat_map(|s| (0..WEIGHTS_050.len()).map(move |w| (s, w)))
        .filter(|&(s, w)| GRID_050[s][w].usable())
        .collect();
    assert_eq!(
        usable,
        [(1, 2), (2, 2), (3, 2)],
        "one weight at three sizes"
    );
    // No robust cell, so no cell for the second round; what failed, per condition.
    let robust = robust_cells(&GRID_050, &CONDITION_GRID_050);
    assert_eq!(robust.as_slice(), ROBUST_050);
    assert_eq!(round_two_cell(&GRID_050, &robust), ROUND_TWO_050);
    assert_eq!(failures_050(&GRID_050, &CONDITION_GRID_050), FAILURES_050);
}

// ------------------------------------------------ the runs, weekly (brief 050)

/// The kick, the backgrounds and the controls of the core grid (brief 050): on ADR-0077's
/// settled image frozen, for each size the background and the control, the kick read on the
/// engine before any cell; each held to its pins, 64 units' to ADR-0115's.
fn core_backgrounds_controls() {
    let name = "brief 050's core backgrounds and controls";
    let image = settled(name);
    let sizes = [0, 1, 2, 3];
    let read = backgrounds_controls_050(&image, &sizes, false, name);
    for (&s, (bg, ctl, reading)) in sizes.iter().zip(&read) {
        let size = SIZES_050[s];
        bg.held_to(
            &CORE_RUNS_050[s][0],
            true,
            &format!("{name}: the background {size}"),
        );
        ctl.held_to(
            &CORE_RUNS_050[s][1],
            false,
            &format!("{name}: the control {size}"),
        );
        assert_eq!(*reading, CORE_READ_050[s], "{name}: {size} read");
    }
}

/// The five weights of one size of the core grid (brief 050): on the settled image, the members
/// marked, grown and wired at each weight, the protocol, each run read against its size's
/// background pinned first. At 64 units ADR-0115's two cells run first and are held to
/// ADR-0115's pins before any other cell runs.
fn five_weights_050(s: usize) {
    let size = SIZES_050[s];
    let name = format!("brief 050's assembly of {size}");
    let image = settled(&name);
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let bg = background_049(CORE_RUNS_050[s][0].rows, &spans);
    assert_eq!(
        bg, CORE_READ_050[s].0,
        "{name}: the background, pinned first"
    );
    let cell = |w: usize| {
        let label = format!("{name} at {:#x}", WEIGHTS_050[w]);
        let run = run_050(
            &image,
            size,
            Some(WEIGHTS_050[w]),
            false,
            DELAY_SEED,
            false,
            &label,
        );
        let read = cell_read(&run, size, bg, &label);
        (run, read)
    };
    let reproduced: Vec<usize> = ADR_0115_CELLS
        .iter()
        .filter(|c| c.0 == s)
        .map(|c| c.1)
        .collect();
    let mut runs: Vec<Option<(Run050, CellRead)>> = (0..WEIGHTS_050.len()).map(|_| None).collect();
    for &w in &reproduced {
        let (run, read) = cell(w);
        run.held_to(
            &GRID_RUNS_050[s][w],
            false,
            &format!("{name}: ADR-0115's cell at {w}"),
        );
        assert_eq!(
            read,
            (GRID_050[s][w], GRID_BURSTS_050[s][w]),
            "{name}: ADR-0115's reading"
        );
        runs[w] = Some((run, read));
    }
    for (w, slot) in runs.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(cell(w));
        }
    }
    for (w, slot) in runs.iter().enumerate() {
        let (run, read) = slot.as_ref().expect("every weight run");
        run.held_to(&GRID_RUNS_050[s][w], false, &format!("{name}: weight {w}"));
        assert_eq!(
            *read,
            (GRID_050[s][w], GRID_BURSTS_050[s][w]),
            "{name}: weight {w}"
        );
    }
}

/// The subset's six cells under condition (b) or (d) (brief 050), on the settled image, each
/// size's read against the condition's background pinned first — for (b) the core grid's, since
/// an unwired run draws no delay, and its kick the core control's — every run dumped before any
/// is held.
fn subset_under(c: usize) {
    let condition = CONDITIONS[c];
    let name = format!("brief 050's subset under {}", CONDITION_NAMES[c]);
    let image = settled(&name);
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let mut out = Vec::new();
    for (i, &s) in SUBSET_SIZES.iter().enumerate() {
        let size = SIZES_050[s];
        let (runs, read, seed, stimulus) = match condition {
            Condition::Seed => (&CORE_RUNS_050[s], &CORE_READ_050[s], DELAY_SEED_050, false),
            Condition::Stimulus => (
                &STIMULUS_RUNS_050[i],
                &STIMULUS_READ_050[i],
                DELAY_SEED,
                true,
            ),
            Condition::Drained => panic!("condition (c) runs in one test with its image"),
        };
        let bg = background_049(runs[0].rows, &spans);
        assert_eq!(bg, read.0, "{name} {size}: the background, pinned first");
        assert!(
            kicked_once_049(runs[1].kicks, size),
            "{name} {size}: the kick read on the engine first"
        );
        for (j, &w) in SUBSET_WEIGHTS.iter().enumerate() {
            let label = format!("{name} {size} at {:#x}", WEIGHTS_050[w]);
            let run = run_050(
                &image,
                size,
                Some(WEIGHTS_050[w]),
                false,
                seed,
                stimulus,
                &label,
            );
            let cell = cell_read(&run, size, bg, &label);
            if stimulus {
                eprintln!("DUMP {label} task = {:?}", task_read(&run.epochs, &spans));
            }
            out.push((i, j, run, cell));
        }
    }
    for (i, j, run, cell) in &out {
        run.held_to(
            &CONDITION_RUNS_050[c][*i][*j],
            false,
            &format!("{name}: {i} {j}"),
        );
        assert_eq!(
            *cell,
            (
                CONDITION_GRID_050[c][*i][*j],
                CONDITION_BURSTS_050[c][*i][*j]
            ),
            "{name}: {i} {j}"
        );
        if CONDITIONS[c] == Condition::Stimulus {
            assert_eq!(
                task_read(&run.epochs, &spans),
                STIMULUS_CELL_TASK_050[*i][*j]
            );
        }
    }
}

/// Condition (c) whole (brief 050): the settled image, H-20's image and its arm from the
/// assignment, held to ADR-0110's; the drained image frozen and read; the backgrounds and the
/// controls at the subset's sizes, the kick read on the engine before any cell; the subset's six
/// cells; every run dumped before any is held.
fn drained_subset() {
    let c = 1;
    let name = "brief 050's subset under (c)";
    let settled_image = settled(name);
    let (image, drained) = drained_image(name, &settled_image);
    let pairs = backgrounds_controls_050(&image, &SUBSET_SIZES, false, name);
    let mut cells = Vec::new();
    for (i, &s) in SUBSET_SIZES.iter().enumerate() {
        let size = SIZES_050[s];
        let bg = pairs[i].2.0;
        for (j, &w) in SUBSET_WEIGHTS.iter().enumerate() {
            let label = format!("{name} {size} at {:#x}", WEIGHTS_050[w]);
            let run = run_050(
                &image,
                size,
                Some(WEIGHTS_050[w]),
                false,
                DELAY_SEED,
                false,
                &label,
            );
            let cell = cell_read(&run, size, bg, &label);
            cells.push((i, j, run, cell));
        }
    }
    assert_eq!(drained, DRAINED_050, "{name}: the drained image");
    for (i, (bg, ctl, reading)) in pairs.iter().enumerate() {
        bg.held_to(
            &DRAINED_RUNS_050[i][0],
            true,
            &format!("{name}: the background {i}"),
        );
        ctl.held_to(
            &DRAINED_RUNS_050[i][1],
            false,
            &format!("{name}: the control {i}"),
        );
        assert_eq!(*reading, DRAINED_READ_050[i], "{name}: {i} read");
    }
    for (i, j, run, cell) in &cells {
        run.held_to(
            &CONDITION_RUNS_050[c][*i][*j],
            false,
            &format!("{name}: {i} {j}"),
        );
        assert_eq!(
            *cell,
            (
                CONDITION_GRID_050[c][*i][*j],
                CONDITION_BURSTS_050[c][*i][*j]
            ),
            "{name}: {i} {j}"
        );
    }
}

/// Condition (d)'s backgrounds and controls (brief 050): on the settled image with the task's
/// stimulus at every epoch's middle, at the subset's sizes; the kick read on the engine before
/// any cell; the readouts' reading and ADR-0065's measure on each background.
fn stimulus_backgrounds_controls() {
    let name = "brief 050's backgrounds and controls under (d)";
    let image = settled(name);
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let pairs = backgrounds_controls_050(&image, &SUBSET_SIZES, true, name);
    for (i, (bg, ctl, reading)) in pairs.iter().enumerate() {
        bg.held_to(
            &STIMULUS_RUNS_050[i][0],
            true,
            &format!("{name}: the background {i}"),
        );
        ctl.held_to(
            &STIMULUS_RUNS_050[i][1],
            false,
            &format!("{name}: the control {i}"),
        );
        assert_eq!(*reading, STIMULUS_READ_050[i], "{name}: {i} read");
        assert_eq!(
            [
                task_read(&bg.epochs, &spans),
                task_read(&ctl.epochs, &spans)
            ],
            STIMULUS_TASK_050[i]
        );
        assert_eq!(
            [sight_050(&bg.epochs, false), sight_050(&bg.epochs, true)],
            STIMULUS_SIGHT_050[i].map(|s| s.to_vec())
        );
    }
}

#[test]
#[ignore]
fn the_kick_the_backgrounds_and_the_controls_of_the_core_grid_under_set_ii_paused() {
    core_backgrounds_controls();
}

#[test]
#[ignore]
fn a_marked_assembly_of_48_units_at_five_weights_under_set_ii_paused() {
    five_weights_050(0);
}

#[test]
#[ignore]
fn a_marked_assembly_of_64_units_at_five_weights_under_set_ii_paused() {
    five_weights_050(1);
}

#[test]
#[ignore]
fn a_marked_assembly_of_80_units_at_five_weights_under_set_ii_paused() {
    five_weights_050(2);
}

#[test]
#[ignore]
fn a_marked_assembly_of_96_units_at_five_weights_under_set_ii_paused() {
    five_weights_050(3);
}

#[test]
#[ignore]
fn the_subset_with_the_delays_of_seed_49_paused() {
    subset_under(0);
}

#[test]
#[ignore]
fn the_subset_on_the_image_h_20_leaves_paused() {
    drained_subset();
}

#[test]
#[ignore]
fn the_kick_the_backgrounds_and_the_controls_under_the_tasks_stimulus_paused() {
    stimulus_backgrounds_controls();
}

#[test]
#[ignore]
fn the_subset_under_the_tasks_stimulus_paused() {
    subset_under(2);
}

// ------------------------------------------------ brief 050's arithmetic, pinned before any run

/// Condition (b)'s delays per size at 0.25: the synapses whose delay differs from seed 48's, of
/// the assembly's.
const SEEDED_WIRING_050: [(u32, u32); 4] = [(1532, 1536), (2042, 2048), (2553, 2560), (3062, 3072)];

/// Condition (d)'s presentation on the instrument's network at rest, the epoch's reading.
const PRESENTED_050: TaskEpoch = (0, [[5, 2, 1, 0], [18, 26, 2, 3]]);

// ------------------------------------------------ brief 050's readings, pinned from the runs

/// The robust cells, `(size, weight)` by index into the grid: none.
const ROBUST_050: &[(usize, usize)] = &[];

/// The cell for the second round: none, since no cell is robust.
const ROUND_TWO_050: Option<(usize, usize)> = None;

// ================================================ brief 051 (ADR-0120): an inhibition of the context's own

// ------------------------------------------------ the inhibition, the grid and the conditions (brief 051), before any run

/// The assembly every run of brief 051 wires: ADR-0117's 64 marked members under set (ii), at
/// ADR-0112's placement on ADR-0116's geometry; and its size's index in ADR-0117's grid.
const SIZE_051: u32 = 64;

const SIZE_051_IN_050: usize = 1;

/// ADR-0117's cell in its grid, by index: 64 units at a quarter of the range, with no added
/// inhibition; brief 051 reproduces it under each condition before any other cell runs.
const CELL_050: (usize, usize) = (SIZE_051_IN_050, 2);

/// The sources a member receives the added inhibition from (ADR-0119).
const SOURCES_PER_MEMBER: usize = 16;

/// The seed of the sources' draw and of the added synapses' delays: the brief's number, which no
/// earlier draw of this file uses (the assembly's delays are 48's and 49's).
const INHIBITION_SEED: u64 = 51;

/// The recurrent weights, Q1.15: a quarter, three eighths, a half and three quarters of the range.
/// ADR-0117's cell is at the first.
const RECURRENT_051: [i16; 4] = [0x2000, 0x3000, 0x4000, 0x6000];

/// The added inhibitory weights, Q1.15, negative by Dale's principle: a quarter, a half and the
/// whole of the range, the whole the prior's inhibitory rail, `-i16::MAX` (the prior saturates an
/// inhibitory magnitude at the width, ADR-0044), one LSB above −1.0.
const INHIBITORY_051: [i16; 3] = [-0x2000, -0x4000, -i16::MAX];

/// The recurrent weights the cells of one weekly test of a condition take, by index: the two
/// lighter, then the two heavier.
const RECURRENT_HALVES: [[usize; 2]; 2] = [[0, 1], [2, 3]];

/// The conditions of brief 051 in ADR-0117's order: the core, then (b), (c) and (d).
const CONDITIONS_051: [Option<Condition>; 4] = [
    None,
    Some(Condition::Seed),
    Some(Condition::Drained),
    Some(Condition::Stimulus),
];

const CONDITION_NAMES_051: [&str; 4] = ["the core", "(b)", "(c)", "(d)"];

/// The backgrounds a condition's cells are read against, by index into the three conditions
/// that run their own — the core, (c) and (d): (b)'s are the core's, since an unwired assembly
/// draws no delay and the inhibition's draw is `INHIBITION_SEED`'s under every condition.
const fn backgrounds_of(c: usize) -> usize {
    match c {
        0 | 1 => 0,
        2 => 1,
        _ => 2,
    }
}

/// The prior's inhibitory units at 1 024 units, ascending: every fifth from the fifth, places 4,
/// 9, 14 and 19 of each period of twenty, 204 of them.
fn inhibitory_units() -> Vec<u32> {
    let p = prior(1024);
    (0..p.units).filter(|&u| p.is_inhibitory(u)).collect()
}

/// The sixteen sources of `member`: a partial Fisher–Yates shuffle of `inhibitory_units` in
/// ascending order, the swap of step `i` drawn by `mix64` of `INHIBITION_SEED`, the member and
/// `i`; distinct by construction, in the order drawn. A source within the prior's window of the
/// member is not excluded, and two members may share one (ADR-0120).
fn sources_of(member: u32) -> [u32; SOURCES_PER_MEMBER] {
    let mut pool = inhibitory_units();
    let n = pool.len();
    for i in 0..SOURCES_PER_MEMBER {
        let left = n.saturating_sub(i) as u64;
        let draw = mix64(INHIBITION_SEED ^ (u64::from(member) << 32 | i as u64));
        let j = i.saturating_add(draw.checked_rem(left).unwrap_or(0) as usize);
        pool.swap(i, j);
    }
    core::array::from_fn(|i| pool[i])
}

/// The added inhibition's plan: every inhibitory unit some member drew, ascending, with the
/// members that drew it in `members(SIZE_051)`'s order.
fn inhibition_plan() -> Vec<(u32, Vec<u32>)> {
    let m = members(SIZE_051);
    let drawn: Vec<[u32; SOURCES_PER_MEMBER]> = m.iter().map(|&u| sources_of(u)).collect();
    inhibitory_units()
        .into_iter()
        .filter_map(|s| {
            let targets: Vec<u32> = m
                .iter()
                .zip(&drawn)
                .filter(|(_, d)| d.contains(&s))
                .map(|(&u, _)| u)
                .collect();
            (!targets.is_empty()).then_some((s, targets))
        })
        .collect()
}

/// The sources: every inhibitory unit some member drew, ascending, the units whose spikes each
/// run of brief 051 counts.
fn inhibition_sources() -> Vec<u32> {
    inhibition_plan().into_iter().map(|(s, _)| s).collect()
}

/// The blocks the added inhibition appends: each source's synapses, four to a block.
fn inhibition_blocks() -> u64 {
    inhibition_plan().iter().fold(0u64, |n, (_, t)| {
        n.saturating_add(t.len().div_ceil(SYNAPSES_PER_BLOCK) as u64)
    })
}

/// Wires the added inhibition at `weight` into `exec`, whose arena ends with the assembly's
/// `extra_blocks(SIZE_051)` blocks and then `inhibition_blocks()` more: each source's synapses,
/// in the plan's order, fill its blocks four to a block, basal, each with its delay drawn from
/// the prior's local band by `INHIBITION_SEED` (`delay_drawn`); the source's first block chained
/// after the last block of its own chain, the rest after it in order. Every synapse leaves an
/// inhibitory unit of the prior at a weight at or below zero (Dale's principle, ADR-0049).
fn wire_inhibition(exec: &mut Engine, weight: i16) {
    assert!(weight <= 0, "Dale's principle: an inhibitory unit's weight");
    let base = blocks_for(&prior(1024)).saturating_add(extra_blocks(SIZE_051)) as u32;
    let plan = inhibition_plan();
    let (units, blocks) = exec.arenas_mut();
    assert_eq!(
        blocks.len() as u64,
        u64::from(base).saturating_add(inhibition_blocks()),
        "the arena holds the prior's blocks, the assembly's and the inhibition's"
    );
    let per = SYNAPSES_PER_BLOCK as u32;
    let mut first = base;
    for (source, targets) in &plan {
        let unit = &units[*source as usize];
        assert!(
            unit.flags & FLAG_INHIBITORY != 0,
            "source {source} is inhibitory"
        );
        let last = unit
            .chain(blocks)
            .last()
            .expect("a unit of the prior has a chain");
        assert!(blocks[last as usize].link(first), "the first block chained");
        for (d, &target) in targets.iter().enumerate() {
            let d = d as u32;
            let block = first.saturating_add(d.checked_div(per).unwrap_or(0));
            let slot = d.checked_rem(per).unwrap_or(0) as usize;
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
                    delay_drawn(INHIBITION_SEED, *source, target),
                    false
                ),
                "the synapse written"
            );
        }
        first = first.saturating_add(targets.len().div_ceil(SYNAPSES_PER_BLOCK) as u32);
    }
}

/// The class and the marks of the members of `size` under set `set`, asserted on `exec`.
fn marks_held(exec: &Engine, set: usize, size: u32) {
    assert_eq!(exec.stp_class(), Some(SETS[set]), "the image's class");
    let of = membership(&members(size));
    for unit in exec.units() {
        assert_eq!(
            unit.flags & FLAG_FACILITATING != 0,
            of[unit.id as usize],
            "unit {} marked as a member",
            unit.id
        );
    }
}

/// The engine of a run of brief 051 on `image`: the members of `SIZE_051` marked under set (ii),
/// the image grown by the assembly's blocks and the inhibition's and decoded; the added
/// inhibition wired at `inhibitory`, and the assembly at `recurrent` with the delays of `seed`
/// when given (a cell). A background and a control are grown alike and wire no assembly.
fn engine_051(image: &[u8], recurrent: Option<i16>, inhibitory: i16, seed: u64) -> Engine {
    let img = marked(image, SETS[SET_050], SIZE_051);
    let extra = extra_blocks(SIZE_051).saturating_add(inhibition_blocks());
    let mut exec = frozen_from(&grown(&img, extra), 1024);
    marks_held(&exec, SET_050, SIZE_051);
    wire_inhibition(&mut exec, inhibitory);
    if let Some(w) = recurrent {
        wire_among(&mut exec, SIZE_051, w, seed);
    }
    exec
}

// ------------------------------------------------ the rules (brief 051), before any run

/// A grid of brief 051 read by ADR-0115's rules, `[recurrent][inhibitory]` over `RECURRENT_051`
/// and `INHIBITORY_051`.
type Grid051 = [[Holding; 3]; 4];

/// Robust: the cells usable in the core grid and under each of (b), (c) and (d), `(recurrent,
/// inhibitory)` by index, in the grid's order. `grids` holds the four in `CONDITIONS_051`'s order.
fn robust_051(grids: &[Grid051; 4]) -> Vec<(usize, usize)> {
    (0..RECURRENT_051.len())
        .flat_map(|r| (0..INHIBITORY_051.len()).map(move |i| (r, i)))
        .filter(|&(r, i)| grids.iter().all(|g| g[r][i].usable()))
        .collect()
}

/// The cell for the second round: of the robust cells, the one with the most usable neighbours in
/// the core grid, one recurrent step and one inhibitory step either way (`usable_neighbours`);
/// ties to the lighter recurrent weight, then the lighter inhibition. None when none is robust.
fn round_two_051(core: &Grid051, robust: &[(usize, usize)]) -> Option<(usize, usize)> {
    let mut ordered = robust.to_vec();
    ordered.sort_unstable();
    let mut best: Option<((usize, usize), u32)> = None;
    for cell in ordered {
        let n = usable_neighbours(core, cell);
        if best.is_none_or(|(_, most)| n > most) {
            best = Some((cell, n));
        }
    }
    best.map(|(cell, _)| cell)
}

/// What failed in each cell, `[recurrent][inhibitory][condition]`, in `CONDITIONS_051`'s order,
/// each by ADR-0115's `failed`: the reading ADR-0120 gives where no cell is robust.
fn failures_051(grids: &[Grid051; 4]) -> [[[[bool; 3]; 4]; 3]; 4] {
    let mut out = [[[[false; 3]; 4]; 3]; 4];
    for (r, row) in out.iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            for (c, grid) in grids.iter().enumerate() {
                cell[c] = failed(&grid[r][i]);
            }
        }
    }
    out
}

// ------------------------------------------------ the arithmetic (brief 051), before any run

/// A rate as ADR-0117's backgrounds give it: spikes over unit-ticks, the units times the ticks.
type Rate = (u64, u64);

/// The interval a rate's unit fires at on average, in ticks.
fn interval_of((spikes, unit_ticks): Rate) -> u32 {
    unit_ticks.checked_div(spikes).unwrap_or(0) as u32
}

/// The basal standing, Q16.16, that `delivered` — the sum of what one spike of each of a set of
/// synapses delivers, after the gain — holds when every source fires at `rate`: the input per
/// tick times the basal leak's time constant, $2^9$ ticks.
fn standing((spikes, unit_ticks): Rate, delivered: i64) -> i64 {
    delivered
        .saturating_mul(1 << BASAL_LEAK_SHIFT)
        .saturating_mul(spikes as i64)
        .checked_div(unit_ticks as i64)
        .unwrap_or(0)
}

/// The members' rate and the rest's in ADR-0117's background at 64 units under the core, (c) and
/// (d): the backgrounds it pinned, 64 members and 960 other units over the ticks after the
/// lead-in.
fn rates_050() -> [(Rate, Rate); 3] {
    [
        CORE_READ_050[SIZE_051_IN_050].0,
        DRAINED_READ_050[0].0,
        STIMULUS_READ_050[0].0,
    ]
    .map(|bg| {
        let rest = 1024u64.saturating_sub(u64::from(SIZE_051));
        (
            (bg.members, bg.ticks.saturating_mul(u64::from(SIZE_051))),
            (bg.rest, bg.ticks.saturating_mul(rest)),
        )
    })
}

/// The lowest somatic potential a unit at the drive's mean standing falls to under the drive's
/// mean input when `basal` (after the gain) lands on tick one, within a pair window.
fn fall(basal: i32) -> i32 {
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
    s.soma.iter().copied().min().unwrap_or(DRIVE_STANDING.1)
}

/// What one spike of a source delivers to a member at each of `INHIBITORY_051`, at the pair of
/// ADR-0019's constants steady at the rest's interval in ADR-0117's core background, and the
/// trough of the soma it takes a member at the drive's mean standing to.
fn inhibition_delivered() -> Vec<(i32, i32)> {
    let pair = steady(interval_of(rates_050()[0].1));
    INHIBITORY_051
        .iter()
        .map(|&w| {
            let d = delivered(w, pair);
            (d, fall(d))
        })
        .collect()
}

/// The members' mean input from the assembly at each of `RECURRENT_051`, as the basal standing it
/// would hold, while every member fires at the members' rate of ADR-0117's core background: each
/// member receives `fan(SIZE_051)` synapses, each delivering at the class's pair steady at that
/// rate (unkicked) and at the primed peak's pair (`primed`).
fn assembly_input() -> Vec<[i64; 2]> {
    let class = SETS[SET_050];
    let rate = rates_050()[0].0;
    let pairs = [
        steady_under(interval_of(rate), Some(class)),
        primed(class).4,
    ];
    RECURRENT_051
        .iter()
        .map(|&w| {
            pairs.map(|pair| {
                standing(
                    rate,
                    i64::from(fan(SIZE_051)).saturating_mul(i64::from(delivered(w, pair))),
                )
            })
        })
        .collect()
}

/// The members' mean input from the added inhibition at each of `INHIBITORY_051`, as the basal
/// standing it would hold, while every source fires at the rest's rate of ADR-0117's background
/// under the core, (c) and (d), `[condition][weight]`: sixteen synapses a member, each delivering
/// at the pair of ADR-0019's constants steady at that rate.
fn inhibition_input() -> [[i64; 3]; 3] {
    rates_050().map(|(_, rest)| {
        let pair = steady(interval_of(rest));
        INHIBITORY_051.map(|w| {
            standing(
                rest,
                (SOURCES_PER_MEMBER as i64).saturating_mul(i64::from(delivered(w, pair))),
            )
        })
    })
}

/// The members' mean input from the prior's synapses on `exec`, `(excitatory, inhibitory)`, as the
/// basal standing it would hold, Q16.16: every synapse of the arena onto a member of `SIZE_051`,
/// each delivering at the pair its source releases with steady at its rate — a member's under
/// set (ii) at the members' rate, any other unit's under ADR-0019's constants at the rest's —
/// summed and divided among the members.
fn prior_input(exec: &Engine, (members_rate, rest_rate): (Rate, Rate)) -> (i64, i64) {
    let of = membership(&members(SIZE_051));
    let member_pair = steady_under(interval_of(members_rate), Some(SETS[SET_050]));
    let rest_pair = steady(interval_of(rest_rate));
    // `[excitatory, inhibitory]` from the members and from the rest.
    let (mut from_members, mut from_rest) = ([0i64; 2], [0i64; 2]);
    for unit in exec.units() {
        let member = of[unit.id as usize];
        let pair = if member { member_pair } else { rest_pair };
        let polarity = usize::from(unit.flags & FLAG_INHIBITORY != 0);
        for s in unit.fan_out(exec.blocks()) {
            if of[s.target as usize] {
                let into = if member {
                    &mut from_members
                } else {
                    &mut from_rest
                };
                into[polarity] =
                    into[polarity].saturating_add(i64::from(delivered(s.weight_q1_15, pair)));
            }
        }
    }
    let per = |p: usize| {
        standing(members_rate, from_members[p])
            .saturating_add(standing(rest_rate, from_rest[p]))
            .checked_div(i64::from(SIZE_051))
            .unwrap_or(0)
    };
    (per(0), per(1))
}

/// `part` per mille of `whole`, the sign kept.
fn per_mille_of(part: i64, whole: i64) -> i64 {
    part.saturating_mul(1_000).checked_div(whole).unwrap_or(0)
}

/// The arithmetic brief 051 computes on the settled image before any run (`arithmetic_051`): the
/// members' mean input from the prior's excitatory and inhibitory synapses at ADR-0117's rates
/// under the core, (c) and (d), (c)'s with the drained image's sums by polarity scaled onto the
/// settled image's; and under (c) and (d), at each inhibitory weight, what the added inhibition's
/// change from the core takes back of the prior's excitatory change, and of its net change, per
/// mille (`[(c), (d)][weight]`).
type Arithmetic051 = ([(i64, i64); 3], [[(i64, i64); 3]; 2]);

fn arithmetic_051(settled: &[u8]) -> Arithmetic051 {
    let exec = frozen_from(settled, 1024);
    let rates = rates_050();
    let sums = weights_by_polarity(&exec);
    let drained = DRAINED_050.4;
    let mut prior_in = rates.map(|r| prior_input(&exec, r));
    // (c): the drained image's sums by polarity over the settled image's, onto its input.
    let scale = |x: i64, num: i64, den: i64| x.saturating_mul(num).checked_div(den).unwrap_or(0);
    prior_in[1] = (
        scale(prior_in[1].0, drained.1, sums.1),
        scale(prior_in[1].1, drained.0, sums.0),
    );
    let added = inhibition_input();
    let taken = [1, 2].map(|c| {
        let excitatory = prior_in[c].0.saturating_sub(prior_in[0].0);
        let net = excitatory.saturating_add(prior_in[c].1.saturating_sub(prior_in[0].1));
        core::array::from_fn(|w| {
            let back = added[0][w].saturating_sub(added[c][w]);
            (per_mille_of(back, excitatory), per_mille_of(back, net))
        })
    });
    (prior_in, taken)
}

// ------------------------------------------------ the runs (brief 051)

/// The sources' spikes of a run by kind of span — the lead-in, the unkicked spans, the hold spans,
/// the release spans and the tails — and every window's as one number (FNV-1a).
type SourcesRead = ([u32; 5], u64);

/// The index of a kind of span in a reading by kind, `bursts_049`'s order.
const fn span_index(span: Span) -> usize {
    match span {
        Span::LeadIn => 0,
        Span::Unkicked => 1,
        Span::Hold => 2,
        Span::Release => 3,
        Span::Tail => 4,
    }
}

fn sources_read(windows: &[u32], spans: &[(Span, usize, u32)]) -> SourcesRead {
    let mut by = [0u32; 5];
    let mut at = 0usize;
    for &(span, _, epochs) in spans {
        let n = epochs.saturating_mul(WINDOWS as u32) as usize;
        let these = windows.get(at..at.saturating_add(n)).unwrap_or(&[]);
        let sum = these.iter().fold(0u32, |s, &c| s.saturating_add(c));
        by[span_index(span)] = by[span_index(span)].saturating_add(sum);
        at = at.saturating_add(n);
    }
    let words: Vec<i32> = windows.iter().map(|&c| c as i32).collect();
    (by, fnv1a_64(&words))
}

/// A run of brief 051 as pinned: ADR-0117's pin of it, and its sources' reading.
#[derive(Clone, Copy, Debug)]
struct Pinned051 {
    rows: &'static [StretchRow],
    kicks: &'static [[u32; 2]],
    windows: u64,
    epochs: u64,
    sources: SourcesRead,
}

impl Pinned051 {
    fn as_050(&self) -> Pinned050 {
        Pinned050 {
            rows: self.rows,
            kicks: self.kicks,
            windows: self.windows,
            epochs: self.epochs,
        }
    }
}

/// Holds a run of brief 051 to its pin: ADR-0117's `held_to`, and the sources' reading.
fn held_051(run: &Run050, pin: &Pinned051, background: bool, name: &str) {
    run.held_to(&pin.as_050(), background, name);
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    assert_eq!(
        sources_read(&run.run.sources, &spans),
        pin.sources,
        "{name}: the sources"
    );
}

/// A run of brief 051 on `image`, dumped: a background (no kick, no release), a control (the kick
/// and the release, no assembly) or a cell (the assembly wired at `recurrent` with the delays of
/// `seed`), the added inhibition wired at `inhibitory` in each; with `stimulus`, the task's
/// stimulus at every epoch's middle (condition (d)). The sources' spikes are counted beside the
/// rows. Every weight of the arena at its end is its value at the start, the added ones included.
fn run_051(
    image: &[u8],
    recurrent: Option<i16>,
    inhibitory: i16,
    background: bool,
    seed: u64,
    stimulus: bool,
    name: &str,
) -> Run050 {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let mut exec = engine_051(image, recurrent, inhibitory, seed);
    let m = members(SIZE_051);
    let before = weights_of(&exec);
    let task = stimulus.then(stimulus_task);
    let sources = inhibition_sources();
    let (run, epochs) =
        span_protocol_counting(&mut exec, &m, &spans, !background, task.as_ref(), &sources);
    let rows = stretch_rows(&run.windows, SIZE_051);
    eprintln!("DUMP {name} stretches = {rows:?}");
    eprintln!("DUMP {name} kicks = {:?}", run.kicks);
    eprintln!(
        "DUMP {name} hashes = {:#018x} {:#018x}",
        windows_hash(&run.windows),
        epochs_hash(&epochs)
    );
    eprintln!(
        "DUMP {name} sources = {:?}",
        sources_read(&run.sources, &spans)
    );
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    Run050 { run, rows, epochs }
}

/// ADR-0117's cell reproduced under condition `c` of `CONDITIONS_051` on `image` (the drained
/// image under (c)), bit for bit: the run held to ADR-0117's pin and read against ADR-0117's
/// background to ADR-0117's reading, its bursts and, under (d), its readouts.
fn reproduce_050(image: &[u8], c: usize, name: &str) {
    let (s, w) = CELL_050;
    let size = SIZES_050[s];
    let label = format!("{name}: ADR-0117's cell under {}", CONDITION_NAMES_051[c]);
    let (pin, read, bursts, bg) = match CONDITIONS_051[c] {
        None => (
            GRID_RUNS_050[s][w],
            GRID_050[s][w],
            GRID_BURSTS_050[s][w],
            CORE_READ_050[s].0,
        ),
        Some(condition) => {
            let k = c.saturating_sub(1);
            let bg = match condition {
                Condition::Seed => CORE_READ_050[s].0,
                Condition::Drained => DRAINED_READ_050[0].0,
                Condition::Stimulus => STIMULUS_READ_050[0].0,
            };
            (
                CONDITION_RUNS_050[k][0][1],
                CONDITION_GRID_050[k][0][1],
                CONDITION_BURSTS_050[k][0][1],
                bg,
            )
        }
    };
    let seed = if CONDITIONS_051[c] == Some(Condition::Seed) {
        DELAY_SEED_050
    } else {
        DELAY_SEED
    };
    let stimulus = CONDITIONS_051[c] == Some(Condition::Stimulus);
    let run = run_050(
        image,
        size,
        Some(WEIGHTS_050[w]),
        false,
        seed,
        stimulus,
        &label,
    );
    let cell = cell_read(&run, size, bg, &label);
    run.held_to(&pin, false, &label);
    assert_eq!(cell, (read, bursts), "{label}: ADR-0117's reading");
    if stimulus {
        assert_eq!(
            task_read(&run.epochs, &layout(RELEASE_STRETCHES_049[SET_050])),
            STIMULUS_CELL_TASK_050[0][1],
            "{label}: ADR-0117's readouts"
        );
    }
}

/// The backgrounds and the controls of brief 051 on `image`, with the task's stimulus when
/// `stimulus`: for each inhibitory weight the background and the control, each dumped with its
/// readings; then each control's lead-in and first unkicked span shown to be its background's,
/// and the kick read on the engine by ADR-0112's measure, which fires every member once at every
/// inhibitory weight or no cell of the condition runs.
fn backgrounds_controls_051(
    image: &[u8],
    stimulus: bool,
    name: &str,
) -> Vec<(Run050, Run050, BackgroundControl)> {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let mut out = Vec::new();
    for &w in &INHIBITORY_051 {
        let bg = run_051(
            image,
            None,
            w,
            true,
            DELAY_SEED,
            stimulus,
            &format!("{name} background at {w}"),
        );
        let ctl = run_051(
            image,
            None,
            w,
            false,
            DELAY_SEED,
            stimulus,
            &format!("{name} control at {w}"),
        );
        let background = background_049(&bg.rows, &spans);
        let (control, bursts) = cell_read(
            &ctl,
            SIZE_051,
            background,
            &format!("{name} control at {w}"),
        );
        let kick = kick_reading_049(&ctl.run.kicks, SIZE_051);
        eprintln!(
            "DUMP {name} at {w}: background {background:?} kick {kick:?} once {}",
            kicked_once_049(&ctl.run.kicks, SIZE_051)
        );
        if stimulus {
            for (k, run) in [&bg, &ctl].into_iter().enumerate() {
                eprintln!(
                    "DUMP {name} at {w} task[{k}] = {:?}",
                    task_read(&run.epochs, &spans)
                );
            }
            eprintln!(
                "DUMP {name} at {w} sight = {:?} nine = {:?}",
                sight_050(&bg.epochs, false),
                sight_050(&bg.epochs, true)
            );
        }
        out.push((bg, ctl, (background, kick, control, bursts)));
    }
    for (&w, (bg, ctl, _)) in INHIBITORY_051.iter().zip(&out) {
        assert_eq!(
            ctl.rows[..first],
            bg.rows[..first],
            "{name} at {w}: the growth changes nothing unkicked"
        );
        assert!(
            kicked_once_049(&ctl.run.kicks, SIZE_051),
            "{name} at {w}: the kick fires every member once, or it is derived again"
        );
    }
    out
}

/// Holds the backgrounds and controls of condition `b` (of the core, (c) and (d)) to their pins.
fn backgrounds_held(pairs: &[(Run050, Run050, BackgroundControl)], b: usize, name: &str) {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    for (i, (bg, ctl, reading)) in pairs.iter().enumerate() {
        held_051(
            bg,
            &BG_RUNS_051[b][i][0],
            true,
            &format!("{name}: the background {i}"),
        );
        held_051(
            ctl,
            &BG_RUNS_051[b][i][1],
            false,
            &format!("{name}: the control {i}"),
        );
        assert_eq!(*reading, BG_READ_051[b][i], "{name}: {i} read");
        if b == 2 {
            assert_eq!(
                [
                    task_read(&bg.epochs, &spans),
                    task_read(&ctl.epochs, &spans)
                ],
                BG_TASK_051[i],
                "{name}: {i}'s readouts"
            );
            assert_eq!(
                [sight_050(&bg.epochs, false), sight_050(&bg.epochs, true)],
                BG_SIGHT_051[i].map(|s| s.to_vec()),
                "{name}: {i}'s sight"
            );
        }
    }
}

/// The image condition `c` runs on: the settled image, or under (c) the drained image, H-20's arm
/// held to ADR-0110's and the frozen image to ADR-0117's reading of it.
fn image_051(c: usize, name: &str) -> Vec<u8> {
    let settled_image = settled(name);
    if CONDITIONS_051[c] != Some(Condition::Drained) {
        return settled_image;
    }
    let (image, drained) = drained_image(name, &settled_image);
    assert_eq!(drained, DRAINED_050, "{name}: ADR-0117's drained image");
    image
}

/// The cells of condition `c` at the recurrent weights `recurrent` (by index) and every
/// inhibitory weight (brief 051): on the condition's image, each read against its inhibitory
/// weight's background pinned first — under (b) the core's — the kick read on the engine first
/// by its control's pin; every run dumped before any is held.
fn cells_051(c: usize, recurrent: &[usize]) {
    let condition = CONDITIONS_051[c];
    let name = format!(
        "brief 051's cells under {} at {recurrent:?}",
        CONDITION_NAMES_051[c]
    );
    let image = image_051(c, &name);
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let b = backgrounds_of(c);
    let bgs: Vec<Background> = (0..INHIBITORY_051.len())
        .map(|i| {
            let bg = background_049(BG_RUNS_051[b][i][0].rows, &spans);
            assert_eq!(
                bg, BG_READ_051[b][i].0,
                "{name}: the background {i}, pinned first"
            );
            assert!(
                kicked_once_049(BG_RUNS_051[b][i][1].kicks, SIZE_051),
                "{name}: the kick read on the engine at {i} first"
            );
            bg
        })
        .collect();
    let seed = if condition == Some(Condition::Seed) {
        DELAY_SEED_050
    } else {
        DELAY_SEED
    };
    let stimulus = condition == Some(Condition::Stimulus);
    let mut out = Vec::new();
    for &r in recurrent {
        for (i, &w) in INHIBITORY_051.iter().enumerate() {
            let label = format!(
                "brief 051's cell under {} at {:#x} and {w}",
                CONDITION_NAMES_051[c], RECURRENT_051[r]
            );
            let run = run_051(
                &image,
                Some(RECURRENT_051[r]),
                w,
                false,
                seed,
                stimulus,
                &label,
            );
            let cell = cell_read(&run, SIZE_051, bgs[i], &label);
            if stimulus {
                eprintln!("DUMP {label} task = {:?}", task_read(&run.epochs, &spans));
            }
            out.push((r, i, run, cell));
        }
    }
    for (r, i, run, cell) in &out {
        held_051(
            run,
            &CELL_RUNS_051[c][*r][*i],
            false,
            &format!("{name}: {r} {i}"),
        );
        assert_eq!(
            *cell,
            (GRID_051[c][*r][*i], BURSTS_051[c][*r][*i]),
            "{name}: {r} {i}"
        );
        if stimulus {
            assert_eq!(
                task_read(&run.epochs, &spans),
                CELL_TASK_051[*r][*i],
                "{name}: {r} {i}'s readouts"
            );
        }
    }
}

// ------------------------------------------------ the gate (brief 051)

/// The recurrent and inhibitory weights, by index, of the gate's assembly kicked with its
/// inhibition: the heaviest of each.
const GATE_CELL_051: (usize, usize) = (3, 2);

/// The ticks the gate runs after firing one source of the added inhibition: past the local band's
/// longest delay and a refractory window.
const GATE_SOURCE_TICKS: u32 = 600;

const _: () = assert!(GATE_SOURCE_TICKS > 300 + REFRACTORY_TICKS as u32);

/// The draw's reading: the sources, the most and the fewest added synapses a source sends, the
/// added blocks, and the draws within the prior's window of their member.
type SourcesDrawn = (u32, u32, u32, u64, u32);

fn sources_drawn() -> SourcesDrawn {
    let plan = inhibition_plan();
    let counts: Vec<u32> = plan.iter().map(|(_, t)| t.len() as u32).collect();
    let near = members(SIZE_051)
        .iter()
        .map(|&m| {
            sources_of(m)
                .iter()
                .filter(|&&s| ring_distance(1024, m, s) <= PRIOR_WINDOW)
                .count() as u32
        })
        .fold(0u32, u32::saturating_add);
    (
        plan.len() as u32,
        counts.iter().copied().max().unwrap_or(0),
        counts.iter().copied().min().unwrap_or(0),
        inhibition_blocks(),
        near,
    )
}

#[test]
fn an_inhibition_of_the_contexts_own_the_wiring_the_grid_the_arithmetic_the_rules_and_an_assembly_kicked_with_it()
 {
    // The grid: ADR-0117's cell beside it, the inhibitory weights at or below zero.
    assert_eq!(SIZES_050[CELL_050.0], SIZE_051);
    assert_eq!(WEIGHTS_050[CELL_050.1], RECURRENT_051[0]);
    assert_eq!(fan(SIZE_051), FAN_MAX);
    assert!(RECURRENT_051.windows(2).all(|w| w[0] < w[1]));
    assert!(INHIBITORY_051.windows(2).all(|w| w[0] > w[1]));
    assert!(INHIBITORY_051.iter().all(|&w| w < 0));
    assert_eq!(INHIBITORY_051[2], -i16::MAX, "the prior's inhibitory rail");
    assert_eq!(
        RECURRENT_HALVES.concat(),
        (0..RECURRENT_051.len()).collect::<Vec<_>>()
    );
    assert!(
        CONDITIONS_051[1..]
            .iter()
            .zip(CONDITIONS)
            .all(|(a, b)| *a == Some(b))
    );
    assert_eq!((0..4).map(backgrounds_of).collect::<Vec<_>>(), [0, 0, 1, 2]);
    // The prior's inhibitory units: places 4, 9, 14 and 19, readout places of both geometries.
    let inhibitory = inhibitory_units();
    assert_eq!(inhibitory.len(), 204);
    let readout_places = R0_MASK_050 | R1_MASK_050;
    for &u in &inhibitory {
        let place = u.checked_rem(PERIOD).unwrap_or(0);
        assert!([4, 9, 14, 19].contains(&place));
        assert!(readout_places & (1 << place) != 0, "a readout place");
    }
    // The draw: sixteen distinct inhibitory units a member; the plan is the draw.
    let plan = inhibition_plan();
    let m = members(SIZE_051);
    for &member in &m {
        let s = sources_of(member);
        let mut sorted = s.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SOURCES_PER_MEMBER, "sixteen distinct sources");
        assert!(s.iter().all(|u| inhibitory.contains(u)));
        for &u in &s {
            let (_, targets) = plan
                .iter()
                .find(|(source, _)| *source == u)
                .expect("the plan holds every source");
            assert!(targets.contains(&member));
        }
    }
    assert_eq!(
        plan.iter()
            .map(|(_, t)| t.len())
            .fold(0usize, usize::saturating_add),
        m.len().saturating_mul(SOURCES_PER_MEMBER)
    );
    let drawn = sources_drawn();
    eprintln!("DUMP SOURCES_051 = {drawn:?}");
    assert_eq!(drawn, SOURCES_051);
    // The wiring on the instrument's network: the added synapses as the plan says, every
    // synapse the prior wrote untouched, Dale's principle over the whole arena.
    inhibition_on_the_prior();
    // The arithmetic, dumped, then held.
    let delivered_read = inhibition_delivered();
    let assembly_read = assembly_input();
    let inhibition_read = inhibition_input();
    eprintln!("DUMP INHIBITION_DELIVERED_051 = {delivered_read:?}");
    eprintln!("DUMP ASSEMBLY_INPUT_051 = {assembly_read:?}");
    eprintln!("DUMP INHIBITION_INPUT_051 = {inhibition_read:?}");
    assert_eq!(
        delivered_read.as_slice(),
        INHIBITION_DELIVERED_051.as_slice()
    );
    assert_eq!(assembly_read.as_slice(), ASSEMBLY_INPUT_051.as_slice());
    assert_eq!(inhibition_read, INHIBITION_INPUT_051);
    // The rules at their edges, over tables written by hand.
    rules_051_at_their_edges();
    // A marked assembly with its inhibition on the new geometry, kicked for a few hundred ticks.
    let (r, i) = GATE_CELL_051;
    let kicked = kicked_on(
        engine_051(
            &prior_image(),
            Some(RECURRENT_051[r]),
            INHIBITORY_051[i],
            DELAY_SEED,
        ),
        SET_050,
        SIZE_051,
    );
    eprintln!("DUMP GATE_KICKED_051 = {kicked:?}");
    assert_eq!(kicked.as_slice(), GATE_KICKED_051);
    // The rules over the pinned tables.
    over_the_051_tables();
}

/// The added inhibition wired on the instrument's network at the gate's cell, beside the same
/// engine with nothing wired: every synapse the prior wrote is where it was with its target,
/// weight, delay and compartment; each source's chain holds its prior synapses and then its added
/// ones as the plan says, at the inhibitory weight, basal, each delay in the local band as drawn;
/// each member receives sixteen from sixteen distinct inhibitory units; every synapse of an
/// inhibitory unit is at or below zero and every other at or above. Then one source fired alone
/// releases through every added synapse the efficacy its weight gives under its pair.
fn inhibition_on_the_prior() {
    let image = prior_image();
    let (r, i) = GATE_CELL_051;
    let weight = INHIBITORY_051[i];
    let wired = engine_051(&image, Some(RECURRENT_051[r]), weight, DELAY_SEED);
    let img = marked(&image, SETS[SET_050], SIZE_051);
    let bare = frozen_from(
        &grown(
            &img,
            extra_blocks(SIZE_051).saturating_add(inhibition_blocks()),
        ),
        1024,
    );
    let p = prior(1024);
    let plan = inhibition_plan();
    let of = membership(&members(SIZE_051));
    let mut received = vec![Vec::new(); 1024];
    for (unit, before) in wired.units().iter().zip(bare.units()) {
        let after: Vec<_> = unit.fan_out(wired.blocks()).collect();
        let prior_synapses: Vec<_> = before.fan_out(bare.blocks()).collect();
        assert_eq!(prior_synapses.len(), 32, "the prior's 32");
        assert_eq!(
            after[..prior_synapses.len()],
            prior_synapses[..],
            "unit {}'s prior synapses untouched",
            unit.id
        );
        let inhibitory = unit.flags & FLAG_INHIBITORY != 0;
        assert!(
            after
                .iter()
                .all(|s| (s.weight_q1_15 <= 0) == inhibitory || s.weight_q1_15 == 0),
            "Dale's principle at unit {}",
            unit.id
        );
        let added = &after[prior_synapses.len()..];
        let source = unit.id as u32;
        match plan.iter().find(|(s, _)| *s == source) {
            Some((_, targets)) => {
                assert_eq!(
                    added.len(),
                    targets.len(),
                    "source {source}'s added synapses"
                );
                for (s, &t) in added.iter().zip(targets) {
                    assert_eq!(s.target, t);
                    assert_eq!(s.weight_q1_15, weight);
                    assert!(!s.apical, "basal");
                    assert_eq!(s.delay_ticks, delay_drawn(INHIBITION_SEED, source, t));
                    assert!((p.delay_min..=p.delay_max).contains(&s.delay_ticks));
                    received[t as usize].push(source);
                }
            }
            None if of[unit.id as usize] => {
                assert_eq!(added.len(), fan(SIZE_051) as usize, "a member's recurrent");
            }
            None => assert!(added.is_empty(), "unit {} unwired", unit.id),
        }
    }
    for (u, from) in received.iter().enumerate() {
        let expected = if of[u] { SOURCES_PER_MEMBER } else { 0 };
        assert_eq!(from.len(), expected, "unit {u} receives the inhibition");
        let mut distinct = from.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert_eq!(distinct.len(), expected, "from distinct sources");
    }
    // One source fired alone, with no drive: its every added slot holds the release its weight
    // gives under the pair of its last spike, which ADR-0019's step gives from its rest.
    let mut exec = wired;
    let (source, targets) = plan.first().expect("a source");
    let rested = &exec.units()[*source as usize];
    let mut copy = DendriticSuperNeuron::new(0);
    copy.stp_u_rel = rested.stp_u_rel;
    copy.stp_r_ves = rested.stp_r_ves;
    copy.flags = rested.flags;
    let mut previous =
        (rested.last_soma_spike_tick != NO_SPIKE_ON_RECORD).then_some(rested.last_soma_spike_tick);
    let inject = exec.injector();
    for _ in 0..STIMULUS_MESSAGES {
        inject
            .inject(*source, spike_message(STIMULUS_Q16, false))
            .expect("the ring has room");
    }
    let before = weights_of(&exec);
    for _ in 0..GATE_SOURCE_TICKS {
        exec.tick();
    }
    assert_eq!(weights_of(&exec), before, "no weight moved");
    let fired: Vec<u32> = exec
        .train()
        .iter()
        .filter(|&&(_, u)| u == *source)
        .map(|&(t, _)| t)
        .collect();
    assert!(!fired.is_empty(), "the source fired");
    assert!(
        exec.train().iter().all(|&(_, u)| u == *source),
        "nothing else fired"
    );
    let mut pair = (0u8, 0u8);
    for &t in &fired {
        let elapsed = previous.map_or(u32::MAX, |p| t.wrapping_sub(p));
        pair = copy.step_stp(elapsed);
        previous = Some(t);
    }
    let efficacy = synaptic_efficacy_q16(weight, pair.0, pair.1);
    assert!(efficacy < 0, "an inhibitory release");
    let added: Vec<_> = exec.units()[*source as usize]
        .fan_out(exec.blocks())
        .skip(32)
        .collect();
    assert_eq!(added.len(), targets.len());
    for s in added {
        assert_eq!(
            exec.blocks()[s.block_idx as usize].last_release_q16[s.slot as usize],
            efficacy,
            "the added synapse onto {} released",
            s.target
        );
    }
}

/// The rules of brief 051 at their edges, over tables written by hand: robust, the neighbours on
/// the recurrent and inhibitory steps, the second round's cell and its ties, what failed.
fn rules_051_at_their_edges() {
    let usable = Holding {
        first_half: 8,
        held: 8,
        ignited: 0,
        let_go: 8,
        spills: Some(false),
    };
    let never = Holding { held: 6, ..usable };
    let runs = Holding {
        ignited: 2,
        ..usable
    };
    let stays = Holding {
        let_go: 6,
        ..usable
    };
    let spills = Holding {
        spills: Some(true),
        ..usable
    };
    assert!(usable.usable() && !never.usable() && !runs.usable() && !stays.usable());
    assert!(!spills.usable());
    // Robust: usable under every one of the four.
    let mut grids: [Grid051; 4] = [[[never; 3]; 4]; 4];
    assert!(robust_051(&grids).is_empty());
    assert_eq!(round_two_051(&grids[0], &[]), None);
    for g in grids.iter_mut() {
        g[1][2] = usable;
        g[3][0] = usable;
    }
    assert_eq!(robust_051(&grids), [(1, 2), (3, 0)]);
    for (c, name) in CONDITION_NAMES_051.iter().enumerate() {
        let mut under = grids;
        under[c][1][2] = runs;
        assert_eq!(robust_051(&under), [(3, 0)], "{name} fails it");
    }
    // The neighbours: one recurrent step and one inhibitory step either way, a diagonal not.
    let mut core: Grid051 = [[never; 3]; 4];
    core[2][2] = usable;
    assert_eq!(
        usable_neighbours(&core, (1, 1)),
        0,
        "a diagonal is not a neighbour"
    );
    for cell in [(0, 1), (2, 1), (1, 0), (1, 2)] {
        core[cell.0][cell.1] = usable;
    }
    assert_eq!(usable_neighbours(&core, (1, 1)), 4);
    assert_eq!(usable_neighbours(&core, (3, 2)), 1, "a corner, one usable");
    assert_eq!(usable_neighbours(&core, (0, 0)), 2, "a corner has two");
    assert_eq!(usable_neighbours(&core, (3, 0)), 0);
    // The second round's cell: the most usable neighbours, then the lighter recurrent weight, then
    // the lighter inhibition.
    let mut core: Grid051 = [[never; 3]; 4];
    for cell in [(1, 0), (1, 1), (2, 1), (3, 2)] {
        core[cell.0][cell.1] = usable;
    }
    assert_eq!(
        [(1, 0), (1, 1), (2, 1), (3, 2)].map(|c| usable_neighbours(&core, c)),
        [1, 2, 1, 0]
    );
    assert_eq!(
        round_two_051(&core, &[(3, 2), (2, 1), (1, 1)]),
        Some((1, 1))
    );
    assert_eq!(
        round_two_051(&core, &[(2, 1), (1, 0)]),
        Some((1, 0)),
        "a tie to the lighter recurrent weight"
    );
    let mut core: Grid051 = [[never; 3]; 4];
    for cell in [(2, 0), (2, 2), (3, 0), (3, 2)] {
        core[cell.0][cell.1] = usable;
    }
    assert_eq!(
        [(2, 0), (2, 2)].map(|c| usable_neighbours(&core, c)),
        [1, 1]
    );
    assert_eq!(
        round_two_051(&core, &[(2, 2), (2, 0)]),
        Some((2, 0)),
        "at one recurrent weight a tie to the lighter inhibition"
    );
    // What failed, per condition.
    let mut under = grids;
    under[2][1][2] = runs;
    under[3][1][2] = spills;
    under[1][0][0] = stays;
    let failures = failures_051(&under);
    assert_eq!(
        failures[1][2],
        [
            [false; 3],
            [false; 3],
            [false, true, false],
            [false, true, false]
        ]
    );
    assert_eq!(
        failures[0][0],
        [
            [true, false, false],
            [false, false, true],
            [true, false, false],
            [true, false, false]
        ]
    );
    // The sources' reading by kind of span.
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let windows: Vec<u32> = (0..RUN_EPOCHS_049[SET_050].saturating_mul(WINDOWS as u32))
        .map(|_| 1)
        .collect();
    let (by, _) = sources_read(&windows, &spans);
    assert_eq!(
        by,
        [128, 1_024, 1_024, 768, 256],
        "sixteen, 8 × 16, 8 × 16, 8 × 12 and 8 × 4 epochs of eight windows"
    );
    assert_eq!(
        by.iter().fold(0u32, |s, &c| s.saturating_add(c)),
        windows.len() as u32
    );
}

/// The rules over brief 051's pinned tables: each run's stretches as the rules read them against
/// its own background; the kick firing every member once under every condition and inhibitory
/// weight; each control's lead-in and first unkicked span its background's; and the reading
/// ADR-0120 gives — the core grid's usable region, the robust cells and the cell for the second
/// round, and what failed.
fn over_the_051_tables() {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let n = stretches_of(&spans).len();
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    for (b, runs) in BG_RUNS_051.iter().enumerate() {
        for (i, [bg, ctl]) in runs.iter().enumerate() {
            assert_eq!((bg.rows.len(), ctl.rows.len()), (n, n));
            let background = background_049(bg.rows, &spans);
            let read = BG_READ_051[b][i];
            assert_eq!(background, read.0);
            assert_eq!(kick_reading_049(ctl.kicks, SIZE_051), read.1);
            assert!(
                kicked_once_049(ctl.kicks, SIZE_051),
                "the kick fires every member once under {b} at {i}"
            );
            assert_eq!(holding(ctl.rows, &spans, background), read.2);
            assert_eq!(
                ctl.rows[..first],
                bg.rows[..first],
                "the growth changes nothing unkicked"
            );
        }
    }
    for (c, runs) in CELL_RUNS_051.iter().enumerate() {
        for (r, row) in runs.iter().enumerate() {
            for (i, pin) in row.iter().enumerate() {
                let bg = BG_READ_051[backgrounds_of(c)][i].0;
                assert_eq!(pin.rows.len(), n);
                assert_eq!(pin.kicks.len(), ROUNDS, "a kick a round");
                assert_eq!(
                    holding(pin.rows, &spans, bg),
                    GRID_051[c][r][i],
                    "{} at {r} {i}",
                    CONDITION_NAMES_051[c]
                );
            }
        }
    }
    let usable: Vec<(usize, usize)> = (0..RECURRENT_051.len())
        .flat_map(|r| (0..INHIBITORY_051.len()).map(move |i| (r, i)))
        .filter(|&(r, i)| GRID_051[0][r][i].usable())
        .collect();
    assert_eq!(
        usable.as_slice(),
        USABLE_051,
        "the core grid's usable region"
    );
    let robust = robust_051(&GRID_051);
    assert_eq!(robust.as_slice(), ROBUST_051);
    assert_eq!(round_two_051(&GRID_051[0], &robust), ROUND_TWO_051);
    assert_eq!(failures_051(&GRID_051), FAILURES_051);
}

// ------------------------------------------------ the runs, weekly (brief 051)

/// The core's backgrounds and controls (brief 051): on the settled image, the arithmetic first,
/// before any run; then ADR-0117's cell reproduced under the core and under (b), bit for bit;
/// then at each inhibitory weight the background and the control, the kick read on the engine.
fn core_backgrounds_051() {
    let name = "brief 051's backgrounds and controls of the core";
    let image = image_051(0, name);
    let arithmetic = arithmetic_051(&image);
    eprintln!("DUMP ARITHMETIC_051 = {arithmetic:?}");
    assert_eq!(
        arithmetic, ARITHMETIC_051,
        "{name}: the arithmetic, pinned before any run"
    );
    reproduce_050(&image, 0, name);
    reproduce_050(&image, 1, name);
    let pairs = backgrounds_controls_051(&image, false, name);
    backgrounds_held(&pairs, 0, name);
}

/// Condition (c)'s backgrounds and controls (brief 051): the drained image from H-20's arm, the
/// members' input from its prior synapses read at (c)'s rates, ADR-0117's cell reproduced on it,
/// then at each inhibitory weight the background and the control.
fn drained_backgrounds_051() {
    let name = "brief 051's backgrounds and controls under (c)";
    let image = image_051(2, name);
    let input = prior_input(&frozen_from(&image, 1024), rates_050()[1]);
    eprintln!("DUMP DRAINED_INPUT_051 = {input:?}");
    reproduce_050(&image, 2, name);
    let pairs = backgrounds_controls_051(&image, false, name);
    assert_eq!(input, DRAINED_INPUT_051, "{name}: the members' input");
    backgrounds_held(&pairs, 1, name);
}

/// Condition (d)'s backgrounds and controls (brief 051): on the settled image with the task's
/// stimulus at every epoch's middle, ADR-0117's cell reproduced under it, then at each inhibitory
/// weight the background and the control, their readouts and ADR-0065's measure on each
/// background.
fn stimulus_backgrounds_051() {
    let name = "brief 051's backgrounds and controls under (d)";
    let image = image_051(3, name);
    reproduce_050(&image, 3, name);
    let pairs = backgrounds_controls_051(&image, true, name);
    backgrounds_held(&pairs, 2, name);
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_the_backgrounds_and_the_controls_of_the_core_paused() {
    core_backgrounds_051();
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_the_core_grid_at_the_lighter_recurrent_weights_paused() {
    cells_051(0, &RECURRENT_HALVES[0]);
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_the_core_grid_at_the_heavier_recurrent_weights_paused() {
    cells_051(0, &RECURRENT_HALVES[1]);
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_with_the_delays_of_seed_49_at_the_lighter_recurrent_weights_paused()
{
    cells_051(1, &RECURRENT_HALVES[0]);
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_with_the_delays_of_seed_49_at_the_heavier_recurrent_weights_paused()
{
    cells_051(1, &RECURRENT_HALVES[1]);
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_the_backgrounds_and_the_controls_on_the_image_h_20_leaves_paused() {
    drained_backgrounds_051();
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_on_the_image_h_20_leaves_paused() {
    cells_051(2, &[0, 1, 2, 3]);
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_the_backgrounds_and_the_controls_under_the_tasks_stimulus_paused() {
    stimulus_backgrounds_051();
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_under_the_tasks_stimulus_at_the_lighter_recurrent_weights_paused() {
    cells_051(3, &RECURRENT_HALVES[0]);
}

#[test]
#[ignore]
fn the_contexts_own_inhibition_under_the_tasks_stimulus_at_the_heavier_recurrent_weights_paused() {
    cells_051(3, &RECURRENT_HALVES[1]);
}

// ------------------------------------------------ brief 051's arithmetic, pinned before any run

/// The draw: the sources, the most and the fewest added synapses one sends, the added blocks, and
/// the draws within the prior's window of their member.
const SOURCES_051: SourcesDrawn = (203, 13, 1, 332, 20);

/// The members' mean input from the assembly at each recurrent weight, `[unkicked, primed]`.
const ASSEMBLY_INPUT_051: [[i64; 2]; 4] = [[1827, 2974], [2741, 4461], [3656, 5948], [5483, 8922]];

// ------------------------------------------------ brief 051's readings, pinned from the runs

/// Condition (c)'s drained image: the members' input from its prior synapses at (c)'s rates.
const DRAINED_INPUT_051: (i64, i64) = (2274, -110);

/// The core grid's usable cells, `(recurrent, inhibitory)` by index.
const USABLE_051: &[(usize, usize)] = &[];

/// The robust cells.
const ROBUST_051: &[(usize, usize)] = &[];

/// The cell for the second round.
const ROUND_TWO_051: Option<(usize, usize)> = None;

// ================================================ brief 052 (ADR-0124): the slow current, measured

// ------------------------------------------------ the arms, the constants and the grid (brief 052), before any run

/// The assembly every run of brief 052 wires: ADR-0117's cell's 64 members, at ADR-0112's
/// placement on ADR-0116's geometry.
const SIZE_052: u32 = 64;

/// The two arms (ADR-0122): every member marked for the slow current, (A) under ADR-0019's
/// short-term plasticity — no class — and (B) marked facilitating under ADR-0114's set (ii) as
/// well.
const ARMS_052: [Option<StpClass>; 2] = [None, Some(SETS[SET_050])];

const ARM_NAMES_052: [&str; 2] = ["(A)", "(B)"];

/// The slow potential's leak shift: $\tau_s = 2^{13}$ ticks, 82 ms (ADR-0122).
const LEAK_SHIFT_052: u8 = 13;

/// The gate's two voltages (ADR-0122): $V_{lo}$ the soma's standing under the drive's mean
/// input as the oracle reads it (`DRIVE_STANDING`, 0.436), and $V_{hi}$ the threshold's base.
const V_LO_052: i32 = DRIVE_STANDING.1;

const V_HI_052: i32 = THRESHOLD_BASE;

/// The recurrent weights, Q1.15: an eighth, a quarter, three eighths and a half of the range.
const WEIGHTS_052: [i16; 4] = [0x1000, 0x2000, 0x3000, 0x4000];

/// The three input shifts $g$, placed by a rule written before the arithmetic was read (ADR-0124):
/// the middle one is the largest at which, under arm (A) at ADR-0117's weight, the slow potential of
/// the held state at 20 Hz is at or above the critical level (`SLOW_CRITICAL_052`), and the other two
/// are one step either side — 0, 1 and 2 when it is 0. By the arithmetic it is 1: 119 859 at shift 1
/// and 60 113 at shift 2 against 74 589 (`ARITHMETIC_052`, the gate).
const SHIFTS_052: [u8; 3] = [0, 1, 2];

/// The most cells the conditions run (ADR-0124): past it, those with the most usable neighbours.
const CONDITION_CELLS_MAX: usize = 8;

/// The slow current at input shift `shift`: ADR-0122's leak and voltages.
const fn current_052(shift: u8) -> SlowCurrent {
    SlowCurrent {
        leak_shift: LEAK_SHIFT_052,
        input_shift: shift,
        v_lo_q16: V_LO_052,
        v_hi_q16: V_HI_052,
    }
}

/// The class a run's bursts are read against (`bursts_049`): the arm's, or ADR-0019's constants,
/// which `step_stp_class` at `StpClass::REFERENCE` is bit for bit (ADR-0114).
fn class_052(arm: usize) -> StpClass {
    ARMS_052[arm].unwrap_or(StpClass::REFERENCE)
}

// ------------------------------------------------ the image, marked for the slow current (brief 052)

/// `image` with arm `arm`'s constants and marks: the slow current at input shift `shift` set in
/// its modulator record, and the arm's class when it has one; the members of `SIZE_052` marked for
/// the slow current and, under a class, facilitating; each section re-sealed. The test's own marks,
/// as `grown` and `wire` are its own wiring; every other byte the image's.
fn slow_marked(image: &[u8], arm: usize, shift: u8) -> Vec<u8> {
    slow_marked_under(image, arm, current_052(shift))
}

/// `slow_marked` with the slow current `current` in place of ADR-0124's at a shift (brief 053's
/// gate raised).
fn slow_marked_under(image: &[u8], arm: usize, current: SlowCurrent) -> Vec<u8> {
    let mut img = image.to_vec();
    let header = MenteionFileHeader::decode(img[0..64].try_into().unwrap());
    let m = members(SIZE_052);
    let class = ARMS_052[arm];
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        let (offset, length) = (entry.offset as usize, entry.length as usize);
        let section = &mut img[offset..][..length];
        match entry.kind {
            SECTION_MODULATOR => {
                assert_eq!(
                    &section[32..48],
                    &[0u8; 16],
                    "no class and no slow current before"
                );
                if let Some(c) = class {
                    section[32..36].copy_from_slice(&[1, c.u, c.tau_f_shift, c.tau_d_shift]);
                }
                section[36..40].copy_from_slice(&[1, current.leak_shift, current.input_shift, 0]);
                section[40..44].copy_from_slice(&current.v_lo_q16.to_le_bytes());
                section[44..48].copy_from_slice(&current.v_hi_q16.to_le_bytes());
            }
            SECTION_NEURON => {
                let mark = if class.is_some() {
                    FLAG_SLOW | FLAG_FACILITATING
                } else {
                    FLAG_SLOW
                };
                for &u in &m {
                    let flags = (u as usize).saturating_mul(64).saturating_add(57);
                    section[flags] |= mark;
                }
            }
            _ => continue,
        }
        entry.crc64 = crc64(&img[offset..][..length]);
        img[at..][..64].copy_from_slice(&entry.encode());
    }
    img
}

/// The arm's constants and marks asserted on `exec`: the slow current at `shift` and the arm's
/// class; every member marked for the slow current and, under a class, facilitating, and no other
/// unit; no slow potential in any unit.
fn slow_marks_held(exec: &Engine, arm: usize, current: SlowCurrent) {
    assert_eq!(
        (exec.slow_current(), exec.stp_class()),
        (Some(current), ARMS_052[arm]),
        "the image's slow current and class"
    );
    let of = membership(&members(SIZE_052));
    for unit in exec.units() {
        let member = of[unit.id as usize];
        assert_eq!(
            (
                unit.flags & FLAG_SLOW != 0,
                unit.flags & FLAG_FACILITATING != 0
            ),
            (member, member && ARMS_052[arm].is_some()),
            "unit {} marked as a member",
            unit.id
        );
        assert_eq!(unit.v_slow, 0, "unit {}: no slow potential yet", unit.id);
    }
}

/// The engine of a run of brief 052 under arm `arm` at input shift `shift`: the frozen image
/// marked; grown by the assembly's blocks when `grow` (a control or a cell), and wired at `weight`
/// with the delays of `seed` when given (a cell). The constants and the marks asserted.
fn engine_052(
    image: &[u8],
    arm: usize,
    shift: u8,
    grow: bool,
    weight: Option<i16>,
    seed: u64,
) -> Engine {
    engine_under(image, arm, current_052(shift), grow, weight, seed)
}

/// `engine_052` with the slow current `current` in place of ADR-0124's at a shift (brief 053's
/// gate raised).
fn engine_under(
    image: &[u8],
    arm: usize,
    current: SlowCurrent,
    grow: bool,
    weight: Option<i16>,
    seed: u64,
) -> Engine {
    let img = slow_marked_under(image, arm, current);
    let mut exec = if grow {
        frozen_from(&grown(&img, extra_blocks(SIZE_052)), 1024)
    } else {
        frozen_from(&img, 1024)
    };
    slow_marks_held(&exec, arm, current);
    if let Some(w) = weight {
        wire_drawn(&mut exec, SIZE_052, w, seed);
    }
    exec
}

// ------------------------------------------------ the rules (brief 052), before any run

/// Brief 052's core grid read by ADR-0115's rules, `[arm][shift][weight]` over `ARMS_052`,
/// `SHIFTS_052` and `WEIGHTS_052`.
type Grid052 = [[[Holding; 4]; 3]; 2];

/// A cell of brief 052's grid by index: `(arm, shift, weight)`.
type Cell052 = (usize, usize, usize);

/// The order ties go by, ADR-0117's: the lighter weight, then the smaller shift, then arm (A).
fn tie_key((a, s, w): Cell052) -> (usize, usize, usize) {
    (w, s, a)
}

/// A cell's usable neighbours in the core grid, within its arm: of the cells one weight step either
/// way at its shift and one shift step either way at its weight, at most four, those usable.
fn neighbours_052(core: &Grid052, (a, s, w): Cell052) -> u32 {
    usable_neighbours(&core[a], (s, w))
}

/// The core grid's usable cells, in the grid's order.
fn usable_052(core: &Grid052) -> Vec<Cell052> {
    let mut out = Vec::new();
    for (a, arm) in core.iter().enumerate() {
        for (s, row) in arm.iter().enumerate() {
            for (w, cell) in row.iter().enumerate() {
                if cell.usable() {
                    out.push((a, s, w));
                }
            }
        }
    }
    out
}

/// The cells the conditions run (ADR-0124): every cell usable in the core grid; when more than
/// `CONDITION_CELLS_MAX` are, those with the most usable neighbours, ties by `tie_key`. In the
/// grid's order.
fn condition_cells_052(core: &Grid052) -> Vec<Cell052> {
    let mut cells = usable_052(core);
    if cells.len() > CONDITION_CELLS_MAX {
        cells.sort_by_key(|&c| (core::cmp::Reverse(neighbours_052(core, c)), tie_key(c)));
        cells.truncate(CONDITION_CELLS_MAX);
        cells.sort_unstable();
    }
    cells
}

/// Robust: of `cells` — the cells the conditions ran, each usable in the core grid — those usable
/// under each of (b), (c) and (d), `under[c][i]` the reading of `cells[i]` under condition `c`; in
/// the grid's order.
fn robust_052(cells: &[Cell052], under: &[Vec<Holding>; 3]) -> Vec<Cell052> {
    cells
        .iter()
        .enumerate()
        .filter(|&(i, _)| under.iter().all(|c| c.get(i).is_some_and(Holding::usable)))
        .map(|(_, &c)| c)
        .collect()
}

/// The cell for the second round: of the robust cells, the one with the most usable neighbours in
/// the core grid, ties by `tie_key`; none when none is robust.
fn round_two_052(core: &Grid052, robust: &[Cell052]) -> Option<Cell052> {
    let mut ordered = robust.to_vec();
    ordered.sort_by_key(|&c| tie_key(c));
    let mut best: Option<(Cell052, u32)> = None;
    for cell in ordered {
        let n = neighbours_052(core, cell);
        if best.is_none_or(|(_, most)| n > most) {
            best = Some((cell, n));
        }
    }
    best.map(|(cell, _)| cell)
}

/// What failed in each cell the conditions ran, in the core grid and under (b), (c) and (d), each
/// by ADR-0115's `failed`.
fn failures_052(
    core: &Grid052,
    cells: &[Cell052],
    under: &[Vec<Holding>; 3],
) -> Vec<[[bool; 3]; 4]> {
    cells
        .iter()
        .enumerate()
        .map(|(i, &(a, s, w))| {
            let mut out = [[false; 3]; 4];
            out[0] = failed(&core[a][s][w]);
            for (c, condition) in under.iter().enumerate() {
                if let Some(h) = condition.get(i) {
                    out[c.saturating_add(1)] = failed(h);
                }
            }
            out
        })
        .collect()
}

// ------------------------------------------------ the arithmetic (brief 052), before any run

/// A slow input as a train: `(efficacy after the gain, period in ticks)`, its message landing on
/// every tick of the run that is a multiple of its period.
type Train = (i32, u32);

/// The ticks the slow rule is stepped from zero before it is read, sixteen time constants, and
/// the ticks it is read over.
const SLOW_WARM: u32 = 1 << 17;

const SLOW_READ: u32 = 1 << 17;

/// The slow potential under `trains` by the slow rule itself: a unit marked for the slow current
/// whose threshold is zero — it never fires, and nothing but its slow potential is read — stepped
/// by `integrate_slow` under the slow current `c` with the trains' messages as its slow input,
/// from zero; after `SLOW_WARM` ticks, its mean over `SLOW_READ`, Q16.16. The gate's voltages
/// are not read by the slow potential's rule, so its level is the same at every voltage (brief
/// 053).
fn slow_level(trains: &[Train], c: SlowCurrent) -> i32 {
    let mut unit = DendriticSuperNeuron::new(0);
    unit.flags = FLAG_SLOW;
    let mut sum = 0i64;
    for t in 0..SLOW_WARM.saturating_add(SLOW_READ) {
        let input = trains
            .iter()
            .filter(|&&(_, period)| t.checked_rem(period) == Some(0))
            .fold(0i32, |s, &(e, _)| s.saturating_add(e));
        unit.integrate_slow(0, 0, input, t, c);
        if t >= SLOW_WARM {
            sum = sum.saturating_add(i64::from(unit.v_slow));
        }
    }
    sum.checked_div(i64::from(SLOW_READ)).unwrap_or(0) as i32
}

/// The assembly's input to one member while every member fires every `interval` ticks: `fan`
/// synapses at `weight`, each delivering at the arm's pair steady at that interval, after the gain
/// (`delivered`), as one train of `fan` times the members' rate.
fn assembly_train(arm: usize, weight: i16, interval: u32) -> Train {
    let pair = steady_under(interval, ARMS_052[arm]);
    (
        delivered(weight, pair),
        interval.checked_div(fan(SIZE_052)).unwrap_or(0),
    )
}

/// The weights of the prior's excitatory synapses onto the members that land in their basal
/// compartment, on `exec`: `(from the rest, from the members)`. The prior's far band joins a few
/// members to one another, whose synapses fire at the members' rate under the arm's plasticity.
fn prior_onto_members(exec: &Engine) -> (Vec<i16>, Vec<i16>) {
    let of = membership(&members(SIZE_052));
    let (mut rest, mut from_members) = (Vec::new(), Vec::new());
    for unit in exec.units() {
        if unit.flags & FLAG_INHIBITORY != 0 {
            continue;
        }
        for s in unit.fan_out(exec.blocks()) {
            if of[s.target as usize] && !s.apical {
                if of[unit.id as usize] {
                    from_members.push(s.weight_q1_15);
                } else {
                    rest.push(s.weight_q1_15);
                }
            }
        }
    }
    (rest, from_members)
}

/// `weights`' synapses onto the members as one train, each source firing every `interval` ticks and
/// delivering at `pair` after the gain: per member, `count / SIZE_052` messages every interval, as one
/// message of the mean efficacy every `interval × SIZE_052 / count` ticks; none without a synapse.
fn train_of(weights: &[i16], pair: (u8, u8), interval: u32) -> Option<Train> {
    let count = weights.len() as i64;
    let sum = weights.iter().fold(0i64, |s, &w| {
        s.saturating_add(i64::from(delivered(w, pair)))
    });
    let efficacy = sum.checked_div(count)? as i32;
    let period = i64::from(interval)
        .saturating_mul(i64::from(SIZE_052))
        .checked_div(count)? as u32;
    Some((efficacy, period))
}

/// The intervals the held state is read at: 5, 10 and 20 Hz of every member.
const HELD_INTERVALS: [u32; 3] = [20_000, 10_000, 5_000];

/// The slow potential's levels by the slow rule, `[arm][weight][shift]`, with the prior's
/// synapses from the rest at the rest's rate of ADR-0117's core background and the members' at the
/// members' own rate: the background (the members unwired, at their rate in ADR-0117's core
/// background); the quiet state (the assembly's input beside it at that rate); and the held state
/// (every member at 5, 10 and 20 Hz, the assembly's input and the members' prior synapses at that
/// rate).
type Levels052 = [[[[i32; 5]; 3]; 4]; 2];

fn slow_levels(exec: &Engine, shifts: &[u8; 3]) -> Levels052 {
    slow_levels_under(exec, shifts, V_LO_052)
}

/// `slow_levels` with the gate's lower voltage at `v_lo` (brief 053): the slow potential's rule
/// reads no gate, so the levels are `slow_levels`' at every voltage, which brief 053 holds.
fn slow_levels_under(exec: &Engine, shifts: &[u8; 3], v_lo: i32) -> Levels052 {
    let (rest_weights, member_weights) = prior_onto_members(exec);
    let (quiet, rest) = (interval_of(rates_050()[0].0), interval_of(rates_050()[0].1));
    let from_rest =
        train_of(&rest_weights, steady(rest), rest).expect("the prior reaches the members");
    let mut out = [[[[0i32; 5]; 3]; 4]; 2];
    for (a, by_weight) in out.iter_mut().enumerate() {
        // The trains at the members' interval `interval`: the prior's, and with the assembly's at
        // weight `w` when given.
        let trains = |interval: u32, weight: Option<i16>| -> Vec<Train> {
            let pair = steady_under(interval, ARMS_052[a]);
            [Some(from_rest), train_of(&member_weights, pair, interval)]
                .into_iter()
                .chain(core::iter::once(
                    weight.map(|w| assembly_train(a, w, interval)),
                ))
                .flatten()
                .collect()
        };
        for (w, by_shift) in by_weight.iter_mut().enumerate() {
            let weight = WEIGHTS_052[w];
            for (k, levels) in by_shift.iter_mut().enumerate() {
                let c = current_053(shifts[k], v_lo);
                levels[0] = slow_level(&trains(quiet, None), c);
                levels[1] = slow_level(&trains(quiet, Some(weight)), c);
                for (level, &interval) in levels[2..].iter_mut().zip(HELD_INTERVALS.iter()) {
                    *level = slow_level(&trains(interval, Some(weight)), c);
                }
            }
        }
    }
    out
}

/// The ticks a held slow potential is given to fire a unit, an epoch.
const CRITICAL_TICKS: u32 = EPOCH_TICKS;

/// Whether a slow potential held at `level` — written back into the unit before every tick — fires
/// a unit under the drive's mean input within `CRITICAL_TICKS`, by the slow rule itself with the
/// gate's lower voltage at `v_lo`, its soma from an eighth of the gate's span above `v_lo`. At
/// ADR-0124's voltage, the drive's mean standing, the span is the gap above the standing
/// (brief 052); brief 053 reads it at each of its voltages.
fn fires_held(level: i32, v_lo: i32) -> bool {
    let c = current_053(0, v_lo);
    let mean = drive_mean_per_tick();
    let gap = THRESHOLD_BASE.saturating_sub(v_lo);
    let mut unit = DendriticSuperNeuron::new(0);
    unit.flags = FLAG_SLOW;
    unit.v_thresh = THRESHOLD_BASE;
    unit.v_basal = DRIVE_STANDING.0;
    unit.v_soma = v_lo.saturating_add(gap >> 3);
    (1..=CRITICAL_TICKS).any(|t| {
        unit.v_slow = level;
        unit.integrate_slow(mean, 0, 0, t, c)
    })
}

/// The highest slow potential the critical level is searched below: 4.0.
const CRITICAL_MAX: i32 = 4 * THRESHOLD_BASE;

/// The critical slow potential: the least level that, held, fires a unit at the drive's mean
/// standing by the slow rule (`fires_held`), by bisection over `[0, CRITICAL_MAX]`: a larger level
/// never gives the soma less, so the search is over a monotone predicate, and the found level is
/// held to fire where one LSB less does not.
fn slow_critical() -> i32 {
    slow_critical_at(V_LO_052, CRITICAL_MAX)
}

/// `slow_critical` with the gate's lower voltage at `v_lo` (`fires_held`, brief 053), searched
/// over `[0, max]`.
fn slow_critical_at(v_lo: i32, max: i32) -> i32 {
    least_firing(max, |level| fires_held(level, v_lo))
}

/// The least value in `(0, max]` at which `fires` holds, by bisection: `fires` is monotone, false
/// at zero and true at `max` (asserted, "the search brackets it"), and the found value is held to
/// fire where one LSB less does not.
fn least_firing(max: i32, fires: impl Fn(i32) -> bool) -> i32 {
    let (mut low, mut high) = (0i32, max);
    assert!(!fires(low) && fires(high), "the search brackets it");
    for _ in 0..u32::BITS {
        if high.saturating_sub(low) <= 1 {
            break;
        }
        let mid = low.saturating_add(high.saturating_sub(low) >> 1);
        if fires(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    assert!(fires(high) && !fires(high.saturating_sub(1)));
    high
}

/// The gate's opening at the soma's standing under the drive's mean input, a quarter, a half and
/// three quarters of the gap above it, at 0.9 and at the threshold's base.
fn gate_openings() -> [i32; 6] {
    let c = current_052(0);
    let gap = THRESHOLD_BASE.saturating_sub(V_LO_052);
    [
        V_LO_052,
        V_LO_052.saturating_add(gap >> 2),
        V_LO_052.saturating_add(gap >> 1),
        V_LO_052.saturating_add(gap.saturating_sub(gap >> 2)),
        58_982,
        THRESHOLD_BASE,
    ]
    .map(|v| c.gate_q16(v))
}

/// The per-spike release the held state has over the quiet one, per mille, `[arm][interval]`: what
/// one spike of a member delivers at the arm's pair steady at 5, 10 and 20 Hz over what it delivers
/// at the members' rate of ADR-0117's core background. The four rounds before brief 052 read 1.40
/// to 1.60 between a primed and an unkicked pair.
fn per_spike_052() -> [[u32; 3]; 2] {
    let quiet = interval_of(rates_050()[0].0);
    core::array::from_fn(|a| {
        let at = delivered(0x2000, steady_under(quiet, ARMS_052[a]));
        HELD_INTERVALS.map(|interval| {
            i64::from(delivered(0x2000, steady_under(interval, ARMS_052[a])))
                .saturating_mul(1_000)
                .checked_div(i64::from(at))
                .unwrap_or(0) as u32
        })
    })
}

/// The arithmetic brief 052 computes on the settled image before any run (`arithmetic_052`): the
/// prior's excitatory basal synapses onto the members, from the rest and from the members, and the
/// sums of their weights; and the slow potential's levels by the slow rule at the grid's shifts.
type Arithmetic052 = ([(u32, i64); 2], Levels052);

fn arithmetic_052(settled: &[u8]) -> Arithmetic052 {
    let exec = frozen_from(settled, 1024);
    let (rest, from_members) = prior_onto_members(&exec);
    let census = |w: &[i16]| {
        (
            w.len() as u32,
            w.iter().fold(0i64, |s, &x| s.saturating_add(i64::from(x))),
        )
    };
    (
        [census(&rest), census(&from_members)],
        slow_levels(&exec, &SHIFTS_052),
    )
}

// ------------------------------------------------ the runs (brief 052)

/// A run's slow reading (brief 052) by kind of span — the lead-in, the unkicked spans, the hold
/// spans' first halves, their second halves, the release spans and the tails — each the sums over
/// the members and the span's ticks of the slow potential and of the gate's opening; and every
/// window's as one number (FNV-1a over each sum's two halves).
type SlowRead = ([[u64; 2]; 6], u64);

fn slow_read(slow: &[[u64; 2]], spans: &[(Span, usize, u32)]) -> SlowRead {
    let mut by = [[0u64; 2]; 6];
    let mut at = 0usize;
    for &(span, _, epochs) in spans {
        let n = epochs.saturating_mul(WINDOWS as u32) as usize;
        for (i, window) in slow
            .get(at..at.saturating_add(n))
            .unwrap_or(&[])
            .iter()
            .enumerate()
        {
            let first_half =
                (i.checked_div(STRETCH_WINDOWS).unwrap_or(0) as u32) < SECOND_HALF_FROM;
            let kind = match span {
                Span::LeadIn => 0,
                Span::Unkicked => 1,
                Span::Hold if first_half => 2,
                Span::Hold => 3,
                Span::Release => 4,
                Span::Tail => 5,
            };
            for (sum, &v) in by[kind].iter_mut().zip(window.iter()) {
                *sum = sum.saturating_add(v);
            }
        }
        at = at.saturating_add(n);
    }
    let words: Vec<i32> = slow
        .iter()
        .flat_map(|w| {
            w.iter()
                .flat_map(|&v| [v as u32 as i32, (v >> 32) as u32 as i32])
        })
        .collect();
    (by, fnv1a_64(&words))
}

/// A run of brief 052 as pinned: ADR-0117's pin of it, and its slow reading.
#[derive(Clone, Copy, Debug)]
struct Pinned052 {
    rows: &'static [StretchRow],
    kicks: &'static [[u32; 2]],
    windows: u64,
    epochs: u64,
    slow: SlowRead,
}

impl Pinned052 {
    fn as_050(&self) -> Pinned050 {
        Pinned050 {
            rows: self.rows,
            kicks: self.kicks,
            windows: self.windows,
            epochs: self.epochs,
        }
    }
}

/// Holds a run of brief 052 to its pin: ADR-0117's `held_to`, and the slow reading.
fn held_052(run: &Run050, pin: &Pinned052, background: bool, name: &str) {
    run.held_to(&pin.as_050(), background, name);
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    assert_eq!(
        slow_read(&run.run.slow, &spans),
        pin.slow,
        "{name}: the slow potential and the gate"
    );
}

/// The slow potential and the gate over the held stretches of a run's hold spans' second halves,
/// held against `bg` (`held_stretch`): their sums over the members and the stretches' ticks, and
/// the stretches.
type SlowHeld = ([u64; 2], u32);

fn slow_held(run: &Run050, bg: Background) -> SlowHeld {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let mut out = ([0u64; 2], 0u32);
    for ((row, &(span, _, k)), windows) in run
        .rows
        .iter()
        .zip(stretches_of(&spans).iter())
        .zip(run.run.slow.chunks(STRETCH_WINDOWS))
    {
        if span == Span::Hold && k >= SECOND_HALF_FROM && held_stretch(row, bg) {
            for w in windows {
                for (sum, &v) in out.0.iter_mut().zip(w.iter()) {
                    *sum = sum.saturating_add(v);
                }
            }
            out.1 = out.1.saturating_add(1);
        }
    }
    out
}

/// A run of brief 052 under arm `arm` at input shift `shift` on `image`, dumped: a background (not
/// grown, no kick, no release), a control (grown, unwired, the kick and the release) or a cell
/// (wired at `weight` with the delays of `seed`); with `stimulus`, the task's stimulus at every
/// epoch's middle (condition (d)). Every weight of the arena at its end is its value at the start.
fn run_052(
    image: &[u8],
    (arm, shift): (usize, u8),
    weight: Option<i16>,
    background: bool,
    seed: u64,
    stimulus: bool,
    name: &str,
) -> Run050 {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let mut exec = engine_052(image, arm, shift, !background, weight, seed);
    let m = members(SIZE_052);
    let before = weights_of(&exec);
    let task = stimulus.then(stimulus_task);
    let (run, epochs) = span_protocol_under(&mut exec, &m, &spans, !background, task.as_ref());
    let rows = stretch_rows(&run.windows, SIZE_052);
    eprintln!("DUMP {name} stretches = {rows:?}");
    eprintln!("DUMP {name} kicks = {:?}", run.kicks);
    eprintln!(
        "DUMP {name} hashes = {:#018x} {:#018x}",
        windows_hash(&run.windows),
        epochs_hash(&epochs)
    );
    eprintln!("DUMP {name} slow = {:?}", slow_read(&run.slow, &spans));
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    Run050 { run, rows, epochs }
}

/// A cell of brief 052 read by the rules: its holding, its bursts, and the slow potential and the
/// gate over its held stretches.
type CellRead052 = (Holding, BurstRead049, SlowHeld);

/// Reads a run of arm `arm` against `bg` and dumps the reading.
fn cell_read_052(run: &Run050, arm: usize, bg: Background, name: &str) -> CellRead052 {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let read = holding(&run.rows, &spans, bg);
    let bursts = bursts_049(&run.run.windows, &spans, SIZE_052, bg, class_052(arm));
    let held = slow_held(run, bg);
    eprintln!(
        "DUMP {name} read = {read:?} usable {} failed {:?} kick {:?} bursts = {bursts:?} held = {held:?}",
        read.usable(),
        failed(&read),
        kick_reading_049(&run.run.kicks, SIZE_052)
    );
    (read, bursts, held)
}

/// The backgrounds and the controls of arm `arm` at every shift on `image`: for each shift the
/// background and the control, each dumped with its readings; then each control's lead-in and first
/// unkicked span shown to be its background's, the slow readings with them. The kick is read on the
/// engine by ADR-0112's measure and held as the pinned reading, which is that it fails at every arm
/// and shift, so that no cell runs (ADR-0124).
fn backgrounds_controls_052(
    image: &[u8],
    arm: usize,
    name: &str,
) -> Vec<(Run050, Run050, BackgroundControl)> {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let mut out = Vec::new();
    for &g in &SHIFTS_052 {
        let label = |kind: &str| format!("{name} {kind} {} at {g}", ARM_NAMES_052[arm]);
        let bg = run_052(
            image,
            (arm, g),
            None,
            true,
            DELAY_SEED,
            false,
            &label("background"),
        );
        let ctl = run_052(
            image,
            (arm, g),
            None,
            false,
            DELAY_SEED,
            false,
            &label("control"),
        );
        let background = background_049(&bg.rows, &spans);
        let (control, bursts, _) = cell_read_052(&ctl, arm, background, &label("control"));
        let kick = kick_reading_049(&ctl.run.kicks, SIZE_052);
        eprintln!(
            "DUMP {}: background {background:?} kick {kick:?} once {}",
            label("pair"),
            kicked_once_049(&ctl.run.kicks, SIZE_052)
        );
        out.push((bg, ctl, (background, kick, control, bursts)));
    }
    for (&g, (bg, ctl, _)) in SHIFTS_052.iter().zip(&out) {
        assert_eq!(
            ctl.rows[..first],
            bg.rows[..first],
            "{name} at {g}: the growth changes nothing unkicked"
        );
        let windows = first.saturating_mul(STRETCH_WINDOWS);
        assert_eq!(
            ctl.run.slow[..windows],
            bg.run.slow[..windows],
            "{name} at {g}: nor the slow potential"
        );
    }
    out
}

/// Holds arm `arm`'s backgrounds and controls to their pins.
fn backgrounds_held_052(pairs: &[(Run050, Run050, BackgroundControl)], arm: usize, name: &str) {
    for (k, (bg, ctl, reading)) in pairs.iter().enumerate() {
        held_052(
            bg,
            &BG_RUNS_052[arm][k][0],
            true,
            &format!("{name}: the background at {k}"),
        );
        held_052(
            ctl,
            &BG_RUNS_052[arm][k][1],
            false,
            &format!("{name}: the control at {k}"),
        );
        assert_eq!(*reading, BG_READ_052[arm][k], "{name}: {k} read");
    }
}

/// ADR-0117's cell reproduced with nothing marked for the slow current, before any cell of brief
/// 052: 64 units at a quarter of the range under set (ii) in the core, run through ADR-0117's own
/// `run_050` and held to ADR-0117's pinned stretches, kicks and windows and read against ADR-0117's
/// background to ADR-0117's reading and bursts.
fn reproduce_050_in_the_core(image: &[u8], name: &str) {
    let (s, w) = CELL_050;
    let size = SIZES_050[s];
    let label = format!("{name}: ADR-0117's cell, nothing marked for the slow current");
    let run = run_050(
        image,
        size,
        Some(WEIGHTS_050[w]),
        false,
        DELAY_SEED,
        false,
        &label,
    );
    assert!(
        run.run.slow.is_empty(),
        "{label}: no slow current, no reading"
    );
    let cell = cell_read(&run, size, CORE_READ_050[s].0, &label);
    run.held_to(&GRID_RUNS_050[s][w], false, &label);
    assert_eq!(
        cell,
        (GRID_050[s][w], GRID_BURSTS_050[s][w]),
        "{label}: ADR-0117's reading"
    );
}

/// The core's backgrounds and controls of arm `arm` (brief 052): on the settled image, the
/// arithmetic first, held to its pin before any run; under arm (A) ADR-0117's cell reproduced with
/// nothing marked for the slow current; then at each shift the background and the control, the kick
/// read on the engine and held to its reading.
fn core_backgrounds_052(arm: usize) {
    let name = format!(
        "brief 052's backgrounds and controls of the core under {}",
        ARM_NAMES_052[arm]
    );
    let image = settled(&name);
    let arithmetic = arithmetic_052(&image);
    eprintln!("DUMP ARITHMETIC_052 = {arithmetic:?}");
    assert_eq!(
        arithmetic, ARITHMETIC_052,
        "{name}: the arithmetic, pinned before any run"
    );
    assert_eq!(
        shift_placed(&arithmetic.1, &SHIFTS_052, SLOW_CRITICAL_052),
        Some(SHIFT_PLACED_052),
        "{name}: the shifts the rule places"
    );
    if arm == 0 {
        reproduce_050_in_the_core(&image, &name);
    }
    let pairs = backgrounds_controls_052(&image, arm, &name);
    backgrounds_held_052(&pairs, arm, &name);
}

// ------------------------------------------------ the gate (brief 052)

/// The gate's assembly kicked with the slow current: arm (B) at the heaviest weight and the smallest
/// shift, the most the slow potential can take from the kick's volley.
const GATE_CELL_052: Cell052 = (1, 0, 3);

#[test]
fn a_slow_current_the_arms_the_grid_the_arithmetic_the_rules_and_a_marked_assembly_kicked_with_it()
{
    // The grid: ADR-0117's cell's size, four weights, three distinct shifts the rule resolves.
    assert_eq!(SIZES_050[CELL_050.0], SIZE_052);
    assert_eq!(fan(SIZE_052), FAN_MAX);
    assert!(WEIGHTS_052.windows(2).all(|w| w[0] < w[1]));
    assert!(SHIFTS_052.windows(2).all(|w| w[0] < w[1]));
    assert!(SHIFTS_052.iter().all(|&g| current_052(g).is_valid()));
    assert_eq!(
        WEIGHTS_052[1], WEIGHTS_050[CELL_050.1],
        "ADR-0117's weight among them"
    );
    assert_eq!(ARMS_052, [None, Some(SETS[SET_050])]);
    // The gate's voltages: the drive's mean standing as the oracle reads it, and the base.
    assert_eq!(drive_standing().1, V_LO_052);
    assert_eq!(V_HI_052, THRESHOLD_BASE);
    // The release stands for both arms: its silence outlasts set (ii)'s fade (ADR-0115), and sixteen
    // of the slow potential's time constants; under ADR-0019's constants no silence leaves a member
    // releasing below the unkicked product, since a kick never primes one (ADR-0113), so there is
    // nothing of it to fade.
    let silence = silence_of(RELEASE_STRETCHES_049[SET_050]);
    let faded = fade(SETS[SET_050]).expect("set (ii)'s fade");
    eprintln!("DUMP RELEASE_052 = {silence} {faded}");
    assert!(silence >= u64::from(faded));
    assert_eq!(
        fade(StpClass::REFERENCE),
        None,
        "ADR-0019's constants never prime"
    );
    assert!(silence >= 16u64 << LEAK_SHIFT_052);
    // The arithmetic the settled image is not needed for, dumped, then held.
    let critical = slow_critical();
    let openings = gate_openings();
    let per_spike = per_spike_052();
    eprintln!("DUMP SLOW_CRITICAL_052 = {critical}");
    eprintln!("DUMP GATE_052 = {openings:?}");
    eprintln!("DUMP PER_SPIKE_052 = {per_spike:?}");
    assert_eq!(critical, SLOW_CRITICAL_052);
    assert_eq!(openings, GATE_052);
    assert_eq!(per_spike, PER_SPIKE_052);
    // The shifts, placed by the rule over the arithmetic on the settled image (pinned there before any
    // run): the largest shift at which arm (A) at ADR-0117's weight reaches the critical level at 20
    // Hz is 1, and one step either side. A larger shift only shrinks the input, so none above the
    // grid's reaches it either: at shift 2 the level is already below.
    let placed = shift_placed(&ARITHMETIC_052.1, &SHIFTS_052, SLOW_CRITICAL_052);
    assert_eq!(placed, Some(SHIFT_PLACED_052));
    assert_eq!(
        SHIFTS_052,
        [SHIFT_PLACED_052 - 1, SHIFT_PLACED_052, SHIFT_PLACED_052 + 1]
    );
    assert!(ARITHMETIC_052.1[0][1][2][4] < SLOW_CRITICAL_052);
    // The marks on the instrument's network.
    slow_marked_on_the_prior();
    // The rules at their edges, over tables written by hand.
    rules_052_at_their_edges();
    // A marked assembly with the slow current, kicked for a few hundred ticks.
    let (a, k, w) = GATE_CELL_052;
    let (kicked, exec) = kicked_on_keeping(
        engine_052(
            &prior_image(),
            a,
            SHIFTS_052[k],
            true,
            Some(WEIGHTS_052[w]),
            DELAY_SEED,
        ),
        SET_050,
        SIZE_052,
    );
    let slow: Vec<i32> = members(SIZE_052)
        .iter()
        .map(|&m| exec.units()[m as usize].v_slow)
        .collect();
    eprintln!("DUMP GATE_KICKED_052 = {kicked:?}");
    eprintln!("DUMP GATE_SLOW_052 = {slow:?}");
    assert!(
        slow.iter().all(|&s| s > 0),
        "the volley charged every member's slow potential"
    );
    assert_eq!(kicked.as_slice(), GATE_KICKED_052);
    assert_eq!(slow.as_slice(), GATE_SLOW_052);
    // The rules over the pinned tables.
    over_the_052_tables();
}

/// The members' spikes the background gives the pair window after a kick's span, over a control's
/// kicks: its members' spikes per tick times the window times the kicks, floored.
fn background_after(background: Background, kicks: u64) -> u64 {
    background
        .members
        .saturating_mul(u64::from(PAIR_WINDOW))
        .saturating_mul(kicks)
        .checked_div(background.ticks)
        .unwrap_or(0)
}

/// The rules over brief 052's pinned tables: each background's and control's stretches as the rules
/// read them; each control's lead-in and first unkicked span its background's, the lead-in's slow
/// reading with them; and the reading ADR-0124 gives. At every arm and shift the kick's volley is a
/// full one and its second clause fails, so the kick does not fire every member once by ADR-0112's
/// measure and no cell runs: at shifts 0 and 1 the members' own background alone gives the pair
/// window after the span more than the clause's mark, and at shift 2 the control's members fire
/// there more than both the mark and their background.
fn over_the_052_tables() {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let n = stretches_of(&spans).len();
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let mut after = [[(0u64, 0u64); 3]; 2];
    for (a, runs) in BG_RUNS_052.iter().enumerate() {
        for (k, [bg, ctl]) in runs.iter().enumerate() {
            assert_eq!((bg.rows.len(), ctl.rows.len()), (n, n));
            let background = background_049(bg.rows, &spans);
            let read = BG_READ_052[a][k];
            assert_eq!(background, read.0);
            let kick = kick_reading_049(ctl.kicks, SIZE_052);
            assert_eq!(kick, read.1);
            assert_eq!(holding(ctl.rows, &spans, background), read.2);
            assert_eq!(
                ctl.rows[..first],
                bg.rows[..first],
                "the growth changes nothing unkicked"
            );
            assert_eq!(
                bg.slow.0[0], ctl.slow.0[0],
                "nor the lead-in's slow potential"
            );
            let (volley, spikes_after, kicks, _) = kick;
            let whole = u64::from(SIZE_052).saturating_mul(kicks);
            assert!(
                volley <= whole
                    && volley.saturating_mul(10)
                        >= whole
                            .saturating_mul(10)
                            .saturating_sub(VOLLEY_TOLERANCE_TENTHS.saturating_mul(kicks)),
                "{} at {k}: a full volley",
                ARM_NAMES_052[a]
            );
            assert!(
                !kicked_once_049(ctl.kicks, SIZE_052),
                "{} at {k}: the kick fails ADR-0112's measure",
                ARM_NAMES_052[a]
            );
            after[a][k] = (background_after(background, kicks), spikes_after);
        }
    }
    assert_eq!(after, KICK_AFTER_052, "the pair window after the kick");
    let mark = u64::from(SIZE_052)
        .saturating_mul(ROUNDS as u64)
        .saturating_mul(AFTER_MAX_TENTHS)
        .checked_div(10)
        .unwrap_or(0);
    for by_shift in &after {
        for (k, &(expected, spikes)) in by_shift.iter().enumerate() {
            assert!(spikes > mark, "the second clause fails");
            if k < 2 {
                assert!(expected > mark, "the background alone passes the mark");
            } else {
                assert!(expected <= mark && spikes > expected);
            }
        }
    }
}

/// The marks on the instrument's network: under each arm at each shift the image decodes with the
/// slow current and the class, the members marked and no other unit, and the prior's blocks as they
/// were; with the slow current unset in the record a member's mark is refused; under the drive,
/// unwired, a member's slow potential rises from the prior's synapses and every other unit's stays
/// zero.
fn slow_marked_on_the_prior() {
    let image = prior_image();
    let plain = frozen_from(&image, 1024);
    for arm in 0..ARMS_052.len() {
        for &g in &SHIFTS_052 {
            let exec = engine_052(&image, arm, g, true, None, DELAY_SEED);
            assert_eq!(&exec.blocks()[..plain.blocks().len()], plain.blocks());
        }
    }
    let mut unset = slow_marked(&image, 0, SHIFTS_052[0]);
    let header = MenteionFileHeader::decode(unset[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(unset[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_MODULATOR {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            unset[offset..][36..48].copy_from_slice(&[0; 12]);
            entry.crc64 = crc64(&unset[offset..][..length]);
            unset[at..][..64].copy_from_slice(&entry.encode());
        }
    }
    assert!(matches!(
        Image::decode::<2048>(&unset, config(1024, 2, 0)),
        Err(menteion_runtime::ImageError::MarkWithoutSlowCurrent(5))
    ));
    let mut exec = engine_052(&image, 0, SHIFTS_052[0], false, None, DELAY_SEED);
    let drive = drive(1024);
    let until = exec.ticks().saturating_add(u64::from(GROWTH_TICKS) * 4);
    run_driven(&mut exec, &drive, until).expect("the drive runs");
    let of = membership(&members(SIZE_052));
    let charged = exec
        .units()
        .iter()
        .filter(|u| {
            assert!(of[u.id as usize] || u.v_slow == 0, "unit {} unmarked", u.id);
            assert!(u.v_slow >= 0, "unit {}: the input is excitatory", u.id);
            u.v_slow > 0
        })
        .count();
    assert!(charged > 32, "the prior charged the members: {charged}");
}

/// The rules of brief 052 at their edges, over tables written by hand: the cells the conditions
/// run and their cut at eight, the neighbours within an arm, robust, the second round's cell and
/// its ties, what failed; and the slow reading by kind of span.
fn rules_052_at_their_edges() {
    let usable = Holding {
        first_half: 8,
        held: 8,
        ignited: 0,
        let_go: 8,
        spills: Some(false),
    };
    let never = Holding { held: 6, ..usable };
    let runs = Holding {
        ignited: 2,
        ..usable
    };
    let stays = Holding {
        let_go: 6,
        ..usable
    };
    let spills = Holding {
        spills: Some(true),
        ..usable
    };
    assert!(usable.usable() && !never.usable() && !runs.usable() && !stays.usable());
    assert!(!spills.usable());
    // Nothing usable: no cell for the conditions, none robust, no second round.
    let mut core: Grid052 = [[[never; 4]; 3]; 2];
    assert!(condition_cells_052(&core).is_empty());
    assert_eq!(round_two_052(&core, &[]), None);
    // Up to eight usable: every one, in the grid's order.
    for &(a, s, w) in &[(1, 2, 0), (0, 1, 3), (0, 1, 2)] {
        core[a][s][w] = usable;
    }
    assert_eq!(
        condition_cells_052(&core),
        [(0, 1, 2), (0, 1, 3), (1, 2, 0)]
    );
    // The neighbours: within an arm, one weight step and one shift step either way; a diagonal
    // is not one, nor the same cell in the other arm.
    assert_eq!(neighbours_052(&core, (0, 1, 2)), 1);
    assert_eq!(
        neighbours_052(&core, (1, 1, 3)),
        0,
        "a diagonal, and the other arm"
    );
    assert_eq!(neighbours_052(&core, (0, 2, 3)), 1, "one shift step");
    // Ten usable: the eight with the most usable neighbours, ties to the lighter weight, then the
    // smaller shift, then arm (A).
    let mut core: Grid052 = [[[never; 4]; 3]; 2];
    let ten = [
        (0, 0, 0),
        (0, 0, 1),
        (0, 0, 2),
        (0, 1, 1),
        (0, 2, 3),
        (1, 0, 3),
        (1, 1, 0),
        (1, 1, 1),
        (1, 1, 2),
        (1, 2, 1),
    ];
    for &(a, s, w) in &ten {
        core[a][s][w] = usable;
    }
    assert_eq!(
        ten.map(|c| neighbours_052(&core, c)),
        [1, 3, 1, 1, 0, 0, 1, 3, 1, 1]
    );
    // By neighbours alone, no tie at the cut: (0,0,1) and (1,1,1) with three and the six with one
    // are eight; (0,2,3) and (1,0,3), with none, are cut.
    assert_eq!(
        condition_cells_052(&core),
        [
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (0, 1, 1),
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, 2),
            (1, 2, 1)
        ]
    );
    // A tie at the cut: a row of four at shift 0 in arm (A), with one, two, two and one usable
    // neighbours, and one of three in arm (B), with one, two and one, are seven; three cells with
    // none tie for the eighth place, and it goes to the lighter weight, (0,2,0).
    let mut cut: Grid052 = [[[never; 4]; 3]; 2];
    let row = [
        (0, 0, 0),
        (0, 0, 1),
        (0, 0, 2),
        (0, 0, 3),
        (1, 0, 0),
        (1, 0, 1),
        (1, 0, 2),
    ];
    let alone = [(0, 2, 0), (0, 2, 2), (1, 2, 3)];
    for &(a, s, w) in row.iter().chain(alone.iter()) {
        cut[a][s][w] = usable;
    }
    assert_eq!(row.map(|c| neighbours_052(&cut, c)), [1, 2, 2, 1, 1, 2, 1]);
    assert_eq!(alone.map(|c| neighbours_052(&cut, c)), [0, 0, 0]);
    assert_eq!(
        condition_cells_052(&cut),
        [
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (0, 0, 3),
            (0, 2, 0),
            (1, 0, 0),
            (1, 0, 1),
            (1, 0, 2)
        ]
    );
    // Robust: usable under every one of the three conditions.
    let cells = [(0, 0, 1), (1, 1, 1)];
    let all = [
        vec![usable, usable],
        vec![usable, usable],
        vec![usable, usable],
    ];
    assert_eq!(robust_052(&cells, &all), cells);
    for c in 0..3 {
        let mut under = all.clone();
        under[c][0] = runs;
        assert_eq!(
            robust_052(&cells, &under),
            [(1, 1, 1)],
            "condition {c} fails one"
        );
    }
    let short = [vec![usable, usable], vec![usable, usable], vec![usable]];
    assert_eq!(
        robust_052(&cells, &short),
        [(0, 0, 1)],
        "a reading not made is no reading"
    );
    // The second round's cell: the most usable neighbours, then the lighter weight, then the
    // smaller shift, then arm (A).
    assert_eq!(
        round_two_052(&core, &[(0, 0, 0), (1, 1, 1)]),
        Some((1, 1, 1))
    );
    assert_eq!(
        round_two_052(&core, &[(0, 0, 2), (1, 1, 0)]),
        Some((1, 1, 0)),
        "a tie to the lighter weight"
    );
    assert_eq!(
        round_two_052(&core, &[(1, 1, 1), (0, 0, 1)]),
        Some((0, 0, 1)),
        "a tie to the smaller shift"
    );
    assert_eq!(
        round_two_052(&cut, &[(1, 0, 0), (0, 0, 0)]),
        Some((0, 0, 0)),
        "a tie to arm (A)"
    );
    // What failed, per condition.
    let under = [
        vec![usable, runs],
        vec![stays, usable],
        vec![usable, spills],
    ];
    let mut failing = core;
    failing[0][0][1] = never;
    assert_eq!(
        failures_052(&failing, &cells, &under),
        [
            [
                [true, false, false],
                [false, false, false],
                [false, false, true],
                [false, false, false]
            ],
            [
                [false, false, false],
                [false, true, false],
                [false, false, false],
                [false, true, false]
            ]
        ]
    );
    // The slow reading by kind of span: sixteen lead-in epochs, eight rounds of sixteen unkicked
    // epochs, eight and eight of the hold's halves, twelve of the release and four of the tail, of
    // eight windows each.
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let windows: Vec<[u64; 2]> = (0..RUN_EPOCHS_049[SET_050].saturating_mul(WINDOWS as u32))
        .map(|_| [1, 2])
        .collect();
    let (by, _) = slow_read(&windows, &spans);
    assert_eq!(
        by.map(|[s, _]| s),
        [128, 1_024, 512, 512, 768, 256],
        "the windows of each kind"
    );
    assert!(by.iter().all(|&[s, g]| g == s.saturating_mul(2)));
}

// ------------------------------------------------ the runs, weekly (brief 052)

#[test]
#[ignore]
fn the_slow_current_alone_the_backgrounds_and_the_controls_of_the_core_paused() {
    core_backgrounds_052(0);
}

#[test]
#[ignore]
fn the_slow_current_with_the_facilitating_class_the_backgrounds_and_the_controls_of_the_core_paused()
 {
    core_backgrounds_052(1);
}

// ------------------------------------------------ brief 052's arithmetic, pinned before any run

/// The critical slow potential (`slow_critical`): 1.138 of the threshold, where the soma's steady
/// rise under an open gate, $s \cdot 128 / 257$, meets the gap of 0.564.
const SLOW_CRITICAL_052: i32 = 74_589;

/// The gate's opening at the drive's mean standing, a quarter, a half and three quarters of the gap
/// above it, at 0.9 and at the threshold's base (`gate_openings`).
const GATE_052: [i32; 6] = [0, 16_382, 32_767, 49_153, 53_919, 65_536];

/// The per-spike release of the held state over the quiet one, per mille, `[arm][5, 10, 20 Hz]`
/// (`per_spike_052`): under ADR-0019's constants the synapse depresses, under set (ii) it
/// facilitates by the 1.4 to 1.6 the four rounds before were bounded by.
const PER_SPIKE_052: [[u32; 3]; 2] = [[809, 601, 380], [1_453, 1_708, 1_505]];

/// The shift rule's reading: the largest shift at which arm (A) at ADR-0117's weight holds the
/// critical level at 20 Hz.
const SHIFT_PLACED_052: u8 = 1;

/// The shift the rule places over a table of levels: the largest of `shifts` at which arm (A) at
/// ADR-0117's weight reads the critical level at 20 Hz; none when no shift does.
fn shift_placed(levels: &Levels052, shifts: &[u8; 3], critical: i32) -> Option<u8> {
    let w = WEIGHTS_052
        .iter()
        .position(|&x| x == WEIGHTS_050[CELL_050.1])
        .expect("ADR-0117's weight");
    shifts
        .iter()
        .zip(levels[0][w].iter())
        .filter(|(_, l)| l[4] >= critical)
        .map(|(&g, _)| g)
        .max()
}

// ------------------------------------------------ brief 052's readings, pinned from the runs

// ================================================ brief 053 (ADR-0126): the gate raised, measured

// ------------------------------------------------ the voltages and the grid (brief 053), before any run

/// The three lower gate voltages (ADR-0125): 0.7, 0.8 and 0.9 of the threshold's base, floored in
/// Q16.16, with $V_{hi}$ at the base. Every other slow constant is ADR-0124's: $\tau_s = 2^{13}$
/// ticks and the input shifts `SHIFTS_052`.
const V_LOS_053: [i32; 3] = [45_875, 52_428, 58_982];

const V_LO_NAMES_053: [&str; 3] = ["0.7", "0.8", "0.9"];

/// The slow current at input shift `shift` with its gate from `v_lo` to the threshold's base,
/// ADR-0124's leak: at ADR-0124's voltage, `current_052` itself.
const fn current_053(shift: u8, v_lo: i32) -> SlowCurrent {
    SlowCurrent {
        leak_shift: LEAK_SHIFT_052,
        input_shift: shift,
        v_lo_q16: v_lo,
        v_hi_q16: V_HI_052,
    }
}

/// A pair of brief 053 by index, `(arm, V_lo, shift)` over `ARMS_052`, `V_LOS_053` and
/// `SHIFTS_052`: a background and a control, as ADR-0124's.
type Pair053 = (usize, usize, usize);

/// The eighteen pairs read by the rules, `[arm][V_lo][shift]`: each background, the kick's
/// reading on its control, the control's holding and its bursts.
type Read053 = [[[BackgroundControl; 3]; 3]; 2];

/// The most pairs whose cells run (ADR-0125).
const CELL_PAIRS_MAX: usize = 4;

/// A cell of brief 053: a pair whose cells ran, and a weight by index into `WEIGHTS_052`.
type Cell053 = (Pair053, usize);

/// The cells that ran: each pair the rule picked, in the grid's order, with its four weights read
/// by ADR-0117's rules against the pair's background.
type Cells053 = [(Pair053, [Holding; 4])];

// ------------------------------------------------ the rules (brief 053), before any run

/// Whether a pair's kick passes: ADR-0112's measure (`fires_every_member_once`) over the kick's
/// reading on its control — a full volley, and at most a tenth of a spike a member a kick in the
/// pair window after the span.
fn passes_053(read: &BackgroundControl) -> bool {
    fires_every_member_once(read.1, SIZE_052)
}

/// The pairs whose cells run (ADR-0125): every pair whose kick passes; when more than
/// `CELL_PAIRS_MAX` pass, the four whose background's members fire slowest, ties to the higher
/// $V_{lo}$, then the larger shift, then arm (A). In the grid's order.
fn cell_pairs_053(read: &Read053) -> Vec<Pair053> {
    let mut passing = Vec::new();
    for (a, by_v) in read.iter().enumerate() {
        for (v, by_shift) in by_v.iter().enumerate() {
            for (k, pair) in by_shift.iter().enumerate() {
                if passes_053(pair) {
                    passing.push((a, v, k));
                }
            }
        }
    }
    if passing.len() > CELL_PAIRS_MAX {
        let background = |&(a, v, k): &Pair053| read[a][v][k].0;
        passing.sort_by(|x, y| {
            let (bx, by) = (background(x), background(y));
            bx.members
                .saturating_mul(by.ticks)
                .cmp(&by.members.saturating_mul(bx.ticks))
                .then(y.1.cmp(&x.1))
                .then(y.2.cmp(&x.2))
                .then(x.0.cmp(&y.0))
        });
        passing.truncate(CELL_PAIRS_MAX);
        passing.sort_unstable();
    }
    passing
}

/// Whether the cell at `pair` and weight `w` ran and is usable.
fn usable_at(cells: &Cells053, pair: Pair053, w: usize) -> bool {
    cells
        .iter()
        .any(|(p, row)| *p == pair && row.get(w).is_some_and(Holding::usable))
}

/// A cell's usable neighbours among the cells that ran, within its arm and $V_{lo}$: of the cells
/// one weight step either way at its shift and one shift step either way at its weight, at most
/// four, those usable; a cell that did not run is not usable.
fn neighbours_053(cells: &Cells053, ((a, v, k), w): Cell053) -> u32 {
    let near = [
        (k.checked_sub(1), Some(w)),
        (k.checked_add(1), Some(w)),
        (Some(k), w.checked_sub(1)),
        (Some(k), w.checked_add(1)),
    ];
    let mut n = 0u32;
    for pair in near {
        if let (Some(nk), Some(nw)) = pair {
            n = n.saturating_add(u32::from(usable_at(cells, (a, v, nk), nw)));
        }
    }
    n
}

/// The usable cells, in the grid's order.
fn usable_053(cells: &Cells053) -> Vec<Cell053> {
    let mut out = Vec::new();
    for &(pair, row) in cells {
        for (w, h) in row.iter().enumerate() {
            if h.usable() {
                out.push((pair, w));
            }
        }
    }
    out.sort_unstable();
    out
}

/// The order ties among cells go by: ADR-0124's — the lighter weight, then the smaller shift —
/// then the higher $V_{lo}$, then arm (A).
fn tie_key_053(((a, v, k), w): Cell053) -> (usize, usize, core::cmp::Reverse<usize>, usize) {
    (w, k, core::cmp::Reverse(v), a)
}

/// The cells the conditions would run in the next round (ADR-0125's case 3, as ADR-0124's rule
/// picks them): every usable cell; when more than `CONDITION_CELLS_MAX` are, those with the most
/// usable neighbours, ties by `tie_key_053`. In the grid's order.
fn condition_cells_053(cells: &Cells053) -> Vec<Cell053> {
    let mut out = usable_053(cells);
    if out.len() > CONDITION_CELLS_MAX {
        out.sort_by_key(|&c| (core::cmp::Reverse(neighbours_053(cells, c)), tie_key_053(c)));
        out.truncate(CONDITION_CELLS_MAX);
        out.sort_unstable();
    }
    out
}

/// The case of ADR-0125's rule that this round's readings reach. Its fourth case, a robust cell,
/// needs the conditions' round.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Case053 {
    /// Case 1: no kick passes at any arm, shift and $V_{lo}$; the line pauses.
    NoKick,
    /// Case 2: cells ran and none is usable in the core; the line pauses.
    NoneUsable,
    /// Case 3: some cells are usable; the next round runs ADR-0117's conditions (b), (c) and (d)
    /// on these, at most eight.
    Conditions(Vec<Cell053>),
}

impl Case053 {
    /// Whether the case pauses the line.
    fn pauses(&self) -> bool {
        !matches!(self, Case053::Conditions(_))
    }
}

/// ADR-0125's rule over the pairs' reading and the cells that ran, which are the pairs the rule
/// picks (asserted): no pair picked, case 1; cells and none usable, case 2; else case 3 with the
/// cells the conditions would run.
fn case_053(read: &Read053, cells: &Cells053) -> Case053 {
    let picked = cell_pairs_053(read);
    assert!(
        cells.iter().map(|c| c.0).eq(picked.iter().copied()),
        "the cells that ran are the pairs the rule picks"
    );
    if picked.is_empty() {
        return Case053::NoKick;
    }
    let chosen = condition_cells_053(cells);
    if chosen.is_empty() {
        Case053::NoneUsable
    } else {
        Case053::Conditions(chosen)
    }
}

// ------------------------------------------------ the arithmetic (brief 053), before any run

/// The highest slow potential brief 053's critical levels are searched below: 16.0. At a lower
/// voltage of 0.9 the soma starts an eighth of a span of 0.1 above it, where the gate is an eighth
/// open and the soma's pull back toward the drive's standing is nearly the whole gap, so the
/// critical level is several times ADR-0124's.
const CRITICAL_MAX_053: i32 = 16 * THRESHOLD_BASE;

/// The gate's opening at the drive's mean standing, at `v_lo`, a quarter, a half and three quarters
/// of the gate's span above it, and at the threshold's base.
fn gate_openings_053(v_lo: i32) -> [i32; 6] {
    let c = current_053(0, v_lo);
    let span = THRESHOLD_BASE.saturating_sub(v_lo);
    [
        DRIVE_STANDING.1,
        v_lo,
        v_lo.saturating_add(span >> 2),
        v_lo.saturating_add(span >> 1),
        v_lo.saturating_add(span.saturating_sub(span >> 2)),
        THRESHOLD_BASE,
    ]
    .map(|v| c.gate_q16(v))
}

/// The highest basal potential the effective threshold is searched below: 4.0, under which a unit's
/// soma stands at about 2.0 and fires with no slow current.
const BASAL_MAX_053: i32 = 4 * THRESHOLD_BASE;

/// Whether a unit whose basal potential and slow potential are held at `basal` and `level` —
/// written back before every tick — fires within `CRITICAL_TICKS` from rest, by the slow rule
/// itself with the gate from `v_lo`, and no other input. The soma rises toward its steady level
/// without overshoot, so it fires when that level, the slow current's share included, reaches the
/// threshold.
fn fires_standing(basal: i32, level: i32, v_lo: i32) -> bool {
    let c = current_053(0, v_lo);
    let mut unit = DendriticSuperNeuron::new(0);
    unit.flags = FLAG_SLOW;
    unit.v_thresh = THRESHOLD_BASE;
    (1..=CRITICAL_TICKS).any(|t| {
        unit.v_basal = basal;
        unit.v_slow = level;
        unit.integrate_slow(0, 0, 0, t, c)
    })
}

/// The somatic potential a unit settles at with its basal potential held at `basal` and no slow
/// current: a unit whose threshold is zero never fires (`integrate`), stepped from rest over
/// `STANDING_TICKS`.
fn soma_standing(basal: i32) -> i32 {
    let mut unit = DendriticSuperNeuron::new(0);
    for t in 1..=STANDING_TICKS {
        unit.v_basal = basal;
        unit.integrate(0, 0, t);
    }
    unit.v_soma
}

/// The effective threshold under a slow potential held at `level`, the gate from `v_lo`: the
/// soma's standing with no slow current under the least basal potential at which the unit fires
/// with it (`fires_standing`, by bisection over `[0, BASAL_MAX_053]`; a higher basal potential
/// never leaves the soma or the gate lower, so the predicate is monotone). A fluctuation that lifts
/// a quiet soma's standing this far fires the unit; with no slow potential it is the threshold's
/// base. Its distance above the drive's mean standing is ADR-0124's distance to the threshold.
fn effective_threshold(level: i32, v_lo: i32) -> i32 {
    soma_standing(least_firing(BASAL_MAX_053, |b| {
        fires_standing(b, level, v_lo)
    }))
}

/// The slow potentials the effective threshold is read at: none, and ADR-0124's six backgrounds'
/// means per member and tick over their unkicked spans, `(A)` then `(B)` at shifts 0, 1 and 2, from
/// its pinned slow readings (`BG_RUNS_052`).
fn background_levels_052() -> [i32; 7] {
    let ticks = u64::from(SIZE_052)
        .saturating_mul(ROUNDS as u64)
        .saturating_mul(u64::from(UNKICKED_EPOCHS))
        .saturating_mul(u64::from(EPOCH_TICKS));
    let mut out = [0i32; 7];
    for (level, runs) in out
        .iter_mut()
        .skip(1)
        .zip(BG_RUNS_052.iter().flat_map(|arm| arm.iter()))
    {
        *level = runs[0].slow.0[1][0].checked_div(ticks).unwrap_or(0) as i32;
    }
    out
}

/// The effective threshold at each of `levels`, `[level][V_lo]`, the voltages ADR-0124's and then
/// brief 053's three.
fn effective_053(levels: &[i32; 7]) -> [[i32; 4]; 7] {
    let voltages = [V_LO_052, V_LOS_053[0], V_LOS_053[1], V_LOS_053[2]];
    levels.map(|level| voltages.map(|v| effective_threshold(level, v)))
}

// ------------------------------------------------ the runs (brief 053)

/// A run of brief 053 as pinned: ADR-0124's pin of it without a task's epochs, and the soma's
/// reading.
#[derive(Clone, Copy, Debug)]
struct Pinned053 {
    rows: &'static [StretchRow],
    kicks: &'static [[u32; 2]],
    windows: u64,
    slow: SlowRead,
    soma: SomaRead,
}

/// Holds a run of brief 053 to its pin: ADR-0124's `held_052`, and the soma's reading.
fn held_053(run: &Run050, pin: &Pinned053, background: bool, name: &str) {
    let as_052 = Pinned052 {
        rows: pin.rows,
        kicks: pin.kicks,
        windows: pin.windows,
        epochs: NO_EPOCHS_HASH,
        slow: pin.slow,
    };
    held_052(run, &as_052, background, name);
    assert_eq!(run.run.soma, Some(pin.soma), "{name}: the soma");
}

/// A run of brief 053 under arm `arm` with the slow current `current` on `image`, dumped: a
/// background (not grown, no kick, no release), a control (grown, unwired, the kick and the
/// release) or a cell (wired at `weight` with ADR-0112's delays); the members' somatic potentials
/// read after the lead-in. Every weight of the arena at its end is its value at the start.
fn run_053(
    image: &[u8],
    arm: usize,
    current: SlowCurrent,
    weight: Option<i16>,
    background: bool,
    name: &str,
) -> Run050 {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let mut exec = engine_under(image, arm, current, !background, weight, DELAY_SEED);
    let m = members(SIZE_052);
    let before = weights_of(&exec);
    let (run, epochs) = span_protocol_reading(&mut exec, &m, &spans, !background, None, &[], true);
    let rows = stretch_rows(&run.windows, SIZE_052);
    eprintln!("DUMP {name} stretches = {rows:?}");
    eprintln!("DUMP {name} kicks = {:?}", run.kicks);
    eprintln!(
        "DUMP {name} hashes = {:#018x} {:#018x}",
        windows_hash(&run.windows),
        epochs_hash(&epochs)
    );
    eprintln!("DUMP {name} slow = {:?}", slow_read(&run.slow, &spans));
    eprintln!("DUMP {name} soma = {:?}", run.soma);
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    Run050 { run, rows, epochs }
}

/// The quiet soma's run (brief 053), under arm `arm`'s substrate with nothing of the slow current:
/// the settled image, frozen, not grown, the drive alone over ADR-0117's layout, the members'
/// somatic potentials read after the lead-in. Under (A) the members are marked for nothing, so
/// their synapses release under ADR-0019's constants; under (B) they are marked facilitating under
/// set (ii), which is ADR-0117's background itself. Every weight at its end is its value at the
/// start.
fn quiet_run_053(image: &[u8], arm: usize, name: &str) -> Run050 {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let mut exec = match ARMS_052[arm] {
        None => frozen_from(image, 1024),
        Some(_) => marked_engine_drawn(image, SET_050, SIZE_052, false, None, DELAY_SEED),
    };
    assert_eq!(
        (exec.slow_current(), exec.stp_class()),
        (None, ARMS_052[arm]),
        "{name}: no slow current, the arm's class"
    );
    let m = members(SIZE_052);
    let before = weights_of(&exec);
    let (run, epochs) = span_protocol_reading(&mut exec, &m, &spans, false, None, &[], true);
    let rows = stretch_rows(&run.windows, SIZE_052);
    eprintln!("DUMP {name} stretches = {rows:?}");
    eprintln!(
        "DUMP {name} hashes = {:#018x} {:#018x}",
        windows_hash(&run.windows),
        epochs_hash(&epochs)
    );
    eprintln!("DUMP {name} soma = {:?}", run.soma);
    assert!(run.slow.is_empty(), "{name}: no slow current, no reading");
    assert_eq!(weights_of(&exec), before, "{name}: no weight moved");
    Run050 { run, rows, epochs }
}

/// The quiet soma (brief 053): on the settled image, the slow potential's levels by the slow rule
/// held to ADR-0124's at each of the three voltages first; ADR-0117's cell reproduced with nothing
/// marked for the slow current; then the quiet soma's run under each arm's substrate, dumped, (B)'s
/// held to ADR-0117's background bit for bit, and both read.
fn quiet_soma_053() {
    let name = "brief 053's quiet soma";
    let image = settled(name);
    let exec = frozen_from(&image, 1024);
    for &v in &V_LOS_053 {
        assert_eq!(
            slow_levels_under(&exec, &SHIFTS_052, v),
            ARITHMETIC_052.1,
            "{name}: the slow potential's levels at {v}, ADR-0124's"
        );
    }
    reproduce_050_in_the_core(&image, name);
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let runs: Vec<Run050> = (0..ARMS_052.len())
        .map(|a| quiet_run_053(&image, a, &format!("{name} under {}", ARM_NAMES_052[a])))
        .collect();
    let reads: Vec<(Background, SomaRead)> = runs
        .iter()
        .map(|r| (background_049(&r.rows, &spans), r.run.soma.expect("read")))
        .collect();
    eprintln!("DUMP QUIET_READ_053 = {reads:?}");
    runs[0].held_to(&QUIET_053_A, true, &format!("{name} under (A)"));
    runs[1].held_to(
        &CORE_RUNS_050[SIZE_051_IN_050][0],
        true,
        &format!("{name} under (B): ADR-0117's background"),
    );
    assert_eq!(reads[1].0, CORE_READ_050[SIZE_051_IN_050].0);
    assert_eq!(reads.as_slice(), QUIET_READ_053, "{name}: the reading");
}

/// The backgrounds and the controls of arm `arm` at $V_{lo}$ `V_LOS_053[v]` and every shift on
/// `image`: for each shift the background and the control, each dumped with its readings; then each
/// control's lead-in and first unkicked span shown to be its background's, the slow readings with
/// them. The kick is read on the engine by ADR-0112's measure.
fn backgrounds_controls_053(
    image: &[u8],
    arm: usize,
    v: usize,
    name: &str,
) -> Vec<(Run050, Run050, BackgroundControl)> {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let mut out = Vec::new();
    for &g in &SHIFTS_052 {
        let current = current_053(g, V_LOS_053[v]);
        let label = |kind: &str| {
            format!(
                "{name} {kind} {} at {} at {g}",
                ARM_NAMES_052[arm], V_LO_NAMES_053[v]
            )
        };
        let bg = run_053(image, arm, current, None, true, &label("background"));
        let ctl = run_053(image, arm, current, None, false, &label("control"));
        let background = background_049(&bg.rows, &spans);
        let (control, bursts, _) = cell_read_052(&ctl, arm, background, &label("control"));
        let kick = kick_reading_049(&ctl.run.kicks, SIZE_052);
        eprintln!(
            "DUMP {}: background {background:?} kick {kick:?} once {}",
            label("pair"),
            kicked_once_049(&ctl.run.kicks, SIZE_052)
        );
        out.push((bg, ctl, (background, kick, control, bursts)));
    }
    for (&g, (bg, ctl, _)) in SHIFTS_052.iter().zip(&out) {
        assert_eq!(
            ctl.rows[..first],
            bg.rows[..first],
            "{name} at {g}: the growth changes nothing unkicked"
        );
        let windows = first.saturating_mul(STRETCH_WINDOWS);
        assert_eq!(
            ctl.run.slow[..windows],
            bg.run.slow[..windows],
            "{name} at {g}: nor the slow potential"
        );
    }
    out
}

/// The backgrounds and controls of arm `arm` at $V_{lo}$ `V_LOS_053[v]` (brief 053), on the
/// settled image, every run dumped before any is held to its pin.
fn pairs_053(arm: usize, v: usize) {
    let name = format!(
        "brief 053's backgrounds and controls under {} at {}",
        ARM_NAMES_052[arm], V_LO_NAMES_053[v]
    );
    let image = settled(&name);
    let pairs = backgrounds_controls_053(&image, arm, v, &name);
    for (k, (bg, ctl, reading)) in pairs.iter().enumerate() {
        held_053(
            bg,
            &BG_RUNS_053[arm][v][k][0],
            true,
            &format!("{name}: the background at {k}"),
        );
        held_053(
            ctl,
            &BG_RUNS_053[arm][v][k][1],
            false,
            &format!("{name}: the control at {k}"),
        );
        assert_eq!(*reading, BG_READ_053[arm][v][k], "{name}: {k} read");
    }
}

// ------------------------------------------------ the gate (brief 053)

/// The gate's assembly kicked at the highest voltage: ADR-0124's gate cell — arm (B), the smallest
/// shift, the heaviest weight — with the gate from 0.9.
const GATE_CELL_053: (usize, usize, usize, usize) = (1, 2, 0, 3);

#[test]
fn the_gate_raised_the_voltages_the_arithmetic_the_rules_and_a_marked_assembly_kicked_at_the_highest()
 {
    // The voltages: 0.7, 0.8 and 0.9 of the threshold's base, floored, above ADR-0124's; the
    // soma's levels 0.5 to 0.95, floored, the voltages among them.
    let base = i64::from(THRESHOLD_BASE);
    for (&level, twentieths) in SOMA_LEVELS_053.iter().zip([10i64, 12, 14, 16, 18, 19]) {
        assert_eq!(
            Some(i64::from(level)),
            base.saturating_mul(twentieths).checked_div(20)
        );
    }
    assert_eq!(SOMA_LEVELS_053[2..5], V_LOS_053);
    assert!(V_LO_052 < V_LOS_053[0] && V_LOS_053.windows(2).all(|w| w[0] < w[1]));
    for &v in &V_LOS_053 {
        assert!(SHIFTS_052.iter().all(|&g| current_053(g, v).is_valid()));
    }
    // ADR-0124's definitions at its own voltage, bit for bit: the current, and the critical level
    // searched over the wider range.
    for &g in &SHIFTS_052 {
        assert_eq!(current_053(g, V_LO_052), current_052(g));
    }
    assert_eq!(
        slow_critical_at(V_LO_052, CRITICAL_MAX_053),
        SLOW_CRITICAL_052
    );
    // The arithmetic, dumped, then held.
    let critical = V_LOS_053.map(|v| slow_critical_at(v, CRITICAL_MAX_053));
    let openings = V_LOS_053.map(gate_openings_053);
    let levels = background_levels_052();
    let effective = effective_053(&levels);
    eprintln!("DUMP SLOW_CRITICAL_053 = {critical:?}");
    eprintln!("DUMP GATE_053 = {openings:?}");
    eprintln!("DUMP LEVELS_053 = {levels:?}");
    eprintln!("DUMP EFFECTIVE_053 = {effective:?}");
    assert_eq!(critical, SLOW_CRITICAL_053);
    assert_eq!(openings, GATE_053);
    assert_eq!(levels, LEVELS_053);
    assert_eq!(effective, EFFECTIVE_053);
    // With no slow potential the effective threshold is the threshold's base, less the soma's
    // standing's floor, at every voltage; with one it is at or above the voltage and never above
    // the base.
    assert!(effective[0].iter().all(|&e| e == effective[0][0]));
    for row in &effective {
        for (&e, v) in row
            .iter()
            .zip([V_LO_052, V_LOS_053[0], V_LOS_053[1], V_LOS_053[2]])
        {
            assert!(e >= v && e <= effective[0][0]);
        }
    }
    // The soma's reading at its edges: above a level is strictly above it.
    let mut read: SomaRead = ([0; 6], 0, 0);
    for v in [
        SOMA_LEVELS_053[0],
        SOMA_LEVELS_053[0].saturating_add(1),
        V_RESET,
        THRESHOLD_BASE,
    ] {
        soma_add(&mut read, v);
    }
    assert_eq!(
        read,
        (
            [2, 1, 1, 1, 1, 1],
            i64::from(SOMA_LEVELS_053[0])
                .saturating_mul(2)
                .saturating_add(1)
                .saturating_add(i64::from(V_RESET))
                .saturating_add(base),
            4
        )
    );
    // The marks at each voltage on the instrument's network.
    let image = prior_image();
    let plain = frozen_from(&image, 1024);
    for arm in 0..ARMS_052.len() {
        for &v in &V_LOS_053 {
            let exec = engine_under(&image, arm, current_053(SHIFTS_052[0], v), true, None, 0);
            assert_eq!(&exec.blocks()[..plain.blocks().len()], plain.blocks());
        }
    }
    // The rules at their edges, over tables written by hand.
    rules_053_at_their_edges();
    // A marked assembly with the gate raised to 0.9, kicked for a few hundred ticks.
    let (a, v, k, w) = GATE_CELL_053;
    let (kicked, exec) = kicked_on_keeping(
        engine_under(
            &image,
            a,
            current_053(SHIFTS_052[k], V_LOS_053[v]),
            true,
            Some(WEIGHTS_052[w]),
            DELAY_SEED,
        ),
        SET_050,
        SIZE_052,
    );
    let slow: Vec<i32> = members(SIZE_052)
        .iter()
        .map(|&m| exec.units()[m as usize].v_slow)
        .collect();
    eprintln!("DUMP GATE_KICKED_053 = {kicked:?}");
    eprintln!("DUMP GATE_SLOW_053 = {slow:?}");
    assert!(
        slow.iter().all(|&s| s > 0),
        "the volley charged every member's slow potential"
    );
    assert_eq!(kicked.as_slice(), GATE_KICKED_053);
    assert_eq!(slow.as_slice(), GATE_SLOW_053);
    // The volley is ADR-0124's gate's spike for spike: the gate opens on what follows it.
    let volley = SIZE_052 as usize;
    assert_eq!(kicked[..volley], GATE_KICKED_052[..volley]);
    assert_ne!(kicked[volley], GATE_KICKED_052[volley]);
    // The rules over the pinned tables.
    over_the_053_tables();
}

/// The rules over brief 053's pinned tables: the quiet soma's readings over every member and tick
/// after the lead-in; each background's and control's stretches as the rules read them; each
/// control's lead-in and first unkicked span its background's, the lead-in's slow reading with
/// them; the kick's reading, whether it passes, and the pair window after its span; and the pairs
/// the rule picks.
fn over_the_053_tables() {
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let n = stretches_of(&spans).len();
    let first = stretches_of(&spans)
        .iter()
        .position(|s| s.0 == Span::Hold)
        .expect("a hold span");
    let member_ticks = |b: Background| u64::from(SIZE_052).saturating_mul(b.ticks);
    assert_eq!(QUIET_READ_053.len(), ARMS_052.len());
    for &(b, soma) in QUIET_READ_053 {
        assert_eq!(
            soma.2,
            member_ticks(b),
            "the quiet soma read at every member and tick"
        );
        assert!(
            soma.0.windows(2).all(|w| w[0] >= w[1]),
            "fewer above a higher level"
        );
    }
    assert_eq!(
        background_049(QUIET_053_A.rows, &spans),
        QUIET_READ_053[0].0
    );
    assert_eq!(QUIET_READ_053[1].0, CORE_READ_050[SIZE_051_IN_050].0);
    let mut after = [[[(0u64, 0u64); 3]; 3]; 2];
    for (a, by_v) in BG_RUNS_053.iter().enumerate() {
        for (v, by_shift) in by_v.iter().enumerate() {
            for (k, [bg, ctl]) in by_shift.iter().enumerate() {
                assert_eq!((bg.rows.len(), ctl.rows.len()), (n, n));
                let read = BG_READ_053[a][v][k];
                let background = background_049(bg.rows, &spans);
                assert_eq!(background, read.0);
                assert_eq!(bg.soma.2, member_ticks(background));
                let kick = kick_reading_049(ctl.kicks, SIZE_052);
                assert_eq!(kick, read.1);
                assert_eq!(passes_053(&read), kicked_once_049(ctl.kicks, SIZE_052));
                assert_eq!(holding(ctl.rows, &spans, background), read.2);
                assert_eq!(
                    ctl.rows[..first],
                    bg.rows[..first],
                    "the growth changes nothing unkicked"
                );
                assert_eq!(
                    bg.slow.0[0], ctl.slow.0[0],
                    "nor the lead-in's slow potential"
                );
                after[a][v][k] = (background_after(background, kick.2), kick.1);
            }
        }
    }
    assert_eq!(after, KICK_AFTER_053, "the pair window after the kick");
    assert_eq!(cell_pairs_053(&BG_READ_053), CELL_PAIRS_053);
    // The cells: each read against its pair's background, a kick a round; none usable, so
    // ADR-0125's case 2, and the line pauses. Every cell fails to let go, all but one run away,
    // and four never hold.
    let mut failures = [0u32; 3];
    for (i, &((a, v, k), reads)) in CELLS_053.iter().enumerate() {
        let background = background_049(BG_RUNS_053[a][v][k][0].rows, &spans);
        for (w, pin) in CELL_RUNS_053[i].iter().enumerate() {
            assert_eq!(pin.rows.len(), n);
            assert_eq!(pin.kicks.len(), ROUNDS, "a kick a round");
            assert_eq!(pin.soma.2, member_ticks(background));
            assert_eq!(holding(pin.rows, &spans, background), reads[w]);
            for (count, f) in failures.iter_mut().zip(failed(&reads[w])) {
                *count = count.saturating_add(u32::from(f));
            }
        }
    }
    let case = case_053(&BG_READ_053, &CELLS_053);
    assert_eq!(case, Case053::NoneUsable);
    assert!(case.pauses(), "ADR-0125's case 2: the line pauses");
    assert_eq!(
        failures,
        [4, 15, 16],
        "never holds, runs away, does not let go"
    );
}

/// A pair's reading written by hand: a background of `members` spikes over the backgrounds' ticks,
/// and a kick whose volley is `volley` of 512 and whose pair window after the span holds `after`.
fn hand_pair(members: u64, volley: u64, after: u64) -> BackgroundControl {
    (
        Background {
            members,
            rest: 100_000,
            ticks: 6_291_456,
        },
        (volley, after, ROUNDS as u64, 0),
        Holding {
            first_half: 0,
            held: 0,
            ignited: 0,
            let_go: 8,
            spills: None,
        },
        ([0; 5], [0; 8], None, 0),
    )
}

/// The rules of brief 053 at their edges, over tables written by hand: the kick's measure, the
/// pairs whose cells run and their cut at four, the neighbours within an arm and a voltage, the
/// cells the conditions would run and their cut at eight, and ADR-0125's cases.
fn rules_053_at_their_edges() {
    // The kick's measure: a volley within the tolerance, and at most 51 spikes after at 64 members
    // over eight kicks.
    let fails = hand_pair(10_000, 512, 52);
    assert!(passes_053(&hand_pair(10_000, 512, 51)) && !passes_053(&fails));
    assert!(passes_053(&hand_pair(10_000, 496, 0)) && !passes_053(&hand_pair(10_000, 495, 0)));
    // No kick passes: no pair, case 1, the line pauses.
    let mut read: Read053 = [[[fails; 3]; 3]; 2];
    assert!(cell_pairs_053(&read).is_empty());
    let none = case_053(&read, &[]);
    assert_eq!(none, Case053::NoKick);
    assert!(none.pauses());
    // Up to four pass: every one, in the grid's order.
    for &(a, v, k) in &[(1, 0, 2), (0, 2, 1), (0, 1, 2)] {
        read[a][v][k] = hand_pair(20_000, 512, 40);
    }
    assert_eq!(cell_pairs_053(&read), [(0, 1, 2), (0, 2, 1), (1, 0, 2)]);
    // Six pass: the four whose background fires slowest.
    read[1][2][0] = hand_pair(9_000, 512, 40);
    read[0][0][0] = hand_pair(30_000, 512, 40);
    read[1][1][1] = hand_pair(8_000, 512, 40);
    assert_eq!(
        cell_pairs_053(&read),
        [(0, 1, 2), (0, 2, 1), (1, 1, 1), (1, 2, 0)]
    );
    // Five pass at one rate, and the cut falls on each tie in turn: the lowest V_lo, then the smaller
    // shift, then arm (B); a slower background outranks every tie.
    let tie = |passing: &[Pair053], slower: Option<Pair053>| {
        let mut t: Read053 = [[[fails; 3]; 3]; 2];
        for &(a, v, k) in passing {
            t[a][v][k] = hand_pair(20_000, 512, 40);
        }
        if let Some((a, v, k)) = slower {
            t[a][v][k] = hand_pair(19_999, 512, 40);
        }
        cell_pairs_053(&t)
    };
    let by_voltage = [(0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 2, 0), (1, 1, 0)];
    assert_eq!(
        tie(&by_voltage, None),
        [(0, 1, 0), (0, 2, 0), (1, 1, 0), (1, 2, 0)],
        "the lowest V_lo is cut"
    );
    assert_eq!(
        tie(&by_voltage[1..], Some((0, 0, 0))),
        [(0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 2, 0)],
        "a slower background outranks the ties"
    );
    assert_eq!(
        tie(
            &[(0, 0, 0), (0, 0, 1), (0, 0, 2), (1, 0, 1), (1, 0, 2)],
            None
        ),
        [(0, 0, 1), (0, 0, 2), (1, 0, 1), (1, 0, 2)],
        "the smaller shift is cut"
    );
    assert_eq!(
        tie(
            &[(0, 2, 2), (1, 2, 2), (0, 1, 1), (0, 1, 0), (1, 1, 0)],
            None
        ),
        [(0, 1, 0), (0, 1, 1), (0, 2, 2), (1, 2, 2)],
        "arm (B) is cut"
    );
    // Cells ran and none usable: case 2, the line pauses.
    let usable = Holding {
        first_half: 8,
        held: 8,
        ignited: 0,
        let_go: 8,
        spills: Some(false),
    };
    let never = Holding { held: 6, ..usable };
    let picked = cell_pairs_053(&read);
    let mut cells: Vec<(Pair053, [Holding; 4])> = picked.iter().map(|&p| (p, [never; 4])).collect();
    let none_usable = case_053(&read, &cells);
    assert_eq!(none_usable, Case053::NoneUsable);
    assert!(none_usable.pauses());
    // Some usable: case 3, every usable cell in the grid's order; the line does not pause.
    cells[3].1[1] = usable;
    cells[0].1[0] = usable;
    let some = case_053(&read, &cells);
    assert_eq!(
        some,
        Case053::Conditions(vec![((0, 1, 2), 0), ((1, 2, 0), 1)])
    );
    assert!(!some.pauses());
    // The neighbours: within an arm and a voltage, one weight step and one shift step either way;
    // a cell that did not run is not one, nor the same cell at another voltage or in the other arm.
    let grid: Vec<(Pair053, [Holding; 4])> = vec![
        ((0, 0, 0), [usable, usable, never, usable]),
        ((0, 0, 1), [usable, never, never, never]),
        ((0, 1, 1), [usable; 4]),
        ((1, 0, 1), [usable; 4]),
    ];
    assert_eq!(
        neighbours_053(&grid, ((0, 0, 0), 0)),
        2,
        "a weight step and a shift step"
    );
    assert_eq!(neighbours_053(&grid, ((0, 0, 0), 3)), 0);
    assert_eq!(
        neighbours_053(&grid, ((0, 0, 1), 1)),
        2,
        "a shift that did not run is not one"
    );
    assert_eq!(
        neighbours_053(&grid, ((0, 0, 1), 2)),
        0,
        "nor the other voltage, nor the other arm"
    );
    assert_eq!(neighbours_053(&grid, ((0, 1, 1), 1)), 2);
    // Twelve usable: the eight with the most usable neighbours, ties by the key.
    assert_eq!(usable_053(&grid).len(), 12);
    assert_eq!(
        condition_cells_053(&grid),
        [
            ((0, 0, 0), 0),
            ((0, 0, 1), 0),
            ((0, 1, 1), 0),
            ((0, 1, 1), 1),
            ((0, 1, 1), 2),
            ((1, 0, 1), 0),
            ((1, 0, 1), 1),
            ((1, 0, 1), 2)
        ],
        "cut: three with one neighbour at heavier weights, and one with none"
    );
    // The key: the lighter weight, then the smaller shift, then the higher V_lo, then arm (A).
    let key = tie_key_053;
    assert!(key(((1, 2, 2), 0)) < key(((0, 0, 0), 1)));
    assert!(key(((1, 2, 1), 1)) < key(((0, 0, 2), 1)));
    assert!(key(((1, 2, 1), 1)) < key(((0, 1, 1), 1)));
    assert!(key(((0, 1, 1), 1)) < key(((1, 1, 1), 1)));
}

/// The core grid's cells of the pair `pair` (brief 053): its four weights on the settled image,
/// each wired as ADR-0124's and read against the pair's background, pinned first; the pair one the
/// rule picks from the pinned reading; every run dumped before any is held.
fn cells_053(pair: Pair053) {
    let (a, v, k) = pair;
    let name = format!(
        "brief 053's core grid under {} at {} at {}",
        ARM_NAMES_052[a], V_LO_NAMES_053[v], SHIFTS_052[k]
    );
    let at = CELL_PAIRS_053
        .iter()
        .position(|&p| p == pair)
        .expect("a pair the rule picks");
    let spans = layout(RELEASE_STRETCHES_049[SET_050]);
    let bg = background_049(BG_RUNS_053[a][v][k][0].rows, &spans);
    assert_eq!(
        bg, BG_READ_053[a][v][k].0,
        "{name}: the background, pinned first"
    );
    assert!(passes_053(&BG_READ_053[a][v][k]), "{name}: the kick passes");
    let image = settled(&name);
    let current = current_053(SHIFTS_052[k], V_LOS_053[v]);
    let runs: Vec<Run050> = WEIGHTS_052
        .iter()
        .map(|&w| {
            let label = format!("{name} weight {w:#06x}");
            let run = run_053(&image, a, current, Some(w), false, &label);
            cell_read_052(&run, a, bg, &label);
            run
        })
        .collect();
    for (w, run) in runs.iter().enumerate() {
        let label = format!("{name} weight {w}");
        held_053(run, &CELL_RUNS_053[at][w], false, &label);
        let (read, bursts, held) = cell_read_052(run, a, bg, &label);
        assert_eq!(read, CELLS_053[at].1[w], "{label}: read");
        assert_eq!(bursts, CELL_BURSTS_053[at][w], "{label}: bursts");
        assert_eq!(held, CELL_HELD_053[at][w], "{label}: held");
    }
}

// ------------------------------------------------ the runs, weekly (brief 053)

#[test]
#[ignore]
fn the_gate_raised_the_quiet_soma_and_adr_0117s_cell_exhaustive() {
    quiet_soma_053();
}

#[test]
#[ignore]
fn the_gate_raised_to_0_7_the_slow_current_alone_the_backgrounds_and_the_controls_paused() {
    pairs_053(0, 0);
}

#[test]
#[ignore]
fn the_gate_raised_to_0_8_the_slow_current_alone_the_backgrounds_and_the_controls_paused() {
    pairs_053(0, 1);
}

#[test]
#[ignore]
fn the_gate_raised_to_0_9_the_slow_current_alone_the_backgrounds_and_the_controls_paused() {
    pairs_053(0, 2);
}

#[test]
#[ignore]
fn the_gate_raised_to_0_7_with_the_facilitating_class_the_backgrounds_and_the_controls_paused() {
    pairs_053(1, 0);
}

#[test]
#[ignore]
fn the_gate_raised_to_0_8_with_the_facilitating_class_the_backgrounds_and_the_controls_paused() {
    pairs_053(1, 1);
}

#[test]
#[ignore]
fn the_gate_raised_to_0_9_with_the_facilitating_class_the_backgrounds_and_the_controls_paused() {
    pairs_053(1, 2);
}

#[test]
#[ignore]
fn the_gate_raised_to_0_8_the_slow_current_alone_the_core_grid_at_shift_2_paused() {
    cells_053((0, 1, 2));
}

#[test]
#[ignore]
fn the_gate_raised_to_0_9_the_slow_current_alone_the_core_grid_at_shift_2_exhaustive() {
    cells_053((0, 2, 2));
}

#[test]
#[ignore]
fn the_gate_raised_to_0_8_with_the_facilitating_class_the_core_grid_at_shift_2_paused() {
    cells_053((1, 1, 2));
}

#[test]
#[ignore]
fn the_gate_raised_to_0_9_with_the_facilitating_class_the_core_grid_at_shift_2_exhaustive() {
    cells_053((1, 2, 2));
}

// ------------------------------------------------ brief 053's arithmetic, pinned before any run

/// The critical slow potential at each of the three voltages (`slow_critical_at`): the least that,
/// held, fires a unit under the drive's mean input from an eighth of the gate's span above the
/// voltage.
const SLOW_CRITICAL_053: [i32; 3] = [317_851, 410_185, 502_261];

/// ADR-0124's backgrounds' mean slow potentials over their unkicked spans, after none
/// (`background_levels_052`).
const LEVELS_053: [i32; 7] = [0, 36_815, 19_244, 7_442, 52_995, 24_035, 7_983];

// ------------------------------------------------ brief 053's runs, pinned

/// The pairs the rule picks from the pinned reading (`cell_pairs_053`): the kick passes at four,
/// shift 2 at 0.8 and 0.9 under both arms, so every one runs, in the grid's order.
const CELL_PAIRS_053: &[Pair053] = &[(0, 1, 2), (0, 2, 2), (1, 1, 2), (1, 2, 2)];
