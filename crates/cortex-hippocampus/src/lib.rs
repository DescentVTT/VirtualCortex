//! Hippocampus: the episodic ledger and its replay bookkeeping (whitepaper §5.2.15, §8.8;
//! ADR-0038), beside the attractor and place-field state whose dynamics are Specified. The
//! metric map is `cortex-spatial`'s (ADR-0016).
//!
//! An [`Episode`] is a tagged pattern of units appended to an index-addressed ledger the
//! executor holds and the image carries. The pattern and the tick it was tagged at are never
//! written again; the tag and the replay count are the ledger's annotations. During slow-wave
//! sleep (ADR-0037) the executor replays one episode per ripple, every $2^{\text{RIPPLE\_SHIFT}}$
//! ticks, by firing its pattern together, so that the pair rule of ADR-0022 potentiates every
//! synapse among its units and the modulator of ADR-0032 consolidates them; during REM a
//! ripple lowers an episode's tag instead, and an episode whose tag reached zero is spent and
//! never replayed again. [`HippocampalAttractorState`] keeps the ledger's length and the hand
//! the next ripple starts from.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

/// The most units one episode names: what a 64-byte record holds beside its header.
pub const PATTERN_MAX: usize = 12;
/// A replay event every $2^{11}$ ticks (2 048 ticks, 20.48 ms at the fine tick): four basal
/// time constants of the unit (ADR-0018), so that successive drives do not sum in the
/// dendrite and one replay is one spike of each unit of the pattern; above the refractory
/// window; about the length of one sharp-wave ripple, whose rate of occurrence in sleep is
/// slower and a Target to tune (ADR-0038).
pub const RIPPLE_SHIFT: u32 = 11;

/// 64-byte episode (whitepaper §5.2.15; ADR-0038): one record of the ledger. The all-zero
/// record (`Default`) is the empty slot of the arena beyond the ledger's length; it names no
/// unit and is not well formed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct Episode {
    pub tagged_tick: u32, // [0..4] The tick the episode was tagged at; never written again
    pub tag: u8,          // [4] The REM ripples the episode survives; 0 is spent
    pub replays: u8,      // [5] Slow-wave replays so far, saturating
    pub len: u8,          // [6] Units in the pattern, 1 to PATTERN_MAX
    pub _pad: u8,         // [7] Reserved; MUST be zero
    pub pattern: [u32; PATTERN_MAX], // [8..56] Unit indices, the first `len`; the rest MUST be zero
    pub _reserved: [u8; 8], // [56..64] Reserved; MUST be zero
}

impl Episode {
    /// A new episode: `units` in the order given, tagged at `tick` with `priority` REM
    /// ripples to survive. Refused for no unit, more than [`PATTERN_MAX`], a unit named
    /// twice (a ripple would fire it twice as hard) or a priority of zero (spent before it
    /// was ever replayed).
    pub fn tag(tick: u32, units: &[u32], priority: u8) -> Option<Self> {
        if units.is_empty() || units.len() > PATTERN_MAX || priority == 0 {
            return None;
        }
        let mut pattern = [0u32; PATTERN_MAX];
        for (i, &unit) in units.iter().enumerate() {
            if units[..i].contains(&unit) {
                return None;
            }
            pattern[i] = unit;
        }
        Some(Self {
            tagged_tick: tick,
            tag: priority,
            replays: 0,
            len: units.len() as u8,
            _pad: 0,
            pattern,
            _reserved: [0; 8],
        })
    }

    /// The units of the pattern, in the order they were tagged.
    #[inline]
    pub fn pattern(&self) -> &[u32] {
        &self.pattern[..(self.len as usize).min(PATTERN_MAX)]
    }

    /// True once the tag reached zero: the episode is never replayed again.
    #[inline]
    pub const fn is_spent(&self) -> bool {
        self.tag == 0
    }

    /// A slow-wave replay: counts it, saturating, and yields the pattern for the executor to
    /// fire together; `None`, with nothing changed, for a spent episode.
    pub fn replay(&mut self) -> Option<&[u32]> {
        if self.is_spent() {
            return None;
        }
        self.replays = self.replays.saturating_add(1);
        Some(self.pattern())
    }

