//! Zero-Overhead Kernel-Bypass Observability & LFP/EEG Synthesizer

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug)]
pub struct LfpSamplePacket {
    pub timestamp_us: u64,
    pub column_id: u32,
    pub lfp_voltage_uv: i32,
    pub band_delta: u32,
    pub band_theta: u32,
    pub band_alpha: u32,
    pub band_beta: u32,
    pub band_gamma: u32,
    pub spike_count: u32,
    pub _padding: [u8; 24],
}

const _: () = {
    assert!(core::mem::size_of::<LfpSamplePacket>() == 64);
    assert!(core::mem::align_of::<LfpSamplePacket>() == 64);
};
