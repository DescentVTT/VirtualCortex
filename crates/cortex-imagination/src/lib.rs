//! Mental canvas frames: one step of an offline counterfactual rollout that never drives the
//! motor channel (whitepaper §5.2.32, §8.8; admitted by ADR-0016), and, since ADR-0020 and
//! ADR-0021, the rollout's self-model with its fixed point and the deterministic wandering of
//! the default mode (§8.12, §8.13).
//!
//! `cortex-executive` holds goal-directed plan trees; this crate holds the frames of a
//! generative rehearsal that has no goal, only a hypothetical action and where it leads. A
//! frame is sandboxed by construction: `motor_release_flag` MUST be zero, and a frame whose
//! flag is set is invalid and refuses to step. The step, the divergence test, the reflection
//! and the wander are Implemented; the generative model that supplies the deltas is Specified.

#![no_std]

/// 64-byte canvas frame (whitepaper §5.2.32).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct MentalCanvasFrame {
    pub simulation_id: u64,            // [0..8] The rollout this frame belongs to
    pub hypothetical_action_hash: u32, // [8..12] The action being imagined
    pub predicted_outcome_valence_q16: i32, // [12..16] Accumulated predicted valence (Q16.16)
    pub divergence_uncertainty_q16: u32, // [16..20] Accumulated uncertainty of the rollout (Q16.16)
    pub canvas_epoch_ticks: u32,       // [20..24] Imagined time elapsed, in ticks
    pub rollout_depth: u16,            // [24..26] Steps taken
    pub motor_release_flag: u8,        // [26] MUST be zero: a canvas frame never reaches the egress
    pub reflection_converged: u8, // [27] 1 when the last reflect() found the self-model's fixed point (ADR-0020)
    pub strange_loop_fixed_point_hash: u32, // [28..32] The self the rollout last observed itself to be (ADR-0020)
    pub dmn_wander_temperature_q16: u32, // [32..36] Amplitude of the default mode's wandering (Q16.16, ADR-0021)
    pub wander_state: u32, // [36..40] State of the deterministic generator behind wander() (ADR-0021)
    pub _reserved: [u8; 24], // [40..64] Reserved; MUST be zero
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

    /// The strange loop (ADR-0020, whitepaper §8.12): the rollout observes the self that is
    /// doing the imagining, as a hash of its state, and stores it; the self-model is at its
    /// fixed point when the self observed now is the self observed at the previous reflection,
    /// so the model of the self contains a model that agrees with itself. Returns whether the
    /// fixed point was reached, and records it. Refused for a frame that is not sandboxed.
    pub fn reflect(&mut self, observed_self_hash: u32) -> bool {
        if !self.is_sandboxed() {
            return false;
        }
        let converged = self.strange_loop_fixed_point_hash == observed_self_hash;
        self.strange_loop_fixed_point_hash = observed_self_hash;
        self.reflection_converged = converged as u8;
        converged
    }

    /// One wandering step of the default mode (ADR-0021, whitepaper §8.13): a deterministic
    /// generator seeded from the rollout advances, the hypothetical action becomes a mix of the
    /// previous one and the draw, and a valence perturbation of magnitude up to the wandering
    /// temperature is taken as a step of one tick with the temperature as its uncertainty
    /// growth. A zero temperature wanders nowhere: the action still changes, the valence does
    /// not. Returns the perturbation. Refused, with nothing changed, for a frame that is not
    /// sandboxed.
    pub fn wander(&mut self) -> Option<i32> {
        if !self.is_sandboxed() {
            return None;
        }
        if self.wander_state == 0 {
            self.wander_state =
                (self.simulation_id as u32) ^ ((self.simulation_id >> 32) as u32) | 1;
        }
        self.wander_state = self
            .wander_state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let draw = self.wander_state;
        self.hypothetical_action_hash =
            (self.hypothetical_action_hash ^ draw).wrapping_mul(0x0100_0193);
        // A signed fraction in [-1, 1) from the high bits, scaled by the temperature.
        let fraction = (draw >> 16) as i32 - 0x8000;
        let perturbation = ((fraction as i64 * self.dmn_wander_temperature_q16 as i64) >> 15)
            .clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        self.step(perturbation, self.dmn_wander_temperature_q16, 1);
        Some(perturbation)
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
        assert_eq!(d.reflection_converged, 0);
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
    fn a_frame_that_could_reach_the_motor_channel_refuses_to_step_reflect_or_wander() {
        let mut f = MentalCanvasFrame {
            motor_release_flag: 1,
            ..Default::default()
        };
        let before = f;
        assert!(!f.is_sandboxed());
        assert!(!f.step(1, 1, 1));
        assert!(!f.reflect(7));
        assert_eq!(f.wander(), None);
        assert_eq!(f, before);
    }

    #[test]
    fn the_self_model_reaches_its_fixed_point_when_the_observed_self_stops_changing() {
        let mut f = MentalCanvasFrame::default();
        assert!(!f.reflect(0xA1), "the first observation is new");
        assert_eq!(f.reflection_converged, 0);
        assert!(!f.reflect(0xA2), "a changed self is not a fixed point");
        assert!(f.reflect(0xA2), "the same self twice is");
        assert_eq!(f.reflection_converged, 1);
        assert_eq!(f.strange_loop_fixed_point_hash, 0xA2);
        assert!(
            !f.reflect(0xA3),
            "and it is lost when the self changes again"
        );
        assert_eq!(f.reflection_converged, 0);
    }

    #[test]
    fn wandering_is_deterministic_and_a_zero_temperature_moves_the_action_but_not_the_valence() {
        let mut a = MentalCanvasFrame {
            simulation_id: 42,
            dmn_wander_temperature_q16: ONE / 4,
            ..Default::default()
        };
        let mut b = a;
        for _ in 0..100 {
            assert_eq!(a.wander(), b.wander());
            assert_eq!(a, b);
        }
        assert_eq!(a.rollout_depth, 100);
        assert_eq!(a.canvas_epoch_ticks, 100, "each wander is one tick");
        assert!(
            a.divergence_uncertainty_q16 >= 100 * (ONE / 4),
            "uncertainty grows by the temperature"
        );
        let mut cold = MentalCanvasFrame {
            simulation_id: 42,
            ..Default::default()
        };
        let action = cold.hypothetical_action_hash;
        assert_eq!(cold.wander(), Some(0));
        assert_ne!(
            cold.hypothetical_action_hash, action,
            "the action still drifts"
        );
        assert_eq!(
            cold.predicted_outcome_valence_q16, 0,
            "the valence does not"
        );
        let mut hot = MentalCanvasFrame {
            simulation_id: 42,
            dmn_wander_temperature_q16: ONE,
            ..Default::default()
        };
        let mut seen_negative = false;
        let mut seen_positive = false;
        for _ in 0..64 {
            let p = hot.wander().unwrap();
            assert!(
                p.unsigned_abs() <= ONE,
                "a perturbation never exceeds the temperature"
            );
            seen_negative |= p < 0;
            seen_positive |= p > 0;
        }
        assert!(seen_negative && seen_positive, "wandering goes both ways");
    }
}
