/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_chunky_vec::ChunkSize;

use super::gid::GID;
use crate::container::ChunkyVec;

#[derive(Debug)]
pub struct GIDLookup {
    lookup:  ChunkyVec<GID>,
    indices: Vec<usize>,
    length: usize,
}

impl GIDLookup {

    pub fn new(chunk_size: ChunkSize) -> GIDLookup {
        GIDLookup{
            lookup:  ChunkyVec::new(chunk_size),
            indices: Vec::new(),
            length: 0,
        }
    }

    pub fn insert(&mut self, gid: GID) -> usize {
        let idx = gid.get_idx();
        if self.lookup[idx].is_valid() { panic!("Slot already contains value"); }
        return self.set_raw(gid);
    }

    pub fn remove(&mut self, gid: GID) -> Option<usize> {
        // "and_then" not playing nice with "self"
        let gidx = gid.get_idx();
        match self.lookup.get_mut(gidx) {
            Some(data_gid) => {
                // Check gen to prevent id aliasing
                if !gid.match_gen(data_gid) { return None }

                // Note index, and mark invalid
                let idx = data_gid.get_idx();
                *data_gid = data_gid.as_invalid();

                // Remove data
                self.length -= 1;

                // Update lookup of swapped element to point to it's new location
                if idx < self.indices.len() {
                    let lookup_gid = &mut self.lookup[self.indices[idx]];
                    *lookup_gid = lookup_gid.with_idx(idx);
                }

                Some(self.length)
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

    pub fn get_offset(&self, gid: GID) -> Option<usize> {
        self.lookup.get(gid.get_idx()).and_then(|data_gid|
            match gid.match_gen(data_gid) {
                true => Some(data_gid.get_idx()),
                false => None
            }
        )
    }


    pub fn clear(&mut self) {
        self.lookup.clear();
        self.indices.clear();
        self.length = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.length <= 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = GID> + 'a {
        self.indices.iter().map(move |v| self.lookup[*v].with_idx(*v))
    }

    pub fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.lookup.shrink_to_fit(); // Only shrink, don't remove freed lookups
        self.indices.shrink_to_fit();
    }

    pub fn get_key_at(&self, id: usize) -> Option<GID> {
        self.lookup.get(self.indices[id]).and_then(|v| {
            Some(v.with_idx(self.indices[id]))
        })
    }

    //TODO pub fn retain<F>(&mut self, f: F) where F: FnMut(&K, &mut V) -> bool,
    fn expand_lookup(&mut self, count: usize) {
        let lookup_len = self.lookup.len();
        self.lookup.resize(lookup_len + count, GID::new());
    }

    fn set_raw(&mut self, gid: GID) -> usize {
        let idx = gid.get_idx();
        if self.lookup.len() <= idx {
            let chunk_size = self.lookup.chunk_size();
            self.expand_lookup(idx + chunk_size + 1); // TODO better size constraint?
        }
        self.lookup[idx] = gid.with_idx(self.length);
        self.length += 1;
        self.indices.push(idx);
        return self.length-1;
    }
}