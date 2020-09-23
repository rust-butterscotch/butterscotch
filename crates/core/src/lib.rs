/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(bool_to_option)]
#![feature(box_syntax)]
#![feature(clamp)]
#![feature(maybe_uninit_uninit_array)]
#![feature(min_const_generics)]
#![feature(unchecked_math)]

#![feature(const_panic)]
#![feature(const_int_pow)]
#![feature(const_euclidean_int_methods)]
#![feature(const_fn)]

#![feature(generators)]
#![feature(generator_trait)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(shrink_to)]

#[macro_use]
extern crate static_assertions;

pub mod container;
pub mod math;
pub mod util;
pub mod chrono;
pub mod init;
pub mod engine;
pub mod event;
