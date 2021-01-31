/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{alloc::{Allocator, Global, Layout}, ops::{Deref}, ptr::NonNull};

use super::{RefPoolHeap, Weak};

/// Strong reference to a pool allocated value
/// Once all strong references are freed, excl. the one in the pool, the storage
/// is returned to the pool if it still exists.
#[derive(Debug)]
pub struct Prc<T: ?Sized> {
    pub(super) heap: NonNull<RefPoolHeap<T>>
}

impl<T: ?Sized> Prc<T> {
    pub(super) fn new(heap: NonNull<RefPoolHeap<T>>) -> Prc<T> {
        unsafe { heap.as_ref().inc_strong() }
        Self { heap }
    }
}

impl<T: ?Sized> Prc<T> {
    pub fn downgrade(&self) -> Weak<T> {
        Weak::new(self.heap)
    }

    pub fn release(&self) {
        unsafe{&*self.heap.as_ref()}.release();
    }

    pub fn is_released(&self) -> bool {
        unsafe{&*self.heap.as_ref()}.is_released()
    }

    pub fn strong_count(&self) -> usize {
        unsafe{&*self.heap.as_ref()}.strong_count.get()
    }

    pub fn weak_count(&self) -> usize {
        unsafe{&*self.heap.as_ref()}.strong_count.get()
    }
}

impl<T: ?Sized> Clone for Prc<T> {
    fn clone(&self) -> Self {
        Self::new(self.heap)
    }
}

impl<T: ?Sized> Deref for Prc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.heap.as_ref().value }
    }
}

unsafe impl<#[may_dangle] T: ?Sized> Drop for Prc<T> {
    fn drop(&mut self) {
        unsafe {
            if self.heap.as_mut().dec_strong() {
                std::ptr::drop_in_place(self.heap.as_ptr());
                Global.deallocate(self.heap.cast(), Layout::for_value(self.heap.as_ref()));
            }
        }
    }
}
