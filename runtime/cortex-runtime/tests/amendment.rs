//! The policy amendment's closed loop (ADR-0031; whitepaper §6.16): propose, admit through the
//! veto gate by id, trial in two forks of the image, commit into the live policy when the forks
//! behaved alike and the cost fell, persist in the image, reload with the policy derived from
//! the committed amendments; and every way the loop refuses.

use cortex_connectome::{SECTION_AMENDMENT, SectionEntry, crc64};
use cortex_core::{STP_MAX, STP_U, spike_message, synaptic_efficacy_q16};
use cortex_ethics::{EthicalEvaluationGate, Q16_ONE};
use cortex_executive::{
    AMENDMENT_ADMITTED, AMENDMENT_COMMITTED, AMENDMENT_PROPOSED, AMENDMENT_REJECTED, GATES_ALL,
    OBJECTIVE_REHYDRATIONS, OBJECTIVE_RESIDENT_UNITS, PARAM_SWEEP_BUDGET, PARAM_SWEEP_QUIET_TICKS,
    PolicyAmendment, REJECT_NO_GAIN, REJECT_UNKNOWN_PARAMETER, REJECT_VETOED,
};
use cortex_runtime::{AmendError, Config, Executor, Image, ImageError, Policy, Trial, run_trial};
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("vcortex-amendment-tests");
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir.join(format!("{}-{name}", std::process::id()))
}

fn log_dir(name: &str) -> PathBuf {
    let dir = scratch(name);
    std::fs::create_dir_all(&dir).expect("a log directory");
    dir
}

fn config() -> Config {
    Config {
        workers: 2,
        units: 96,
        blocks: 96,
        nodes_per_worker: 4096,
        injector_capacity: 1024,
        trace_capacity: 1 << 14,
        amendments: 4,
        ..Config::default()
    }
}

/// A pseudo-random network of 96 units, one block each.
fn wire(exec: &mut Executor<64>) {
    let mut x = 0x2545_F491u32;
    let mut next = || {
        x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        x
    };
    let units = exec.units().len();
    let blocks = exec.blocks_mut();
    for block in blocks.iter_mut() {
        for slot in 0..4 {
            let target = (next() >> 8) % units as u32;
            let delay = match next() % 6 {
                0 => 0,
                _ => 1 + (next() >> 8) % 300,
            } as u16;
            let weight = ((next() >> 8) % 20_000) as i16 + 10_000;
            assert!(block.set_synapse(slot, target, weight, delay, next() % 4 == 0));
        }
    }
    let units = exec.units_mut();
    for (i, unit) in units.iter_mut().enumerate() {
        unit.synapse_slab_idx = i as u32 + 1;
    }
}

/// Fourteen strong basal messages fire a unit at rest ten ticks later.
fn kick(exec: &Executor<64>, unit: u32) {
    let inject = exec.injector();
    let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
    for _ in 0..14 {
        inject.inject(unit, strong).unwrap();
    }
}

/// A wired network run to a quiescent point, and its image.
fn quiescent_network() -> (Executor<64>, Vec<u8>) {
    let mut exec = Executor::<64>::new(config()).unwrap();
    wire(&mut exec);
    kick(&exec, 3);
    exec.run(3_000);
    assert!(exec.is_quiescent(), "the network rang down");
    let image = Image::encode(&exec).unwrap();
    (exec, image)
}

/// A gate that permits `id`.
fn permitting(id: u32) -> EthicalEvaluationGate {
    let mut gate = EthicalEvaluationGate {
        proposal_action_id: id,
        harm_threshold_q16: Q16_ONE / 4,
        authorization_level: 1,
        ..Default::default()
    };
    assert!(!gate.evaluate(0, 1));
    assert!(gate.is_permitted());
    gate
}

