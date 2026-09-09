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
        // Direct pathway disinhibits thalamus; indirect + STN hyperdirect reinforce inhibition
        let net_output =
            self.striatal_d2_drive + self.stn_hyperdirect_drive - self.striatal_d1_drive;
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
