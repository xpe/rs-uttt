use data::{Game, Board, SBoard, Play, SPlay, Loc, SLoc, Player};

/// The "game logic" of Ultimate Tic-Tac-Toe.

// -> game ---------------------------------------------------------------------

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

impl Game {
    /// Returns the last player in a game.
    #[allow(unused_variables)]
    pub fn last_player(game: Game) -> Option<Player> {
        unimplemented!(); // TODO
    }
}

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
        true // TODO: implement
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
        true // TODO: implement
    }
}
