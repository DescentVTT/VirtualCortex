//! Milestone M3's exit test: a three-neuron delayed oscillator whose period is exact to the
//! tick (whitepaper Appendix C; brief 013; ADR-0022). Single-threaded: one wheel, one mailbox
//! arena, three units in a ring, four synapse blocks per unit (thirteen synapses per hop, the
//! last block holding one). Every
//! tick advances the wheel, delivers the due tokens into mailboxes as spike messages, gives
//! each unit its turn in index order (claim the turn, drain the mailbox, order the batch by
//! message value, integrate, end the turn) and fans a spike out through the unit's chain:
//! zero-delay synapses into mailboxes now, the rest into the wheel as synapse tokens. Short-term
//! plasticity and STDP are not stepped here, so the drive is constant and the steady orbit is
//! periodic; both have their own unit tests.

use cortex_core::{
    DendriticSuperNeuron, FlatTimingWheel, GateState, MailboxNode, STP_MAX, STP_U, SynapseBlock,
    THRESHOLD_BASE, message_efficacy_q16, message_is_apical, spike_message, synapse_token,
    token_block, token_slot,
};

const UNITS: usize = 3;
/// Synapses from each unit to the next. At the rested short-term-plasticity factors each releases
/// 0.198, so a hop delivers 2.58 into the basal compartment: enough to cross the 1.0 threshold
/// (a single arrival peaks the soma near 0.47 of the input) and little enough that the charge
/// left after the 200-tick refractory window does not fire the unit again (which 15 do).
const SYNAPSES_PER_HOP: usize = 13;
const BLOCKS_PER_UNIT: usize = SYNAPSES_PER_HOP.div_ceil(4);
const NODES: usize = 64;
const CYCLES: usize = 200;
const STEADY_CYCLES: usize = 100;

struct Ring {
    units: Vec<DendriticSuperNeuron>,
    blocks: Vec<SynapseBlock>,
    nodes: Vec<MailboxNode>,
    free: Vec<u32>,
    wheel: Box<FlatTimingWheel<64>>,
    /// Ticks at which each unit fired.
    spikes: Vec<Vec<u32>>,
    /// Ticks at which a message was pushed into each unit's mailbox.
    arrivals: Vec<Vec<u32>>,
}

impl Ring {
    fn new(delays: [u16; UNITS]) -> Self {
        let mut units: Vec<_> = (0..UNITS as u64).map(DendriticSuperNeuron::new).collect();
        let mut blocks = vec![SynapseBlock::new(); BLOCKS_PER_UNIT * UNITS + 1];
        for (i, unit) in units.iter_mut().enumerate() {
            let target = ((i + 1) % UNITS) as u32;
            let base = BLOCKS_PER_UNIT * i;
            for k in 0..SYNAPSES_PER_HOP {
                assert!(blocks[base + k / 4].set_synapse(
                    k % 4,
                    target,
                    i16::MAX,
                    delays[i],
                    false
                ));
            }
            for j in 0..BLOCKS_PER_UNIT - 1 {
                assert!(blocks[base + j].link((base + j + 1) as u32));
            }
            unit.v_thresh = THRESHOLD_BASE;
            assert!(unit.set_first_block(base as u32));
        }
        let mut ring = Self {
            units,
            blocks,
            nodes: (0..NODES).map(|_| MailboxNode::new()).collect(),
            free: (0..NODES as u32).rev().collect(),
            wheel: Box::new(FlatTimingWheel::new()),
            spikes: vec![Vec::new(); UNITS],
            arrivals: vec![Vec::new(); UNITS],
        };
        // The kick: one hop's worth of messages into unit 0, before the first tick.
        let one = cortex_core::synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
        for _ in 0..SYNAPSES_PER_HOP {
            ring.push(0, spike_message(one, false), 0);
        }
        ring
    }

    fn push(&mut self, target: usize, message: u32, now: u32) {
        let node = self.free.pop().expect("a free mailbox node");
        assert!(self.units[target].mailbox_push(&self.nodes, node, message));
        // The deque is the gate itself here: a scheduled unit is served on its next turn.
        let _ = self.units[target].try_schedule();
        self.arrivals[target].push(now);
    }

    fn tick(&mut self) {
        let due: Vec<u32> = self.wheel.advance().to_vec();
        let now = self.wheel.tick() as u32;
        for token in due {
            let (b, s) = (token_block(token) as usize, token_slot(token) as usize);
            let block = self.blocks[b];
            let target = block.target(s).expect("a scheduled synapse has a target") as usize;
            let message = spike_message(block.last_release_q16[s], block.is_apical(s));
            self.push(target, message, now);
        }
        for i in 0..UNITS {
            let (mut basal, mut apical) = (0i32, 0i32);
            let took_turn =
                self.units[i].gate() == Some(GateState::Scheduled) && self.units[i].begin_turn();
            if took_turn {
                let mut batch: Vec<u32> = Vec::new();
                for (node, payload) in self.units[i].mailbox_drain(&self.nodes) {
                    batch.push(payload);
                    self.free.push(node);
                }
                // §8.3: the batch is ordered by message value, never by arrival.
                batch.sort_unstable();
                for m in batch {
                    let e = message_efficacy_q16(m);
                    if message_is_apical(m) {
                        apical = apical.saturating_add(e);
                    } else {
                        basal = basal.saturating_add(e);
                    }
                }
            }
            let fired = self.units[i].integrate(basal, apical, now);
            if took_turn {
                assert!(
                    !self.units[i].end_turn(),
                    "nothing arrives during a single-threaded turn"
                );
            }
            if fired {
                self.spikes[i].push(now);
                self.fan_out(i, now);
            }
        }
    }

