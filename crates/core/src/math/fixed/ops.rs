/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;
use super::util::internal::*;

impl<const SCALE: i64> Into<f64> for FixedBase<SCALE> { fn into(self)   -> f64  { self.into_f64()      }}
impl<const SCALE: i64> From<f64> for FixedBase<SCALE> { fn from(v: f64) -> Self { Self::from_f64(v)    }}

impl<const SCALE_L: i64> FixedBase<SCALE_L> {

    pub const fn mul_mul_add<const SCALE_R: i64>(self, rhs1: FixedBase<SCALE_R>, lhs2: FixedBase<SCALE_L>, rhs2: FixedBase<SCALE_L>) -> Self {
        Self(dbg_i128_i64_overflow(
            (i128_mul(self.0, rhs1.0) + i128_mul(lhs2.0, rhs2.0))/(SCALE_R as i128)
        ))
    }

    pub const fn mul_mul_sub<const SCALE_R: i64>(self, rhs1: FixedBase<SCALE_R>, lhs2: FixedBase<SCALE_L>, rhs2: FixedBase<SCALE_L>) -> Self {
        Self(dbg_i128_i64_overflow(
            (i128_mul(self.0, rhs1.0) - i128_mul(lhs2.0, rhs2.0))/(SCALE_R as i128)
        ))
    }

    pub const fn mul_div<const SCALE_R: i64, const SCALE_D: i64>(self, rhs: FixedBase<SCALE_R>, div: FixedBase<SCALE_D>) -> Self {
        assert!(
            (SCALE_D as i128).abs() <= i128::MAX/i128_mul(i64::MAX, i64::MAX),
            "mul_div: Denominator scale is a max of 2, otherwise the intermediate values could overflow."
        );
        unsafe { self.mul_div_unchecked(rhs, div) }
    }

    pub const unsafe fn mul_div_unchecked<const SCALE_R: i64, const SCALE_D: i64>(self, rhs: FixedBase<SCALE_R>, div: FixedBase<SCALE_D>) -> Self {
        Self(dbg_i128_i64_overflow(
            (i128_mul(self.0, rhs.0) * (SCALE_D as i128))/i128_mul(div.0, SCALE_R)
        ))
    }
}


// ///////// //
// // Add // //
// ///////// //

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Add<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;

    fn add(self, rhs: FixedBase<SCALE_R>) -> Self::Output {
        Self(dbg_i128_i64_overflow(
            (i128_mul(SCALE_R, self.0) + i128_mul(SCALE_L, rhs.0))/(SCALE_R as i128)
        ))
    }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::AddAssign<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    fn add_assign(&mut self, rhs: FixedBase<SCALE_R>) { *self = *self + rhs; }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Add<&FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;
    fn add(self, rhs: &FixedBase<SCALE_R>) -> Self::Output { self + *rhs }
}

// ///////// //
// // Sub // //
// ///////// //

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Sub<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;

    fn sub(self, rhs: FixedBase<SCALE_R>) -> Self::Output {
        Self(dbg_i128_i64_overflow(
            (i128_mul(SCALE_R, self.0) - i128_mul(SCALE_L, rhs.0))/(SCALE_R as i128)
        ))
    }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::SubAssign<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    fn sub_assign(&mut self, rhs: FixedBase<SCALE_R>) { *self = *self - rhs; }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Sub<&FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;
    fn sub(self, rhs: &FixedBase<SCALE_R>) -> Self::Output { self - *rhs }
}

// ///////// //
// // Mul // //
// ///////// //

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Mul<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;

    fn mul(self, rhs: FixedBase<SCALE_R>) -> Self::Output {
        Self(dbg_i128_i64_overflow(
            i128_mul(self.0, rhs.0)/(SCALE_R as i128)
        ))
    }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::MulAssign<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    fn mul_assign(&mut self, rhs: FixedBase<SCALE_R>) { *self = *self * rhs; }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Mul<&FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;
    fn mul(self, rhs: &FixedBase<SCALE_R>) -> Self::Output { self * *rhs }
}

