//! Hot-Pluggable Sensory Ingestion & Thalamic HAL

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default)]
pub struct SensoryEvent {
    pub timestamp_us: u32,
    pub address: u16,
    pub peripheral_type: u8,
    pub payload: u8,
}

const _: () = {
    assert!(core::mem::size_of::<SensoryEvent>() == 8);
    assert!(core::mem::align_of::<SensoryEvent>() == 8);
};

pub trait SensoryPeripheral: Send + Sync {
    fn poll_batch(&mut self, output: &mut [SensoryEvent]) -> usize;
    fn peripheral_name(&self) -> &'static str;
    fn channel_count(&self) -> usize;
}
