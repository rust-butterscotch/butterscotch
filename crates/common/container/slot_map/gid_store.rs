/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_chunky_vec::{ChunkSize, ChunkyVecIter};

use super::gid::GID;
use crate::container::ChunkyVec;

#[derive(Debug)]
pub struct GIDStore<T> {
    lookup:  ChunkyVec<GID>,
    data:    ChunkyVec<T>,
    indices: Vec<usize>,
}

impl<T> GIDStore<T> {

    pub fn new(chunk_size: ChunkSize) -> GIDStore<T> {
        GIDStore{
            lookup:  ChunkyVec::new(chunk_size),
            data:    ChunkyVec::new(chunk_size),
            indices: Vec::new(),
        }
    }

    pub fn insert(&mut self, gid: GID, v: T) {
        let idx = gid.get_idx();
        if self.data.len() <= idx { self.expand_lookup(); }
        if self.lookup[idx].is_valid() { panic!("Slot already contains value"); }
        self.set_raw(gid, v);
    }

    pub fn replace(&mut self, gid: GID, v: T) {
        let idx = gid.get_idx();
        if self.data.len() <= idx { self.expand_lookup(); }
        self.set_raw(gid, v);
    }

    pub fn remove(&mut self, gid: GID) -> Option<T> {
        // "and_then" not playing nice with "self"
        let gidx = gid.get_idx();
        match self.lookup.get_mut(gidx) {
            Some(data_gid) => {
                // Check gen to prevent id aliasing
                if !gid.match_gen(data_gid) { return None }

                // Note index, and mark invalid
                let idx = data_gid.get_idx();
                *data_gid = data_gid.as_invalid();

                // Remove data & back-reference via swap & pop
                let result = Some(self.data.swap_remove(idx));
                self.indices.swap_remove(idx);

                // Update lookup of swapped element to point to it's new location
                if idx < self.indices.len() {
                    let lookup_gid = &mut self.lookup[self.indices[idx]];
                    *lookup_gid = lookup_gid.with_idx(idx);
                }

                result
            },
            None => None
        }
    }

    pub fn contains_key(&self, gid: GID) -> bool {
        match self.lookup.get(gid.get_idx()) {
            Some(data_gid) => gid.match_gen(data_gid),
            None           => false,
        }
    }

    pub fn get(&self, gid: GID) -> Option<&T> {
        self.lookup.get(gid.get_idx()).and_then(|data_gid|
            match gid.match_gen(data_gid) {
                true => self.data.get(data_gid.get_idx()),
                false => None
            }
        )
    }

    pub fn get_mut(&mut self, gid: GID) -> Option<&mut T> {
        self.lookup.get(gid.get_idx()).copied().and_then(move |v|{
            match gid.match_gen(&v) {
                true  => self.data.get_mut(v.get_idx()),
                false => None,
            }
        })
    }

    pub fn clear(&mut self) {
        self.lookup.clear();
        self.data.clear();
        self.indices.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() <= 0
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> ChunkyVecIter<'_, T> {
        self.data.iter()
    }

    // TODO fix....
    //pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
    //    self.data.iter_mut() 
    //}

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
        self.indices.reserve(additional);
        self.lookup.reserve(self.data.capacity() - self.lookup.len());
    }

    pub fn shrink_to_fit(&mut self) {
        self.lookup.shrink_to_fit(); // Don't remove freed lookups
        self.data.shrink_to_fit();
        self.indices.shrink_to_fit();
    }

    pub fn keys<'a>(&'a self) -> ComponentMapKeyIter::<'a, T> {
        ComponentMapKeyIter::<'a, T>{ map: self, current: 0, }
    }

    pub fn get_key_at(&self, id: usize) -> Option<GID> {
        self.lookup.get(self.indices[id]).and_then(|v| {
            Some(v.with_idx(self.indices[id]))
        })
    }

    //TODO pub fn retain<F>(&mut self, f: F) where F: FnMut(&K, &mut V) -> bool,
    fn expand_lookup(&mut self) {
        let chunk_size = self.data.chunk_size();
        let lookup_len = self.lookup.len();
        if lookup_len >= std::usize::MAX-chunk_size { panic!("SlotMap out of memory"); }

        self.lookup.resize(lookup_len + chunk_size, GID::new());
        self.data.reserve(chunk_size);
    }

    fn set_raw(&mut self, gid: GID, v: T) {
        let idx = gid.get_idx();
        self.lookup[idx] = gid.with_idx(self.data.len());
        self.data.push(v);
        self.indices.push(idx);
    }
}


pub struct ComponentMapKeyIter<'a, T> {
    map: &'a GIDStore<T>,
    current: usize,
}

impl<'a, T> Iterator for ComponentMapKeyIter<'a, T> {
    type Item = GID;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current != self.map.len() {
            true => {
                self.current += 1;
                self.map.get_key_at(self.current-1)
            },
            false => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.map.len(), Some(self.map.len()))
    }
}