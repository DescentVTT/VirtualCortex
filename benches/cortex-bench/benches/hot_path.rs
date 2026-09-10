//! Micro-benchmarks for the parts of whitepaper target T-3 that exist (ADR-0014).
//!
//! Every benchmark draws its inputs from `cortex_bench::Lcg` seeded with `Lcg::SEED`, so the
//! measured sequence is identical from run to run. Figures produced by this file are Measured in
//! the whitepaper's sense only when obtained under the protocol in `docs/benchmarks/README.md`
//! on the reference platform; anywhere else they are recorded as non-admissible.

use cortex_basal_ganglia::BasalGangliaChannelState;
use cortex_bench::Lcg;
use cortex_core::{DendriticSuperNeuron, MailboxNode, WorkerWheel, synaptic_efficacy_q16};
use cortex_workspace::GlobalWorkspaceSlot;
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

/// A production-geometry wheel (4 MB) built on a thread with a large stack and moved to the
/// heap; `Box::new(WorkerWheel::new())` on the main thread would overflow its stack.
fn boxed_worker_wheel() -> Box<WorkerWheel> {
    std::thread::Builder::new()
        .stack_size(32 << 20)
        .spawn(|| Box::new(WorkerWheel::new()))
        .expect("spawn")
        .join()
        .expect("join")
}

/// R-1 step 1: one `schedule` followed by one `advance`, steady state, one token per tick.
/// Delays are uniform over both rings (1..2559), so the coarse ring and its cascade are
/// exercised at their real frequency.
fn wheel(c: &mut Criterion) {
    let mut group = c.benchmark_group("wheel");
    group.throughput(Throughput::Elements(1));

    group.bench_function("advance_empty", |b| {
        let mut w = boxed_worker_wheel();
        b.iter(|| black_box(w.advance().len()));
    });

    group.bench_function("schedule_then_advance/both_rings", |b| {
        let mut w = boxed_worker_wheel();
        let mut rng = Lcg(Lcg::SEED);
        b.iter(|| {
            let delay = rng.range(1, 2560);
            let token = rng.next_u32() & 0x0FFF_FFFF;
            w.schedule(black_box(delay), black_box(token))
                .expect("in range");
            black_box(w.advance().len())
        });
    });

    group.bench_function("schedule_then_advance/fine_ring", |b| {
        let mut w = boxed_worker_wheel();
        let mut rng = Lcg(Lcg::SEED);
        b.iter(|| {
            let delay = rng.range(1, 256);
            let token = rng.next_u32() & 0x0FFF_FFFF;
            w.schedule(black_box(delay), black_box(token))
                .expect("in range");
            black_box(w.advance().len())
        });
    });

    group.finish();
}

/// The synaptic efficacy product (ADR-0012): one call, and a batch of 1 024 calls with the
/// inputs precomputed so the batch measures the arithmetic rather than the generator.
fn efficacy(c: &mut Criterion) {
    let mut group = c.benchmark_group("efficacy");

    group.throughput(Throughput::Elements(1));
    group.bench_function("single", |b| {
        let mut rng = Lcg(Lcg::SEED);
        b.iter(|| {
            let w = rng.next_u32() as i16;
            let u = rng.next_u32() as u8;
            let r = rng.next_u32() as u8;
            black_box(synaptic_efficacy_q16(
                black_box(w),
                black_box(u),
                black_box(r),
            ))
        });
    });

    let mut rng = Lcg(Lcg::SEED);
    let inputs: Vec<(i16, u8, u8)> = (0..1024)
        .map(|_| {
            (
                rng.next_u32() as i16,
                rng.next_u32() as u8,
                rng.next_u32() as u8,
            )
        })
        .collect();
    group.throughput(Throughput::Elements(inputs.len() as u64));
    group.bench_function("batch_1024", |b| {
        b.iter(|| {
            let mut acc = 0i32;
            for &(w, u, r) in black_box(&inputs) {
                acc = acc.saturating_add(synaptic_efficacy_q16(w, u, r));
            }
            black_box(acc)
        });
    });

    group.finish();
}

