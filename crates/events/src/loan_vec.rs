/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_common::likely;

#[derive(Debug)]
pub(crate) struct LoanVec<T> {
    data: Result<Vec<T>, usize>,
}

impl<T> Default for LoanVec<T> {
    fn default() -> Self { Self::new() }
}

impl<T> LoanVec<T> {
    pub fn new() -> Self {
        Self{data: Err(0)}
    }

    pub fn take(&mut self) -> Self {
        std::mem::replace(self, self.store_capacity())
    }

    pub fn store_capacity(&self) -> Self {
        Self{data: Err(match self.data.as_ref() {
            Ok (v) =>  v.capacity(),
            Err(v) => *v,
        })}
    }

    pub fn try_store_capacity_into(&self, dst: &mut Self) {
        if dst.is_borrowed() {
            *dst = self.store_capacity();
        }
    }

    pub fn swap(&mut self, dst: &mut Self) {
        std::mem::swap(&mut self.data, &mut dst.data);
    }

    pub fn len(&self) -> usize {
        self.data.as_ref().ok().unwrap().len()
    }

    pub fn clear(&mut self) {
        self.data.as_mut().ok().unwrap().clear()
    }

    pub fn is_borrowed(&self) -> bool {
        self.data.is_err()
    }

    pub fn as_mut(&mut self) -> &mut Vec<T> {
        self.ensure();
        self.data.as_mut().ok().unwrap()
    }

    pub fn ensure(&mut self) {
        if likely!(self.data.is_ok()) { return; }
        let c = *self.data.as_ref().err().unwrap();
        self.data = Ok(Vec::with_capacity(c));
    }
}