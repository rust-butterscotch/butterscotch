/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(const_fn)]
#![feature(const_panic)]
#![feature(raw)]
#![feature(unsize)]
#![feature(core_intrinsics)]
#![feature(maybe_uninit_ref)]
#![feature(const_mut_refs)]
#![feature(box_syntax)]
#![feature(maybe_uninit_uninit_array)]

#[macro_use] extern crate static_assertions;

pub mod tuple;
pub mod container;
pub mod utility;
