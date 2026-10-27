//! Telemetry: the 64-byte local-field-potential sample record (whitepaper §5.2.18). The ring,
//! band synthesis and streaming are Specified (§8.11).

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_is_one_cache_line() {
        assert_eq!(core::mem::size_of::<LfpSamplePacket>(), 64);
        assert_eq!(core::mem::align_of::<LfpSamplePacket>(), 64);
    }
}
