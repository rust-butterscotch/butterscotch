/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;

pub struct Accumulator {
    timer: Timer,
    accum: Time,
    delta: Time,
    accum_max: Time,
}

impl Accumulator {

    pub fn new(delta: Time, count_max: u64) -> Accumulator {
        Accumulator{
            timer: Timer::new(),
            accum: Time::ZERO,
            delta,
            accum_max: if count_max == 0 {
                Time::MAX
            } else {
                delta * count_max
            }
        }
    }

    pub fn accumulate(&mut self) {
        self.timer.mark();
        self.accum = (self.accum + self.timer.delta()).min(self.accum_max);
    }

    pub fn consume(&mut self) {
        self.accum = (self.accum - self.delta).max(Time::ZERO);
    }

    pub fn consume_all(&mut self) {
        self.accum = Time::ZERO;
    }

    pub fn amount(&self) -> Time {
        self.accum
    }

    pub fn count(&self) -> u64 {
        (self.accum/self.delta).trunc().into()
    }

    pub fn has_accumulated(&self) -> bool {
        self.accum >= self.delta
    }

    pub fn dt_real(&self) -> Time {
        self.timer.delta()
    }

    pub fn dt_fixed(&self) -> Time {
        self.delta
    }

    pub fn dt_total(&self) -> Time {
        self.timer.total()
    }

}