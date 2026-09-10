//! Whitepaper TC-5 for the tick loop: after `Executor::new`, running ticks allocates nothing,
//! however many spikes, deliveries, wheel cascades and injections they carry. A counting
//! global allocator (the `unsafe` the `GlobalAlloc` trait requires) counts every allocation
//! while a flag is set; the flag is set only around `run`.

use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16};
use cortex_runtime::{Config, Executor};
use std::alloc::{GlobalAlloc, Layout, System};
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

#[test]
fn the_tick_loop_allocates_nothing_after_new() {
    let mut exec = Executor::<64>::new(Config {
        workers: 2,
        units: 3,
        blocks: 12,
        nodes_per_worker: 256,
        injector_capacity: 64,
        trace_capacity: 1024,
        ..Config::default()
    })
    .unwrap();
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
    let reports = exec.shutdown();
    assert!(
        reports.iter().map(|r| r.spikes.len()).sum::<usize>() > 30,
        "the loop did real work: spikes, fan-out, deliveries"
    );
    assert_eq!(allocations, 0, "no allocation in 20 000 ticks");
}