/// Basal-ganglia gating: one channel, drives drawn per call.
fn gating(c: &mut Criterion) {
    let mut group = c.benchmark_group("gating");
    group.throughput(Throughput::Elements(1));
    group.bench_function("compute_gating", |b| {
        let mut rng = Lcg(Lcg::SEED);
        let mut ch = BasalGangliaChannelState {
            channel_id: 0,
            striatal_d1_drive: 0,
            striatal_d2_drive: 0,
            stn_hyperdirect_drive: 0,
            gpi_snr_inhibition: 0,
            dopamine_modulation: 0,
            habit_strength: 0,
            selected_flag: 0,
            _reserved: [0; 32],
        };
        b.iter(|| {
            ch.striatal_d1_drive = (rng.next_u32() >> 4) as i32;
            ch.striatal_d2_drive = (rng.next_u32() >> 4) as i32;
            ch.stn_hyperdirect_drive = (rng.next_u32() >> 5) as i32;
            black_box(ch.compute_gating())
        });
    });
    group.finish();
}

/// Workspace ignition: 64 sub-threshold steps on a fresh slot per iteration (evidence 0x100,
/// 64 × 0x100 < the 1.5 threshold), so the non-ignited path is what is measured. Divide the
/// reported time by 64 for the per-step cost.
fn ignition(c: &mut Criterion) {
    let mut group = c.benchmark_group("ignition");
    group.throughput(Throughput::Elements(64));
    group.bench_function("step_ignition_x64", |b| {
        b.iter_batched(
            || GlobalWorkspaceSlot {
                slot_id: 0,
                binding_hash: 0,
                ignition_potential: 0,
                persistence_ticks: 0,
                confidence_q16: 0,
                broadcast_channel_mask: 0,
                p300_wave_phase: 0,
                is_ignited: 0,
                _reserved: [0; 32],
            },
            |mut s| {
                let mut ignited = false;
                for _ in 0..64 {
                    ignited |= s.step_ignition(black_box(0x100));
                }
                black_box(ignited)
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

/// R-1 steps 2 and 3 (ADR-0017): sixteen pushes into one unit's mailbox from a sixteen-node
/// arena, then one drain that walks them all. Divide by 16 for the per-push cost, which
/// includes its share of the drain.
fn mailbox(c: &mut Criterion) {
    let mut group = c.benchmark_group("mailbox");
    group.throughput(Throughput::Elements(16));
    group.bench_function("push_drain_x16", |b| {
        let unit = DendriticSuperNeuron::new(1);
        let nodes: [MailboxNode; 16] = core::array::from_fn(|_| MailboxNode::new());
        let mut rng = Lcg(Lcg::SEED);
        b.iter(|| {
            for i in 0..16u32 {
                unit.mailbox_push(black_box(&nodes[..]), i, rng.next_u32());
            }
            black_box(unit.mailbox_drain(&nodes[..]).count())
        });
    });
    group.finish();
}

/// The turn gate (ADR-0017): schedule, begin and end on an idle unit with an empty mailbox;
/// three atomic operations on the gate and one load of the head, three of them sequentially
/// consistent.
fn gate(c: &mut Criterion) {
    let mut group = c.benchmark_group("gate");
    group.throughput(Throughput::Elements(1));
    group.bench_function("schedule_begin_end", |b| {
        let unit = DendriticSuperNeuron::new(1);
        b.iter(|| {
            let scheduled = unit.try_schedule();
            let claimed = unit.begin_turn();
            let rescheduled = unit.end_turn();
            black_box((scheduled, claimed, rescheduled))
        });
    });
    group.finish();
}

criterion_group!(benches, wheel, efficacy, gating, ignition, mailbox, gate);
criterion_main!(benches);
