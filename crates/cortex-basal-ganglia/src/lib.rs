//! Basal Ganglia Action Selection, Striatal Dual-Pathway & STN Gating

#[repr(C, align(64))]
pub struct BasalGangliaChannelState {
    pub channel_id: u32,            // [0..4] Action channel index (0..63)
    pub striatal_d1_drive: i32,     // [4..8] Direct pathway Go activation (Q16.16)
    pub striatal_d2_drive: i32,     // [8..12] Indirect pathway No-Go activation (Q16.16)
    pub stn_hyperdirect_drive: i32, // [12..16] Hyperdirect emergency brake (Q16.16)
    pub gpi_snr_inhibition: i32,    // [16..20] Net basal ganglia output to thalamus (Q16.16)
    pub dopamine_modulation: i32,   // [20..24] Local striatal DA concentration (Q16.16)
    pub habit_strength: u32,        // [24..28] Procedural habit chunking weight (Q16.16)
    pub selected_flag: u32,         // [28..32] 1 if channel won winner-take-all, 0 otherwise
    pub _reserved: [u8; 32],        // [32..64] Strict 64-byte cache-line alignment padding
}

impl BasalGangliaChannelState {
    #[inline(always)]
    pub fn compute_gating(&mut self) -> bool {
        // Direct pathway disinhibits thalamus; indirect + STN hyperdirect reinforce inhibition.
        // Saturating arithmetic (whitepaper §8.1): an extreme drive clamps rather than wraps,
        // so the sign of the net output, and therefore the selection, is preserved.
        let net_output = self
            .striatal_d2_drive
            .saturating_add(self.stn_hyperdirect_drive)
            .saturating_sub(self.striatal_d1_drive);
        self.gpi_snr_inhibition = net_output;
        // If net output is below zero, thalamocortical loop is released (Action Gated)
        let is_selected = net_output < 0;
        self.selected_flag = if is_selected { 1 } else { 0 };
        is_selected
    }
}

const _: () = {
    assert!(core::mem::size_of::<BasalGangliaChannelState>() == 64);
    assert!(core::mem::align_of::<BasalGangliaChannelState>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: i32 = 0x0001_0000;

    fn channel(d1: i32, d2: i32, stn: i32) -> BasalGangliaChannelState {
        BasalGangliaChannelState {
            channel_id: 0,
            striatal_d1_drive: d1,
            striatal_d2_drive: d2,
            stn_hyperdirect_drive: stn,
            gpi_snr_inhibition: 0,
            dopamine_modulation: 0,
            habit_strength: 0,
            selected_flag: 0,
            _reserved: [0; 32],
        }
    }

    #[test]
    fn go_drive_releases_the_channel() {
        let mut c = channel(ONE, 0, 0);
        assert!(c.compute_gating());
        assert_eq!(c.gpi_snr_inhibition, -ONE);
        assert_eq!(c.selected_flag, 1);
    }

    #[test]
    fn no_go_plus_brake_holds_the_channel() {
        let mut c = channel(ONE, ONE, ONE / 2);
        assert!(!c.compute_gating());
        assert_eq!(c.gpi_snr_inhibition, ONE / 2);
        assert_eq!(c.selected_flag, 0);
    }

    #[test]
    fn maximal_inhibition_saturates_and_stays_suppressed() {
        // d2 + stn would overflow; it clamps at i32::MAX instead of wrapping negative.
        let mut c = channel(0, i32::MAX, i32::MAX);
        assert!(!c.compute_gating());
        assert_eq!(c.gpi_snr_inhibition, i32::MAX);
    }

    #[test]
    fn maximal_release_saturates_and_stays_selected() {
        // i32::MIN - i32::MAX wraps to +1 under wrapping arithmetic, which would suppress a
        // channel that should be released; saturation keeps it at i32::MIN.
        let mut c = channel(i32::MAX, i32::MIN, 0);
        assert!(c.compute_gating());
        assert_eq!(c.gpi_snr_inhibition, i32::MIN);
    }

    #[test]
    fn maximal_go_drive_alone_is_selected() {
        let mut c = channel(i32::MAX, 0, 0);
        assert!(c.compute_gating());
        assert_eq!(c.gpi_snr_inhibition, -i32::MAX);
    }
}
