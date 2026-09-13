//! Episodes tagged from a spike train (whitepaper §5.2.15, §6.6, §6.10; ADR-0048, ADR-0050).
//! The rules are `cortex-hippocampus`'s (`burst`, `capture`); this module composes them with
//! the executor's ledger and with a rewarded invention: the pattern active in the ripple
//! before the reward becomes the invention's episode, and the pair is the caller's
//! association between the invented predicate's id and the ledger's index. Between ticks;
//! nothing here allocates. The train is a caller's slice (a fork's of the same run,
//! [`fork`](crate::branching::fork)) for the three functions of ADR-0048, and the executor's
//! own ([`Executor::train`], ADR-0050) for the three `recent` forms and for [`discover`],
//! which runs the search of ADR-0045, rewards the modulator and tags the coincidence before
//! the reward in one call.

use crate::discovery::{Discovery, DiscoveryError, SearchReport, search};
use crate::executor::{Executor, TagError};
use cortex_affect::InteroceptiveState;
use cortex_hippocampus::{Burst, PATTERN_MAX, burst, capture};
use cortex_reasoning::InduceScratch;

/// A rewarded invention's episode: the invented predicate's id, the ledger index of the
/// pattern active in the ripple before its reward, the pattern itself, and the span it was
/// read from. The caller's table, as the clause store is (ADR-0043): no record holds it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Association {
    pub predicate: u32,
    pub episode: u32,
    pub pattern: [u32; PATTERN_MAX],
    pub len: u8,
    /// The densest span of the window before the reward, where the pattern fired.
    pub burst: Burst,
}

impl Association {
    /// The units of the pattern, in rank order.
    pub fn pattern(&self) -> &[u32] {
        &self.pattern[..(self.len as usize).min(PATTERN_MAX)]
    }
}

/// Tags the pattern of the span `[from, from + window)` of `train` (`capture`) into the
/// executor's ledger with `priority`, between ticks; returns the ledger index, the pattern
/// and its length. The ledger's refusals as `tag_episode` gives them, in its order: a full
/// ledger is `LedgerFull` before anything else, then a unit outside the arena, then a span
/// with no spike (`InvalidPattern`).
pub fn tag_from_trace<const CAP: usize>(
    exec: &mut Executor<CAP>,
    train: &[(u32, u32)],
    from: u32,
    window: u32,
    priority: u8,
) -> Result<(u32, [u32; PATTERN_MAX], u8), TagError> {
    let mut pattern = [0u32; PATTERN_MAX];
    let len = capture(train, from, window, &mut pattern);
    let index = exec.tag_episode(&pattern[..(len as usize).min(PATTERN_MAX)], priority)?;
    Ok((index, pattern, len))
}

/// Tags the pattern of the densest span of `window` ticks in `train` (`burst`, then
/// `capture`); returns the burst with what `tag_from_trace` returns. `InvalidPattern` for an
/// empty train or a window of zero.
pub fn tag_burst<const CAP: usize>(
    exec: &mut Executor<CAP>,
    train: &[(u32, u32)],
    window: u32,
    priority: u8,
) -> Result<(Burst, u32, [u32; PATTERN_MAX], u8), TagError> {
    let found = burst(train, window).ok_or(TagError::InvalidPattern)?;
    let (index, pattern, len) = tag_from_trace(exec, train, found.from, window, priority)?;
    Ok((found, index, pattern, len))
}

/// The rule ADR-0045 named for the synaptic half of H-11: when `discovery`'s reward is
/// positive, the pattern active before `at` (the tick the reward came at) is tagged with
/// `priority` and returned with the invented predicate's id as the caller's association;
/// the pattern is the densest span of `coincidence` ticks within the `window` ticks before
/// `at` (the span `[at - window, at)`, from tick zero when `at` is nearer than the window),
/// ranked as `capture` ranks it: the units that fired together, not the units the drive
/// happened to fire most over the ripple. A reward that is not positive tags nothing
/// (`None`); a window with no spike is `InvalidPattern`. The train is the run's, sorted.
pub fn tag_discovery<const CAP: usize>(
    exec: &mut Executor<CAP>,
    train: &[(u32, u32)],
    at: u32,
    window: u32,
    coincidence: u32,
    discovery: &Discovery,
    priority: u8,
) -> Result<Option<Association>, TagError> {
    if discovery.reward_q16 <= 0 {
        return Ok(None);
    }
    let spikes = span(train, at.saturating_sub(window), at);
    let (burst, episode, pattern, len) = tag_burst(exec, spikes, coincidence, priority)?;
    Ok(Some(Association {
        predicate: discovery.invention.predicate,
        episode,
        pattern,
        len,
        burst,
    }))
}

