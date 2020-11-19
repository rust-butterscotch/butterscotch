/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::rc::Rc;

use crate::dpi::Pixels;

pub trait WindowController : raw_window_handle::HasRawWindowHandle {
    fn request_redraw(&self);

    fn request_close(&self);
    fn prevent_close(&self);

    fn set_size(&self, size: Pixels);
    fn get_size_physical(&self) -> Pixels;
    fn get_size_logical (&self) -> Pixels;

    fn get_scale_factor(&self) -> f64;
}

pub trait WindowTitleController : WindowController {
    fn set_title(&self, str: &str);
}

pub enum WindowEvent {
    Init  (Rc<dyn WindowController>),
    Update(Rc<dyn WindowController>),
    Redraw(Rc<dyn WindowController>),
    Close (Rc<dyn WindowController>),
    Quit,

    TitleSync(Rc<dyn WindowTitleController>)
}