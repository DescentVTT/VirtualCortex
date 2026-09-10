//! The Tier-2 plastic delta: a 16-byte record of one weight change waiting to be applied to a
//! synapse in far memory (whitepaper §8.6, Appendix A row 37; ADR-0024; finding F-20). A
//! unit's deltas form an index-chained list from `DendriticSuperNeuron::plastic_delta_head`,
//! with the index + 1 encoding of ADR-0022 so that a zeroed arena holds no deltas. This round
//! stores, links, walks and serialises deltas; applying them is Specified (milestone M5).

use super::neuron::DendriticSuperNeuron;
use super::synapse::SYNAPSES_PER_BLOCK;

/// `next` of the last delta of a list, and `plastic_delta_head` of a unit without deltas.
pub const DELTA_END: u32 = 0;

/// One pending weight change, 16 bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(16))]
pub struct PlasticDelta {
    pub block_idx: u32, // [0..4] The SynapseBlock the delta applies to, as index + 1; 0 = none
    pub slot: u8,       // [4] Slot within the block, 0 to 3
    pub _pad: u8,       // [5] Reserved; MUST be zero
    pub delta_q1_15: i16, // [6..8] The change, Q1.15, added saturating when applied
    pub epoch: u32,     // [8..12] The epoch (1 ms) the change was recorded in
    pub next: u32,      // [12..16] Next delta of the same unit's list, as index + 1; 0 = end
}

impl PlasticDelta {
    /// A delta for `slot` of `block_idx`. `None` for a slot that does not exist or a block index
    /// the encoding cannot hold.
    pub const fn new(block_idx: u32, slot: u8, delta_q1_15: i16, epoch: u32) -> Option<Self> {
        if block_idx == u32::MAX || slot as usize >= SYNAPSES_PER_BLOCK {
            return None;
        }
        Some(Self {
            block_idx: block_idx + 1,
            slot,
            _pad: 0,
            delta_q1_15,
            epoch,
            next: DELTA_END,
        })
    }

    /// The block the delta applies to, decoded, or `None` for an empty record.
    pub const fn block(&self) -> Option<u32> {
        if self.block_idx == 0 {
            None
        } else {
            Some(self.block_idx - 1)
        }
    }

    /// The next delta of the list, decoded, or `None` at the end.
    pub const fn next_delta(&self) -> Option<u32> {
        if self.next == DELTA_END {
            None
        } else {
            Some(self.next - 1)
        }
    }

    /// Links `next_idx` after this delta. Refused for `u32::MAX`.
    pub fn link(&mut self, next_idx: u32) -> bool {
        if next_idx == u32::MAX {
            return false;
        }
        self.next = next_idx + 1;
        true
    }

    /// The record's bytes, little-endian.
    pub fn encode(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[0..4].copy_from_slice(&self.block_idx.to_le_bytes());
        out[4] = self.slot;
        out[5] = self._pad;
        out[6..8].copy_from_slice(&self.delta_q1_15.to_le_bytes());
        out[8..12].copy_from_slice(&self.epoch.to_le_bytes());
        out[12..16].copy_from_slice(&self.next.to_le_bytes());
        out
    }

    /// A record from its bytes.
    pub fn decode(bytes: &[u8; 16]) -> Self {
        Self {
            block_idx: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            slot: bytes[4],
            _pad: bytes[5],
            delta_q1_15: i16::from_le_bytes([bytes[6], bytes[7]]),
            epoch: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            next: u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }

    /// The deltas of the list that starts at `head` (encoded as a unit stores it), in order,
    /// bounded by the arena so that a corrupt or cyclic list terminates.
    pub fn chain(deltas: &[PlasticDelta], head: u32) -> DeltaChain<'_> {
        DeltaChain {
            deltas,
            next: head,
            remaining: deltas.len(),
        }
    }
}

/// The indices of a delta list, with the same termination as `FanOut`.
#[derive(Debug)]
pub struct DeltaChain<'a> {
    deltas: &'a [PlasticDelta],
    next: u32,
    remaining: usize,
}

impl Iterator for DeltaChain<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.next == DELTA_END || self.remaining == 0 {
            return None;
        }
        let idx = self.next - 1;
        let Some(delta) = self.deltas.get(idx as usize) else {
            self.next = DELTA_END;
            return None;
        };
        self.remaining -= 1;
        self.next = delta.next;
        Some(idx)
    }
}