// ///////// //
// // Div // //
// ///////// //

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Div<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;

    fn div(self, rhs: FixedBase<SCALE_R>) -> Self::Output {
        Self(dbg_i128_i64_overflow(
            i128_mul(self.0, SCALE_R)/(rhs.0 as i128)
        ))
    }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::DivAssign<FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    fn div_assign(&mut self, rhs: FixedBase<SCALE_R>) { *self = *self / rhs; }
}

impl<const SCALE_L: i64, const SCALE_R: i64> std::ops::Div<&FixedBase<SCALE_R>> for FixedBase<SCALE_L> {
    type Output = Self;
    fn div(self, rhs: &FixedBase<SCALE_R>) -> Self::Output { self / *rhs }
}


// ///////////// //
// // Int Ops // //
// ///////////// //

macro_rules! impl_int_ops {
    ($int_type:ty) => {
        // ///////////// //
        // // Mul Int // //
        // ///////////// //

        impl<const SCALE: i64> std::ops::Mul<$int_type> for FixedBase<SCALE> {
            type Output = FixedBase<SCALE>;
            fn mul(self, rhs: $int_type) -> Self::Output {
                FixedBase::<SCALE>(((self.0 as i128)*(rhs as i128)) as i64)
            }
        }

        impl<const SCALE: i64> std::ops::Mul<FixedBase<SCALE>> for $int_type {
            type Output = FixedBase<SCALE>;
            fn mul(self, rhs: FixedBase<SCALE>) -> Self::Output {
                FixedBase::<SCALE>(((self as i128)*(rhs.0 as i128)) as i64)
            }
        }

        impl<const SCALE: i64> std::ops::MulAssign<$int_type> for FixedBase<SCALE> {
            fn mul_assign(&mut self, rhs: $int_type) {
                *self = *self * rhs;
            }
        }

        impl<const SCALE: i64> std::ops::Mul<&$int_type> for FixedBase<SCALE> {
            type Output = Self;
            fn mul(self, rhs: &$int_type) -> Self::Output {
                self * *rhs
            }
        }

        // ///////////// //
        // // Div Int // //
        // ///////////// //

        impl<const SCALE: i64> std::ops::Div<$int_type> for FixedBase<SCALE> {
            type Output = FixedBase<SCALE>;
            fn div(self, rhs: $int_type) -> Self::Output {
                FixedBase::<SCALE>(((self.0 as i128)/(rhs as i128)) as i64)
            }
        }

        impl<const SCALE: i64> std::ops::DivAssign<$int_type> for FixedBase<SCALE> {
            fn div_assign(&mut self, rhs: $int_type) { *self = *self / rhs; }
        }

        impl<const SCALE: i64> std::ops::Div<&$int_type> for FixedBase<SCALE> {
            type Output = Self;
            fn div(self, rhs: &$int_type) -> Self::Output { self / *rhs }
        }

        // ///////////// //
        // // To/From // //
        // ///////////// //

        // TODO try from // try into

        impl<const SCALE: i64> Into<$int_type> for FixedBase<SCALE> {
            fn into(self) -> $int_type  {
                self.into_i64(1) as $int_type
            }
        }

        impl<const SCALE: i64> From<$int_type> for FixedBase<SCALE> {
            fn from(v: $int_type) -> Self {
                Self::from_i64(v as i64, 1)
            }
        }
    };
}

impl_int_ops!(i128);
impl_int_ops!(i64);
impl_int_ops!(i32);
impl_int_ops!(i16);
impl_int_ops!(i8);

impl_int_ops!(u128);
impl_int_ops!(u64);
impl_int_ops!(u32);
impl_int_ops!(u16);
impl_int_ops!(u8);

impl_int_ops!(isize);
impl_int_ops!(usize);
















//