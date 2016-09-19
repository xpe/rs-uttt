/// Functions for intelligent behaviors.

use data::*;
use lru_cache::{LruCache};
use std::cmp::Ordering;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benchmarks;

// == cache ====================================================================

pub type Cache = LruCache<Game, Solution>;

/// Constructs and returns a least-recently-used cache.
pub fn new_cache(capacity: usize) -> Cache {
    Cache::new(capacity)
}

// == data structures ==========================================================

/// A solution to a game state, containing (a) the optimal next play (optional)
/// and (b) the resulting outcome.
///
/// For example, if the outcome is a 'win in N plays', this means that the
/// current player will beat an optimal opponent in N plays. However, the
/// current player may beat a non-optimal opponent in fewer than N plays.
///
/// Some specific examples, assuming optimal players:
///
/// 1. See turn 25 in the table below; it is X's move. Let's assume that X's
///    best move will ensure victory in 5 moves (counting the moves of both
///    players), so the 'outcome' of turn 25 is a 'win by X in 5'. In this kind
///    of situation (ie.e X is next to move and will guarantee victory in K
///    turns), K is always odd.
///
/// ```text
///       last                     next
/// turn  player  winner  outcome  player
/// ----  ------  ------  -------  ------
///    0  -       -       .        X
///    1  X       -       .        O
///   ..  .       -       .        .
///   ..  .       -       .        .
///   ..  .       -       .        .
///   25  O       -       Win X 5  X
///   31  X       -       Win X 4  O
///   32  O       -       Win X 3  X
///   33  X       -       Win X 2  O
///   34  O       -       Win X 1  X
///   35  X       X       Win X 0  -
/// ```
///
/// 2. If O reasons he cannot win or tie against an optimal opponent, then his
///    optimal sequence of moves will delay defeat as long as possible. For
///    example, look at the table above at turn 31; it is O's turn. The best O
///    can do is delay X's win by 4 moves, so the outcome of turn 31 is 'win by
///    X in 4'. Note that such a number must be even.
///
/// 3. If X's best move is to put O in a position so that a tie is optimal for O
///    (meaning that O can do no better than tie), then the outcome will be
///    'tie in 2', meaning that a tie is 2 plays away.
///
/// 4. If X is only willing/able to compute 10 moves, it is possible that a
///    solution will not be found. In such a situation, the optional next play
///    will be `None` and the outcome will be 'unknown for depth 10'.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Solution {
    /// The optimal next play (optional)
    pub opt_play: Option<Play>,
    /// The resulting outcome
    pub outcome: Outcome,
}

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

// == data accessors ===========================================================

impl Outcome {
    pub fn turns(self) -> Count {
        match self {
            Outcome::Win { player: _, turns: t } => t,
            Outcome::Tie { turns: t } => t,
            Outcome::Unknown { turns: t } => t,
        }
    }
}

// == solving functions ========================================================

impl Game {
    /// Returns the solution for a given depth and game. This function looks
    /// ahead for a number of moves (specified by `depth`). If there are no
    /// valid moves, returns a solution where the optional play is `None` and
    /// the outcome is either a win or a tie.
    pub fn solve_for(&self, depth: Count, cache: &mut Cache) -> Solution {
        let s1 = match cache.get_mut(self) {
            Some(sol_mut) => Some(*sol_mut),
            None => None,
        };
        let s2 = match s1 {
            // a cache hit
            Some(solution) => Some(match solution.outcome {
                Outcome::Unknown { turns: t } if t < depth => {
                    self.solve_for_cached(depth, cache)
                },
                _ => solution,
            }),
            None => None,
        };
        match s2 {
            Some(solution) => solution,
            // a cache miss
            None => {
                let solution = self.solve_for_cached(depth, cache);
                cache.insert(*self, solution);
                solution
            },
        }
    }

    /// Cached. Supporting function for `solve_for`.
    fn solve_for_cached(&self, depth: Count, cache: &mut Cache) -> Solution {
        if depth == 0 {
            self.solve_depth_0()
        } else if depth > 0 {
            self.solve_depth_cached(depth, cache)
        } else {
            panic!("Internal Error: depth < 0");
        }
    }

    /// Uncached. Returns the solution for a given depth and game. This function
    /// looks ahead for a number of moves (specified by `depth`). If there are
    /// no valid moves, returns a solution where the optional play is `None` and
    /// the outcome is either a win or a tie.
    pub fn solve_for_uncached(&self, depth: Count) -> Solution {
        if depth == 0 {
            self.solve_depth_0()
        } else if depth > 0 {
            self.solve_depth_uncached(depth)
        } else {
            panic!("Internal Error: depth < 0");
        }
    }

