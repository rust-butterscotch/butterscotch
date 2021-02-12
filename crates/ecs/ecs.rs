/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{collections::HashMap};

use butterscotch_common::{container::ChunkSize, utility::{downcast_mut_unchecked, downcast_ref_unchecked}};

use crate::{BadIntHasher, Component, ComponentID, ComponentStore, ComponentStoreAny};

#[derive(Debug, Default)]
pub struct ECS {
    component_stores: HashMap<ComponentID, Box<dyn ComponentStoreAny>, BadIntHasher>
}

impl ECS {

    pub fn register_component<T: Component>(&mut self, chunk_size: ChunkSize) {
        let result = self.component_stores.insert(T::ID, box ComponentStore::<T>::new(chunk_size));
        assert!(result.is_none(), "ComponentID({}) conflict between \"{}\" and \"{}\"", T::ID.0, T::ID_STR, result.unwrap().component_id_str());
    }

    pub fn get_store_ref<'a, T: Component + 'static>(&'a self) -> &'a ComponentStore<T> { unsafe { 
        let store = self.component_stores
            .get(&T::ID)
            .expect(&format!("ComponentStore not registered for \"{}\"", std::any::type_name::<T>()));
        downcast_ref_unchecked::<ComponentStore<T>>(store) // Assuming that typeid doesn't collide (it "can") we don't need to check before casting
    }}

    pub fn get_store_mut<'a, T: Component>(&'a mut self) -> &'a mut ComponentStore<T> { unsafe { 
        let store = self.component_stores
            .get_mut(&T::ID)
            .expect(&format!("ComponentStore not registered for \"{}\"", std::any::type_name::<T>()));
        downcast_mut_unchecked::<ComponentStore<T>>(store) // Assuming that typeid doesn't collide (it "can") we don't need to check before casting
    }}

}
