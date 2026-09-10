//! Hippocampus: attractor, replay scheduling and place-field state (whitepaper §5.2.15). The
//! dynamics are Specified (§8.8); the metric map is `cortex-spatial`'s (ADR-0016).

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct HippocampalAttractorState {
    pub dg_sparsity_bits: u32,     // Dentate Gyrus pattern separation sparsity
    pub ca3_recurrent_energy: i32, // CA3 auto-associative energy (Q16.16)
    pub ca1_comparator_error: i32, // CA1 pattern matching error (Q16.16)
    pub swr_replay_ticks: u32,     // Sharp-wave ripple replay cycle countdown
    pub grid_theta_phase: u32,     // Toroidal grid cell attractor phase
    pub place_field_id: u32,       // Currently mapped spatial place field
    pub _reserved: [u8; 40],       // Strict 64-byte alignment
}

const _: () = {
    assert!(core::mem::size_of::<HippocampalAttractorState>() == 64);
    assert!(core::mem::align_of::<HippocampalAttractorState>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_is_one_cache_line() {
        assert_eq!(core::mem::size_of::<HippocampalAttractorState>(), 64);
        assert_eq!(core::mem::align_of::<HippocampalAttractorState>(), 64);
    }
}
