/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::RefCell, rc::Rc};

use butterscotch::{
    event::EventSystem, 
    window::{
        WindowController
    }, 
    render::{
        Renderer,
        create_debug_pipeline
    },
    chrono::{
        Accumulator, 
        TimerSmooth
    },
    dpi::PixelsRaw, 
    interop::WindowHandle
};

use super::GameEvent;

const SAMPLE_WINDOW: usize = 10;

pub struct Engine {
    accum_update: Accumulator,

    request_close: bool,

    timer_update: TimerSmooth<{SAMPLE_WINDOW}>,
    timer_frame:  TimerSmooth<{SAMPLE_WINDOW}>,

    event_system: Rc<EventSystem<GameEvent>>,
    renderer: Rc<RefCell<Option<Renderer>>>,
    
    pipeline: Option<wgpu::RenderPipeline>
}

impl Engine {

    pub fn open(&mut self, window: &WindowController) {
        self.event_system.broadcast_async(create_renderer(self.renderer.clone(), window.get_window_handle(), window.get_size_raw()));
        self.world_init();
    }

    pub fn update(&mut self, window: &WindowController) {
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

    pub fn render(&mut self, window: &WindowController) {
        self.timer_frame.end_start();
        let mut borrow = self.renderer.borrow_mut();
        let renderer = match borrow.as_mut() { Some(v) => v, _ => { println!("No renderer"); return;} };

        if self.pipeline.is_none() {
            self.pipeline = Some(create_debug_pipeline(&renderer));
        }
        
        renderer.render(window.get_size_raw(), self.pipeline.as_ref().unwrap());
    }

    pub fn resize(&mut self, window: &WindowController) {
        let mut borrow = self.renderer.borrow_mut();
        let renderer = match borrow.as_mut() { Some(v) => v, _ => return };

        renderer.resize(window.get_size_raw());
    }

    pub fn quit(&mut self, _window: &WindowController) {

    }

    pub fn close(&mut self, _window: &WindowController) {
        self.request_close = true;
    }

    pub fn  update_title(&mut self, window: &WindowController) {
        window.set_title(&format!(
            "fps: {}, tps: {}",
            self.timer_frame.tps_average().round(),
            self.timer_update.tps_average().round()
        ));
    }
}

impl Engine {

    pub fn new(event_system: Rc<EventSystem<GameEvent>>) -> Engine {
        Engine{
            accum_update: Accumulator::new(1.0/60.0, 10),
            request_close: false,
            timer_update: TimerSmooth::new(),
            timer_frame:  TimerSmooth::new(),
            renderer: Rc::new(RefCell::new(None)),
            event_system,
            pipeline: None,
        }
    }

    fn world_init(&mut self) {

    }

    fn engine_update(&mut self, _dt: f64) {

    }

    fn frame_update(&mut self) {

    }
}

async fn create_renderer(dest: Rc<RefCell<Option<Renderer>>>, window_handle: WindowHandle, window_size: PixelsRaw) -> Option<GameEvent> {
    let renderer = Some(Renderer::new(window_handle, window_size).await);
    *dest.borrow_mut() = renderer;
    return None;
}

