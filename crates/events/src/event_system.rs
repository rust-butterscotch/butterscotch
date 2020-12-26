/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::{Cell, RefCell}, future::Future};
use butterscotch_common::{container::DoubleBuffer, future::IncrementalLocalExecutor, interop, unlikely};

pub struct EventSystem<Event> {
    processing: Cell<bool>,
    broadcasts: RefCell<DoubleBuffer<Event>>,
    interrupts: RefCell<DoubleBuffer<Event>>,
    spawned: IncrementalLocalExecutor<Option<Event>>,
}

impl<Event> Default for EventSystem<Event> {
    fn default() -> Self { Self::new() }
}

impl<Event> EventSystem<Event> {
    pub fn new() -> Self {
        Self{
            processing: Cell::new(false),
            broadcasts: RefCell::new(DoubleBuffer::default()),
            interrupts: RefCell::new(DoubleBuffer::default()),
            spawned:    IncrementalLocalExecutor::new(100),
        }
    }
}

impl<Event> EventSystem<Event> {
    fn is_processing(&self) -> bool {
        self.processing.get()
    }
}

impl<Event> interop::EventSystem<Event> for EventSystem<Event> {

    fn broadcast_async(&self, task: impl Future<Output = Option<Event>> + 'static) {
        self.spawned.spawn(task, false);
    }

    fn broadcast(&self, event: Event) {
        self.broadcasts.borrow_mut().push(event);
    }

    fn interrupt(&self, event: Event) {
        if unlikely!(!self.is_processing()) { panic!("Cannot interrupt when there are no events being processed."); }
        self.interrupts.borrow_mut().push(event);
    }

    fn enqueue(&self, event: Event) {
        match self.is_processing() {
            true  => self.interrupt(event),
            false => self.broadcast(event),
        }
    }

    fn process(&self, router: &mut impl FnMut(&Self, &Event)) {
        // Reentrancy disallowed
        if unlikely!(self.processing.replace(true)) {
            panic!("Cannot process events whilst already processing events.");
        }

        self.spawned.run_cb(&mut |event| match event {
            Some(e) => self.broadcast(e),
            None    => {}
        });

        let broadcasts = self.broadcasts.borrow_mut().expect_take();
        for event in &broadcasts {
            // Process event
            router(self, event);

            // Process interrupts
            if self.interrupts.borrow_mut().len() <= 0 { continue; }
            let mut interrupts = self.interrupts.borrow_mut().expect_take();
            loop {
                for event in &interrupts { router(self, event); }
                self.interrupts.borrow_mut().swap(&mut interrupts);
                if interrupts.len() <= 0 { break; }
            }
            self.interrupts.borrow_mut().replace(interrupts);
        }
        self.broadcasts.borrow_mut().replace(broadcasts);

        // Allow calling the function again
        self.processing.set(false);
    }
}