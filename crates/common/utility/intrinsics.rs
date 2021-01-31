/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#[macro_export] macro_rules! likely {
    ($x:expr) => { { core::intrinsics::likely($x) } }
}

#[macro_export] macro_rules! unlikely {
    ($x:expr) => { { core::intrinsics::unlikely($x) } }
}