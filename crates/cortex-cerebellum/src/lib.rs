//! Cerebellum: a per-microzone forward model with an in-record delay line (whitepaper §5.2.6).

#![no_std]

/// Per-microzone forward model with an in-record delay line.
///
/// At each step the model predicts the observation `d` steps ahead from the current
/// observation and the current motor command through a learned scalar gain, remembers that
/// prediction, and compares the observation arriving now with the prediction it made `d`
/// steps ago (whitepaper §5.2.6, §8.8). The climbing-fibre error drives the gain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct CerebellarMicrozone {
    pub microzone_id: u32,           // [0..4] Anatomical microzone identifier
    pub purkinje_output_rate: i32, // [4..8] Predicted sensory change for the current command (Q16.16)
    pub mossy_fiber_input: i32,    // [8..12] Sensorimotor state input (Q16.16)
    pub granule_expansion_code: u32, // [12..16] High-dimensional sparse pattern hash (Specified)
    pub climbing_fiber_error: i32, // [16..20] Observation now minus the prediction made d steps ago (Q16.16)
    pub ltd_synaptic_weight: i32, // [20..24] Learned forward gain, parallel fibre -> Purkinje (Q16.16)
    pub forward_model_pred: i32,  // [24..28] Predicted observation d steps ahead (Q16.16)
    pub lead_compensation_q16: i32, // [28..32] Smith predictor lead time offset (Specified)
    pub pred_ring: [i32; 7], // [32..60] Delay line: the predictions made at the last seven steps (Q16.16)
    pub delay_ctl: u32,      // [60..64] Packed: head | plant delay d | filled | command-sign bitmap
}

impl CerebellarMicrozone {
    /// Longest plant delay the in-record delay line can hold, in steps.
    pub const MAX_PLANT_DELAY: u8 = 7;

    /// Learning rate as a right shift: eta = 1/16. The adjustment is rounded to nearest
    /// (`HALF_STEP` added before the shift) rather than floored, so the steady-state residual
    /// is bounded by half a learning step (8 LSB) instead of a whole one.
    const LEARNING_SHIFT: u32 = 4;
    const HALF_STEP: i32 = 1 << (Self::LEARNING_SHIFT - 1);

    const RING: u32 = 7;
    const DELAY_SHIFT: u32 = 8;
    const FILLED_SHIFT: u32 = 16;
    const SIGN_SHIFT: u32 = 24;
    const BYTE: u32 = 0xFF;

    /// Sets the plant delay `d` in steps, clamped to `0..=MAX_PLANT_DELAY`, and clears the
    /// delay line. A delay of 0 disables comparison and learning.
    #[inline]
    pub fn set_plant_delay(&mut self, d: u8) {
        let d = if d > Self::MAX_PLANT_DELAY {
            Self::MAX_PLANT_DELAY
        } else {
            d
        };
        self.delay_ctl = (d as u32) << Self::DELAY_SHIFT;
        self.pred_ring = [0; 7];
        self.climbing_fiber_error = 0;
    }

    /// The plant delay `d` in steps (0 when unset). Every field of the packed word is read
    /// within its bound, so a word set through the public field or read from an image cannot
    /// index outside the ring (ADR-0028).
    #[inline]
    pub const fn plant_delay(&self) -> u8 {
        let d = ((self.delay_ctl >> Self::DELAY_SHIFT) & Self::BYTE) as u8;
        if d > Self::MAX_PLANT_DELAY {
            Self::MAX_PLANT_DELAY
        } else {
            d
        }
    }

    /// Number of valid entries in the delay line, saturating at seven.
    #[inline]
    pub const fn filled(&self) -> u8 {
        let filled = (self.delay_ctl >> Self::FILLED_SHIFT) & Self::BYTE;
        if filled > Self::RING {
            Self::RING as u8
        } else {
            filled as u8
        }
    }

    #[inline]
    const fn head(&self) -> u32 {
        // The head lives in bits 0-7: no shift.
        (self.delay_ctl & Self::BYTE) % Self::RING
    }

    #[inline]
    const fn sign_negative(&self, slot: u32) -> bool {
        self.delay_ctl & (1 << (Self::SIGN_SHIFT + slot)) != 0
    }

