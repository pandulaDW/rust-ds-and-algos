#![allow(dead_code)]

use std::sync::{Arc, Condvar, Mutex};

struct Wg {
    lock: Mutex<()>,
    cond: Condvar,
}

#[derive(Clone)]
pub struct WaitGroup(Arc<Wg>);

impl Drop for WaitGroup {
    fn drop(&mut self) {
        self.0.cond.notify_one();
    }
}

impl WaitGroup {
    /// Create a new `WaitGroup` instance.
    ///
    /// You can use clone to creates copies of this instance that can be safely moved across threads.
    pub fn new() -> Self {
        let wg = Wg {
            lock: Mutex::new(()),
            cond: Condvar::new(),
        };
        WaitGroup(Arc::new(wg))
    }

    /// This call would block the invoked thread until all the clones are dropped
    pub fn wait(self) {
        let mut started = self.0.lock.lock().unwrap();

        // take ownership of the lock and and returning it back at each iteration
        while Arc::strong_count(&self.0) != 1 {
            started = self.0.cond.wait(started).unwrap();
        }
    }
}
