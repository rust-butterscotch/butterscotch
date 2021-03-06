/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

mod slot_map;

pub use slot_map::*;
pub use butterscotch_chunky_vec::*;

use crate::utility::GenericRetype;

impl<T> GenericRetype for ChunkyVec<T> {
    type RetypeWith<R> = ChunkyVec<R>;
}