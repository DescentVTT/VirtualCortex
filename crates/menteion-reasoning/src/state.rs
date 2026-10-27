//! The induction record (whitepaper §5.2.30; ADR-0052): what an engine that owns a term
//! arena and a clause store must carry across a restart beside the nodes and the store's
//! indices, in one 64-byte record. The arena's cursor and the two counters that number what
//! the operators create (`InduceScratch::{free, next_variable, next_invented}`), the store's
//! length, the search's cursor (the pair after which the next search resumes, so that a
//! bounded budget makes progress instead of repeating its first pairs), the search's budget
//! and cadence and the tag an invention's episode is given. The bindings, the trail, the
//! stack and the pair table are the scratch they are: empty between searches, since a commit
//! instantiates its outputs ([`instantiate`](crate::induce::instantiate)) and unbinds. The
//! executor composes the record with the arena and the store; this crate holds the layout
//! and the rules that keep it well formed.

use crate::induce::{INVENTED_BASE, INVENTED_LIMIT};

/// The largest search shift: the cadence's period is $2^{\text{shift}}$ ticks, and a shift at
/// the width of the tick counter is no cadence.
pub const SEARCH_SHIFT_MAX: u8 = 63;

/// 64-byte induction record (whitepaper §5.2.30; ADR-0052). Equal to `Default` at rest, with
/// the invented band's first id as the next invention; an all-zero record is not this one
/// (its next invention is outside the band) and the loader refuses it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct InductionState {
    pub free: u32, // [0..4] Nodes of the arena in use: the cursor the next node takes
    pub next_variable: u32, // [4..8] The number the next variable the operators create takes
    pub next_invented: u32, // [8..12] The id the next invention takes, within the invented band
    pub clauses: u32, // [12..16] The clause store's length
    pub resume_i: u32, // [16..20] The pair the next search resumes after, first index + 1; 0 = the start
    pub resume_j: u32, // [20..24] Its second index + 1; 0 with resume_i = 0
    pub search_budget: u32, // [24..28] Attempts one search may spend
    pub search_shift: u8, // [28] The search's cadence, 2^shift ticks; 0 never searches
    pub tag: u8,       // [29] The REM ripples an invention's episode survives
    pub _pad: u16,     // [30..32] Reserved; MUST be zero
    pub _reserved: [u8; 32], // [32..64] Reserved; MUST be zero
}

impl InductionState {
    /// The record at rest: an empty arena and store, the next invention the band's first id,
    /// the cursor at the start, no budget, no cadence, no tag. Equal to `Default`.
    pub const fn new() -> Self {
        Self {
            free: 0,
            next_variable: 0,
            next_invented: INVENTED_BASE,
            clauses: 0,
            resume_i: 0,
            resume_j: 0,
            search_budget: 0,
            search_shift: 0,
            tag: 0,
            _pad: 0,
            _reserved: [0; 32],
        }
    }

    /// The pair after which the next search resumes, decoded: `None` for the start.
    pub const fn resume(&self) -> Option<(usize, usize)> {
        // Both are index + 1, zero the start; a well-formed record has both or neither.
        match (self.resume_i.checked_sub(1), self.resume_j.checked_sub(1)) {
            (Some(i), Some(j)) => Some((i as usize, j as usize)),
            _ => None,
        }
    }

    /// Sets the pair after which the next search resumes; `None` is the start. Refused, with
    /// nothing changed, for a pair the width cannot encode or that is not `i < j`.
    pub fn set_resume(&mut self, after: Option<(usize, usize)>) -> bool {
        match after {
            None => {
                self.resume_i = 0;
                self.resume_j = 0;
                true
            }
            Some((i, j)) => {
                let (Ok(i), Ok(j)) = (u32::try_from(i), u32::try_from(j)) else {
                    return false;
                };
                let (Some(ri), Some(rj)) = (i.checked_add(1), j.checked_add(1)) else {
                    return false;
                };
                if i >= j {
                    return false;
                }
                self.resume_i = ri;
                self.resume_j = rj;
                true
            }
        }
    }

    /// True for a record the rules keep: the next invention within the band, the shift at
    /// most [`SEARCH_SHIFT_MAX`], the cursor at the start or a pair `i < j` below the store's
    /// length, the pad and the reserved bytes zero. Whether `free` and `clauses` are the
    /// sections' counts is the loader's check (ADR-0028).
    pub fn is_well_formed(&self) -> bool {
        let cursor = match (self.resume_i, self.resume_j) {
            (0, 0) => true,
            (0, _) | (_, 0) => false,
            (i, j) => i < j && j <= self.clauses,
        };
        (INVENTED_BASE..=INVENTED_LIMIT).contains(&self.next_invented)
            && self.search_shift <= SEARCH_SHIFT_MAX
            && cursor
            && self._pad == 0
            && self._reserved == [0; 32]
    }

