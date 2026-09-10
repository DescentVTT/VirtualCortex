//! The policy amendment's closed loop (ADR-0031; whitepaper §6.16): propose, admit through the
//! veto gate by id, trial in two forks of the image, commit into the live policy when the forks
//! behaved alike and the cost fell, persist in the image, reload with the policy derived from
//! the committed amendments; and every way the loop refuses.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{SECTION_AMENDMENT, SectionEntry, crc64};
use cortex_core::{STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16};
use cortex_ethics::{EthicalEvaluationGate, Q16_ONE};
use cortex_executive::{
    AMENDMENT_ADMITTED, AMENDMENT_COMMITTED, AMENDMENT_PROPOSED, AMENDMENT_REJECTED,
    AMENDMENT_TRIALLED, GATES_ALL, OBJECTIVE_REHYDRATIONS, OBJECTIVE_RESIDENT_UNITS,
    PARAM_SWEEP_BUDGET, PARAM_SWEEP_QUIET_TICKS, REJECT_EMPTY_TRIAL, REJECT_NO_GAIN,
    REJECT_UNKNOWN_PARAMETER, REJECT_VETOED,
};
use cortex_runtime::{
    ACTIVATE, AmendError, Config, Executor, Image, ImageError, InjectError, Policy, Trial,
    run_trial,
};
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

/// The forks' configuration: no room for proposals of their own.
fn fork_config() -> Config {
    Config {
        amendments: 0,
        ..config()
    }
}

/// A pseudo-random network of 96 units, one block each, every unit armed to fire.
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
            let target = (next() >> 8)
                .checked_rem(units as u32)
                .expect("the arena holds a unit");
            let delay = match next() % 6 {
                0 => 0,
                _ => ((next() >> 8) % 300).wrapping_add(1),
            } as u16;
            let weight = (((next() >> 8) % 20_000) as i16).wrapping_add(10_000);
            assert!(block.set_synapse(slot, target, weight, delay, next() % 4 == 0));
        }
    }
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        assert!(unit.set_first_block(i as u32));
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
}

/// A wired network that has ticked but never spiked, so every unit is at rest with its
/// threshold at its base, and its image at that quiescent point.
fn quiescent_network() -> (Executor<64>, Vec<u8>) {
    let mut exec = Executor::<64>::new(config()).unwrap();
    wire(&mut exec);
    exec.run(3_000);
    assert!(exec.is_quiescent(), "nothing is in flight");
    let image = Image::encode(&exec).unwrap();
    (exec, image)
}

/// One strong basal message: fourteen in one tick fire a unit at rest ten ticks later.
fn strong() -> u32 {
    spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false)
}

/// Kicks on a cadence that outlasts every sweep, each round a different unit, plus a bare
/// activation of another (a turn without a message); the network is quiet again by 3 000.
fn active_injections() -> Vec<(u64, u32, u32)> {
    let mut injections = Vec::new();
    for round in 0..8u64 {
        let at = round.wrapping_mul(200);
        for _ in 0..14 {
            injections.push((
                at.wrapping_add(50),
                (round as u32).wrapping_add(3),
                strong(),
            ));
        }
        injections.push((
            at.wrapping_add(60),
            (round as u32).wrapping_add(20),
            ACTIVATE,
        ));
    }
    injections
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

/// Proposes and admits a change; the arena index.
fn admitted(live: &mut Executor<64>, parameter: u16, value: i32, objective: u8) -> usize {
    let i = live.propose(parameter, value, objective, 0).unwrap();
    let id = live.amendments()[i].amendment_id;
    assert_eq!(live.admit(i, &permitting(id)), Ok(true));
    i
}

/// A trial with no injections that sweeps every 500 ticks for 2 000.
fn quiet_trial<'a>(image: &'a [u8], dir: &'a PathBuf) -> Trial<'a> {
    Trial {
        image,
        config: fork_config(),
        injections: &[],
        ticks: 2_000,
        sweep_every: 500,
        log_dir: dir,
    }
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
        Err(AmendError::ArenaFull),
        "the configured room, not the allocation's"
    );
    assert_eq!(exec.amendments().len(), 4);
    let mut none = Executor::<64>::new(Config {
        amendments: 0,
        ..config()
    })
    .unwrap();
    assert_eq!(none.amendment_room(), 0);
    assert_eq!(
        none.propose(PARAM_SWEEP_BUDGET, 3, OBJECTIVE_REHYDRATIONS, 1),
        Err(AmendError::ArenaFull),
        "no room is no self-amendment"
    );
}

