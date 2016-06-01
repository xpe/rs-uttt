/// Game logic; e.g. rules of the game. This module does not include accessors,
/// constants, constructors, or data structure definitions.

use constants::*;
use data::*;

#[cfg(test)]
mod tests;

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
        self.mut_sboard_at_idx(bi).update_with(sbi, play.player);
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
    fn play_at(self, sci: SCI, player: Player) -> Row {
        match (player, sci, self) {
            (Player::O, SCI::C0, Row::EEE) => Row::OEE,
            (Player::O, SCI::C0, Row::EEO) => Row::OEO,
            (Player::O, SCI::C0, Row::EEX) => Row::OEX,
            (Player::O, SCI::C0, Row::EOE) => Row::OOE,
            (Player::O, SCI::C0, Row::EOO) => Row::OOO,
            (Player::O, SCI::C0, Row::EOX) => Row::OOX,
            (Player::O, SCI::C0, Row::EXE) => Row::OXE,
            (Player::O, SCI::C0, Row::EXO) => Row::OXO,
            (Player::O, SCI::C0, Row::EXX) => Row::OXX,
            (Player::O, SCI::C0, Row::OEE) => Row::OEE,
            (Player::O, SCI::C0, Row::OEO) => Row::OEO,
            (Player::O, SCI::C0, Row::OEX) => Row::OEX,
            (Player::O, SCI::C0, Row::OOE) => Row::OOE,
            (Player::O, SCI::C0, Row::OOO) => Row::OOO,
            (Player::O, SCI::C0, Row::OOX) => Row::OOX,
            (Player::O, SCI::C0, Row::OXE) => Row::OXE,
            (Player::O, SCI::C0, Row::OXO) => Row::OXO,
            (Player::O, SCI::C0, Row::OXX) => Row::OXX,
            (Player::O, SCI::C0, Row::XEE) => Row::OEE,
            (Player::O, SCI::C0, Row::XEO) => Row::OEO,
            (Player::O, SCI::C0, Row::XEX) => Row::OEX,
            (Player::O, SCI::C0, Row::XOE) => Row::OOE,
            (Player::O, SCI::C0, Row::XOO) => Row::OOO,
            (Player::O, SCI::C0, Row::XOX) => Row::OOX,
            (Player::O, SCI::C0, Row::XXE) => Row::OXE,
            (Player::O, SCI::C0, Row::XXO) => Row::OXO,
            (Player::O, SCI::C0, Row::XXX) => Row::OXX,
            (Player::O, SCI::C1, Row::EEE) => Row::EOE,
            (Player::O, SCI::C1, Row::EEO) => Row::EOO,
            (Player::O, SCI::C1, Row::EEX) => Row::EOX,
            (Player::O, SCI::C1, Row::EOE) => Row::EOE,
            (Player::O, SCI::C1, Row::EOO) => Row::EOO,
            (Player::O, SCI::C1, Row::EOX) => Row::EOX,
            (Player::O, SCI::C1, Row::EXE) => Row::EOE,
            (Player::O, SCI::C1, Row::EXO) => Row::EOO,
            (Player::O, SCI::C1, Row::EXX) => Row::EOX,
            (Player::O, SCI::C1, Row::OEE) => Row::OOE,
            (Player::O, SCI::C1, Row::OEO) => Row::OOO,
            (Player::O, SCI::C1, Row::OEX) => Row::OOX,
            (Player::O, SCI::C1, Row::OOE) => Row::OOE,
            (Player::O, SCI::C1, Row::OOO) => Row::OOO,
            (Player::O, SCI::C1, Row::OOX) => Row::OOX,
            (Player::O, SCI::C1, Row::OXE) => Row::OOE,
            (Player::O, SCI::C1, Row::OXO) => Row::OOO,
            (Player::O, SCI::C1, Row::OXX) => Row::OOX,
            (Player::O, SCI::C1, Row::XEE) => Row::XOE,
            (Player::O, SCI::C1, Row::XEO) => Row::XOO,
            (Player::O, SCI::C1, Row::XEX) => Row::XOX,
            (Player::O, SCI::C1, Row::XOE) => Row::XOE,
            (Player::O, SCI::C1, Row::XOO) => Row::XOO,
            (Player::O, SCI::C1, Row::XOX) => Row::XOX,
            (Player::O, SCI::C1, Row::XXE) => Row::XOE,
            (Player::O, SCI::C1, Row::XXO) => Row::XOO,
            (Player::O, SCI::C1, Row::XXX) => Row::XOX,
            (Player::O, SCI::C2, Row::EEE) => Row::EEO,
            (Player::O, SCI::C2, Row::EEO) => Row::EEO,
            (Player::O, SCI::C2, Row::EEX) => Row::EEO,
            (Player::O, SCI::C2, Row::EOE) => Row::EOO,
            (Player::O, SCI::C2, Row::EOO) => Row::EOO,
            (Player::O, SCI::C2, Row::EOX) => Row::EOO,
            (Player::O, SCI::C2, Row::EXE) => Row::EXO,
            (Player::O, SCI::C2, Row::EXO) => Row::EXO,
            (Player::O, SCI::C2, Row::EXX) => Row::EXO,
            (Player::O, SCI::C2, Row::OEE) => Row::OEO,
            (Player::O, SCI::C2, Row::OEO) => Row::OEO,
            (Player::O, SCI::C2, Row::OEX) => Row::OEO,
            (Player::O, SCI::C2, Row::OOE) => Row::OOO,
            (Player::O, SCI::C2, Row::OOO) => Row::OOO,
            (Player::O, SCI::C2, Row::OOX) => Row::OOO,
            (Player::O, SCI::C2, Row::OXE) => Row::OXO,
            (Player::O, SCI::C2, Row::OXO) => Row::OXO,
            (Player::O, SCI::C2, Row::OXX) => Row::OXO,
            (Player::O, SCI::C2, Row::XEE) => Row::XEO,
            (Player::O, SCI::C2, Row::XEO) => Row::XEO,
            (Player::O, SCI::C2, Row::XEX) => Row::XEO,
            (Player::O, SCI::C2, Row::XOE) => Row::XOO,
            (Player::O, SCI::C2, Row::XOO) => Row::XOO,
            (Player::O, SCI::C2, Row::XOX) => Row::XOO,
            (Player::O, SCI::C2, Row::XXE) => Row::XXO,
            (Player::O, SCI::C2, Row::XXO) => Row::XXO,
            (Player::O, SCI::C2, Row::XXX) => Row::XXO,
            (Player::X, SCI::C0, Row::EEE) => Row::XEE,
            (Player::X, SCI::C0, Row::EEO) => Row::XEO,
            (Player::X, SCI::C0, Row::EEX) => Row::XEX,
            (Player::X, SCI::C0, Row::EOE) => Row::XOE,
            (Player::X, SCI::C0, Row::EOO) => Row::XOO,
            (Player::X, SCI::C0, Row::EOX) => Row::XOX,
            (Player::X, SCI::C0, Row::EXE) => Row::XXE,
            (Player::X, SCI::C0, Row::EXO) => Row::XXO,
            (Player::X, SCI::C0, Row::EXX) => Row::XXX,
            (Player::X, SCI::C0, Row::OEE) => Row::XEE,
            (Player::X, SCI::C0, Row::OEO) => Row::XEO,
            (Player::X, SCI::C0, Row::OEX) => Row::XEX,
            (Player::X, SCI::C0, Row::OOE) => Row::XOE,
            (Player::X, SCI::C0, Row::OOO) => Row::XOO,
            (Player::X, SCI::C0, Row::OOX) => Row::XOX,
            (Player::X, SCI::C0, Row::OXE) => Row::XXE,
            (Player::X, SCI::C0, Row::OXO) => Row::XXO,
            (Player::X, SCI::C0, Row::OXX) => Row::XXX,
            (Player::X, SCI::C0, Row::XEE) => Row::XEE,
            (Player::X, SCI::C0, Row::XEO) => Row::XEO,
            (Player::X, SCI::C0, Row::XEX) => Row::XEX,
            (Player::X, SCI::C0, Row::XOE) => Row::XOE,
            (Player::X, SCI::C0, Row::XOO) => Row::XOO,
            (Player::X, SCI::C0, Row::XOX) => Row::XOX,
            (Player::X, SCI::C0, Row::XXE) => Row::XXE,
            (Player::X, SCI::C0, Row::XXO) => Row::XXO,
            (Player::X, SCI::C0, Row::XXX) => Row::XXX,
            (Player::X, SCI::C1, Row::EEE) => Row::EXE,
            (Player::X, SCI::C1, Row::EEO) => Row::EXO,
            (Player::X, SCI::C1, Row::EEX) => Row::EXX,
            (Player::X, SCI::C1, Row::EOE) => Row::EXE,
            (Player::X, SCI::C1, Row::EOO) => Row::EXO,
            (Player::X, SCI::C1, Row::EOX) => Row::EXX,
            (Player::X, SCI::C1, Row::EXE) => Row::EXE,
            (Player::X, SCI::C1, Row::EXO) => Row::EXO,
            (Player::X, SCI::C1, Row::EXX) => Row::EXX,
            (Player::X, SCI::C1, Row::OEE) => Row::OXE,
            (Player::X, SCI::C1, Row::OEO) => Row::OXO,
            (Player::X, SCI::C1, Row::OEX) => Row::OXX,
            (Player::X, SCI::C1, Row::OOE) => Row::OXE,
            (Player::X, SCI::C1, Row::OOO) => Row::OXO,
            (Player::X, SCI::C1, Row::OOX) => Row::OXX,
            (Player::X, SCI::C1, Row::OXE) => Row::OXE,
            (Player::X, SCI::C1, Row::OXO) => Row::OXO,
            (Player::X, SCI::C1, Row::OXX) => Row::OXX,
            (Player::X, SCI::C1, Row::XEE) => Row::XXE,
            (Player::X, SCI::C1, Row::XEO) => Row::XXO,
            (Player::X, SCI::C1, Row::XEX) => Row::XXX,
            (Player::X, SCI::C1, Row::XOE) => Row::XXE,
            (Player::X, SCI::C1, Row::XOO) => Row::XXO,
            (Player::X, SCI::C1, Row::XOX) => Row::XXX,
            (Player::X, SCI::C1, Row::XXE) => Row::XXE,
            (Player::X, SCI::C1, Row::XXO) => Row::XXO,
            (Player::X, SCI::C1, Row::XXX) => Row::XXX,
            (Player::X, SCI::C2, Row::EEE) => Row::EEX,
            (Player::X, SCI::C2, Row::EEO) => Row::EEX,
            (Player::X, SCI::C2, Row::EEX) => Row::EEX,
            (Player::X, SCI::C2, Row::EOE) => Row::EOX,
            (Player::X, SCI::C2, Row::EOO) => Row::EOX,
            (Player::X, SCI::C2, Row::EOX) => Row::EOX,
            (Player::X, SCI::C2, Row::EXE) => Row::EXX,
            (Player::X, SCI::C2, Row::EXO) => Row::EXX,
            (Player::X, SCI::C2, Row::EXX) => Row::EXX,
            (Player::X, SCI::C2, Row::OEE) => Row::OEX,
            (Player::X, SCI::C2, Row::OEO) => Row::OEX,
            (Player::X, SCI::C2, Row::OEX) => Row::OEX,
            (Player::X, SCI::C2, Row::OOE) => Row::OOX,
            (Player::X, SCI::C2, Row::OOO) => Row::OOX,
            (Player::X, SCI::C2, Row::OOX) => Row::OOX,
            (Player::X, SCI::C2, Row::OXE) => Row::OXX,
            (Player::X, SCI::C2, Row::OXO) => Row::OXX,
            (Player::X, SCI::C2, Row::OXX) => Row::OXX,
            (Player::X, SCI::C2, Row::XEE) => Row::XEX,
            (Player::X, SCI::C2, Row::XEO) => Row::XEX,
            (Player::X, SCI::C2, Row::XEX) => Row::XEX,
            (Player::X, SCI::C2, Row::XOE) => Row::XOX,
            (Player::X, SCI::C2, Row::XOO) => Row::XOX,
            (Player::X, SCI::C2, Row::XOX) => Row::XOX,
            (Player::X, SCI::C2, Row::XXE) => Row::XXX,
            (Player::X, SCI::C2, Row::XXO) => Row::XXX,
            (Player::X, SCI::C2, Row::XXX) => Row::XXX,
        }
    }

    /// Returns an 'updated' row by putting the player at the specified column.
    /// This is reference implementation for `play_at`.
    #[allow(dead_code)]
    fn play_at_2(self, sci: SCI, player: Player) -> Row {
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
        if self.is_over() {
            None
        } else {
            Some(match self.last_player() {
                None => FIRST_PLAYER,
                Some(Player::X) => Player::O,
                Some(Player::O) => Player::X,
            })
        }
    }

    /// Returns the winning player of a game.
    pub fn winner(self) -> Option<Player> {
        self.board.winner()
    }
}

