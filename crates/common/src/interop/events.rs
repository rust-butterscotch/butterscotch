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
