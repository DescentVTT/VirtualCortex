//! The amendment's trial (ADR-0031; whitepaper §6.16, §8.18): two forks of one image, the
//! baseline under the live policy and the candidate under the proposed value, run for the same
//! ticks with the same injections and the same sweep cadence to a quiescent point; then the
//! behaviour hash of each (every unit's 64 bytes, an evicted one's read back from its log,
//! every synapse block, the spike train and the delivered count) and the objective's cost in
//! each. The amendment's `record_trial` judges them: equal hashes, or the candidate is rejected
//! for having behaved differently; a cost that fell by the amendment's `min_gain`, or it is
//! rejected for no gain. The verdict is written into the live executor's arena by this
//! function and nothing else.
//!
//! The fork is the image: `Image::decode` is the only copy of the state the runtime knows how
//! to make, and it validates the bytes on the way in; the image must carry the value the
//! amendment started from, or the baseline is not the live one and the trial is refused. A
//! fork must end where an image could be written, at a quiescent point, since a pending
//! mailbox names pool nodes the delivering worker chose; and its spike train must be wholly
//! traced, or the hash would not cover it. A trial runs outside the tick loop, where the writer
//! runs, so its allocations and its two log files are outside TC-5's scope, like the writer's.

use crate::executor::{ACTIVATE, AmendError, Config, Executor};
use crate::image::{Image, ImageError};
use cortex_connectome::Crc64;
use cortex_executive::{AMENDMENT_ADMITTED, OBJECTIVE_REHYDRATIONS, OBJECTIVE_RESIDENT_UNITS};
use std::path::Path;

/// What a trial runs: the same on both forks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trial<'a> {
    /// The image both forks start from, written at a quiescent point of the live executor
    /// after its last commit.
    pub image: &'a [u8],
    /// The forks' configuration; the arena sizes come from the image. `trace_capacity` must
    /// hold every spike of the run.
    pub config: Config,
    /// `(tick, unit, payload)`, sorted by tick: a message (or `ACTIVATE`) injected into the
    /// unit before that tick runs.
    pub injections: &'a [(u64, u32, u32)],
    /// Ticks each fork runs; a zero-tick trial is recorded and rejected as empty. The forks
    /// must be quiescent when the ticks end.
    pub ticks: u64,
    /// A sweep under the fork's policy after every this many ticks; 0 never sweeps, so the
    /// forks cannot differ in cost under a sweep parameter.
    pub sweep_every: u64,
    /// Where the two forks' write-ahead logs are created (`baseline.wal`, `candidate.wal`).
    pub log_dir: &'a Path,
}

/// What one fork came to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ForkReport {
    /// CRC-64/XZ over every unit's image bytes, every block's bytes, the spike train in
    /// `(tick, unit)` order and the delivered count.
    pub behaviour_hash: u64,
    /// Units still resident when the fork ended.
    pub resident_units: u32,
    /// Re-hydrations during the fork.
    pub rehydrations: u32,
    /// Evictions during the fork.
    pub evictions: u32,
    /// Spikes during the fork, every one of them traced.
    pub spikes: u32,
}

/// What the trial came to, as recorded on the amendment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrialReport {
    pub baseline: ForkReport,
    pub candidate: ForkReport,
    /// The amendment may now be committed.
    pub may_commit: bool,
}

/// Runs `trial` for the admitted amendment at `index` of `exec`'s arena, records the verdict
/// on it and returns the two forks' reports. Refused, with the amendment unchanged, for an
/// index the arena does not hold or an amendment that is not admitted, for a configuration
/// that traces no spikes, for an image whose policy is not the value the amendment started
/// from, for an injection the fork refuses, for a fork that does not end at a quiescent point,
/// and for a fork that dropped a spike from its trace.
pub fn run<const CAP: usize>(
    exec: &mut Executor<CAP>,
    index: usize,
    trial: &Trial<'_>,
) -> Result<TrialReport, ImageError> {
    let amendment = *exec
        .amendments()
        .get(index)
        .ok_or(ImageError::Amendment(AmendError::NoSuchAmendment))?;
    if amendment.status != AMENDMENT_ADMITTED {
        return Err(ImageError::Amendment(AmendError::NotAdmitted));
    }
    if trial.config.trace_capacity == 0 {
        return Err(ImageError::NoTrace);
    }
    let cost = |fork: &ForkReport| match amendment.objective {
        OBJECTIVE_RESIDENT_UNITS => fork.resident_units,
        OBJECTIVE_REHYDRATIONS => fork.rehydrations,
        _ => u32::MAX,
    };
    let start = (amendment.parameter, amendment.current_value);
    let baseline = run_fork::<CAP>(trial, &trial.log_dir.join("baseline.wal"), start, None)?;
    let candidate = run_fork::<CAP>(
        trial,
        &trial.log_dir.join("candidate.wal"),
        start,
        Some(amendment.proposed_value),
    )?;
    let may_commit = exec.record_trial(
        index,
        trial.ticks.min(u32::MAX as u64) as u32,
        baseline.behaviour_hash,
        candidate.behaviour_hash,
        cost(&baseline),
        cost(&candidate),
    );
    Ok(TrialReport {
        baseline,
        candidate,
        may_commit,
    })
}

