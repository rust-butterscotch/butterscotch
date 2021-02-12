/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_chunky_vec::{ChunkSize, ChunkyVecIter};

use super::{ComponentMapKeyIter, GIDRegistry, GIDStore, GID};

#[derive(Debug)]
pub struct SlotMap<T> {
    registry: GIDRegistry,
    store:    GIDStore<T>
}

impl<T> SlotMap<T> {

    pub fn new(chunk_size: ChunkSize) -> Self {
        Self{
            registry: Default::default(),
            store:    GIDStore::new(chunk_size)
        }
    }

    pub fn insert(&mut self, v: T) -> GID {
        let gid = self.registry.acquire();
        self.store.insert(gid, v);
        return gid;
    }

    pub fn remove(&mut self, gid: GID) -> Option<T> {
        self.registry.release(gid);
        return self.store.remove(gid);
    }

    pub fn contains_key(&self, gid: GID) -> bool {
        self.registry.contains_key(gid)
    }

    pub fn get(&self, gid: GID) -> Option<&T> {
        self.store.get(gid)
    }

    pub fn get_mut(&mut self, gid: GID) -> Option<&mut T> {
        self.store.get_mut(gid)
    }

    pub fn clear(&mut self) {
        self.registry.clear();
        self.store.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.registry.len() <= 0
    }

    pub fn capacity(&self) -> usize {
        self.registry.capacity()
    }

    pub fn len(&self) -> usize {
        self.registry.len()
    }

    pub fn freelist_len(&self) -> usize {
        self.registry.freelist_len()
    }

    pub fn iter(&self) -> ChunkyVecIter<'_, T> {
        self.store.iter()
    }

    // TODO fix....
    //pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
    //    self.store.iter_mut()
    //}

    pub fn reserve(&mut self, additional: usize) {
        self.registry.reserve(additional);
        self.store.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.registry.shrink_to_fit();
        self.store.shrink_to_fit();
    }

    pub fn keys<'a>(&'a self) -> ComponentMapKeyIter::<'a, T> {
        self.store.keys()
    }

    pub fn get_key_at(&self, id: usize) -> Option<GID> {
        self.store.get_key_at(id)
    }
}