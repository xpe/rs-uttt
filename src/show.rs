/// Show (text output) functions.

use data::*;

pub trait Show {
    fn show(&self) -> String;
}

// -- game ---------------------------------------------------------------------

impl Show for Game {
    fn show(&self) -> String {
        format!("{:?}", self)
    }
}

// -- board --------------------------------------------------------------------

impl Show for Board {
    fn show(&self) -> String {
        let hl = "───┼───┼───  ───┼───┼───  ───┼───┼───";
        let s: [[Slot; 9]; 9] = self.slots_9x9();
        format!(
            "{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n{}",
            s[0].show(), hl, s[1].show(), hl, s[2].show(),
            s[3].show(), hl, s[4].show(), hl, s[5].show(),
            s[6].show(), hl, s[7].show(), hl, s[8].show()
        )
    }
}

// -- sub-board ----------------------------------------------------------------

impl Show for SBoard {
    fn show(&self) -> String {
        let rows: [Row; 3] = self.rows();
        format!(
            "{}\n{}\n{}",
            rows[0].show(),
            rows[1].show(),
            rows[2].show()
        )
    }
}

// -- row ----------------------------------------------------------------------

impl Show for Row {
    fn show(&self) -> String {
        self.slots().show()
    }
}

// -- board play ---------------------------------------------------------------

// -- sub-board play -----------------------------------------------------------

// -- board location -----------------------------------------------------------

// -- sub-board location -------------------------------------------------------

// -- slots --------------------------------------------------------------------

impl Show for [Slot; 9] {
    fn show(&self) -> String {
        format!(
            " {} │ {} │ {}    {} │ {} │ {}    {} │ {} │ {} ",
            self[0].show(),
            self[1].show(),
            self[2].show(),
            self[3].show(),
            self[4].show(),
            self[5].show(),
            self[6].show(),
            self[7].show(),
            self[8].show(),
        )
    }
}

impl Show for [Slot; 3] {
    fn show(&self) -> String {
        format!(
            " {} │ {} │ {} ",
            self[0].show(),
            self[1].show(),
            self[2].show(),
        )
    }
}

// -- slot ---------------------------------------------------------------------

impl Show for Slot {
    fn show(&self) -> String {
        match *self {
            Slot::Empty => " ", // "·"
            Slot::Taken(Player::X) => "X",
            Slot::Taken(Player::O) => "O",
        }.to_string()
    }
}

// -- board indexes ------------------------------------------------------------

// -- sub-board indexes --------------------------------------------------------

// -- player -------------------------------------------------------------------
