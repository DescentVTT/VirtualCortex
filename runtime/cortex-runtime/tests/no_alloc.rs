//! Whitepaper TC-5 for the tick loop: after `Executor::new`, running ticks allocates nothing,
//! however many spikes, deliveries, wheel cascades and injections they carry, and however
//! many searches the discovery loop runs on its cadence over the engine's own store, with
//! their commits, rewards and tags (ADR-0052), and through a night whose slow-wave onset
//! compacts the arena (ADR-0056). A counting global allocator (the `unsafe` the
//! `GlobalAlloc` trait requires) counts every allocation while a flag is set; the flag is set
//! only around `run`.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{CortexFileHeader, SECTION_HOMEOSTASIS, SectionEntry, crc64};
use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16};
use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool, PRESSURE_MAX_Q16, STAGE_SWS,
};
use cortex_reasoning::TermNode;
use cortex_runtime::{Config, Executor, Image};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct Counting;

static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

// SAFETY: every call is forwarded to the system allocator unchanged; the counter is the only
// addition and it is an atomic.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        // SAFETY: the same layout the caller passed.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` came from `System.alloc` with this layout.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// The counter is the process's: the two tests run one at a time, each holding this for
/// its whole body, so that neither's set-up is counted in the other's window.
static SERIAL: Mutex<()> = Mutex::new(());

fn config() -> Config {
    Config {
        workers: 2,
        units: 3,
        blocks: 12,
        nodes_per_worker: 256,
        injector_capacity: 64,
        trace_capacity: 1024,
        train_capacity: 64,
        episodes: 2,
        terms: 256,
        clauses: 16,
        search_shift: 9,
        search_budget: 64,
        discovery_tag: 3,
        ..Config::default()
    }
}

/// Three units in a ring of thirteen synapses each, armed, with the exit store of ADR-0045
/// in the engine's own arena; nothing injected, so the engine is quiescent.
fn network() -> Executor<64> {
    let mut exec = Executor::<64>::new(config()).unwrap();
    {
        let blocks = exec.blocks_mut();
        let delays = [300u16, 500, 700];
        for i in 0..3 {
            let target = ((i + 1) % 3) as u32;
            for k in 0..13 {
                assert!(blocks[4 * i + k / 4].set_synapse(
                    k % 4,
                    target,
                    i16::MAX,
                    delays[i],
                    false
                ));
            }
            for j in 0..3 {
                assert!(blocks[4 * i + j].link((4 * i + j + 1) as u32));
            }
        }
    }
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        unit.v_thresh = THRESHOLD_BASE;
        assert!(unit.set_first_block((4 * i) as u32));
    }
    // The exit store of ADR-0045 in the engine's own arena: three rules of one head sharing
    // four literals and differing in a fifth, so that the loop's first search on its cadence
    // commits two inventions, rewards the modulator and tags the coincidence before it, and
    // every later search walks the pairs and commits nothing.
    let next = exec.induction().next_variable;
    let x = exec.term(TermNode::variable(next)).unwrap();
    let head = exec.term(TermNode::compound(0x100, &[x]).unwrap()).unwrap();
    let literals: Vec<u32> = (0x200..0x207u32)
        .map(|l| exec.term(TermNode::compound(l, &[x]).unwrap()).unwrap())
        .collect();
    for own in 4..7 {
        let body = [
            literals[0],
            literals[1],
            literals[2],
            literals[3],
            literals[own],
        ];
        exec.assert_clause(head, &body).unwrap();
    }
    exec
}

#[test]
fn the_tick_loop_allocates_nothing_after_new() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut exec = network();
    let inject = exec.injector();
    let one = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..13 {
        inject.inject(0, spike_message(one, false)).unwrap();
    }
    // Warm up: the worker thread has started and the first spikes have fanned out.
    exec.run(2000);

    COUNTING.store(true, Ordering::SeqCst);
    for round in 0..40u32 {
        // A kick of twenty messages into one unit every 500 ticks keeps the units firing and
        // the fan-out, the wheel and the deliveries busy; injecting allocates nothing either.
        for _ in 0..20 {
            inject
                .inject(round % 3, spike_message(one, round % 2 == 0))
                .unwrap();
        }
        exec.run(500);
    }
    COUNTING.store(false, Ordering::SeqCst);

    let allocations = ALLOCATIONS.load(Ordering::SeqCst);
    assert!(
        exec.searches() >= 39 && exec.inventions() == 2 && exec.episodes().len() == 1,
        "the loop ran on its cadence: {} searches, {} inventions, {} episodes",
        exec.searches(),
        exec.inventions(),
        exec.episodes().len()
    );
    let reports = exec.shutdown();
    assert!(
        reports.iter().map(|r| r.spikes.len()).sum::<usize>() > 30,
        "the loop did real work: spikes, fan-out, deliveries"
    );
    assert_eq!(allocations, 0, "no allocation in 20 000 ticks");
}

#[test]
fn a_night_inside_the_tick_allocates_nothing() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    // The same network, quiescent, searched once between ticks so the arena holds garbage
    // (no spike is on the train, so the rewarded search tags nothing and says so), an
    // episode of the three units tagged by hand, then put at the edge of sleep through its
    // image (the pressure at its maximum, the shift 5): the first window's step is the
    // slow-wave onset, which compacts the arena inside the tick, and every ripple of the
    // stage replays the episode. Decoding allocates; the night does not.
    let mut exec = network();
    assert!(exec.discover().is_err(), "no coincidence on an empty train");
    assert!(exec.inventions() >= 2);
    assert_eq!(exec.tag_episode(&[0, 1, 2], 3), Ok(0));
    let mut img = Image::encode(&exec).expect("quiescent");
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_HOMEOSTASIS {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            let mut pool = HomeostaticDrivePool::decode((&img[offset..][..64]).try_into().unwrap());
            pool.sleep_shift = 5;
            pool.sleep_pressure_q16 = PRESSURE_MAX_Q16;
            img[offset..][..length].copy_from_slice(&pool.encode());
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            break;
        }
    }
    let mut exec = Image::decode::<64>(&img, config()).unwrap();
    let inject = exec.injector();
    let one = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..13 {
        inject.inject(0, spike_message(one, false)).unwrap();
    }
    exec.run(500);
    let window = 1u64 << (ACTIVITY_BIN_SHIFT + ACTIVITY_WINDOW_SHIFT);
    let to_boundary = window.wrapping_sub(exec.ticks().wrapping_rem(window));

    COUNTING.store(true, Ordering::SeqCst);
    exec.run(to_boundary.wrapping_add(window.wrapping_mul(2)));
    COUNTING.store(false, Ordering::SeqCst);

    let allocations = ALLOCATIONS.load(Ordering::SeqCst);
    assert_eq!(exec.sleep_stage(), STAGE_SWS);
    assert!(
        exec.compactions() == 1 && exec.reclaimed() > 0,
        "the onset compacted: {} compactions, {} nodes",
        exec.compactions(),
        exec.reclaimed()
    );
    assert!(exec.replays() > 0, "the night replays the tagged episode");
    assert_eq!(
        allocations, 0,
        "no allocation through the onset and two windows of sleep"
    );
}
