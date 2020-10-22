/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

macro_rules! impl_mat2d { ($s:ident, $v:ident) => {

    impl $s {

        /// Construct a rotation matrix that will rotate a vector ccw by the given angle
        #[inline] pub fn new_rotation_from_angle(a: real) -> Self {
            Self::new_rotation_from_norm(Vec2::from_angle(a))
        }

        /// Constructs a rotation matrix that will rotate a vector ccw from +x to the target point
        /// If `v` is zero-length, then `none` will be returned
        #[inline] pub fn new_rotation_from_dir(v: Vec2) -> Option<Self> {
            match v.normalize() {
                Some(v) => Some(Self::new_rotation_from_norm(v)),
                None    => None,
            }
        }

        /// Construct a reflection matrix that will reflect a vector about a line along +x rotated ccw by the given angle
        #[inline] pub fn new_reflection(a: real) -> Self {
            Self::new_reflection_from_dnorm(Vec2::from_angle(2.0*a))
        }

        /// Construct a reflection matrix that will reflect a vector about a line
        /// If `v` is zero-length, then `none` will be returned
        #[inline] pub fn new_reflection_over_dir(v: Vec2) -> Option<Self> {
            match v.normalize() {
                Some(v) => Some(Self::new_reflection_from_dnorm(v.rotate_by(v))),
                None    => None,
            }
        }
    }

    impl $s {
        /// Returns a copy of the row at the given index
        #[inline] pub fn row(self, i: usize) -> $v {
            self.0[i]
        }

        /// Returns a copy of the col at the given index
        #[inline] pub fn col(self, i: usize) -> $v {
            let mut result: $v = Default::default();
            for j in 0..self.0.len() { result[i] = self[j][i]; }
            result
        }
    }

    // ///////////// //
    // // Default // //
    // ///////////// //

    impl Default for $s {
        #[inline] fn default() -> Self { Self::IDENTITY }
    }

    // ///////////////////// //
    // // Index operation // //
    // ///////////////////// //
    impl core::ops::Index<usize> for $s {
        type Output = $v;
        #[inline(always)] fn index(&self, i: usize) -> &$v { &self.0[i] }
    }

    impl core::ops::IndexMut<usize> for $s {
        #[inline(always)] fn index_mut(&mut self, i: usize) -> &mut $v { &mut self.0[i] }
    }

    // /////////////////// //
    // // Add operation // //
    // /////////////////// //
    impl core::ops::Add<$s> for $s {
        type Output = $s;
        #[inline] fn add(self, rhs: $s) -> $s { self.add_comp(rhs) }
    }

    impl core::ops::AddAssign<$s> for $s {
        #[inline] fn add_assign(&mut self, rhs: $s) { *self = *self + rhs; }
    }

    // /////////////////// //
    // // Sub operation // //
    // /////////////////// //
    impl core::ops::Sub<$s> for $s {
        type Output = $s;
        #[inline] fn sub(self, rhs: $s) -> $s { self.sub_comp(rhs) }
    }

    impl core::ops::SubAssign<$s> for $s {
        #[inline] fn sub_assign(&mut self, rhs: $s) { *self = *self - rhs; }
    }

    // /////////////////////// //
    // // Mul mat operation // //
    // /////////////////////// //
    impl core::ops::Mul<$s> for $s {
        type Output = $s;
        #[inline] fn mul(self, rhs: $s) -> $s { self.mul_mat(rhs) }
    }

    impl core::ops::MulAssign<$s> for $s {
        #[inline] fn mul_assign(&mut self, rhs: $s) { *self = *self * rhs; }
    }

    // /////////////////////// //
    // // Mul vec operation // //
    // /////////////////////// //
    impl core::ops::Mul<$v> for $s {
        type Output = $v;
        #[inline] fn mul(self, rhs: $v) -> $v { self.mul_vec(rhs) }
    }

    impl core::ops::Mul<$s> for $v {
        type Output = $v;
        #[inline] fn mul(self, rhs: $s) -> $v { rhs.mul_vec(self) }
    }

    impl core::ops::MulAssign<$s> for $v {
        #[inline] fn mul_assign(&mut self, rhs: $s) { *self = rhs * *self; }
    }

    // /////////////////// //
    // // Mul operation // //
    // /////////////////// //
    impl core::ops::Mul<real> for $s {
        type Output = $s;
        #[inline] fn mul(self, rhs: real) -> $s { self.mul_scalar(rhs) }
    }

    impl core::ops::Mul<$s> for real {
        type Output = $s;
        #[inline] fn mul(self, rhs: $s) -> $s { rhs.mul_scalar(self) }
    }

    impl core::ops::MulAssign<real> for $s {
        #[inline] fn mul_assign(&mut self, rhs: real) { *self = *self * rhs; }
    }

    // /////////////////// //
    // // Div operation // //
    // /////////////////// //
    impl core::ops::Div<real> for $s {
        type Output = $s;
        #[inline] fn div(self, rhs: real) -> $s { self.div_scalar(rhs) }
    }

    impl core::ops::Div<$s> for real {
        type Output = $s;
        #[inline] fn div(self, rhs: $s) -> $s { rhs.recip_scalar(self) }
    }

    impl core::ops::DivAssign<real> for $s {
        #[inline] fn div_assign(&mut self, rhs: real) { *self = *self / rhs; }
    }

};}