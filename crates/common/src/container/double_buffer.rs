/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#[derive(Debug)]
pub struct DoubleBuffer<T>(Vec<T>, Option<Vec<T>>);

impl<T> DoubleBuffer<T> {
    pub fn new() -> Self {
        Self(Vec::default(), Some(Vec::default()))
    }

    pub fn push(&mut self, v: T) {
        self.0.push(v);
    }

    pub fn take(&mut self) -> Vec<T> {
        match self.1.as_mut() {
            Some(v) => {
                std::mem::swap(&mut self.0, v);
                std::mem::replace(&mut self.1, None).unwrap()
            }
            None => {
                let capacity = self.0.capacity();
                std::mem::replace(&mut self.0, Vec::<T>::with_capacity(capacity))
            }
        }
    }

    pub fn expect_take(&mut self) -> Vec<T> {
        std::mem::swap(&mut self.0, &mut self.1.as_mut().unwrap());
        std::mem::replace(&mut self.1, None).unwrap()
    }

    pub fn try_take(&mut self) -> Option<Vec<T>> {
        match self.1.as_mut() {
            Some(v) => {
                std::mem::swap(&mut self.0, v);
                std::mem::replace(&mut self.1, None)
            }
            None => None,
        }
    }

    pub fn replace(&mut self, mut buffer: Vec<T>) {
        buffer.clear();
        self.1 = Some(buffer);
    }

    pub fn swap(&mut self, buffer: &mut Vec<T>) {
        std::mem::swap(&mut self.0, buffer);
    }

    #[inline] pub fn len(&mut self) -> usize {
        self.0.len()
    }
}

impl <T> Default for DoubleBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}