//! A cadence: the ticks at which a rule slower than the tick runs (ADR-0035). A period of
//! $2^k$ ticks and a phase below it; a rule is due at the ticks whose low $k$ bits are the
//! phase. The period is a power of two for the two reasons the wheel's rings are
//! (ADR-0013): the test is a mask, not a division, and the wrap of the `u64` clock is exact,
//! since $2^{64}$ is a multiple of every period. A cadence is a function of the tick alone, so
//! a rule stepped on one runs at the same ticks on every worker count and after a reload
//! (the loader resumes the clock, ADR-0033), and adds no barrier: multirate stepping is a
//! schedule, not a scheduler.

/// The ticks at which a slow rule runs: every $2^{\text{shift}}$ ticks, at `phase`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cadence {
    mask: u64,
    phase: u64,
}

impl Cadence {
    /// A cadence of period $2^{\text{period\_shift}}$ ticks at `phase`; `None` for a shift the
    /// clock cannot hold (64 or more) or a phase at or beyond the period.
    pub const fn new(period_shift: u32, phase: u64) -> Option<Self> {
        if period_shift >= u64::BITS {
            return None;
        }
        // `period_shift < 64`: the shift cannot reach the width, and the period is at least 1.
        let mask = (1u64 << period_shift).wrapping_sub(1);
        if phase > mask {
            return None;
        }
        Some(Self { mask, phase })
    }

    /// True at the ticks whose low bits are the phase: `tick & (period − 1) == phase`.
    #[inline]
    pub const fn is_due(&self, tick: u64) -> bool {
        tick & self.mask == self.phase
    }

    /// The period in ticks.
    pub const fn period(&self) -> u64 {
        // The mask is one below a power of two, so it is below `u64::MAX`.
        self.mask.wrapping_add(1)
    }

    /// The phase, below the period.
    pub const fn phase(&self) -> u64 {
        self.phase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cadence_is_due_at_its_phase_every_period_and_at_no_other_tick() {
        let c = Cadence::new(3, 5).unwrap();
        assert_eq!((c.period(), c.phase()), (8, 5));
        let mut due = [u64::MAX; 6];
        let mut found = 0usize;
        for t in 0..40u64 {
            if c.is_due(t) {
                due[found] = t;
                found = found.wrapping_add(1);
            }
        }
        assert_eq!(due, [5, 13, 21, 29, 37, u64::MAX]);
        let every = Cadence::new(0, 0).unwrap();
        assert_eq!(every.period(), 1);
        assert!(
            (0..100).all(|t| every.is_due(t)),
            "a period of one is every tick"
        );
    }

    #[test]
    fn the_bounds_are_refused_and_their_neighbours_accepted() {
        assert_eq!(Cadence::new(64, 0), None, "the clock has 64 bits");
        assert_eq!(Cadence::new(u32::MAX, 0), None);
        let widest = Cadence::new(63, 0).unwrap();
        assert_eq!(widest.period(), 1 << 63);
        assert!(widest.is_due(0) && !widest.is_due(1));
        assert!(widest.is_due(1 << 63), "once per half of the clock's range");
        assert_eq!(Cadence::new(3, 8), None, "a phase at the period");
        assert_eq!(Cadence::new(3, u64::MAX), None);
        let last = Cadence::new(3, 7).unwrap();
        assert!(last.is_due(7) && last.is_due(15) && !last.is_due(8));
        assert_eq!(
            Cadence::new(63, (1 << 63) - 1).map(|c| c.phase()),
            Some((1 << 63) - 1)
        );
        assert_eq!(Cadence::new(63, 1 << 63), None);
    }

    #[test]
    fn the_clock_s_wrap_keeps_the_cadence_exact() {
        let c = Cadence::new(4, 3).unwrap();
        // The last due tick before the wrap is `u64::MAX - 12` (its low four bits are 3),
        // the next after it is 3: sixteen ticks apart across the wrap.
        assert!(c.is_due(u64::MAX - 12));
        assert!(!c.is_due(u64::MAX));
        assert!(!c.is_due(0));
        assert!(c.is_due(3));
        let across = 3u64.wrapping_sub(u64::MAX - 12);
        assert_eq!(across, 16, "one period across the wrap");
    }
}

/// Property tests (ADR-0030): every tick is due on exactly one phase of a period, and the
/// period's phases partition the walk.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    #[test]
    fn every_tick_is_due_on_exactly_one_phase_of_a_period() {
        let mut rng = Lcg::new(23);
        for shift in [0u32, 1, 2, 5, 12, 17, 31, 32, 63] {
            let period = 1u64 << shift;
            for i in 0..2_000u32 {
                let tick = if (i as usize) < U32_LATTICE.len() {
                    // The lattice and its images at the top of the clock.
                    U32_LATTICE[i as usize] as u64
                } else if i % 7 == 0 {
                    u64::MAX.wrapping_sub(rng.below(4_096) as u64)
                } else {
                    rng.next_u64()
                };
                let phases = if shift >= 12 { 4_096 } else { period };
                let mut due = 0u32;
                for phase in 0..phases {
                    let c = Cadence::new(shift, phase).unwrap();
                    assert_eq!(c.period(), period);
                    if c.is_due(tick) {
                        due = due.wrapping_add(1);
                        assert_eq!(tick & (period.wrapping_sub(1)), phase);
                    }
                }
                let expected = if tick & (period.wrapping_sub(1)) < phases {
                    1
                } else {
                    0
                };
                assert_eq!(due, expected, "shift {shift}, tick {tick}");
            }
        }
    }
}
