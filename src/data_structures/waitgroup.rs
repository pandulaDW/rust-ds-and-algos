#![allow(dead_code)]

use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc, Condvar, Mutex,
};

struct Wg {
    counter: AtomicI32,
    lock: Mutex<bool>,
    cond: Condvar,
}

#[derive(Clone)]
pub struct WaitGroup(Arc<Wg>);

impl WaitGroup {
    pub fn new() -> Self {
        let wg = Wg {
            counter: AtomicI32::new(0),
            lock: Mutex::new(false),
            cond: Condvar::new(),
        };
        WaitGroup(Arc::new(wg))
    }

    pub fn add(&self, val: u32) {
        self.0.counter.fetch_add(val as i32, Ordering::SeqCst);
    }

    pub fn done(&self) {
        self.0.counter.fetch_add(-1, Ordering::SeqCst);
        self.0.cond.notify_one();
    }

    pub fn wait(self) {
        let mut started = self.0.lock.lock().unwrap();

        // take ownership of the lock and and returning it back at each iteration
        while self.0.counter.load(Ordering::SeqCst) != 0 {
            started = self.0.cond.wait(started).unwrap();
        }
    }
}
