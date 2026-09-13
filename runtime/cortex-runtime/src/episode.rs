//! Episodes tagged from a spike train (whitepaper §5.2.15, §6.6, §6.10; ADR-0048). The rules
//! are `cortex-hippocampus`'s (`burst`, `capture`); this module composes them with the
//! executor's ledger and with a rewarded invention: the pattern active in the ripple before
//! the reward becomes the invention's episode, and the pair is the caller's association
//! between the invented predicate's id and the ledger's index. Between ticks; nothing here
//! allocates. The train is a fork's of the same run ([`fork`](crate::branching::fork)), since
//! a worker's spikes are its own until it stops.

use crate::discovery::Discovery;
use crate::executor::{Executor, TagError};
use cortex_hippocampus::{Burst, PATTERN_MAX, burst, capture};

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
    let from = at.saturating_sub(window);
    // The train is sorted by tick: the spikes of `[from, at)` are one contiguous run, found
    // by two binary searches.
    let low = train.partition_point(|&(t, _)| t < from);
    let high = train.partition_point(|&(t, _)| t < at);
    let span = train.get(low..high).unwrap_or(&[]);
    let (burst, episode, pattern, len) = tag_burst(exec, span, coincidence, priority)?;
    Ok(Some(Association {
        predicate: discovery.invention.predicate,
        episode,
        pattern,
        len,
        burst,
    }))
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