/// The spikes of `[from, to)` in a train sorted by tick: one contiguous run, found by two
/// binary searches; empty when `to` is at or before `from`.
fn span(train: &[(u32, u32)], from: u32, to: u32) -> &[(u32, u32)] {
    let low = train.partition_point(|&(t, _)| t < from);
    let high = train.partition_point(|&(t, _)| t < to);
    train.get(low..high).unwrap_or(&[])
}

/// [`tag_from_trace`] over the executor's own train (ADR-0050): the pattern of
/// `[from, from + window)` of the spikes the ring holds, tagged with `priority`.
pub fn tag_recent<const CAP: usize>(
    exec: &mut Executor<CAP>,
    from: u32,
    window: u32,
    priority: u8,
) -> Result<(u32, [u32; PATTERN_MAX], u8), TagError> {
    let mut pattern = [0u32; PATTERN_MAX];
    let len = capture(exec.train(), from, window, &mut pattern);
    let index = exec.tag_episode(&pattern[..(len as usize).min(PATTERN_MAX)], priority)?;
    Ok((index, pattern, len))
}

/// [`tag_burst`] over the spikes of `[from, to)` of the executor's own train: the densest
/// span of `window` ticks in them, tagged with `priority`. `InvalidPattern` when they hold
/// no spike or the window is zero.
pub fn tag_burst_in<const CAP: usize>(
    exec: &mut Executor<CAP>,
    from: u32,
    to: u32,
    window: u32,
    priority: u8,
) -> Result<(Burst, u32, [u32; PATTERN_MAX], u8), TagError> {
    let mut pattern = [0u32; PATTERN_MAX];
    let (found, len) = {
        let spikes = span(exec.train(), from, to);
        let found = burst(spikes, window).ok_or(TagError::InvalidPattern)?;
        (found, capture(spikes, found.from, window, &mut pattern))
    };
    let index = exec.tag_episode(&pattern[..(len as usize).min(PATTERN_MAX)], priority)?;
    Ok((found, index, pattern, len))
}

/// [`tag_discovery`] at the executor's clock over its own train: the reward's tick is now,
/// the window the `window` ticks before it.
pub fn tag_discovery_recent<const CAP: usize>(
    exec: &mut Executor<CAP>,
    window: u32,
    coincidence: u32,
    discovery: &Discovery,
    priority: u8,
) -> Result<Option<Association>, TagError> {
    if discovery.reward_q16 <= 0 {
        return Ok(None);
    }
    let at = exec.ticks() as u32;
    let (burst, episode, pattern, len) =
        tag_burst_in(exec, at.saturating_sub(window), at, coincidence, priority)?;
    Ok(Some(Association {
        predicate: discovery.invention.predicate,
        episode,
        pattern,
        len,
        burst,
    }))
}

/// Why [`discover`] did not run to its report.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscoverError {
    /// The search's own refusal (the store, the scratch, a bound).
    Discovery(DiscoveryError),
    /// The ledger's refusal of the rewarded moment's pattern (the reward is the modulator's
    /// by then).
    Tag(TagError),
    /// Fewer association slots than discovery slots: refused before the search.
    OutFull,
}

impl From<DiscoveryError> for DiscoverError {
    fn from(e: DiscoveryError) -> Self {
        Self::Discovery(e)
    }
}

impl From<TagError> for DiscoverError {
    fn from(e: TagError) -> Self {
        Self::Tag(e)
    }
}

/// What [`discover`] came to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DiscoverReport {
    /// The search's report.
    pub search: SearchReport,
    /// The dopamine signal after the committed rewards' total went into the modulator, or
    /// the signal as it stood when nothing was committed.
    pub signal_q16: i32,
    /// The ledger index and the span of the rewarded moment's pattern, when one was tagged.
    pub tagged: Option<(u32, Burst)>,
}

