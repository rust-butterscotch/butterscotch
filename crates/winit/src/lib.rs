/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_common::{interop, chrono::{Timer}};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;



pub struct WindowController {
    pub title: Option<String>,
    pub close: bool,
    pub redraw: bool,
}

impl Default for WindowController {
    fn default() -> Self {
        Self{
            title: None,
            close: false,
            redraw: false,
        }
    }
}

pub trait WindowEventLoopController {
    fn init  (&mut self, window: &mut WindowController);
    fn render(&mut self, window: &mut WindowController);
    fn update(&mut self, window: &mut WindowController);
    fn quit  (&mut self);
    fn close (&mut self, window: &mut WindowController);

    #[cfg(target_arch = "wasm32")]
    fn get_wasm_web_canvas(&self) -> Option<HtmlCanvasElement>;
}

pub fn run_event_loop<EventLoopController: 'static + WindowEventLoopController>(mut controller: EventLoopController) -> ! {

    let event_loop = EventLoop::new();
    let window = {
        #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowBuilderExtWebSys;
            WindowBuilder::new().with_canvas(controller.get_wasm_web_canvas()).build(&event_loop).unwrap()
        }
        #[cfg(not(target_arch = "wasm32"))] {
            WindowBuilder::new().build(&event_loop).unwrap()
        }
    };


    let mut init = false;
    let mut title_timer = Timer::new();
    let mut title_pending: Option<String> = None;
    let     title_min_time = 1.0/60.0;

    event_loop.run(move |event, _, control_flow| {
        let mut window_controller = WindowController::default();

        if !init {
            init = true;
            controller.init(&mut window_controller);
        }

        match event {
            Event::MainEventsCleared  => {
                controller.update(&mut window_controller);
            },
            Event::RedrawRequested(_) => {
                controller.render(&mut window_controller);
            },
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                controller.close(&mut window_controller);
            },
            _ => ()
        }

        /* Limit title sets, can cause window system to hang if unlimited */
        if let Some(title) = window_controller.title { title_pending = Some(title); }
        if let Some(title) = &title_pending {
            title_timer.mark();
            if title_timer.total() > title_min_time {
                window.set_title(&title);
                title_pending = None;
                title_timer.reset();
            }
        }

        if window_controller.redraw {
            window.request_redraw();
        }

        if window_controller.close {
            *control_flow = ControlFlow::Exit;
            controller.quit();
        } else {
            *control_flow = ControlFlow::Poll;
        };
    });
}
