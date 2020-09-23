/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

mod event_consumer;
mod event_queue;
mod event_publisher;

pub use event_publisher::*;
pub use event_consumer::*;
pub use event_queue::*;

// offset counter
type OffsetInt = u64; // 500_000_000_000 events/second for 1 year to overflow

#[derive(Debug, Copy, Clone)]
pub enum Event {
    Notify(&'static str)
}
