/// Constants. This module does not include accessors, constructors, data
/// structure definitions, or game logic.

use data::*;

// -- game ---------------------------------------------------------------------

pub const EMPTY_GAME: Game = Game {
    board: EMPTY_BOARD,
    last_loc: None
};

// -- board --------------------------------------------------------------------

pub const EMPTY_BOARD: Board = Board {
    sboards: [EMPTY_SBOARD; 9],
};

// -- sub-board ----------------------------------------------------------------

/// A constant for an empty sub-board, constructed by assuming that the 0
/// encoding corresponds to three empty rows.
pub const EMPTY_SBOARD: SBoard = SBoard {
    encoding: 0,
};

// -- row ----------------------------------------------------------------------

pub const EMPTY_ROW: Row = Row::EEE;

// -- board play ---------------------------------------------------------------

// -- sub-board play -----------------------------------------------------------

// -- board location -----------------------------------------------------------

// -- sub-board location -------------------------------------------------------

pub const ALL_SLOC: [SLoc; 9] = [
    SLoc { row: SRI::R0, col: SCI::C0 },
    SLoc { row: SRI::R0, col: SCI::C1 },
    SLoc { row: SRI::R0, col: SCI::C2 },
    SLoc { row: SRI::R1, col: SCI::C0 },
    SLoc { row: SRI::R1, col: SCI::C1 },
    SLoc { row: SRI::R1, col: SCI::C2 },
    SLoc { row: SRI::R2, col: SCI::C0 },
    SLoc { row: SRI::R2, col: SCI::C1 },
    SLoc { row: SRI::R2, col: SCI::C2 },
];

// -- slot ---------------------------------------------------------------------

pub const SE: Slot = Slot::Empty;
pub const SX: Slot = Slot::Taken(Player::X);
pub const SO: Slot = Slot::Taken(Player::O);

// -- board indexes ------------------------------------------------------------

pub const BI_WINS: [[BI; 3]; 8] = [
    [BI::I0, BI::I1, BI::I2], // row 0
    [BI::I3, BI::I4, BI::I5], // row 1
    [BI::I6, BI::I7, BI::I8], // row 2
    [BI::I0, BI::I3, BI::I6], // col 0
    [BI::I1, BI::I4, BI::I7], // col 1
    [BI::I2, BI::I5, BI::I8], // col 2
    [BI::I0, BI::I4, BI::I8], // \ diagonal
    [BI::I2, BI::I4, BI::I6], // / diagonal
];

pub const ALL_BI: [BI; 9] = [
    BI::I0,
    BI::I1,
    BI::I2,
    BI::I3,
    BI::I4,
    BI::I5,
    BI::I6,
    BI::I7,
    BI::I8,
];

// -- sub-board indexes --------------------------------------------------------

pub const SBI_WINS: [[SBI; 3]; 8] = [
    [SBI::I0, SBI::I1, SBI::I2], // row 0
    [SBI::I3, SBI::I4, SBI::I5], // row 1
    [SBI::I6, SBI::I7, SBI::I8], // row 2
    [SBI::I0, SBI::I3, SBI::I6], // col 0
    [SBI::I1, SBI::I4, SBI::I7], // col 1
    [SBI::I2, SBI::I5, SBI::I8], // col 2
    [SBI::I0, SBI::I4, SBI::I8], // \ diagonal
    [SBI::I2, SBI::I4, SBI::I6], // / diagonal
];

pub const ALL_SBI: [SBI; 9] = [
    SBI::I0,
    SBI::I1,
    SBI::I2,
    SBI::I3,
    SBI::I4,
    SBI::I5,
    SBI::I6,
    SBI::I7,
    SBI::I8,
];

// -- player -------------------------------------------------------------------

pub const FIRST_PLAYER: Player = Player::X;
