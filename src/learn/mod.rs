/// Learning.

use data::*;

pub type NetIn = BitVec;

pub type NetOut = BitVec;

impl NetIn {
    pub fn from_game(game: &Game) -> NetIn {
        let mut bv = BitVec::with_capacity(81);
        bv as NetIn
    }
}
