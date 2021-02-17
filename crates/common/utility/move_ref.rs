/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{any::{Any, TypeId}, ptr::NonNull};
use std::marker::PhantomData;

use crate::{unlikely, utility::{downcast_mut_ptr_unchecked, downcast_mut_unchecked}};

const ERROR_NO_VALUE:  &str = "The value has already been removed from the MoveRef";
const ERROR_HAS_VALUE: &str = "MoveRef already has a value";
const ERROR_NO_CAST:   &str = "Couldn't downcast value into requested type";

/// Constructs an un/occupied MoveRef and passes it to the given 
/// callback to consume. This function ensures that use of the 
/// MoveRef will not result in undefined behaviour or leaks.
///
/// MoveRef allows for the opaque transfer of stack-allocated
/// values that implement the Any trait. Ultimately to allow
/// moving data through traits without preventing the creation
/// of trait-objects due to generics or the cost of moving a
/// value to the heap and the cost of to then Copy/Clone the value.
///
/// This function safely constructs and sends a MoveRef made from an 
/// optional, but the callback may consume and replace the value 
/// stored within it. This is defined behaviour and the caller of this 
/// function may use this value.
#[inline] pub fn send_moveref<T: Any, F: FnMut(&mut MoveRef)>(value: Option<T>, callback: &mut F) -> Option<T> {
    let is_init = value.is_some();
    let mut result = match value {
        Some(v) => std::mem::MaybeUninit::<T>::new(v),
        None    => std::mem::MaybeUninit::<T>::uninit()
    };
    let mut boxed = unsafe { MoveRef::new(is_init, NonNull::new_unchecked(result.as_mut_ptr())) };
    callback(&mut boxed);
    return match boxed.is_occupied() {
        true  => Some(unsafe{result.assume_init()}),
        false => None,
    };
}


/// Contains a referenced Any trait and provides the ability to
/// manually move it, without requring heap allocation. This is
/// special because it allows us to move through trait-objects,
/// as it also doesn't require generics which prevent v-tables.
///
/// This is achieved through a pointer-pointer copy, where the
/// source data is never touched again - skirting around the UB
/// around doing this with non-copy types and the cost of clone.
///
/// Data can also be moved into it, to facilitate return 
/// paramaters with the same guarentees
///
/// To construct and send an MoveRef, see send_moveref
pub struct MoveRef<'a>(bool, NonNull<dyn Any>, TypeId, PhantomData<&'a mut dyn Any>);

impl<'a> MoveRef<'a> {
    /// Use send_moveref, unless you know exactly what you need to do.
    #[inline(always)] pub unsafe fn new<T: Any>(occupied: bool, value: NonNull<T>) -> Self { 
        Self(occupied, value, TypeId::of::<T>(), PhantomData::default()) 
    }

    /// Returns if the MoveRef contains a value
    #[inline(always)] pub fn is_occupied(&self) -> bool {
        self.0
    }

    /// Returns if the MoveRef does not contain a value
    #[inline(always)] pub fn is_unoccupied(&self) -> bool {
        !self.0
    }
}

impl<'a> MoveRef<'a> {
    /// Swaps the content of this MoveRef with the provided value.
    /// Panics if the MoveRef is unoccupied, or if the value can't be downcast to the requested type.
    pub fn swap<T: Sized + Any + 'static>(&mut self, value: &mut T) {
        if unlikely!(self.try_swap(value)) {
            panic!("{}", if self.is_unoccupied() { ERROR_NO_VALUE } else { ERROR_NO_CAST })
        }
    }

    /// Swaps the content of this MoveRef with the provided value.
    /// Returns false if the MoveRef is unoccupied, or if the value can't be downcast to the requested type.
    pub fn try_swap<T: Sized + Any + 'static>(&mut self, value: &mut T) -> bool {
        if self.is_occupied() && self.2 == TypeId::of::<T>() { unsafe { // Manual check as target isn't initialized
            std::ptr::swap_nonoverlapping(value, downcast_mut_unchecked::<T>(self.1.as_mut()), 1);
            return true;
        }}
        return false;
    }
}


impl<'a> MoveRef<'a> {
    /// Swaps the content of this MoveRef with the provided value.
    /// Panics if the MoveRef is unoccupied, without checking if the value can be safely downcast (except in Debug builds, it will panic)
    pub unsafe fn swap_unchecked<T: Sized + Any + 'static>(&mut self, value: &mut T) {
        if unlikely!(self.try_swap_unchecked(value)) {
            panic!("{}", ERROR_NO_VALUE)
        }
    }

    /// Swaps the content of this MoveRef with the provided value.
    /// Returns false if the MoveRef is unoccupied, without checking if the value can be safely downcast (except in Debug builds, it will panic)
    pub unsafe fn try_swap_unchecked<T: Sized + Any + 'static>(&mut self, value: &mut T) -> bool {
        debug_assert_eq!(TypeId::of::<T>(), self.2, "Attempt to perform unchecked cast on incompatible types");

        if self.is_occupied() {
            std::ptr::swap_nonoverlapping(value, downcast_mut_unchecked::<T>(self.1.as_mut()), 1);
            return true;
        }
        return false;
    }
}