    /// Returns the solution for depth == 0. This function supports both cached
    /// and uncached solving.
    fn solve_depth_0(&self) -> Solution {
        match self.state() {
            GameState::Won(player) => Solution {
                opt_play: None,
                outcome: Outcome::Win { player: player, turns: 0 },
            },
            GameState::Tied => Solution {
                opt_play: None,
                outcome: Outcome::Tie { turns: 0 },
            },
            GameState::Ongoing => Solution {
                opt_play: None,
                outcome: Outcome::Unknown { turns: 0 },
            },
        }
    }

    /// Cached. Returns the solution for depth == k. To solve this, it first
    /// solves depth == k - 1.
    fn solve_depth_cached(&self, depth: Count, cache: &mut Cache) -> Solution {
        let sol = self.solve_for(depth - 1, cache);
        match sol.dominant(self.next_player(), depth) {
            Some(dom) => dom,
            None => self.solve_only_cached(depth, sol.opt_solution(), cache),
        }
    }

    /// Uncached. Returns the solution for depth == k. To solve this, it first
    /// solves depth == k - 1.
    fn solve_depth_uncached(&self, depth: Count) -> Solution {
        let sol = self.solve_for_uncached(depth - 1);
        match sol.dominant(self.next_player(), depth) {
            Some(dom) => dom,
            None => self.solve_only_uncached(depth, sol.opt_solution()),
        }
    }

    /// Cached. Returns the solution for a particular depth (not lower depths)
    /// and an optional 'shallower' solution. If there is a actionable solution
    /// (containing a play) from the lower depths, pass in `Some(_)` for the
    /// 'shallow solution' argument. Otherwise, pass in `None`.
    fn solve_only_cached(&self, depth: Count, shallow: Option<Solution>,
                         cache: &mut Cache) -> Solution {
        let player = self.next_player().unwrap();
        let solutions = self.candidate_solutions_cached(depth, cache);
        let merged = merge_solutions(shallow, solutions);
        best_solution(player, merged)
    }

    /// Uncached. Returns the solution for a particular depth (not lower depths)
    /// and an optional 'shallower' solution. If there is a actionable solution
    /// (containing a play) from the lower depths, pass in `Some(_)` for the
    /// 'shallow solution' argument. Otherwise, pass in `None`.
    fn solve_only_uncached(&self, depth: Count, shallow: Option<Solution>)
                           -> Solution {
        let player = self.next_player().unwrap();
        let solutions = self.candidate_solutions_uncached(depth);
        let merged = merge_solutions(shallow, solutions);
        best_solution(player, merged)
    }

    /// Cached. Returns candidate solutions (i.e. possible solutions) for an
    /// exact depth; does not consider lower depths.
    fn candidate_solutions_cached(&self, depth: Count,
                                  cache: &mut Cache) -> Vec<Solution> {
        self.valid_plays().iter().map(|&play| {
            let mut game = self.clone();
            game.play(play);
            game.solve_for(depth - 1, cache).time_shift(play)
        }).collect::<Vec<Solution>>()
    }

    /// Uncached. Returns candidate solutions (i.e. possible solutions) for an
    /// exact depth; does not consider lower depths.
    fn candidate_solutions_uncached(&self, depth: Count) -> Vec<Solution> {
        self.valid_plays().iter().map(|&play| {
            let mut game = self.clone();
            game.play(play);
            game.solve_for_uncached(depth - 1).time_shift(play)
        }).collect::<Vec<Solution>>()
    }
}

/// Returns the best solution.
fn best_solution(p: Player, ss: Vec<Solution>) -> Solution {
    let mut xs = ss.clone();
    xs.sort_by(|a, b| Solution::compare(p, *a, *b));
    xs.first().unwrap().clone()
}

impl Solution {
    /// If a given solution is dominant, return it. A dominant solution is one
    /// good enough such that there is no need in searching for others.
    fn dominant(self, opt_player: Option<Player>,
                depth: Count) -> Option<Solution> {
        match self.outcome {
            Outcome::Win { player: p, turns: t } => {
                if t == 0 {
                    Some(self)
                } else if t < depth {
                    match opt_player {
                        Some(player) if p == player => Some(self),
                        _ => None,
                    }
                } else {
                    None
                }
            },
            Outcome::Tie { turns: t } if t == 0 => Some(self),
            _ => None,
        }
    }
}

