//! Whitepaper TC-5 for the tick loop: after `Executor::new`, running ticks allocates nothing,
//! however many spikes, deliveries, wheel cascades and injections they carry, and however
//! many searches the discovery loop runs on its cadence over the engine's own store, with
//! their commits, rewards and tags (ADR-0052), and through a night whose slow-wave onset
//! compacts the arena (ADR-0056). A counting global allocator (the `unsafe` the
//! `GlobalAlloc` trait requires) counts the allocations of the engine's own threads while a
//! flag is set; the flag is set only around `run`.
//!
//! Whose allocation it is (F-64). The counter is the process's, and the process holds a
//! thread that is not the engine's: the test harness's own, which prints a finished test's
//! line and, past a minute, a notice for a slow one. Counting it reads the harness's
//! allocation as the tick loop's. So every thread is classed at its first allocation, in a
//! thread-local the allocator reads without allocating:
//! - a thread first seen before the test builds an engine is not the engine's, and is never
//!   counted;
//! - the test's own thread, which runs the executor's coordinator, is marked the engine's by
//!   the test;
//! - a thread first seen after that is one the executor spawned, and is the engine's.
//!
//! The file holds one test function, so that no second test's thread exists to be classed by
//! when it happened to start.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{CortexFileHeader, SECTION_HOMEOSTASIS, SectionEntry, crc64};
use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16};
use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool, PRESSURE_MAX_Q16, STAGE_SWS,
};
use cortex_reasoning::TermNode;
use cortex_runtime::{Config, Executor, Image};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Barrier};

struct Counting;

static COUNTING: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

/// Set by the test before it builds an engine: a thread first seen from here on is one the
/// executor spawned.
static ENGINES: AtomicBool = AtomicBool::new(false);

/// A thread that has not allocated yet.
const UNSEEN: u8 = 0;
/// The engine's: the test's own thread, or one first seen after [`ENGINES`] was set.
const ENGINE: u8 = 1;
/// Not the engine's: first seen before [`ENGINES`] was set, as the harness's thread is.
const OTHER: u8 = 2;

thread_local! {
    /// Whose thread this is. A constant initialiser and no destructor, so reading it inside
    /// the allocator allocates nothing and registers nothing.
    static WHOSE: Cell<u8> = const { Cell::new(UNSEEN) };
}

/// Classes the calling thread at its first allocation and says whether it is the engine's.
/// A thread whose thread-local cannot be read is taken as the engine's, so that no
/// allocation of the engine's is ever left out.
fn engine_thread() -> bool {
    WHOSE
        .try_with(|whose| {
            if whose.get() == UNSEEN {
                whose.set(if ENGINES.load(Ordering::Relaxed) {
                    ENGINE
                } else {
                    OTHER
                });
            }
            whose.get() == ENGINE
        })
        .unwrap_or(true)
}

// SAFETY: every call is forwarded to the system allocator unchanged; the class and the
// counter are the only additions, a thread-local without a destructor and an atomic.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if engine_thread() && COUNTING.load(Ordering::Relaxed) {
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
        // Every index is below the twelve blocks and the three units: the named operations
        // are the lint's (§8.1), not a bound.
        for (i, &delay) in delays.iter().enumerate() {
            let target = i.wrapping_add(1).wrapping_rem(3) as u32;
            let first = i.wrapping_mul(4);
            for k in 0..13usize {
                assert!(blocks[first.wrapping_add(k >> 2)].set_synapse(
                    k & 3,
                    target,
                    i16::MAX,
                    delay,
                    false
                ));
            }
            for j in 0..3usize {
                let block = first.wrapping_add(j);
                assert!(blocks[block].link(block.wrapping_add(1) as u32));
            }
        }
    }
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        unit.v_thresh = THRESHOLD_BASE;
        assert!(unit.set_first_block(i.wrapping_mul(4) as u32));
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