impl Board {
    /// Returns the winning player of a board.
    pub fn winner(self) -> Option<Player> {
        let mut win = None;
        for is in BI_WINS.iter() {
            let w0 = self.sboard_at_idx(is[0]).winner();
            let w1 = self.sboard_at_idx(is[1]).winner();
            let w2 = self.sboard_at_idx(is[2]).winner();
            match w0 {
                Some(player) => if (w0 == w1) && (w1 == w2) {
                    win = Some(player);
                    break
                } else {
                    ()
                },
                None => (),
            }
        }
        win
    }
}

// -> bool ---------------------------------------------------------------------

impl Game {
    /// Is the game over (by win or tie)?
    pub fn is_over(self) -> bool {
        !self.board.is_open()
    }

    /// Is the play valid for the given game?
    pub fn is_valid_play(self, p: Play) -> bool {
        if self.is_over() {
            // The game is over; no more plays are allowed.
            false
        } else if self.next_player() == Some(p.player) {
            // The play may or may not be valid.
            self.is_valid_sboard(p) && self.board.is_location_empty(p.loc)
        } else {
            // The player is out of turn.
            false
        }
    }

    /// Is the play in a valid sub-board?
    fn is_valid_sboard(self, play: Play) -> bool {
        match self.last_loc {
            // The first player can play anywhere
            None => true,
            // Subsequent plays are constrained
            Some(loc) => {
                let loc_sbi: SBI = SBI::from_loc(loc);
                let loc_bi: BI = loc_sbi.as_bi();
                let play_bi: BI = BI::from_loc(play.loc);
                let board: Board = self.board;
                if board.is_sboard_open(loc_bi) {
                    // If the sub-board is open (not won or filled), then the
                    // player must play in it.
                    loc_bi == play_bi
                } else {
                    // Otherwise, the player may play in any open sub-board.
                    board.is_sboard_open(play_bi)
                }
            }
        }
    }
}

