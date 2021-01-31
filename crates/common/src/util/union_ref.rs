/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{marker::Unsize, ops::{Deref}, rc::Rc, sync::Arc};

use crate::container::Prc;


pub enum UnionRef<T: ?Sized + 'static> {
    Arc(Arc<T>),
    Prc(Prc<T>),
    Rc(Rc<T>),
    Box(Box<T>),
    Ref(&'static T)
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> Into<UnionRef<U>> for Arc<T> {
    fn into(self) -> UnionRef<U> { UnionRef::Arc(self) }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> Into<UnionRef<U>> for Prc<T> {
    fn into(self) -> UnionRef<U> { UnionRef::Prc(self) }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> Into<UnionRef<U>> for Rc<T> {
    fn into(self) -> UnionRef<U> { UnionRef::Rc(self) }
}

impl<T: ?Sized + 'static> UnionRef<T> {
    pub fn try_clone(this: &Self) -> Option<UnionRef<T>> {
        match this {
            UnionRef::Arc(v) => Some(UnionRef::Arc(Arc::clone(v))),
            UnionRef::Prc(v) => Some(UnionRef::Prc(Prc::clone(v))),
            UnionRef::Rc(v)  => Some(UnionRef::Rc(  Rc::clone(v))),
            UnionRef::Box(_) => None,
            UnionRef::Ref(v) => Some(UnionRef::Ref(v))
        }
    }
}

impl<T: Clone + 'static> Clone for UnionRef<T> {
    fn clone(&self) -> UnionRef<T> {
        match self {
            UnionRef::Arc(v) => UnionRef::Arc(Arc::clone(v)),
            UnionRef::Prc(v) => UnionRef::Prc(Prc::clone(v)),
            UnionRef::Rc(v)  => UnionRef::Rc(  Rc::clone(v)),
            UnionRef::Box(v) => UnionRef::Box(Box::clone(v)),
            UnionRef::Ref(v) => UnionRef::Ref(v)
        }
    }
}

impl<T: ?Sized + 'static> Deref for UnionRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            UnionRef::Arc(v) => v.deref(),
            UnionRef::Prc(v) => v.deref(),
            UnionRef::Rc(v)  => v.deref(),
            UnionRef::Box(v) => v.deref(),
            UnionRef::Ref(v) => v
        }
    }
}