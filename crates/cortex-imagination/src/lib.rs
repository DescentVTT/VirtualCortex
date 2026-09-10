//! Mental canvas frames: one step of an offline counterfactual rollout that never drives the
//! motor channel (whitepaper §5.2.32, §8.8; admitted by ADR-0016).
//!
//! `cortex-executive` holds goal-directed plan trees; this crate holds the frames of a
//! generative rehearsal that has no goal, only a hypothetical action and where it leads. A
//! frame is sandboxed by construction: `motor_release_flag` MUST be zero, and a frame whose
//! flag is set is invalid and refuses to step. The step and the divergence test are
//! Implemented; the generative model that supplies the deltas is Specified.

#![no_std]

/// 64-byte canvas frame (whitepaper §5.2.32).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct MentalCanvasFrame {
    pub simulation_id: u64,            // [0..8] The rollout this frame belongs to
    pub hypothetical_action_hash: u32, // [8..12] The action being imagined
    pub predicted_outcome_valence_q16: i32, // [12..16] Accumulated predicted valence (Q16.16)
    pub divergence_uncertainty_q16: u32, // [16..20] Accumulated uncertainty of the rollout (Q16.16)
    pub canvas_epoch_ticks: u32,       // [20..24] Imagined time elapsed, in ticks
    pub rollout_depth: u16,            // [24..26] Steps taken
    pub motor_release_flag: u8,        // [26] MUST be zero: a canvas frame never reaches the egress
    pub _reserved: [u8; 37],           // [27..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for MentalCanvasFrame {
    fn default() -> Self {
        Self {
            simulation_id: 0,
            hypothetical_action_hash: 0,
            predicted_outcome_valence_q16: 0,
            divergence_uncertainty_q16: 0,
            canvas_epoch_ticks: 0,
            rollout_depth: 0,
            motor_release_flag: 0,
            _reserved: [0; 37],
        }
    }
}

impl MentalCanvasFrame {
    /// True when the frame can never reach the motor channel. Every valid frame is.
    #[inline]
    pub const fn is_sandboxed(&self) -> bool {
        self.motor_release_flag == 0
    }

    /// One imagined step: valence accumulates (saturating), uncertainty grows (saturating),
    /// imagined time advances by `dt_ticks` (saturating) and the depth grows (saturating).
    /// Refused, with nothing changed, for a frame that is not sandboxed.
    pub fn step(
        &mut self,
        delta_valence_q16: i32,
        uncertainty_growth_q16: u32,
        dt_ticks: u32,
    ) -> bool {
        if !self.is_sandboxed() {
            return false;
        }
        self.predicted_outcome_valence_q16 = self
            .predicted_outcome_valence_q16
            .saturating_add(delta_valence_q16);
        self.divergence_uncertainty_q16 = self
            .divergence_uncertainty_q16
            .saturating_add(uncertainty_growth_q16);
        self.canvas_epoch_ticks = self.canvas_epoch_ticks.saturating_add(dt_ticks);
        self.rollout_depth = self.rollout_depth.saturating_add(1);
        true
    }

    /// True once the rollout's uncertainty has reached `limit_q16`: its predictions are no
    /// longer worth reading.
    #[inline]
    pub const fn has_diverged(&self, limit_q16: u32) -> bool {
        self.divergence_uncertainty_q16 >= limit_q16
    }
}

const _: () = {
    assert!(core::mem::size_of::<MentalCanvasFrame>() == 64);
    assert!(core::mem::align_of::<MentalCanvasFrame>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: u32 = 0x0001_0000;

    #[test]
    fn record_is_one_cache_line_and_default_is_a_sandboxed_empty_rollout() {
        assert_eq!(core::mem::size_of::<MentalCanvasFrame>(), 64);
        assert_eq!(core::mem::align_of::<MentalCanvasFrame>(), 64);
        let d = MentalCanvasFrame::default();
        assert!(d.is_sandboxed());
        assert_eq!(d.rollout_depth, 0);
        assert!(!d.has_diverged(1));
        assert!(d.has_diverged(0), "a zero limit is reached immediately");
    }

    #[test]
    fn steps_accumulate_valence_uncertainty_time_and_depth() {
        let mut f = MentalCanvasFrame {
            simulation_id: 3,
            ..Default::default()
        };
        assert!(f.step(ONE as i32, ONE / 4, 100));
        assert!(f.step(-(ONE as i32) / 2, ONE / 4, 100));
        assert_eq!(f.predicted_outcome_valence_q16, (ONE / 2) as i32);
        assert_eq!(f.divergence_uncertainty_q16, ONE / 2);
        assert_eq!((f.canvas_epoch_ticks, f.rollout_depth), (200, 2));
        assert!(f.has_diverged(ONE / 2));
        assert!(!f.has_diverged(ONE / 2 + 1));
    }

    #[test]
    fn every_accumulator_saturates() {
        let mut f = MentalCanvasFrame {
            predicted_outcome_valence_q16: i32::MAX,
            divergence_uncertainty_q16: u32::MAX,
            canvas_epoch_ticks: u32::MAX,
            rollout_depth: u16::MAX,
            ..Default::default()
        };
        assert!(f.step(1, 1, 1));
        assert_eq!(f.predicted_outcome_valence_q16, i32::MAX);
        assert_eq!(f.divergence_uncertainty_q16, u32::MAX);
        assert_eq!(
            (f.canvas_epoch_ticks, f.rollout_depth),
            (u32::MAX, u16::MAX)
        );
        let mut g = MentalCanvasFrame {
            predicted_outcome_valence_q16: i32::MIN,
            ..Default::default()
        };
        g.step(-1, 0, 0);
        assert_eq!(g.predicted_outcome_valence_q16, i32::MIN);
    }

    #[test]
    fn a_frame_that_could_reach_the_motor_channel_refuses_to_step() {
        let mut f = MentalCanvasFrame {
            motor_release_flag: 1,
            ..Default::default()
        };
        let before = f;
        assert!(!f.is_sandboxed());
        assert!(!f.step(1, 1, 1));
        assert_eq!(f, before);
    }
}
