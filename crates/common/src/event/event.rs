/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::util::TinyString;

pub trait Event {
    fn id(&self) -> TinyString;
}
