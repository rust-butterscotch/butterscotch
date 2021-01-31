/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::Cell, mem::{ManuallyDrop, MaybeUninit}, ptr::NonNull, rc::Weak};

use crate::container::GID;

use super::object_pool::ObjectPool;

#[derive(Debug)]
pub(super) struct RefPoolHeap<T: ?Sized> {
    owner_ref:    Weak<ObjectPool<T>>,
    pub(super) owner_id:     Cell<Option<GID>>,
    pub(super) strong_count: Cell<usize>,
    pub(super) weak_count:   Cell<usize>,
    pub(super) value: ManuallyDrop<T>,
}

impl<T> RefPoolHeap<T> {
    pub fn new(owner_ref: Weak<ObjectPool<T>>) -> NonNull<RefPoolHeap<T>> {
        return Box::leak(box Self{
            owner_ref,
            owner_id: None.into(),
            strong_count: 0.into(),
            weak_count:   0.into(),
            value: unsafe { MaybeUninit::uninit().assume_init() } 
        }).into();
    }

    pub fn set(&mut self, id: GID, value: T) {
        unsafe{std::ptr::write(&raw mut self.value, ManuallyDrop::new(value));}
        self.owner_id.replace(Some(id));
    }
}

impl<T: ?Sized> RefPoolHeap<T> {
    pub fn inc_strong(&self) {
        self.strong_count.replace(self.strong_count.get() + 1);
    }

    #[must_use = "You must use the value returned here to determine if you should deallocate the structure (true = dealloc)"]
    pub fn dec_strong(&mut self) -> bool {
        self.strong_count.replace(self.strong_count.get() - 1);
        self.check_alloc()
    }

    pub fn inc_weak(&self) {
        self.weak_count.replace(self.weak_count.get() + 1);
    }

    #[must_use = "You must use the value returned here to determine if you should deallocate the structure (true = dealloc)"]
    pub fn dec_weak(&mut self) -> bool {
        self.weak_count.replace(self.weak_count.get() - 1);
        self.check_alloc()
    }

    pub fn release(&self) {
        if let Some(gid) = self.owner_id.get() {
            if let Some(owner) = self.owner_ref.upgrade() {
                self.owner_id.replace(None);
                owner.release_gid(gid);
            }
        }
    }

    pub fn is_released(&self) -> bool {
        self.owner_id.get().is_none()
    }

    fn check_alloc(&mut self) -> bool {
        let strong_count = self.strong_count.get();
        if strong_count > 1 { return false; }

        if strong_count == 1 {
            if let Some(gid) = self.owner_id.get() {
                // If the pool is the only one with the reference, then deinit and return slot.
                if let Some(owner) = self.owner_ref.upgrade() {
                    self.deinit();
                    owner.return_gid(gid);
                }
            }
            return false;
        }

        self.deinit();
        return self.weak_count.get() <= 0;
    }

    fn deinit(&mut self) {
        if self.is_released() { return; }
        unsafe{ std::ptr::drop_in_place(&raw mut self.value); }
        self.owner_id.replace(None);
    }    
}

/// Avoid double-drop (Technically allowed, but not expected by programmers)
impl<T: ?Sized> Drop for RefPoolHeap<T> {
    fn drop(&mut self) {
        if self.is_released() { return; }
        self.deinit();
    }
}