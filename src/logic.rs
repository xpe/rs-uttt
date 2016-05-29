/// Game logic; e.g. rules of the game. This module does not include accessors,
/// constants, constructors, or data structure definitions.

use data::{Game, Board, SBoard, Row, Play, Loc, SLoc, Slot};
use data::{BI, SRI, SCI, SBI, Player};
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
    fn play_sans_validate(self, play: Play) -> Game {
        Game {
            board: self.board.play_sans_validate(play),
            last_loc: Some(play.loc),
        }
    }
}

// -> board --------------------------------------------------------------------

impl Board {
    /// Returns a copy of the board after making the play (without validation).
    fn play_sans_validate(self, play: Play) -> Board {
        let mut board = self.clone();
        board.update_with_play(play);
        board
    }

    /// Mutates the board after making the play (without validation).
    fn update_with_play(&mut self, play: Play) {
        let bi: BI = BI::from_loc(play.loc);
        let sbi: SBI = SBI::from_loc(play.loc);
        self.0[bi.as_u8() as usize].update_with(sbi, play.player);
    }
}

// -> sub-board ----------------------------------------------------------------

impl SBoard {
    /// Mutates the sub-board after 'making the play' (without validation).
    fn update_with(&mut self, sbi: SBI, player: Player) {
        let sri = SRI::from_idx(sbi);
        let (mask, shift) = match sri {
            SRI::R0 => (0b0111111111100000, 0),
            SRI::R1 => (0b0111110000011111, 5),
            SRI::R2 => (0b0000001111111111, 10),
        };
        let row = self.row_at(sri).play_at(SCI::from_idx(sbi), player);
        self.0 = (self.0 & mask) | ((Row::as_u8(row) as u16) << shift);
    }
}

// -> rows ---------------------------------------------------------------------

// -> row ----------------------------------------------------------------------

impl Row {
    /// Returns an 'updated' row by putting the player at the specified column.
    /// Note: This could also be written as a match with 27 arms, which would
    /// probably be faster.
    fn play_at(self, sci: SCI, player: Player) -> Row {
        // TODO: rewrite with match
        let mut slots = self.slots();
        let i = match sci { SCI::C0 => 0, SCI::C1 => 1, SCI::C2 => 2 };
        slots[i] = Slot::Taken(player);
        Row::from_slots(slots)
    }
}

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
