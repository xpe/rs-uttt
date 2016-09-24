use data::{Player, Count};

/// The computed outcome of a game; either:
/// 1. A win for player X in some number of plays
/// 2. A win for player O in some number of plays
/// 3. A tie in some number of turns
/// 4. Not one of the above, so unknown.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Outcome {
    Win { player: Player, turns: Count },
    Tie { turns: Count },
    Unknown { turns: Count },
}

impl Outcome {
    pub fn turns(self) -> Count {
        match self {
            Outcome::Win { player: _, turns: t } => t,
            Outcome::Tie { turns: t } => t,
            Outcome::Unknown { turns: t } => t,
        }
    }
}
