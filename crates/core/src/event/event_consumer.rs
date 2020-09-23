/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::RefCell, rc::Rc};

use super::{Event, EventQueue, OffsetInt};

#[derive(Debug)]
pub struct EventConsumer {
    queue: Rc<RefCell<EventQueue>>,
    pos_read: OffsetInt
}

impl EventConsumer {
    pub(super) fn new(queue: &Rc<RefCell<EventQueue>>) -> EventConsumer {
        EventConsumer{
            queue: queue.clone(),
            pos_read: queue.borrow_mut().listen()
        }
    }

    pub fn remaining(&self) -> usize {
        (self.queue.borrow().count_offset() - self.pos_read) as usize
    }

    pub fn empty(&self) -> bool {
        self.queue.borrow().count_offset() == self.pos_read
    }
}

impl Iterator for EventConsumer {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.borrow_mut().consume(self.pos_read) {
            Ok(v) => {
                self.pos_read += 1;
                Some(v)
            },
            Err(id) => {
                debug_assert_eq!(self.pos_read, id, "EventConsumer out of sync: pos_read != id");
                self.pos_read = id;
                None
            }
        }
    }
}
