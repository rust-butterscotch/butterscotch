/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(box_syntax)]

use std::{cell::Cell, rc::Rc};
use winit::{window::Window, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

use butterscotch_common::{chrono::{Timer}, dpi::Pixels, interop::{EventSystem, WindowController as EngineWindowController, WindowEvent as EngineWindowEvent, WindowTitleController as EngineWindowTitleController}, math::Vec2};

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;


pub struct WindowController {
    window: Window,
    close_requested: Cell<bool>,
    redraw_requested: Cell<bool>
}

impl WindowController {
    pub fn new(window:  Window) -> Self {
        Self{
            window,
            close_requested: false.into(),
            redraw_requested: false.into()
        }
    }
}

impl EngineWindowController for WindowController {
    fn request_redraw(&self) {
        self.redraw_requested.set(true);
    }

    fn request_close(&self) {
        self.close_requested.set(true);
    }

    fn prevent_close(&self) {
        self.close_requested.set(true);
    }

    fn set_size(&self, size: Pixels) {
        match size {
            Pixels::Logical (v) => self.window.set_inner_size(winit::dpi::LogicalSize {width:v.x, height:v.y}),
            Pixels::Physical(v) => self.window.set_inner_size(winit::dpi::PhysicalSize{width:v.x, height:v.y}),
        }
    }

    fn get_size_physical(&self) -> Pixels {
        let size = self.window.inner_size();
        Pixels::Physical(Vec2::new(size.width as f64, size.height as f64))
    }

    fn get_size_logical(&self) -> Pixels {
        self.get_size_physical().to_logical(self.get_scale_factor())
    }

    fn get_scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }
}

unsafe impl raw_window_handle::HasRawWindowHandle for WindowController {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl EngineWindowTitleController for WindowController {
    fn set_title(&self, str: &str) {
        self.window.set_title(str);
    }
}

pub trait WindowEventLoopController {
    fn init  (&mut self, window: &mut WindowController);
    fn render(&mut self, window: &mut WindowController);
    fn update(&mut self, window: &mut WindowController);
    fn quit  (&mut self);
    fn close (&mut self, window: &mut WindowController);
}

#[derive(Debug, Default)]
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

pub fn run_event_loop<EngineEvent: From<EngineWindowEvent>, EngineEventSystem: 'static + EventSystem<EngineEvent>, EngineEventHandler: 'static + FnMut(&mut EngineEventSystem, &EngineEvent)>(settings: WindowSettings, mut event_system: EngineEventSystem, mut router: EngineEventHandler) -> ! {
    let event_loop = EventLoop::new();
    let controller = Rc::new(WindowController::new({cfg_if::cfg_if!{
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
    }}));

    let mut init = false;
    let mut title_timer = Timer::new();
    let     title_min_time = 2.0/60.0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if !init {
            init = true;
            event_system.enqueue(EngineEvent::from(EngineWindowEvent::Init(controller.clone())));
            event_system.process(&mut router);
        }

        match event {
            Event::MainEventsCleared  => {
                event_system.enqueue(EngineEvent::from(EngineWindowEvent::Update(controller.clone())));
                event_system.process(&mut router);
                if controller.redraw_requested.get() {
                    controller.redraw_requested.set(false);
                    controller.window.request_redraw();
                }
            },
            Event::RedrawRequested(_) => {
                event_system.enqueue(EngineEvent::from(EngineWindowEvent::Redraw(controller.clone())));
                event_system.process(&mut router);
            },
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                controller.close_requested.set(true);
            },
            // TODO resize/dpi
            _ => ()
        }

        if controller.close_requested.get() {
            event_system.enqueue(EngineEvent::from(EngineWindowEvent::Close(controller.clone())));
            event_system.process(&mut router);

            if controller.close_requested.get() {
                controller.close_requested.set(false);
                *control_flow = ControlFlow::Exit;
                event_system.enqueue(EngineEvent::from(EngineWindowEvent::Quit));
                event_system.process(&mut router);
            }
        }

        title_timer.mark();
        if title_timer.total() > title_min_time {
            event_system.enqueue(EngineEvent::from(EngineWindowEvent::TitleSync(controller.clone())));
            event_system.process(&mut router);
            title_timer.reset();
        }
    });
}
