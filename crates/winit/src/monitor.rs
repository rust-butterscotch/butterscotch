/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use winit::monitor::MonitorHandle;

pub struct Monitor {
    pub(crate) handle: Option<MonitorHandle>
}

impl Monitor {

    pub fn name(&self) -> String {
        match self.handle.as_ref() {
            Some(v) => v.name(),
            None    => None
        }.unwrap_or_else(|| "Unknown Monitor".to_owned())
    }

}