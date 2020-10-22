/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

macro_rules! impl_vec2d { ($s:ident) => {
    impl $s {
        /// Creates a new unit-length vector pointing in the given angle ccw from right
        #[inline] pub fn from_angle(a: real) -> Self {
            let v = a.sin_cos();
            Self::ZERO.with_xy(v.1, v.0)
        }

        /// Rotates the vector a quarter turn counter-clockwise
        #[inline] pub fn perp_ccw(self) -> Self {
            self.with_xy(-self.y,  self.x)
        }

        /// Rotates the vector a quarter turn clockwise
        #[inline] pub fn perp_cw(self)  -> Self {
            self.with_xy( self.y, -self.x)
        }

        /// Rotates this vector by the given angle in radians
        #[inline] pub fn rotate(self, angle: real) -> Self {
            self.rotate_by(Self::from_angle(angle))
        }

        /// Calculates the dot product between this vector and another
        #[inline] pub fn dot(self, other: Self) -> real {
            self.x*other.x + self.y*other.y
        }

        /// Attempts to normalize the vector to the given length.
        /// Returns `None` if this vector is zero-length
        #[inline] pub fn normalize_to(self, num: real) -> Option<Self> {
            let inv_mag = num/self.length();
            match inv_mag.is_finite() {
                true  => Some(self.mul_scalar(inv_mag)),
                false => None,
            }
        }

        /// Attempts to normalize the vector to unit length.
        /// Returns `None` if this vector is zero-length
        #[inline] pub fn normalize(self) -> Option<Self> {
            self.normalize_to(1.0)
        }

        /// Calculates the length of the vector
        #[inline] pub fn length(self) -> real {
            self.length_sqr().sqrt()
        }

        /// Calculates the squared length of the vector
        #[inline] pub fn length_sqr(self) -> real {
            self.dot(self)
        }

        /// Attempts to calculate the angle of a vector ccw from +x
        /// Does not require a normalized vector
        /// Returns `None` if this vector is zero-length
        #[inline] pub fn angle(self) -> Option<real> {
            match (self.x != 0.0) || (self.y != 0.0) {
                true  => Some(self.angle_fast()),
                false => None,
            }
        }

        /// Attempts to reflect this vector over the given normal
        /// Returns `None` if the normal is zero-length
        #[inline] pub fn reflect(self, normal: Self) -> Option<Self> {
            match normal.normalize() {
                Some(n) => Some(self.reflect_by(n)),
                None    => None,
            }
        }
    }

    // ///////////////////////// //
    // // "Unsafe" operations // //
    // ///////////////////////// //

    impl $s {
        /// Normalizes the vector to the given length.
        /// `self` should not be zero-length
        #[inline] pub fn normalize_to_fast(self, num: real) -> Self {
            self.mul_scalar(num/self.length())
        }

        /// Normalizes the vector to unit length.
        /// `self` should not be zero-length
        #[inline] pub fn normalize_fast(self) -> Self {
            self.normalize_to_fast(1.0)
        }

        /// Calculates the angle of the vector in radians
        /// If `self` is zero-length, then this will return zero
        #[inline] pub fn angle_fast(self) -> real {
            self.y.atan2(self.x)
        }

        /// Reflects this vector over the given normal
        /// `normal` should not be zero-length
        #[inline] pub fn reflect_fast(self, normal: Self) -> Self {
            self.reflect_by(normal.normalize_fast())
        }

        /// Reflects this vector over the given normal
        /// `normal` should already be a unit-length vector
        #[inline] pub fn reflect_by(self, normal: Self) -> Self {
            self.sub_comp(normal.mul_scalar(2.0*self.dot(normal)))
        }

        /// Rotates this vector by the assumed-to-be normalized vector
        /// This is equivalent to a matrix rotation b
        /// | +x | -y |
        /// | +y | +x |
        #[inline] pub fn rotate_by(self, other: Self) -> Self {
            self.with_xy(
                self.x*other.x - self.y*other.y,
                self.y*other.y + self.x*other.x
            )
        }
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
}}