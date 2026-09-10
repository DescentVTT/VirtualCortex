//! Foveal attention focus: where the sensory field is sampled at full resolution, and the
//! saccades that move it (whitepaper §5.2.22, §8.8; admitted by ADR-0016).
//!
//! One record per attention field. A saccade is a flight of a fixed number of ticks toward a
//! target during which the field is not sampled; on landing, fixation starts counting. The
//! saccade state machine is Implemented; the salience map it is driven by (`cortex-salience`)
//! and the foveal gating of `cortex-thalamus` relay gains are Specified.

#![no_std]

/// `attention_mode_flags` bit: the focus follows a moving target between saccades.
pub const MODE_SMOOTH_PURSUIT: u16 = 0x0001;

/// 64-byte attention focus (whitepaper §5.2.22).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct FovealAttentionFocus {
    pub gaze_target_x_q16: i32, // [0..4] Target in the sensory field (Q16.16)
    pub gaze_target_y_q16: i32, // [4..8] Target in the sensory field (Q16.16)
    pub fixation_duration_ticks: u32, // [8..12] Ticks since the last landing
    pub saccade_remaining_ticks: u32, // [12..16] Flight ticks left; 0 when fixating
    pub salience_peak_magnitude_q16: u32, // [16..20] Salience that selected the target (Q16.16)
    pub attention_mode_flags: u16, // [20..22] MODE_* bits
    pub saccade_in_flight: u8,  // [22] 1 while a saccade is in flight
    pub _reserved: [u8; 41],    // [23..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for FovealAttentionFocus {
    fn default() -> Self {
        Self {
            gaze_target_x_q16: 0,
            gaze_target_y_q16: 0,
            fixation_duration_ticks: 0,
            saccade_remaining_ticks: 0,
            salience_peak_magnitude_q16: 0,
            attention_mode_flags: 0,
            saccade_in_flight: 0,
            _reserved: [0; 41],
        }
    }
}

impl FovealAttentionFocus {
    /// Starts a saccade of `flight_ticks` toward `(x, y)` selected by a salience peak of
    /// `salience_q16`. Refused, with nothing changed, while a saccade is in flight or when
    /// `flight_ticks` is zero (a zero-length flight would land before it began).
    pub fn begin_saccade(
        &mut self,
        x_q16: i32,
        y_q16: i32,
        flight_ticks: u32,
        salience_q16: u32,
    ) -> bool {
        if self.saccade_in_flight != 0 || flight_ticks == 0 {
            return false;
        }
        self.gaze_target_x_q16 = x_q16;
        self.gaze_target_y_q16 = y_q16;
        self.salience_peak_magnitude_q16 = salience_q16;
        self.saccade_remaining_ticks = flight_ticks;
        self.saccade_in_flight = 1;
        true
    }

    /// Advances one tick. In flight, counts the flight down and returns `true` on the tick the
    /// saccade lands, resetting fixation; while fixating, counts fixation up (saturating) and
    /// returns `false`.
    pub fn tick(&mut self) -> bool {
        if self.saccade_in_flight != 0 {
            self.saccade_remaining_ticks = self.saccade_remaining_ticks.saturating_sub(1);
            if self.saccade_remaining_ticks == 0 {
                self.saccade_in_flight = 0;
                self.fixation_duration_ticks = 0;
                return true;
            }
            false
        } else {
            self.fixation_duration_ticks = self.fixation_duration_ticks.saturating_add(1);
            false
        }
    }

    /// True while the field is not being sampled.
    #[inline]
    pub const fn is_in_flight(&self) -> bool {
        self.saccade_in_flight != 0
    }
}

const _: () = {
    assert!(core::mem::size_of::<FovealAttentionFocus>() == 64);
    assert!(core::mem::align_of::<FovealAttentionFocus>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_is_one_cache_line_and_default_is_fixating() {
        assert_eq!(core::mem::size_of::<FovealAttentionFocus>(), 64);
        assert_eq!(core::mem::align_of::<FovealAttentionFocus>(), 64);
        assert!(!FovealAttentionFocus::default().is_in_flight());
    }

    #[test]
    fn a_saccade_flies_for_exactly_its_ticks_then_lands() {
        let mut f = FovealAttentionFocus::default();
        assert!(f.begin_saccade(0x0001_0000, -0x0002_0000, 3, 0x0000_8000));
        assert!(f.is_in_flight());
        assert!(!f.tick());
        assert!(!f.tick());
        assert!(f.tick(), "lands on the third tick");
        assert!(!f.is_in_flight());
        assert_eq!(f.fixation_duration_ticks, 0);
        assert_eq!(
            (f.gaze_target_x_q16, f.gaze_target_y_q16),
            (0x0001_0000, -0x0002_0000)
        );
    }

    #[test]
    fn a_second_saccade_is_refused_while_one_is_in_flight() {
        let mut f = FovealAttentionFocus::default();
        assert!(f.begin_saccade(1, 1, 2, 1));
        let before = f;
        assert!(!f.begin_saccade(9, 9, 5, 9));
        assert_eq!(f, before);
    }

    #[test]
    fn a_zero_length_flight_is_refused() {
        let mut f = FovealAttentionFocus::default();
        assert!(!f.begin_saccade(1, 1, 0, 1));
        assert!(!f.is_in_flight());
    }

    #[test]
    fn fixation_counts_while_not_in_flight_and_saturates() {
        let mut f = FovealAttentionFocus {
            fixation_duration_ticks: u32::MAX - 1,
            ..Default::default()
        };
        assert!(!f.tick());
        assert_eq!(f.fixation_duration_ticks, u32::MAX);
        assert!(!f.tick());
        assert_eq!(
            f.fixation_duration_ticks,
            u32::MAX,
            "saturates rather than wraps"
        );
    }
}
