/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(core_intrinsics)]
#![feature(unboxed_closures)]

mod loan_vec;
mod event_system;

pub(crate) use loan_vec::*;
pub use event_system::*;