#[test]
fn the_default_policy_is_the_documented_one_and_its_values_are_registered() {
    let p = Policy::default();
    assert_eq!((p.sweep_quiet_ticks, p.sweep_budget), (10_000, 1_024));
    assert_eq!(p.value(PARAM_SWEEP_QUIET_TICKS), Some(10_000));
    assert_eq!(p.value(PARAM_SWEEP_BUDGET), Some(1_024));
    assert_eq!(p.value(0), None);
    assert_eq!(p.value(9), None);
    let mut q = p;
    assert!(
        q.set(PARAM_SWEEP_QUIET_TICKS, 0),
        "the lower bound is inside"
    );
    assert!(
        q.set(PARAM_SWEEP_BUDGET, i32::MAX),
        "the upper bound is inside"
    );
    assert_eq!((q.sweep_quiet_ticks, q.sweep_budget), (0, i32::MAX as u32));
    assert_eq!(q.value(PARAM_SWEEP_BUDGET), Some(i32::MAX));
    assert!(!q.set(PARAM_SWEEP_QUIET_TICKS, -1), "outside the bounds");
    assert!(!q.set(9, 1), "unregistered");
    assert_eq!(
        q.value(PARAM_SWEEP_QUIET_TICKS),
        Some(0),
        "a refused set changes nothing"
    );
    let saturated = Policy {
        sweep_quiet_ticks: u32::MAX,
        sweep_budget: u32::MAX,
    };
    assert_eq!(saturated.value(PARAM_SWEEP_QUIET_TICKS), Some(i32::MAX));
    assert_eq!(saturated.value(PARAM_SWEEP_BUDGET), Some(i32::MAX));
    let exec = Executor::<64>::new(config()).unwrap();
    assert_eq!(exec.policy(), Policy::default());
    assert_eq!(exec.amendment_room(), 4);
    assert!(exec.amendments().is_empty());
}

#[test]
fn a_proposal_takes_a_slot_with_the_live_value_and_the_tick_and_the_arena_fills() {
    let mut exec = Executor::<64>::new(config()).unwrap();
    exec.run(7);
    let i = exec
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(i, 0);
    let a = exec.amendments()[0];
    assert_eq!((a.amendment_id, a.proposed_tick), (1, 7));
    assert_eq!((a.current_value, a.proposed_value), (10_000, 100));
    assert_eq!(a.status, AMENDMENT_PROPOSED);
    assert_eq!(exec.amendment_room(), 3);
    let rejected = exec.propose(9, 100, OBJECTIVE_RESIDENT_UNITS, 0).unwrap();
    assert_eq!(rejected, 1, "a rejected proposal is a record too");
    assert_eq!(exec.amendments()[1].status, AMENDMENT_REJECTED);
    assert_eq!(exec.amendments()[1].reason, REJECT_UNKNOWN_PARAMETER);
    assert_eq!(
        exec.amendments()[1].current_value,
        0,
        "no live value for an unknown parameter"
    );
    exec.propose(PARAM_SWEEP_BUDGET, 1, OBJECTIVE_REHYDRATIONS, 1)
        .unwrap();
    exec.propose(PARAM_SWEEP_BUDGET, 2, OBJECTIVE_REHYDRATIONS, 1)
        .unwrap();
    assert_eq!(exec.amendment_room(), 0);
    assert_eq!(
        exec.propose(PARAM_SWEEP_BUDGET, 3, OBJECTIVE_REHYDRATIONS, 1),
        Err(AmendError::ArenaFull)
    );
    assert_eq!(exec.amendments().len(), 4);
    let none = Executor::<64>::new(Config {
        amendments: 0,
        ..config()
    })
    .unwrap();
    let mut none = none;
    assert_eq!(
        none.propose(PARAM_SWEEP_BUDGET, 3, OBJECTIVE_REHYDRATIONS, 1),
        Err(AmendError::ArenaFull),
        "no room is no self-amendment"
    );
}

