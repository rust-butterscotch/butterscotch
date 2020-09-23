/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

pub const GID_IDX_MASK:      u32 = 0x00FFFFFF;
pub const GID_GEN_MASK:      u32 = 0xFF000000;
pub const GID_GEN_VALID_MIN: u32 = 0x01000000;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct GID(u32);

impl GID {
    #[inline(always)]
    pub fn new() -> GID {
        GID(0)
    }

    pub fn with_idx(self, idx: usize) -> GID {
        if (idx & !(GID_IDX_MASK as usize)) != 0 { panic!("GIndex idx overflow"); }
        GID(self.get_gen() | (idx as u32))
    }

    pub fn renew_as(self, idx: usize) -> GID {
        self.renew_as_32(idx as u32)
    }

    pub fn try_renew_as(&self, idx: usize) -> Option<GID> {
        self.try_renew_as_32(idx as u32)
    }

    pub fn with_idx_32(self, idx: u32) -> GID {
        self.with_idx(idx as usize)
    }

    pub fn renew_as_32(self, idx: u32) -> GID {
        if (idx & !GID_IDX_MASK) != 0 { panic!("GIndex idx overflow"); }
        match self.get_gen().checked_add(GID_GEN_VALID_MIN) {
            Some(g) => GID(                g | idx),
            None    => GID(GID_GEN_VALID_MIN | idx)
        }
    }

    pub fn try_renew_as_32(&self, idx: u32) -> Option<GID> {
        if (idx & !GID_IDX_MASK) != 0 { panic!("GIndex idx overflow"); }
        match self.get_gen().checked_add(GID_GEN_VALID_MIN) {
            Some(g) => Some(GID(g | idx)),
            None    => None
        }
    }

    #[inline(always)]
    pub fn as_invalid(self) -> GID {
        GID(self.get_idx() as u32)
    }

    #[inline(always)]
    pub fn match_gen(&self, other: &GID) -> bool {
        ((self.0 ^ other.0) & GID_GEN_MASK) == 0
    }

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.0 >= GID_GEN_VALID_MIN
    }

    #[inline(always)]
    pub fn get_idx(&self) -> usize {
        self.get_idx_32() as usize
    }

    #[inline(always)]
    pub fn get_idx_32(&self) -> u32 {
        self.0 & GID_IDX_MASK
    }

    #[inline(always)]
    pub fn get_gen(&self) -> u32 {
        self.0 & GID_GEN_MASK
    }
}

// usize must be at-least 32-bits or GIndex may misbehave
const_assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u32>());