#[test]
fn the_veto_gate_admits_a_proposed_amendment_by_id_only_and_a_closed_gate_admits_nothing() {
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
    assert_eq!(
        exec.admit(i, &permitting(1)),
        Err(AmendError::NotProposed),
        "a rejected amendment is not proposed"
    );
    let j = exec
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert_eq!(exec.admit(j, &permitting(2)), Ok(true));
    assert_eq!(exec.amendments()[j].status, AMENDMENT_ADMITTED);
    assert_eq!(
        exec.admit(j, &permitting(2)),
        Err(AmendError::NotProposed),
        "an admitted amendment is not proposed"
    );
    assert_eq!(
        exec.admit(9, &permitting(10)),
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
    let i = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir = log_dir("shorter");
    let report = run_trial(&mut live, i, &quiet_trial(&image, &dir)).unwrap();
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
        report.candidate.evictions,
        96 - report.candidate.resident_units
    );
    assert_eq!(
        (report.baseline.rehydrations, report.candidate.rehydrations),
        (0, 0)
    );
    assert_eq!((report.baseline.spikes, report.candidate.spikes), (0, 0));
    let amendment = live.amendments()[i];
    assert_eq!(
        (amendment.status, amendment.gates),
        (AMENDMENT_TRIALLED, GATES_ALL)
    );
    assert_eq!(amendment.trial_ticks, 2_000);
    assert_eq!(
        (amendment.baseline_cost, amendment.candidate_cost),
        (96, report.candidate.resident_units)
    );
    assert_eq!(
        (amendment.baseline_hash, amendment.candidate_hash),
        (
            report.baseline.behaviour_hash,
            report.candidate.behaviour_hash
        )
    );
    // The live executor commits between ticks.
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
    let swept = live.sweep_by_policy().unwrap();
    assert_eq!(
        swept, 96,
        "every unit is at rest and quiet for more than 100 ticks"
    );
    assert_eq!(live.evictions(), 96);
}

#[test]
fn a_bound_that_evicts_active_tissue_costs_rehydrations_and_is_rejected_for_no_gain() {
    let (mut live, image) = quiescent_network();
    let i = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        0,
        OBJECTIVE_REHYDRATIONS,
    );
    let injections = active_injections();
    let dir = log_dir("active");
    let trial = Trial {
        image: &image,
        config: fork_config(),
        injections: &injections,
        ticks: 3_000,
        sweep_every: 100,
        log_dir: &dir,
    };
    let report = run_trial(&mut live, i, &trial).unwrap();
    assert!(!report.may_commit);
    assert_eq!(
        report.baseline.behaviour_hash, report.candidate.behaviour_hash,
        "evict, spike, re-hydrate is still transparent"
    );
    assert_eq!(
        report.baseline.rehydrations, 0,
        "the baseline never evicts within 3 000 ticks"
    );
    assert!(report.candidate.rehydrations > 0, "{report:?}");
    assert!(report.baseline.spikes > 0, "the kicks fire: {report:?}");
    assert_eq!(report.baseline.spikes, report.candidate.spikes);
    let amendment = live.amendments()[i];
    assert_eq!(amendment.status, AMENDMENT_REJECTED);
    assert_eq!(amendment.reason, REJECT_NO_GAIN);
    assert_eq!(
        (amendment.baseline_cost, amendment.candidate_cost),
        (report.baseline.rehydrations, report.candidate.rehydrations),
        "the objective's cost is the re-hydration count"
    );
    assert_eq!(live.commit(i), Err(AmendError::NotCommittable));
    assert_eq!(live.policy(), Policy::default(), "nothing moved");
}

