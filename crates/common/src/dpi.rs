/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::math::Vec2;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PixelsUser(pub Vec2);

impl PixelsUser {
    pub fn to_unscaled(self, scale: f64) -> PixelsRaw {
        assert!(scale > 0.0, "Scale must be > 0");
        PixelsRaw(self.0/scale)
    }

    pub fn raw_u64(self) -> (u64, u64) {
        return (self.0.x.round() as u64, self.0.y.round() as u64);
    }
}

impl Into<(u64, u64)> for PixelsUser {
    fn into(self) -> (u64, u64) { self.raw_u64() }
}

impl Into<Vec2> for PixelsUser {
    fn into(self) -> Vec2 { self.0 }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PixelsRaw(pub Vec2);

impl PixelsRaw {
    pub fn to_user_scale(self, scale: f64) -> PixelsUser {
        assert!(scale > 0.0, "Scale must be > 0");
        PixelsUser(self.0*scale)
    }

    pub fn raw_u64(self) -> (u64, u64) {
        return (self.0.x.round() as u64, self.0.y.round() as u64);
    }
}

impl Into<(u64, u64)> for PixelsRaw {
    fn into(self) -> (u64, u64) { self.raw_u64() }
}

impl Into<Vec2> for PixelsRaw {
    fn into(self) -> Vec2 { self.0 }
}

pub enum Pixels {
    User(PixelsUser),
    Raw(PixelsRaw)
}