use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Chunk<T>(Vec<T>);

impl<T> Chunk<T> {

    pub fn new(size: usize) -> Self {
        Self(
            Vec::with_capacity(size)
        )
    }

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

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len)
    }

    pub fn exhausted(&self) -> bool {
        self.len() >= self.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<T> Index<usize> for Chunk<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Chunk<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
