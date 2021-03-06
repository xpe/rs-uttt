/// Data structure definitions. This module does not include accessors,
/// constants, constructors, or game logic.

// -- data: game state ---------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GameState { Won(Player), Tied, Ongoing }

// -- data: game ---------------------------------------------------------------

/// A `Game` is the combination of a `Board` and an optional last location of
/// play. (A last location is only None for an empty board.)
///
/// Note: It might be useful to cache if a game is in progress or ended. This
/// would require only one bit and possibly could be combined with `board.`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Game {
    pub board: Board,
    pub last_loc: Option<Loc>
}

// -- data: board --------------------------------------------------------------

/// A `Board` is an array of 9 sub-boards (`SBoard`), indexed like this:
///
/// * row 0 : `0 1 2`
/// * row 1 : `3 4 5`
/// * row 2 : `6 7 8`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Board {
    pub sboards: [SBoard; 9],
}

// -- data: sub-board ----------------------------------------------------------

/// An `SBoard` (a sub-board) has 3 rows, each having 3 slots. This
/// representation requires 16 bits:
///
/// * row 0 : `0b0000000000011111`
/// * row 1 : `0b0000001111100000`
/// * row 2 : `0b0111110000000000`
///
/// Note: It might be useful to cache if a sub-board is in progress or
/// ended. This would require one bit, which is available.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SBoard {
    pub encoding: u16,
}

// -- data: row ----------------------------------------------------------------

/// An enumeration of possible row values:
///
/// * 'E' means empty
/// * 'X' means player X
/// * 'O' means player O.
///
/// Note: I'd prefer to only use 5 bits but Rust prefers to align data
/// structures on byte boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Row {
    EEE, EEO, EEX, EOE, EOO, EOX, EXE, EXO, EXX,
    OEE, OEO, OEX, OOE, OOO, OOX, OXE, OXO, OXX,
    XEE, XEO, XEX, XOE, XOO, XOX, XXE, XXO, XXX,
}

// -- data: board play ---------------------------------------------------------

/// A board play, consisting of a location and player.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Play {
    pub loc: Loc,
    pub player: Player
}

// -- data: sub-board play -----------------------------------------------------

/// A sub-board play, consisting of a sub-board location and player.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPlay {
    pub loc: SLoc,
    pub player: Player
}

// -- data: board location -----------------------------------------------------

/// A location on a board (a row and column), represented with 8 bits:
///
/// * row: `0b11110000` (upper nibble)
/// * col: `0b00001111` (lower nibble)
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Loc {
    pub encoding: u8,
}

// -- data: sub-board location -------------------------------------------------

/// A sub-board location, having two indexes (sub-board row and col).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SLoc {
    pub row: SRI,
    pub col: SCI
}

// -- data: slot ---------------------------------------------------------------

/// A slot is either taken by a player or empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Slot {
    Taken(Player),
    Empty
}

// -- data: board indexes ------------------------------------------------------

/// A board row index, ranging from 0 to 8, inclusive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RI { R0, R1, R2, R3, R4, R5, R6, R7, R8 }

/// A board column index, ranging from 0 to 8, inclusive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CI { C0, C1, C2, C3, C4, C5, C6, C7, C8 }

/// A board index, ranging from 0 to 8, inclusive.
///
/// * row 0 : `I0 I1 I2`
/// * row 1 : `I3 I4 I5`
/// * row 2 : `I6 I7 I8`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BI { I0, I1, I2, I3, I4, I5, I6, I7, I8 }

// -- data: sub-board indexes --------------------------------------------------

/// A sub-board row index: 0, 1, or 2.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SRI { R0, R1, R2 }

/// A sub-board column index: 0, 1, or 2.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SCI { C0, C1, C2 }

/// A sub-board index, ranging from 0 to 8, inclusive.
///
/// * row 0 : `I0 I1 I2`
/// * row 1 : `I3 I4 I5`
/// * row 2 : `I6 I7 I8`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SBI { I0, I1, I2, I3, I4, I5, I6, I7, I8 }

// -- data: player -------------------------------------------------------------

/// A player. Either X or O.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Player { X, O }

// -- data: count ---------------------------------------------------------

/// A count, useful for counting the number of plays, for example.
pub type Count = u8;
