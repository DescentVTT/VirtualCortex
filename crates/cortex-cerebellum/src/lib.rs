//! Cerebellar Internal Forward Models & Microsecond Motor Coordination

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct CerebellarMicrozone {
    pub microzone_id: u32,           // [0..4] Anatomical microzone identifier
    pub purkinje_output_rate: i32,   // [4..8] High-frequency Purkinje inhibition (Q16.16)
    pub mossy_fiber_input: i32,      // [8..12] Sensorimotor state input (Q16.16)
    pub granule_expansion_code: u32, // [12..16] High-dimensional sparse pattern hash
    pub climbing_fiber_error: i32,   // [16..20] Inferior Olive sensory prediction error
    pub ltd_synaptic_weight: i32,    // [20..24] Parallel fiber -> Purkinje plastic weight
    pub forward_model_pred: i32,     // [24..28] Predicted sensorimotor outcome (Q16.16)
    pub lead_compensation_q16: i32,  // [28..32] Smith predictor lead time offset
    pub _reserved: [u8; 32],         // [32..64] Strict 64-byte cache-line alignment padding
}

impl CerebellarMicrozone {
    /// Placeholder forward model. The prediction and the error are computed from the same
    /// sample, so the error carries no information about the plant (whitepaper finding F-8);
    /// a real forward model compares the prediction made at `t` with the observation at
    /// `t + d`. Arithmetic is saturating (whitepaper §8.1).
    #[inline(always)]
    pub fn step_forward_model(&mut self, current_sensory: i32, motor_command: i32) -> i32 {
        self.mossy_fiber_input = motor_command;
        // Internal forward prediction: estimated outcome before physical body responds.
        // `>>` on i32 is an arithmetic shift, so a negative command scales toward zero.
        self.forward_model_pred = current_sensory.saturating_add(motor_command >> 2);
        // Error from inferior olive climbing fiber
        self.climbing_fiber_error = current_sensory.saturating_sub(self.forward_model_pred);
        // Purkinje cell LTD adaptation
        if self.climbing_fiber_error != 0 {
            self.ltd_synaptic_weight = self
                .ltd_synaptic_weight
                .saturating_sub(self.climbing_fiber_error >> 4);
        }
        self.purkinje_output_rate
    }
}

const _: () = {
    assert!(core::mem::size_of::<CerebellarMicrozone>() == 64);
    assert!(core::mem::align_of::<CerebellarMicrozone>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: i32 = 0x0001_0000;

    fn zone() -> CerebellarMicrozone {
        CerebellarMicrozone {
            microzone_id: 0,
            purkinje_output_rate: 0,
            mossy_fiber_input: 0,
            granule_expansion_code: 0,
            climbing_fiber_error: 0,
            ltd_synaptic_weight: 0,
            forward_model_pred: 0,
            lead_compensation_q16: 0,
            _reserved: [0; 32],
        }
    }

    #[test]
    fn prediction_adds_a_quarter_of_the_command() {
        let mut z = zone();
        z.step_forward_model(ONE, 4 * ONE);
        assert_eq!(z.mossy_fiber_input, 4 * ONE);
        assert_eq!(z.forward_model_pred, 2 * ONE);
    }

    #[test]
    fn error_is_the_negated_command_quarter_and_drives_ltd() {
        // Documents the F-8 placeholder: the error depends on the command alone.
        let mut z = zone();
        z.step_forward_model(ONE, 4 * ONE);
        assert_eq!(z.climbing_fiber_error, -ONE);
        assert_eq!(z.ltd_synaptic_weight, ONE >> 4);
    }

    #[test]
    fn negative_command_shift_is_arithmetic_and_prediction_saturates() {
        let mut z = zone();
        z.step_forward_model(i32::MIN, i32::MIN);
        assert_eq!(i32::MIN >> 2, -0x2000_0000);
        assert_eq!(z.forward_model_pred, i32::MIN);
        assert_eq!(z.climbing_fiber_error, 0);
        assert_eq!(z.ltd_synaptic_weight, 0);
    }

    #[test]
    fn ltd_weight_saturates_at_the_minimum() {
        let mut z = zone();
        z.ltd_synaptic_weight = i32::MIN;
        z.step_forward_model(0, i32::MIN);
        assert_eq!(z.climbing_fiber_error, 0x2000_0000);
        assert_eq!(z.ltd_synaptic_weight, i32::MIN);
    }
}
