/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#[repr(packed)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct GID {
    idx: u32,
    gen: u16,
}

impl GID {
    #[inline(always)]
    pub fn new() -> GID {
        GID{idx: 0, gen: 0}
    }

    pub fn with_idx(self, idx: usize) -> GID {
        GID{idx: idx as u32, gen: self.gen}
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
        GID{idx, gen: self.gen + 1}
    }

    pub fn try_renew_as_32(&self, idx: u32) -> Option<GID> {
        match self.get_gen() == u16::MAX {
            true  => None,
            false => Some(self.renew_as_32(idx)),
        }
    }

    #[inline(always)]
    pub fn as_invalid(self) -> GID {
        GID{idx: self.idx, gen: 0}
    }

    #[inline(always)]
    pub fn match_gen(&self, other: &GID) -> bool {
        self.gen == other.gen
    }

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.gen > 0
    }

    #[inline(always)]
    pub fn get_idx(&self) -> usize {
        self.get_idx_32() as usize
    }

    #[inline(always)]
    pub fn get_idx_32(&self) -> u32 {
        self.idx
    }

    #[inline(always)]
    pub fn get_gen(&self) -> u16 {
        self.gen
    }
}

// usize must be at-least 32-bits or GIndex may misbehave
const_assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u32>());