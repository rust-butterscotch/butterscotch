/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::{RefCell}, collections::VecDeque, rc::{Rc, Weak}};

use crate::container::{GID, SlotMap};

use super::{Prc, RefPoolHeap};

/// ObjectPool provides reusable heap allocations for sized objects, 
/// which are automatically returned when they fall out of scope.
#[derive(Debug)]
pub struct ObjectPool<T: ?Sized> {
    self_ref: Weak<ObjectPool<T>>,
    pool:     RefCell<SlotMap<Prc<T>>>,
    freelist: RefCell<VecDeque<Prc<T>>>,
}

impl<T> ObjectPool<T> {
    pub fn new() -> Rc<ObjectPool<T>> {
        Rc::new_cyclic(|self_ref| {
            Self{
                self_ref: self_ref.clone(),
                pool: Default::default(),
                freelist: Default::default()
            }
        })
    }

    pub fn with_capacity(capacity: usize) -> Rc<ObjectPool<T>> {
        let result = Self::new();
        result.reserve(capacity);
        return result;
    }

    #[must_use = "Allocating without using the return value will result in the allocation immediately being freed"]
    pub fn alloc(&self, value: T) -> Prc<T> {
        let gid = self.pool.borrow_mut().insert(
            match self.freelist.borrow_mut().pop_front() {
                Some(free) => free,
                None       => Prc::new(RefPoolHeap::new(self.self_ref.clone()))
            }
        );
        let mut obj = self.pool.borrow_mut().get(gid).unwrap().clone();
        unsafe{obj.heap.as_mut()}.set(gid, value);
        return obj;
    }

    pub fn shrink_to_fit(&self) {
        self.freelist.borrow_mut().clear();
    }

    pub fn len(&self) -> usize {
        self.pool.borrow().len()
    }

    pub fn capacity(&self) -> usize {
        self.pool.borrow().len() + self.freelist.borrow().len()
    }

    pub fn reserve(&self, count: usize) {
        let mut freelist = self.freelist.borrow_mut();
        freelist.reserve(count);
        for _ in (freelist.capacity()-freelist.len())..count {
            freelist.push_back(Prc::new(RefPoolHeap::new(self.self_ref.clone())))
        }
    }
}

impl<T: ?Sized> ObjectPool<T> {
    pub(super) fn return_gid(&self, gid: GID) -> bool {
        let a = match self.pool.borrow_mut().remove(gid) {
            Some(v) => { self.freelist.borrow_mut().push_back(v); true },
            None    => { false },
        };
        a
    }

    pub(super) fn release_gid(&self, gid: GID) {
        self.pool.borrow_mut().remove(gid);
    }
}