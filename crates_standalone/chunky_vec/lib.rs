/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(shrink_to)]

use std::ops::{Index, IndexMut};

pub struct ChunkyVec<T, const N: usize> {
    length: usize,
    chunks: Vec<Chunk<T, N>>,
}

impl<T, const N: usize> ChunkyVec<T,N> {
    pub fn new() -> Self {
        Self{
            length: 0,
            chunks: Default::default()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self{
            length: 0,
            chunks: std::iter::repeat_with(|| Default::default()).take((capacity+N-1)/N).collect()
        }
    }

}

impl<T, const N: usize> ChunkyVec<T,N> {

    pub fn clear(&mut self) {
        for i in 0..self.length {
            self.chunks[i].clear();
        }
        self.length = 0;
    }

    pub fn pop(&mut self) -> Option<T> {
        let result = self.chunks[self.length - 1].pop();
        if self.chunks[self.length - 1].is_empty() { self.length = self.length - 1; }
        result
    }

    pub fn remove(&mut self, index: usize) -> T {
        let index_chunk = index/N;
        let index_within = index - index_chunk*N;
        
        let mut carry = None;
        for i in (index_chunk..self.length).rev() {
            let carry_new = self.chunks[i].remove(if i == index_chunk { index_within } else { 0 });
            if carry.is_some() { self.chunks[i].push(carry.unwrap()); }
            carry = Some(carry_new);
        };
        
        return carry.unwrap();
    }

    pub fn swap_remove(&mut self, index: usize) {
        assert!(self.length > 0, "Attempt to index out of bounds");
        let tmp = self.pop();
        if index+1 == self.len() { return; }
        self[index] = tmp.unwrap();
    }

    pub fn push(&mut self, value: T) {
        if self.chunks[self.length - 1].exhausted() {
            if self.length >= self.chunks.len() {
                self.chunks.push(Default::default());
            }
            self.length = self.length + 1;
        }
        self.chunks[self.length - 1].push(value);
    }

    pub fn insert(&mut self, value: T, index: usize) -> Option<T> {
        let index_chunk = index/N;
        let index_within = index - index_chunk*N;

        if (index_chunk > self.length) || (index_within > self.chunks[index_chunk].len()) {
            Some(value)
        } else {
            let mut carry = self.chunks[index_chunk].insert(value, index_within);
            if carry.is_some() {
                for i in index_chunk+1..self.length {
                    carry = self.chunks[i].insert(carry.unwrap(), 0);
                    if carry.is_none() { break; }
                }
                // Ran out of carry room, append it to the end
                if carry.is_some() { self.push(carry.unwrap()); }
            }
            None
        }
    }
}


impl<T, const N: usize> ChunkyVec<T,N> {

    pub fn reserve(&mut self, count: usize) {
        let remaining = self.capacity() - self.len();
        if remaining >= count { return; }
        // TODO is this correct?
        self.chunks.resize_with(1+(count - remaining)/N, || Default::default());
    }
    
    pub fn reserve_exact(&mut self, count: usize) {
        self.reserve(count)
    }

    pub fn shrink_to(&mut self, length: usize) {
        self.chunks.shrink_to(1+(length/N));
    }

    pub fn shrink_to_fit(&mut self) {
        self.chunks.shrink_to(self.length);
    }

    pub fn capacity(&self) -> usize {
        self.chunks.len() * N
    }

    pub fn len(&self) -> usize {
        self.chunks[self.length-1].len() + (self.length-1)*N
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    
    // == Can't implement currently ==
    // pub fn try_reserve(&mut self, count: usize);
    // pub fn try_reserve_exact(&mut self, count: usize);
}

impl<T, const N: usize> Index<usize> for ChunkyVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let index_chunk = index/N;
        let index_within = index - index_chunk*N;
        &self.chunks[index_chunk][index_within]
    }
}

impl<T, const N: usize> IndexMut<usize> for ChunkyVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index_chunk = index/N;
        let index_within = index - index_chunk*N;
        &mut self.chunks[index_chunk][index_within]
    }
}


impl<T, const N: usize> Default for ChunkyVec<T,N> {
    fn default() -> Self {
        Self::new()
    }
}

// impl<T, const N: usize> ChunkyVec<T,N> {
// pub fn append(&mut self) {}
// pub fn extend_from_slice(&mut self) {}
// pub fn remove_item(&mut self) {}
// pub fn resize(&mut self) {}
// pub fn resize_with(&mut self) {}
// pub fn set_len(&mut self) {}
// pub fn truncate(&mut self) {}

// == ???? ==
// pub fn retain(&mut self) {}
// pub fn dedup(&mut self) {}
// pub fn dedup_by(&mut self) {}
// pub fn dedup_by_key(&mut self) {}
// pub fn leak(&mut self) {}
// pub fn into_boxed_slice(&mut self) {}
// pub fn into_raw_parts(&mut self) {}
// pub fn from_raw_parts(&mut self) {}
// pub fn split_off(&mut self) {}
// pub fn splice(&mut self) {}

// == Skip? ==
// pub fn drain(&mut self) {}
// pub fn drain_filter(&mut self) {}

// == Can't implement? ==
// pub fn spare_capacity_mut(&mut self) {}
// pub fn as_mut_ptr(&mut self) {}s
// pub fn as_mut_slice(&mut self) {}
// pub fn as_ptr(&mut self) {}
// pub fn as_slice(&mut self) {}

// }







struct Chunk<T, const N: usize>(Vec<T>);

impl<T, const N: usize> Default for Chunk<T,N> {
    fn default() -> Self {
        Self(Vec::<T>::with_capacity(N))
    }
}

impl<T, const N: usize> Chunk<T,N> {

    pub fn push(&mut self, v: T) {
        debug_assert!(!self.exhausted(), "Chunk is fully exhusted. Logic error.");
        self.0.push(v);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn insert(&mut self, value: T, index: usize) -> Option<T> {
        if self.exhausted() {
            let result = self.pop();
            self.insert(value, index);
            return result;
        }
        self.insert(value, index);
        None
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    pub fn exhausted(&self) -> bool {
        self.len() < N
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T, const N: usize> Index<usize> for Chunk<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Chunk<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}