/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_chunky_vec::ChunkyVec;

use crate::tuple::{TupleElementGetter, TupleElementWrapper, TupleRef};

use super::{GID, GIDLookup, GIDMultiStoreHelper};

pub struct GIDMultiStore<T: GIDMultiStoreHelper + TupleElementWrapper + TupleRef> where <T as TupleElementWrapper>::WrapWith<ChunkyVec<()>>: TupleElementGetter {
    lookup: GIDLookup,
    data: T::WrapWith<ChunkyVec<()>>
}

impl<T: GIDMultiStoreHelper + TupleElementWrapper> GIDMultiStore<T> where <T as TupleElementWrapper>::WrapWith<ChunkyVec<()>>: TupleElementGetter {

    pub fn insert(&mut self, gid: GID, v: T) {
        let idx = self.lookup.insert(gid);
        T::insert(v, &mut self.data, idx);
    }

    pub fn remove(&mut self, gid: GID) -> Option<T> {
        match self.lookup.remove(gid) {
            Some(v) => Some(T::swap_remove(&mut self.data, v)),
            None    => None,
        }
    }

    pub fn get(&mut self, gid: GID) -> Option<T::AsRef<'_>> {
        match self.lookup.get_offset(gid) {
            Some(v) => Some(T::get(&mut self.data, v)),
            None    => None,
        }
    }

}
