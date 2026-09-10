//! Autonomic vitals: the hardware substrate's voltage, temperature and power, the limits they
//! are held against, and the emergency flags that cross them (whitepaper §5.2.24, §8.9;
//! admitted by ADR-0016).
//!
//! One record per worker core. The runtime samples the platform sensors (a driver, outside the
//! tick loop) and calls [`AutonomicVitalsState::sample`]; the flags it returns are what the
//! power-shedding and throttling policy acts on. The limit check is Implemented; the policy,
//! and how `arousal_tone_q16` feeds `cortex-neuromod`, are Specified. The external watchdog of
//! whitepaper §8.9 remains the last line of defence; these flags are the first.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029; this one 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

/// `emergency_cut_flags` bit: core temperature above `thermal_limit_milli_c`.
pub const CUT_OVER_TEMPERATURE: u16 = 0x0001;
/// `emergency_cut_flags` bit: power draw above `power_limit_mw`.
pub const CUT_OVER_POWER: u16 = 0x0002;
/// `emergency_cut_flags` bit: bus voltage below `voltage_floor_mv`.
pub const CUT_UNDER_VOLTAGE: u16 = 0x0004;

/// 64-byte vitals record (whitepaper §5.2.24).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct AutonomicVitalsState {
    pub watchdog_heartbeat_counter: u64, // [0..8] Samples taken; wraps
    pub bus_voltage_mv: u32,             // [8..12] Last sampled bus voltage, mV
    pub core_temperature_milli_c: i32,   // [12..16] Last sampled core temperature, m°C
    pub power_draw_mw: u32,              // [16..20] Last sampled power draw, mW
    pub arousal_tone_q16: u32,           // [20..24] Autonomic arousal (Q16.16, Specified)
    pub thermal_limit_milli_c: i32,      // [24..28] Cut above this temperature
    pub power_limit_mw: u32,             // [28..32] Cut above this power
    pub voltage_floor_mv: u32,           // [32..36] Cut below this voltage
    pub emergency_cut_flags: u16,        // [36..38] CUT_* bits from the last sample
    pub _reserved: [u8; 26],             // [38..64] Reserved; MUST be zero
}

impl AutonomicVitalsState {
    /// A record with the given limits and no sample yet.
    pub const fn with_limits(
        thermal_limit_milli_c: i32,
        power_limit_mw: u32,
        voltage_floor_mv: u32,
    ) -> Self {
        Self {
            watchdog_heartbeat_counter: 0,
            bus_voltage_mv: 0,
            core_temperature_milli_c: 0,
            power_draw_mw: 0,
            arousal_tone_q16: 0,
            thermal_limit_milli_c,
            power_limit_mw,
            voltage_floor_mv,
            emergency_cut_flags: 0,
            _reserved: [0; 26],
        }
    }

    /// Records one sample, advances the heartbeat (wrapping), recomputes every flag from this
    /// sample alone, and returns the flags. A flag from an earlier sample does not persist:
    /// the policy that acts on it is the runtime's.
    pub fn sample(&mut self, voltage_mv: u32, temperature_milli_c: i32, power_mw: u32) -> u16 {
        self.bus_voltage_mv = voltage_mv;
        self.core_temperature_milli_c = temperature_milli_c;
        self.power_draw_mw = power_mw;
        self.watchdog_heartbeat_counter = self.watchdog_heartbeat_counter.wrapping_add(1);
        let mut flags = 0;
        if temperature_milli_c > self.thermal_limit_milli_c {
            flags |= CUT_OVER_TEMPERATURE;
        }
        if power_mw > self.power_limit_mw {
            flags |= CUT_OVER_POWER;
        }
        if voltage_mv < self.voltage_floor_mv {
            flags |= CUT_UNDER_VOLTAGE;
        }
        self.emergency_cut_flags = flags;
        flags
    }

    /// True when the last sample crossed no limit.
    #[inline]
    pub const fn is_within_limits(&self) -> bool {
        self.emergency_cut_flags == 0
    }
}

const _: () = {
    assert!(core::mem::size_of::<AutonomicVitalsState>() == 64);
    assert!(core::mem::align_of::<AutonomicVitalsState>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reading_exactly_at_a_limit_is_within_it_and_one_past_is_a_cut() {
        let mut v = vitals();
        assert_eq!(
            v.sample(11_400, 85_000, 250_000),
            0,
            "at every limit at once"
        );
        assert_eq!(v.sample(11_399, 85_000, 250_000), CUT_UNDER_VOLTAGE);
        assert_eq!(v.sample(11_400, 85_001, 250_000), CUT_OVER_TEMPERATURE);
        assert_eq!(v.sample(11_400, 85_000, 250_001), CUT_OVER_POWER);
        assert_eq!(
            v.sample(0, i32::MAX, u32::MAX),
            CUT_UNDER_VOLTAGE | CUT_OVER_TEMPERATURE | CUT_OVER_POWER
        );
        assert_eq!(v.sample(u32::MAX, i32::MIN, 0), 0);
    }

    fn vitals() -> AutonomicVitalsState {
        AutonomicVitalsState::with_limits(85_000, 250_000, 11_400)
    }

    #[test]
    fn record_is_one_cache_line_and_default_is_zero() {
        assert_eq!(core::mem::size_of::<AutonomicVitalsState>(), 64);
        assert_eq!(core::mem::align_of::<AutonomicVitalsState>(), 64);
        let d = AutonomicVitalsState::default();
        assert_eq!(d.watchdog_heartbeat_counter, 0);
        assert!(d.is_within_limits());
    }

    #[test]
    fn a_sample_within_limits_sets_no_flag_and_beats_the_heart() {
        let mut v = vitals();
        assert_eq!(v.sample(12_000, 60_000, 180_000), 0);
        assert!(v.is_within_limits());
        assert_eq!(v.watchdog_heartbeat_counter, 1);
        assert_eq!(
            (
                v.bus_voltage_mv,
                v.core_temperature_milli_c,
                v.power_draw_mw
            ),
            (12_000, 60_000, 180_000)
        );
    }

    #[test]
    fn each_limit_sets_its_own_flag_at_strictly_beyond_it() {
        let mut v = vitals();
        assert_eq!(
            v.sample(12_000, 85_000, 250_000),
            0,
            "at the limit is within it"
        );
        assert_eq!(v.sample(12_000, 85_001, 180_000), CUT_OVER_TEMPERATURE);
        assert_eq!(v.sample(12_000, 60_000, 250_001), CUT_OVER_POWER);
        assert_eq!(v.sample(11_399, 60_000, 180_000), CUT_UNDER_VOLTAGE);
        assert_eq!(
            v.sample(11_000, 90_000, 300_000),
            CUT_OVER_TEMPERATURE | CUT_OVER_POWER | CUT_UNDER_VOLTAGE
        );
    }

    #[test]
    fn flags_are_recomputed_from_each_sample_and_do_not_latch() {
        let mut v = vitals();
        v.sample(11_000, 90_000, 300_000);
        assert!(!v.is_within_limits());
        assert_eq!(v.sample(12_000, 60_000, 180_000), 0);
        assert!(v.is_within_limits());
    }

    #[test]
    fn the_heartbeat_wraps_rather_than_panics() {
        let mut v = vitals();
        v.watchdog_heartbeat_counter = u64::MAX;
        v.sample(12_000, 60_000, 180_000);
        assert_eq!(v.watchdog_heartbeat_counter, 0);
    }
}