#[test]
fn a_fork_sweeps_after_every_sweep_every_ticks_and_never_with_a_zero_cadence() {
    let (mut live, image) = quiescent_network();
    // No cadence: no sweep, whatever the bound.
    let never = log_dir("never");
    let a = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let report = run_trial(
        &mut live,
        a,
        &Trial {
            ticks: 600,
            sweep_every: 0,
            ..quiet_trial(&image, &never)
        },
    )
    .unwrap();
    assert_eq!(
        (report.candidate.evictions, report.candidate.resident_units),
        (0, 96)
    );
    assert!(!report.may_commit);
    assert_eq!(live.amendments()[a].reason, REJECT_NO_GAIN);
    // One tick short of the cadence: no sweep yet.
    let short = log_dir("short");
    let b = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let report = run_trial(
        &mut live,
        b,
        &Trial {
            ticks: 499,
            ..quiet_trial(&image, &short)
        },
    )
    .unwrap();
    assert_eq!(
        (report.candidate.evictions, report.candidate.resident_units),
        (0, 96)
    );
    assert_eq!(live.amendments()[b].reason, REJECT_NO_GAIN);
    // Exactly the cadence: the one sweep runs on the last tick.
    let exact = log_dir("exact");
    let c = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let report = run_trial(
        &mut live,
        c,
        &Trial {
            ticks: 500,
            ..quiet_trial(&image, &exact)
        },
    )
    .unwrap();
    assert!(report.candidate.evictions > 0, "{report:?}");
    assert_eq!(
        report.baseline.evictions, 0,
        "the baseline's bound is 10 000 ticks"
    );
    assert!(report.may_commit);
}

#[test]
fn a_trial_is_refused_before_it_runs_and_a_refused_trial_leaves_the_amendment_unchanged() {
    let (mut live, image) = quiescent_network();
    let dir = log_dir("refused");
    let trial = quiet_trial(&image, &dir);
    // Not admitted.
    let proposed = live
        .propose(PARAM_SWEEP_QUIET_TICKS, 100, OBJECTIVE_RESIDENT_UNITS, 0)
        .unwrap();
    assert!(matches!(
        run_trial(&mut live, proposed, &trial),
        Err(ImageError::Amendment(AmendError::NotAdmitted))
    ));
    assert_eq!(live.amendments()[proposed].status, AMENDMENT_PROPOSED);
    assert!(matches!(
        run_trial(&mut live, 9, &trial),
        Err(ImageError::Amendment(AmendError::NoSuchAmendment))
    ));
    // No trace: the spike train would not be hashed.
    let i = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let untraced = Trial {
        config: Config {
            trace_capacity: 0,
            ..fork_config()
        },
        ..trial.clone()
    };
    assert!(matches!(
        run_trial(&mut live, i, &untraced),
        Err(ImageError::NoTrace)
    ));
    assert_eq!(live.amendments()[i].status, AMENDMENT_ADMITTED, "unchanged");
    // A trace too small for the spikes: a dropped spike is a hash that does not cover them.
    let injections = active_injections();
    let small = Trial {
        config: Config {
            trace_capacity: 2,
            ..fork_config()
        },
        injections: &injections,
        ticks: 3_000,
        sweep_every: 100,
        ..trial.clone()
    };
    assert!(matches!(
        run_trial(&mut live, i, &small),
        Err(ImageError::NoTrace)
    ));
    assert_eq!(live.amendments()[i].status, AMENDMENT_ADMITTED);
    // A fork that ends with tokens in flight is not at a quiescent point.
    let kicks: Vec<(u64, u32, u32)> = (0..14).map(|_| (0u64, 3u32, strong())).collect();
    let mid_flight = Trial {
        injections: &kicks,
        ticks: 30,
        sweep_every: 0,
        ..trial.clone()
    };
    assert!(matches!(
        run_trial(&mut live, i, &mid_flight),
        Err(ImageError::NotQuiescent)
    ));
    assert_eq!(live.amendments()[i].status, AMENDMENT_ADMITTED);
    // An injection the fork refuses: a unit outside the arena, then a full ring.
    let outside = [(0u64, 96u32, strong())];
    let refused = Trial {
        injections: &outside,
        ..trial.clone()
    };
    assert!(matches!(
        run_trial(&mut live, i, &refused),
        Err(ImageError::Injection(InjectError::NoSuchUnit))
    ));
    let too_many: Vec<(u64, u32, u32)> = (0..2_000).map(|_| (0u64, 3u32, strong())).collect();
    let full = Trial {
        injections: &too_many,
        ..trial.clone()
    };
    assert!(matches!(
        run_trial(&mut live, i, &full),
        Err(ImageError::Injection(InjectError::Full))
    ));
    // A corrupt image is refused before any fork runs.
    let mut bad = image.clone();
    bad[100] ^= 1;
    let corrupt = Trial {
        image: &bad,
        ..trial.clone()
    };
    assert!(run_trial(&mut live, i, &corrupt).is_err());
    assert_eq!(live.amendments()[i].status, AMENDMENT_ADMITTED);
    // An empty trial is recorded and rejected.
    let empty = Trial {
        ticks: 0,
        ..trial.clone()
    };
    let report = run_trial(&mut live, i, &empty).unwrap();
    assert!(!report.may_commit);
    assert_eq!(
        report.baseline.behaviour_hash, report.candidate.behaviour_hash,
        "two forks of one image that ran nothing"
    );
    assert_eq!(live.amendments()[i].status, AMENDMENT_REJECTED);
    assert_eq!(live.amendments()[i].reason, REJECT_EMPTY_TRIAL);
}

