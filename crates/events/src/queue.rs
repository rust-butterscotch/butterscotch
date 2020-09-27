/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::collections::VecDeque;

pub(crate) type OffsetInt = u64; // 500_000_000_000 events/second for 1 year to overflow

#[derive(Debug)]
pub(crate) struct Queue<T: Clone> {
    off_read: OffsetInt,
    events: VecDeque<T>,
    counters: VecDeque<u64>,
}

impl<T: Clone> Queue<T> {

    pub(crate) fn new() -> Queue<T> {
        Queue{
            off_read: 0,
            events: VecDeque::<T>::new(),
            counters: [0].iter().copied().collect(), // counters always has events.len + 1, for up-to-date consumers
        }
    }

    pub(crate) fn publish(&mut self, ev: T) {
        self.events.push_back(ev);
        self.counters.push_front(0);
    }

    pub(crate) fn consume(&mut self, id: OffsetInt) -> Result<T, OffsetInt> {
        if id < self.off_read { return Err(self.off_read); }

        debug_assert!(
            id - self.off_read < usize::MAX as OffsetInt,
            "usize overflow. Should only occur if we fail to track off_read"
        );

        let index = (id - self.off_read) as usize;
        if index >= self.events.len() {
            return Err(self.off_read);
        }

        let result = Ok(self.events[index].clone());
        self.counters[index + 1] += 1;
        self.counters[index    ] -= 1;
        if index == 0 && self.counters.len() > 0 && self.counters[index] == 0 {
            self.events.pop_front();
            self.counters.pop_front();
        }

        return result;
    }

    pub(crate) fn listen(&mut self) -> OffsetInt {
        *self.counters.back_mut().unwrap() += 1;
        self.off_read + (self.events.len() as OffsetInt)
    }

    pub(crate) fn unlisten(&mut self, id: OffsetInt) {
        if id < self.off_read { return; }

        debug_assert!(
            id - self.off_read < usize::MAX as OffsetInt,
            "usize overflow. Should only occur if we fail to track off_read"
        );

        let index = (id - self.off_read) as usize;
        if index >= self.events.len() { return; }

        self.counters[index] -= 1;
        if index == 0 {
            loop {
                if self.counters[index] == 0 {
                    self.counters.pop_front();
                }
            }
        }
    }

    pub(crate) fn count_offset(&self) -> OffsetInt {
        self.off_read + (self.events.len() as OffsetInt)
    }

    pub(crate) fn count(&self) -> OffsetInt {
        self.events.len() as OffsetInt
    }
}

// OffsetInt must be able to hold usize values
const_assert!(std::mem::size_of::<OffsetInt>() >= std::mem::size_of::<usize>());