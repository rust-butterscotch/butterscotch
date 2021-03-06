/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::ops::{Index, IndexMut};

use crate::Chunk;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChunkSize {
    Elements(usize),
    MaxBytes(usize),
}

impl ChunkSize {

    pub fn into_chunk_size<T>(self) -> usize {
        let chunk_size = match self {
            ChunkSize::Elements(v) => v,
            ChunkSize::MaxBytes(v) => v/std::mem::size_of::<T>(),
        };
        chunk_size.max(1)
    }

}


#[derive(Debug)]
pub struct ChunkyVec<T> {
    chunk_size: usize,
    chunks_used: usize,
    chunks: Vec<Chunk<T>>,
}

impl<T> ChunkyVec<T> {
    pub fn new(chunk_size: ChunkSize) -> Self {
        Self{
            chunk_size: chunk_size.into_chunk_size::<T>(),
            chunks_used: 0,
            chunks: Default::default()
        }
    }

    pub fn with_capacity(chunk_size: usize, capacity: usize) -> Self {
        Self{
            chunk_size,
            chunks_used: 0,
            chunks: std::iter::repeat_with(|| Chunk::new(chunk_size)).take((capacity+chunk_size-1)/chunk_size).collect()
        }
    }

}

impl<T> ChunkyVec<T> {

    pub fn get(&self, index: usize) -> Option<&T> {
        match index < self.len() {
            true => Some(&self[index]),
            false => None,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.len() {
            true => Some(&mut self[index]),
            false => None,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.chunks_used {
            self.chunks[i].clear();
        }
        self.chunks_used = 0;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.chunks_used <= 0 { return None; }
        let result = self.chunks[self.chunks_used - 1].pop();
        if self.chunks[self.chunks_used - 1].is_empty() { self.chunks_used = self.chunks_used - 1; }
        result
    }

    pub fn remove(&mut self, index: usize) -> T {
        let index_chunk = index/self.chunk_size;
        let index_within = index - index_chunk*self.chunk_size;
        
        let mut carry = None;
        for i in (index_chunk..self.chunks_used).rev() {
            let carry_new = self.chunks[i].remove(if i == index_chunk { index_within } else { 0 });
            if carry.is_some() { self.chunks[i].push(carry.unwrap()); }
            carry = Some(carry_new);
        };
        
        return carry.unwrap();
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(self.chunks_used > 0, "Attempt to index out of bounds");
        let tmp = self.pop().unwrap();
        if index+1 == self.len() { return tmp; }
        return std::mem::replace(&mut self[index], tmp);
    }

    pub fn push(&mut self, value: T) {
        if self.chunks_used == 0 || self.chunks[self.chunks_used - 1].exhausted() {
            if self.chunks_used >= self.chunks.len() {
                self.chunks.push(Chunk::new(self.chunk_size));
            }
            self.chunks_used += 1;
        }
        self.chunks[self.chunks_used-1].push(value);
    }

    pub fn insert(&mut self, value: T, index: usize) -> Option<T> {
        let index_chunk = index/self.chunk_size;
        let index_within = index - index_chunk*self.chunk_size;

        // TODO handle insert at end
        if (index_chunk > self.chunks_used) || (index_within > self.chunks[index_chunk].len()) {
            Some(value)
        } else {
            let mut carry = self.chunks[index_chunk].insert(value, index_within);
            if carry.is_some() {
                for i in index_chunk+1..self.chunks_used {
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

impl<T: Clone> ChunkyVec<T> {

    pub fn resize(&mut self, len: usize, value: T) {
        if len < self.len() {
            self.truncate(len);
        } else {
            self.extend(len, value)
        }
    }

    pub fn extend(&mut self, len: usize, value: T) {
        let vec_len = self.len();
        self.reserve(len - vec_len);
        for _ in len..vec_len {
            self.push(value.clone());
        }
    }

}

impl<T> ChunkyVec<T> {

    pub fn resize_with<F>(&mut self, len: usize, f: F) where F: FnMut() -> T {
        if len < self.len() {
            self.truncate(len);
        } else {
            self.extend_with(len, f);
        }
    }

    pub fn extend_with<F>(&mut self, len: usize, mut f: F) where F: FnMut() -> T {
        let vec_len = self.len();
        self.reserve(len - vec_len);
        for _ in len..vec_len {
            self.push(f());
        }
    }

    pub fn reserve(&mut self, count: usize) {
        let remaining = self.capacity() - self.len();
        if remaining >= count { return; }
        // TODO is this correct?
        let chunk_size = self.chunk_size;
        self.chunks.resize_with(1+(count - remaining)/self.chunk_size, || Chunk::new(chunk_size));
    }
    
    pub fn reserve_exact(&mut self, count: usize) {
        self.reserve(count)
    }

    pub fn truncate(&mut self, chunks_used: usize) {
        let blocks = 1+(chunks_used/self.chunk_size); // TODO is this right? Probably not...
        if blocks > self.chunks_used { return; }
        for i in blocks..self.chunks_used {
            self.chunks[i-1].clear();
        }
        self.chunks_used = blocks;

        let within_block = chunks_used - (blocks-1)*self.chunk_size;
        self.chunks[blocks - 1].truncate(self.chunk_size - within_block);
    }

    pub fn shrink_to_fit(&mut self) {
        debug_assert!(self.chunks_used <= self.chunks.len());
        self.chunks.truncate(self.chunks_used);
        self.chunks.shrink_to_fit();
    }

    pub fn capacity(&self) -> usize {
        self.chunks.len() * self.chunk_size
    }

    pub fn len(&self) -> usize {
        if self.chunks_used == 0 { return 0; }
        self.chunks[self.chunks_used-1].len() + (self.chunks_used-1)*self.chunk_size
    }

    pub fn is_empty(&self) -> bool {
        self.chunks_used == 0
    }
    
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub fn chunks_used(&self) -> usize {
        self.chunks_used
    }
}

impl<T> ChunkyVec<T> {
    pub fn iter<'a>(&'a self) -> ChunkyVecIter<'a, T> {
        ChunkyVecIter{
            current: 0,
            vec: &self
        }
    }
}

impl<T> Index<usize> for ChunkyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let index_chunk = index/self.chunk_size;
        let index_within = index - index_chunk*self.chunk_size;
        &self.chunks[index_chunk][index_within]
    }
}

impl<T> IndexMut<usize> for ChunkyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index_chunk = index/self.chunk_size;
        let index_within = index - index_chunk*self.chunk_size;
        &mut self.chunks[index_chunk][index_within]
    }
}


pub struct ChunkyVecIter<'a, T> {
    vec: &'a ChunkyVec<T>,
    current: usize,
}

impl<'a, T> Iterator for ChunkyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current != self.vec.len() {
            true => {
                self.current += 1;
                self.vec.get(self.current-1)
            },
            false => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.vec.len(), Some(self.vec.len()))
    }
}



#[cfg(test)]
impl<T> ChunkyVec<T> {

    pub fn check_integrity(&self) -> Result<(), String> {
        for i in 0..self.chunks.len() {
            if i > self.len() {
                if self.chunks[i].len() > 0 { return Err("Chunk's length higher than expected. Should be empty.".to_owned()); }
            } else {
                if self.chunks[i].len() > self.chunk_size { return Err("Chunk's length higher than expected. Should be N.".to_owned()); }
            }
            if self.chunks[i].capacity() < self.chunk_size { return Err("Chunk's capacity lower than expected. Should be N.".to_owned()); }
            if self.chunks[i].capacity() > self.chunk_size { return Err("Chunk's capacity higher than expected. Should be N.".to_owned()); }
        }
        Ok(())
    }

}