#[test]
fn a_trial_needs_the_image_written_after_the_last_commit_to_its_parameter() {
    let (mut live, before) = quiescent_network();
    let a = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir = log_dir("stale-a");
    assert!(
        run_trial(&mut live, a, &quiet_trial(&before, &dir))
            .unwrap()
            .may_commit
    );
    assert_eq!(live.commit(a), Ok(()));
    let after = Image::encode(&live).unwrap();
    // The next amendment starts from 100; the image written before the commit says 10 000.
    let b = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        99,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let stale = log_dir("stale-b");
    assert!(matches!(
        run_trial(&mut live, b, &quiet_trial(&before, &stale)),
        Err(ImageError::StaleBaseline(PARAM_SWEEP_QUIET_TICKS))
    ));
    assert_eq!(live.amendments()[b].status, AMENDMENT_ADMITTED, "unchanged");
    // On the image written after the commit, 100 to 99 frees nothing more: no gain.
    let fresh = log_dir("stale-c");
    let report = run_trial(&mut live, b, &quiet_trial(&after, &fresh)).unwrap();
    assert!(!report.may_commit);
    assert_eq!(
        report.baseline.resident_units, report.candidate.resident_units,
        "the baseline is the live policy"
    );
    assert_eq!(live.amendments()[b].reason, REJECT_NO_GAIN);
    // A parameter the image does hold at its starting value is trialled: the budget.
    let c = admitted(&mut live, PARAM_SWEEP_BUDGET, 8, OBJECTIVE_RESIDENT_UNITS);
    let budget = log_dir("stale-d");
    let report = run_trial(&mut live, c, &quiet_trial(&after, &budget)).unwrap();
    assert!(!report.may_commit, "a smaller budget frees less");
    assert!(report.candidate.resident_units > report.baseline.resident_units);
}