#[test]
fn the_veto_gate_admits_by_id_only_and_a_closed_gate_admits_nothing() {
    let mut exec = Executor::<64>::new(config()).unwrap();
    let i = exec
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    let other = permitting(2);
    assert_eq!(exec.admit(i, &other), Err(AmendError::WrongProposal));
    assert_eq!(exec.amendments()[i].status, AMENDMENT_PROPOSED, "unchanged");
    let unevaluated = EthicalEvaluationGate {
        proposal_action_id: 1,
        ..Default::default()
    };
    assert_eq!(
        exec.admit(i, &unevaluated),
        Ok(false),
        "a gate not evaluated is closed"
    );
    assert_eq!(exec.amendments()[i].status, AMENDMENT_REJECTED);
    assert_eq!(exec.amendments()[i].reason, REJECT_VETOED);
    let j = exec
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(exec.admit(j, &permitting(2)), Ok(true));
    assert_eq!(exec.amendments()[j].status, AMENDMENT_ADMITTED);
    assert_eq!(
        exec.admit(9, &permitting(10)),
        Err(AmendError::NoSuchAmendment)
    );
    assert_eq!(
        exec.record_trial(9, 1, 1, 1, 1, 0),
        Err(AmendError::NoSuchAmendment)
    );
    assert_eq!(exec.commit(9), Err(AmendError::NoSuchAmendment));
    assert_eq!(
        exec.commit(j),
        Err(AmendError::NotCommittable),
        "not yet trialled"
    );
}

#[test]
fn a_shorter_quiet_bound_frees_memory_without_changing_behaviour_and_is_committed() {
    let (mut live, image) = quiescent_network();
    let i = live
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(live.admit(i, &permitting(1)), Ok(true));
    let mut amendment = live.amendments()[i];
    let dir = log_dir("shorter");
    let trial = Trial {
        image: &image,
        config: Config {
            amendments: 0,
            ..config()
        },
        injections: &[],
        ticks: 2_000,
        sweep_every: 500,
        log_dir: &dir,
    };
    let report = run_trial::<64>(&trial, &mut amendment).unwrap();
    assert!(report.may_commit, "{report:?}");
    assert_eq!(
        report.baseline.behaviour_hash, report.candidate.behaviour_hash,
        "the sweep is transparent (ADR-0024)"
    );
    assert_eq!(
        report.baseline.resident_units, 96,
        "quiet for 10 000 ticks: nothing evicted"
    );
    assert!(
        report.candidate.resident_units < 96,
        "quiet for 100 ticks: units evicted"
    );
    assert_eq!(
        report.candidate.evictions as u32,
        96 - report.candidate.resident_units
    );
    assert_eq!(
        (report.baseline.rehydrations, report.candidate.rehydrations),
        (0, 0)
    );
    assert_eq!(amendment.gates, GATES_ALL);
    assert_eq!(amendment.trial_ticks, 2_000);
    assert_eq!(
        (amendment.baseline_cost, amendment.candidate_cost),
        (96, report.candidate.resident_units)
    );
    // The live executor learns the result and commits between ticks.
    assert_eq!(
        live.record_trial(
            i,
            amendment.trial_ticks,
            amendment.baseline_hash,
            amendment.candidate_hash,
            amendment.baseline_cost,
            amendment.candidate_cost
        ),
        Ok(true)
    );
    live.run(5);
    assert_eq!(live.commit(i), Ok(()));
    let committed = live.amendments()[i];
    assert_eq!(committed.status, AMENDMENT_COMMITTED);
    assert_eq!(committed.committed_tick, 3_005);
    assert_eq!(
        live.policy().sweep_quiet_ticks,
        100,
        "the live policy moved"
    );
    assert_eq!(live.policy().sweep_budget, 1_024, "and nothing else did");
    assert_eq!(live.commit(i), Err(AmendError::NotCommittable), "once");
    // The live sweep now runs under the amended policy.
    live.attach_log(&scratch("live.wal")).unwrap();
    // Every unit at rest is quiet for more than 100 ticks; the kicked unit's threshold is
    // still decaying toward its base, so it is not at rest and stays.
    let swept = live.sweep_by_policy().unwrap();
    assert_eq!(swept, 95, "every unit at rest");
    assert_eq!(live.evictions(), 95);
    assert!(
        !live.is_evicted(3),
        "the kicked unit is the one still settling"
    );
}

