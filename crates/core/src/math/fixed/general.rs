/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;
use super::util::internal::*;

// ////////// //
// // Sign // //
// ////////// //
impl<const SCALE: i64> FixedBase<SCALE> {

    pub const fn abs(self) -> Self {
        Self(self.0)
    }

    pub const fn copysign(self, other: Self) -> Self {
        Self(self.0.abs() * if other.0 < 0 { -1 } else { 1 })
    }

    pub const fn signum(self) -> Self {
        Self(self.0.signum() * SCALE)
    }

    pub const fn is_sign_positive(self) -> bool {
        self.0 >= 0
    }

    pub const fn is_sign_negative(self) -> bool {
        self.0 < 0
    }
}

// ////////////// //
// // Division // //
// ////////////// //

impl<const SCALE: i64> FixedBase<SCALE> {
    pub const fn fract(self) -> Self {
        Self(self.0 - ((self.0/SCALE)*SCALE))
    }

    pub const fn recip(self) -> Self {
        Self(dbg_i128_i64_overflow(Self::SCALE_128_SQR/(self.0 as i128)))
    }

    pub const fn rem_euclid(self, rhs: Self) -> Self {
        Self(self.0.rem_euclid(rhs.0))
    }

    pub const fn div_euclid(self, rhs: Self) -> Self {
        Self(self.0.div_euclid(rhs.0))
    }
}

// ///////////// //
// // Convert // //
// ///////////// //

impl<const SCALE: i64> FixedBase<SCALE> {
    //TODO pub fn to_degrees(self) -> Unit {}
    //TODO pub fn to_radians(self) -> Unit {}
}

// /////////// //
// // Power // //
// /////////// //

impl<const SCALE: i64> FixedBase<SCALE> {

    pub const fn powi(self, p: i32) -> Self {
        Self(f64_saturate_i64(match (self.0, p) {
            (_, 0) => 1i128,
            (v, _) if v < 0 =>                 Self::powi_i128(SCALE as i128, v as i128, -p as u32),
            (v, _) if v > 0 => (SCALE as i128)/Self::powi_i128(SCALE as i128, v as i128,  p as u32),
            (_, _) => 0i128,
        }))
    }

    const fn powi_i128(result: i128, value: i128, exponent: u32) -> i128 {
        match exponent {
            0 => result,
            _ => Self::powi_i128(result.saturating_mul(value)/(SCALE as i128), value, exponent-1)
        }
    }
}

// ////////////// //
// // Rounding // //
// ////////////// //

impl<const SCALE: i64> FixedBase<SCALE> {
    pub const fn round(self) -> Self {
        Self(match self.is_sign_positive() {
            true  => ((self.0 + SCALE/2)/SCALE)*SCALE,
            false => ((self.0 - SCALE/2)/SCALE)*SCALE
        })
    }

    pub const fn ceil(self) -> Self {
        Self(((self.0 + (SCALE-1))/SCALE)*SCALE)
    }

    pub const fn floor(self) -> Self {
        Self(((self.0 - (SCALE-1))/SCALE)*SCALE)
    }

    pub const fn trunc(self) -> Self {
        Self((self.0/SCALE)*SCALE)
    }
}

// ////////////////// //
// // Trigonometry // //
// ////////////////// //

impl<const SCALE: i64> FixedBase<SCALE> {
    pub fn hypot(self, other: Self) -> Self {
        let s_v = self.0 as i128;
        let o_v = other.0 as i128;
        Self(dbg_i128_i64_overflow((s_v.pow(2) + o_v.pow(2))/Self::SCALE_128_SQR)).sqrt()
    }
}