/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::cell::Cell;

use super::{Time, Timer};

#[derive(Debug, Clone)]
pub struct TimerSmooth<const SAMPLE_COUNT: usize> {
    index: usize,
    sample_count: usize,
    samples: [Time; SAMPLE_COUNT],
    max: Time,
    min: Time,
    timer: Timer,
    cache_valid: Cell<bool>,
    cache: Cell<Time>,
}

impl<const SAMPLE_COUNT: usize> TimerSmooth<SAMPLE_COUNT> {

    pub fn new() -> Self {
        Self{
            index: 0,
            sample_count: 0,
            samples: [Time::ZERO; SAMPLE_COUNT],
            timer: Timer::new(),
            max: Time::ZERO,
            min: Time::MAX,
            cache_valid: Cell::new(false),
            cache: Cell::new(Time::ZERO),
        }
    }

    pub fn end_start(&mut self) {
        self.end();
        self.start();
    }

    pub fn start(&mut self) {
        self.timer.reset();
    }

    pub fn end(&mut self) {
        self.timer.mark();
        self.samples[self.index] = self.timer.delta();
        self.index = (self.index + 1) % SAMPLE_COUNT;
        self.sample_count = (self.sample_count + 1).min(SAMPLE_COUNT);

        self.max = self.timer.delta().max(self.max);
        self.min = self.timer.delta().min(self.min);

        self.cache_valid.replace(false);
    }

    pub fn tps_average(&self) -> Time {
        let time = self.time_avg();
        if time != Time::ZERO { time.recip() } else { Time::ZERO }
    }

    pub fn tps_max(&self) -> Time {
        let time = self.time_max();
        if time != Time::ZERO { time.recip() } else { Time::ZERO }
    }

    pub fn tps_min(&self) -> Time {
        let time = self.time_min();
        if time != Time::ZERO { time.recip() } else { Time::ZERO }
    }

    pub fn time_avg(&self) -> Time {

        if !self.cache_valid.get() {
            self.cache.replace(if self.sample_count <= 0 {
                Time::ZERO
            } else {
                let mut t = Time::ZERO;
                for i in 0..self.sample_count {
                    t += self.samples[i];
                }
                t/self.sample_count
            });
            self.cache_valid.replace(true);
        }

        self.cache.get()
    }

    pub fn time_max(&self) -> Time {
        self.max
    }

    pub fn time_min(&self) -> Time {
        if self.min == Time::MAX {
            Time::ZERO
        } else {
            self.min
        }
    }

}