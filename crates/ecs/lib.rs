/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(core_intrinsics)]
#![feature(box_syntax)]
#![feature(unsize)]
#![feature(concat_idents)]

use std::{convert::TryInto};

use arrayvec::ArrayVec;
use butterscotch_common::container::GID;

mod ecs;

mod component;
mod component_tuple;

mod component_store;

mod query;

pub use ecs::*;

pub use component::*;
pub use component_tuple::*;

pub use component_store::*;

pub use query::*;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ComponentID(pub u16);
pub type EntityID    = GID;
pub type QueryID     = ArrayVec<[ComponentID; 8]>;

// // Passthrough TypeID Hasher // //
#[derive(Debug, Default)]
pub(crate) struct BadIntHasher(u64);

impl std::hash::Hasher for BadIntHasher {
    fn finish(&self) -> u64 { 
        self.0
    }

    fn write(&mut self, bytes: &[u8]) { 
        self.0 ^= match bytes.len() {
            16 => u128::from_ne_bytes(bytes.try_into().unwrap()) as u64,
             8 =>  u64::from_ne_bytes(bytes.try_into().unwrap()) as u64,
             4 =>  u32::from_ne_bytes(bytes.try_into().unwrap()) as u64,
             2 =>  u16::from_ne_bytes(bytes.try_into().unwrap()) as u64,
             1 =>   u8::from_ne_bytes(bytes.try_into().unwrap()) as u64,
            _ => unimplemented!()
        };
    }
}

impl std::hash::BuildHasher for BadIntHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher { Self(0) }
}