    /// A REM ripple: the tag falls by one, saturating at zero, where the episode is spent.
    /// Returns the tag.
    pub fn depotentiate(&mut self) -> u8 {
        self.tag = self.tag.saturating_sub(1);
        self.tag
    }

    /// True for a record `tag` could have produced and the annotations could have left: one
    /// to [`PATTERN_MAX`] units, the slots beyond them zero, no unit twice, the pad and the
    /// reserved bytes zero. The tag may be zero (spent). The loader refuses anything else,
    /// and separately refuses a unit outside the arena (ADR-0028).
    pub fn is_well_formed(&self) -> bool {
        let len = self.len as usize;
        if len == 0 || len > PATTERN_MAX {
            return false;
        }
        if self.pattern[len..].iter().any(|&unit| unit != 0) {
            return false;
        }
        for i in 1..len {
            if self.pattern[..i].contains(&self.pattern[i]) {
                return false;
            }
        }
        self._pad == 0 && self._reserved == [0; 8]
    }

    /// The record's 64 bytes, little-endian, field by field (§8.7).
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..4].copy_from_slice(&self.tagged_tick.to_le_bytes());
        out[4] = self.tag;
        out[5] = self.replays;
        out[6] = self.len;
        out[7] = self._pad;
        for (slot, unit) in self.pattern.iter().enumerate() {
            let at = slot.wrapping_mul(4).wrapping_add(8);
            out[at..at.wrapping_add(4)].copy_from_slice(&unit.to_le_bytes());
        }
        out[56..64].copy_from_slice(&self._reserved);
        out
    }

    /// A record from its 64 bytes; not validated (`is_well_formed` is the check).
    pub fn decode(bytes: &[u8; 64]) -> Self {
        let mut pattern = [0u32; PATTERN_MAX];
        for (slot, unit) in pattern.iter_mut().enumerate() {
            let at = slot.wrapping_mul(4).wrapping_add(8);
            *unit = u32::from_le_bytes(bytes[at..at.wrapping_add(4)].try_into().unwrap_or([0; 4]));
        }
        Self {
            tagged_tick: u32::from_le_bytes(bytes[0..4].try_into().unwrap_or([0; 4])),
            tag: bytes[4],
            replays: bytes[5],
            len: bytes[6],
            _pad: bytes[7],
            pattern,
            _reserved: bytes[56..64].try_into().unwrap_or([0; 8]),
        }
    }
}

/// 64-byte hippocampal state (whitepaper §5.2.15): the ledger's length and its hand
/// (ADR-0038), and the attractor and place-field quantities whose dynamics are Specified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct HippocampalAttractorState {
    pub dg_sparsity_bits: u32, // [0..4] Dentate Gyrus pattern separation sparsity (Specified)
    pub ca3_recurrent_energy: i32, // [4..8] CA3 auto-associative energy (Q16.16; Specified)
    pub ca1_comparator_error: i32, // [8..12] CA1 pattern matching error (Q16.16; Specified)
    pub episodes: u32, // [12..16] The ledger's length (ADR-0038; the replay countdown lived here until format 13, finding F-29)
    pub grid_theta_phase: u32, // [16..20] Toroidal grid cell attractor phase (Specified)
    pub place_field_id: u32, // [20..24] Currently mapped spatial place field (Specified)
    pub replay_hand: u32, // [24..28] The episode the next ripple considers, below the length
    pub _reserved: [u8; 36], // [28..64] Reserved; MUST be zero
}

impl HippocampalAttractorState {
    /// The state at rest: an empty ledger, the hand at zero, every other field zero. Equal to
    /// `Default`.
    pub const fn new() -> Self {
        Self {
            dg_sparsity_bits: 0,
            ca3_recurrent_energy: 0,
            ca1_comparator_error: 0,
            episodes: 0,
            grid_theta_phase: 0,
            place_field_id: 0,
            replay_hand: 0,
            _reserved: [0; 36],
        }
    }

    /// One more episode in the ledger: returns the index the new episode takes, or `None`,
    /// with nothing changed, when the width is full.
    pub fn append(&mut self) -> Option<u32> {
        let index = self.episodes;
        self.episodes = index.checked_add(1)?;
        Some(index)
    }