    /// One step of the forward model. `current_sensory` is the observation arriving now;
    /// `motor_command` is the command issued now. Returns the predicted sensory change for that
    /// command (the Purkinje output), which is the compensation signal a Smith predictor uses.
    ///
    /// Update rule (whitepaper §8.8, discretised): with gain `w`, `delta = w * u`,
    /// `prediction = y + delta`; if the line holds a prediction from `d` steps ago,
    /// `error = y - prediction_old` and `w += sign(u_old) * round(error / 16)`, which converges
    /// on a plant `y(t + d) = y(t) + k * u(t)` to `w = k` within half a learning step. Every
    /// operation saturates (§8.1).
    #[inline(always)]
    pub fn step_forward_model(&mut self, current_sensory: i32, motor_command: i32) -> i32 {
        self.mossy_fiber_input = motor_command;

        // Predicted change and predicted observation d steps ahead.
        let delta = mul_q16(self.ltd_synaptic_weight, motor_command);
        self.purkinje_output_rate = delta;
        let prediction = current_sensory.saturating_add(delta);
        self.forward_model_pred = prediction;

        // Compare with the prediction made d steps ago and adapt the gain.
        let d = self.plant_delay() as u32;
        let head = self.head();
        let filled = self.filled() as u32;
        if d > 0 && filled >= d {
            let slot = (head + Self::RING - d) % Self::RING;
            let old_prediction = self.pred_ring[slot as usize];
            let error = current_sensory.saturating_sub(old_prediction);
            self.climbing_fiber_error = error;
            let adjustment = error.saturating_add(Self::HALF_STEP) >> Self::LEARNING_SHIFT;
            self.ltd_synaptic_weight = if self.sign_negative(slot) {
                self.ltd_synaptic_weight.saturating_sub(adjustment)
            } else {
                self.ltd_synaptic_weight.saturating_add(adjustment)
            };
        } else {
            self.climbing_fiber_error = 0;
        }

        // Push the new prediction into the delay line.
        self.pred_ring[head as usize] = prediction;
        let sign_bit = 1 << (Self::SIGN_SHIFT + head);
        // The slot's old sign is cleared first, then set for a negative command: the fields
        // of the word never overlap, so the assembly below is a disjoint union.
        let others = self.delay_ctl & (0x7F << Self::SIGN_SHIFT) & !sign_bit;
        let signs = if motor_command < 0 {
            others | sign_bit
        } else {
            others
        };
        let new_head = (head + 1) % Self::RING;
        let new_filled = if filled < Self::RING {
            filled + 1
        } else {
            filled
        };
        self.delay_ctl =
            new_head | (d << Self::DELAY_SHIFT) | (new_filled << Self::FILLED_SHIFT) | signs;

        self.purkinje_output_rate
    }
}

/// Q16.16 × Q16.16 → Q16.16, widened to `i64`, shifted once, clamped to `i32` (whitepaper §8.1).
#[inline(always)]
const fn mul_q16(a: i32, b: i32) -> i32 {
    let p = (a as i64 * b as i64) >> 16;
    if p > i32::MAX as i64 {
        i32::MAX
    } else if p < i32::MIN as i64 {
        i32::MIN
    } else {
        p as i32
    }
}

