/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::future::Future;

pub trait EventSystem<Event> {
    fn broadcast_async(&self, task: impl Future<Output = Option<Event>> + 'static);
    fn broadcast(&self, event: Event);
    fn interrupt(&self, event: Event);
    fn enqueue(&self, event: Event);
    fn process(&self, router: &mut impl FnMut(&Self, &Event));
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
pub struct RequestTracker(core::cell::Cell<bool>);

impl RequestTracker {
    pub fn reject(&self) { self.0.replace(true); }
    pub fn was_rejected(self) -> bool { self.0.get() }
}

impl Default for RequestTracker {
    fn default() -> Self { Self(false.into()) }
}

impl Into<bool> for RequestTracker {
    fn into(self) -> bool { self.0.get() }
}

impl PartialEq<bool> for RequestTracker {
    fn eq(&self, other: &bool) -> bool {
        self.0.get() == *other
    }
}