// == helpers for solving functions ============================================

impl Solution {
    /// Helper function for `Game::solve_depth`.
    fn opt_solution(self) -> Option<Solution> {
        match self.opt_play {
            Some(_) => Some(self),
            None => None,
        }
    }
}

/// Returns the combined (merged) solution vector from an optional solution and
/// a vector of solutions.
fn merge_solutions(opt_solution: Option<Solution>,
                   solutions: Vec<Solution>) -> Vec<Solution> {
    match opt_solution {
        None => solutions,
        Some(solution) => {
            let mut ss = solutions;
            ss.push(solution);
            ss.clone()
        },
    }
}

// == 'modifiers' ==============================================================

impl Solution {
    /// Returns an 'update' time-shifted solution by setting the play field and
    /// incrementing the associated count on the outcome field.
    fn time_shift(self, play: Play) -> Solution {
        Solution {
            opt_play: Some(play),
            outcome: self.outcome.inc(),
        }
    }
}

impl Outcome {
    /// Returns an 'updated' outcome by incrementing the 'turns' for the given
    /// outcome.
    fn inc(self) -> Outcome {
        match self {
            Outcome::Win { player: p, turns: k } =>
                Outcome::Win { player: p, turns: k + 1 },
            Outcome::Tie { turns: k } =>
                Outcome::Tie { turns: k + 1 },
            Outcome::Unknown { turns: k } =>
                Outcome::Unknown { turns: k + 1 },
        }
    }
}

// == comparators ==============================================================

impl Solution {
    /// Compare two solutions.
    fn compare(p: Player, a: Solution, b: Solution) -> Ordering {
        let play_a = a.opt_play.unwrap();
        let play_b = b.opt_play.unwrap();
        if play_a == play_b {
            let turns_a = a.outcome.turns();
            let turns_b = b.outcome.turns();
            turns_b.cmp(&turns_a)
        } else {
            Outcome::compare(p, a.outcome, b.outcome)
        }
    }
}

impl Outcome {
    /// Compare two outcomes from the point of view of the given player.
    fn compare(p: Player, a: Outcome, b: Outcome) -> Ordering {
        let o = p.opponent();
        match (a, b) {
            (Outcome::Win { player: p1, turns: t1 },
             Outcome::Win { player: p2, turns: t2 }) => {
                if p1 == p && p2 == o {
                    Ordering::Less // prefer to win, not lose
                } else if p1 == o && p2 == p {
                    Ordering::Greater // prefer to win, not lose
                } else if p1 == p && p2 == p {
                    t1.cmp(&t2) // prefer to win sooner, not later
                } else { // p1 == o && p2 == o
                    t2.cmp(&t1) // prefer to lose later, not sooner
                }
            },
            (Outcome::Win { player: p1, turns: _ },
             Outcome::Tie { turns: _ }) => {
                if p1 == p {
                    Ordering::Less // prefer to win, not tie
                } else { // p1 == o
                    Ordering::Greater // prefer to tie, not lose
                }
            },
            (Outcome::Tie { turns: _ },
             Outcome::Win { player: p2, turns: _ }) => {
                if p2 == o {
                    Ordering::Less // prefer to tie, not lose
                } else { // p2 == p
                    Ordering::Greater // prefer to win, not tie
                }
            },
            (Outcome::Win { player: p1, turns: _ },
             Outcome::Unknown { turns: _ }) => {
                if p1 == p {
                    Ordering::Less // prefer to win over the unknown
                } else { // p1 == o
                    Ordering::Greater // prefer the unknown over losing
                }
            },
            (Outcome::Unknown { turns: _ },
             Outcome::Win { player: p2, turns: _ }) => {
                if p2 == o {
                    Ordering::Less // prefer the unknown over losing
                } else { // p2 == p
                    Ordering::Greater // prefer to win over the unknown
                }
            },
            (Outcome::Tie { turns: t1 },
             Outcome::Tie { turns: t2 }) => {
                t2.cmp(&t1) // prefer to tie later, not sooner
            },
            (Outcome::Tie { turns: _ },
             Outcome::Unknown { turns: _ }) => {
                Ordering::Greater // prefer the unknown over the tie
            },
            (Outcome::Unknown { turns: _ },
             Outcome::Tie { turns: _ }) => {
                Ordering::Less // prefer the unknown over the tie
            },
            (Outcome::Unknown { turns: t1 },
             Outcome::Unknown { turns: t2 }) => {
                t2.cmp(&t1) // Prefer the unknown later, not sooner
            },
        }
    }
}
