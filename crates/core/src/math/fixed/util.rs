
pub mod internal {

    #[inline(always)] pub const fn i128_add(l: i64, r: i64) -> i128 { (l as i128) + (r as i128) }
    #[inline(always)] pub const fn i128_sub(l: i64, r: i64) -> i128 { (l as i128) - (r as i128) }
    #[inline(always)] pub const fn i128_mul(l: i64, r: i64) -> i128 { (l as i128) * (r as i128) }
    #[inline(always)] pub const fn i128_div(l: i64, r: i64) -> i128 { (l as i128) / (r as i128) }

    #[inline(always)]
    pub const fn dbg_i128_i64_overflow(v: i128) -> i64 {
        debug_assert!(v <= (i64::MAX as i128), "overflow when converting i128 to i64");
        debug_assert!(v >= (i64::MIN as i128), "underflow when converting i128 to i64");
        v as i64
    }

    pub const fn f64_saturate_i64(v: i128) -> i64 {
        match v.checked_abs() {
            Some(r) => match r < (1 << 63) {
                true  => v as i64,
                false => i64::MAX*(v.signum() as i64),
            },
            None => i64::MIN,
        }
    }
}