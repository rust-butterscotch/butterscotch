/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::cell::Cell;

use super::Timer;

#[derive(Debug, Clone)]
pub struct TimerSmooth<const SAMPLE_COUNT: usize> {
    index: usize,
    sample_count: usize,
    samples: [u64; SAMPLE_COUNT],
    max: f64,
    min: f64,
    timer: Timer,
    cache_valid: Cell<bool>,
    cache: Cell<u64>,
}

impl<const SAMPLE_COUNT: usize> TimerSmooth<SAMPLE_COUNT> {

    pub fn new() -> Self {
        Self{
            index: 0,
            sample_count: 0,
            samples: [0; SAMPLE_COUNT],
            timer: Timer::new(),
            max: 0.0,
            min: f64::MAX,
            cache_valid: Cell::new(false),
            cache: Cell::new(0),
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
        self.samples[self.index] = self.timer.delta_nano();
        self.index = (self.index + 1) % SAMPLE_COUNT;
        self.sample_count = (self.sample_count + 1).min(SAMPLE_COUNT);

        self.max = self.timer.delta().max(self.max);
        self.min = self.timer.delta().min(self.min);

        self.cache_valid.replace(false);
    }

    pub fn tps_average(&self) -> f64 {
        let time = self.time_avg();
        if time != 0.0 { time.recip() } else { 0.0 }
    }

    pub fn tps_max(&self) -> f64 {
        let time = self.time_max();
        if time != 0.0 { time.recip() } else { 0.0 }
    }

    pub fn tps_min(&self) -> f64 {
        let time = self.time_min();
        if time != 0.0 { time.recip() } else { 0.0 }
    }

    pub fn time_avg(&self) -> f64 {

        if !self.cache_valid.get() {
            self.cache.replace(if self.sample_count <= 0 {
                0
            } else {
                let mut t = 0;
                for i in 0..self.sample_count {
                    t += self.samples[i];
                }
                t/(self.sample_count as u64)
            });
            self.cache_valid.replace(true);
        }

        (self.cache.get() as f64)/1e9
    }

    pub fn time_max(&self) -> f64 {
        (self.max as f64)/1e9
    }

    pub fn time_min(&self) -> f64 {
        if self.min > self.max {
            (self.max as f64)/1e9
        } else {
            (self.min as f64)/1e9
        }
    }

}