    /// The episode the next ripple considers, and the hand moved to the one after it, round
    /// robin over the ledger; `None` for an empty ledger.
    pub fn next_hand(&mut self) -> Option<u32> {
        if self.episodes == 0 {
            return None;
        }
        let hand = self.replay_hand;
        // The ledger is not empty, checked above: the fallback is never taken.
        self.replay_hand = hand.wrapping_add(1).checked_rem(self.episodes).unwrap_or(0);
        Some(hand)
    }

    /// True for a record the rules keep: the hand below the ledger's length, or both zero;
    /// the reserved bytes zero. The loader refuses anything else (ADR-0028).
    pub fn is_well_formed(&self) -> bool {
        ((self.episodes == 0 && self.replay_hand == 0) || self.replay_hand < self.episodes)
            && self._reserved == [0; 36]
    }

    /// The record's 64 bytes, little-endian, field by field (§8.7).
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..4].copy_from_slice(&self.dg_sparsity_bits.to_le_bytes());
        out[4..8].copy_from_slice(&self.ca3_recurrent_energy.to_le_bytes());
        out[8..12].copy_from_slice(&self.ca1_comparator_error.to_le_bytes());
        out[12..16].copy_from_slice(&self.episodes.to_le_bytes());
        out[16..20].copy_from_slice(&self.grid_theta_phase.to_le_bytes());
        out[20..24].copy_from_slice(&self.place_field_id.to_le_bytes());
        out[24..28].copy_from_slice(&self.replay_hand.to_le_bytes());
        out[28..64].copy_from_slice(&self._reserved);
        out
    }

    /// A record from its 64 bytes; not validated (`is_well_formed` is the check).
    pub fn decode(bytes: &[u8; 64]) -> Self {
        let u32_at = |at: usize| {
            u32::from_le_bytes(bytes[at..at.wrapping_add(4)].try_into().unwrap_or([0; 4]))
        };
        Self {
            dg_sparsity_bits: u32_at(0),
            ca3_recurrent_energy: u32_at(4) as i32,
            ca1_comparator_error: u32_at(8) as i32,
            episodes: u32_at(12),
            grid_theta_phase: u32_at(16),
            place_field_id: u32_at(20),
            replay_hand: u32_at(24),
            _reserved: bytes[28..64].try_into().unwrap_or([0; 36]),
        }
    }
}

