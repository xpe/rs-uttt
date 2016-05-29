/// Accessor functions, some of which might be considered 'conversion'
/// functions. This module does not include constants, constructors, data
/// structure definitions, or game logic.

use data::{Game, Board, SBoard, Row};
use data::{Loc, SLoc, Slot, RI, CI, SRI, SCI, Player};
use constants::{SE, SX, SO};

// -- -> game ------------------------------------------------------------------

// -- -> board -----------------------------------------------------------------

// -- -> sub-board -------------------------------------------------------------

impl Board {
    /// Returns an array of 9 sub-boards for a given board.
    pub fn sboards(self) -> [SBoard; 9] {
        self.0
    }
}

// -- -> rows ------------------------------------------------------------------

impl SBoard {
    pub fn rows(self) -> [Row; 3] {
        let x0: u8 = (self.0 & 0b11111) as u8;
        let x1: u8 = (self.0 >> 5 & 0b11111) as u8;
        let x2: u8 = (self.0 >> 10 & 0b011111) as u8;
        [Row::from_u8(x0), Row::from_u8(x1), Row::from_u8(x2)]
    }
}

// -- -> row -------------------------------------------------------------------

impl SBoard {
    pub fn row(self, ri: SRI) -> Row {
        let s = match ri { SRI::R0 => 0, SRI::R1 => 5, SRI::R2 => 10 };
        Row::from_u8((self.0 >> s & 0b11111) as u8)
    }
}

// -- -> board play ------------------------------------------------------------

// -- -> sub-board play --------------------------------------------------------

// -- -> board location --------------------------------------------------------

// -- -> sub-board location ----------------------------------------------------

// -- -> slots -----------------------------------------------------------------

impl Board {
    /// Returns an array of 81 slots for a given board.
    pub fn slots(self) -> [Slot; 81] {
        let sbs: [SBoard; 9] = self.0;
	let mut a = [SE; 81];
	a[ 0 ..  9].copy_from_slice(&sbs[0].slots());
	a[ 9 .. 18].copy_from_slice(&sbs[1].slots());
	a[18 .. 27].copy_from_slice(&sbs[2].slots());
	a[27 .. 36].copy_from_slice(&sbs[3].slots());
	a[36 .. 45].copy_from_slice(&sbs[4].slots());
	a[45 .. 54].copy_from_slice(&sbs[5].slots());
	a[54 .. 63].copy_from_slice(&sbs[6].slots());
	a[63 .. 72].copy_from_slice(&sbs[7].slots());
	a[72 .. 81].copy_from_slice(&sbs[8].slots());
	a
    }

    /// Returns a two dimensional array (9x9) of slots for a given board.
    pub fn slots_9x9(self) -> [[Slot; 9]; 9] {
        let sbs: [SBoard; 9] = self.0;
        [
            sbs[0].slots(),
            sbs[1].slots(),
            sbs[2].slots(),
            sbs[3].slots(),
            sbs[4].slots(),
            sbs[5].slots(),
            sbs[6].slots(),
            sbs[7].slots(),
            sbs[8].slots(),
        ]
    }
}

impl SBoard {
    /// Returns an array of 9 slots for a given sub-board.
    pub fn slots(self) -> [Slot; 9] {
        let rs: [Row; 3] = self.rows();
	let mut a = [SE; 9];
	a[0 .. 3].copy_from_slice(&Row::slots(rs[0]));
	a[3 .. 6].copy_from_slice(&Row::slots(rs[1]));
	a[6 .. 9].copy_from_slice(&Row::slots(rs[2]));
	a
    }

    /// Returns a two dimensional array (3x3) of slots for a given sub-board.
    pub fn slots_3x3(self) -> [[Slot; 3]; 3] {
        let rs: [Row; 3] = self.rows();
        [
            Row::slots(rs[0]),
            Row::slots(rs[1]),
            Row::slots(rs[2]),
        ]
    }
}