const _: () = {
    assert!(core::mem::size_of::<CerebellarMicrozone>() == 64);
    assert!(core::mem::align_of::<CerebellarMicrozone>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sign_of_each_command_is_kept_in_its_slot_of_the_control_word() {
        let mut z = zone(1);
        z.step_forward_model(0, -HALF);
        assert_ne!(
            z.delay_ctl & (1 << 24),
            0,
            "slot 0 remembers a negative command"
        );
        z.step_forward_model(0, HALF);
        assert_eq!(z.delay_ctl & (1 << 25), 0, "slot 1 a positive one");
        assert_ne!(z.delay_ctl & (1 << 24), 0, "and slot 0 is untouched");
        for _ in 0..7 {
            z.step_forward_model(0, HALF);
        }
        assert_eq!(
            z.delay_ctl & (1 << 24),
            0,
            "the ring wrapped: a positive rewrite clears it"
        );
        assert_eq!(z.delay_ctl >> 31, 0);
        let mut again = zone(1);
        for _ in 0..8 {
            again.step_forward_model(0, -HALF);
        }
        assert_ne!(
            again.delay_ctl & (1 << 24),
            0,
            "a negative command on a slot already negative keeps the bit"
        );
        assert_eq!(
            again.delay_ctl & (0x7F << 24),
            0x7F << 24,
            "every slot negative"
        );
        let mut zero = zone(1);
        zero.step_forward_model(0, 0);
        assert_eq!(
            zero.delay_ctl & (1 << 24),
            0,
            "a zero command is not negative (the mutation gate\'s first catch)"
        );
    }

    const ONE: i32 = 0x0001_0000;
    const HALF: i32 = 0x0000_8000;

    fn zone(d: u8) -> CerebellarMicrozone {
        let mut z = CerebellarMicrozone {
            microzone_id: 0,
            purkinje_output_rate: 0,
            mossy_fiber_input: 0,
            granule_expansion_code: 0,
            climbing_fiber_error: 0,
            ltd_synaptic_weight: 0,
            forward_model_pred: 0,
            lead_compensation_q16: 0,
            pred_ring: [0; 7],
            delay_ctl: 0,
        };
        z.set_plant_delay(d);
        z
    }

    /// Runs the model against the plant `y(t + d) = y(t) + k * u(t)` for `n` steps.
    /// Returns the final climbing-fibre error and the learned gain.
    fn run(k: i32, d: u8, n: usize, alternating: bool) -> (i32, i32) {
        let mut z = zone(d);
        let du = d as usize;
        let mut y = [0i32; 512];
        for t in 0..n {
            let u = if alternating && t % 2 == 1 { -ONE } else { ONE };
            z.step_forward_model(y[t], u);
            y[t + du] = y[t].saturating_add(mul_q16(k, u));
        }
        (z.climbing_fiber_error, z.ltd_synaptic_weight)
    }

    #[test]
    fn purkinje_output_is_the_predicted_change() {
        let mut z = zone(2);
        z.ltd_synaptic_weight = HALF;
        let out = z.step_forward_model(3 * ONE, 2 * ONE);
        assert_eq!(out, ONE);
        assert_eq!(z.purkinje_output_rate, ONE);
        assert_eq!(z.forward_model_pred, 4 * ONE);
        assert_eq!(z.mossy_fiber_input, 2 * ONE);
    }

    #[test]
    fn first_d_steps_produce_no_error() {
        let d = 3u8;
        let mut z = zone(d);
        let mut y = [0i32; 16];
        for t in 0..3usize {
            z.step_forward_model(y[t], ONE);
            y[t + 3] = y[t] + mul_q16(HALF, ONE);
            assert_eq!(z.climbing_fiber_error, 0, "step {t}");
            assert_eq!(z.ltd_synaptic_weight, 0);
        }
        // Step 3 compares y[3] = 0.5 with the prediction made at step 0 (gain 0 → 0).
        z.step_forward_model(y[3], ONE);
        assert_eq!(z.climbing_fiber_error, HALF);
        assert_eq!(z.ltd_synaptic_weight, HALF >> 4);
    }

    #[test]
    fn converges_on_a_linear_plant() {
        // The residual is bounded by half a learning step (8 LSB) because the adjustment is
        // rounded to nearest; with a floored adjustment it would be a whole step (16 LSB).
        for &(k, d) in &[(HALF, 2u8), (ONE / 4, 7), (3 * ONE / 4, 1)] {
            let (error, gain) = run(k, d, 400, false);
            assert!(error.abs() <= 8, "k={k:#x} d={d}: error {error}");
            assert!((gain - k).abs() <= 8, "k={k:#x} d={d}: gain {gain:#x}");
        }
    }

    #[test]
    fn converges_with_alternating_command_sign() {
        let (error, gain) = run(HALF, 3, 400, true);
        assert!(error.abs() <= 8, "error {error}");
        assert!((gain - HALF).abs() <= 8, "gain {gain:#x}");
    }

    #[test]
    fn plant_delay_is_clamped_and_zero_disables_learning() {
        let mut z = zone(9);
        assert_eq!(z.plant_delay(), 7);
        let (error, gain) = run(HALF, 0, 50, false);
        assert_eq!(error, 0);
        assert_eq!(gain, 0);
        z.set_plant_delay(0);
        assert_eq!(z.plant_delay(), 0);
    }

    #[test]
    fn a_control_word_outside_its_bounds_is_read_within_them() {
        let mut z = zone(1);
        z.delay_ctl = 7 | (1 << 8) | (7 << 16);
        assert_eq!(z.plant_delay(), 1);
        z.step_forward_model(0, 0);
        assert_eq!(
            z.delay_ctl & 0xFF,
            1,
            "a head of seven is slot zero, then one"
        );
        let mut z = zone(1);
        z.delay_ctl = (8 << 8) | (8 << 16);
        assert_eq!((z.plant_delay(), z.filled()), (7, 7));
        z.step_forward_model(HALF, ONE);
        assert_eq!(z.plant_delay(), 7, "the write-back keeps the clamped delay");
        let mut z = zone(0);
        z.delay_ctl = u32::MAX;
        z.step_forward_model(i32::MAX, i32::MIN);
        assert_eq!((z.plant_delay(), z.filled()), (7, 7));
    }

    #[test]
    fn delay_line_fills_to_seven_and_wraps() {
        let mut z = zone(7);
        for t in 0..20 {
            z.step_forward_model(t, ONE);
            let expected = if t < 7 { t as u8 + 1 } else { 7 };
            assert_eq!(z.filled(), expected, "step {t}");
        }
        assert_eq!(z.head(), 20 % 7);
    }

    #[test]
    fn saturates_at_the_extremes_without_panicking() {
        let mut z = zone(1);
        z.ltd_synaptic_weight = i32::MAX;
        // MAX × MAX clamps the predicted change; MAX + MAX saturates the prediction.
        let out = z.step_forward_model(i32::MAX, i32::MAX);
        assert_eq!(out, i32::MAX);
        assert_eq!(z.forward_model_pred, i32::MAX);
        // MIN − MAX saturates the error; round(MIN / 16) = −2^27 is applied to the gain
        // (the remembered command was positive), which stays in range.
        z.step_forward_model(i32::MIN, i32::MIN);
        assert_eq!(z.climbing_fiber_error, i32::MIN);
        assert_eq!(z.ltd_synaptic_weight, i32::MAX - 0x0800_0000);
        // The next prediction from a large gain and a maximal command saturates again.
        z.step_forward_model(i32::MAX, i32::MAX);
        assert_eq!(z.forward_model_pred, i32::MAX);
    }
}

/// Property tests (ADR-0030): the Q16.16 product saturates exactly where a wider reference
/// says, and a step never panics or leaves the ring's bounds for any observation, command or
/// control word.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn the_product_saturates_exactly_where_the_reference_does() {
        let reference = |a: i32, b: i32| {
            ((a as i64 * b as i64) >> 16).clamp(i32::MIN as i64, i32::MAX as i64) as i32
        };
        for &a in I32_LATTICE.iter() {
            for &b in I32_LATTICE.iter() {
                assert_eq!(mul_q16(a, b), reference(a, b), "{a} * {b}");
            }
        }
        let mut rng = Lcg::new(31);
        for _ in 0..200_000 {
            let (a, b) = (rng.i32_edge_biased(), rng.i32_edge_biased());
            assert_eq!(mul_q16(a, b), reference(a, b), "{a} * {b}");
        }
    }

    #[test]
    fn a_step_keeps_the_ring_in_bounds_for_every_input_and_control_word() {
        let mut rng = Lcg::new(37);
        for _ in 0..50_000 {
            let mut z = CerebellarMicrozone {
                microzone_id: 0,
                purkinje_output_rate: rng.i32_edge_biased(),
                mossy_fiber_input: 0,
                granule_expansion_code: 0,
                climbing_fiber_error: 0,
                ltd_synaptic_weight: rng.i32_edge_biased(),
                forward_model_pred: 0,
                lead_compensation_q16: 0,
                pred_ring: core::array::from_fn(|_| rng.i32_edge_biased()),
                delay_ctl: if rng.below(2) == 0 {
                    rng.next_u32()
                } else {
                    (rng.below(8) << 8) | rng.below(8)
                },
            };
            for _ in 0..16 {
                z.step_forward_model(rng.i32_edge_biased(), rng.i32_edge_biased());
                // The raw fields of the written-back word, not the clamping accessors.
                assert!(
                    (z.delay_ctl >> 8) & 0xFF <= 7,
                    "the stored delay is clamped"
                );
                assert!(
                    (z.delay_ctl >> 16) & 0xFF <= 7,
                    "the stored fill count is clamped"
                );
                assert!((z.delay_ctl & 0xFF) < 7, "the head stays in the ring");
                assert_eq!(z.delay_ctl >> 31, 0, "bit 31 stays zero");
            }
        }
    }
}
