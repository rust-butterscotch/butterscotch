/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::util::TinyString;

use super::Event;

pub struct EventSimple<T: ?Sized> {
    id: &'static TinyString,
    value: T,
}

impl<T> EventSimple<T> {
    pub fn new(id: &'static TinyString, value: T) -> Self {
        Self{id, value}
    }
}

impl<T: ?Sized> EventSimple<T> {
    pub fn get_ref(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Event for EventSimple<T> {
    fn id(&self) -> TinyString {
        return *self.id;
    }
}