    /// The record's 64 bytes, little-endian, field by field (§8.7).
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..4].copy_from_slice(&self.free.to_le_bytes());
        out[4..8].copy_from_slice(&self.next_variable.to_le_bytes());
        out[8..12].copy_from_slice(&self.next_invented.to_le_bytes());
        out[12..16].copy_from_slice(&self.clauses.to_le_bytes());
        out[16..20].copy_from_slice(&self.resume_i.to_le_bytes());
        out[20..24].copy_from_slice(&self.resume_j.to_le_bytes());
        out[24..28].copy_from_slice(&self.search_budget.to_le_bytes());
        out[28] = self.search_shift;
        out[29] = self.tag;
        out[30..32].copy_from_slice(&self._pad.to_le_bytes());
        out[32..64].copy_from_slice(&self._reserved);
        out
    }

    /// A record from its 64 bytes; not validated (`is_well_formed` is the check).
    pub fn decode(bytes: &[u8; 64]) -> Self {
        let u32_at = |at: usize| {
            u32::from_le_bytes(bytes[at..at.wrapping_add(4)].try_into().unwrap_or([0; 4]))
        };
        Self {
            free: u32_at(0),
            next_variable: u32_at(4),
            next_invented: u32_at(8),
            clauses: u32_at(12),
            resume_i: u32_at(16),
            resume_j: u32_at(20),
            search_budget: u32_at(24),
            search_shift: bytes[28],
            tag: bytes[29],
            _pad: u16::from_le_bytes(bytes[30..32].try_into().unwrap_or([0; 2])),
            _reserved: bytes[32..64].try_into().unwrap_or([0; 32]),
        }
    }
}

impl Default for InductionState {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::size_of::<InductionState>() == 64);
    assert!(core::mem::align_of::<InductionState>() == 64);
    assert!((SEARCH_SHIFT_MAX as u32) < u64::BITS);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_record_at_rest_is_well_formed_and_an_all_zero_one_is_not() {
        let rest = InductionState::new();
        assert_eq!(rest, InductionState::default());
        assert!(rest.is_well_formed());
        assert_eq!(rest.resume(), None);
        assert_eq!(rest.next_invented, INVENTED_BASE);
        let zero = InductionState::decode(&[0; 64]);
        assert!(
            !zero.is_well_formed(),
            "a next invention of zero is outside the band"
        );
        assert_eq!(InductionState::decode(&rest.encode()), rest);
    }

    #[test]
    fn the_cursor_encodes_a_pair_as_index_plus_one_and_refuses_what_is_not_a_pair() {
        let mut s = InductionState {
            clauses: 5,
            ..InductionState::new()
        };
        assert!(s.set_resume(Some((1, 4))));
        assert_eq!((s.resume_i, s.resume_j), (2, 5));
        assert_eq!(s.resume(), Some((1, 4)));
        assert!(s.is_well_formed(), "j + 1 at the length: the last pair");
        assert!(!s.set_resume(Some((4, 4))), "not i < j");
        assert!(!s.set_resume(Some((4, 1))));
        assert!(!s.set_resume(Some((0, u32::MAX as usize))), "j + 1 wraps");
        assert!(
            !s.set_resume(Some((u32::MAX as usize, u32::MAX as usize + 1))),
            "i wraps"
        );
        assert_eq!(s.resume(), Some((1, 4)), "a refusal changes nothing");
        assert!(s.set_resume(None));
        assert_eq!((s.resume_i, s.resume_j), (0, 0));
        assert!(s.set_resume(Some((0, 1))));
        assert_eq!(s.resume(), Some((0, 1)));
    }

    #[test]
    fn the_record_is_well_formed_on_each_clause_alone() {
        let good = InductionState {
            free: 100,
            next_variable: 7,
            next_invented: INVENTED_BASE + 3,
            clauses: 5,
            resume_i: 1,
            resume_j: 5,
            search_budget: 64,
            search_shift: SEARCH_SHIFT_MAX,
            tag: 200,
            ..InductionState::new()
        };
        assert!(good.is_well_formed());
        let mut at_limit = good;
        at_limit.next_invented = INVENTED_LIMIT;
        assert!(
            at_limit.is_well_formed(),
            "every id taken: the operators refuse the next one"
        );
        type Mutation = fn(&mut InductionState);
        let clauses: [(&str, Mutation); 9] = [
            ("the next invention below the band", |s| {
                s.next_invented = INVENTED_BASE - 1
            }),
            ("the next invention past the band", |s| {
                s.next_invented = INVENTED_LIMIT + 1
            }),
            ("a shift at the width", |s| {
                s.search_shift = SEARCH_SHIFT_MAX + 1
            }),
            ("a cursor with only its first index", |s| s.resume_j = 0),
            ("a cursor with only its second index", |s| s.resume_i = 0),
            ("a cursor that is not i < j", |s| s.resume_i = 5),
            ("a cursor past the store", |s| s.resume_j = 6),
            ("the pad", |s| s._pad = 1),
            ("a reserved byte", |s| s._reserved[31] = 1),
        ];
        for (what, clause) in clauses {
            let mut bad = good;
            clause(&mut bad);
            assert!(!bad.is_well_formed(), "{what}");
        }
    }

    #[test]
    fn the_record_round_trips_through_its_bytes_field_by_field() {
        let s = InductionState {
            free: 0x0102_0304,
            next_variable: 0x0506_0708,
            next_invented: 0xFFFE_0009,
            clauses: 0x0A0B_0C0D,
            resume_i: 0x1112_1314,
            resume_j: 0x1516_1718,
            search_budget: 0x191A_1B1C,
            search_shift: 0x1D,
            tag: 0x1E,
            _pad: 0x2021,
            _reserved: [0x22; 32],
        };
        let bytes = s.encode();
        assert_eq!(&bytes[0..4], &[4, 3, 2, 1]);
        assert_eq!(&bytes[8..12], &[9, 0, 0xFE, 0xFF]);
        assert_eq!(&bytes[24..28], &[0x1C, 0x1B, 0x1A, 0x19]);
        assert_eq!(&bytes[28..32], &[0x1D, 0x1E, 0x21, 0x20]);
        assert_eq!(&bytes[32..64], &[0x22; 32]);
        assert_eq!(InductionState::decode(&bytes), s);
    }
}