impl DendriticSuperNeuron {
    /// The first delta of the unit's list, decoded, or `None` for a unit without deltas.
    pub const fn delta_head(&self) -> Option<u32> {
        if self.plastic_delta_head == DELTA_END {
            None
        } else {
            Some(self.plastic_delta_head - 1)
        }
    }

    /// Names the first delta of the unit's list. Refused for `u32::MAX`.
    pub fn set_delta_head(&mut self, delta_idx: u32) -> bool {
        if delta_idx == u32::MAX {
            return false;
        }
        self.plastic_delta_head = delta_idx + 1;
        true
    }

    /// Pushes delta `delta_idx` of `deltas` at the head of the unit's list (the delta's `next`
    /// becomes the old head). Refused for an index outside the arena or one the encoding
    /// cannot hold.
    pub fn push_delta(&mut self, deltas: &mut [PlasticDelta], delta_idx: u32) -> bool {
        if delta_idx == u32::MAX {
            return false;
        }
        let Some(delta) = deltas.get_mut(delta_idx as usize) else {
            return false;
        };
        delta.next = self.plastic_delta_head;
        self.plastic_delta_head = delta_idx + 1;
        true
    }

    /// The unit's deltas in `deltas`, most recently pushed first.
    pub fn deltas<'a>(&self, deltas: &'a [PlasticDelta]) -> DeltaChain<'a> {
        PlasticDelta::chain(deltas, self.plastic_delta_head)
    }
}

const _: () = {
    assert!(core::mem::size_of::<PlasticDelta>() == 16);
    assert!(core::mem::align_of::<PlasticDelta>() == 16);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_zeroed_delta_is_empty_and_the_record_is_sixteen_bytes() {
        let d = PlasticDelta::default();
        assert_eq!((d.block(), d.next_delta()), (None, None));
        assert_eq!(core::mem::size_of::<PlasticDelta>(), 16);
        assert_eq!(PlasticDelta::new(u32::MAX, 0, 1, 1), None);
        assert_eq!(PlasticDelta::new(0, 4, 1, 1), None);
        let d = PlasticDelta::new(0, 3, -5, 9).unwrap();
        assert_eq!(
            (d.block(), d.slot, d.delta_q1_15, d.epoch),
            (Some(0), 3, -5, 9)
        );
        assert_eq!(d.block_idx, 1, "block 0 is index + 1");
    }

    #[test]
    fn deltas_round_trip_through_bytes() {
        let mut d = PlasticDelta::new(0x0102_0304, 2, i16::MIN, 0xDEAD_BEEF).unwrap();
        assert!(d.link(7));
        assert!(!d.link(u32::MAX));
        let bytes = d.encode();
        assert_eq!(&bytes[0..4], &0x0102_0305u32.to_le_bytes());
        assert_eq!(PlasticDelta::decode(&bytes), d);
        assert_eq!(d.next_delta(), Some(7));
    }

    #[test]
    fn a_unit_pushes_deltas_at_the_head_and_walks_them_most_recent_first() {
        let mut deltas = [PlasticDelta::default(); 4];
        for (i, d) in deltas.iter_mut().enumerate() {
            *d = PlasticDelta::new(i as u32, 0, i as i16, 0).unwrap();
        }
        let mut unit = DendriticSuperNeuron::new(1);
        assert_eq!(unit.delta_head(), None);
        assert!(unit.push_delta(&mut deltas, 0));
        assert!(unit.push_delta(&mut deltas, 2));
        assert!(unit.push_delta(&mut deltas, 3));
        assert!(!unit.push_delta(&mut deltas, 4), "outside the arena");
        assert!(!unit.push_delta(&mut deltas, u32::MAX));
        assert_eq!(unit.delta_head(), Some(3));
        let walked: [Option<u32>; 4] = {
            let mut it = unit.deltas(&deltas);
            [it.next(), it.next(), it.next(), it.next()]
        };
        assert_eq!(walked, [Some(3), Some(2), Some(0), None]);
        assert_eq!(deltas[3].next_delta(), Some(2));
        assert_eq!(deltas[0].next_delta(), None);
        // A cycle terminates within the arena.
        assert!(deltas[0].link(3));
        assert_eq!(unit.deltas(&deltas).count(), 4);
        // A head outside the arena walks nothing.
        assert!(unit.set_delta_head(9));
        assert_eq!(unit.deltas(&deltas).count(), 0);
        assert!(!unit.set_delta_head(u32::MAX));
    }
}
