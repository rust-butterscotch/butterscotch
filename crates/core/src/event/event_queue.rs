/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::collections::VecDeque;

use super::{Event, OffsetInt};

#[derive(Debug)]
pub struct EventQueue {
    off_read: OffsetInt,
    events: VecDeque<Event>,
    counters: VecDeque<u64>,
}

impl EventQueue {

    pub(super) fn new() -> EventQueue {
        EventQueue{
            off_read: 0,
            events: VecDeque::<Event>::new(),
             // counters always has events.len + 1 to hold a
             // count on consumers that are up-to-date
            counters: [0].iter().copied().collect()
        }
    }

    pub(super) fn publish(&mut self, ev: Event) {
        self.events.push_back(ev);
        self.counters.push_front(0);
    }

    pub(super) fn consume(&mut self, id: OffsetInt) -> Result<Event, OffsetInt> {
        if id < self.off_read { return Err(self.off_read); }

        debug_assert!(
            id - self.off_read < usize::MAX as OffsetInt,
            "usize overflow. Should only occur if we fail to track off_read"
        );

        let index = (id - self.off_read) as usize;
        if index >= self.events.len() {
            return Err(self.off_read);
        }

        let result = Ok(self.events[index]);
        self.counters[index    ] -= 1;
        self.counters[index + 1] += 1;
        if self.counters[index] == 0 {
            self.events.pop_front();
            self.counters.pop_front();
        }

        return result;
    }

    pub(super) fn listen(&mut self) -> OffsetInt {
        *self.counters.back_mut().unwrap() += 1;
        self.off_read + (self.events.len() as OffsetInt)
    }

    pub(super) fn count_offset(&self) -> OffsetInt {
        self.off_read + (self.events.len() as OffsetInt)
    }

    pub(super) fn count(&self) -> OffsetInt {
        self.events.len() as OffsetInt
    }
}

// OffsetInt must be able to hold usize values
const_assert!(std::mem::size_of::<OffsetInt>() >= std::mem::size_of::<usize>());