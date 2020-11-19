/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch::interop::{WindowController, WindowTitleController};
use butterscotch::chrono::{Accumulator, TimerSmooth};


const SAMPLE_WINDOW: usize = 10;

pub struct Engine {
    accum_update: Accumulator,

    request_close: bool,

    timer_update: TimerSmooth<{SAMPLE_WINDOW}>,
    timer_frame:  TimerSmooth<{SAMPLE_WINDOW}>,
}

impl Engine {

    pub fn init(&mut self, _window: &dyn WindowController) {
        self.world_init();
    }

    pub fn update(&mut self, window: &dyn WindowController) {
        self.accum_update.accumulate();

        let mut should_render = true;
        if self.accum_update.has_accumulated() {

            self.engine_update(self.accum_update.dt_fixed());

            self.timer_update.end_start();

            self.accum_update.consume();
            should_render = !self.accum_update.has_accumulated();
        }

        if should_render {
            window.request_redraw();
            self.frame_update();
        }

        // TODO check if engine is allowed to close
        // if self.request_close {
        //     self.request_close = false;
        //     window.prevent_close();
        // }
    }

    pub fn render(&mut self, _window: &dyn WindowController) {
        // TODO draw frame

        self.timer_frame.end_start();
    }

    pub fn quit(&mut self) {

    }

    pub fn close(&mut self, _window: &dyn WindowController) {
        self.request_close = true;
    }

    pub fn update_title(&mut self, window: &dyn WindowTitleController) {
        window.set_title(&format!(
            "fps: {}, tps: {}",
            self.timer_frame.tps_average().round(),
            self.timer_update.tps_average().round()
        ));
    }
}

impl Engine {

    pub fn new() -> Engine {
        Engine{
            accum_update: Accumulator::new(1.0/60.0, 10),
            request_close: false,
            timer_update: TimerSmooth::new(),
            timer_frame:  TimerSmooth::new(),
        }
    }

    fn world_init(&mut self) {

    }

    fn engine_update(&mut self, _dt: f64) {

    }

    fn frame_update(&mut self) {

    }
}