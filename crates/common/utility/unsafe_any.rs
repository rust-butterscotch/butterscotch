/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{any::Any, raw::TraitObject};

pub unsafe fn downcast_ref_unchecked<T: Any>(target: &dyn Any) -> &T {
    std::mem::transmute(std::mem::transmute::<*const dyn Any, TraitObject>(target).data)
}

pub unsafe fn downcast_mut_unchecked<T: Any>(target: &mut dyn Any) -> &mut T {
    std::mem::transmute(std::mem::transmute::<*mut dyn Any, TraitObject>(target).data)
}

pub unsafe fn downcast_mut_ptr_unchecked<T: Any>(target: *mut dyn Any) -> *mut T {
    std::mem::transmute(std::mem::transmute::<*mut dyn Any, TraitObject>(target).data)
}

pub unsafe fn downcast_const_ptr_unchecked<T: Any>(target: *mut dyn Any) -> *const T {
    std::mem::transmute(std::mem::transmute::<*const dyn Any, TraitObject>(target).data)
}