#[test]
fn commits_to_one_parameter_keep_the_arena_s_order_and_a_stale_or_superseded_one_is_refused() {
    let (mut live, image) = quiescent_network();
    let a = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let b = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        50,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir_a = log_dir("order-a");
    let dir_b = log_dir("order-b");
    assert!(
        run_trial(&mut live, a, &quiet_trial(&image, &dir_a))
            .unwrap()
            .may_commit
    );
    assert!(
        run_trial(&mut live, b, &quiet_trial(&image, &dir_b))
            .unwrap()
            .may_commit
    );
    // Both were trialled from 10 000; b is committed first.
    assert_eq!(live.commit(b), Ok(()));
    assert_eq!(live.policy().sweep_quiet_ticks, 50);
    assert_eq!(
        live.commit(a),
        Err(AmendError::Superseded),
        "a later proposal for the same parameter was committed first"
    );
    assert_eq!(live.amendments()[a].status, AMENDMENT_TRIALLED);
    // Back to 10 000 under the re-hydration objective on active tissue, from an image
    // written after b's commit: c is committed, and a is still superseded although the live
    // value is again the one it started from.
    let after_b = Image::encode(&live).unwrap();
    let c = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        10_000,
        OBJECTIVE_REHYDRATIONS,
    );
    let injections = active_injections();
    let dir_c = log_dir("order-c");
    let report = run_trial(
        &mut live,
        c,
        &Trial {
            image: &after_b,
            config: fork_config(),
            injections: &injections,
            ticks: 3_000,
            sweep_every: 100,
            log_dir: &dir_c,
        },
    )
    .unwrap();
    assert!(report.may_commit, "{report:?}");
    assert!(report.baseline.rehydrations > report.candidate.rehydrations);
    assert_eq!(live.commit(c), Ok(()));
    assert_eq!(live.policy().sweep_quiet_ticks, 10_000);
    assert_eq!(live.commit(a), Err(AmendError::Superseded));
    // A trialled amendment whose starting value moved is stale: d starts from 10 000 and is
    // trialled; the policy is then moved by hand through a fresh arena? No: the only way the
    // live value moves is a commit, so d is stale exactly when a later commit landed, which
    // is `Superseded` first. Staleness alone is reachable through the loader: an image whose
    // committed records leave the policy elsewhere than a trialled record started from.
    let after_c = Image::encode(&live).unwrap();
    let d = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir_d = log_dir("order-d");
    assert!(
        run_trial(&mut live, d, &quiet_trial(&after_c, &dir_d))
            .unwrap()
            .may_commit
    );
    assert_eq!(
        live.propose(PARAM_SWEEP_QUIET_TICKS, 25, OBJECTIVE_RESIDENT_UNITS, 0),
        Err(AmendError::ArenaFull),
        "four slots: a, b, c, d"
    );
    // The image of this arena loads, in order, to the same policy.
    let loaded = Image::decode::<64>(&after_c, fork_config()).unwrap();
    assert_eq!(loaded.policy().sweep_quiet_ticks, 10_000);
    assert_eq!(loaded.amendments().len(), 3);
    let mut loaded = Image::decode::<64>(&Image::encode(&live).unwrap(), fork_config()).unwrap();
    assert_eq!(loaded.amendments(), live.amendments());
    assert_eq!(loaded.policy(), live.policy());
    assert_eq!(
        loaded.commit(d),
        Ok(()),
        "the trialled record commits after the reload"
    );
    assert_eq!(loaded.policy().sweep_quiet_ticks, 100);
}

#[test]
fn a_trialled_amendment_whose_starting_value_moved_is_stale() {
    // A stale record reaches a live executor through the loader: an image whose committed
    // records leave the policy at 50 and a trialled record that started from 10 000. The
    // writer never writes one (a commit in between is `Superseded` first), so it is forged.
    let (mut live, image) = quiescent_network();
    let a = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        50,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let b = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        100,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir_a = log_dir("moved-a");
    let dir_b = log_dir("moved-b");
    assert!(
        run_trial(&mut live, a, &quiet_trial(&image, &dir_a))
            .unwrap()
            .may_commit
    );
    assert!(
        run_trial(&mut live, b, &quiet_trial(&image, &dir_b))
            .unwrap()
            .may_commit
    );
    assert_eq!(live.commit(a), Ok(()));
    assert_eq!(
        live.commit(b),
        Err(AmendError::Stale),
        "b started from 10 000, and the live value is 50"
    );
    assert_eq!(live.policy().sweep_quiet_ticks, 50, "nothing moved");
    assert_eq!(live.amendments()[b].status, AMENDMENT_TRIALLED);
    let reloaded = Image::decode::<64>(&Image::encode(&live).unwrap(), config()).unwrap();
    assert_eq!(reloaded.amendments(), live.amendments());
    let mut reloaded = reloaded;
    assert_eq!(reloaded.commit(b), Err(AmendError::Stale));
}

