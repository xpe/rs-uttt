/// Functions for intelligent behaviors.

use data::*;
use lru_time_cache::LruCache;
use std::cmp::Ordering;

pub type Depth = i16;

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
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Outcome {
    Win { player: Player, turns: Count },
    Tie { turns: Count },
    Unknown { depth: Depth },
}

// == solving functions ========================================================

impl Game {
    /// Returns the solution for a given depth and game. This function looks
    /// ahead for a number of moves (specified by `depth`). If there are no
    /// valid moves, returns a solution where the optional play is `None` and
    /// the outcome is either a win or a tie.
    pub fn solve_for(self, depth: Depth) -> Solution {
        print!(".");
        if depth == 0 {
            self.solve_depth_0()
        } else if depth > 0 {
            self.solve_depth(depth)
        } else {
            panic!("Internal Error: depth < 0");
        }
    }

    /// Returns the solution for depth == 0.
    fn solve_depth_0(self) -> Solution {
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
                outcome: Outcome::Unknown { depth: 0 },
            },
        }
    }

    /// Returns the solution for depth == k. To solve this, it first solves
    /// depth == k - 1.
    fn solve_depth(self, depth: Depth) -> Solution {
        let sol = self.solve_for(depth - 1);
        match sol.dominant() {
            Some(dom_sol) => dom_sol,
            None => {
                let opt_sol = match sol {
                    Solution { opt_play: Some(_),
                               outcome: _ } => {
                        Some(sol)
                    },
                    Solution { opt_play: None,
                               outcome: Outcome::Win { .. } } => {
                        // Should be handled by `dominant()`, above.
                        panic!("Internal Error: unexpected match");
                    },
                    Solution { opt_play: None,
                               outcome: Outcome::Tie { .. } } => {
                        // Should be handled by `dominant()`, above.
                        panic!("Internal Error: unexpected match");
                    },
                    Solution { opt_play: None,
                               outcome: Outcome::Unknown { .. } } => {
                        // TODO: maybe room for improvement here?
                        None
                    }
                };
                self.solve_only(depth, opt_sol)
            },
        }
    }

    /// Returns the solution for a particular depth (not lower depths) and an
    /// optional 'shallower' solution.
    ///
    /// Note about the 'shallower' solution: If there is a actionable solution
    /// (containing a play) from the lower depths, pass in `Some(_)`. Otherwise,
    /// pass in `None`.
    fn solve_only(self, depth: Depth, shallow: Option<Solution>) -> Solution {
        let player = self.next_player().unwrap();
        let merged = merge_solutions(shallow, self.candidate_solutions(depth));
        best_solution(player, merged)
    }

    /// Returns candidate solutions (i.e. possible solutions) for an exact
    /// depth; does not consider lower depths.
    fn candidate_solutions(self, depth: Depth) -> Vec<Solution> {
        let valid_plays = self.valid_plays();
        let solutions = valid_plays.iter().map(|&play| {
            self.play(play).unwrap().solve_for(depth - 1).time_shift(play)
        }).collect::<Vec<Solution>>();
        if solutions.is_empty() {
            panic!("Internal Error: `solutions` is empty");
        }
        solutions
    }
}

fn merge_solutions(shallow: Option<Solution>,
                   solutions: Vec<Solution>) -> Vec<Solution> {
    match shallow {
        None => solutions,
        Some(solution) => {
            let mut xs = solutions;
            xs.push(solution);
            xs.clone()
        },
    }
}

/// Returns the best solution.
fn best_solution(p: Player, ss: Vec<Solution>) -> Solution {
    let mut xs = ss.clone();
    xs.sort_by(|a, b| Solution::compare(p, a, b));
    xs.first().unwrap().clone()
}

impl Solution {
    /// If a given solution is dominant, return it. A dominant solution is one
    /// good enough such that there is no need in searching for others.
    fn dominant(self) -> Option<Solution> {
        match self.outcome {
            Outcome::Win { player: _, turns: t } if t == 0 => Some(self),
            Outcome::Tie { turns: t } if t == 0 => Some(self),
            _ => None,
        }
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
    /// Returns an 'updated' outcome by incrementing the 'turns' or 'depth' (as
    /// appropriate) for the given outcome.
    fn inc(self) -> Outcome {
        match self {
            Outcome::Win { player: p, turns: k } =>
                Outcome::Win { player: p, turns: k + 1 },
            Outcome::Tie { turns: k } =>
                Outcome::Tie { turns: k + 1 },
            Outcome::Unknown { depth: k } =>
                Outcome::Unknown { depth: k + 1 },
        }
    }
}

// == comparators ==============================================================

impl Solution {
    /// Compare two solutions.
    fn compare(p: Player, a: &Solution, b: &Solution) -> Ordering {
        Outcome::compare(p, &a.outcome, &b.outcome)
    }
}

impl Outcome {
    /// Compare two outcomes from the point of view of the given player.
    fn compare(p: Player, a: &Outcome, b: &Outcome) -> Ordering {
        let o = p.opponent();
        match (*a, *b) {
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
             Outcome::Unknown { depth: _ }) => {
                if p1 == p {
                    Ordering::Less // prefer to win over the unknown
                } else { // p1 == o
                    Ordering::Greater // prefer the unknown over losing
                }
            },
            (Outcome::Unknown { depth: _ },
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
             Outcome::Unknown { depth: _ }) => {
                Ordering::Greater // prefer the unknown over the tie
            },
            (Outcome::Unknown { depth: _ },
             Outcome::Tie { turns: _ }) => {
                Ordering::Less // prefer the unknown over the tie
            },
            (Outcome::Unknown { depth: d1 },
             Outcome::Unknown { depth: d2 }) => {
                d2.cmp(&d1) // Prefer the unknown later, not sooner
            },
        }
    }
}

// == cache ====================================================================

/// Constructs and returns a least-recently-used cache.
pub fn outcome_cache(capacity: usize) -> LruCache<Game, Outcome> {
    LruCache::<Game, Outcome>::with_capacity(capacity)
}
