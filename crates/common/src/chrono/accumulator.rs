/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;

pub struct Accumulator {
    timer: Timer,
    accum: u64,
    delta: u64,
    accum_max: u64,
}

impl Accumulator {

    pub fn new(delta_f: f64, count_max: u64) -> Accumulator {
        let delta = (delta_f*1e9).round() as u64;
        Accumulator{
            timer: Timer::new(),
            accum: 0,
            delta,
            accum_max: if count_max == 0 {
                u64::MAX
            } else {
                delta * count_max
            }
        }
    }

    pub fn accumulate(&mut self) {
        self.timer.mark();
        self.accum = (self.accum + self.timer.delta_nano()).min(self.accum_max);
    }

    pub fn consume(&mut self) {
        self.accum = self.accum.saturating_sub(self.delta);
    }

    pub fn consume_all(&mut self) {
        self.accum = 0;
    }

    pub fn total(&self) -> f64 {
        (self.accum as f64)/1e9
    }

    pub fn total_nano(&self) -> u64 {
        self.accum
    }

    pub fn count(&self) -> u64 {
        self.accum/self.delta
    }

    pub fn has_accumulated(&self) -> bool {
        self.accum >= self.delta
    }


    pub fn dt_real(&self) -> f64 {
        self.timer.delta()
    }

    pub fn dt_real_nano(&self) -> u64 {
        self.timer.delta_nano()
    }


    pub fn dt_fixed(&self) -> f64 {
        (self.delta as f64)/1e9
    }

    pub fn dt_fixed_nano(&self) -> u64 {
        self.delta
    }


    pub fn dt_total(&self) -> f64 {
        self.timer.total()
    }

    pub fn dt_total_nano(&self) -> u64 {
        self.timer.total_nano()
    }

}