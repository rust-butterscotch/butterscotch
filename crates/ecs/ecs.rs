/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{any::{Any, TypeId}, collections::HashMap, convert::TryInto};

use butterscotch_common::utility::{downcast_mut_unchecked, downcast_ref_unchecked};

use crate::{Component, ComponentID, ComponentStore};

#[derive(Debug, Default)]
pub struct ECS {
    component_stores: HashMap<ComponentID, Box<dyn Any>, TypeIDHasher>
}

impl ECS {

    pub fn register_component<T: Component>(&mut self) {
        let result = self.component_stores.insert(TypeId::of::<T>(), box ComponentStore::<T>::default());
        assert!(result.is_none(), "Component Type already registered");
    }

    pub fn get_store_ref<'a, T: Component + 'static>(&'a self) -> &'a ComponentStore<T> { unsafe { 
        let store = self.component_stores
            .get(&TypeId::of::<T>())
            .expect(&format!("ComponentStore not registered for \"{}\"", std::any::type_name::<T>()));
        downcast_ref_unchecked::<ComponentStore<T>>(store) // Assuming that typeid doesn't collide (it "can") we don't need to check before casting
    }}

    pub fn get_store_mut<'a, T: Component>(&'a mut self) -> &'a mut ComponentStore<T> { unsafe { 
        let store = self.component_stores
            .get_mut(&TypeId::of::<T>())
            .expect(&format!("ComponentStore not registered for \"{}\"", std::any::type_name::<T>()));
        downcast_mut_unchecked::<ComponentStore<T>>(store) // Assuming that typeid doesn't collide (it "can") we don't need to check before casting
    }}

}

// // Passthrough TypeID Hasher // //

#[derive(Debug, Default)]
struct TypeIDHasher(u64);

impl std::hash::Hasher for TypeIDHasher {
    fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, bytes: &[u8]) { self.0 = u64::from_ne_bytes(bytes.try_into().unwrap()); }
}

impl std::hash::BuildHasher for TypeIDHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher { Self(0) }
}