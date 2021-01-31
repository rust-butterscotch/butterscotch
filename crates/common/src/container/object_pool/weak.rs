/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{alloc::{Allocator, Global, Layout}, convert::TryInto, ptr::NonNull};

use crate::container::GID;

use super::{Prc, RefPoolHeap};

/// Weak reference to a pool allocated value
/// Once the pool value is freed, the weak ref will never be able to be upgraded
/// Even if the slot is reused
#[derive(Debug)]
pub struct Weak<T: ?Sized> {
    owner_id: GID,
    heap: NonNull<RefPoolHeap<T>>
}

impl<T: ?Sized> Weak<T> {
    pub(super) fn new(heap: NonNull<RefPoolHeap<T>>) -> Weak<T> {
        let owner_id = unsafe {
            let heap = heap.as_ref();
            heap.inc_weak();
            heap.owner_id.get()
        }.unwrap_or(GID::new());
        Self { heap, owner_id }
    }
}

impl<T: ?Sized> Weak<T> {
    pub fn upgrade(&self) -> Option<Prc<T>> {
        let heap = unsafe {self.heap.as_ref()};
        if heap.is_released() { return None; }
        if heap.owner_id.get() != Some(self.owner_id) { return None; }
        return Some(Prc::new(self.heap));
    }
}

impl<T: ?Sized> TryInto<Prc<T>> for Weak<T> {
    type Error = ();
    fn try_into(self) -> Result<Prc<T>, Self::Error> {
        self.upgrade().ok_or(())
    }
}

impl<T: ?Sized> Into<Option<Prc<T>>> for Weak<T> {
    fn into(self) -> Option<Prc<T>> {
        self.upgrade()
    }
}

impl<T: ?Sized> From<Prc<T>> for Weak<T> {
    fn from(value: Prc<T>) -> Self {
        value.downgrade()
    }
}

impl<T: ?Sized> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self::new(self.heap)
    }
}

unsafe impl<#[may_dangle] T: ?Sized> Drop for Weak<T> {
    fn drop(&mut self) {
        unsafe {
            if self.heap.as_mut().dec_weak() {
                std::ptr::drop_in_place(self.heap.as_ptr());
                Global.deallocate(self.heap.cast(), Layout::for_value(self.heap.as_ref()));
            }
        }
    }
}