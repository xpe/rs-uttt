/// Show (text output) functions.

use data::*;
use solver::solution::Solution;
use solver::outcome::Outcome;

pub trait Show {
    fn show(&self) -> String;
}

// -- solution -----------------------------------------------------------------

impl Show for Vec<Solution> {
    fn show(&self) -> String {
        format!("[{}]", self.iter().map(
            |solution| solution.show()
        ).collect::<Vec<String>>().join(", "))
    }
}

impl Show for Option<Solution> {
    fn show(&self) -> String {
        match *self {
            None => "None".to_string(),
            Some(solution) => solution.show(),
        }
    }
}

impl Show for Solution {
    fn show(&self) -> String {
        format!("❨Play:{} Outcome:{}❩",
                self.opt_play.show(), self.outcome.show())
    }
}

// -- outcome ------------------------------------------------------------------

impl Show for Outcome {
    fn show(&self) -> String {
        match *self {
            Outcome::Win { player: p, turns: t } => {
                format!("❨Win:{} {}❩", p.show(), t.show())
            },
            Outcome::Tie { turns: t } => {
                format!("❨Tie:{}❩", t.show())
            },
            Outcome::Unknown { turns: t } => {
                format!("❨Unknown:{}❩", t.show())
            },
        }
    }
}

// -- game ---------------------------------------------------------------------

impl Show for Game {
    fn show(&self) -> String {
        format!(
            "{}\n{}",
            self.board.show(),
            match self.state() {
                GameState::Won(_) => {
                    format!("    n={:2}      last={}:{}      {} won",
                            self.board.play_count(),
                            self.last_player().show(),
                            self.last_loc.show(),
                            self.winner().show()
                    )
                },
                GameState::Tied => {
                    format!("    n={:2}      last={}:{}          tie",
                            self.board.play_count(),
                            self.last_player().show(),
                            self.last_loc.show()
                    )
                },
                GameState::Ongoing => {
                    format!("    n={:2}      last={}:{}      ongoing",
                            self.board.play_count(),
                            self.last_player().show(),
                            self.last_loc.show()
                    )
                },
            }
        )
    }
}

// -- board --------------------------------------------------------------------

impl Show for Board {
    fn show(&self) -> String {
        let s: [[Slot; 9]; 9] = reorder_slots(self.slots_9x9());
        let cols = "     0   1   2    3   4   5    6   7   8     ";
        let line = "    ───┼───┼───  ───┼───┼───  ───┼───┼───    ";
        format!(
            "{}\n\
             \n\
             0   {}   0\n\
             {}\n\
             1   {}   1\n\
             {}\n\
             2   {}   2\n\
             \n\
             3   {}   3\n\
             {}\n\
             4   {}   4\n\
             {}\n\
             5   {}   5\n\
             \n\
             6   {}   6\n\
             {}\n\
             7   {}   7\n\
             {}\n\
             8   {}   8\n\
             \n\
             {}",
            cols,
            s[0].show(), line, s[1].show(), line, s[2].show(),
            s[3].show(), line, s[4].show(), line, s[5].show(),
            s[6].show(), line, s[7].show(), line, s[8].show(),
            cols
        )
    }
}

/// Reorders the 9x9 slots in a useful order for displaying:
///
/// In case it is useful, here is the 'flattened' output order:
///
/// row 0:  0  1  2  3  4  5  6  7  8
/// row 1:  9 10 11 12 13 14 15 16 17
/// row 2: 18 19 20 21 22 23 24 25 26
/// row 3: 27 28 29 30 31 32 33 34 35
/// row 4: 36 37 38 39 40 41 42 43 44
/// row 5: 45 46 47 48 49 50 51 52 53
/// row 6: 54 55 56 57 58 59 60 61 62
/// row 7: 63 64 65 66 67 68 69 70 71
/// row 8: 72 73 74 75 76 77 78 79 80
fn reorder_slots(s: [[Slot; 9]; 9]) -> [[Slot; 9]; 9] {
    [
        [
            s[0][0], s[0][1], s[0][2],
            s[1][0], s[1][1], s[1][2],
            s[2][0], s[2][1], s[2][2],
        ],
        [
            s[0][3], s[0][4], s[0][5],
            s[1][3], s[1][4], s[1][5],
            s[2][3], s[2][4], s[2][5],
        ],
        [
            s[0][6], s[0][7], s[0][8],
            s[1][6], s[1][7], s[1][8],
            s[2][6], s[2][7], s[2][8],
        ],
        [
            s[3][0], s[3][1], s[3][2],
            s[4][0], s[4][1], s[4][2],
            s[5][0], s[5][1], s[5][2],
        ],
        [
            s[3][3], s[3][4], s[3][5],
            s[4][3], s[4][4], s[4][5],
            s[5][3], s[5][4], s[5][5],
        ],
        [
            s[3][6], s[3][7], s[3][8],
            s[4][6], s[4][7], s[4][8],
            s[5][6], s[5][7], s[5][8],
        ],
        [
            s[6][0], s[6][1], s[6][2],
            s[7][0], s[7][1], s[7][2],
            s[8][0], s[8][1], s[8][2],
        ],
        [
            s[6][3], s[6][4], s[6][5],
            s[7][3], s[7][4], s[7][5],
            s[8][3], s[8][4], s[8][5],
        ],
        [
            s[6][6], s[6][7], s[6][8],
            s[7][6], s[7][7], s[7][8],
            s[8][6], s[8][7], s[8][8],
        ],
    ]
}