// Arrays longer than 32 elements do not implement Default, so the record at rest is spelled
// out; every field of a default record is zero.
impl Default for HippocampalAttractorState {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::size_of::<HippocampalAttractorState>() == 64);
    assert!(core::mem::align_of::<HippocampalAttractorState>() == 64);
    assert!(core::mem::size_of::<Episode>() == 64);
    assert!(core::mem::align_of::<Episode>() == 64);
    // The pattern fits between the header and the reserved bytes, and the ripple's shift is
    // below the clock's width.
    assert!(PATTERN_MAX * 4 + 8 <= 56);
    assert!(RIPPLE_SHIFT < 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_are_one_cache_line_and_the_empty_slot_is_not_an_episode() {
        assert_eq!(core::mem::size_of::<HippocampalAttractorState>(), 64);
        assert_eq!(core::mem::align_of::<HippocampalAttractorState>(), 64);
        assert_eq!(core::mem::size_of::<Episode>(), 64);
        assert_eq!(core::mem::align_of::<Episode>(), 64);
        assert_eq!((PATTERN_MAX, RIPPLE_SHIFT), (12, 11));
        let empty = Episode::default();
        assert!(!empty.is_well_formed(), "no unit: the empty slot");
        assert!(empty.is_spent());
        assert_eq!(empty.pattern(), &[] as &[u32]);
        let rest = HippocampalAttractorState::new();
        assert_eq!(rest, HippocampalAttractorState::default());
        assert!(rest.is_well_formed());
        assert_eq!((rest.episodes, rest.replay_hand), (0, 0));
    }

    #[test]
    fn tagging_keeps_the_pattern_in_order_and_refuses_what_a_ripple_could_not_replay() {
        let e = Episode::tag(7, &[3, 1, 2], 5).expect("three units");
        assert_eq!(e.pattern(), &[3, 1, 2]);
        assert_eq!(
            (e.tagged_tick, e.tag, e.replays, e.len, e._pad),
            (7, 5, 0, 3, 0)
        );
        assert_eq!(&e.pattern[3..], &[0; 9]);
        assert_eq!(e._reserved, [0; 8]);
        assert!(e.is_well_formed());
        assert!(!e.is_spent());
        assert_eq!(Episode::tag(7, &[], 5), None, "no unit");
        let thirteen: [u32; 13] = core::array::from_fn(|i| i as u32);
        assert_eq!(Episode::tag(7, &thirteen, 5), None, "thirteen units");
        let twelve = Episode::tag(0, &thirteen[..12], 1).expect("twelve units");
        assert_eq!(twelve.len, 12);
        assert_eq!(twelve.pattern(), &thirteen[..12]);
        assert!(twelve.is_well_formed());
        assert_eq!(Episode::tag(7, &[3, 1, 3], 5), None, "a unit twice");
        let mut far = thirteen;
        far[11] = 0;
        assert_eq!(
            Episode::tag(7, &far[..12], 5),
            None,
            "a unit twice, eleven apart"
        );
        assert_eq!(
            Episode::tag(7, &[3], 0),
            None,
            "a priority of zero is spent"
        );
        let one = Episode::tag(u32::MAX, &[0], 255).expect("one unit, unit 0");
        assert_eq!(one.pattern(), &[0]);
        assert!(one.is_well_formed());
    }

    #[test]
    fn a_replay_counts_until_the_tag_is_spent_and_depotentiation_reaches_zero_and_stays() {
        let mut e = Episode::tag(1, &[4, 5], 2).unwrap();
        assert_eq!(e.replay(), Some(&[4u32, 5][..]));
        assert_eq!(e.replay(), Some(&[4u32, 5][..]));
        assert_eq!(
            (e.replays, e.tag),
            (2, 2),
            "replays count, the tag does not"
        );
        assert_eq!(e.depotentiate(), 1);
        assert!(!e.is_spent());
        assert_eq!(e.replay(), Some(&[4u32, 5][..]));
        assert_eq!(e.depotentiate(), 0);
        assert!(e.is_spent());
        let spent = e;
        assert_eq!(e.replay(), None, "spent: nothing to replay");
        assert_eq!(e, spent, "and nothing changed");
        assert_eq!(e.depotentiate(), 0, "zero is a fixed point");
        assert_eq!(e, spent);
        assert!(
            e.is_well_formed(),
            "a spent episode is a well-formed record"
        );
        let mut many = Episode::tag(1, &[4], 1).unwrap();
        many.replays = u8::MAX;
        assert!(many.replay().is_some());
        assert_eq!(many.replays, u8::MAX, "saturates");
    }

    #[test]
    fn an_episode_is_well_formed_on_each_clause_alone() {
        let ok = Episode::tag(9, &[8, 6, 7], 3).unwrap();
        assert!(ok.is_well_formed());
        type Mutation = fn(&mut Episode);
        let cases: [(&str, Mutation); 6] = [
            ("no unit", |e| e.len = 0),
            ("thirteen units", |e| e.len = 13),
            ("a unit beyond the length", |e| e.pattern[3] = 1),
            ("a unit twice", |e| e.pattern[2] = 8),
            ("the pad", |e| e._pad = 1),
            ("a reserved byte", |e| e._reserved[7] = 1),
        ];
        for (what, mutate) in cases {
            let mut e = ok;
            mutate(&mut e);
            assert!(!e.is_well_formed(), "{what}");
        }
        // The edges: one unit, twelve units, a duplicate at the far ends, a tag of zero.
        let mut one = ok;
        one.len = 1;
        one.pattern = [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(one.is_well_formed());
        let mut twelve = ok;
        twelve.len = 12;
        twelve.pattern = core::array::from_fn(|i| i as u32 + 100);
        assert!(twelve.is_well_formed());
        twelve.pattern[11] = 100;
        assert!(!twelve.is_well_formed(), "the first and the last the same");
        let mut spent = ok;
        spent.tag = 0;
        assert!(spent.is_well_formed());
        let mut zero_unit = ok;
        zero_unit.pattern[2] = 0;
        assert!(
            zero_unit.is_well_formed(),
            "unit 0 inside the length is a unit, not an empty slot"
        );
    }

    #[test]
    fn the_ledger_appends_in_order_and_the_hand_walks_it_round_robin() {
        let mut s = HippocampalAttractorState::new();
        assert_eq!(s.next_hand(), None, "an empty ledger has no hand");
        assert_eq!(s.replay_hand, 0);
        assert_eq!(s.append(), Some(0));
        assert_eq!(s.next_hand(), Some(0));
        assert_eq!(s.next_hand(), Some(0), "one episode: always it");
        assert_eq!(s.append(), Some(1));
        assert_eq!(s.append(), Some(2));
        assert_eq!(s.episodes, 3);
        assert_eq!(
            [s.next_hand(), s.next_hand(), s.next_hand(), s.next_hand()],
            [Some(0), Some(1), Some(2), Some(0)],
            "round robin, wrapping at the length"
        );
        assert_eq!(s.replay_hand, 1);
        assert!(s.is_well_formed());
        // An appended episode joins the round without moving the hand.
        assert_eq!(s.append(), Some(3));
        assert_eq!(
            [s.next_hand(), s.next_hand(), s.next_hand()],
            [Some(1), Some(2), Some(3)]
        );
        assert_eq!(s.next_hand(), Some(0));
        // The width: a full ledger appends nothing and keeps its length.
        let mut full = HippocampalAttractorState {
            episodes: u32::MAX,
            replay_hand: u32::MAX - 1,
            ..HippocampalAttractorState::new()
        };
        assert_eq!(full.append(), None);
        assert_eq!(full.episodes, u32::MAX);
        assert_eq!(full.next_hand(), Some(u32::MAX - 1));
        assert_eq!(full.replay_hand, 0, "the hand wraps at the length");
    }

    #[test]
    fn the_state_is_well_formed_only_with_the_hand_below_the_length() {
        let mut s = HippocampalAttractorState::new();
        s.replay_hand = 1;
        assert!(!s.is_well_formed(), "a hand into an empty ledger");
        s.episodes = 1;
        assert!(!s.is_well_formed(), "a hand at the length");
        s.replay_hand = 0;
        assert!(s.is_well_formed());
        s.episodes = 5;
        s.replay_hand = 4;
        assert!(s.is_well_formed(), "one below the length");
        s.replay_hand = 5;
        assert!(!s.is_well_formed());
        let mut reserved = HippocampalAttractorState::new();
        reserved._reserved[35] = 1;
        assert!(!reserved.is_well_formed());
        let specified = HippocampalAttractorState {
            dg_sparsity_bits: 1,
            ca3_recurrent_energy: -1,
            ca1_comparator_error: 2,
            grid_theta_phase: 3,
            place_field_id: 4,
            ..HippocampalAttractorState::new()
        };
        assert!(specified.is_well_formed(), "the Specified fields are free");
    }

    #[test]
    fn both_records_round_trip_through_their_bytes() {
        let e = Episode {
            tagged_tick: 0x0102_0304,
            tag: 5,
            replays: 6,
            len: 12,
            _pad: 7,
            pattern: core::array::from_fn(|i| 0x1000_0000 + i as u32),
            _reserved: [8; 8],
        };
        let bytes = e.encode();
        assert_eq!(&bytes[0..4], &[4, 3, 2, 1]);
        assert_eq!(&bytes[4..8], &[5, 6, 12, 7]);
        assert_eq!(&bytes[8..12], &0x1000_0000u32.to_le_bytes());
        assert_eq!(&bytes[52..56], &0x1000_000Bu32.to_le_bytes());
        assert_eq!(&bytes[56..64], &[8; 8]);
        assert_eq!(Episode::decode(&bytes), e);
        assert_eq!(Episode::decode(&[0; 64]), Episode::default());
        let s = HippocampalAttractorState {
            dg_sparsity_bits: 1,
            ca3_recurrent_energy: -2,
            ca1_comparator_error: i32::MIN,
            episodes: 0xAABB_CCDD,
            grid_theta_phase: 5,
            place_field_id: 6,
            replay_hand: 7,
            _reserved: [9; 36],
        };
        let bytes = s.encode();
        assert_eq!(&bytes[4..8], &(-2i32).to_le_bytes());
        assert_eq!(&bytes[12..16], &[0xDD, 0xCC, 0xBB, 0xAA]);
        assert_eq!(&bytes[24..28], &7u32.to_le_bytes());
        assert_eq!(&bytes[28..64], &[9; 36]);
        assert_eq!(HippocampalAttractorState::decode(&bytes), s);
        assert_eq!(
            HippocampalAttractorState::decode(&[0; 64]),
            HippocampalAttractorState::new()
        );
    }
}

/// Property tests (ADR-0030): `tag` accepts exactly the patterns a ripple could replay and
/// produces a well-formed record; the annotations keep it well formed; the ledger's hand
/// stays below its length.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn tag_accepts_exactly_the_replayable_patterns_and_the_annotations_keep_the_record() {
        let mut rng = Lcg::new(37);
        let mut accepted = 0u32;
        for round in 0..4_000u32 {
            let len = rng.below(15) as usize;
            let mut units = [0u32; 14];
            for u in units.iter_mut() {
                *u = if rng.below(3) == 0 {
                    rng.below(4)
                } else {
                    rng.next_u32()
                };
            }
            let units = &units[..len];
            let priority = rng.next_u8();
            let mut valid = (1..=PATTERN_MAX).contains(&len) && priority != 0;
            for i in 0..len {
                for j in 0..i {
                    if units[i] == units[j] {
                        valid = false;
                    }
                }
            }
            let tick = rng.next_u32();
            match Episode::tag(tick, units, priority) {
                Some(mut e) => {
                    assert!(valid, "round {round}: accepted {units:?}");
                    accepted = accepted.wrapping_add(1);
                    assert!(e.is_well_formed());
                    assert_eq!(e.pattern(), units);
                    assert_eq!(Episode::decode(&e.encode()), e);
                    let mut tag = priority;
                    for _ in 0..300 {
                        if rng.below(2) == 0 {
                            let before = e.replays;
                            let r = e.replay();
                            assert_eq!(r.is_some(), tag != 0);
                            assert_eq!(
                                e.replays,
                                if tag != 0 {
                                    before.saturating_add(1)
                                } else {
                                    before
                                }
                            );
                        } else {
                            tag = tag.saturating_sub(1);
                            assert_eq!(e.depotentiate(), tag);
                        }
                        assert!(e.is_well_formed());
                        assert_eq!(e.pattern(), units, "the pattern never moves");
                        assert_eq!(e.tagged_tick, tick);
                    }
                }
                None => assert!(!valid, "round {round}: refused {units:?} at {priority}"),
            }
        }
        assert!(accepted > 1_000, "{accepted} accepted");
    }

    #[test]
    fn the_hand_stays_below_the_length_through_every_append_and_step() {
        let mut rng = Lcg::new(41);
        for _ in 0..200u32 {
            let mut s = HippocampalAttractorState::new();
            let mut seen = [0u32; 64];
            for step in 0..2_000u32 {
                if rng.below(4) == 0 && s.episodes < 64 {
                    let before = s.episodes;
                    assert_eq!(s.append(), Some(before));
                    assert_eq!(s.episodes, before.wrapping_add(1));
                } else if let Some(hand) = s.next_hand() {
                    assert!(hand < s.episodes, "step {step}: {hand} of {}", s.episodes);
                    seen[hand as usize] = seen[hand as usize].wrapping_add(1);
                } else {
                    assert_eq!(s.episodes, 0);
                }
                assert!(s.is_well_formed());
                assert_eq!(HippocampalAttractorState::decode(&s.encode()), s);
            }
            // Round robin: every episode in the ledger was the hand at least once, and the
            // counts differ by at most the number of appends that came between.
            for (i, &count) in seen.iter().enumerate().take(s.episodes as usize) {
                assert!(count > 0 || i as u32 == s.episodes.wrapping_sub(1));
            }
        }
    }
}
