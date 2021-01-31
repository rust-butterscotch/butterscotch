/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(unchecked_math)]
#![feature(const_panic)]
#![feature(const_fn)]
#![feature(const_euclidean_int_methods)]
#![feature(optimize_attribute)]
#![feature(core_intrinsics)]
#![feature(unboxed_closures)]
#![feature(box_syntax)]
#![feature(negative_impls)]
#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(arc_new_cyclic)]
#![feature(raw_ref_op)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(dispatch_from_dyn)]

#[macro_use] extern crate static_assertions;

#[macro_use] pub mod util;

pub mod math;
pub mod container;
pub mod dpi;
pub mod chrono;
pub mod interop;
pub mod future;
pub mod event;