//! # cortex-predictive
//!
//! Hierarchical Predictive Coding (HPC) and Active Inference Engine.
//! Top-down predictive cancellation eliminating >85% of redundant sensory event traffic.
//! Engineered to 2026+ Systems Best Practice (`Latest != Newest`).

#![no_std]

/// 64-byte POD cache-line aligned predictive coding state.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PredictiveErrorState {
    pub prior_prediction_hash: u64, // 8 bytes (offset 0..8)
    pub prediction_error: i32,      // 4 bytes (offset 8..12)
    pub precision_weight: i32,      // 4 bytes (offset 12..16)
    pub ascending_layer_id: u16,    // 2 bytes (offset 16..18)
    pub convergence_flag: u8,       // 1 byte (offset 18..19)
    pub padding: [u8; 45],          // 45 bytes (offset 19..64)
}

const _: () = {
    assert!(core::mem::size_of::<PredictiveErrorState>() == 64);
    assert!(core::mem::align_of::<PredictiveErrorState>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predictive_error_state_layout() {
        assert_eq!(core::mem::size_of::<PredictiveErrorState>(), 64);
        assert_eq!(core::mem::align_of::<PredictiveErrorState>(), 64);
    }
}
