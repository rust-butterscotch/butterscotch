/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::{GID_IDX_MASK, gid::GID};
use std::collections::VecDeque;

const RESERVE_BLOCK_SIZE: usize = 128;

pub struct SlotMap<T> {
    lookup: Vec<GID>,
    data:   Vec<T>,
    indices: Vec<usize>,
    freelist: VecDeque<GID>,
}

impl<T> SlotMap<T> {

    pub fn new() -> SlotMap<T> {
        SlotMap{
            lookup: vec!(),
            data:   vec!(),
            indices: vec!(),
            freelist: VecDeque::<GID>::new(),
        }
    }

    pub fn insert(&mut self, v: T) -> GID {
        if self.data.len() > GID_IDX_MASK as usize {
            panic!("SlotMap out of room");
        }

        if self.freelist.is_empty() {
            self.expand_freelist();
        }

        let index = self.freelist.pop_front().expect("Failed to allocate from freelist");
        self.data.push(v);
        self.indices.push(index.get_idx());
        return index;
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

                // Mark data free
                self.freelist.push_back(gid);

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
        self.freelist.clear();
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

    pub fn freelist_len(&self) -> usize {
        self.freelist.len()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
        self.indices.reserve(additional);
        self.lookup.reserve(self.data.capacity() - self.lookup.len());
        self.freelist.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.lookup.shrink_to_fit(); // Don't remove freed lookups
        self.data.shrink_to_fit();
        self.indices.shrink_to_fit();
        self.freelist.shrink_to_fit();
    }

    pub fn keys<'a>(&'a self) -> SlotMapKeyIter::<'a, T> {
        SlotMapKeyIter::<'a, T>{ map: self, current: 0, }
    }

    pub fn get_key_at(&self, id: usize) -> Option<GID> {
        self.lookup.get(self.indices[id]).and_then(|v| {
            Some(v.with_idx(self.indices[id]))
        })
    }

    //TODO pub fn retain<F>(&mut self, f: F) where F: FnMut(&K, &mut V) -> bool,

    fn expand_freelist(&mut self) {
        let lookup_len = self.lookup.len();
        if lookup_len >= std::usize::MAX - 1 { panic!("SlotMap out of memory"); }
        // We don't need to check freelist since freelist.len <= lookup_len

        let reserve_count = RESERVE_BLOCK_SIZE.min(std::usize::MAX - lookup_len);
        self.lookup.resize(lookup_len + reserve_count, GID::new());
        self.data.reserve(reserve_count);

        let mut i = 0;
        self.freelist.reserve(reserve_count);
        self.freelist.resize_with(self.freelist.len()+reserve_count, ||{
            i+=1;
            GID::new().renew_as(lookup_len+i-1)
        });
    }
}

pub struct SlotMapKeyIter<'a, T> {
    map: &'a SlotMap<T>,
    current: usize,
}

impl<'a, T> Iterator for SlotMapKeyIter<'a, T> {
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