//! Neuromodulatory Dynamics & Three-Factor Plasticity

#[repr(C, align(16))]
pub struct NeuromodulatorState {
    pub dopamine_rpe: i32,   // DA (TD-RPE, Q16.16)
    pub norepinephrine: u32, // NE (Arousal/Surprise, Q16.16)
    pub serotonin: u32,      // 5-HT (Discount factor, Q16.16)
    pub acetylcholine: u32,  // ACh (Sensory precision, Q16.16)
}

const _: () = {
    assert!(core::mem::size_of::<NeuromodulatorState>() == 16);
    assert!(core::mem::align_of::<NeuromodulatorState>() == 16);
};
