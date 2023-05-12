use core::task::Waker;

use heapless::Deque;

/// Utility struct to register and wake multiple wakers.
pub struct MultiWakerRegistration<const N: usize> {
    wakers: Deque<Waker, N>,
}

impl<const N: usize> MultiWakerRegistration<N> {
    /// Create a new empty instance
    pub const fn new() -> Self {
        Self { wakers: Deque::new() }
    }

    /// Register a waker. If the buffer is full the function returns it in the error
    pub fn register<'a>(&mut self, w: &'a Waker) -> Result<(), &'a Waker> {
        // If there is already a waker in queue that will wake the same task as the new waker,
        // skip the clone
        for existing_waker in &self.wakers {
            if existing_waker.will_wake(w) {
                return Ok(());
            }
        }

        self.wakers.push_back(w.clone()).map_err(|_| w)
    }

    pub fn register_infallable<'a>(&mut self, w: &'a Waker) {
        if let Err(waker) = self.register(w) {
            // Failed to register due to full buffer
            // Replace the old waker with the new one, 
            // wake the old waker so the other task can 
            // reregister itself if it's still interested.
            let old_waker = self.wakers.pop_back().unwrap();
            self.wakers.push_back(waker.clone()).unwrap();
            old_waker.wake();
        }
    }

    /// Wake all registered wakers. This clears the buffer
    pub fn wake(&mut self) {
        while let Some(waker) = self.wakers.pop_front() {
            waker.wake();
        }
    }
}
