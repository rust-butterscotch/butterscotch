/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{any::TypeId, collections::HashMap, hash::{BuildHasherDefault, Hasher}};
use crate::util::AnyCast;

pub struct TraitMap<T: ?Sized + AnyCast> {
    data: HashMap<TypeId, Box<T>, BuildHasherDefault<TypePassthroughHasher>>
}

impl<T:'static + ?Sized + AnyCast> TraitMap<T> {

    pub fn new() -> TraitMap<T> {
        TraitMap::<T>{ data: HashMap::with_hasher(Default::default()) }
    }

    pub fn insert(&mut self, value: Box<T>) -> Option<Box<T>> {
        self.data.insert(value.as_ref().type_id(), value)
    }

    pub fn remove<V:'static>(&mut self) -> Option<Box<T>> {
        let id = TypeId::of::<V>();
        self.data.remove(&id)
    }

    pub fn get_ref<V:'static>(&self) -> Option<&V> {
        match self.get_ref_for(TypeId::of::<V>()) {
            Some(v) => v.as_any_ref().downcast_ref::<V>(),
            None => None
        }
    }

    pub fn get_mut<V:'static>(&mut self) -> Option<&mut V> {
        match self.get_mut_for(TypeId::of::<V>()) {
            Some(v) => v.as_any_mut().downcast_mut::<V>(),
            None => None
        }
    }

    pub fn get_ref_for(&self, id: TypeId) -> Option<&T> {
        match self.data.get(&id) {
            Some(v) => Some(v.as_ref()),
            None => None
        }
    }

    pub fn get_mut_for(&mut self, id: TypeId) -> Option<&mut T> {
        match self.data.get_mut(&id) {
            Some(v) => Some(v.as_mut()),
            None => None
        }
    }
}

#[derive(Default)]
struct TypePassthroughHasher {
    value: u64
}

impl Hasher for TypePassthroughHasher {
    #[inline] fn finish(&self) -> u64 { self.value }
    #[inline] fn write(&mut self, bytes: &[u8]) {
        assert!(bytes.len() == 8);
        self.value = u64::from(bytes[0]) <<  0 | u64::from(bytes[1]) <<  8 | u64::from(bytes[2]) << 16
                   | u64::from(bytes[3]) << 24 | u64::from(bytes[4]) << 32 | u64::from(bytes[5]) << 40
                   | u64::from(bytes[6]) << 48 | u64::from(bytes[7]) << 56;
    }
}