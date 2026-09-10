//! Thalamic relay nodes: the routing and gating stage between peripheral ingestion and the
//! cortical arenas (whitepaper §5.2.19, §6.3, §8.8; admitted by ADR-0016).
//!
//! One record per relay channel. The runtime maps a `SensoryEvent`'s
//! `(peripheral_type, address)` to a relay node, and the node decides whether and how strongly
//! the event reaches `target_cortical_column`: tonic mode passes every input scaled by the
//! gain, burst mode passes one input in [`BURST_LENGTH`] (a decimating gate), and closed mode
//! drops every input (sensory gating during sleep). The gate is Implemented; the burst
//! waveform and corticothalamic synchrony are Specified.

#![no_std]

/// Every input is relayed, scaled by the gain.
pub const GATING_TONIC: u8 = 0;
/// One input in [`BURST_LENGTH`] is relayed, scaled by the gain; the rest are withheld.
pub const GATING_BURST: u8 = 1;
/// No input is relayed.
pub const GATING_CLOSED: u8 = 2;
/// Inputs per burst in [`GATING_BURST`] mode.
pub const BURST_LENGTH: u8 = 4;

/// 64-byte relay node (whitepaper §5.2.19).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct ThalamicRelayNode {
    pub relay_channel_id: u32,       // [0..4] Relay channel index
    pub target_cortical_column: u32, // [4..8] Column the relayed input is delivered to
    pub sensory_gain_q16: u32,       // [8..12] Multiplier applied to a relayed input (Q16.16)
    pub oscillation_phase_q16: u32,  // [12..16] Corticothalamic phase, wraps at 16 bits (Specified)
    pub source_address: u16,         // [16..18] SensoryEvent::address this node relays
    pub source_peripheral_type: u8,  // [18] SensoryEvent::peripheral_type this node relays
    pub gating_mode: u8,             // [19] GATING_TONIC / GATING_BURST / GATING_CLOSED
    pub burst_spikes_pending: u8,    // [20] Inputs withheld toward the next burst
    pub _reserved: [u8; 43],         // [21..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for ThalamicRelayNode {
    fn default() -> Self {
        Self {
            relay_channel_id: 0,
            target_cortical_column: 0,
            sensory_gain_q16: 0,
            oscillation_phase_q16: 0,
            source_address: 0,
            source_peripheral_type: 0,
            gating_mode: 0,
            burst_spikes_pending: 0,
            _reserved: [0; 43],
        }
    }
}

impl ThalamicRelayNode {
    /// Switches the gate. Unknown modes are refused and nothing changes; a switch resets the
    /// burst counter so that a mode never inherits another mode's pending inputs.
    pub fn set_gating_mode(&mut self, mode: u8) -> bool {
        if mode > GATING_CLOSED {
            return false;
        }
        self.gating_mode = mode;
        self.burst_spikes_pending = 0;
        true
    }

    /// Relays one input through the gate: `Some(input × gain)` when the gate passes it,
    /// `None` when it is withheld or dropped. The product is widened to `i64` and clamped to
    /// the `i32` range (whitepaper §8.1).
    #[inline]
    pub fn relay(&mut self, input_q16: i32) -> Option<i32> {
        match self.gating_mode {
            GATING_TONIC => Some(self.scale(input_q16)),
            GATING_BURST => {
                self.burst_spikes_pending = self.burst_spikes_pending.saturating_add(1);
                if self.burst_spikes_pending >= BURST_LENGTH {
                    self.burst_spikes_pending = 0;
                    Some(self.scale(input_q16))
                } else {
                    None
                }
            }
            // An unknown mode, reachable only through the public field or an image, relays
            // nothing, like a closed gate (ADR-0028).
            _ => None,
        }
    }

    #[inline(always)]
    fn scale(&self, input_q16: i32) -> i32 {
        let product = (input_q16 as i64 * self.sensory_gain_q16 as i64) >> 16;
        product.clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }
}

const _: () = {
    assert!(core::mem::size_of::<ThalamicRelayNode>() == 64);
    assert!(core::mem::align_of::<ThalamicRelayNode>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: u32 = 0x0001_0000;

    fn node(mode: u8, gain: u32) -> ThalamicRelayNode {
        ThalamicRelayNode {
            gating_mode: mode,
            sensory_gain_q16: gain,
            ..Default::default()
        }
    }

    #[test]
    fn record_is_one_cache_line_and_default_is_zero() {
        assert_eq!(core::mem::size_of::<ThalamicRelayNode>(), 64);
        assert_eq!(core::mem::align_of::<ThalamicRelayNode>(), 64);
        let d = ThalamicRelayNode::default();
        assert_eq!(d.gating_mode, GATING_TONIC);
        assert_eq!(d.sensory_gain_q16, 0, "a default node relays silence");
    }

    #[test]
    fn tonic_mode_scales_every_input_by_the_gain() {
        let mut n = node(GATING_TONIC, ONE);
        assert_eq!(n.relay(0x0000_8000), Some(0x0000_8000));
        n.sensory_gain_q16 = ONE / 2;
        assert_eq!(n.relay(0x0001_0000), Some(0x0000_8000));
        assert_eq!(n.relay(-0x0001_0000), Some(-0x0000_8000));
    }

    #[test]
    fn closed_mode_drops_everything_and_changes_nothing() {
        let mut n = node(GATING_CLOSED, ONE);
        let before = n;
        assert_eq!(n.relay(i32::MAX), None);
        assert_eq!(n, before);
    }

    #[test]
    fn burst_mode_passes_one_input_in_burst_length() {
        let mut n = node(GATING_BURST, ONE);
        for _ in 0..(BURST_LENGTH - 1) {
            assert_eq!(n.relay(ONE as i32), None);
        }
        assert_eq!(n.burst_spikes_pending, BURST_LENGTH - 1);
        assert_eq!(n.relay(ONE as i32), Some(ONE as i32));
        assert_eq!(
            n.burst_spikes_pending, 0,
            "the counter restarts after a burst"
        );
    }

    #[test]
    fn a_large_gain_clamps_instead_of_wrapping() {
        let mut n = node(GATING_TONIC, u32::MAX);
        assert_eq!(n.relay(i32::MAX), Some(i32::MAX));
        assert_eq!(n.relay(i32::MIN), Some(i32::MIN));
    }

    #[test]
    fn an_unknown_mode_in_the_field_relays_nothing_like_a_closed_gate() {
        let mut n = ThalamicRelayNode {
            gating_mode: GATING_TONIC,
            sensory_gain_q16: 0x0001_0000,
            ..Default::default()
        };
        assert!(!n.set_gating_mode(3));
        n.gating_mode = 3;
        assert_eq!(n.relay(0x0001_0000), None);
        assert_eq!(n.burst_spikes_pending, 0, "and counts nothing");
    }

    #[test]
    fn set_gating_mode_refuses_unknown_modes_and_resets_the_counter() {
        let mut n = node(GATING_BURST, ONE);
        n.relay(1);
        assert_eq!(n.burst_spikes_pending, 1);
        assert!(!n.set_gating_mode(3));
        assert_eq!(
            n.gating_mode, GATING_BURST,
            "a refused switch changes nothing"
        );
        assert!(n.set_gating_mode(GATING_TONIC));
        assert_eq!(n.burst_spikes_pending, 0);
    }
}
