//! The bytes of the two arena records, for the `.cortex` image (whitepaper §8.7; ADR-0024).
//! Explicit little-endian field by field: no `unsafe`, no transmute, the same bytes on every
//! target (§8.3). The atomics of `DendriticSuperNeuron` are stored as their plain values and
//! an image at rest holds them zero: an idle gate and an empty mailbox.

use crate::dynamics::neuron::{DendriticSuperNeuron, GateState, MAILBOX_EMPTY, SynapseBlock};
use core::sync::atomic::{AtomicU8, AtomicU64, Ordering};

// The readers take the field's own bytes, sliced by a literal range at the call site, so that
// no offset is computed: the layout is written once, as the same ranges `encode` writes.
fn u16_le(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}

fn u32_le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn u64_le(b: &[u8]) -> u64 {
    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

impl DendriticSuperNeuron {
    /// True when the record may be written to an image at rest: the gate idle and the mailbox
    /// empty (§8.7).
    pub fn is_image_ready(&self) -> bool {
        self.gate() == Some(GateState::Idle) && self.mailbox_is_empty()
    }

    /// The record's 64 bytes, little-endian, atomics as their current values.
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[0..8].copy_from_slice(&self.id.to_le_bytes());
        out[8..16].copy_from_slice(&self.mailbox_head_ptr.load(Ordering::SeqCst).to_le_bytes());
        out[16..24].copy_from_slice(&self.mailbox_reserved.to_le_bytes());
        out[24..28].copy_from_slice(&self.v_soma.to_le_bytes());
        out[28..32].copy_from_slice(&self.v_basal.to_le_bytes());
        out[32..36].copy_from_slice(&self.v_apical.to_le_bytes());
        out[36..40].copy_from_slice(&self.v_thresh.to_le_bytes());
        out[40..42].copy_from_slice(&self.bac_plateau_ticks.to_le_bytes());
        out[42..44].copy_from_slice(&self.refractory_ticks.to_le_bytes());
        out[44..48].copy_from_slice(&self.last_soma_spike_tick.to_le_bytes());
        out[48..52].copy_from_slice(&self.synapse_slab_idx.to_le_bytes());
        out[52..54].copy_from_slice(&self._reserved.to_le_bytes());
        out[54..56].copy_from_slice(&self.spatial_voxel_morton.to_le_bytes());
        out[56] = self.gate_state.load(Ordering::SeqCst);
        out[57] = self.flags;
        out[58] = self.stp_r_ves;
        out[59] = self.stp_u_rel;
        out[60..64].copy_from_slice(&self.plastic_delta_head.to_le_bytes());
        out
    }

    /// A record from its 64 bytes.
    pub fn decode(bytes: &[u8; 64]) -> Self {
        Self {
            id: u64_le(&bytes[0..8]),
            mailbox_head_ptr: AtomicU64::new(u64_le(&bytes[8..16])),
            mailbox_reserved: u64_le(&bytes[16..24]),
            v_soma: u32_le(&bytes[24..28]) as i32,
            v_basal: u32_le(&bytes[28..32]) as i32,
            v_apical: u32_le(&bytes[32..36]) as i32,
            v_thresh: u32_le(&bytes[36..40]) as i32,
            bac_plateau_ticks: u16_le(&bytes[40..42]),
            refractory_ticks: u16_le(&bytes[42..44]),
            last_soma_spike_tick: u32_le(&bytes[44..48]),
            synapse_slab_idx: u32_le(&bytes[48..52]),
            _reserved: u16_le(&bytes[52..54]),
            spatial_voxel_morton: u16_le(&bytes[54..56]),
            gate_state: AtomicU8::new(bytes[56]),
            flags: bytes[57],
            stp_r_ves: bytes[58],
            stp_u_rel: bytes[59],
            plastic_delta_head: u32_le(&bytes[60..64]),
        }
    }

    /// Copies every plain field of `from` into this record, leaving the gate and the mailbox
    /// head as they are: what re-hydration does to a slot that has already received a message
    /// (ADR-0024).
    pub fn restore_plain_fields(&mut self, from: &DendriticSuperNeuron) {
        self.id = from.id;
        self.mailbox_reserved = from.mailbox_reserved;
        self.v_soma = from.v_soma;
        self.v_basal = from.v_basal;
        self.v_apical = from.v_apical;
        self.v_thresh = from.v_thresh;
        self.bac_plateau_ticks = from.bac_plateau_ticks;
        self.refractory_ticks = from.refractory_ticks;
        self.last_soma_spike_tick = from.last_soma_spike_tick;
        self.synapse_slab_idx = from.synapse_slab_idx;
        self._reserved = from._reserved;
        self.spatial_voxel_morton = from.spatial_voxel_morton;
        self.flags = from.flags;
        self.stp_r_ves = from.stp_r_ves;
        self.stp_u_rel = from.stp_u_rel;
        self.plastic_delta_head = from.plastic_delta_head;
    }

    /// True when every field, atomics included, equals `other`'s.
    pub fn same_bytes(&self, other: &DendriticSuperNeuron) -> bool {
        self.encode() == other.encode()
    }

    /// Whether the record decodes to an image at rest: an idle gate, an empty mailbox and zero
    /// reserved bytes (§8.7).
    pub fn is_at_rest_image(&self) -> bool {
        self.is_image_ready()
            && self.mailbox_reserved == 0
            && self._reserved == 0
            && self.mailbox_head_ptr.load(Ordering::SeqCst) == MAILBOX_EMPTY
    }
}

