/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

pub trait EventSystem<Event> {
    fn broadcast(&mut self, event: Event);
    fn interrupt(&mut self, event: Event);
    fn enqueue(&mut self, event: Event);
    fn process(&mut self, router: &mut impl FnMut(&mut Self, &Event));
    fn len(&self) -> usize;
    fn is_processing(&self) -> bool;
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