/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(min_const_generics)]
#![feature(unchecked_math)]
#![feature(const_panic)]
#![feature(const_fn)]
#![feature(const_euclidean_int_methods)]
#![feature(optimize_attribute)]
#![feature(core_intrinsics)]
#![feature(unboxed_closures)]
#![feature(box_syntax)]

#[macro_use] extern crate static_assertions;

#[macro_use] pub mod util;

pub mod math;
pub mod container;
pub mod dpi;
pub mod chrono;
pub mod interop;
pub mod future;
pub mod event;