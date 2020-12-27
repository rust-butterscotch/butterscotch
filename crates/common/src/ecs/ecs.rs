/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{any::TypeId, cell::Ref, cell::RefCell, cell::RefMut, collections::HashMap, convert::TryInto, hash::BuildHasherDefault, hash::Hasher};
use butterscotch_common::container::{GID, GIDRegistry};

use super::System;

pub struct ECS<T: Clone> {
    entities: GIDRegistry,
    systems: Vec<Box<RefCell<dyn System<T>>>>,
    systems_by_type: HashMap<TypeId, usize, BuildHasherDefault<TypeHasher>>,
}

impl<T: Clone> ECS<T> {

    pub fn new(systems: Vec<Box<RefCell<dyn System<T>>>>) -> ECS<T> {
        let mut systems_by_type = HashMap::with_hasher(Default::default());
        for i in 0..systems.len() {
            systems_by_type.insert(systems[i].borrow().type_id(), i).expect("type_id conflict");
        }
        ECS{
            entities: GIDRegistry::new(),
            systems,
            systems_by_type,
        }
    }

    pub fn create(&mut self) -> GID {
        self.entities.acquire()
    }

    pub fn destroy(&mut self, id: GID) {
        for system in &self.systems {
            system.borrow_mut().entity_destroyed(id);
        }
        self.entities.release(id);
    }

    pub fn notify(&mut self, ev: &T) {
        for system in &self.systems {
            system.borrow_mut().notify(ev);
        }
    }

    pub fn try_find_system(&self, type_id: &TypeId) -> Option<Ref<'_, dyn System<T>>> {
        match self.systems_by_type.get(type_id) {
            Some(v) => Some(self.systems.get(*v).unwrap().borrow()),
            None    => None,
        }
    }

    pub fn try_find_system_mut(&mut self, type_id: &TypeId) -> Option<RefMut<'_, dyn System<T>>> {
        match self.systems_by_type.get(type_id) {
            Some(v) => Some(self.systems.get(*v).unwrap().borrow_mut()),
            None    => None,
        }
    }

    pub fn try_get_system<S: System<T> + 'static>(&self) -> Option<Ref<'_, dyn System<T>>> {
        match self.systems_by_type.get(&TypeId::of::<S>()) {
            Some(v) => Some(self.systems.get(*v).unwrap().borrow()),
            None    => None,
        }
    }

    pub fn try_get_system_mut<S: System<T> + 'static>(&mut self) -> Option<RefMut<'_, dyn System<T>>> {
        match self.systems_by_type.get(&TypeId::of::<S>()) {
            Some(v) => Some(self.systems.get(*v).unwrap().borrow_mut()),
            None    => None,
        }
    }
}

// //////////////////////////////////////// //
// // Quick insecure hasher for type ids // //
// //////////////////////////////////////// //

#[repr(transparent)]
#[derive(Default)]
struct TypeHasher(u64);
impl Hasher for TypeHasher {
    #[inline] fn finish(&self) -> u64 { self.0 }
    #[inline] fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_ne_bytes(
            bytes.try_into().expect("bad size for byte array when hashing type id")
        );
    }
}