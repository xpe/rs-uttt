/// Accessor functions, some of which might be considered 'conversion'
/// functions. This module does not include constants, constructors, data
/// structure definitions, or game logic.

use data::{Game, Board, SBoard, Row, Loc, SLoc};
use data::{Slot, RI, CI, BI, SRI, SCI, SBI, Player};
use constants::{SE, SX, SO};

// -- -> game ------------------------------------------------------------------

// -- -> board -----------------------------------------------------------------

// -- -> sub-boards ------------------------------------------------------------

impl Board {
    /// Returns an array of 9 sub-boards for a given board.
    pub fn sboards(self) -> [SBoard; 9] {
        self.0
    }
}

// -- -> sub-board -------------------------------------------------------------

impl Board {
    /// Returns the sub-board at a given board row and col.
    pub fn sboard_at_row_col(self, row: RI, col: CI) -> SBoard {
        self.sboard_at_idx(BI::from_row_col(row, col))
    }

    /// Returns the sub-board at a given board index.
    pub fn sboard_at_idx(self, idx: BI) -> SBoard {
        self.0[idx.as_u8() as usize]
    }

    /// Returns a mutable reference to a sub-board at a given index.
    pub fn mut_sboard_at_idx(&mut self, idx: BI) -> &mut SBoard {
        &mut self.0[idx.as_u8() as usize]
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
    pub fn row_at(self, ri: SRI) -> Row {
        let shift = match ri { SRI::R0 => 0, SRI::R1 => 5, SRI::R2 => 10 };
        Row::from_u8((self.0 >> shift & 0b11111) as u8)
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
    pub fn slot_at_loc(self, loc: Loc) -> Slot {
        self.slot_at_row_col(loc.row(), loc.col())
    }

    /// Returns the Slot at a particular row and column in a board.
    pub fn slot_at_row_col(self, row: RI, col: CI) -> Slot {
        let sb: SBoard = self.sboard_at_row_col(row, col);
        sb.slot_at_idx(SBI::from_row_col(row, col))
    }
}

impl SBoard {
    /// Returns the Slot at a particular row and column in a sub-board.
    pub fn slot_at_row_col(self, ri: SRI, ci: SCI) -> Slot {
        self.row_at(ri).slot_at(ci)
    }

    /// Returns the Slot at a particular sub-board index in a sub-board.
    pub fn slot_at_idx(self, idx: SBI) -> Slot {
        let (ri, ci) = idx.as_row_col();
        self.slot_at_row_col(ri, ci)
    }
}

impl Row {
    /// Returns the Slot for a particular column index for a given Row.
    pub fn slot_at(self, col: SCI) -> Slot {
        let slots: [Slot; 3] = self.slots();
        match col {
            SCI::C0 => slots[0],
            SCI::C1 => slots[1],
            SCI::C2 => slots[2],
        }
    }
}

// -- -> board indexes ---------------------------------------------------------

impl Loc {
    /// Returns the row index for a board location.
    pub fn row(self) -> RI {
        RI::from_u8(self.0 >> 4)
    }

    /// Returns the row index for a board location.
    pub fn col(self) -> CI {
        CI::from_u8(self.0 & 0b00001111)
    }
}

impl RI {
    /// Convert a u8 value to a row index.
    pub fn from_u8(x: u8) -> RI {
        match x {
            0 => RI::R0,
            1 => RI::R1,
            2 => RI::R2,
            3 => RI::R3,
            4 => RI::R4,
            5 => RI::R5,
            6 => RI::R6,
            7 => RI::R7,
            8 => RI::R8,
            _ => panic!("internal error"),
        }
    }
}

impl CI {
    /// Convert a u8 value to a column index.
    pub fn from_u8(x: u8) -> CI {
        match x {
            0 => CI::C0,
            1 => CI::C1,
            2 => CI::C2,
            3 => CI::C3,
            4 => CI::C4,
            5 => CI::C5,
            6 => CI::C6,
            7 => CI::C7,
            8 => CI::C8,
            _ => panic!("internal error"),
        }
    }
}

impl BI {
    /// Returns a board index for a given board location.
    pub fn from_loc(loc: Loc) -> BI {
        BI::from_row_col(loc.row(), loc.col())
    }

    /// Returns a board index for a given board row and col.
    pub fn from_row_col(row: RI, col: CI) -> BI {
        match (row, col) {
            (RI::R0, CI::C0) => BI::I0, //
            (RI::R0, CI::C1) => BI::I0,
            (RI::R0, CI::C2) => BI::I0,
            (RI::R0, CI::C3) => BI::I1,
            (RI::R0, CI::C4) => BI::I1,
            (RI::R0, CI::C5) => BI::I1,
            (RI::R0, CI::C6) => BI::I2,
            (RI::R0, CI::C7) => BI::I2,
            (RI::R0, CI::C8) => BI::I2,
            (RI::R1, CI::C0) => BI::I0,
            (RI::R1, CI::C1) => BI::I0,
            (RI::R1, CI::C2) => BI::I0,
            (RI::R1, CI::C3) => BI::I1,
            (RI::R1, CI::C4) => BI::I1,
            (RI::R1, CI::C5) => BI::I1,
            (RI::R1, CI::C6) => BI::I2,
            (RI::R1, CI::C7) => BI::I2,
            (RI::R1, CI::C8) => BI::I2,
            (RI::R2, CI::C0) => BI::I0,
            (RI::R2, CI::C1) => BI::I0,
            (RI::R2, CI::C2) => BI::I0,
            (RI::R2, CI::C3) => BI::I1,
            (RI::R2, CI::C4) => BI::I1,
            (RI::R2, CI::C5) => BI::I1,
            (RI::R2, CI::C6) => BI::I2,
            (RI::R2, CI::C7) => BI::I2,
            (RI::R2, CI::C8) => BI::I2,
            (RI::R3, CI::C0) => BI::I3, //
            (RI::R3, CI::C1) => BI::I3,
            (RI::R3, CI::C2) => BI::I3,
            (RI::R3, CI::C3) => BI::I4,
            (RI::R3, CI::C4) => BI::I4,
            (RI::R3, CI::C5) => BI::I4,
            (RI::R3, CI::C6) => BI::I5,
            (RI::R3, CI::C7) => BI::I5,
            (RI::R3, CI::C8) => BI::I5,
            (RI::R4, CI::C0) => BI::I3,
            (RI::R4, CI::C1) => BI::I3,
            (RI::R4, CI::C2) => BI::I3,
            (RI::R4, CI::C3) => BI::I4,
            (RI::R4, CI::C4) => BI::I4,
            (RI::R4, CI::C5) => BI::I4,
            (RI::R4, CI::C6) => BI::I5,
            (RI::R4, CI::C7) => BI::I5,
            (RI::R4, CI::C8) => BI::I5,
            (RI::R5, CI::C0) => BI::I3,
            (RI::R5, CI::C1) => BI::I3,
            (RI::R5, CI::C2) => BI::I3,
            (RI::R5, CI::C3) => BI::I4,
            (RI::R5, CI::C4) => BI::I4,
            (RI::R5, CI::C5) => BI::I4,
            (RI::R5, CI::C6) => BI::I5,
            (RI::R5, CI::C7) => BI::I5,
            (RI::R5, CI::C8) => BI::I5,
            (RI::R6, CI::C0) => BI::I6, //
            (RI::R6, CI::C1) => BI::I6,
            (RI::R6, CI::C2) => BI::I6,
            (RI::R6, CI::C3) => BI::I7,
            (RI::R6, CI::C4) => BI::I7,
            (RI::R6, CI::C5) => BI::I7,
            (RI::R6, CI::C6) => BI::I8,
            (RI::R6, CI::C7) => BI::I8,
            (RI::R6, CI::C8) => BI::I8,
            (RI::R7, CI::C0) => BI::I6,
            (RI::R7, CI::C1) => BI::I6,
            (RI::R7, CI::C2) => BI::I6,
            (RI::R7, CI::C3) => BI::I7,
            (RI::R7, CI::C4) => BI::I7,
            (RI::R7, CI::C5) => BI::I7,
            (RI::R7, CI::C6) => BI::I8,
            (RI::R7, CI::C7) => BI::I8,
            (RI::R7, CI::C8) => BI::I8,
            (RI::R8, CI::C0) => BI::I6,
            (RI::R8, CI::C1) => BI::I6,
            (RI::R8, CI::C2) => BI::I6,
            (RI::R8, CI::C3) => BI::I7,
            (RI::R8, CI::C4) => BI::I7,
            (RI::R8, CI::C5) => BI::I7,
            (RI::R8, CI::C6) => BI::I8,
            (RI::R8, CI::C7) => BI::I8,
            (RI::R8, CI::C8) => BI::I8,
        }
    }

    /// Convert a u8 value to a row index.
    pub fn from_u8(x: u8) -> BI {
        match x {
            0 => BI::I0,
            1 => BI::I1,
            2 => BI::I2,
            3 => BI::I3,
            4 => BI::I4,
            5 => BI::I5,
            6 => BI::I6,
            7 => BI::I7,
            8 => BI::I8,
            _ => panic!("internal error"),
        }
    }
}

impl SBI {
    pub fn as_bi(self) -> BI {
        match self {
            SBI::I0 => BI::I0,
            SBI::I1 => BI::I1,
            SBI::I2 => BI::I2,
            SBI::I3 => BI::I3,
            SBI::I4 => BI::I4,
            SBI::I5 => BI::I5,
            SBI::I6 => BI::I6,
            SBI::I7 => BI::I7,
            SBI::I8 => BI::I8,
        }
    }
}

// -- -> sub-board indexes -----------------------------------------------------

// Note: see also struct accessors: `SLoc::row` and `SLoc::col`.

impl SRI {
    /// Convert a sub-board index to a sub-board row index.
    pub fn from_idx(sbi: SBI) -> SRI {
        match sbi {
            SBI::I0 => SRI::R0,
            SBI::I1 => SRI::R0,
            SBI::I2 => SRI::R0,
            SBI::I3 => SRI::R1,
            SBI::I4 => SRI::R1,
            SBI::I5 => SRI::R1,
            SBI::I6 => SRI::R2,
            SBI::I7 => SRI::R2,
            SBI::I8 => SRI::R2,
        }
    }
}

impl SCI {
    /// Convert a sub-board index to a sub-board column index.
    pub fn from_idx(sbi: SBI) -> SCI {
        match sbi {
            SBI::I0 => SCI::C0,
            SBI::I1 => SCI::C1,
            SBI::I2 => SCI::C2,
            SBI::I3 => SCI::C0,
            SBI::I4 => SCI::C1,
            SBI::I5 => SCI::C2,
            SBI::I6 => SCI::C0,
            SBI::I7 => SCI::C1,
            SBI::I8 => SCI::C2,
        }
    }
}

impl SBI {
    /// Returns a (row idx, col idx) tuple for a given sub-board index.
    pub fn as_row_col(self) -> (SRI, SCI) {
        match self {
            SBI::I0 => (SRI::R0, SCI::C0),
            SBI::I1 => (SRI::R0, SCI::C1),
            SBI::I2 => (SRI::R0, SCI::C2),
            SBI::I3 => (SRI::R1, SCI::C0),
            SBI::I4 => (SRI::R1, SCI::C1),
            SBI::I5 => (SRI::R1, SCI::C2),
            SBI::I6 => (SRI::R2, SCI::C0),
            SBI::I7 => (SRI::R2, SCI::C1),
            SBI::I8 => (SRI::R2, SCI::C2),
        }
    }

    /// Returns a sub-board index for a given board location.
    pub fn from_loc(loc: Loc) -> SBI {
        SBI::from_row_col(loc.row(), loc.col())
    }

    /// Returns a sub-board index for a given board row and col.
    pub fn from_row_col(row: RI, col: CI) -> SBI {
        match (row, col) {
            (RI::R0, CI::C0) => SBI::I0, // RI::R0
            (RI::R0, CI::C1) => SBI::I1,
            (RI::R0, CI::C2) => SBI::I2,
            (RI::R0, CI::C3) => SBI::I0,
            (RI::R0, CI::C4) => SBI::I1,
            (RI::R0, CI::C5) => SBI::I2,
            (RI::R0, CI::C6) => SBI::I0,
            (RI::R0, CI::C7) => SBI::I1,
            (RI::R0, CI::C8) => SBI::I2,
            (RI::R1, CI::C0) => SBI::I3, // RI::R1
            (RI::R1, CI::C1) => SBI::I4,
            (RI::R1, CI::C2) => SBI::I5,
            (RI::R1, CI::C3) => SBI::I3,
            (RI::R1, CI::C4) => SBI::I4,
            (RI::R1, CI::C5) => SBI::I5,
            (RI::R1, CI::C6) => SBI::I3,
            (RI::R1, CI::C7) => SBI::I4,
            (RI::R1, CI::C8) => SBI::I5,
            (RI::R2, CI::C0) => SBI::I6, // RI::R2
            (RI::R2, CI::C1) => SBI::I7,
            (RI::R2, CI::C2) => SBI::I8,
            (RI::R2, CI::C3) => SBI::I6,
            (RI::R2, CI::C4) => SBI::I7,
            (RI::R2, CI::C5) => SBI::I8,
            (RI::R2, CI::C6) => SBI::I6,
            (RI::R2, CI::C7) => SBI::I7,
            (RI::R2, CI::C8) => SBI::I8,
            (RI::R3, CI::C0) => SBI::I0, // RI::R3
            (RI::R3, CI::C1) => SBI::I1,
            (RI::R3, CI::C2) => SBI::I2,
            (RI::R3, CI::C3) => SBI::I0,
            (RI::R3, CI::C4) => SBI::I1,
            (RI::R3, CI::C5) => SBI::I2,
            (RI::R3, CI::C6) => SBI::I0,
            (RI::R3, CI::C7) => SBI::I1,
            (RI::R3, CI::C8) => SBI::I2,
            (RI::R4, CI::C0) => SBI::I3, // RI::R4
            (RI::R4, CI::C1) => SBI::I4,
            (RI::R4, CI::C2) => SBI::I5,
            (RI::R4, CI::C3) => SBI::I3,
            (RI::R4, CI::C4) => SBI::I4,
            (RI::R4, CI::C5) => SBI::I5,
            (RI::R4, CI::C6) => SBI::I3,
            (RI::R4, CI::C7) => SBI::I4,
            (RI::R4, CI::C8) => SBI::I5,
            (RI::R5, CI::C0) => SBI::I6, // RI::R5
            (RI::R5, CI::C1) => SBI::I7,
            (RI::R5, CI::C2) => SBI::I8,
            (RI::R5, CI::C3) => SBI::I6,
            (RI::R5, CI::C4) => SBI::I7,
            (RI::R5, CI::C5) => SBI::I8,
            (RI::R5, CI::C6) => SBI::I6,
            (RI::R5, CI::C7) => SBI::I7,
            (RI::R5, CI::C8) => SBI::I8,
            (RI::R6, CI::C0) => SBI::I0, // RI::R6
            (RI::R6, CI::C1) => SBI::I1,
            (RI::R6, CI::C2) => SBI::I2,
            (RI::R6, CI::C3) => SBI::I0,
            (RI::R6, CI::C4) => SBI::I1,
            (RI::R6, CI::C5) => SBI::I2,
            (RI::R6, CI::C6) => SBI::I0,
            (RI::R6, CI::C7) => SBI::I1,
            (RI::R6, CI::C8) => SBI::I2,
            (RI::R7, CI::C0) => SBI::I3, // RI::R7
            (RI::R7, CI::C1) => SBI::I4,
            (RI::R7, CI::C2) => SBI::I5,
            (RI::R7, CI::C3) => SBI::I3,
            (RI::R7, CI::C4) => SBI::I4,
            (RI::R7, CI::C5) => SBI::I5,
            (RI::R7, CI::C6) => SBI::I3,
            (RI::R7, CI::C7) => SBI::I4,
            (RI::R7, CI::C8) => SBI::I5,
            (RI::R8, CI::C0) => SBI::I6, // RI::R8
            (RI::R8, CI::C1) => SBI::I7,
            (RI::R8, CI::C2) => SBI::I8,
            (RI::R8, CI::C3) => SBI::I6,
            (RI::R8, CI::C4) => SBI::I7,
            (RI::R8, CI::C5) => SBI::I8,
            (RI::R8, CI::C6) => SBI::I6,
            (RI::R8, CI::C7) => SBI::I7,
            (RI::R8, CI::C8) => SBI::I8,
        }
    }

    /// Convert a u8 value to a row index.
    pub fn from_u8(x: u8) -> SBI {
        match x {
            0 => SBI::I0,
            1 => SBI::I1,
            2 => SBI::I2,
            3 => SBI::I3,
            4 => SBI::I4,
            5 => SBI::I5,
            6 => SBI::I6,
            7 => SBI::I7,
            8 => SBI::I8,
            _ => panic!("internal error"),
        }
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
    /// Returns the player at a board location, if present.
    pub fn player_at_loc(self, loc: Loc) -> Option<Player> {
        self.player_at_row_col(loc.row(), loc.col())
    }

    /// Returns the player at a row + col, if present.
    pub fn player_at_row_col(self, row: RI, col: CI) -> Option<Player> {
        match self.slot_at_row_col(row, col) {
            Slot::Empty => None,
            Slot::Taken(player) => Some(player),
        }
    }
}

impl SBoard {
    /// Returns the player at a sub-board location, if present.
    pub fn player_at_loc(self, loc: SLoc) -> Option<Player> {
        self.player_at_row_col(loc.row, loc.col)
    }

    /// Returns the player at a row + col, if present.
    pub fn player_at_row_col(self, row: SRI, col: SCI) -> Option<Player> {
        match self.slot_at_row_col(row, col) {
            Slot::Empty => None,
            Slot::Taken(player) => Some(player),
        }
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

impl RI {
    /// Convert a row index into a u8 value.
    pub fn as_u8(self) -> u8 {
        match self {
            RI::R0 => 0,
            RI::R1 => 1,
            RI::R2 => 2,
            RI::R3 => 3,
            RI::R4 => 4,
            RI::R5 => 5,
            RI::R6 => 6,
            RI::R7 => 7,
            RI::R8 => 8,
        }
    }
}

impl CI {
    /// Convert a column index into a u8 value.
    pub fn as_u8(self) -> u8 {
        match self {
            CI::C0 => 0,
            CI::C1 => 1,
            CI::C2 => 2,
            CI::C3 => 3,
            CI::C4 => 4,
            CI::C5 => 5,
            CI::C6 => 6,
            CI::C7 => 7,
            CI::C8 => 8,
        }
    }
}

impl BI {
    /// Convert a board index into a u8 value.
    pub fn as_u8(self) -> u8 {
        match self {
            BI::I0 => 0,
            BI::I1 => 1,
            BI::I2 => 2,
            BI::I3 => 3,
            BI::I4 => 4,
            BI::I5 => 5,
            BI::I6 => 6,
            BI::I7 => 7,
            BI::I8 => 8,
        }
    }
}

// -- -> bool ------------------------------------------------------------------