    /// R-1 step 6: the chain is walked, each block releases under the unit's (rested)
    /// short-term-plasticity factors, and each synapse is delivered now or scheduled.
    fn fan_out(&mut self, i: usize, now: u32) {
        let chain: Vec<u32> = self.units[i].chain(&self.blocks).collect();
        for b in chain {
            let released = self.blocks[b as usize].release_all(STP_U, STP_MAX);
            let block = self.blocks[b as usize];
            for (slot, &release) in released.iter().enumerate() {
                // Every slot of every block: the walk is what `fan_out` does for a reader.
                let Some(target) = block.target(slot) else {
                    continue;
                };
                let delay = block.delays_ticks[slot];
                if delay == 0 {
                    let message = spike_message(release, block.is_apical(slot));
                    self.push(target as usize, message, now);
                } else {
                    let token = synapse_token(b, slot as u8).expect("a small arena");
                    self.wheel
                        .schedule(delay as u32, token)
                        .expect("within the horizon and the slot capacity");
                }
            }
        }
    }

    fn run(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.tick();
        }
    }
}

fn intervals(ticks: &[u32]) -> Vec<u32> {
    ticks.windows(2).map(|w| w[1] - w[0]).collect()
}

/// The latency of each of the last `STEADY_CYCLES` spikes of a unit: the spike tick minus the
/// tick of the latest arrival at or before it.
fn latencies(spikes: &[u32], arrivals: &[u32]) -> Vec<u32> {
    let mut unique = arrivals.to_vec();
    unique.dedup();
    spikes[spikes.len() - STEADY_CYCLES..]
        .iter()
        .map(|&s| {
            let arrival = unique
                .iter()
                .rev()
                .find(|&&a| a <= s)
                .expect("an arrival precedes every spike");
            s - arrival
        })
        .collect()
}

/// Runs the ring for `CYCLES` cycles and returns the steady period, asserting that it is exact
/// to the tick over the last `STEADY_CYCLES` cycles on every unit, that it is the delays plus the
/// three integration latencies, and that a second ring reproduces the same spike train.
fn exact_period(delays: [u16; UNITS]) -> u32 {
    let sum: u32 = delays.iter().map(|&d| d as u32).sum();
    let ticks = CYCLES as u32 * (sum + 3 * 64);
    let mut ring = Ring::new(delays);
    ring.run(ticks);
    for i in 0..UNITS {
        assert!(
            ring.spikes[i].len() > STEADY_CYCLES + 1,
            "unit {i} fired {} times in {ticks} ticks",
            ring.spikes[i].len()
        );
    }
    let period = {
        let p = intervals(&ring.spikes[0]);
        let steady = &p[p.len() - STEADY_CYCLES..];
        assert!(
            steady.iter().all(|&x| x == steady[0]),
            "unit 0's periods over the last {STEADY_CYCLES} cycles: {steady:?}"
        );
        steady[0]
    };
    let mut total_latency = 0;
    for i in 0..UNITS {
        let p = intervals(&ring.spikes[i]);
        assert!(
            p[p.len() - STEADY_CYCLES..].iter().all(|&x| x == period),
            "unit {i} keeps the period {period}: {:?}",
            &p[p.len() - STEADY_CYCLES..]
        );
        let l = latencies(&ring.spikes[i], &ring.arrivals[i]);
        assert!(
            l.iter().all(|&x| x == l[0]),
            "unit {i}'s integration latency is the same every cycle: {l:?}"
        );
        assert!(
            l[0] > 0,
            "a unit integrates for at least one tick before it fires"
        );
        total_latency += l[0];
    }
    assert_eq!(
        period,
        sum + total_latency,
        "the period is the three delays plus the three integration latencies"
    );
    assert!(period > cortex_core::REFRACTORY_TICKS as u32);

    let mut again = Ring::new(delays);
    again.run(ticks);
    assert_eq!(
        again.spikes, ring.spikes,
        "the same ring twice is the same spike train"
    );
    assert!(ring.free.len() == NODES, "every mailbox node returned");
    period
}

#[test]
fn a_ring_with_delays_in_both_wheel_rings_oscillates_with_an_exact_period() {
    let period = exact_period([300, 500, 700]);
    assert_eq!(
        period,
        1500 + 3 * 9,
        "observed: nine ticks of integration per hop"
    );
}

#[test]
fn a_ring_at_the_horizon_and_in_the_first_fine_slot_oscillates_with_an_exact_period() {
    let period = exact_period([2559, 1, 1200]);
    assert_eq!(period, 3760 + 3 * 11);
}

#[test]
fn a_ring_with_a_zero_delay_hop_through_the_mailbox_oscillates_with_an_exact_period() {
    let period = exact_period([0, 1500, 900]);
    assert_eq!(
        period,
        2400 + 3 * 11,
        "the zero-delay hop lands in the same tick, since its target's turn comes later"
    );
}
