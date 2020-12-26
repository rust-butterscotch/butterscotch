/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::WindowController;

pub enum WindowEvent<'a> {
    Open  (&'a WindowController),
    Update(&'a WindowController),
    Redraw(&'a WindowController),
    Cleanup(&'a WindowController),
    Resize(&'a WindowController),
    Close (&'a WindowController),
    Quit  (&'a WindowController),
    Title (&'a WindowController)
}