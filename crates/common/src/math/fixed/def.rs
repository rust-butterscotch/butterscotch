/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::util::internal::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FixedBase<const SCALE: i64>(pub i64);

impl<const SCALE: i64> FixedBase<SCALE> {

    pub const fn new(raw: i64) -> Self {
        Self(raw)
    }

    pub const fn from_i64(units: i64, scale: i64) -> Self {
        Self(dbg_i128_i64_overflow((units as i128 * Self::SCALE_128)/(scale as i128)))
    }

    pub fn from_f64(units: f64) -> Self {
        Self((units * Self::SCALE_64_F) as i64) // TODO overflow
    }

    pub const fn from_fixed<const SCALE_2: i64>(value: FixedBase<SCALE_2>) -> FixedBase<SCALE> {
        FixedBase::<SCALE>::from_i64(value.0, SCALE_2)
    }

    pub fn into_f64(self) -> f64 {
        (self.0 as f64) / Self::SCALE_64_F
    }

    pub const fn into_i64(self, scale: i64) -> i64 {
        (self.0 * scale)/SCALE
    }

    pub const fn into_fixed<const SCALE_2: i64>(self) -> FixedBase<SCALE_2> {
        FixedBase::from_fixed(self)
    }

    pub const fn to_bits(self) -> u64 {
        self.0 as u64
    }

    pub const fn from_bits(v: u64) -> Self {
        Self(v as i64)
    }

    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub const fn to_ne_bytes(self) -> [u8; 8] {
        self.0.to_ne_bytes()
    }

    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(i64::from_be_bytes(bytes))
    }

    pub const fn from_ne_bytes(bytes: [u8; 8]) -> Self {
        Self(i64::from_ne_bytes(bytes))
    }

    pub const fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self(i64::from_le_bytes(bytes))
    }

}

impl<const SCALE: i64> FixedBase<SCALE> {

    pub unsafe fn unchecked_add(self, rhs: Self) -> Self {
        Self(self.0.unchecked_add(rhs.0))
    }

    pub unsafe fn unchecked_sub(self, rhs: Self) -> Self {
        Self(self.0.unchecked_sub(rhs.0))
    }

    pub unsafe fn unchecked_mul(self, rhs: Self) -> Self {
        Self(dbg_i128_i64_overflow((self.0 as i128).unchecked_mul(rhs.0 as i128)/Self::SCALE_128))
    }

}

impl<const SCALE: i64> FixedBase<SCALE> {

    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let result = self.0.overflowing_add(rhs.0);
        (Self(result.0), result.1)
    }

    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let result = self.0.overflowing_sub(rhs.0);
        (Self(result.0), result.1)
    }

    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let result     = (self.0 as i128).overflowing_mul(rhs.0 as i128);
        let normalized = result.0.overflowing_div(Self::SCALE_128);

        let overflow = result.1                         // Overflow on multiply (impossible? 2^63 * 2^63 = 2^126)
                   ||  normalized.1                     // Overflow on divide   (only on bad setups)
                   || (normalized.0 < i64::MIN as i128) // Overflow on cast to i64 (most likely of the bunch)
                   || (normalized.0 > i64::MAX as i128); // Overflow on cast to i64 (most likely of the bunch)

        (Self(dbg_i128_i64_overflow(normalized.0)), overflow)
    }

    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        let result     = (self.0 as i128).overflowing_mul(Self::SCALE_128);
        let normalized = result.0.overflowing_div(rhs.0 as i128);

        let overflow = result.1                          // Overflow on multiply (impossible? 2^63 * 2^63 = 2^126)
                   ||  normalized.1                      // Overflow on divide   (bad divisor)
                   || (normalized.0 < i64::MIN as i128)  // Overflow on cast to i64 (most likely of the bunch)
                   || (normalized.0 > i64::MAX as i128); // Overflow on cast to i64 (most likely of the bunch)

        (Self(dbg_i128_i64_overflow(result.0)), overflow)
    }

}

impl<const SCALE: i64> FixedBase<SCALE> {

    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    pub fn wrapping_mul(self, rhs: Self) -> Self {
        Self(dbg_i128_i64_overflow((self.0 as i128).wrapping_mul(rhs.0 as i128).wrapping_div(Self::SCALE_128)))
    }

}

impl<const SCALE: i64> FixedBase<SCALE> {
    pub const ONE: Self = Self::from_i64(1, 1);
    pub const ZERO: Self = Self(0);

    pub const DIGITS: u32 = 19;
    pub const EPSILON: Self = Self(1);

    pub const MIN: Self = Self(i64::MIN);
    pub const MIN_POSITIVE: Self = Self(1);
    pub const MAX: Self = Self(i64::MAX);

    //pub const MIN_EXP: Self = Self::MIN.log2().floor() + Self::ONE; // TODO log2 not const
    //pub const MAX_EXP: Self = Self::MAX.log2().floor();             // TODO log2 not const

    //pub const MIN_10_EXP: Self = Self::MIN.log10().floor() + Self::ONE; // TODO log10 not const
    //pub const MAX_10_EXP: Self = Self::MAX.log10().floor();             // TODO log10 not const

    // Mostly for internal use
    pub const SCALE_64_F: f64 = SCALE as f64;
    pub const SCALE_64:      i64  =  SCALE;
    pub const SCALE_128:     i128 =  SCALE as i128;
    pub const SCALE_128_SQR: i128 = (SCALE as i128) * (SCALE as i128);
}

// pub type Fixed = FixedBase::<1_000_000>;
