/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::any::TypeId;

use butterscotch_common::container::{GID};

pub trait System<T> {
    fn notify(&mut self, event: &T);
    fn entity_destroyed(&mut self, id: GID);
    fn type_id(&self) -> TypeId;
}