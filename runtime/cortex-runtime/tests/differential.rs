//! The first differential test toward target T-1 (whitepaper §8.3): the same network and the
//! same injections on one worker and on four leave bit-identical arenas and the same spike
//! train. Two networks: the three-unit ring of `crates/cortex-core/tests/oscillator.rs` (here
//! under short-term plasticity, which that harness holds at rest, so the ring rings down
//! rather than oscillating) and a random network of 128 units with STDP at work.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::Crc64;
use cortex_core::{
    DendriticSuperNeuron, GateState, MODULATION_ONE_Q16, STP_MAX, STP_U, SynapseBlock,
    THRESHOLD_BASE, spike_message, synaptic_efficacy_q16,
};
use cortex_runtime::{Config, Executor};

#[derive(Clone, Debug, PartialEq, Eq)]
struct UnitSnapshot {
    id: u64,
    mailbox_head: u64,
    potentials: (i32, i32, i32, i32),
    windows: (u16, u16),
    last_spike: u32,
    first_block: u32,
    gate: u8,
    stp: (u8, u8),
}

fn snapshot(units: &[DendriticSuperNeuron]) -> Vec<UnitSnapshot> {
    units
        .iter()
        .map(|u| UnitSnapshot {
            id: u.id,
            mailbox_head: u.mailbox_head_ptr.load(std::sync::atomic::Ordering::SeqCst),
            potentials: (u.v_soma, u.v_basal, u.v_apical, u.v_thresh),
            windows: (u.bac_plateau_ticks, u.refractory_ticks),
            last_spike: u.last_soma_spike_tick,
            first_block: u.synapse_slab_idx,
            gate: u.gate().map_or(9, |g| g as u8),
            stp: (u.stp_r_ves, u.stp_u_rel),
        })
        .collect()
}

struct Outcome {
    units: Vec<UnitSnapshot>,
    /// Every unit's 64 image bytes, atomics as plain values (`flags` and the delta head
    /// included, which the snapshot does not carry).
    unit_bytes: Vec<[u8; 64]>,
    blocks: Vec<SynapseBlock>,
    spikes: Vec<(u32, u32)>,
}

fn run<F: Fn(&mut Executor<64>)>(workers: usize, config: Config, wire: F, ticks: u64) -> Outcome {
    let mut exec = Executor::<64>::new(Config { workers, ..config }).unwrap();
    wire(&mut exec);
    exec.run(ticks);
    assert!(
        exec.units()
            .iter()
            .all(|u| u.gate() != Some(GateState::Running)),
        "no turn is open between ticks"
    );
    let units = snapshot(exec.units());
    let unit_bytes = exec.units().iter().map(|u| u.encode()).collect();
    let blocks = exec.blocks().to_vec();
    let reports = exec.shutdown();
    assert_eq!(reports.iter().map(|r| r.dropped).sum::<u64>(), 0);
    let mut spikes: Vec<(u32, u32)> = reports
        .iter()
        .flat_map(|r| r.spikes.iter().map(|&(u, t)| (t, u)))
        .collect();
    spikes.sort_unstable();
    Outcome {
        units,
        unit_bytes,
        blocks,
        spikes: spikes.into_iter().map(|(t, u)| (u, t)).collect(),
    }
}

/// The oscillator ring: thirteen synapses per hop in four blocks, delays (300, 500, 700).
fn wire_ring(exec: &mut Executor<64>) {
    const HOP: usize = 13;
    let delays = [300u16, 500, 700];
    {
        let blocks = exec.blocks_mut();
        for (i, &delay) in delays.iter().enumerate() {
            let target = (i.wrapping_add(1) % 3) as u32;
            let base = i.wrapping_mul(4);
            for k in 0..HOP {
                assert!(blocks[base.wrapping_add(k / 4)].set_synapse(
                    k % 4,
                    target,
                    i16::MAX,
                    delay,
                    false
                ));
            }
            for j in 0..3 {
                let block = base.wrapping_add(j);
                assert!(blocks[block].link(block.wrapping_add(1) as u32));
            }
        }
    }
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        unit.v_thresh = THRESHOLD_BASE;
        assert!(unit.set_first_block(i.wrapping_mul(4) as u32));
    }
    let inject = exec.injector();
    let one = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..HOP {
        inject.inject(0, spike_message(one, false)).unwrap();
    }
}

