/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{any::Any, fmt::Debug};

use crate::ComponentID;

pub trait Component: Any + Debug {
    const ID: ComponentID;
    const ID_STR: &'static str;
}