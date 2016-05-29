/// Game logic; e.g. rules of the game. This module does not include accessors,
/// constants, constructors, or data structure definitions.

use data::{Game, Board, SBoard, Play, Loc, SLoc, Player};
use constants::{FIRST_PLAYER};

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

    /// Is the play valid for the given game?
    pub fn is_valid_play(self, p: Play) -> bool {
        if self.next_player() == Some(p.player) {
            self.board.is_location_empty(p.loc)
        } else {
            false
        }
    }

    /// Makes the play (without validation) and returns the 'updated' game.
    #[allow(unused_variables)]
    fn play_sans_validate(self, play: Play) -> Game {
        println!("play_sans_validate : unimplemented");
        unimplemented!(); // TODO: implement
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

impl Game {
    /// Returns the next player in the game, if there is one. Returns None if
    /// the game is finished.
    pub fn next_player(self) -> Option<Player> {
        if self.is_complete() {
            None
        } else {
            Some(match self.last_player() {
                None => FIRST_PLAYER,
                Some(Player::X) => Player::O,
                Some(Player::O) => Player::X,
            })
        }
    }
}

// -> bool ---------------------------------------------------------------------

impl Game {
    /// Returns true if the game is complete; i.e. a player won or there is a
    /// tie.
    pub fn is_complete(self) -> bool {
        false // TODO: implement
    }
}

impl Board {
    /// Is the board location empty?
    pub fn is_location_empty(self, loc: Loc) -> bool {
        match self.player_at_loc(loc) {
            None => true,
            Some(_) => false,
        }
    }
}

impl SBoard {
    /// Is the sub-board location empty?
    pub fn is_location_empty(self, loc: SLoc) -> bool {
        match self.player_at_loc(loc) {
            None => true,
            Some(_) => false,
        }
    }
}
