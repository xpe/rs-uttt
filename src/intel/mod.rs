/// Functions for intelligent behaviors.

use data::*;
use lru_time_cache::LruCache;

/// The computed outcome of a game; either:
/// 1. A win for player X in some number of turns
/// 2. A win for player O in some number of turns
/// 3. A tie in some number of turns
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Outcome {
    WinX(u8),
    WinO(u8),
    Tie(u8)
}

/// Constructs and returns a least-recently-used cache.
pub fn outcome_cache(capacity: usize) -> LruCache<Game, Outcome> {
    LruCache::<Game, Outcome>::with_capacity(capacity)
}

impl Game {
    /// Returns the outcome of a game for up to `depth` moves.
    pub fn outcome_for(self, depth: u8) -> Option<Outcome> {
        match depth {
            0 => match self.winner() {
                // TODO: differentiate between a tie and more plays remaining
                // ? => Some(Outcome::Tie(0))
                None => None, // outcome unknown for depth=0
                Some(Player::X) => Some(Outcome::WinX(0)),
                Some(Player::O) => Some(Outcome::WinO(0)),
            },
            1 => {
                let outcomes = self.valid_plays().iter()
                    .map(|&play| self.play(play).unwrap().outcome_for(0))
                    .collect::<Vec<Option<Outcome>>>();
                // Some possible `outcomes`:
                //
                // vec![]
                // (no valid plays)
                // Note: Perhaps this path should not be reached because depth=0
                // should be checked first!
                //
                // vec![Some(WinX(0))]
                // (one valid play; it wins the game for X)
                //
                // vec![Some(WinO(0))]
                // (one valid play; it wins the game for O)
                //
                // vec![Some(Tie(0))]
                // (one valid play; it ties the game)
                //
                // vec![None]
                // (one valid play; outcome unknown)
                //
                // vec![Some(WinX(0)), None]
                // (two valid plays: a win for X or an unknown outcome)
                //
                // vec![Some(WinO(0)), None]
                // (two valid plays: a win for O or an unknown outcome)
                //
                // impossible: vec![Some(WinO(0)), Some(WinX(0))]
                // (impossible; only X or O can win on the next play)
                if outcomes.is_empty() {
                    None
                } else {
                    // These are the possible return values:
                    //
                    // If the next player is X:
                    // Some(Outcome::WinX(0))
                    // Some(Outcome::WinO(1))
                    // Some(Outcome::Tie(0))
                    // Some(Outcome::Tie(1))
                    // None
                    //
                    // If the next player is O:
                    // Some(Outcome::WinO(0))
                    // Some(Outcome::WinX(1))
                    // Some(Outcome::Tie(0))
                    // Some(Outcome::Tie(1))
                    // None
                    unimplemented!()
                }
            },
            _ => {
                unimplemented!()
            },
        }
    }
}
