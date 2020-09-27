/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::gid::GID;
use std::collections::VecDeque;

const RESERVE_BLOCK_SIZE: usize = 128;

pub struct GIDRegistry {
    gen_lookup: Vec<u16>,
    freelist: VecDeque<GID>,
}

impl GIDRegistry {

    pub fn new() -> GIDRegistry {
        GIDRegistry{
            gen_lookup: vec!(),
            freelist: VecDeque::<GID>::new(),
        }
    }

    pub fn acquire(&mut self) -> GID {
        if self.freelist.is_empty() {
            self.expand_freelist();
        }

        let index = self.freelist.pop_front().expect("Failed to allocate from freelist");
        self.gen_lookup[index.get_idx()] = index.get_gen();
        return index;
    }

    pub fn release(&mut self, gid: GID) -> bool {
        let gidx = gid.get_idx();
        match self.gen_lookup.get_mut(gidx) {
            Some(generation) => {
                // Check gen to prevent id aliasing
                if gid.get_gen() != *generation { return false }
                *generation = 0;
                self.freelist.push_back(gid);
                true
            },
            None => false
        }
    }

    pub fn contains_key(&self, gid: GID) -> bool {
        match self.gen_lookup.get(gid.get_idx()) {
            Some(generation) => gid.get_gen() != *generation,
            None             => false,
        }
    }

    pub fn clear(&mut self) {
        self.gen_lookup.clear();
        self.freelist.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.gen_lookup.len() <= self.freelist.len()
    }

    pub fn capacity(&self) -> usize {
        self.gen_lookup.capacity()
    }

    pub fn len(&self) -> usize {
        self.gen_lookup.len() - self.freelist.len()
    }

    pub fn freelist_len(&self) -> usize {
        self.freelist.len()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.gen_lookup.reserve(additional);
        self.freelist.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.gen_lookup.shrink_to_fit(); // Don't remove freed lookups
        self.freelist.shrink_to_fit();
    }

    //TODO pub fn retain<F>(&mut self, f: F) where F: FnMut(&K, &mut V) -> bool,

    fn expand_freelist(&mut self) {
        let lookup_len = self.gen_lookup.len();
        if lookup_len >= std::usize::MAX - 1 { panic!("SlotMap out of memory"); }
        // We don't need to check freelist since freelist.len <= lookup_len

        let reserve_count = RESERVE_BLOCK_SIZE.min(std::usize::MAX - lookup_len);
        self.gen_lookup.resize(lookup_len + reserve_count, 0);

        let mut i = 0;
        self.freelist.reserve(reserve_count);
        self.freelist.resize_with(self.freelist.len()+reserve_count, ||{
            i+=1;
            GID::new().renew_as(lookup_len+i-1)
        });
    }
}