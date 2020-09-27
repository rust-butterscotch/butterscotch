/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::RefCell, rc::Rc};

use super::{Consumer, Queue};

#[derive(Debug, Clone)]
pub  struct Publisher<T: Clone> {
    queue: Rc<RefCell<Queue<T>>>,
}

impl<T: Clone> Publisher<T> {
    pub fn new() -> Publisher<T> {
        Publisher{
            queue: Rc::new(RefCell::new(Queue::new()))
        }
    }

    pub fn publish(&mut self, ev: T) {
        self.queue.borrow_mut().publish(ev)
    }

    pub fn listen(&self) -> Consumer<T> {
        Consumer::new(&self.queue)
    }

    pub fn count(&self) -> usize {
        self.queue.borrow().count() as usize
    }
}
