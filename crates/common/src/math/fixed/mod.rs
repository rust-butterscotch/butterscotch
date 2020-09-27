/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#[optimize(speed)] mod def;
#[optimize(speed)] pub mod consts;
#[optimize(speed)] mod general;
#[optimize(speed)] mod polyfill;
#[optimize(speed)] mod util;
#[optimize(speed)] mod ops;

pub use def::*;
// pub use consts::*;
pub use general::*;
pub use polyfill::*;
pub use util::*;
pub use ops::*;

#[cfg(test)]
mod test;