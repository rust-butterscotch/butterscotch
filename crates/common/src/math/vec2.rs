/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::{Vec3, real};

/// Constructs a vec2 through shorthand.
#[macro_export] macro_rules! vec2 {
    ($x: expr, $y: expr) => { crate::math::Vec2{x: $x, y: $y} }
}

/// Vector representing a 2D coordinate in cartesian space
#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub struct Vec2{
    pub x: real,
    pub y: real,
}

impl Vec2 {
    /// Vector with all components set to 0
    pub const ZERO: Self = Self{x: 0.0, y:  0.0};

    /// Vector with all components set to 1
    pub const ONE: Self = Self{x: 1.0, y:  1.0};

    /// Direction vector towards the right (+x) axis
    pub const RIGHT: Self = Self{x: 1.0, y:  0.0};

    /// Direction vector towards the up (+y) axis
    pub const UP: Self = Self{x: 0.0, y:  1.0};

    /// Direction vector towards the left (-x) axis
    pub const LEFT: Self = Self{x: -1.0, y:  0.0};

    /// Direction vector towards the down (-y) axis
    pub const DOWN: Self = Self{x: 0.0, y: -1.0};
}

impl Vec2 {
    /// Creates a new vector given (x, y)
    #[inline] pub fn new(x: real, y: real) -> Self {
        Self{x: x, y: y}
    }

    /// Converts this vector into homogenous space, by adding a w component
    #[inline] pub fn into_homogenous(self, w: real) -> Vec3 {
        Vec3::new(self.x, self.y, w)
    }

    #[inline] pub fn len() -> usize {
        3
    }

    #[doc(hidden)]
    #[inline] fn with_xy(self, x: real, y: real) -> Self {
        Self{x: x, y: y}
    }
}

impl From<Vec3> for Vec2 {
    /// Creates a new cartesian vector from a homogenous space vector, by ignoring w
    #[inline] fn from(vec: Vec3) -> Self {
        Self{x: vec.x, y: vec.y}
    }
}

impl Vec2 {
    // Calculates the sum of each element
    #[inline] pub fn taxicab_distance(self) -> real {
        self.x + self.y
    }

    /// Negates the vector
    #[inline] pub fn negate(self) -> Self {
        Self::new(-self.x, -self.y)
    }

    /// Calculates (self.x + rhs.x, self.y + rhs.y...)
    #[inline] pub fn add_comp(self, rhs: Self) -> Self {
        Self::new(self.x+rhs.x, self.y+rhs.y)
    }

    /// Calculates (self.x - rhs.x, self.y - rhs.y...)
    #[inline] pub fn sub_comp(self, rhs: Self) -> Self {
        Self::new(self.x-rhs.x, self.y-rhs.y)
    }

    /// Calculates (self.x * rhs.x, self.y * rhs.y...)
    #[inline] pub fn mul_comp(self, rhs: Self) -> Self {
        Self::new(self.x*rhs.x, self.y*rhs.y)
    }

    /// Calculates (self.x / rhs.x, self.y / rhs.y...)
    #[inline] pub fn div_comp(self, rhs: Self) -> Self {
        Self::new(self.x/rhs.x, self.y/rhs.y)
    }

    /// Calculates (self.x * rhs, self.y * rhs...)
    #[inline] pub fn mul_scalar(self, rhs: real) -> Self {
        Self::new(self.x*rhs, self.y*rhs)
    }

    /// Calculates (self.x / rhs, self.y / rhs...)
    #[inline] pub fn div_scalar(self, rhs: real) -> Self {
        self.mul_scalar(1.0/rhs)
    }

    /// Calculates (num / self.x, num / self.y...)
    #[inline] pub fn recip_scalar(self, num: real) -> Self {
        Self::new(num/self.x, num/self.y)
    }
}

// ///////////////////// //
// // Index operation // //
// ///////////////////// //
impl core::ops::Index<usize> for Vec2 {
    type Output = real;
    #[inline] fn index(&self, index: usize) -> &real {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Attempt to index out of range")
        }
    }
}

impl core::ops::IndexMut<usize> for Vec2 {
    #[inline] fn index_mut(&mut self, index: usize) -> &mut real {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Attempt to index out of range")
        }
    }
}

impl_vec2d!(Vec2);
const_assert!(core::mem::size_of::<Vec2>() == 2*core::mem::size_of::<real>());