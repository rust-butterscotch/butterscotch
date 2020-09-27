/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::*;

const LOOP_TEST_COUNT: i64 = 128;

type Fixed = FixedBase<1_000_000_000>;

#[test]
pub fn test_fixed_ops() {
    for i in -LOOP_TEST_COUNT..=LOOP_TEST_COUNT {
        for j in -LOOP_TEST_COUNT..=LOOP_TEST_COUNT {
            test_int_add::<{Fixed::SCALE_64}>(i, j);
            test_int_sub::<{Fixed::SCALE_64}>(i, j);
            test_int_mul::<{Fixed::SCALE_64}>(i, j);
            test_int_div::<{Fixed::SCALE_64}>(i, j);
        }
    }
}

fn test_int_add<const SCALE: i64>(i: i64, j: i64) {
    let l = FixedBase::<SCALE>::from(i);
    let r = FixedBase::<SCALE>::from(j);
    let a = l+r;
    let e = FixedBase::<SCALE>::from(i+j);
    assert_eq!(l+r, e, "{}+{} = {}, should be {}", l.into_i64(1), r.into_i64(1), a.into_i64(1), e.into_i64(1));
}

fn test_int_sub<const SCALE: i64>(i: i64, j: i64) {
    let l = FixedBase::<SCALE>::from(i);
    let r = FixedBase::<SCALE>::from(j);
    let a = l-r;
    let e = FixedBase::<SCALE>::from(i-j);
    assert_eq!(l-r, e, "{}+{} = {}, should be {}", l.into_i64(1), r.into_i64(1), a.into_i64(1), e.into_i64(1));
}

fn test_int_mul<const SCALE: i64>(i: i64, j: i64) {
    let l = FixedBase::<SCALE>::from(i);
    let r = FixedBase::<SCALE>::from(j);
    let a = l*r;
    let e = FixedBase::<SCALE>::from(i*j);
    assert_eq!(l*r, e, "{}*{} = {}, should be {}", l.into_i64(1), r.into_i64(1), a.into_i64(1), e.into_i64(1));
}

fn test_int_div<const SCALE: i64>(i: i64, j: i64) {
    if (i == 0) || (j == 0) { return; } // Skip 0 divisions
    if (i/j)*j != i { return; } // Skip non-integer divisions
    let l = FixedBase::<SCALE>::from(i);
    let r = FixedBase::<SCALE>::from(j);
    let a = l/r;
    let e = FixedBase::<SCALE>::from(i/j);
    assert_eq!(l/r, e, "{}/{} = {}, should be {}", l.into_i64(1), r.into_i64(1), a.into_i64(1), e.into_i64(1));
}