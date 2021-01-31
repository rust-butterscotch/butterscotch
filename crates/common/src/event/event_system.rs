/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::{Cell, RefCell}};
use crate::{container::DoubleBuffer, unlikely, util::UnionRef};

use super::Event;

pub struct EventSystem {
    processing: Cell<bool>,
    broadcasts: RefCell<DoubleBuffer<UnionRef<dyn Event>>>,
    interrupts: RefCell<DoubleBuffer<UnionRef<dyn Event>>>,
}

impl Default for EventSystem {
    fn default() -> Self { Self::new() }
}

impl EventSystem {
    pub fn new() -> Self {
        Self{
            processing: Cell::new(false),
            broadcasts: Default::default(),
            interrupts: Default::default(),
        }
    }
    
    pub fn is_processing(&self) -> bool {
        self.processing.get()
    }

    pub fn broadcast(&self, event: UnionRef<dyn Event>) {
        self.broadcasts.borrow_mut().push(event);
    }

    pub fn interrupt(&self, event: UnionRef<dyn Event>) {
        if unlikely!(!self.is_processing()) { panic!("Cannot interrupt when there are no events being processed."); }
        self.interrupts.borrow_mut().push(event);
    }

    pub fn enqueue(&self, event: UnionRef<dyn Event>) {
        match self.is_processing() {
            true  => self.interrupt(event),
            false => self.broadcast(event),
        }
    }

    pub fn process(&self, router: &mut impl FnMut(&Self, &UnionRef<dyn Event>)) {
        // Reentrancy disallowed
        if unlikely!(self.processing.replace(true)) {
            panic!("Cannot process events whilst already processing events.");
        }

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