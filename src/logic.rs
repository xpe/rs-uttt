/// Game logic; e.g. rules of the game. This module does not include accessors,
/// constants, constructors, or data structure definitions.

use data::{Game, Board, SBoard, Play, Loc, SLoc};

// -> game ---------------------------------------------------------------------

impl Game {
    /// If the play is valid, makes the play and returns the 'updated' game.
    pub fn play(self, play: Play) -> Option<Game> {
        if self.is_valid_play(play) {
            Some(self.play_sans_validate(play))
        } else {
            None
        }
    }

    /// Is the play valid for the given board?
    pub fn is_valid_play(self, p: Play) -> bool {
        if self.last_loc == None {
            self.board.is_location_taken(p.loc)
        } else if self.last_player() == Some(p.player) {
            false
        } else {
            self.board.is_location_taken(p.loc)
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

// -> bool ---------------------------------------------------------------------

impl Board {
    /// Is the board location taken?
    pub fn is_location_taken(self, loc: Loc) -> bool {
        match self.player_at_loc(loc) {
            None => false,
            Some(_) => true,
        }
    }
}

impl SBoard {
    /// Is the sub-board location taken?
    pub fn is_location_taken(self, loc: SLoc) -> bool {
        match self.player_at_loc(loc) {
            None => false,
            Some(_) => true,
        }
    }
}