#[test]
fn a_bound_that_evicts_active_tissue_costs_rehydrations_and_is_rejected_for_no_gain() {
    let (mut live, image) = quiescent_network();
    let i = live
        .propose(PARAM_SWEEP_QUIET_TICKS, 0, OBJECTIVE_REHYDRATIONS, 0)
        .unwrap();
    assert_eq!(live.admit(i, &permitting(1)), Ok(true));
    let mut amendment = live.amendments()[i];
    let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
    // Kicks on a cadence that outlasts every sweep: evicted units get messages.
    let mut injections = Vec::new();
    for round in 0..8u64 {
        for _ in 0..14 {
            injections.push((round * 200 + 50, 3 + round as u32, strong));
        }
    }
    let dir = log_dir("active");
    let trial = Trial {
        image: &image,
        config: Config {
            amendments: 0,
            ..config()
        },
        injections: &injections,
        ticks: 2_000,
        sweep_every: 100,
        log_dir: &dir,
    };
    let report = run_trial::<64>(&trial, &mut amendment).unwrap();
    assert!(!report.may_commit);
    assert_eq!(
        report.baseline.behaviour_hash, report.candidate.behaviour_hash,
        "evict, spike, re-hydrate is still transparent"
    );
    assert_eq!(
        report.baseline.rehydrations, 0,
        "the baseline never evicts within 2 000 ticks"
    );
    assert!(report.candidate.rehydrations > 0, "{report:?}");
    assert_eq!(amendment.status, AMENDMENT_REJECTED);
    assert_eq!(amendment.reason, REJECT_NO_GAIN);
    assert_eq!(
        live.record_trial(
            i,
            amendment.trial_ticks,
            amendment.baseline_hash,
            amendment.candidate_hash,
            amendment.baseline_cost,
            amendment.candidate_cost
        ),
        Ok(false)
    );
    assert_eq!(live.commit(i), Err(AmendError::NotCommittable));
    assert_eq!(live.policy(), Policy::default(), "nothing moved");
}

#[test]
fn a_trial_needs_an_admitted_amendment_and_an_empty_trial_shows_nothing() {
    let (_, image) = quiescent_network();
    let dir = log_dir("unadmitted");
    let trial = Trial {
        image: &image,
        config: config(),
        injections: &[],
        ticks: 10,
        sweep_every: 0,
        log_dir: &dir,
    };
    let mut proposed = PolicyAmendment::propose(
        1,
        0,
        PARAM_SWEEP_QUIET_TICKS,
        10_000,
        100,
        OBJECTIVE_RESIDENT_UNITS,
        0,
    );
    let before = proposed;
    let report = run_trial::<64>(&trial, &mut proposed).unwrap();
    assert!(!report.may_commit);
    assert_eq!(proposed, before, "not admitted: nothing recorded");
    let mut admitted = before;
    admitted.admit(true);
    let empty = Trial {
        ticks: 0,
        ..trial.clone()
    };
    let report = run_trial::<64>(&empty, &mut admitted).unwrap();
    assert!(!report.may_commit);
    assert_eq!(admitted.status, AMENDMENT_REJECTED);
    assert_eq!(
        report.baseline.behaviour_hash, report.candidate.behaviour_hash,
        "two forks of one image that ran nothing"
    );
    // A trial cannot set a value the policy refuses; the amendment is left as it was.
    let mut forged = PolicyAmendment::propose(
        1,
        0,
        PARAM_SWEEP_QUIET_TICKS,
        10_000,
        100,
        OBJECTIVE_RESIDENT_UNITS,
        0,
    );
    forged.admit(true);
    forged.parameter = 9;
    let before = forged;
    let report = run_trial::<64>(&trial, &mut forged).unwrap();
    assert!(!report.may_commit);
    assert_eq!(forged, before);
    // A corrupt image is refused before any fork runs.
    let mut bad = image.clone();
    bad[100] ^= 1;
    let mut fresh = before;
    fresh.parameter = PARAM_SWEEP_QUIET_TICKS;
    let corrupt = Trial {
        image: &bad,
        ..trial.clone()
    };
    assert!(run_trial::<64>(&corrupt, &mut fresh).is_err());
}