// -- sub-board ----------------------------------------------------------------

impl Show for SBoard {
    fn show(&self) -> String {
        let rows: [Row; 3] = self.rows();
        format!("{}\n{}\n{}", rows[0].show(), rows[1].show(), rows[2].show())
    }
}

// -- row ----------------------------------------------------------------------

impl Show for Row {
    fn show(&self) -> String {
        self.slots().show()
    }
}

// -- board play ---------------------------------------------------------------

impl Show for Vec<Play> {
    fn show(&self) -> String {
        format!("[{}]", self.iter().map(
            |play| play.show()
        ).collect::<Vec<String>>().join(", "))
    }
}

impl Show for Option<Play> {
    fn show(&self) -> String {
        match *self {
            None => "None".to_string(),
            Some(play) => play.show(),
        }
    }
}

impl Show for Play {
    fn show(&self) -> String {
        format!("❨{}:{}❩", self.player.show(), self.loc.show())
    }
}

// -- sub-board play -----------------------------------------------------------

impl Show for SPlay {
    fn show(&self) -> String {
        format!("❨{} {}❩", self.player.show(), self.loc.show())
    }
}

// -- board location -----------------------------------------------------------

impl Show for Option<Loc> {
    fn show(&self) -> String {
        match *self {
            None => "❨--,--❩".to_string(),
            Some(loc) => loc.show(),
        }
    }
}

impl Show for Loc {
    fn show(&self) -> String {
        format!("❨{},{}❩", self.row().show(), self.col().show())
    }
}

// -- sub-board location -------------------------------------------------------

impl Show for SLoc {
    fn show(&self) -> String {
        format!("❨{},{}❩", self.row.show(), self.col.show())
    }
}

// -- slots --------------------------------------------------------------------

impl Show for [Slot; 9] {
    fn show(&self) -> String {
        format!(
            " {} │ {} │ {}    {} │ {} │ {}    {} │ {} │ {} ",
            self[0].show(), self[1].show(), self[2].show(),
            self[3].show(), self[4].show(), self[5].show(),
            self[6].show(), self[7].show(), self[8].show(),
        )
    }
}

impl Show for [Slot; 3] {
    fn show(&self) -> String {
        format!(" {} │ {} │ {} ",
                self[0].show(), self[1].show(), self[2].show())
    }
}

// -- slot ---------------------------------------------------------------------

impl Show for Slot {
    fn show(&self) -> String {
        match *self {
            Slot::Empty => " ",
            Slot::Taken(Player::X) => "X",
            Slot::Taken(Player::O) => "O",
        }.to_string()
    }
}

// -- board indexes ------------------------------------------------------------

impl Show for RI {
    fn show(&self) -> String {
        format!("R{}", self.as_u8())
    }
}

impl Show for CI {
    fn show(&self) -> String {
        format!("C{}", self.as_u8())
    }
}

// -- sub-board indexes --------------------------------------------------------

impl Show for SRI {
    fn show(&self) -> String {
        format!("SR{}", self.as_u8())
    }
}

impl Show for SCI {
    fn show(&self) -> String {
        format!("SC{}", self.as_u8())
    }
}

// -- player -------------------------------------------------------------------

impl Show for Option<Player> {
    fn show(&self) -> String {
        match *self {
            None => "-".to_string(),
            Some(player) => player.show(),
        }
    }
}

impl Show for Player {
    fn show(&self) -> String {
        match *self {
            Player::X => "X",
            Player::O => "O",
        }.to_string()
    }
}

// -- count --------------------------------------------------------------------

impl Show for Count {
    fn show(&self) -> String {
        format!("{}", self)
    }
}
