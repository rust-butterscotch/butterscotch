/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(min_const_generics)]
#![feature(unchecked_math)]
#![feature(const_panic)]
#![feature(const_fn)]
#![feature(const_euclidean_int_methods)]
#![feature(optimize_attribute)]

#[macro_use] extern crate static_assertions;

pub mod container;
pub mod math;
pub mod chrono;
pub mod util;