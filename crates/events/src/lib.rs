/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

mod consumer;
mod queue;
mod publisher;

pub use publisher::*;
pub use consumer::*;
pub(crate) use queue::*;

#[macro_use] extern crate static_assertions;