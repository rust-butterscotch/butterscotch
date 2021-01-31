/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(core_intrinsics)]
#![feature(box_syntax)]
#![feature(unsize)]
#![feature(concat_idents)]

use std::any::TypeId;

use butterscotch_common::container::GID;

mod ecs;

mod component;
mod component_tuple;
mod component_tuple_gen;

mod component_store;

pub use ecs::*;

pub use component::*;
pub use component_tuple::*;
pub use component_tuple_gen::*;

pub use component_store::*;

#[cfg(test)]
mod test;

type EntityID    = GID;
type ComponentID = TypeId;
//type PropertyID  = TinyString64;
//type EventID     = TinyString128;

/*
pub trait Component: ComponentMetadata + ComponentObject {}

pub trait ComponentMetadata {
    fn id() -> ComponentID;
}
*/