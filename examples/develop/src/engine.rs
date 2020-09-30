/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch::{WindowController, WindowEventLoopController, chrono::{Accumulator, TimerSmooth}};

const SAMPLE_WINDOW: usize = 10;

pub struct Engine {
    accum_update: Accumulator,

    request_close: bool,

    timer_update: TimerSmooth<{SAMPLE_WINDOW}>,
    timer_frame:  TimerSmooth<{SAMPLE_WINDOW}>,
}

impl WindowEventLoopController for Engine {

    fn init(&mut self, _window: &mut WindowController) {
        self.world_init();
    }

    fn update(&mut self, window: &mut WindowController) {
        self.accum_update.accumulate();

        let mut should_render = true;
        if self.accum_update.has_accumulated() {

            self.engine_update(self.accum_update.dt_fixed());

            self.timer_update.end_start();

            self.accum_update.consume();
            should_render = !self.accum_update.has_accumulated();

            window.set_title(&format!(
                "fps: {}, tps: {}",
                self.timer_frame.tps_average().round(),
                self.timer_update.tps_average().round()
            ));
        }

        if should_render {
            window.mark_redraw();
            self.frame_update();
        }

        if self.request_close {
            self.request_close = false;

            // TODO check if engine is allowed to close

            window.mark_close();
        }
    }

    fn render(&mut self, _window: &mut WindowController) {
        // TODO draw frame

        self.timer_frame.end_start();
    }

    fn quit(&mut self) {

    }

    fn close(&mut self, _window: &mut WindowController) {
        self.request_close = true;
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