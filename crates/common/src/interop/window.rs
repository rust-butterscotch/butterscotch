/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

pub struct WindowHandle {
    handle: raw_window_handle::RawWindowHandle
}

impl WindowHandle {
    pub fn new(handle: raw_window_handle::RawWindowHandle) -> WindowHandle {
        WindowHandle{handle}
    }
}

unsafe impl raw_window_handle::HasRawWindowHandle for WindowHandle {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.handle
    }
}