impl<'a> MoveRef<'a> {
    /// Sets the content of this MoveRef to the provided value.
    /// Panics if the MoveRef is occupied, or if the value can't be downcast to the requested type.
    pub fn set<T: Sized + Any + 'static>(&mut self, value: T) {
        match self.try_set(value) {
            Some(_) => panic!("{}", if self.is_occupied() { ERROR_HAS_VALUE } else { ERROR_NO_CAST }),
            None    => {}
        }
    }

    /// Sets the content of this MoveRef to the provided value.
    /// Returns false if the MoveRef is occupied, or if the value can't be downcast to the requested type.
    pub fn try_set<T: Sized + Any + 'static>(&mut self, value: T) -> Option<T> {
        if self.is_unoccupied() && self.2 == TypeId::of::<T>() { unsafe{ // Manual any check as target isn't initialized
            std::ptr::copy_nonoverlapping(&value, downcast_mut_ptr_unchecked::<T>(self.1.as_ptr()), 1);
            self.0 = true;
            return None;
        }}
        Some(value)
    }
}


impl<'a> MoveRef<'a> {
    /// Sets the content of this MoveRef to the provided value.
    /// Panics if the MoveRef is occupied, without checking if the value can be safely downcast (except in Debug builds, it will panic)
    pub unsafe fn set_unchecked<T: Sized + Any + 'static>(&mut self, value: T) {
        match self.try_set_unchecked(value) {
            Some(_) => panic!("{}", if self.is_occupied() { ERROR_HAS_VALUE } else { ERROR_NO_CAST }),
            None    => {}
        }
    }

    /// Sets the content of this MoveRef to the provided value.
    /// Returns false if the MoveRef is occupied, without checking if the value can be safely downcast (except in Debug builds, it will panic)
    pub unsafe fn try_set_unchecked<T: Sized + Any + 'static>(&mut self, value: T) -> Option<T> {
        debug_assert_eq!(TypeId::of::<T>(), self.2, "Attempt to perform unchecked cast on incompatible types");

        if self.is_unoccupied() {
            std::ptr::copy_nonoverlapping(&value, downcast_mut_ptr_unchecked::<T>(self.1.as_ptr()), 1);
            self.0 = true;
            return None;
        }
        Some(value)
    }
}

impl<'a> MoveRef<'a> {
    /// Moves the content of this MoveRef out, consuming it. It's okay to set a new value after this.
    /// Panics if the MoveRef is unoccupied, or if the value can't be downcast to the requested type.
    pub fn unwrap<T: Sized + Any + 'static>(&mut self) -> T {
        match self.try_unwrap() {
            Some(v) => v,
            None    => panic!("{}", if self.is_unoccupied() { ERROR_NO_VALUE } else { ERROR_NO_CAST })
        }
    }

    /// Moves the content of this MoveRef out, consuming it. It's okay to set a new value after this.
    /// Returns None if the MoveRef is unoccupied, or if the value can't be downcast to the requested type.
    pub fn try_unwrap<T: Sized + Any + 'static>(&mut self) -> Option<T> {
        if self.is_occupied() {
            if let Some(v) = unsafe{self.1.as_mut()}.downcast_mut::<T>() { 
                // This performs a "move". We make sure not to use the struct
                // that the data was copied from, so this doesn't cause UB
                self.0 = false;
                let mut result = std::mem::MaybeUninit::<T>::uninit();
                unsafe {
                    std::ptr::copy_nonoverlapping(v, result.as_mut_ptr(), 1);
                    return Some(result.assume_init());
                }
            }
        }
        return None;
    }
}


impl<'a> MoveRef<'a> {
    /// Moves the content of this MoveRef out, consuming it. It's okay to set a new value after this.
    /// Panics if the MoveRef is unoccupied, without checking if the value can be safely downcast (except in Debug builds, it will panic)
    pub unsafe fn unwrap_unchecked<T: Sized + Any + 'static>(&mut self) -> T {
        match self.try_unwrap() {
            Some(v) => v,
            None    => panic!("{}", ERROR_NO_VALUE)
        }
    }

    /// Moves the content of this MoveRef out, consuming it. It's okay to set a new value after this.
    /// Returns None if the MoveRef is unoccupied, without checking if the value can be safely downcast (except in Debug builds, it will panic)
    pub unsafe fn try_unwrap_unchecked<T: Sized + Any + 'static>(&mut self) -> Option<T> {
        debug_assert_eq!(TypeId::of::<T>(), self.2, "Attempt to perform unchecked cast on incompatible types");

        if self.is_occupied() {
            // This performs a "move". We make sure not to use the struct
            // that the data was copied from, so this doesn't cause UB
            self.0 = false;
            let mut result = std::mem::MaybeUninit::<T>::uninit();
            std::ptr::copy_nonoverlapping(downcast_mut_unchecked::<T>(self.1.as_mut()), result.as_mut_ptr(), 1);
            return Some(result.assume_init());
        }
        
        return None;
    }
}

impl<'a> MoveRef<'a> {
    /// Empties the MoveRef, dropping its contents if it has any. Returns if the contents were dropped.
    pub fn discard(&mut self) -> bool {
        if self.is_occupied() {
            self.0 = false;
            unsafe{ std::ptr::drop_in_place(self.1.as_mut()); }
            return true;
        }
        return false;
    }
}