/// One fork: decode the image, check that its policy holds the value the amendment started
/// from, attach a log, put the candidate value in place (if any), run the ticks with the
/// injections and the sweeps to a quiescent point, then hash and count.
fn run_fork<const CAP: usize>(
    trial: &Trial<'_>,
    log: &Path,
    (parameter, current): (u16, i32),
    candidate: Option<i32>,
) -> Result<ForkReport, ImageError> {
    let mut exec = Image::decode::<CAP>(trial.image, trial.config.clone())?;
    if exec.policy().value(parameter) != Some(current) {
        return Err(ImageError::StaleBaseline(parameter));
    }
    exec.attach_log(log)?;
    if let Some(value) = candidate {
        // An admitted amendment passed the bounds gate on this value; a refusal is a bug.
        assert!(
            exec.set_policy_value(parameter, value),
            "a trial value the registry admits was refused by the policy"
        );
    }
    let inject = exec.injector();
    let mut next = 0;
    for tick in 0..trial.ticks {
        while next < trial.injections.len() && trial.injections[next].0 <= tick {
            let (_, unit, payload) = trial.injections[next];
            next += 1;
            let queued = if payload == ACTIVATE {
                inject.activate(unit)
            } else {
                inject.inject(unit, payload)
            };
            if let Err(e) = queued {
                return Err(ImageError::Injection(e));
            }
        }
        exec.tick();
        if trial.sweep_every > 0 && (tick + 1) % trial.sweep_every == 0 {
            exec.sweep_by_policy()?;
        }
    }
    if !exec.is_quiescent() {
        return Err(ImageError::NotQuiescent);
    }
    report(exec)
}

/// The fork's report, consuming the executor for its workers' traces.
fn report<const CAP: usize>(exec: Executor<CAP>) -> Result<ForkReport, ImageError> {
    let mut crc = Crc64::new();
    let mut resident = 0u32;
    for (i, unit) in exec.units().iter().enumerate() {
        let bytes = if exec.is_evicted(i as u32) {
            exec.log().ok_or(ImageError::NoLog)?.read(i as u32)?
        } else {
            resident = resident.saturating_add(1);
            let mut bytes = unit.encode();
            bytes[56] = 0; // as the writer does: a scheduled unit is hashed idle
            bytes
        };
        crc.update(&bytes);
    }
    for block in exec.blocks() {
        crc.update(&block.encode());
    }
    let evictions = exec.evictions().min(u32::MAX as u64) as u32;
    let rehydrations = exec.rehydrations().min(u32::MAX as u64) as u32;
    let reports = exec.shutdown();
    if reports.iter().any(|r| r.dropped > 0) {
        return Err(ImageError::NoTrace);
    }
    let mut spikes: Vec<(u32, u32)> = reports
        .iter()
        .flat_map(|r| r.spikes.iter().map(|&(unit, tick)| (tick, unit)))
        .collect();
    spikes.sort_unstable();
    let spike_count = spikes.len().min(u32::MAX as usize) as u32;
    for (tick, unit) in spikes {
        crc.update(&tick.to_le_bytes());
        crc.update(&unit.to_le_bytes());
    }
    let delivered: u64 = reports.iter().map(|r| r.delivered_count).sum();
    crc.update(&delivered.to_le_bytes());
    Ok(ForkReport {
        behaviour_hash: crc.finish(),
        resident_units: resident,
        rehydrations,
        evictions,
        spikes: spike_count,
    })
}