impl Row {
    /// Returns an array of 3 slots for a given row.
    pub fn slots(self) -> [Slot; 3] {
        match self {
            Row::EEE => [SE, SE, SE],
            Row::EEO => [SE, SE, SO],
            Row::EEX => [SE, SE, SX],
            Row::EOE => [SE, SO, SE],
            Row::EOO => [SE, SO, SO],
            Row::EOX => [SE, SO, SX],
            Row::EXE => [SE, SX, SE],
            Row::EXO => [SE, SX, SO],
            Row::EXX => [SE, SX, SX],
            Row::OEE => [SO, SE, SE],
            Row::OEO => [SO, SE, SO],
            Row::OEX => [SO, SE, SX],
            Row::OOE => [SO, SO, SE],
            Row::OOO => [SO, SO, SO],
            Row::OOX => [SO, SO, SX],
            Row::OXE => [SO, SX, SE],
            Row::OXO => [SO, SX, SO],
            Row::OXX => [SO, SX, SX],
            Row::XEE => [SX, SE, SE],
            Row::XEO => [SX, SE, SO],
            Row::XEX => [SX, SE, SX],
            Row::XOE => [SX, SO, SE],
            Row::XOO => [SX, SO, SO],
            Row::XOX => [SX, SO, SX],
            Row::XXE => [SX, SX, SE],
            Row::XXO => [SX, SX, SO],
            Row::XXX => [SX, SX, SX],
        }
    }
}

// -- -> slot ------------------------------------------------------------------

impl Board {
    /// Returns the Slot at a particular row and column in a board.
    #[allow(unused_variables)]
    pub fn slot(self, row: RI, col: CI) -> Slot {
        unimplemented!(); // TODO
    }
}

impl SBoard {
    /// Returns the Slot at a particular row and column in a sub-board.
    #[allow(unused_variables)]
    pub fn slot(self, row: SRI, col: SCI) -> Slot {
        unimplemented!(); // TODO
    }
}

impl Row {
    /// Returns the Slot for a particular column index for a given Row.
    #[allow(unused_variables)]
    pub fn slot(self, col: SCI) -> Slot {
        unimplemented!(); // TODO
    }
}

// -- -> board indexes ---------------------------------------------------------

impl Loc {
    /// Returns the row index for a board location.
    #[allow(unused_variables)]
    pub fn row(self) -> RI {
        unimplemented!(); // TODO
    }

    /// Returns the row index for a board location.
    #[allow(unused_variables)]
    pub fn col(self) -> CI {
        unimplemented!(); // TODO
    }
}

// -- -> sub-board indexes -----------------------------------------------------

impl SLoc {
    /// Returns the row index for a sub-board location.
    #[allow(unused_variables)]
    pub fn row(self) -> SRI {
        unimplemented!(); // TODO
    }

    /// Returns the row index for a sub-board location.
    #[allow(unused_variables)]
    pub fn col(self) -> SCI {
        unimplemented!(); // TODO
    }
}

// -- -> player ----------------------------------------------------------------

impl Game {
    /// Returns the last (previous) player in a game, if present.
    pub fn last_player(self) -> Option<Player> {
        match self.last_loc {
            None => None,
            Some(loc) => self.board.player_at_loc(loc),
        }
    }
}

impl Board {
    /// Returns the player at a location, if present.
    pub fn player_at_loc(self, loc: Loc) -> Option<Player> {
        self.player_at_row_col(loc.row(), loc.col())
    }

    /// Returns the player at a row + col, if present.
    #[allow(unused_variables)]
    pub fn player_at_row_col(self, row: RI, col: CI) -> Option<Player> {
        unimplemented!(); // TODO
    }
}

// -- -> u16 -------------------------------------------------------------------

// -- -> u8 --------------------------------------------------------------------

impl Row {
    /// Convert a Row into a u8 value.
    pub fn as_u8(self) -> u8 {
        match self {
            Row::EEE => 0x00,
            Row::EEX => 0x01,
            Row::EEO => 0x02,
            Row::EXE => 0x03,
            Row::EXX => 0x04,
            Row::EXO => 0x05,
            Row::EOE => 0x06,
            Row::EOX => 0x07,
            Row::EOO => 0x08,
            Row::XEE => 0x09,
            Row::XEX => 0x0A,
            Row::XEO => 0x0B,
            Row::XXE => 0x0C,
            Row::XXX => 0x0D,
            Row::XXO => 0x0E,
            Row::XOE => 0x0F,
            Row::XOX => 0x10,
            Row::XOO => 0x11,
            Row::OEE => 0x12,
            Row::OEX => 0x13,
            Row::OEO => 0x14,
            Row::OXE => 0x15,
            Row::OXX => 0x16,
            Row::OXO => 0x17,
            Row::OOE => 0x18,
            Row::OOX => 0x19,
            Row::OOO => 0x1A,
        }
    }
}

// -- -> bool ------------------------------------------------------------------
