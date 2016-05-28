use data::{Game, Board, SBoard, Row, Slot, Player};

// -- game ---------------------------------------------------------------------

pub const EMPTY_GAME: Game = Game {
    board: EMPTY_BOARD,
    last_loc: None
};

// -- board --------------------------------------------------------------------

pub const EMPTY_BOARD: Board = Board([EMPTY_SBOARD; 9]);

// -- sub-board ----------------------------------------------------------------

pub const EMPTY_SBOARD: SBoard = SBoard(0); // TODO: FIX

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

// Note: no board index constants needed.

// -- sub-board indexes --------------------------------------------------------

// Note: no sub-board index constants needed.

// -- player -------------------------------------------------------------------

// Note: no player constants needed.