impl Board {
    /// Is the board open for more plays (i.e. not won or filled)?
    pub fn is_open(self) -> bool {
        !self.is_won() && self.has_open_sboard()
    }

    /// Is the board won by a player?
    pub fn is_won(self) -> bool {
        let is_a_win = |&is: &[BI; 3]| {
            let w0 = self.sboard_at_idx(is[0]).winner();
            let w1 = self.sboard_at_idx(is[1]).winner();
            let w2 = self.sboard_at_idx(is[2]).winner();
            match w0 {
                Some(_) => (w0 == w1) && (w1 == w2),
                None => false,
            }
        };
        BI_WINS.iter().any(is_a_win)
    }

    /// Does the board have an open sub-board?
    fn has_open_sboard(self) -> bool {
        self.sboards().iter().any(|&sb| sb.is_open())
    }

    /// Is the sub-board open (i.e. not won or tied)?
    fn is_sboard_open(self, bi: BI) -> bool {
        self.sboard_at_idx(bi).is_open()
    }

    /// Is the board location empty?
    pub fn is_location_empty(self, loc: Loc) -> bool {
        match self.player_at_loc(loc) {
            Some(_) => false,
            None => true,
        }
    }
}

impl SBoard {
    /// Is the sub-board open for more plays (i.e. not won or filled)?
    pub fn is_open(self) -> bool {
        !self.is_won() && !self.is_filled()
    }