#[test]
fn committed_amendments_persist_in_the_image_and_the_loader_derives_the_policy() {
    let (mut live, image) = quiescent_network();
    // 10 000 to 3 700: the image was written at tick 3 000 and a fork's clock resumes there
    // (ADR-0033), so a unit that never spiked has been quiet for 3 501 ticks at the sweep on
    // the fork's tick 500 (nothing is evicted) and 4 001 at tick 1 000 (everything is).
    let a = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        3_700,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir_a = log_dir("persist-a");
    assert!(
        run_trial(&mut live, a, &quiet_trial(&image, &dir_a))
            .unwrap()
            .may_commit
    );
    assert_eq!(live.commit(a), Ok(()));
    let after_a = Image::encode(&live).unwrap();
    // 3 700 to 3 300 over 600 ticks: the one sweep, on the fork's tick 500 (3 501 quiet ticks),
    // evicts under 3 300 and not under 3 700.
    let b = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        3_300,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir_b = log_dir("persist-b");
    let report = run_trial(
        &mut live,
        b,
        &Trial {
            ticks: 600,
            ..quiet_trial(&after_a, &dir_b)
        },
    )
    .unwrap();
    assert!(report.may_commit, "{report:?}");
    assert_eq!(
        (
            report.baseline.resident_units,
            report.candidate.resident_units
        ),
        (96, 0)
    );
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
    assert_eq!(live.policy().sweep_quiet_ticks, 3_300);
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
        3_300,
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
    // The entries start at 64 and are 64 bytes each: an iterator, not a counter.
    for start in (64..).step_by(64).take(count) {
        let entry = SectionEntry::decode(image[start..][..64].try_into().unwrap());
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
    mutate(&mut bytes[offset..][..length]);
    let resealed = SectionEntry::new(
        entry.kind,
        entry.record_size,
        entry.offset,
        entry.length,
        crc64(&bytes[offset..][..length]),
    );
    bytes[entry_at..][..64].copy_from_slice(&resealed.encode());
    bytes
}

#[test]
fn the_loader_refuses_an_amendment_it_could_not_have_written() {
    let (mut live, image) = quiescent_network();
    // 10 000 to 3 700: the image was written at tick 3 000 and a fork's clock resumes there
    // (ADR-0033), so a unit that never spiked has been quiet for 3 501 ticks at the sweep on
    // the fork's tick 500 (nothing is evicted) and 4 001 at tick 1 000 (everything is).
    let a = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        3_700,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir_a = log_dir("forged-a");
    assert!(
        run_trial(&mut live, a, &quiet_trial(&image, &dir_a))
            .unwrap()
            .may_commit
    );
    assert_eq!(live.commit(a), Ok(()));
    let after_a = Image::encode(&live).unwrap();
    // 3 700 to 3 300 over 600 ticks: the one sweep, on the fork's tick 500 (3 501 quiet ticks),
    // evicts under 3 300 and not under 3 700.
    let b = admitted(
        &mut live,
        PARAM_SWEEP_QUIET_TICKS,
        3_300,
        OBJECTIVE_RESIDENT_UNITS,
    );
    let dir_b = log_dir("forged-b");
    assert!(
        run_trial(
            &mut live,
            b,
            &Trial {
                ticks: 600,
                ..quiet_trial(&after_a, &dir_b)
            },
        )
        .unwrap()
        .may_commit
    );
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
    // A zero id where a record should be.
    let zero_id = tamper(&image, |s| s[16..20].fill(0));
    assert_eq!(refused(&zero_id, "zero id"), 0);
    // A commit that does not follow the one before it: the second started from 3 700, the
    // first's proposed value; make its low byte 20.
    let stale = tamper(&image, |s| s[64 + 28] = 20);
    assert_eq!(refused(&stale, "stale"), 1);
    // The first's start is not the default.
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
    let tight = Image::decode::<64>(&image, fork_config()).unwrap();
    assert_eq!(tight.amendment_room(), 0);
    assert_eq!(tight.amendments().len(), 2);
}
