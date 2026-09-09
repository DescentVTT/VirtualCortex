//! Two-Tier Flat Timing Ring Implementation (Zero Allocation)

pub struct FlatTimingWheel {
    pub cursor: usize,
    pub fine_ring: [u64; 200],   // 10us slots
    pub coarse_ring: [u64; 80],  // 100us slots
}

impl FlatTimingWheel {
    pub const fn new() -> Self {
        Self {
            cursor: 0,
            fine_ring: [0; 200],
            coarse_ring: [0; 80],
        }
    }

    #[inline(always)]
    pub fn schedule_fine(&mut self, delay_ticks: usize, event_mask: u64) {
        let slot = (self.cursor + delay_ticks) % 200;
        self.fine_ring[slot] |= event_mask;
    }
}
