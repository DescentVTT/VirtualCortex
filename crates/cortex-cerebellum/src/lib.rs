//! Cerebellar Internal Forward Models & Microsecond Motor Coordination

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
    #[inline(always)]
    pub fn step_forward_model(&mut self, current_sensory: i32, motor_command: i32) -> i32 {
        self.mossy_fiber_input = motor_command;
        // Internal forward prediction: estimated outcome before physical body responds
        self.forward_model_pred = current_sensory + (motor_command >> 2);
        // Error from inferior olive climbing fiber
        self.climbing_fiber_error = current_sensory - self.forward_model_pred;
        // Purkinje cell LTD adaptation
        if self.climbing_fiber_error != 0 {
            self.ltd_synaptic_weight -= self.climbing_fiber_error >> 4;
        }
        self.purkinje_output_rate
    }
}

const _: () = {
    assert!(core::mem::size_of::<CerebellarMicrozone>() == 64);
    assert!(core::mem::align_of::<CerebellarMicrozone>() == 64);
};