#[test]
fn a_commit_is_refused_when_the_live_value_moved_since_the_trial() {
    let mut exec = Executor::<64>::new(config()).unwrap();
    let a = exec
        .propose(PARAM_SWEEP_BUDGET, 10, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    let b = exec
        .propose(PARAM_SWEEP_BUDGET, 20, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(exec.admit(a, &permitting(1)), Ok(true));
    assert_eq!(exec.admit(b, &permitting(2)), Ok(true));
    assert_eq!(exec.record_trial(a, 10, 5, 5, 96, 90), Ok(true));
    assert_eq!(exec.record_trial(b, 10, 5, 5, 96, 80), Ok(true));
    assert_eq!(exec.commit(a), Ok(()));
    assert_eq!(exec.policy().sweep_budget, 10);
    assert_eq!(
        exec.commit(b),
        Err(AmendError::Stale),
        "b was trialled from 1 024, and the live value is 10"
    );
    assert_eq!(exec.policy().sweep_budget, 10, "nothing moved");
    assert_eq!(
        exec.amendments()[b].status,
        cortex_executive::AMENDMENT_TRIALLED
    );
}

#[test]
fn committed_amendments_persist_in_the_image_and_the_loader_derives_the_policy() {
    let (mut live, _) = quiescent_network();
    let a = live
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(live.admit(a, &permitting(1)), Ok(true));
    assert_eq!(live.record_trial(a, 2_000, 7, 7, 96, 10), Ok(true));
    assert_eq!(live.commit(a), Ok(()));
    let b = live
        .propose(PARAM_SWEEP_QUIET_TICKS, 50, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(live.admit(b, &permitting(2)), Ok(true));
    assert_eq!(live.record_trial(b, 2_000, 7, 7, 10, 5), Ok(true));
    assert_eq!(live.commit(b), Ok(()));
    let vetoed = live
        .propose(PARAM_SWEEP_BUDGET, 7, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(
        live.admit(vetoed, &EthicalEvaluationGate::default()),
        Err(AmendError::WrongProposal)
    );
    let closed = EthicalEvaluationGate {
        proposal_action_id: 3,
        ..Default::default()
    };
    assert_eq!(live.admit(vetoed, &closed), Ok(false));
    let pending = live
        .propose(PARAM_SWEEP_BUDGET, 8, OBJECTIVE_REHYDRATIONS, 3)
        .unwrap();
    assert_eq!(live.policy().sweep_quiet_ticks, 50);
    let image = Image::encode(&live).unwrap();
    let loaded = Image::decode::<64>(
        &image,
        Config {
            amendments: 2,
            ..config()
        },
    )
    .unwrap();
    assert_eq!(
        loaded.policy().sweep_quiet_ticks,
        50,
        "derived from the committed amendments"
    );
    assert_eq!(loaded.policy().sweep_budget, 1_024);
    assert_eq!(loaded.amendments(), live.amendments());
    assert_eq!(loaded.amendments().len(), 4);
    assert_eq!(loaded.amendments()[pending].status, AMENDMENT_PROPOSED);
    assert_eq!(
        loaded.amendment_room(),
        2,
        "the image's records plus the configured headroom"
    );
    assert_eq!(
        Image::encode(&loaded).unwrap(),
        image,
        "the round trip is byte-identical"
    );
    let empty = Executor::<64>::new(config()).unwrap();
    let without = Image::encode(&empty).unwrap();
    let reloaded = Image::decode::<64>(&without, config()).unwrap();
    assert_eq!(
        reloaded.amendment_room(),
        4,
        "no section: the configured room alone"
    );
    assert_eq!(reloaded.policy(), Policy::default());
}

/// The image's amendment section entry and the offset of its bytes.
fn amendment_section(image: &[u8]) -> (usize, SectionEntry) {
    let count = u32::from_le_bytes(image[56..60].try_into().unwrap()) as usize;
    for i in 0..count {
        let start = 64 + 64 * i;
        let entry = SectionEntry::decode(image[start..start + 64].try_into().unwrap());
        if entry.kind == SECTION_AMENDMENT {
            return (start, entry);
        }
    }
    panic!("no amendment section");
}

/// Re-seals the amendment section after `mutate` touched its bytes.
fn tamper(image: &[u8], mutate: impl FnOnce(&mut [u8])) -> Vec<u8> {
    let mut bytes = image.to_vec();
    let (entry_at, entry) = amendment_section(&bytes);
    let (offset, length) = (entry.offset as usize, entry.length as usize);
    mutate(&mut bytes[offset..offset + length]);
    let resealed = SectionEntry::new(
        entry.kind,
        entry.record_size,
        entry.offset,
        entry.length,
        crc64(&bytes[offset..offset + length]),
    );
    bytes[entry_at..entry_at + 64].copy_from_slice(&resealed.encode());
    bytes
}

#[test]
fn the_loader_refuses_an_amendment_it_could_not_have_written() {
    let (mut live, _) = quiescent_network();
    let a = live
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(live.admit(a, &permitting(1)), Ok(true));
    assert_eq!(live.record_trial(a, 2_000, 7, 7, 96, 10), Ok(true));
    assert_eq!(live.commit(a), Ok(()));
    let b = live
        .propose(PARAM_SWEEP_QUIET_TICKS, 50, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(live.admit(b, &permitting(2)), Ok(true));
    assert_eq!(live.record_trial(b, 2_000, 7, 7, 10, 5), Ok(true));
    assert_eq!(live.commit(b), Ok(()));
    let image = Image::encode(&live).unwrap();
    assert!(Image::decode::<64>(&image, config()).is_ok());
    let refused = |bytes: &[u8], what: &str| match Image::decode::<64>(bytes, config()) {
        Err(ImageError::MalformedAmendment(i)) => i,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    };
    // A reserved byte.
    let reserved = tamper(&image, |s| s[63] = 1);
    assert_eq!(refused(&reserved, "reserved"), 0);
    // A gate bit missing from a committed record.
    let gate = tamper(&image, |s| s[55] &= !2);
    assert_eq!(refused(&gate, "gate"), 0);
    // A committed record whose forks differed.
    let differed = tamper(&image, |s| s[64] ^= 1);
    assert_eq!(refused(&differed, "hash"), 1);
    // A committed value outside the bounds: the sign bit of the proposed value.
    let out = tamper(&image, |s| s[35] |= 0x80);
    assert_eq!(refused(&out, "bounds"), 0);
    // Out of order: the second record's id.
    let order = tamper(&image, |s| s[64 + 16] = 9);
    assert_eq!(refused(&order, "order"), 1);
    // A commit that does not follow the one before it: the second started from 10, the
    // first's proposed value; make it start from 20.
    let stale = tamper(&image, |s| s[64 + 28] = 20);
    assert_eq!(refused(&stale, "stale"), 1);
    // Two commits with the same starting value: the second's start is right, the first's is
    // not the default.
    let first = tamper(&image, |s| {
        s[28..32].copy_from_slice(&5_000i32.to_le_bytes())
    });
    assert_eq!(refused(&first, "first"), 0);
    // An unknown status.
    let status = tamper(&image, |s| s[52] = 6);
    assert_eq!(refused(&status, "status"), 0);
    // An empty slot where a record should be.
    let empty = tamper(&image, |s| s[0..64].fill(0));
    assert_eq!(refused(&empty, "empty"), 0);
    // The section with a record size other than 64 is a malformed directory.
    let mut size = image.clone();
    let (entry_at, entry) = amendment_section(&size);
    let wrong = SectionEntry::new(entry.kind, 32, entry.offset, entry.length, entry.crc64);
    size[entry_at..entry_at + 64].copy_from_slice(&wrong.encode());
    assert!(matches!(
        Image::decode::<64>(&size, config()),
        Err(ImageError::Directory(SECTION_AMENDMENT))
    ));
    // No room: the image's two records fit, and the configured room is added on top.
    let tight = Image::decode::<64>(
        &image,
        Config {
            amendments: 0,
            ..config()
        },
    )
    .unwrap();
    assert_eq!(tight.amendment_room(), 0);
    assert_eq!(tight.amendments().len(), 2);
}
