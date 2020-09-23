/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::collections::VecDeque;

use crate::chrono::{Accumulator, Time, Timer, TimerSmooth};

const SAMPLE_WINDOW: usize = 10;

pub struct Engine {
    accum_update: Accumulator,

    request_close: bool,
     should_close: bool,
    should_redraw: bool,
         has_init: bool,

    timer_update: TimerSmooth<10>,
    timer_frame:  TimerSmooth<10>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine{
            accum_update: Accumulator::new(Time::from_i64(60, 1).recip()),
            request_close: false,
            should_close: false,
            should_redraw: false,
            has_init: false,
            timer_update: TimerSmooth::<10>::new(),
            timer_frame:  TimerSmooth::<10>::new(),
        }
    }

    pub fn init(&mut self) {

    }

    pub fn update(&mut self) {
        if !self.has_init {
            self.has_init = true;
            self.world_init();
        }


        self.accum_update.accumulate();
        //println!("{}", self.accum_update.amount().0);
        let mut should_render = true;
        if self.accum_update.has_accumulated() {

            self.engine_update(self.accum_update.dt_fixed());

            self.accum_update.consume();
            if self.accum_update.has_accumulated() && !self.should_redraw {
                should_render = false;
            }
            self.timer_update.end();
            self.timer_update.start();
        }

        if should_render {
            self.should_redraw = true;
            self.frame_update();
        }

        if self.request_close {
            self.request_close = false;
            // TODO check if engine is allowed to close
            self.should_close = true;
        }

    }

    pub fn render(&mut self) {

        self.should_redraw = false;
        // TODO draw frame

        self.timer_frame.end();
        self.timer_frame.start();
    }

    pub fn should_redraw(&mut self) -> bool {
        self.should_redraw
    }

    pub fn request_close(&mut self) {
        self.request_close = true;
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn update_timer(&self) -> TimerSmooth<10> {
        return self.timer_update;
    }

    pub fn frame_timer(&self) -> TimerSmooth<10> {
        return self.timer_frame;
    }
}

impl Engine {


    fn world_init(&mut self) {

    }

    fn engine_update(&mut self, dt: Time) {

    }

    fn frame_update(&mut self) {

    }

}