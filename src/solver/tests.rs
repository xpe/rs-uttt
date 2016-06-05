use constants::*;
use solver::*;

#[test]
fn test_empty_game() {
    assert!(EMPTY_GAME.solve_for(0) ==
            Solution {
                opt_play: None,
                outcome: Outcome::Unknown{ turns: 0 },
            });
}
