//! The arenas the workers share. The one `unsafe` in the workspace lives here (whitepaper
//! TC-9; ADR-0023). A record is handed out as `&mut` only to the worker that holds its turn
//! (the gate of axiom A3) or that owns its synapse blocks, and as `&` only in a phase in which
//! no `&mut` to it can exist. The executor's barrier-separated phases are that discipline;
//! every call site of [`Arena::get`] and [`Arena::get_mut`] states which phase it is in and
//! why no other reference to the record is alive.

use core::cell::UnsafeCell;

/// A fixed arena of records, allocated once.
pub struct Arena<T> {
    cells: Box<[UnsafeCell<T>]>,
}

// SAFETY: an arena is shared between worker threads. The executor guarantees that a record is
// never referenced mutably from two threads at once, and never mutably and immutably at once:
// `&mut` references are created only inside a phase, for records the phase gives this worker
// exclusively, and are dropped before the barrier that ends the phase; `&` references are
// created only in phases with no `&mut` to the same record (ADR-0023). `T: Send` because a
// record may be mutated from any worker; `T: Sync` because it may be read from any worker.
unsafe impl<T: Send + Sync> Sync for Arena<T> {}
// SAFETY: moving the arena to another thread moves every record; `T: Send` covers that.
unsafe impl<T: Send> Send for Arena<T> {}

impl<T> Arena<T> {
    /// An arena holding `records`, in order.
    pub fn from_vec(records: Vec<T>) -> Self {
        Self {
            cells: records.into_iter().map(UnsafeCell::new).collect(),
        }
    }

    /// Number of records.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// True for an arena of no records.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// A shared reference to record `index`, or `None` outside the arena.
    ///
    /// # Safety
    ///
    /// No `&mut` to record `index` may exist, on any thread, while the returned reference is
    /// alive.
    pub unsafe fn get(&self, index: usize) -> Option<&T> {
        // SAFETY: the caller upholds the aliasing rule above; the cell's pointer is valid for
        // the arena's lifetime.
        self.cells.get(index).map(|c| unsafe { &*c.get() })
    }

    /// An exclusive reference to record `index`, or `None` outside the arena.
    ///
    /// # Safety
    ///
    /// No other reference to record `index`, shared or exclusive, may exist on any thread
    /// while the returned reference is alive.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self, index: usize) -> Option<&mut T> {
        // SAFETY: the caller upholds the exclusivity rule above; the cell's pointer is valid
        // for the arena's lifetime.
        self.cells.get(index).map(|c| unsafe { &mut *c.get() })
    }

    /// Every record, shared.
    ///
    /// # Safety
    ///
    /// No `&mut` to any record may exist, on any thread, while the returned slice is alive.
    pub unsafe fn as_slice(&self) -> &[T] {
        let first = UnsafeCell::raw_get(self.cells.as_ptr()).cast_const();
        // SAFETY: the pointer comes from the boxed slice, so it has provenance over every
        // cell; `UnsafeCell<T>` is `#[repr(transparent)]`, so consecutive cells are consecutive
        // records; the caller upholds the aliasing rule above.
        unsafe { std::slice::from_raw_parts(first, self.cells.len()) }
    }

    /// Every record, exclusive.
    ///
    /// # Safety
    ///
    /// No other reference to any record may exist, on any thread, while the returned slice is
    /// alive.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        let first = UnsafeCell::raw_get(self.cells.as_ptr());
        // SAFETY: as in `as_slice`, through the cell's own mutable pointer, plus the
        // exclusivity the caller upholds.
        unsafe { std::slice::from_raw_parts_mut(first, self.cells.len()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_arena_holds_its_records_in_order_and_refuses_an_index_outside() {
        let a = Arena::from_vec(vec![10u32, 20, 30]);
        assert_eq!(a.len(), 3);
        assert!(!a.is_empty());
        // SAFETY: a single thread, no `&mut` alive.
        unsafe {
            assert_eq!(a.get(1), Some(&20));
            assert_eq!(a.get(3), None);
            assert_eq!(a.as_slice(), &[10, 20, 30]);
        }
        // SAFETY: a single thread; the previous references are dead.
        unsafe {
            *a.get_mut(2).unwrap() = 33;
            a.as_mut_slice()[0] = 11;
        }
        // SAFETY: as above.
        unsafe {
            assert_eq!(a.as_slice(), &[11, 20, 33]);
        }
        assert!(Arena::<u8>::from_vec(Vec::new()).is_empty());
    }
}