/// The tick loop after `Executor::new`: 20 000 ticks of spikes, fan-out, deliveries, wheel
/// cascades, injections and the discovery loop on its cadence.
fn the_tick_loop_allocates_nothing_after_new() {
    let mut exec = network();
    let inject = exec.injector();
    let one = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..13 {
        inject.inject(0, spike_message(one, false)).unwrap();
    }
    // Warm up: the worker thread has started and the first spikes have fanned out.
    exec.run(2000);

    ALLOCATIONS.store(0, Ordering::SeqCst);
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

/// A night inside the tick: the slow-wave onset's compaction and two windows of replay.
fn a_night_inside_the_tick_allocates_nothing() {
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
    // The window is a power of two, so the ticks into it are a mask on the tick (§8.1: no
    // division by a value the lint cannot see is not zero).
    let to_boundary = window.wrapping_sub(exec.ticks() & window.wrapping_sub(1));

    ALLOCATIONS.store(0, Ordering::SeqCst);
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

/// The counter's own controls, before it is trusted with the engine: an allocation of the
/// test's thread inside a window is counted, one of a thread first seen before the engines
/// is not, and a thread first seen after them is classed the engine's.
///
/// `harness` stands in for the test harness's thread: it was spawned, and made its first
/// allocation, before [`ENGINES`] was set. It waits at `go`, allocates, and waits at `done`,
/// so that its allocation falls inside the window this function opens.
fn the_counter_counts_the_engine_s_threads_and_no_other(
    harness: std::thread::JoinHandle<u8>,
    go: &Barrier,
    done: &Barrier,
) {
    // The test's own thread, inside a window: counted, once.
    ALLOCATIONS.store(0, Ordering::SeqCst);
    COUNTING.store(true, Ordering::SeqCst);
    let boxed = black_box(Box::new(1u8));
    COUNTING.store(false, Ordering::SeqCst);
    assert_eq!(
        ALLOCATIONS.load(Ordering::SeqCst),
        1,
        "an allocation of the test's thread inside a window is counted"
    );
    drop(boxed);

    // The harness's stand-in, inside a window: not counted.
    ALLOCATIONS.store(0, Ordering::SeqCst);
    COUNTING.store(true, Ordering::SeqCst);
    go.wait();
    done.wait();
    COUNTING.store(false, Ordering::SeqCst);
    assert_eq!(
        ALLOCATIONS.load(Ordering::SeqCst),
        0,
        "an allocation of a thread first seen before the engines is not counted"
    );
    assert_eq!(
        harness.join().unwrap(),
        OTHER,
        "a thread first seen before the engines is not the engine's"
    );

    // A thread first seen after the engines, as the executor's workers are: the engine's.
    let spawned = std::thread::spawn(|| {
        drop(black_box(Box::new(3u8)));
        WHOSE.with(Cell::get)
    });
    assert_eq!(
        spawned.join().unwrap(),
        ENGINE,
        "a thread first seen after the engines is the engine's"
    );
}

#[test]
fn the_tick_loop_and_a_night_inside_it_allocate_nothing() {
    // The harness's stand-in is spawned first, while no thread is yet the engine's.
    let ready = Arc::new(Barrier::new(2));
    let go = Arc::new(Barrier::new(2));
    let done = Arc::new(Barrier::new(2));
    let harness = {
        let (ready, go, done) = (ready.clone(), go.clone(), done.clone());
        std::thread::spawn(move || {
            drop(black_box(Box::new(2u8)));
            ready.wait();
            go.wait();
            drop(black_box(Box::new(2u8)));
            done.wait();
            WHOSE.with(Cell::get)
        })
    };
    ready.wait();

    // From here every thread first seen is one an executor spawned, and this thread runs
    // their coordinator.
    WHOSE.with(|whose| whose.set(ENGINE));
    ENGINES.store(true, Ordering::SeqCst);

    the_counter_counts_the_engine_s_threads_and_no_other(harness, &go, &done);
    the_tick_loop_allocates_nothing_after_new();
    a_night_inside_the_tick_allocates_nothing();
}
