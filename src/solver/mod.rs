/// Functions for intelligent behaviors.

use data::*;
use lru_time_cache::LruCache;
use std::cmp::Ordering;

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
/// 1. See turn 30 in the table below; it is X's move. Let's assume that X's
///    best move will ensure victory in 5 moves (counting the moves of both
///    players), so the 'outcome' of turn 30 is a 'win by X in 5'. In this kind
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
///   30  O       -       Win X 5  X
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
    Unknown { depth: Count },
}

// == solving functions ========================================================

impl Game {
    /// Returns the solution for a given depth and game. This function looks
    /// ahead for a number of moves (specified by `depth`). If there are no
    /// valid moves, returns a solution where the optional play is `None` and
    /// the outcome is either a win or a tie.
    pub fn solve_for(self, depth: Count) -> Solution {
        match depth {
            0 => self.solve_0(),
            1 => self.solve_1(),
            2 => self.solve_2(),
            _ => unimplemented!(),
        }
    }

    /// Returns the solution for depth == 0.
    fn solve_0(self) -> Solution {
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

    /// Returns the solution for depth == 1.
    fn solve_1(self) -> Solution {
        let solution = self.solve_0();
        match solution.outcome {
            Outcome::Win { .. } => solution,
            Outcome::Tie { .. } => solution,
            Outcome::Unknown { .. } => {
                let player = self.next_player().unwrap();
                solve_only_1(self, player, solution)
            },
        }
    }

    /// Returns the solution for depth == 2.
    fn solve_2(self) -> Solution { // WIP
        let solution = self.solve_1();
        let player = self.next_player().unwrap();
        match solution.outcome {
            Outcome::Win { player: p, turns: _ } => {
                if p == player {
                    solution
                } else {
                    solve_only_2(self, player, solution)
                }
            },
            Outcome::Tie { .. } => {
                solve_only_2(self, player, solution)
            },
            Outcome::Unknown { .. } => {
                solve_only_2(self, player, solution)
            },
        }
    }
}

// -- solve_only_? -------------------------------------------------------------

/// Returns the solution for depth == 1, but not lower depths.
fn solve_only_1(game: Game, player: Player, sol_0: Solution) -> Solution {
    let solutions_1 = candidate_solutions_1(game);
    best_solution(player, sol_0, solutions_1)
}

/// Returns the solution for depth == 2, but not lower depths.
fn solve_only_2(game: Game, player: Player, sol_1: Solution) -> Solution {
    let solutions_2 = candidate_solutions_2(game);
    best_solution(player, sol_1, solutions_2)
}

// -- candidate_solutions_? ----------------------------------------------------

/// Returns candidate solutions (i.e. possible solutions) at depth == 1. Does
/// not consider lower depths.
fn candidate_solutions_1(game: Game) -> Vec<Solution> {
    let solutions = game.valid_plays().iter().map(|&play| {
        game.play(play).unwrap().solve_0().time_shift(play)
    }).collect::<Vec<Solution>>();
    if solutions.is_empty() {
        panic!("Internal Error: `solutions` is empty");
    }
    solutions
}

/// Returns candidate solutions (i.e. possible solutions) at depth == 2. Does
/// not consider lower depths.
fn candidate_solutions_2(game: Game) -> Vec<Solution> {
    let solutions = game.valid_plays().iter().map(|&play| {
        game.play(play).unwrap().solve_1().time_shift(play)
    }).collect::<Vec<Solution>>();
    if solutions.is_empty() {
        panic!("Internal Error: `solutions` is empty");
    }
    solutions
}

// -- best_solution ------------------------------------------------------------

/// Returns the best solution.
fn best_solution(p: Player, solution: Solution,
                 solutions: Vec<Solution>) -> Solution {
    let mut ss = solutions;
    ss.push(solution);
    ss.sort_by(|a, b| Solution::compare(p, a, b));
    ss.last().unwrap().clone()
}

// == 'modifiers' ==============================================================

impl Solution {
    /// Returns an 'update' time-shifted solution by setting the play field and
    /// incrementing the associated count on the outcome field.
    fn time_shift(self, play: Play) -> Solution {
        Solution {
            opt_play: Some(play),
            outcome: self.outcome.inc_count(),
        }
    }
}

impl Outcome {
    /// Returns an 'updated' outcome by incrementing the 'turns' or 'depth' (as
    /// appropriate) for the given outcome.
    fn inc_count(self) -> Outcome {
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
                    Ordering::Greater
                } else if p1 == o && p2 == p {
                    Ordering::Less
                } else {
                    t2.cmp(&t1)
                }
            },
            (Outcome::Win { player: p1, turns: _ },
             Outcome::Tie { turns: _ }) => {
                if p1 == p {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            },
            (Outcome::Tie { turns: _ },
             Outcome::Win { player: p1, turns: _ }) => {
                if p1 == o {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            },
            (Outcome::Win { player: p1, turns: _ },
             Outcome::Unknown { depth: _ }) => {
                if p1 == p {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            },
            (Outcome::Unknown { depth: _ },
             Outcome::Win { player: p1, turns: _ }) => {
                if p1 == o {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            },
            (Outcome::Tie { turns: t1 },
             Outcome::Tie { turns: t2 }) => t1.cmp(&t2),
            (Outcome::Tie { turns: _ },
             Outcome::Unknown { depth: _ }) => Ordering::Less,
            (Outcome::Unknown { depth: _ },
             Outcome::Tie { turns: _ }) => Ordering::Greater,
            (Outcome::Unknown { depth: d1 },
             Outcome::Unknown { depth: d2 }) => d1.cmp(&d2),
        }
    }
}

// == cache ====================================================================

/// Constructs and returns a least-recently-used cache.
pub fn outcome_cache(capacity: usize) -> LruCache<Game, Outcome> {
    LruCache::<Game, Outcome>::with_capacity(capacity)
}
