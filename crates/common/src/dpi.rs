/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::math::Vec2;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Pixels {
    Physical(Vec2),
    Logical (Vec2),
}

impl Pixels {
    pub fn to_physical(self, scale: f64) -> Pixels {
        assert!(scale > 0.0, "Scale must be >= 0");
        match self {
            Pixels::Physical(_) => self,
            Pixels::Logical (v) => Pixels::Physical(v*scale),
        }
    }

    pub fn to_logical(self, scale: f64) -> Pixels {
        assert!(scale > 0.0, "Scale must be >= 0");
        match self {
            Pixels::Physical(v) => Pixels::Physical(v/scale),
            Pixels::Logical (_) => self,
        }
    }

    pub fn raw_vec(self) -> Vec2 {
        match self {
            Pixels::Logical (v) => v,
            Pixels::Physical(v) => v,
        }
    }

    pub fn raw_u64(self) -> (u64, u64) {
        let tmp = self.raw_vec();
        return (tmp.x.round() as u64, tmp.y.round() as u64);
    }
}

impl Into<(u64, u64)> for Pixels {
    fn into(self) -> (u64, u64) { self.raw_u64() }
}

impl Into<Vec2> for Pixels {
    fn into(self) -> Vec2 { self.raw_vec() }
}