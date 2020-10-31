/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_common::{container::DoubleBuffer, interop, unlikely};

#[derive(Debug)]
pub struct EventSystem<Event> {
    processing: bool,
    broadcasts: DoubleBuffer<Event>,
    interrupts: DoubleBuffer<Event>,
}

impl<Event> Default for EventSystem<Event> {
    fn default() -> Self { Self::new() }
}

impl<Event> EventSystem<Event> {
    pub fn new() -> Self {
        Self{
            processing: false,
            broadcasts: DoubleBuffer::default(),
            interrupts: DoubleBuffer::default(),
        }
    }
}

impl<Event> interop::EventSystem<Event> for EventSystem<Event> {
    fn broadcast(&mut self, event: Event) {
        self.broadcasts.push(event);
    }

    fn interrupt(&mut self, event: Event) {
        if unlikely!(!self.processing) { panic!("Cannot interrupt when there are no events being processed."); }
        self.interrupts.push(event);
    }

    fn enqueue(&mut self, event: Event) {
        match self.is_processing() {
            true  => self.interrupt(event),
            false => self.broadcast(event),
        }
    }

    fn process(&mut self, router: &mut impl FnMut(&mut Self, &Event)) {
        // Reentrancy disallowed
        if unlikely!(std::mem::replace(&mut self.processing, true)) {
            panic!("Cannot process events whilst already processing events.");
        }

        let broadcasts = self.broadcasts.expect_take();
        for event in &broadcasts {
            // Process event
            router(self, event);

            // Process interrupts
            if self.interrupts.len() <= 0 { continue; }

            let mut interrupts = self.interrupts.expect_take();
            loop {
                for event in &interrupts { router(self, event); }
                self.interrupts.swap(&mut interrupts);
                if interrupts.len() <= 0 { break; }
            }
            self.interrupts.replace(interrupts);
        }
        self.broadcasts.replace(broadcasts);

        // Allow calling the function again
        self.processing = false;
    }

    fn len(&self) -> usize {
        self.broadcasts.len()
    }

    fn is_processing(&self) -> bool {
        self.processing
    }
}