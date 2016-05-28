/// The game logic for Ultimate Tic-Tac-Toe; e.g. rules of the game. This module
/// does not include other areas such as data structure definitions,
/// constructors, or conversions.

use data::{Game, Board, SBoard, Play, SPlay, Loc, SLoc};

// -> game ---------------------------------------------------------------------

impl Game {
    /// If the play is valid, makes the play and returns the 'updated' game.
    pub fn play(self, play: Play) -> Option<Game> {
        if self.board.is_valid_play(play) {
            Some(self.play_sans_validate(play))
        } else {
            None
        }
    }

    /// Makes the play (without validation) and returns the 'updated' game.
    #[allow(unused_variables)]
    fn play_sans_validate(self, play: Play) -> Game {
        // TODO: implement
        unimplemented!(); // TODO
    }
}

// -> board --------------------------------------------------------------------

// -> sub-board ----------------------------------------------------------------

// -> rows ---------------------------------------------------------------------

// -> row ----------------------------------------------------------------------

// -> board play ---------------------------------------------------------------

// -> sub-board play -----------------------------------------------------------

// -> board location -----------------------------------------------------------

// -> sub-board location -------------------------------------------------------

// -> slots --------------------------------------------------------------------

// -> slot ---------------------------------------------------------------------

// -> board indexes ------------------------------------------------------------

// -> sub-board indexes --------------------------------------------------------

// -> player -------------------------------------------------------------------

// -> u8 -----------------------------------------------------------------------

// -> bool ---------------------------------------------------------------------

impl Board {
    /// Is the play valid for the given board?
    #[allow(unused_variables)]
    pub fn is_valid_play(self, p: Play) -> bool {
        // TODO: this implementation is incomplete and incorrect
        Board::is_location_taken(self, p.loc)
    }

    /// Is the board location taken?
    #[allow(unused_variables)]
    pub fn is_location_taken(self, loc: Loc) -> bool {
        // true
        // TODO: implement
        unimplemented!();
    }
}

impl SBoard {
    /// Is the play valid for the given sub-board?
    #[allow(unused_variables)]
    pub fn is_valid_play(self, sp: SPlay) -> bool {
        // TODO: this implementation is incomplete and incorrect
        SBoard::is_location_taken(self, sp.loc)
    }

    /// Is the sub-board location taken?
    #[allow(unused_variables)]
    pub fn is_location_taken(self, loc: SLoc) -> bool {
        // true
        // TODO: implement
        unimplemented!();
    }
}
