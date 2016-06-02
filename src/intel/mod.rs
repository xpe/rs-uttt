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
                None => None,
                None => None, // outcome unknown for depth=0
                Some(Player::X) => Some(Outcome::WinX(0)),
                Some(Player::O) => Some(Outcome::WinO(0)),
            },
            1 => {
                let outcomes = self.valid_plays().iter()
                    .map(|&play| self.play(play).unwrap().outcome_for(0))
                    .collect::<Vec<Option<Outcome>>>();
                if outcomes.is_empty() {
                    None
                } else {
                    unimplemented!()
                }
            },
            _ => {
                unimplemented!()
            },
        }
    }
}
