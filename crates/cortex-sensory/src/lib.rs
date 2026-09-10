//! Sensory ingestion: the 8-byte address-event record and the peripheral driver trait
//! (whitepaper §5.2.3). Hot-plug is Specified (§6.3); the relay gate is `cortex-thalamus`.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(8))]
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A driver that produces up to three events per poll into the caller's slice.
    struct Stub {
        produced: u32,
    }

    impl SensoryPeripheral for Stub {
        fn poll_batch(&mut self, output: &mut [SensoryEvent]) -> usize {
            let n = output.len().min(3);
            for (i, slot) in output.iter_mut().take(n).enumerate() {
                // A stub's counter: three events per poll, a handful of polls.
                self.produced = self.produced.wrapping_add(1);
                *slot = SensoryEvent {
                    timestamp_us: self.produced,
                    address: i as u16,
                    peripheral_type: 2,
                    payload: 0xAB,
                };
            }
            n
        }

        fn peripheral_name(&self) -> &'static str {
            "stub"
        }

        fn channel_count(&self) -> usize {
            3
        }
    }

    #[test]
    fn event_is_eight_bytes_eight_aligned() {
        assert_eq!(core::mem::size_of::<SensoryEvent>(), 8);
        assert_eq!(core::mem::align_of::<SensoryEvent>(), 8);
    }

    #[test]
    fn default_event_is_all_zero() {
        assert_eq!(
            SensoryEvent::default(),
            SensoryEvent {
                timestamp_us: 0,
                address: 0,
                peripheral_type: 0,
                payload: 0
            }
        );
    }

    #[test]
    fn a_driver_fills_the_callers_slice_and_reports_the_count() {
        let mut stub = Stub { produced: 0 };
        // The relay holds drivers as trait objects; the trait must be object-safe.
        let driver: &mut dyn SensoryPeripheral = &mut stub;
        let mut buf = [SensoryEvent::default(); 8];
        assert_eq!(driver.poll_batch(&mut buf), 3);
        assert_eq!(buf[2].address, 2);
        assert_eq!(buf[2].timestamp_us, 3);
        assert_eq!(buf[2].peripheral_type, 2);
        assert_eq!(
            buf[3],
            SensoryEvent::default(),
            "untouched beyond the count"
        );
        let mut small = [SensoryEvent::default(); 2];
        assert_eq!(driver.poll_batch(&mut small), 2, "never exceeds the slice");
        assert_eq!(driver.peripheral_name(), "stub");
        assert_eq!(driver.channel_count(), 3);
    }

    #[test]
    fn a_driver_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Stub>();
    }
}
