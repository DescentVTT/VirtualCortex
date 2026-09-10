use core::sync::atomic::{AtomicU8, AtomicU64};

/// 64-Byte POD Cache-Line Aligned Matthew Larkum BAC Dendritic Super-Neuron
// Control record (whitepaper §8.2, rule L-5): holds atomics, so it is Sync but not Copy.
#[derive(Debug)]
#[repr(C, align(64))]
pub struct DendriticSuperNeuron {
    pub id: u64,                     // [0..8] Global neuron ID
    pub mailbox_head_ptr: AtomicU64, // [8..16] Lock-free MPSC mailbox pointer
    pub mailbox_tag: u64,            // [16..24] 64-bit ABA tag
    pub v_soma: i32,                 // [24..28] Soma potential (Q16.16)
    pub v_basal: i32,                // [28..32] Basal feedforward potential (Q16.16)
    pub v_apical: i32,               // [32..36] Apical contextual potential (Q16.16)
    pub v_thresh: i32,               // [36..40] Dynamic adaptive threshold (Q16.16)
    pub bac_plateau_ticks: u16,      // [40..42] Larkum BAC calcium burst countdown
    pub refractory_ticks: u16,       // [42..44] Absolute refractory countdown
    pub last_soma_spike_tick: u32,   // [44..48] Somatic action potential timestamp
    pub synapse_slab_idx: u32,       // [48..52] Index into SynapseBlock arena
    pub plastic_delta_head: u16,     // [52..54] Index into CXL.mem delta table
    pub spatial_voxel_morton: u16,   // [54..56] 16-bit Morton spatial voxel code
    pub gate_state: AtomicU8,        // [56] Virtual actor state machine flag
    pub flags: u8,                   // [57] BURST_MODE / Inhibitory Flags
    pub stp_r_ves: u8,               // [58] Tsodyks-Markram vesicle pool (STD), Q0.8
    pub stp_u_rel: u8,               // [59] Tsodyks-Markram release fraction (STF), Q0.8
    pub _reserved: [u8; 4],          // [60..64] Hardware cache-line alignment padding
}

/// Strict 64-byte synaptic connection block
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SynapseBlock {
    pub target_neuron_ids: [u32; 4], // [0..16] 4 target neuron indices
    pub weights_q1_15: [i16; 4],     // [16..24] 4 base weights, Q1.15 (ADR-0012)
    pub delays_ticks: [u16; 4],      // [24..32] Axonal transmission delays
    pub next_block_idx: u32,         // [32..36] Index to chained overflow block
    pub last_spike_tick: u32,        // [36..40] Synapse timestamp for STDP
    pub _reserved: [u8; 24],         // [40..64] Cache-line alignment padding
}

/// Synaptic efficacy in Q16.16 from a Q1.15 base weight and the two Q0.8 short-term
/// plasticity factors (ADR-0012).
///
/// The product carries 15 + 8 + 8 = 31 fractional bits and is formed exactly in `i64`; one
/// arithmetic shift by 15 yields Q16.16. No intermediate rounding occurs, so every bit of the
/// Q1.15 weight survives into the result, scaled by the coarser STP factors. The magnitude is
/// bounded by 1.0 (65 536), so the result always fits an `i32`; the shift floors toward
/// negative infinity, consistent with whitepaper §8.1.
#[inline(always)]
pub const fn synaptic_efficacy_q16(w_q1_15: i16, u_q0_8: u8, r_q0_8: u8) -> i32 {
    ((w_q1_15 as i64 * u_q0_8 as i64 * r_q0_8 as i64) >> 15) as i32
}

const _: () = {
    assert!(core::mem::size_of::<DendriticSuperNeuron>() == 64);
    assert!(core::mem::align_of::<DendriticSuperNeuron>() == 64);
    assert!(core::mem::size_of::<SynapseBlock>() == 64);
    assert!(core::mem::align_of::<SynapseBlock>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const Q16_ONE: i32 = 0x0001_0000;

    #[test]
    fn full_weight_and_full_stp_is_just_below_one() {
        // 32767 × 255 × 255 = 2 130 674 175; >> 15 floors to 65 023 (0.99217 in Q16.16), below 1.0.
        let e = synaptic_efficacy_q16(i16::MAX, 255, 255);
        assert_eq!(e, 65_023);
        assert!(e < Q16_ONE);
    }

    #[test]
    fn most_negative_weight_stays_in_range() {
        // −1.0 × 255/256 × 255/256 = −0.99221 → −65 025; the i64 product cannot overflow.
        assert_eq!(synaptic_efficacy_q16(i16::MIN, 255, 255), -65_025);
    }

    #[test]
    fn depleted_resource_or_zero_release_silences_the_synapse() {
        assert_eq!(synaptic_efficacy_q16(i16::MAX, 255, 0), 0);
        assert_eq!(synaptic_efficacy_q16(i16::MAX, 0, 255), 0);
    }

    #[test]
    fn half_weight_half_release_full_resource() {
        // 0.5 × 0.5 × 0.996 = 0.249 → 16 320 in Q16.16, exactly.
        assert_eq!(synaptic_efficacy_q16(0x4000, 128, 255), 16_320);
    }

    #[test]
    fn one_lsb_of_weight_survives_at_full_stp_and_vanishes_at_half() {
        assert_eq!(synaptic_efficacy_q16(1, 255, 255), 1);
        assert_eq!(synaptic_efficacy_q16(1, 128, 128), 0);
    }

    #[test]
    fn flooring_is_toward_negative_infinity() {
        // −65 025 >> 15 = −1.98 floors to −2, not −1.
        assert_eq!(synaptic_efficacy_q16(-1, 255, 255), -2);
    }

    #[test]
    fn efficacy_is_monotonic_in_each_factor() {
        let base = synaptic_efficacy_q16(0x2000, 100, 100);
        assert!(synaptic_efficacy_q16(0x2001, 100, 100) >= base);
        assert!(synaptic_efficacy_q16(0x2000, 101, 100) >= base);
        assert!(synaptic_efficacy_q16(0x2000, 100, 101) >= base);
    }
}
