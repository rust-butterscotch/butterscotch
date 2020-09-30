/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::{Vec2, real};

/// Vector representing a 2D coordinate in a homogenous space
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub struct Vec3(pub [real; 3]);

/// Constructs a vec2d through shorthand.
#[macro_export] macro_rules! vec3 {
    ($x:expr, $y:expr, $w:expr) => { crate::math::Vec3([$x, $y, $w]) };
}

/// Constructs a vec2d through shorthand with `w` = 1.0
#[macro_export] macro_rules! pnt2d {
    ($x:expr, $y:expr) => { crate::math::Vec3D([$x, $y, 1.0]); };
}

/// Constructs a vec2d through shorthand with `w` = 0.0
#[macro_export] macro_rules! dir2d {
    ($x:expr, $y:expr) => { crate::math::Vec3D([$x, $y, 0.0]); };
}

impl Vec3 {
    /// Vector with all components set to 0
    pub const ZERO: Self = Self([ 0.0,  0.0,  0.0]);

    /// Vector with all components set to 1
    pub const ONE: Self = Self([ 1.0,  1.0,  1.0]);

    /// Direction vector towards the right (+x) axis
    pub const RIGHT: Self = Self([ 1.0,  0.0,  0.0]);

    /// Direction vector towards the up (+y) axis
    pub const UP: Self = Self([ 0.0,  1.0,  0.0]);

    /// Direction vector towards the left (-x) axis
    pub const LEFT: Self = Self([ -1.0,  0.0,  0.0]);

    /// Direction vector towards the down (-y) axis
    pub const DOWN: Self = Self([ 0.0,  -1.0,  0.0]);
}

impl Vec3 {
    /// Creates a new vector given (x, y)
    #[inline(always)]
    pub fn new(x: real, y:  real, w: real) -> Self {
        Self([x,y,w])
    }

    /// Converts this vector into cartesian space, by adding a w component
    #[inline(always)]
    pub fn from_cartesian(v: Vec2, w: real) -> Self {
        Self([v[0],v[1],w])
    }

    #[doc(hidden)]
    #[inline(always)]
    fn with_xy(self, x: real, y: real) -> Self {
        Self([x, y, self[2]])
    }
}

impl Vec3 {
    /// Negates the vector
    #[inline] pub fn negate(self) -> Self {
        Self::new(-self[0], -self[1], -self[2])
    }

    /// Calculates (self[0] + rhs[0], self[1] + rhs[1]...)
    #[inline] pub fn add_comp(self, rhs: Self) -> Self {
        Self::new(self[0]+rhs[0], self[1]+rhs[1], self[2]+rhs[2])
    }

    /// Calculates (self[0] - rhs[0], self[1] - rhs[1]...)
    #[inline] pub fn sub_comp(self, rhs: Self) -> Self {
        Self::new(self[0]-rhs[0], self[1]-rhs[1], self[2]-rhs[2])
    }

    /// Calculates (self[0] * rhs[0], self[1] * rhs[1]...)
    #[inline] pub fn mul_comp(self, rhs: Self) -> Self {
        Self::new(self[0]*rhs[0], self[1]*rhs[1], self[2]*rhs[2])
    }

    /// Calculates (self[0] / rhs[0], self[1] / rhs[1]...)
    #[inline] pub fn div_comp(self, rhs: Self) -> Self {
        Self::new(self[0]/rhs[0], self[1]/rhs[1], self[2]/rhs[2])
    }

    /// Calculates (self[0] * rhs, self[1] * rhs...)
    #[inline] pub fn mul_scalar(self, rhs: real) -> Self {
        Self::new(self[0]*rhs, self[1]*rhs, self[2]*rhs)
    }

    /// Calculates (self[0] / rhs, self[1] / rhs...)
    #[inline] pub fn div_scalar(self, rhs: real) -> Self {
        self.mul_scalar(1.0/rhs)
    }

    /// Calculates (num / self[0], num / self[1]...)
    #[inline] pub fn recip_scalar(self, num: real) -> Self {
        Self::new(num/self[0], num/self[1], num/self[2])
    }
}

impl_vec2d!(Vec3);
const_assert!(core::mem::size_of::<Vec3>() == 3*core::mem::size_of::<real>());
