//! Neuromodulatory Dynamics & Three-Factor Plasticity

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_is_sixteen_bytes_sixteen_aligned() {
        assert_eq!(core::mem::size_of::<NeuromodulatorState>(), 16);
        assert_eq!(core::mem::align_of::<NeuromodulatorState>(), 16);
    }

    #[test]
    fn dopamine_is_the_only_signed_field() {
        let m = NeuromodulatorState {
            dopamine_rpe: -0x0001_0000,
            norepinephrine: 0,
            serotonin: 0,
            acetylcholine: 0,
        };
        assert!(
            m.dopamine_rpe < 0,
            "a negative reward-prediction error is representable"
        );
        let copy = m;
        assert_eq!(copy, m, "the record is Copy and Eq");
    }
}
