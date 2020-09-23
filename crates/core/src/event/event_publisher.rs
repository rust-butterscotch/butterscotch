/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::RefCell, rc::Rc};

use super::{Event, EventConsumer, EventQueue};

#[derive(Debug, Clone)]
pub  struct EventPublisher {
    queue: Rc<RefCell<EventQueue>>,
}

impl EventPublisher {
    pub fn new() -> EventPublisher {
        EventPublisher{
            queue: Rc::new(RefCell::new(EventQueue::new()))
        }
    }

    pub fn publish(&mut self, ev: Event) {
        self.queue.borrow_mut().publish(ev)
    }

    pub fn listen(&self) -> EventConsumer {
        EventConsumer::new(&self.queue)
    }

    pub fn count(&self) -> usize {
        self.queue.borrow().count() as usize
    }
}