#[test]
fn the_ring_is_identical_on_one_and_four_workers_and_goes_round() {
    let config = Config {
        units: 3,
        blocks: 12,
        nodes_per_worker: 256,
        injector_capacity: 64,
        trace_capacity: 4096,
        ..Config::default()
    };
    let one = run(1, config.clone(), wire_ring, 20_000);
    let four = run(4, config, wire_ring, 20_000);
    assert_eq!(one.units, four.units, "unit arenas are bit-identical");
    assert_eq!(one.blocks, four.blocks, "synapse arenas are bit-identical");
    assert_eq!(one.spikes, four.spikes, "the spike train is the same");
    for unit in 0..3 {
        assert!(
            one.spikes.iter().any(|&(u, _)| u == unit),
            "unit {unit} fired: the spike went round the ring"
        );
    }
    let unit0: Vec<u32> = one
        .spikes
        .iter()
        .filter(|&&(u, _)| u == 0)
        .map(|&(_, t)| t)
        .collect();
    assert!(
        unit0.len() >= 2,
        "unit 0 fired again after the round trip: {unit0:?}"
    );
    let round_trip = unit0[1] - unit0[0];
    assert!(
        (1500..1560).contains(&round_trip),
        "the round trip is the three delays plus a few ticks of integration per hop: {round_trip}"
    );
}

/// A pseudo-random network: 128 units, one block each, four synapses to random targets with
/// random delays (some zero, some at the horizon), random weights, some apical.
fn wire_random(exec: &mut Executor<64>) {
    const UNITS: usize = 128;
    let mut x = 0x9E37_79B9u32;
    let mut next = || {
        x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        x
    };
    {
        let blocks = exec.blocks_mut();
        for (i, block) in blocks.iter_mut().enumerate().take(UNITS) {
            for slot in 0..4 {
                let target = (next() >> 8)
                    .checked_rem(UNITS as u32)
                    .expect("UNITS is not zero");
                let delay = match next() % 8 {
                    0 => 0,
                    1 => 2559,
                    _ => ((next() >> 8) % 400).wrapping_add(1),
                } as u16;
                let weight = (((next() >> 8) % 24_000) as i16).wrapping_add(8_000);
                let apical = next() % 5 == 0;
                assert!(block.set_synapse(slot, target, weight, delay, apical));
            }
            let _ = i;
        }
    }
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
        assert!(unit.set_first_block(i as u32));
    }
    let inject = exec.injector();
    for _ in 0..48 {
        let unit = (next() >> 8)
            .checked_rem(UNITS as u32)
            .expect("UNITS is not zero");
        for _ in 0..8 {
            inject.inject(unit, spike_message(0x7000, false)).unwrap();
        }
    }
}

#[test]
fn a_random_network_with_stdp_is_bit_identical_on_one_and_four_workers() {
    let config = Config {
        units: 128,
        blocks: 128,
        nodes_per_worker: 4096,
        injector_capacity: 1024,
        trace_capacity: 1 << 16,
        ..Config::default()
    };
    let one = run(1, config.clone(), wire_random, 6000);
    let four = run(4, config, wire_random, 6000);
    assert!(!one.spikes.is_empty(), "the network fired");
    assert!(
        one.blocks.iter().any(|b| b.last_spike_tick != 0),
        "fan-out ran through the blocks"
    );
    assert_eq!(one.units, four.units, "unit arenas are bit-identical");
    assert_eq!(
        one.blocks, four.blocks,
        "synapse arenas are bit-identical, STDP included"
    );
    assert_eq!(one.spikes, four.spikes, "the spike train is the same");
}

