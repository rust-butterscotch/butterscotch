/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::{Vec3, real};

/// Constructs a vec2 through shorthand.
#[macro_export] macro_rules! vec2 {
    ($x: expr, $y: expr) => { crate::math::Vec2([$x, $y]); }
}

/// Vector representing a 2D coordinate in cartesian space
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub struct Vec2(pub [real; 2]);

impl Vec2 {
    /// Vector with all components set to 0
    pub const ZERO: Self = Self([ 0.0,  0.0]);

    /// Vector with all components set to 1
    pub const ONE: Self = Self([ 1.0,  1.0]);

    /// Direction vector towards the right (+x) axis
    pub const RIGHT: Self = Self([ 1.0,  0.0]);

    /// Direction vector towards the up (+y) axis
    pub const UP: Self = Self([ 0.0,  1.0]);

    /// Direction vector towards the left (-x) axis
    pub const LEFT: Self = Self([ -1.0,  0.0]);

    /// Direction vector towards the down (-y) axis
    pub const DOWN: Self = Self([ 0.0,  -1.0]);
}

impl Vec2 {
    /// Creates a new vector given (x, y)
    #[inline(always)]
    pub fn new(x: real, y: real) -> Self {
        Self([x,y])
    }

    /// Converts this vector into homogenous space, by adding a w component
    #[inline(always)]
    pub fn into_homogenous(self, w: real) -> Vec3 {
        Vec3([self[0], self[1], w])
    }

    #[doc(hidden)]
    #[inline(always)]
    fn with_xy(self, x: real, y: real) -> Self {
        Self([x,y])
    }
}

impl From<Vec3> for Vec2 {
    /// Creates a new cartesian vector from a homogenous space vector, by ignoring w
    #[inline(always)]
    fn from(vec: Vec3) -> Self {
        Self([vec[0], vec[1]])
    }
}

impl Vec2 {
    /// Negates the vector
    #[inline] pub fn negate(self) -> Self {
        Self::new(-self[0], -self[1])
    }

    /// Calculates (self[0] + rhs[0], self[1] + rhs[1]...)
    #[inline] pub fn add_comp(self, rhs: Self) -> Self {
        Self::new(self[0]+rhs[0], self[1]+rhs[1])
    }

    /// Calculates (self[0] - rhs[0], self[1] - rhs[1]...)
    #[inline] pub fn sub_comp(self, rhs: Self) -> Self {
        Self::new(self[0]-rhs[0], self[1]-rhs[1])
    }

    /// Calculates (self[0] * rhs[0], self[1] * rhs[1]...)
    #[inline] pub fn mul_comp(self, rhs: Self) -> Self {
        Self::new(self[0]*rhs[0], self[1]*rhs[1])
    }

    /// Calculates (self[0] / rhs[0], self[1] / rhs[1]...)
    #[inline] pub fn div_comp(self, rhs: Self) -> Self {
        Self::new(self[0]/rhs[0], self[1]/rhs[1])
    }

    /// Calculates (self[0] * rhs, self[1] * rhs...)
    #[inline] pub fn mul_scalar(self, rhs: real) -> Self {
        Self::new(self[0]*rhs, self[1]*rhs)
    }

    /// Calculates (self[0] / rhs, self[1] / rhs...)
    #[inline] pub fn div_scalar(self, rhs: real) -> Self {
        self.mul_scalar(1.0/rhs)
    }

    /// Calculates (num / self[0], num / self[1]...)
    #[inline] pub fn recip_scalar(self, num: real) -> Self {
        Self::new(num/self[0], num/self[1])
    }
}

impl_vec2d!(Vec2);
const_assert!(core::mem::size_of::<Vec2>() == 2*core::mem::size_of::<real>());