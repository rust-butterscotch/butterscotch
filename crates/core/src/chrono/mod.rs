/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

mod accumulator;
mod timer;
mod timer_smooth;

pub use accumulator::*;
pub use timer::*;
pub use timer_smooth::*;

pub type Time = crate::math::FixedBase<1_000_000_000>; // 292 years @ nanosecond accuracy
