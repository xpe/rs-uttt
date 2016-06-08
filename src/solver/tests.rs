use constants::*;
use solver::*;

#[test]
fn test_empty_game() {
    let threads = 1;
    assert!(EMPTY_GAME.solve_for_uncached(0, threads) ==
            Solution {
                opt_play: None,
                outcome: Outcome::Unknown{ turns: 0 },
            });
}