#[test]
fn a_delayed_synapse_arrives_delay_ticks_after_the_spike_and_a_zero_delay_one_the_next_tick() {
    let mut exec = Executor::<64>::new(Config {
        units: 3,
        blocks: 4,
        nodes_per_worker: 64,
        injector_capacity: 64,
        trace_capacity: 64,
        ..Config::default()
    })
    .unwrap();
    {
        let blocks = exec.blocks_mut();
        // Unit 0 fires on thirteen kicks; its synapses: unit 1 at delay 7, unit 2 at delay 0.
        assert!(blocks[0].set_synapse(0, 1, i16::MAX, 7, false));
        assert!(blocks[0].set_synapse(1, 2, i16::MAX, 0, true));
    }
    for unit in exec.units_mut().iter_mut() {
        unit.v_thresh = THRESHOLD_BASE;
    }
    assert!(exec.units_mut()[0].set_first_block(0));
    let inject = exec.injector();
    let one = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..13 {
        inject.inject(0, spike_message(one, false)).unwrap();
    }
    let mut spike_tick = None;
    let mut arrivals = [None, None];
    for _ in 0..40 {
        exec.tick();
        let now = exec.ticks() as u32;
        let units = exec.units();
        if spike_tick.is_none() && units[0].last_soma_spike_tick != 0 {
            spike_tick = Some(units[0].last_soma_spike_tick);
        }
        if arrivals[0].is_none() && units[1].v_basal != 0 {
            arrivals[0] = Some(now);
        }
        if arrivals[1].is_none() && units[2].v_apical != 0 {
            arrivals[1] = Some(now);
        }
    }
    let spike = spike_tick.expect("unit 0 fired");
    // A potential set at tick t is visible after the tick that ran t, whose `ticks()` is t + 1.
    assert_eq!(
        arrivals[0],
        Some(spike + 7 + 1),
        "delay 7: integrated at spike + 7"
    );
    assert_eq!(
        arrivals[1],
        Some(spike + 1 + 1),
        "delay 0: integrated at spike + 1"
    );
    let reports = exec.shutdown();
    assert_eq!(reports[0].spikes, vec![(0, spike)]);
}

/// The pin for target T-1 (ADR-0030): the random network's arenas and spike train after
/// 20 000 ticks on one worker hash to one value, and CI runs this on x86-64 and AArch64. A
/// deliberate change to the dynamics moves the pin; the change that moves it says why.
/// Moved once, by ADR-0032 (from `0x7603c27186e59994`): the block's bytes changed (the apical
/// mask into the chain word, the eligibility trace at `[56..64)`), and at a weight at the rail
/// a pairing's two terms now sum in the trace before the weight saturates, where ADR-0022
/// clipped the gain and kept the loss. The spike count did not move.
const PINNED_ARENA_HASH: u64 = 0x1724f3486c1d674e;
/// The spike count that goes with the hash: a moved hash with the same count is a change to
/// the state, a moved count a change to the dynamics.
const PINNED_SPIKE_COUNT: usize = 95;

#[test]
fn the_random_network_hashes_to_the_pinned_value_on_every_architecture() {
    let outcome = run(
        1,
        // Every field spelled out: a change to `Config::default()` must not move the pin.
        Config {
            workers: 1,
            units: 128,
            blocks: 128,
            deltas: 0,
            nodes_per_worker: 4096,
            deque_capacity: 0,
            injector_capacity: 1024,
            trace_capacity: 1 << 16,
            amendments: 0,
            modulation_baseline_q16: MODULATION_ONE_Q16,
        },
        wire_random,
        20_000,
    );
    let mut crc = Crc64::new();
    for bytes in &outcome.unit_bytes {
        crc.update(bytes);
    }
    for b in &outcome.blocks {
        crc.update(&b.encode());
    }
    for &(unit, tick) in &outcome.spikes {
        crc.update(&unit.to_le_bytes());
        crc.update(&tick.to_le_bytes());
    }
    let hash = crc.finish();
    assert_eq!(
        outcome.spikes.len(),
        PINNED_SPIKE_COUNT,
        "T-1: the spike count is {}",
        outcome.spikes.len()
    );
    assert_eq!(
        hash, PINNED_ARENA_HASH,
        "T-1: the arena hash is {hash:#018x}; a deliberate change to the dynamics restates the pin and says why"
    );
}
