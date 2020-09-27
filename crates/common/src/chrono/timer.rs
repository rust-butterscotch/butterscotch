/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;
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

    pub fn delta(&self) -> Time {
        Time::new(
            self.current.duration_since(self.last).as_nanos().min(i64::MAX as u128) as i64
        )
    }

    pub fn total(&self) -> Time {
        Time::new(
            self.current.duration_since(self.start).as_nanos().min(i64::MAX as u128) as i64
        )
    }
}