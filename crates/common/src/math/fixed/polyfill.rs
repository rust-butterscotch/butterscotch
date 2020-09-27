/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;

// TODO convert const integer op
#[allow(non_camel_case_types)]
type polyfill_t = f64;

// ///////// //
// // Log // //
// ///////// //

// These don't come up /that/ often in game development, probably only worth converting
// if we need it. There might be some kind of shortcut for log and the like, possibly
// along the line of log2(x @ scale) = #leading zeros x - log2(scale) - though probably
// not entirely correct.  However, if we want const and exact behaviour, then we will need
// to convert them.
impl<const SCALE: i64> FixedBase<SCALE> {
    pub fn exp   (self) -> Self { Into::<polyfill_t>::into(self).exp().into() }
    pub fn exp2  (self) -> Self { Into::<polyfill_t>::into(self).exp2().into() }
    pub fn exp_m1(self) -> Self { Into::<polyfill_t>::into(self).exp_m1().into() }

    pub fn ln   (self) -> Self { Into::<polyfill_t>::into(self).ln   ().into() }
    pub fn ln_1p(self) -> Self { Into::<polyfill_t>::into(self).ln_1p().into() }

    pub fn log  (self, base: Self) -> Self { Into::<polyfill_t>::into(self).log(base.into()).into() }
    pub fn log2 (self)             -> Self { Into::<polyfill_t>::into(self).log2 ().into() }
    pub fn log10(self)             -> Self { Into::<polyfill_t>::into(self).log10().into() }
}

// /////////// //
// // Power // //
// /////////// //

// Powf might be hard, but, we can probably implement a naive sqrt/cbrt using a binary
// search.
impl<const SCALE: i64> FixedBase<SCALE> {
    pub fn cbrt(self) -> Self { Into::<polyfill_t>::into(self).cbrt().into() }
    pub fn sqrt(self) -> Self { Into::<polyfill_t>::into(self).sqrt().into() }
    pub fn powf(self, p: Self) -> Self { Into::<polyfill_t>::into(self).powf(p.into()).into() }
}

// ////////////////// //
// // Trigonometry // //
// ////////////////// //

// Trig, given how close these numbers are to 0, should be more accurate than doing it in fixed point
// We might consider implementing wrapping in fixed point, to handle cases where we're calling something
// like sin(1_000_000_000_000.123_456). However, if we want const and exact behaviour, then we will need
// to convert them.
impl<const SCALE: i64> FixedBase<SCALE> {
    pub fn sin_cos(self) -> (Self, Self) {
        let r = Into::<polyfill_t>::into(self).sin_cos();
        (r.0.into(), r.1.into())
    }

    pub fn sin(self) -> Self { Into::<polyfill_t>::into(self).sin().into() }
    pub fn cos(self) -> Self { Into::<polyfill_t>::into(self).cos().into() }
    pub fn tan(self) -> Self { Into::<polyfill_t>::into(self).tan().into() }

    pub fn sinh(self) -> Self { Into::<polyfill_t>::into(self).sinh().into() }
    pub fn cosh(self) -> Self { Into::<polyfill_t>::into(self).cosh().into() }
    pub fn tanh(self) -> Self { Into::<polyfill_t>::into(self).tanh().into() }

    pub fn asin(self) -> Self { Into::<polyfill_t>::into(self).asin().into() }
    pub fn acos(self) -> Self { Into::<polyfill_t>::into(self).acos().into() }
    pub fn atan(self) -> Self { Into::<polyfill_t>::into(self).atan().into() }

    pub fn atan2(self, x: Self) -> Self { Into::<polyfill_t>::into(self).atan2(x.into()).into() }

    pub fn asinh(self) -> Self { Into::<polyfill_t>::into(self).asinh().into() }
    pub fn acosh(self) -> Self { Into::<polyfill_t>::into(self).acosh().into() }
    pub fn atanh(self) -> Self { Into::<polyfill_t>::into(self).atanh().into() }
}