    /// Has a player won the sub-board?
    pub fn is_won(self) -> bool {
        let is_a_win = |&is: &[SBI; 3]| {
            let s0 = self.slot_at_idx(is[0]);
            let s1 = self.slot_at_idx(is[1]);
            let s2 = self.slot_at_idx(is[2]);
            match s0 {
                Slot::Taken(_) => (s0 == s1) && (s1 == s2),
                Slot::Empty => false,
            }
        };
        SBI_WINS.iter().any(is_a_win)
    }

    /// Returns the winning player of a sub-board.
    pub fn winner(self) -> Option<Player> {
        let mut win = None;
        for is in SBI_WINS.iter() {
            let s0 = self.slot_at_idx(is[0]);
            let s1 = self.slot_at_idx(is[1]);
            let s2 = self.slot_at_idx(is[2]);
            match s0 {
                Slot::Taken(player) => if (s0 == s1) && (s1 == s2) {
                    win = Some(player);
                    break
                } else {
                    ()
                },
                Slot::Empty => (),
            }
        }
        win
    }

    /// Is the sub-board filled (i.e. no slots are open)? Note: a filled
    /// sub-board may or may not be won by a player.
    pub fn is_filled(self) -> bool {
        let is_taken = |&slot: &Slot| {
            match slot {
                Slot::Taken(_) => true,
                Slot::Empty => false,
            }
        };
        self.slots().iter().all(is_taken)
    }

    /// Is the sub-board location empty?
    pub fn is_location_empty(self, loc: SLoc) -> bool {
        match self.player_at_loc(loc) {
            None => true,
            Some(_) => false,
        }
    }
}
