/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::{Vec2, real};

/// Vector representing a 2D coordinate in a homogenous space
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
pub struct Vec3 {
    pub x: real,
    pub y: real,
    pub w: real,
}

impl Vec3 {
    /// Vector with all components set to 0
    pub const ZERO: Self = Self{x: 0.0, y: 0.0, w: 0.0};

    /// Vector with all components set to 1
    pub const ONE: Self = Self{x: 1.0, y: 1.0, w: 1.0};

    /// Direction vector towards the right (+x) axis
    pub const RIGHT: Self = Self{x: 1.0, y: 0.0, w: 0.0};

    /// Direction vector towards the up (+y) axis
    pub const UP: Self = Self{x: 0.0, y: 1.0, w: 0.0};

    /// Direction vector towards the left (-x) axis
    pub const LEFT: Self = Self{x:-1.0, y: 0.0, w: 0.0};

    /// Direction vector towards the down (-y) axis
    pub const DOWN: Self = Self{x: 0.0, y:-1.0, w: 0.0};
}

impl Vec3 {
    /// Creates a new vector given (x, y)
    #[inline] pub fn new(x: real, y:  real, w: real) -> Self {
        Self{x, y, w}
    }

    /// Converts this vector into cartesian space, by adding a w component
    #[inline] pub fn from_cartesian(v: Vec2, w: real) -> Self {
        Self{x: v.x, y: v.x, w}
    }

    #[inline] pub fn len() -> usize {
        3
    }

    #[doc(hidden)]
    #[inline] fn with_xy(self, x: real, y: real) -> Self {
        Self{x, y, w: self.w}
    }
}

impl Vec3 {
    // Calculates the sum of each element
    #[inline] pub fn taxicab_distance(self) -> real {
        self.x + self.y
    }

    /// Negates the vector
    #[inline] pub fn negate(self) -> Self {
        Self::new(-self.x, -self.y, -self.w)
    }

    /// Calculates (self.x + rhs.x, self.y + rhs.y...)
    #[inline] pub fn add_comp(self, rhs: Self) -> Self {
        Self::new(self.x+rhs.x, self.y+rhs.y, self.w+rhs.w)
    }

    /// Calculates (self.x - rhs.x, self.y - rhs.y...)
    #[inline] pub fn sub_comp(self, rhs: Self) -> Self {
        Self::new(self.x-rhs.x, self.y-rhs.y, self.w-rhs.w)
    }

    /// Calculates (self.x * rhs.x, self.y * rhs.y...)
    #[inline] pub fn mul_comp(self, rhs: Self) -> Self {
        Self::new(self.x*rhs.x, self.y*rhs.y, self.w*rhs.w)
    }

    /// Calculates (self.x / rhs.x, self.y / rhs.y...)
    #[inline] pub fn div_comp(self, rhs: Self) -> Self {
        Self::new(self.x/rhs.x, self.y/rhs.y, self.w/rhs.w)
    }

    /// Calculates (self.x * rhs, self.y * rhs...)
    #[inline] pub fn mul_scalar(self, rhs: real) -> Self {
        Self::new(self.x*rhs, self.y*rhs, self.w*rhs)
    }

    /// Calculates (self.x / rhs, self.y / rhs...)
    #[inline] pub fn div_scalar(self, rhs: real) -> Self {
        self.mul_scalar(1.0/rhs)
    }

    /// Calculates (num / self.x, num / self.y...)
    #[inline] pub fn recip_scalar(self, num: real) -> Self {
        Self::new(num/self.x, num/self.y, num/self.w)
    }
}

// ///////////////////// //
// // Index operation // //
// ///////////////////// //
impl core::ops::Index<usize> for Vec3 {
    type Output = real;
    #[inline] fn index(&self, index: usize) -> &real {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.w,
            _ => panic!("Attempt to index out of range")
        }
    }
}

impl core::ops::IndexMut<usize> for Vec3 {
    #[inline] fn index_mut(&mut self, index: usize) -> &mut real {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.w,
            _ => panic!("Attempt to index out of range")
        }
    }
}

impl_vec2d!(Vec3);
const_assert!(core::mem::size_of::<Vec3>() == 3*core::mem::size_of::<real>());
