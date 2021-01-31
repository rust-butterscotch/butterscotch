/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(core_intrinsics)]
#![feature(box_syntax)]
#![feature(unsize)]
#![feature(concat_idents)]

use std::{any::TypeId, convert::TryInto};

use arrayvec::ArrayVec;
use butterscotch_common::container::GID;

mod ecs;
mod ecs_builder;

mod component;
mod component_tuple;
mod component_tuple_gen;

mod component_store;

mod query;

pub use ecs::*;
pub use ecs_builder::*;

pub use component::*;
pub use component_tuple::*;
pub use component_tuple_gen::*;

pub use component_store::*;

pub use query::*;

#[cfg(test)]
mod test;

type EntityID    = GID;
type ComponentID = TypeId;
type QueryID     = ArrayVec<[ComponentID; 8]>;
//type PropertyID  = TinyString64;
//type EventID     = TinyString128;

// // Passthrough TypeID Hasher // //

#[derive(Debug, Default)]
pub(crate) struct TypeIDHasher(u64);

impl std::hash::Hasher for TypeIDHasher {
    fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, bytes: &[u8]) { self.0 = u64::from_ne_bytes(bytes.try_into().unwrap()); }
}

impl std::hash::BuildHasher for TypeIDHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher { Self(0) }
}

/*
pub trait Component: ComponentMetadata + ComponentObject {}

pub trait ComponentMetadata {
    fn id() -> ComponentID;
}
*/