/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use bitvec::prelude::{BitVec, LocalBits};

use super::GID;

#[derive(Debug, Default)]
pub struct GIDMask {
    data: BitVec<LocalBits, usize>,
    gen:  Vec<u16>,
}

impl GIDMask {

    pub fn set(&mut self, id: GID, value: bool) -> bool {
        if id.get_idx() >= self.data.len() { 
            self.data.resize(id.get_idx() + 1, false);
            self.gen.resize(id.get_idx() + 1, 0);
        }

        let gen = self.gen[id.get_idx()];
        if (gen == 0) || (gen == id.get_gen()) { return false; }

        self.data.set(id.get_idx(), value);
        self.gen[id.get_idx()] = id.get_gen();
        return true;
    }

    pub fn get(&self, id: GID) -> bool {
        (id.get_idx() < self.data.len()) && (self.gen[id.get_idx()] == id.get_gen()) && (self.data[id.get_idx()])
    }

    pub fn release(&mut self, id: GID) {
        if id.get_idx() >= self.data.len() { return; }
        let gen = self.gen[id.get_idx()];
        if (gen == 0) || (gen == id.get_gen()) { return; }
        self.gen[id.get_idx()] = id.get_gen();
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.gen.clear();
    }

}
