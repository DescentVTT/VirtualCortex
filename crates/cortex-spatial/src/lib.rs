//! Metric cognitive map: a grid coordinate maintained by dead-reckoning path integration and
//! reset by landmark fixes (whitepaper §5.2.25, §8.8; admitted by ADR-0016).
//!
//! `cortex-hippocampus` holds the episodic attractor and the place field the body is in; this
//! crate holds where the body is on a metric grid and which way it faces. One record per
//! navigating body. Path integration and the fix are Implemented; the grid-cell attractor that
//! would correct drift between fixes is Specified.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// Headings are fractions of a full turn in the low 16 bits and wrap (the phase-counter
/// convention of whitepaper §8.1).
pub const TURN_MASK: u32 = 0xFFFF;
/// Confidence loses 2^-CONFIDENCE_DECAY_SHIFT of itself per integration step, and at least one
/// LSB, so that it reaches zero instead of stalling at 255.
pub const CONFIDENCE_DECAY_SHIFT: u32 = 8;

/// 64-byte map coordinate (whitepaper §5.2.25).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SpatialGridCoordinate {
    pub grid_x_q16: i32,                      // [0..4] Position on the map (Q16.16)
    pub grid_y_q16: i32,                      // [4..8] Position on the map (Q16.16)
    pub grid_z_q16: i32,                      // [8..12] Position on the map (Q16.16)
    pub heading_yaw_turns: u32, // [12..16] Yaw as a fraction of a turn, 16 bits, wraps
    pub heading_pitch_q16: i32, // [16..20] Pitch (Q16.16)
    pub path_integration_confidence_q16: u32, // [20..24] 1.0 at a fix, decaying with each step (Q16.16)
    pub steps_since_fix: u32,                 // [24..28] Integration steps since the last fix
    pub _reserved: [u8; 36],                  // [28..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for SpatialGridCoordinate {
    fn default() -> Self {
        Self {
            grid_x_q16: 0,
            grid_y_q16: 0,
            grid_z_q16: 0,
            heading_yaw_turns: 0,
            heading_pitch_q16: 0,
            path_integration_confidence_q16: 0,
            steps_since_fix: 0,
            _reserved: [0; 36],
        }
    }
}

impl SpatialGridCoordinate {
    /// One dead-reckoning step: position moves by the deltas (saturating), yaw by `dyaw_turns`
    /// (wrapping within the turn), confidence decays, and the step counter grows (saturating).
    pub fn integrate(&mut self, dx_q16: i32, dy_q16: i32, dz_q16: i32, dyaw_turns: u32) {
        self.grid_x_q16 = self.grid_x_q16.saturating_add(dx_q16);
        self.grid_y_q16 = self.grid_y_q16.saturating_add(dy_q16);
        self.grid_z_q16 = self.grid_z_q16.saturating_add(dz_q16);
        self.heading_yaw_turns = self.heading_yaw_turns.wrapping_add(dyaw_turns) & TURN_MASK;
        let decay = (self.path_integration_confidence_q16 >> CONFIDENCE_DECAY_SHIFT).max(1);
        self.path_integration_confidence_q16 =
            self.path_integration_confidence_q16.saturating_sub(decay);
        self.steps_since_fix = self.steps_since_fix.saturating_add(1);
    }

    /// A landmark fix: the position and yaw are known; confidence returns to 1.0.
    pub fn fix(&mut self, x_q16: i32, y_q16: i32, z_q16: i32, yaw_turns: u32) {
        self.grid_x_q16 = x_q16;
        self.grid_y_q16 = y_q16;
        self.grid_z_q16 = z_q16;
        self.heading_yaw_turns = yaw_turns & TURN_MASK;
        self.path_integration_confidence_q16 = Q16_ONE;
        self.steps_since_fix = 0;
    }
}

const _: () = {
    assert!(core::mem::size_of::<SpatialGridCoordinate>() == 64);
    assert!(core::mem::align_of::<SpatialGridCoordinate>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_decays_by_two_to_the_minus_eight_of_itself_each_step_and_reaches_zero() {
        let mut g = SpatialGridCoordinate::default();
        g.fix(0, 0, 0, 0);
        let one = g.path_integration_confidence_q16;
        assert!(one > 0, "a fix restores confidence");
        g.integrate(0, 0, 0, 0);
        assert_eq!(
            g.path_integration_confidence_q16,
            one - (one >> CONFIDENCE_DECAY_SHIFT),
            "one step takes 2^-8 of it"
        );
        for _ in 0..10_000 {
            g.integrate(0, 0, 0, 0);
        }
        assert_eq!(
            g.path_integration_confidence_q16, 0,
            "and it reaches zero exactly"
        );
    }

    #[test]
    fn record_is_one_cache_line_and_default_is_the_origin_with_no_confidence() {
        assert_eq!(core::mem::size_of::<SpatialGridCoordinate>(), 64);
        assert_eq!(core::mem::align_of::<SpatialGridCoordinate>(), 64);
        let d = SpatialGridCoordinate::default();
        assert_eq!(
            d.path_integration_confidence_q16, 0,
            "an unfixed map is not trusted"
        );
    }

    #[test]
    fn a_fix_places_the_body_and_restores_confidence() {
        let mut c = SpatialGridCoordinate::default();
        c.fix(0x0003_0000, -0x0001_0000, 0, 0x4000);
        assert_eq!(
            (c.grid_x_q16, c.grid_y_q16, c.heading_yaw_turns),
            (0x0003_0000, -0x0001_0000, 0x4000)
        );
        assert_eq!(c.path_integration_confidence_q16, Q16_ONE);
        assert_eq!(c.steps_since_fix, 0);
    }

    #[test]
    fn integration_moves_the_body_and_the_yaw_wraps_within_the_turn() {
        let mut c = SpatialGridCoordinate::default();
        c.fix(0, 0, 0, 0xFFFF);
        c.integrate(0x0000_8000, 0, -0x0000_8000, 2);
        assert_eq!((c.grid_x_q16, c.grid_z_q16), (0x0000_8000, -0x0000_8000));
        assert_eq!(c.heading_yaw_turns, 1, "0xFFFF + 2 wraps to 1");
        assert_eq!(c.steps_since_fix, 1);
    }

    #[test]
    fn position_saturates_at_the_edge_of_the_map() {
        let mut c = SpatialGridCoordinate::default();
        c.fix(i32::MAX, i32::MIN, 0, 0);
        c.integrate(1, -1, 0, 0);
        assert_eq!((c.grid_x_q16, c.grid_y_q16), (i32::MAX, i32::MIN));
    }

    #[test]
    fn confidence_decays_monotonically_and_never_below_zero() {
        let mut c = SpatialGridCoordinate::default();
        c.fix(0, 0, 0, 0);
        let mut last = c.path_integration_confidence_q16;
        for _ in 0..5000 {
            c.integrate(0, 0, 0, 0);
            assert!(c.path_integration_confidence_q16 <= last);
            last = c.path_integration_confidence_q16;
        }
        assert!(
            last < Q16_ONE / 1000,
            "after 5000 steps almost nothing is left"
        );
        c.integrate(0, 0, 0, 0);
        assert!(
            c.path_integration_confidence_q16 <= last,
            "the floor is reached without wrapping"
        );
    }
}
