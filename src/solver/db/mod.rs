/// Database.

use data::Game;
use bit_vec::BitVec;

#[allow(unused_variables)]
pub fn create_solutions_table() {
    let s = "CREATE TABLE solutions (
  game BIT(152) PRIMARY KEY,
  play SMALLINT,
  winner SMALLINT
    CONSTRAINT winner_constraint
    CHECK (winner >= 0 AND winner <= 2),
  turns SMALLINT NOT NULL
    CONSTRAINT turns_constraint
    CHECK (turns >= 0)
);";
    unimplemented!()
}

// TODO: Verify correctness.
impl Game {
    /// Converts a Game to a 152-bit BitVec.
    pub fn as_bit_vec(&self) -> BitVec {
        BitVec::from_bytes(&[
            (self.board.sboards[0].encoding & 0xFF) as u8,
            (self.board.sboards[0].encoding >> 8) as u8,
            (self.board.sboards[1].encoding & 0xFF) as u8,
            (self.board.sboards[1].encoding >> 8) as u8,
            (self.board.sboards[2].encoding & 0xFF) as u8,
            (self.board.sboards[2].encoding >> 8) as u8,
            (self.board.sboards[3].encoding & 0xFF) as u8,
            (self.board.sboards[3].encoding >> 8) as u8,
            (self.board.sboards[4].encoding & 0xFF) as u8,
            (self.board.sboards[4].encoding >> 8) as u8,
            (self.board.sboards[5].encoding & 0xFF) as u8,
            (self.board.sboards[5].encoding >> 8) as u8,
            (self.board.sboards[6].encoding & 0xFF) as u8,
            (self.board.sboards[6].encoding >> 8) as u8,
            (self.board.sboards[7].encoding & 0xFF) as u8,
            (self.board.sboards[7].encoding >> 8) as u8,
            (self.board.sboards[8].encoding & 0xFF) as u8,
            (self.board.sboards[8].encoding >> 8) as u8,
            match self.last_loc {
                Some(x) => x.encoding,
                None => 0b1000_0000,
            },
        ])
    }

    /// Converts a 152-bit BitVec to a Game.
    #[allow(unused_variables)]
    pub fn from_bit_vec(bv: BitVec) -> Game {
        unimplemented!()
    }
}
