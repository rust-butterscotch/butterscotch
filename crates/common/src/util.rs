/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::any::Any;

pub trait AnyCast : Any {
    fn as_any_ref(&    self) -> &    dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AnyCast for T {
    #[inline(always)] fn as_any_ref(&    self) -> &    dyn Any { self }
    #[inline(always)] fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[macro_export]
macro_rules! likely {
    ($x:expr) => { unsafe { core::intrinsics::likely($x) } }
}

#[macro_export]
macro_rules! unlikely {
    ($x:expr) => { unsafe { core::intrinsics::unlikely($x) } }
}