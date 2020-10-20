/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;
use std::collections::VecDeque;
use butterscotch_common::unlikely;

#[derive(Debug)]
pub struct EventSystem<Event> {
    processing: bool,
    broadcasts: VecDeque<Event>,
    interrupts: LoanVec<Event>,
    interrupts_buffer: LoanVec<Event>,
}

impl<Event> Default for EventSystem<Event> {
    fn default() -> Self { Self::new() }
}

impl<Event> EventSystem<Event> {

    pub fn new() -> Self {
        Self{
            processing: false,
            broadcasts: VecDeque::default(),
            interrupts:        LoanVec::default(),
            interrupts_buffer: LoanVec::default(),
        }
    }

    pub fn broadcast(&mut self, event: Event) {
        self.broadcasts.push_back(event);
    }

    pub fn interrupt(&mut self, event: Event) {
        if unlikely!(!self.processing) { return; }
        self.interrupts.as_mut().push(event);
    }

    pub fn process(&mut self, router: &mut impl FnMut(&mut EventSystem<Event>, &Event)) {
        // Reentrancy disallowed
        if unlikely!(self.processing) { panic!("Cannot process events whilst already processing events."); }
        self.processing = true;

        // Take the cached buffer and move it onto the stack,
        // then update the stored capacity if its unallocated
        let mut interrupts_processing = self.interrupts_buffer.take();
        self.interrupts.try_store_capacity_into(&mut interrupts_processing);

        for _ in 0..self.broadcasts.len() {
            // Process event
            let event = self.broadcasts.pop_front().unwrap();
            router(self, &event);

            // Process interrupts
            loop {
                if self.interrupts.is_borrowed() { break; }
                if self.interrupts.len() <= 0    { break; }

                // We move the interrupts onto the stack and mark the stack version as empty,
                // noting the capacity so that we can reallocate it with an optimal amount if
                // we require nested interrupts but without making memory allocations if we
                // don't by moving the interrupts back into the struct if nothing was allocated.
                // We only require a maximum of two allocations, since we can swap back and forth
                self.interrupts.swap(&mut interrupts_processing);
                for event in interrupts_processing.as_mut() { router(self, event); }
                interrupts_processing.clear();
            }
        }

        // First preference is to return the buffer to the interrupts
        if self.interrupts.is_borrowed() {
            self.interrupts.swap(&mut interrupts_processing);
        }

        // Second preference is to return the buffer to the buffer store
        self.interrupts_buffer.swap(&mut interrupts_processing);

        // Allow calling the function again
        self.processing = false;
    }

    pub fn is_processing(&self) -> bool {
        self.processing
    }
}