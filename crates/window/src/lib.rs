/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(box_syntax)]
#![feature(core_intrinsics)]

mod monitor;
mod window_controller;
mod event;

pub use monitor::*;
pub use window_controller::*;
pub use event::*;

use winit::{event::Event, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use butterscotch_common::chrono::Timer;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[derive(Default)]
pub struct WindowSettings {
    pub title: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub canvas: Option<HtmlCanvasElement>,
}

impl WindowSettings {
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
        self.canvas = Some(canvas);
        self
    }
}

pub fn open_window<F: 'static + FnMut(WindowEvent)>(settings: WindowSettings, mut handler: F) -> ! {
    let event_loop = EventLoop::new();
    let controller = WindowController::new({cfg_if::cfg_if!{
        if #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowBuilderExtWebSys;
            WindowBuilder::new()
                .with_title(settings.title.unwrap_or_else(|| "Butterscotch".to_owned()))
                .with_canvas(settings.canvas) // TODO make compile error
            .build(&event_loop).unwrap()
        } else {
            WindowBuilder::new()
                .with_title(settings.title.unwrap_or_else(|| "Butterscotch".to_owned()))
            .build(&event_loop).unwrap()
        }
    }});

    let mut title_timer = Timer::new();
    let     title_min_time = 2.0/60.0;

    handler(WindowEvent::Open(&controller));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared  => {
                handler(WindowEvent::Update(&controller));
                if controller.redraw_requested.get() {
                    controller.redraw_requested.set(false);
                    controller.window.request_redraw();
                }
            },
            Event::RedrawRequested(_) => {
                handler(WindowEvent::Redraw(&controller));
            },
            Event::RedrawEventsCleared => {
                handler(WindowEvent::Cleanup(&controller));
            },
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::Resized(_),
                ..
            } => {
                handler(WindowEvent::Resize(&controller));
            },
            Event::WindowEvent {event: winit::event::WindowEvent::CloseRequested, ..} => {
                controller.close_requested.set(true);
            },
            // TODO resize/dpi
            _ => ()
        }

        if controller.close_requested.get() {
            handler(WindowEvent::Close(&controller));

            if controller.close_requested.get() {
                controller.close_requested.set(false);
                *control_flow = ControlFlow::Exit;
                handler(WindowEvent::Quit(&controller));
            }
        }

        title_timer.mark();
        if title_timer.total() > title_min_time {
            handler(WindowEvent::Title(&controller));
            title_timer.reset();
        }
    });
}
