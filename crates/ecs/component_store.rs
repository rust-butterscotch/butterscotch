use std::any::Any;

/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */
use crate::{Component, ComponentID, EntityID};
use butterscotch_common::container::{GIDStore};

pub trait ComponentStoreAny: Any + std::fmt::Debug {
    fn component_id(&self)     -> ComponentID;
    fn component_id_str(&self) -> &'static str;
}


#[derive(Debug)]
pub struct ComponentStore<T: Component> {
    store: GIDStore<T>,
}

impl<T: Component> Default for ComponentStore<T> {
    fn default() -> Self {
        Self{
            store: Default::default()
        }
    }
}

impl<T: Component> ComponentStoreAny for ComponentStore<T> {
    fn component_id(&self)     -> ComponentID  { T::ID     }
    fn component_id_str(&self) -> &'static str { T::ID_STR }
}

impl<T: Component> ComponentStore<T> {

    pub fn get_ref(&self, eid: EntityID) -> Option<& T> {
        self.store.get(eid)
    }

    pub fn contains(&mut self, eid: EntityID) -> bool {
        self.store.contains_key(eid)
    }

    /*pub fn set(&mut self, eid: EntityID, value: &mut MoveRef) {
        todo!()
    }

    pub fn get(&mut self, eid: EntityID, value: &mut MoveRef) {
        todo!()
    }

    pub fn get_ref(&self, eid: EntityID) -> Option<&dyn ComponentObject> {
        todo!()
    }

    pub fn get_mut(&mut self, eid: EntityID) -> Option<&mut dyn ComponentObject> {
        todo!()
    }

    pub fn has(&self, eid: EntityID) -> bool {
        todo!()
    }

    pub fn remove(&mut self, eid: EntityID) -> bool {
        todo!()
    }

    pub fn entities(&self, out: &mut Vec<EntityID>) {
        todo!()
    }
    
    pub fn get_raw_ref(&    self, eid: EntityID) -> Option<&    T> {
        todo!()
    }

    pub fn get_raw_mut(&mut self, eid: EntityID) -> Option<&mut T> {
        todo!()
    }*/
}