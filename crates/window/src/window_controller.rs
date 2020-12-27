/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::cell::Cell;
use butterscotch_common::{dpi::{Pixels, PixelsRaw, PixelsUser}, interop::WindowHandle, math::Vec2};
use raw_window_handle::HasRawWindowHandle;
use winit::window::{Fullscreen, Window};

use crate::monitor::Monitor;

pub struct WindowController {
    pub(crate) window: Window,
    pub(crate) close_requested: Cell<bool>,
    pub(crate) redraw_requested: Cell<bool>
}

pub enum FullscreenMode {
    Monitor(Monitor),
    CurrentMonitor,
    Window
}

impl WindowController {
    pub fn new(window:  Window) -> Self {
        Self{
            window,
            close_requested: false.into(),
            redraw_requested: false.into()
        }
    }

    pub fn request_redraw(&self) {
        self.redraw_requested.set(true);
    }

    pub fn request_close(&self) {
        self.close_requested.set(true);
    }

    pub fn prevent_close(&self) {
        self.close_requested.set(true);
    }

    pub fn set_size(&self, size: Pixels) {
        match size {
            Pixels::User(v) => self.window.set_inner_size(winit::dpi::LogicalSize {width: v.0.x, height: v.0.y}),
            Pixels::Raw (v) => self.window.set_inner_size(winit::dpi::PhysicalSize{width: v.0.x, height: v.0.y}),
        }
    }

    pub fn set_fullscreen(&self, fullscreen: FullscreenMode) {
        self.window.set_fullscreen(match fullscreen {
            FullscreenMode::Monitor(v)     => Some(Fullscreen::Borderless(v.handle)),
            FullscreenMode::CurrentMonitor => Some(Fullscreen::Borderless(None)),
            FullscreenMode::Window         => None
        })
    }

    pub fn get_fullscreen(&self) -> Option<Monitor> {
        match self.window.fullscreen() {
            Some(Fullscreen::Borderless(m)) => Some(Monitor{handle: m}),
            Some(Fullscreen::Exclusive(_)) => Some(self.get_monitor_current()),
            None => None,
        }
    }

    pub fn get_monitor_avaliable(&self) -> impl Iterator<Item = Monitor> {
        self.window.available_monitors().map(|v| Monitor{handle: Some(v)})
    }

    pub fn get_monitor_primary(&self) -> Option<Monitor> {
        match self.window.primary_monitor() {
            Some(v) => Some(Monitor{handle: Some(v)}),
            None    => None
        }
    }

    pub fn get_monitor_current(&self) -> Monitor {
        Monitor{handle: self.window.current_monitor()}
    }

    pub fn get_size_raw(&self) -> PixelsRaw {
        let size = self.window.inner_size();
        PixelsRaw(Vec2::new(size.width as f64, size.height as f64))
    }
    
    pub fn get_size_user(&self) -> PixelsUser {
        self.get_size_raw().to_user_scale(self.get_scale_factor())
    }

    pub fn get_scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn get_window_handle(&self) -> WindowHandle {
        WindowHandle::new(self.window.raw_window_handle())
    }
    
    pub fn set_title(&self, str: &str) {
        self.window.set_title(str);
    }
}