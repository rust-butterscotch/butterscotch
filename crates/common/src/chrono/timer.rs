/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::time::Instant;

#[derive(Debug, Copy, Clone)]
pub struct Timer {
    start:   Instant,
    last:    Instant,
    current: Instant
}

impl Timer {
    pub fn new() -> Timer {
        let time = Instant::now();
        Timer{ start: time, last: time, current: time }
    }

    pub fn reset(&mut self) {
        self.current = Instant::now();
        self.last = self.current;
        self.start = self.last;
    }

    pub fn mark(&mut self) {
        self.last = self.current;
        self.current = Instant::now();
    }

    pub fn delta_nano(&self) -> u64 {
        self.current.duration_since(self.last).as_nanos().min(u64::MAX as u128) as u64
    }

    pub fn delta(&self) -> f64 {
        (self.current.duration_since(self.last).as_nanos().min(u64::MAX as u128) as f64)/1e9
    }

    pub fn total_nano(&self) -> u64 {
        self.current.duration_since(self.start).as_nanos().min(u64::MAX as u128) as u64
    }

    pub fn total(&self) -> f64 {
        (self.current.duration_since(self.start).as_nanos().min(u64::MAX as u128) as f64)/1e9
    }
}