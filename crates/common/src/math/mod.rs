/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#[allow(non_camel_case_types)]
pub type real = f64;

#[macro_use] pub(self) mod mat_common;
#[macro_use] pub(self) mod vec_common;

#[macro_use] mod vec3;
#[macro_use] mod vec2;
#[macro_use] mod mat3;
#[macro_use] mod mat2;

pub use vec3::*;
pub use vec2::*;
pub use mat3::*;
pub use mat2::*;