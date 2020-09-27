/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::RefCell, rc::Rc};

use super::{Queue, OffsetInt};

#[derive(Debug)]
pub struct Consumer<T: Clone> {
    queue: Rc<RefCell<Queue<T>>>,
    pos_read: OffsetInt
}

impl<T: Clone> Consumer<T> {
    pub(super) fn new(queue: &Rc<RefCell<Queue<T>>>) -> Consumer<T> {
        Consumer{
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

impl<T: Clone> Iterator for &mut Consumer<T> {
    type Item = T;
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

impl<T: Clone> Drop for Consumer<T> {
    fn drop(&mut self) {
        self.queue.borrow_mut().unlisten(self.pos_read);
    }
}