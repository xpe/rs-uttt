use constants::*;
use solver::solution::Solution;
use solver::outcome::Outcome;

#[test]
fn test_empty_game() {
    assert!(EMPTY_GAME.solve(0) ==
            Solution {
                opt_play: None,
                outcome: Outcome::Unknown{ turns: 0 },
            });
}