impl SynapseBlock {
    /// The record's 64 bytes, little-endian.
    pub fn encode(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        // Each per-slot array is one literal range of the record, cut into one chunk per slot.
        for (dst, t) in out[0..16].chunks_exact_mut(4).zip(&self.target_neuron_ids) {
            dst.copy_from_slice(&t.to_le_bytes());
        }
        for (dst, w) in out[16..24].chunks_exact_mut(2).zip(&self.weights_q1_15) {
            dst.copy_from_slice(&w.to_le_bytes());
        }
        for (dst, d) in out[24..32].chunks_exact_mut(2).zip(&self.delays_ticks) {
            dst.copy_from_slice(&d.to_le_bytes());
        }
        out[32..36].copy_from_slice(&self.chain.to_le_bytes());
        out[36..40].copy_from_slice(&self.last_spike_tick.to_le_bytes());
        for (dst, r) in out[40..56].chunks_exact_mut(4).zip(&self.last_release_q16) {
            dst.copy_from_slice(&r.to_le_bytes());
        }
        for (dst, e) in out[56..64].chunks_exact_mut(2).zip(&self.eligibility_q1_15) {
            dst.copy_from_slice(&e.to_le_bytes());
        }
        out
    }

    /// A record from its 64 bytes.
    pub fn decode(bytes: &[u8; 64]) -> Self {
        let mut b = Self::new();
        for (dst, src) in b
            .target_neuron_ids
            .iter_mut()
            .zip(bytes[0..16].chunks_exact(4))
        {
            *dst = u32_le(src);
        }
        for (dst, src) in b
            .weights_q1_15
            .iter_mut()
            .zip(bytes[16..24].chunks_exact(2))
        {
            *dst = u16_le(src) as i16;
        }
        for (dst, src) in b.delays_ticks.iter_mut().zip(bytes[24..32].chunks_exact(2)) {
            *dst = u16_le(src);
        }
        for (dst, src) in b
            .last_release_q16
            .iter_mut()
            .zip(bytes[40..56].chunks_exact(4))
        {
            *dst = u32_le(src) as i32;
        }
        for (dst, src) in b
            .eligibility_q1_15
            .iter_mut()
            .zip(bytes[56..64].chunks_exact(2))
        {
            *dst = u16_le(src) as i16;
        }
        b.chain = u32_le(&bytes[32..36]);
        b.last_spike_tick = u32_le(&bytes[36..40]);
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamics::neuron::MailboxNode;

    #[test]
    fn a_reserved_byte_that_is_not_zero_is_not_at_rest() {
        let mut bytes = DendriticSuperNeuron::new(1).encode();
        bytes[52] = 1;
        let u = DendriticSuperNeuron::decode(&bytes);
        assert_eq!(u._reserved, 1);
        assert!(u.is_image_ready(), "the gate and the mailbox are fine");
        assert!(!u.is_at_rest_image(), "but the reserved bytes are not zero");
        let mut bytes = DendriticSuperNeuron::new(1).encode();
        bytes[16] = 1;
        assert!(!DendriticSuperNeuron::decode(&bytes).is_at_rest_image());
    }

    #[test]
    fn a_unit_round_trips_through_its_bytes_and_a_unit_at_rest_is_image_ready() {
        let mut u = DendriticSuperNeuron::new(0x0102_0304_0506_0708);
        u.v_soma = -0x4000;
        u.v_basal = 7;
        u.v_apical = -1;
        u.v_thresh = 0x0001_0000;
        u.bac_plateau_ticks = 3;
        u.refractory_ticks = 200;
        u.last_soma_spike_tick = 0xDEAD_BEEF;
        assert!(u.set_first_block(41));
        u.spatial_voxel_morton = 0x1234;
        u.flags = 0x03;
        u.stp_r_ves = 200;
        u.stp_u_rel = 51;
        assert!(u.set_delta_head(0x00FF_FFFF));
        assert!(u.is_image_ready());
        assert!(u.is_at_rest_image());
        let bytes = u.encode();
        assert_eq!(&bytes[0..8], &0x0102_0304_0506_0708u64.to_le_bytes());
        assert_eq!(bytes[56], 0, "idle gate");
        assert_eq!(
            &bytes[60..64],
            &0x0100_0000u32.to_le_bytes(),
            "the head is index + 1"
        );
        let back = DendriticSuperNeuron::decode(&bytes);
        assert!(back.same_bytes(&u));
        assert_eq!(back.delta_head(), Some(0x00FF_FFFF));
        assert_eq!(back.first_block(), Some(41));
        let mut changed = bytes;
        changed[30] ^= 1;
        assert!(!DendriticSuperNeuron::decode(&changed).same_bytes(&u));
    }

    #[test]
    fn a_running_unit_or_a_full_mailbox_is_not_image_ready() {
        let u = DendriticSuperNeuron::new(1);
        assert!(u.try_schedule());
        assert!(!u.is_image_ready(), "scheduled");
        assert!(u.begin_turn());
        assert!(!u.is_image_ready(), "running");
        assert!(!u.end_turn());
        assert!(u.is_image_ready());
        let nodes = [MailboxNode::new()];
        assert!(u.mailbox_push(&nodes, 0, 5));
        assert!(!u.is_image_ready(), "a message waits");
        let bytes = u.encode();
        assert_eq!(
            &bytes[8..16],
            &1u64.to_le_bytes(),
            "the head is stored as its value"
        );
        assert!(!DendriticSuperNeuron::decode(&bytes).is_at_rest_image());
    }

    #[test]
    fn restore_keeps_the_gate_and_the_mailbox_of_the_slot() {
        let mut cold = DendriticSuperNeuron::new(9);
        cold.v_soma = 123;
        cold.v_thresh = 456;
        cold.stp_u_rel = 77;
        let nodes = [MailboxNode::new()];
        let mut slot = DendriticSuperNeuron::new(9);
        assert!(slot.mailbox_push(&nodes, 0, 42));
        assert!(slot.try_schedule());
        slot.restore_plain_fields(&cold);
        assert_eq!((slot.v_soma, slot.v_thresh, slot.stp_u_rel), (123, 456, 77));
        assert_eq!(
            slot.gate(),
            Some(GateState::Scheduled),
            "the gate is the slot's"
        );
        assert!(!slot.mailbox_is_empty(), "the message is the slot's");
    }

    #[test]
    fn a_block_round_trips_through_its_bytes() {
        let mut b = SynapseBlock::new();
        assert!(b.set_synapse(0, 5, i16::MIN, 2559, true));
        assert!(b.set_synapse(3, 0, 0x1234, 0, false));
        assert!(b.link(77));
        b.stamp_presynaptic(0xABCD);
        assert_eq!(b.release(0, 255, 255), -65_025);
        b.eligibility_q1_15 = [-2, 0, 0, 0x0102];
        let bytes = b.encode();
        assert_eq!(&bytes[0..4], &6u32.to_le_bytes(), "target 5 is stored as 6");
        assert_eq!(
            &bytes[32..36],
            &(78u32 | 1 << 28).to_le_bytes(),
            "the chain word: block 77 as 78, slot 0's apical bit at bit 28"
        );
        assert_eq!(
            &bytes[56..64],
            &[0xFE, 0xFF, 0, 0, 0, 0, 0x02, 0x01],
            "the traces at [56..64), little-endian"
        );
        assert_eq!(SynapseBlock::decode(&bytes), b);
        assert_eq!(SynapseBlock::decode(&[0; 64]), SynapseBlock::new());
    }
}
