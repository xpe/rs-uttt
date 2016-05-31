/// Constants. This module does not include accessors, constructors, data
/// structure definitions, or game logic.

use data::{Game, Board, SBoard, Row, Slot, BI, SBI, Player};

// -- game ---------------------------------------------------------------------

pub const EMPTY_GAME: Game = Game {
    board: EMPTY_BOARD,
    last_loc: None
};

// -- board --------------------------------------------------------------------

pub const EMPTY_BOARD: Board = Board([EMPTY_SBOARD; 9]);

// -- sub-board ----------------------------------------------------------------

/// A constant for an empty sub-board, constructed by assuming that 0
/// corresponds to three empty rows.
pub const EMPTY_SBOARD: SBoard = SBoard(0);

// -- row ----------------------------------------------------------------------

pub const EMPTY_ROW: Row = Row::EEE;

// -- board play ---------------------------------------------------------------

// Note: no board constants needed.

// -- sub-board play -----------------------------------------------------------

// Note: no sub-board constants needed.

// -- board location -----------------------------------------------------------

// Note: no board location constants needed.

// -- sub-board location -------------------------------------------------------

// Note: no sub-board location constants needed.

// -- slot ---------------------------------------------------------------------

pub const SE: Slot = Slot::Empty;
pub const SX: Slot = Slot::Taken(Player::X);
pub const SO: Slot = Slot::Taken(Player::O);

// -- board indexes ------------------------------------------------------------

pub const BI_WINS: [[BI; 3]; 8] = [
    [BI::I0, BI::I1, BI::I2],
    [BI::I3, BI::I4, BI::I5],
    [BI::I6, BI::I7, BI::I8],
    [BI::I0, BI::I3, BI::I6],
    [BI::I1, BI::I4, BI::I2],
    [BI::I2, BI::I5, BI::I8],
    [BI::I0, BI::I4, BI::I8],
    [BI::I2, BI::I4, BI::I6],
];

// -- sub-board indexes --------------------------------------------------------

pub const SBI_WINS: [[SBI; 3]; 8] = [
    [SBI::I0, SBI::I1, SBI::I2],
    [SBI::I3, SBI::I4, SBI::I5],
    [SBI::I6, SBI::I7, SBI::I8],
    [SBI::I0, SBI::I3, SBI::I6],
    [SBI::I1, SBI::I4, SBI::I2],
    [SBI::I2, SBI::I5, SBI::I8],
    [SBI::I0, SBI::I4, SBI::I8],
    [SBI::I2, SBI::I4, SBI::I6],
];

// -- player -------------------------------------------------------------------

pub const FIRST_PLAYER: Player = Player::X;