/// What a search runs over (ADR-0045): the caller's clause store and its length, the
/// induction scratch, the affect state primed to the store's length, and the attempts the
/// search may spend.
pub struct ClauseSearch<'c, 'a> {
    pub store: &'c mut [u32],
    pub len: &'c mut usize,
    pub scratch: &'c mut InduceScratch<'a>,
    pub affect: &'c mut InteroceptiveState,
    pub budget: u32,
}

/// Where a rewarded moment's pattern is read and how it is tagged: the `window` ticks
/// before now, its densest `coincidence` span, the tag's `priority`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tagging {
    pub window: u32,
    pub coincidence: u32,
    pub priority: u8,
}

/// The discovery loop in one call between ticks (ADR-0050, the rule ADR-0048 composed by
/// hand): the search of ADR-0045 over the caller's store, its committed rewards' total into
/// the modulator when positive, and then, for a rewarded search, the pattern active in the
/// window before now (its densest coincidence, from the executor's own train) tagged once,
/// and one association per commit, to that episode, written to `associations` in the
/// commits' order. `OutFull` before the search when `associations` has fewer slots than
/// `discoveries`; the search's and the ledger's refusals as they give them, the search's
/// commits standing in the store either way and the reward the modulator's once given.
pub fn discover<const CAP: usize>(
    exec: &mut Executor<CAP>,
    clauses: ClauseSearch<'_, '_>,
    tagging: Tagging,
    discoveries: &mut [Discovery],
    associations: &mut [Association],
) -> Result<DiscoverReport, DiscoverError> {
    if associations.len() < discoveries.len() {
        return Err(DiscoverError::OutFull);
    }
    let report = search(
        clauses.store,
        clauses.len,
        clauses.scratch,
        clauses.affect,
        clauses.budget,
        discoveries,
    )?;
    if report.reward_total_q16 <= 0 {
        return Ok(DiscoverReport {
            search: report,
            signal_q16: exec.modulator().dopamine_rpe,
            tagged: None,
        });
    }
    let signal_q16 = exec.reward(report.reward_total_q16);
    let at = exec.ticks() as u32;
    let (burst, episode, pattern, len) = tag_burst_in(
        exec,
        at.saturating_sub(tagging.window),
        at,
        tagging.coincidence,
        tagging.priority,
    )?;
    let commits = (report.commits as usize).min(discoveries.len());
    for (slot, discovery) in associations
        .iter_mut()
        .zip(discoveries.iter().take(commits))
    {
        *slot = Association {
            predicate: discovery.invention.predicate,
            episode,
            pattern,
            len,
            burst,
        };
    }
    Ok(DiscoverReport {
        search: report,
        signal_q16,
        tagged: Some((episode, burst)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::Config;
    use cortex_reasoning::Invention;

    fn engine(episodes: usize) -> Executor<64> {
        Executor::<64>::new(Config {
            units: 8,
            episodes,
            ..Config::default()
        })
        .unwrap()
    }

    fn discovery(reward_q16: i32) -> Discovery {
        Discovery {
            invention: Invention {
                predicate: 0xFFFE_0007,
                ..Invention::default()
            },
            length_before: 20,
            length_after: 15,
            valence_q16: reward_q16.saturating_mul(4),
            reward_q16,
        }
    }

    const TRAIN: [(u32, u32); 7] = [
        (90, 6),
        (100, 1),
        (101, 2),
        (102, 1),
        (150, 3),
        (200, 5),
        (201, 5),
    ];

    /// A scratch and a store for a search over nothing: the search runs, attempts nothing
    /// and rewards nothing.
    fn nothing_to_search(
        exec: &mut Executor<64>,
        tagging: Tagging,
        room: usize,
    ) -> Result<DiscoverReport, DiscoverError> {
        use cortex_reasoning::{Binding, TermNode};
        let mut arena = [TermNode::default(); 4];
        let mut bindings = [Binding::UNBOUND; 4];
        let (mut trail, mut stack, mut pairs) = ([0u32; 4], [0u32; 8], [[0u32; 3]; 2]);
        let mut scratch = InduceScratch {
            arena: &mut arena,
            free: 0,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            pairs: &mut pairs,
            next_variable: 0,
            next_invented: 0xFFFE_0000,
        };
        let mut store = [0u32; 2];
        let mut len = 0usize;
        let mut affect = InteroceptiveState::default();
        let mut discoveries = [Discovery::default(); 2];
        let mut associations = [Association::default(); 2];
        discover(
            exec,
            ClauseSearch {
                store: &mut store,
                len: &mut len,
                scratch: &mut scratch,
                affect: &mut affect,
                budget: 8,
            },
            tagging,
            &mut discoveries,
            &mut associations[..room],
        )
    }

    #[test]
    fn the_recent_forms_read_the_executor_s_own_train_and_refuse_an_empty_one() {
        let mut exec = engine(2);
        exec.run(3);
        assert!(exec.train().is_empty(), "no unit fired: an empty train");
        assert_eq!(
            tag_recent(&mut exec, 0, 100, 5),
            Err(TagError::InvalidPattern)
        );
        assert_eq!(
            tag_burst_in(&mut exec, 0, 100, 10, 5),
            Err(TagError::InvalidPattern)
        );
        assert_eq!(
            tag_discovery_recent(&mut exec, 100, 10, &discovery(0x0001_0000), 5),
            Err(TagError::InvalidPattern)
        );
        assert_eq!(
            tag_discovery_recent(&mut exec, 100, 10, &discovery(0), 5),
            Ok(None),
            "a reward that is not positive tags nothing"
        );
        assert_eq!(exec.episodes().len(), 0);
        // The span of the executor's train is the same rule as the caller's train's.
        assert_eq!(span(&TRAIN, 100, 150), &TRAIN[1..4]);
        assert_eq!(span(&TRAIN, 100, 151), &TRAIN[1..5]);
        assert_eq!(span(&TRAIN, 150, 100), &[]);
        assert_eq!(span(&TRAIN, 0, 90), &[]);
        assert_eq!(span(&TRAIN, 201, u32::MAX), &TRAIN[6..]);
    }

    #[test]
    fn discover_refuses_too_few_association_slots_before_the_search_and_a_search_with_nothing_tags_nothing()
     {
        let mut exec = engine(2);
        exec.run(3);
        let tagging = Tagging {
            window: 100,
            coincidence: 10,
            priority: 5,
        };
        assert_eq!(
            nothing_to_search(&mut exec, tagging, 1),
            Err(DiscoverError::OutFull),
            "two discovery slots, one association slot"
        );
        let report = nothing_to_search(&mut exec, tagging, 2).unwrap();
        assert_eq!(
            (report.search.attempts, report.search.commits, report.tagged),
            (0, 0, None)
        );
        assert_eq!(report.signal_q16, 0, "the signal as it stood");
        assert_eq!(exec.episodes().len(), 0);
        assert_eq!(
            DiscoverError::from(DiscoveryError::Length),
            DiscoverError::Discovery(DiscoveryError::Length)
        );
        assert_eq!(
            DiscoverError::from(TagError::LedgerFull),
            DiscoverError::Tag(TagError::LedgerFull)
        );
    }

    #[test]
    fn a_span_is_tagged_in_rank_order_and_an_empty_span_is_refused() {
        let mut exec = engine(2);
        let (index, pattern, len) = tag_from_trace(&mut exec, &TRAIN, 100, 100, 3).unwrap();
        assert_eq!((index, len), (0, 3));
        assert_eq!(
            &pattern[..3],
            &[1, 2, 3],
            "unit 1 twice, then 2 and 3 once each"
        );
        assert_eq!(exec.episodes()[0].pattern(), &[1, 2, 3]);
        assert_eq!(
            (exec.episodes()[0].tag, exec.episodes()[0].tagged_tick),
            (3, 0)
        );
        assert_eq!(
            tag_from_trace(&mut exec, &TRAIN, 300, 100, 3),
            Err(TagError::InvalidPattern),
            "no spike in the span"
        );
        assert_eq!(exec.episodes().len(), 1, "nothing tagged");
        let far = [(0u32, 9u32)];
        assert_eq!(
            tag_from_trace(&mut exec, &far, 0, 10, 3),
            Err(TagError::NoSuchUnit),
            "unit 9 of eight"
        );
        assert_eq!(
            tag_from_trace(&mut exec, &TRAIN, 0, 300, 3).map(|r| r.0),
            Ok(1)
        );
        assert_eq!(
            tag_from_trace(&mut exec, &TRAIN, 0, 300, 3),
            Err(TagError::LedgerFull)
        );
    }

    #[test]
    fn the_burst_s_pattern_is_tagged_and_an_empty_train_is_refused() {
        let mut exec = engine(1);
        let (found, index, pattern, len) = tag_burst(&mut exec, &TRAIN, 3, 9).unwrap();
        assert_eq!(
            found,
            Burst {
                from: 100,
                spikes: 3
            }
        );
        assert_eq!((index, len), (0, 2));
        assert_eq!(&pattern[..2], &[1, 2]);
        let mut fresh = engine(1);
        assert_eq!(
            tag_burst(&mut fresh, &[], 3, 9),
            Err(TagError::InvalidPattern)
        );
        assert_eq!(
            tag_burst(&mut fresh, &TRAIN, 0, 9),
            Err(TagError::InvalidPattern),
            "a window of zero"
        );
        assert!(fresh.episodes().is_empty());
    }

    #[test]
    fn a_positive_reward_tags_the_densest_coincidence_before_it_and_any_other_reward_tags_nothing()
    {
        let mut exec = engine(2);
        let paid = discovery(0x8000);
        // The window [95, 160) holds 100, 101, 102 and 150; the densest span of ten ticks
        // starts at 100 and holds unit 1 twice and unit 2 once; 150 is outside it.
        let a = tag_discovery(&mut exec, &TRAIN, 160, 65, 10, &paid, 5)
            .unwrap()
            .expect("a positive reward");
        assert_eq!((a.predicate, a.episode, a.len), (0xFFFE_0007, 0, 2));
        assert_eq!(a.pattern(), &[1, 2]);
        assert_eq!(
            a.burst,
            Burst {
                from: 100,
                spikes: 3
            }
        );
        assert_eq!(exec.episodes()[0].pattern(), a.pattern());
        assert_eq!(exec.episodes()[0].tag, 5);
        // A coincidence as long as the window takes the window's units.
        let mut wide = engine(1);
        let all = tag_discovery(&mut wide, &TRAIN, 160, 65, 65, &paid, 5)
            .unwrap()
            .unwrap();
        assert_eq!(all.pattern(), &[1, 2, 3]);
        // The smallest positive reward tags; zero and a negative one do not.
        assert_eq!(
            tag_discovery(&mut exec, &TRAIN, 160, 100, 10, &discovery(0), 5),
            Ok(None)
        );
        assert_eq!(
            tag_discovery(&mut exec, &TRAIN, 160, 100, 10, &discovery(-1), 5),
            Ok(None)
        );
        assert_eq!(exec.episodes().len(), 1);
        let least = tag_discovery(&mut exec, &TRAIN, 205, 10, 10, &discovery(1), 5)
            .unwrap()
            .unwrap();
        assert_eq!(
            (least.episode, least.pattern()),
            (1, &[5u32][..]),
            "[195, 205)"
        );
        // A spike exactly a window before the reward is inside the span: from 100 the span
        // [100, 200) holds 100, 101, 102 and 150, and its densest ten ticks rank unit 1
        // (twice) before unit 2; without the spike at 100 the ranking would be 2 then 1.
        let mut edge = engine(1);
        let at_start = tag_discovery(&mut edge, &TRAIN, 200, 100, 10, &paid, 5)
            .unwrap()
            .unwrap();
        assert_eq!(at_start.pattern(), &[1, 2]);
        // A window past the reward's tick starts at zero; the spike at the reward's tick is
        // outside the window; a window with no spike is refused.
        let mut fresh = engine(2);
        let whole = tag_discovery(&mut fresh, &TRAIN, 95, 1000, 50, &paid, 5)
            .unwrap()
            .unwrap();
        assert_eq!(whole.pattern(), &[6]);
        assert_eq!(
            tag_discovery(&mut fresh, &TRAIN, 100, 5, 5, &paid, 5),
            Err(TagError::InvalidPattern),
            "[95, 100): the spike at 100 is the reward's tick"
        );
        assert_eq!(
            tag_discovery(&mut fresh, &TRAIN, 50, 10, 10, &paid, 5),
            Err(TagError::InvalidPattern)
        );
        assert_eq!(fresh.episodes().len(), 1);
        assert_eq!(Association::default().pattern(), &[] as &[u32]);
    }
}
