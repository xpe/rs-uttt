use data::*;
use solver::{Solution, Outcome};
use std::cmp::Ordering;
use solver::Stack;

// == solving functions ========================================================

impl Game {
    /// Returns the solution for a given game and depth. This function looks
    /// ahead for a number of moves (specified by `depth`). If there are no
    /// valid moves, returns a solution where the optional play is `None` and
    /// the outcome is either a win or a tie.
    pub fn solve(&self, depth: Count, stack: &Stack) -> Solution {
        if depth == 0 {
            self.solve_zero_depth()
        } else if depth > 0 {
            self.solve_positive_depth(depth, stack)
        } else {
            panic!("Error 2553: depth < 0");
        }
    }

    /// Returns the solution for depth == 0.
    fn solve_zero_depth(&self) -> Solution {
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

    /// Returns the solution for a given game and depth.
    ///
    /// It works by first solving a simpler case, `depth - 1`. If the solution
    /// is dominant, return it. (For example, if the specified depth is 4 and a
    /// win can be found in 3 moves, there is no need to search 4 levels deep.)
    ///
    /// Alternatively, if the `depth - 1` solution is not dominant, then build a
    /// vector of possible solutions for all possible moves, with a solution
    /// depth of `depth`. Merge in the `depth - 1` solution if it has some
    /// value. (Since it might be the optimal solution, even though it was not
    /// dominant.)  Finally, return the best solution.
    fn solve_positive_depth(&self, depth: Count, stack: &Stack) -> Solution {
        let solution = stack.get_and_put(self, depth - 1, stack)
            .expect("Error 5125");
        match solution.dominant(self.next_player(), depth) {
            Some(dom) => dom,
            None => {
                let player = self.next_player().expect("Error 7751");
                let solutions = self.candidate_solutions(depth, stack);
                best_solution(player, solutions)
            }
        }
    }

    /// Returns candidate (possible) solutions for a given depth.
    ///
    /// Builds a vector of solutions by calculating all valid plays from the
    /// current game, computing the next game state for each play, and solving
    /// each resulting game to a depth of `depth - 1`. (Note: It is essential to
    /// decrease the depth by one as a counterbalance to advancing the game by
    /// one play. Otherwise, this function would not effectively be bounded by
    /// the depth argument.)
    fn candidate_solutions(&self, depth: Count, stack: &Stack)
                           -> Vec<Solution> {
        self.valid_plays().iter().map(|&play| {
            let mut game = self.clone();
            game.play(play);
            game.solve(depth - 1, stack).futurize(play)
        }).collect::<Vec<Solution>>()
    }
}

impl Solution {
    /// If a given solution is dominant, return it. A dominant solution is one
    /// good enough such that there is no need in searching for others.
    fn dominant(self, opt_player: Option<Player>, depth: Count)
                -> Option<Solution> {
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

/// Returns the best solution from given solutions for a player.
///
/// In some cases, the 'best solution' means 'admitting' defeat honestly. For
/// example, if one solution is be 'unknown to 9 turns' and another may be 'lose
/// in 10 turns', then this function returns the latter in this case, since it
/// is deeper.
fn best_solution(p: Player, ss: Vec<Solution>) -> Solution {
    let mut xs = ss.clone();
    xs.sort_by(|a, b| Solution::compare(p, *a, *b));
    xs.first().expect("Error 4875").clone()
}

// == 'modifiers' ==============================================================

impl Solution {
    /// Returns an 'updated' solution shifted into the future. Sets the play
    /// field and increments the associated count on the outcome field.
    fn futurize(self, play: Play) -> Solution {
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
        let play_a = a.opt_play.expect("Error 2643");
        let play_b = b.opt_play.expect("Error 4804");
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
            (Outcome::Tie { turns: t1 },
             Outcome::Tie { turns: t2 }) => {
                t2.cmp(&t1) // prefer to tie later, not sooner
            },
            (Outcome::Unknown { turns: t1 },
             Outcome::Unknown { turns: t2 }) => {
                t2.cmp(&t1) // Prefer the unknown later, not sooner
            },
            (Outcome::Win { player: p1, turns: t1 },
             Outcome::Unknown { turns: t2 }) => {
                if p1 == p {
                    Ordering::Less // prefer to win over the unknown
                } else { // p1 == o
                    if t2 >= t1 {
                        // prefer the unknown over losing, provided that
                        // the unknown is an equal or deeper search
                        Ordering::Greater
                    } else {
                        // admit defeat when it is certain
                        Ordering::Less
                    }
                }
            },
            (Outcome::Unknown { turns: t1 },
             Outcome::Win { player: p2, turns: t2 }) => {
                if p2 == o {
                    if t1 >= t2 {
                        // prefer the unknown over losing, provided that
                        // the unknown is an equal or deeper search
                        Ordering::Less
                    } else {
                        // admit defeat when it is certain
                        Ordering::Greater
                    }
                } else { // p2 == p
                    Ordering::Greater // prefer to win over the unknown
                }
            },
            (Outcome::Tie { turns: t1 },
             Outcome::Unknown { turns: t2 }) => {
                if t2 >= t1 {
                    // prefer the unknown over a tie, provided that the unknown
                    // is an equal or greater depth
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            },
            (Outcome::Unknown { turns: t1 },
             Outcome::Tie { turns: t2 }) => {
                if t1 >= t2 {
                    // prefer the unknown over a tie, provided that the unknown
                    // is an equal or greater depth
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },
        }
    }
}
