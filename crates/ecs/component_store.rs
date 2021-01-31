/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */
use crate::{Component, EntityID};
use butterscotch_common::container::{GIDStore};

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