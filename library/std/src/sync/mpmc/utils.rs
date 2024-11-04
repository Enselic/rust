use crate::cell::Cell;
use crate::ops::{Deref, DerefMut};

pub struct CachePadded<T> {
    value: T,
}

impl<T> CachePadded<T> {
    /// Pads and aligns a value to the length of a cache line.
    pub fn new(value: T) -> CachePadded<T> {
        CachePadded::<T> { value }
    }
}

impl<T> Deref for CachePadded<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for CachePadded<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

const SPIN_LIMIT: u32 = 6;

/// Performs quadratic backoff in spin loops.
pub struct Backoff {
    step: Cell<u32>,
}

impl Backoff {
    /// Creates a new `Backoff`.
    pub fn new() -> Self {
        Backoff { step: Cell::new(0) }
    }

    /// Backs off using lightweight spinning.
    ///
    /// This method should be used for retrying an operation because another thread made
    /// progress. i.e. on CAS failure.
    #[inline]
    pub fn spin_light(&self) {
        let step = self.step.get().min(SPIN_LIMIT);
        for _ in 0..step.pow(2) {
            crate::hint::spin_loop();
        }

        self.step.set(self.step.get() + 1);
    }

    /// Backs off using heavyweight spinning.
    ///
    /// This method should be used in blocking loops where parking the thread is not an option.
    #[inline]
    pub fn spin_heavy(&self) {
        if self.step.get() <= SPIN_LIMIT {
            for _ in 0..self.step.get().pow(2) {
                crate::hint::spin_loop()
            }
        } else {
            crate::thread::yield_now();
        }

        self.step.set(self.step.get() + 1);
    }
}
